//! Retention purge (RG-4): which records are past their period, and the runner that removes them.
//!
//! The periods are the starting ones in `governance/retention-schedule.md`, each a deployment setting
//! (`PROJECT_PORTFOLIO_MANAGEMENT_RETENTION_<RULE>_DAYS`, WW-1.7) with its reason in [`Rule::basis`]. **Only records whose
//! deletion is unambiguous are covered**: read notifications (`COM`), the webhook and notification
//! delivery logs (`EVT`), and the trend snapshots (`DRV`). Plans, tasks, documents, time records and
//! the audit trail itself are *not* purged here: how long to keep them is the buyer's decision, and
//! each needs its own rule (see the schedule's known gaps).
//!
//! Safe by construction: [`run`] is a **dry run unless `apply` is true**, counts first, deletes in
//! one transaction per table and writes one audit row per table. The pure parts ([`Rule`],
//! [`cutoff`], [`parse_days`]) are unit-tested; the SQL is covered by a database test.

use chrono::{DateTime, Duration, Utc};
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, Statement, TransactionTrait};
use uuid::Uuid;

use crate::models::audit_logs::Model as AuditModel;

/// One purge rule: delete rows of `table` whose `age_column` is older than `days`, and that match
/// `also` (a fixed SQL condition, never user input).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rule {
    /// The argument name that overrides the period (`<name>_days`).
    pub name: &'static str,
    /// The retention class in the schedule.
    pub class: &'static str,
    /// The table.
    pub table: &'static str,
    /// The timestamp column the age is measured from.
    pub age_column: &'static str,
    /// An extra fixed condition, or empty.
    pub also: &'static str,
    /// The default period, in calendar days.
    pub default_days: i64,
    /// Why the default is what it is (WW-1.7). None of these is set by a law: a deployment whose
    /// records manager or applicable law asks for another period sets
    /// `PROJECT_PORTFOLIO_MANAGEMENT_RETENTION_<NAME>_DAYS` ([`Rule::env_name`]).
    pub basis: &'static str,
}

/// Every rule, in the order they run.
pub const RULES: &[Rule] = &[
    Rule {
        name: "notifications",
        class: "COM",
        table: "notifications",
        age_column: "read_at",
        also: "",
        default_days: 90,
        basis: "a read notification has done its job; 90 days leaves a quarter to look back on it",
    },
    Rule {
        name: "webhook_log",
        class: "EVT",
        table: "webhook_deliveries",
        age_column: "created_at",
        also: "",
        default_days: 30,
        basis: "a delivery log is for diagnosing a failed delivery, which is noticed within days",
    },
    Rule {
        name: "sink_log",
        class: "EVT",
        table: "sink_deliveries",
        age_column: "created_at",
        also: "next_attempt_at IS NULL",
        default_days: 30,
        basis: "as the webhook log: a settled delivery is only kept for diagnosis",
    },
    Rule {
        name: "idempotency_keys",
        class: "GEN",
        table: "idempotency_keys",
        age_column: "created_at",
        also: "",
        default_days: 1,
        basis: "a retry with the same key comes within minutes; one day is ample (NFR on idempotency)",
    },
    Rule {
        name: "insight_snapshots",
        class: "DRV",
        table: "insight_snapshots",
        age_column: "taken_at",
        also: "",
        default_days: 3 * 365,
        basis: "three years of trend gives a comparison across a typical portfolio planning cycle",
    },
    Rule {
        name: "adoption_snapshots",
        class: "DRV",
        table: "adoption_snapshots",
        age_column: "observed_at",
        also: "",
        default_days: 3 * 365,
        basis: "as the trend snapshots: three years of history for comparison",
    },
];

/// The prefix of the deployment setting for a rule's period: `PROJECT_PORTFOLIO_MANAGEMENT_RETENTION_<NAME>_DAYS`.
pub const ENV_PREFIX: &str = "PROJECT_PORTFOLIO_MANAGEMENT_RETENTION_";

/// The periods a deployment sets (WW-1.7), read through `lookup` (the environment in production, a map in
/// tests). An unset rule keeps its default. A set value that is not a whole number of days from 1 to
/// [`MAX_DAYS`] is an error, not a silent fall-back: a mistyped period must not delete by another one.
///
/// # Errors
///
/// The message names the setting and the value it refused.
pub fn deployment_overrides(
    lookup: impl Fn(&str) -> Option<String>,
) -> Result<Vec<(String, i64)>, String> {
    let mut out = Vec::new();
    for rule in RULES {
        let key = rule.env_name();
        if let Some(raw) = lookup(&key) {
            let days = parse_days(rule.name, &raw).map_err(|m| format!("{key}: {m}"))?;
            out.push((rule.name.to_string(), days));
        }
    }
    Ok(out)
}

/// The deployment's periods from the environment ([`deployment_overrides`]).
///
/// # Errors
///
/// As [`deployment_overrides`].
pub fn env_overrides() -> Result<Vec<(String, i64)>, String> {
    deployment_overrides(|k| std::env::var(k).ok())
}

/// The instant before which a record is past its period.
#[must_use]
pub fn cutoff(now: DateTime<Utc>, days: i64) -> DateTime<Utc> {
    now - Duration::days(days)
}

/// A period given on the command line: a whole number of calendar days, at least 1.
///
/// # Errors
///
/// A message naming the argument when it is not a positive whole number.
pub fn parse_days(name: &str, raw: &str) -> Result<i64, String> {
    match raw.trim().parse::<i64>() {
        Ok(d) if (1..=MAX_DAYS).contains(&d) => Ok(d),
        _ => Err(format!(
            "{name}_days must be a whole number of calendar days from 1 to {MAX_DAYS} (got {raw:?})"
        )),
    }
}

/// The longest period accepted (one hundred years). Without a ceiling an absurd value overflows the
/// date arithmetic and aborts the task.
pub const MAX_DAYS: i64 = 36_500;

impl Rule {
    /// The deployment setting that overrides this rule's period.
    #[must_use]
    pub fn env_name(&self) -> String {
        format!("{ENV_PREFIX}{}_DAYS", self.name.to_ascii_uppercase())
    }

    /// The `WHERE` clause for rows past the cut-off, which is bound as `$1` (CQ-12): only the
    /// rule's own fixed identifiers are written into the SQL.
    #[must_use]
    pub fn condition(&self) -> String {
        let mut c = format!(
            "{} IS NOT NULL AND {} < $1",
            self.age_column, self.age_column
        );
        if !self.also.is_empty() {
            c.push_str(" AND ");
            c.push_str(self.also);
        }
        c
    }
}

/// What one rule did (or would do).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    /// The rule.
    pub rule: &'static str,
    /// The table.
    pub table: &'static str,
    /// The period used, in calendar days.
    pub days: i64,
    /// Rows past their period.
    pub matched: u64,
    /// Rows deleted (zero in a dry run).
    pub deleted: u64,
}

fn db_err(e: &sea_orm::DbErr) -> loco_rs::Error {
    loco_rs::Error::string(&e.to_string())
}

/// Run every rule. `days` overrides a rule's period by name. Without `apply`, nothing is deleted.
///
/// # Errors
///
/// When a database statement fails; earlier tables stay purged, the failing one is rolled back.
pub async fn run(
    db: &DatabaseConnection,
    now: DateTime<Utc>,
    overrides: &[(String, i64)],
    apply: bool,
) -> loco_rs::Result<Vec<Outcome>> {
    let mut out = Vec::new();
    for rule in RULES {
        let days = overrides
            .iter()
            .find(|(n, _)| n == rule.name)
            .map_or(rule.default_days, |(_, d)| *d);
        let cut = cutoff(now, days);
        let where_clause = rule.condition();
        let count_sql = format!(
            "SELECT COUNT(*) AS n FROM {} WHERE {where_clause}",
            rule.table
        );
        let matched = db
            .query_one_raw(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                count_sql,
                [sea_orm::Value::from(cut)],
            ))
            .await
            .map_err(|e| db_err(&e))?
            .map_or(0, |r| r.try_get::<i64>("", "n").unwrap_or(0));
        let matched = u64::try_from(matched).unwrap_or(0);
        let mut deleted = 0;
        if apply && matched > 0 {
            let tx = db.begin().await.map_err(|e| db_err(&e))?;
            tx.execute_unprepared("SELECT set_config('ppm.retention_purge', 'on', true)")
                .await
                .map_err(|e| db_err(&e))?;
            deleted = tx
                .execute_raw(Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    format!("DELETE FROM {} WHERE {where_clause}", rule.table),
                    [sea_orm::Value::from(cut)],
                ))
                .await
                .map_err(|e| db_err(&e))?
                .rows_affected();
            AuditModel::record(
                &tx,
                Uuid::nil(),
                "retention_purged",
                Some("system:retention"),
                Some(serde_json::json!({
                    "table": rule.table, "class": rule.class, "days": days,
                    "cutoff": cut, "deleted": deleted,
                })),
            )
            .await
            .map_err(|e| loco_rs::Error::string(&e.to_string()))?;
            tx.commit().await.map_err(|e| db_err(&e))?;
        }
        out.push(Outcome {
            rule: rule.name,
            table: rule.table,
            days,
            matched,
            deleted,
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(d: u32) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(&format!("2026-10-{d:02}T12:00:00Z"))
            .unwrap()
            .with_timezone(&Utc)
    }

    #[test]
    fn the_cutoff_is_calendar_days_back() {
        assert_eq!(cutoff(at(10), 9), at(1));
        assert_eq!(cutoff(at(10), 90), at(10) - Duration::days(90));
    }

    #[test]
    fn periods_must_be_whole_calendar_days_of_at_least_one() {
        assert_eq!(parse_days("notifications", " 45 "), Ok(45));
        assert_eq!(parse_days("notifications", "36500"), Ok(MAX_DAYS));
        // The ceiling is safe to subtract from a real date.
        assert!(cutoff(at(10), MAX_DAYS) < at(10));
        for bad in [
            "0",
            "-3",
            "1.5",
            "abc",
            "",
            "36501",
            "9223372036854775807",
            "99999999999999999999",
        ] {
            let e = parse_days("notifications", bad).unwrap_err();
            assert!(e.contains("notifications_days"), "{e}");
        }
    }

    #[test]
    fn a_rule_selects_only_old_rows_that_have_a_date() {
        let c = RULES[0].condition();
        assert_eq!(c, "read_at IS NOT NULL AND read_at < $1");
        let c = RULES[2].condition();
        assert!(c.ends_with("AND next_attempt_at IS NULL"), "{c}");
    }

    /// CQ-12: what is written into the SQL is the rules' own fixed identifiers, never data.
    #[test]
    fn rule_identifiers_are_plain_names() {
        let plain = |s: &str| {
            !s.is_empty()
                && s.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        };
        for r in RULES {
            assert!(plain(r.table), "{}", r.table);
            assert!(plain(r.age_column), "{}", r.age_column);
            assert!(!r.also.contains(['\'', ';', '$']), "{}", r.also);
        }
    }

    #[test]
    fn the_defaults_match_the_schedule() {
        let days = |n: &str| RULES.iter().find(|r| r.name == n).unwrap().default_days;
        assert_eq!(days("notifications"), 90);
        assert_eq!(days("webhook_log"), 30);
        assert_eq!(days("insight_snapshots"), 3 * 365);
    }

    #[test]
    fn every_default_states_its_basis() {
        for r in RULES {
            assert!(r.basis.len() > 20, "{} has no stated basis", r.name);
        }
    }

    /// WW-1.7: a deployment sets each period; unset keeps the default; a bad value is refused, not ignored.
    #[test]
    fn a_deployment_sets_the_periods() {
        assert_eq!(
            RULES[0].env_name(),
            "PROJECT_PORTFOLIO_MANAGEMENT_RETENTION_NOTIFICATIONS_DAYS"
        );
        assert_eq!(deployment_overrides(|_| None), Ok(vec![]));
        let set = |k: &str| match k {
            "PROJECT_PORTFOLIO_MANAGEMENT_RETENTION_NOTIFICATIONS_DAYS" => Some("30".to_string()),
            "PROJECT_PORTFOLIO_MANAGEMENT_RETENTION_INSIGHT_SNAPSHOTS_DAYS" => {
                Some(" 3650 ".to_string())
            }
            _ => None,
        };
        assert_eq!(
            deployment_overrides(set),
            Ok(vec![
                ("notifications".to_string(), 30),
                ("insight_snapshots".to_string(), 3650)
            ])
        );
        let bad = |k: &str| {
            (k == "PROJECT_PORTFOLIO_MANAGEMENT_RETENTION_WEBHOOK_LOG_DAYS")
                .then(|| "0".to_string())
        };
        let e = deployment_overrides(bad).unwrap_err();
        assert!(e.contains("RETENTION_WEBHOOK_LOG_DAYS"), "{e}");
    }

    #[test]
    fn rule_names_and_tables_are_unique() {
        for (i, a) in RULES.iter().enumerate() {
            for b in &RULES[i + 1..] {
                assert_ne!(a.name, b.name);
                assert_ne!(a.table, b.table);
            }
        }
    }
}
