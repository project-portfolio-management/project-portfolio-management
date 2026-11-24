//! Numeric settings from the environment, checked once at start-up (spec/rust-checked-arithmetic RCA-IN-4; service spec,
//! "Settings: valid ranges").
//!
//! A setting that is supplied but not a whole number in its range stops the service from starting, naming the setting.
//! A setting that is absent, or blank (declared to mean absent), takes the default its reader states. The readers keep
//! those defaults for the absent case only: an invalid value never reaches them.

use crate::prelude::*;

/// A numeric setting and its valid range, inclusive.
struct Setting {
    /// The environment variable.
    name: &'static str,
    /// The least valid value.
    min: u64,
    /// The greatest valid value.
    max: u64,
}

/// Every numeric setting the service reads, with its valid range. A `0` minimum means `0` turns the feature off.
const SETTINGS: &[Setting] = &[
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_WORKER_SKILLS_CACHE_TTL_SECONDS",
        min: 0,
        max: 604_800,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_EVENT_RELAY_INTERVAL_SECS",
        min: 1,
        max: 86_400,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_GENERAL_PER_MINUTE",
        min: 1,
        max: 1_000_000,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_GENERAL_BURST",
        min: 1,
        max: 1_000_000,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_EXPENSIVE_PER_MINUTE",
        min: 1,
        max: 1_000_000,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_EXPENSIVE_BURST",
        min: 1,
        max: 1_000_000,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_FAILURES_PER_MINUTE",
        min: 1,
        max: 1_000_000,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_PASETO_KEYS_REFRESH_SECS",
        min: 0,
        max: 604_800,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_CPM_FULL_MAX_TASKS",
        min: 1,
        max: 10_000_000,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_BULK_MAX_BYTES",
        min: 1,
        max: 4_294_967_296,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_MAX_BYTES",
        min: 1,
        max: 4_294_967_296,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_BULK_TTL_HOURS",
        min: 1,
        max: 8_760,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_REPORT_TTL_DAYS",
        min: 1,
        max: 3_650,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_MAX_SCHEDULE_BASELINES",
        min: 1,
        max: 10_000,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_SINKS_TICK_SECS",
        min: 0,
        max: 86_400,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_STALL_DAYS",
        min: 1,
        max: 3_650,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_EXCEPTION_WINDOW_HOURS",
        min: 1,
        max: 720,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_SNAPSHOT_HOURS",
        min: 0,
        max: 8_760,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULER_MINUTES",
        min: 0,
        max: 525_600,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_MINUTES_PER_DAY",
        min: 1,
        max: 1_440,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_WIP_LIMIT",
        min: 1,
        max: 1_000,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_SUPPRESSION_FLOOR_MINUTES",
        min: 0,
        max: 100_000,
    },
    Setting {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_MIN_GROUP_PEOPLE",
        min: 2,
        max: 1_000,
    },
];

/// Check one setting's value: `Ok` when absent or blank, or a whole number in range.
///
/// # Errors
///
/// A message naming the setting and its range when the value is not a whole number in range.
fn check(setting: &Setting, value: Option<&str>) -> Result<(), String> {
    let Some(raw) = value.map(str::trim).filter(|v| !v.is_empty()) else {
        return Ok(());
    };
    match raw.parse::<u64>() {
        Ok(n) if (setting.min..=setting.max).contains(&n) => Ok(()),
        _ => Err(format!(
            "{} must be a whole number from {} to {}",
            setting.name, setting.min, setting.max
        )),
    }
}

/// A setting that is one of a few words, or a pattern, rather than a number.
struct Choice {
    /// The environment variable.
    name: &'static str,
    /// What a valid value looks like, for the message.
    expected: &'static str,
    /// Whether a trimmed, non-blank value is valid.
    valid: fn(&str) -> bool,
}

/// Every word-valued setting the service reads and a bad value of which used to be replaced by a default.
const CHOICES: &[Choice] = &[
    Choice {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_DEFAULT_WORKING_WEEK",
        expected: "seven characters of 0 or 1, Monday first, at least one 1 (for example 1111100)",
        valid: |v| crate::calendars::parse_weekdays(v).is_ok(),
    },
    Choice {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_CCPM_ESTIMATE_RULE",
        expected: "explicit_only or half_of_safe",
        valid: |v| matches!(v, "explicit_only" | "half_of_safe"),
    },
    Choice {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_WIP_LIMITS",
        expected: "a JSON object of task status to a whole-number cap of at least 1, such as {\"in_progress\": 3}",
        valid: |v| crate::engineering::parse_wip_limits(Some(v)).is_some(),
    },
    Choice {
        name: "PROJECT_PORTFOLIO_MANAGEMENT_REGISTER_REQUIRED",
        expected: "on, 1, true or yes; or off, 0, false or no",
        valid: |v| {
            matches!(
                v.to_ascii_lowercase().as_str(),
                "on" | "1" | "true" | "yes" | "off" | "0" | "false" | "no"
            )
        },
    },
];

/// Check one word-valued setting: `Ok` when absent or blank, or a valid value.
///
/// # Errors
///
/// A message naming the setting and what it may be.
fn check_choice(choice: &Choice, value: Option<&str>) -> Result<(), String> {
    let Some(raw) = value.map(str::trim).filter(|v| !v.is_empty()) else {
        return Ok(());
    };
    if (choice.valid)(raw) {
        Ok(())
    } else {
        Err(format!("{} must be {}", choice.name, choice.expected))
    }
}

/// Check every numeric setting in the environment.
///
/// # Errors
///
/// The first setting that is supplied but invalid.
pub fn validate() -> Result<(), String> {
    validate_with(|name| std::env::var(name).ok())
}

/// [`validate`] over any source of values, so a test need not touch the process environment.
///
/// # Errors
///
/// The first setting that is supplied but invalid.
pub fn validate_with(get: impl Fn(&str) -> Option<String>) -> Result<(), String> {
    SETTINGS
        .iter()
        .try_for_each(|setting| check(setting, get(setting.name).as_deref()))?;
    CHOICES
        .iter()
        .try_for_each(|choice| check_choice(choice, get(choice.name).as_deref()))
}

#[cfg(test)]
mod tests {
    use super::{CHOICES, SETTINGS, validate_with};

    /// RCA-IN-2, RCA-IN-4: a supplied value outside its range, or not a number, stops start-up and names the setting;
    /// an absent or blank one does not.
    #[test]
    fn an_invalid_setting_is_refused_by_name_and_an_absent_one_is_not() {
        assert!(validate_with(|_| None).is_ok());
        assert!(
            validate_with(|_| Some("  ".to_string())).is_ok(),
            "blank means absent"
        );
        for setting in SETTINGS {
            let with = |v: &str| {
                let v = v.to_string();
                validate_with(move |name| (name == setting.name).then(|| v.clone()))
            };
            assert!(with("lots").is_err(), "{}: junk", setting.name);
            assert!(with("-5").is_err(), "{}: negative", setting.name);
            assert!(
                with("99999999999999").is_err(),
                "{}: past its maximum",
                setting.name
            );
            assert!(
                with(&setting.min.to_string()).is_ok(),
                "{}: its minimum",
                setting.name
            );
            if setting.min > 0 {
                assert!(with("0").is_err(), "{}: below its minimum", setting.name);
            }
            let message = with("lots").err().unwrap_or_default();
            assert!(message.contains(setting.name), "{message}");
        }
    }

    /// RCA-IN-4: a word-valued setting that is supplied and not one of its words stops start-up and names the setting.
    #[test]
    fn a_bad_word_setting_is_refused_by_name_and_a_good_or_absent_one_is_not() {
        let with = |name: &str, v: &str| {
            let (name, v) = (name.to_string(), v.to_string());
            validate_with(move |n| (n == name).then(|| v.clone()))
        };
        for choice in CHOICES {
            let message = with(choice.name, "sometimes").unwrap_err();
            assert!(message.contains(choice.name), "{message}");
            assert!(with(choice.name, " ").is_ok(), "{}: blank", choice.name);
        }
        let week = "PROJECT_PORTFOLIO_MANAGEMENT_DEFAULT_WORKING_WEEK";
        for good in ["1111100", "1111001", "0000001"] {
            assert!(with(week, good).is_ok(), "{good}");
        }
        for bad in ["0000000", "11111", "abcdefg", "11111000"] {
            assert!(with(week, bad).is_err(), "{bad}");
        }
        let rule = "PROJECT_PORTFOLIO_MANAGEMENT_CCPM_ESTIMATE_RULE";
        assert!(with(rule, "half_of_safe").is_ok());
        assert!(with(rule, "Half_Of_Safe").is_err(), "the words are exact");
        let flag = "PROJECT_PORTFOLIO_MANAGEMENT_REGISTER_REQUIRED";
        for good in ["on", "1", "TRUE", "Yes", "off", "0", "false", "no"] {
            assert!(with(flag, good).is_ok(), "{good}");
        }
        assert!(with(flag, "maybe").is_err());
        let wip = "PROJECT_PORTFOLIO_MANAGEMENT_WIP_LIMITS";
        for good in ["{}", r#"{"in_progress": 3}"#] {
            assert!(with(wip, good).is_ok(), "{good}");
        }
        for bad in [
            "in_progress=3",
            r#"{"in_progress": 0}"#,
            r#"{"nonsense": 3}"#,
            r#"{"in_progress": "3"}"#,
        ] {
            assert!(with(wip, bad).is_err(), "{bad}");
        }
    }
}
