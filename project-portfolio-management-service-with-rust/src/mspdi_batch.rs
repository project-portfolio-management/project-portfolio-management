//! Many MSPDI files in one job (NC-13): the shared resource pool and the custom-field mapping. DB-free, unit-tested.
//!
//! **The pool is a report, not a table.** Resources are still not matched to people (names are not identities), so
//! "merged once" means the job lists each distinct resource once, with the projects that use it, however many files
//! name it. Names match when they are equal after trimming, collapsing inner spaces and ignoring case.
//!
//! **Custom fields are mapped by the caller.** A project declares its fields (`Text1`, `Enterprise Text1`, each with
//! an alias); the job maps a field to an existing custom field of the service by the caller's `field_map`, matching the
//! map's key to the field's alias, name or id (ignoring case). A field with no mapping is listed as left out, with
//! its number of values.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::cpm::Kind;
use crate::mspdi::{MsCalendar, Parsed};

/// The most files one job reads.
pub const MAX_PROJECTS: usize = 10;

/// A resource as the job lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PoolEntry {
    /// The name as the first file spelled it, inner spaces collapsed.
    pub name: String,
    /// The projects that name it, in file order, each once.
    pub projects: Vec<String>,
    /// Assignments of it across all the files.
    pub assignments: usize,
}

/// A resource name reduced to what makes two spellings the same person: trimmed, inner spaces collapsed, lower case.
#[must_use]
pub fn normalize(name: &str) -> String {
    name.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Merge the resources of several projects into one pool, each distinct resource once, in first-seen order.
#[must_use]
pub fn merge_resources(projects: &[(String, &Parsed)]) -> Vec<PoolEntry> {
    let mut pool: Vec<(String, PoolEntry)> = Vec::new();
    for (project, parsed) in projects {
        for (uid, name) in &parsed.resources {
            let key = normalize(name);
            if key.is_empty() {
                continue;
            }
            let uses = parsed.assignments.iter().filter(|(_, r)| r == uid).count();
            let at = if let Some(i) = pool.iter().position(|(k, _)| *k == key) {
                i
            } else {
                pool.push((
                    key,
                    PoolEntry {
                        name: name.split_whitespace().collect::<Vec<_>>().join(" "),
                        projects: vec![],
                        assignments: 0,
                    },
                ));
                pool.len() - 1
            };
            let entry = &mut pool[at].1;
            if !entry.projects.contains(project) {
                entry.projects.push(project.clone());
            }
            entry.assignments += uses;
        }
    }
    pool.into_iter().map(|(_, e)| e).collect()
}

/// One value to store: the task, the service's custom-field key, and the text MS Project held.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mapped {
    /// The task's `UID`.
    pub uid: i64,
    /// The service's custom-field key.
    pub key: String,
    /// The value as text.
    pub raw: String,
}

/// A project field that was not carried over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeftOut {
    /// The field's alias, else its name.
    pub field: String,
    /// How many tasks had a value for it.
    pub values: usize,
    /// Why.
    pub reason: String,
}

/// Apply `field_map` to one project: the values to store, and the declared fields left out.
#[must_use]
pub fn map_fields(
    parsed: &Parsed,
    field_map: &BTreeMap<String, String>,
) -> (Vec<Mapped>, Vec<LeftOut>) {
    let wanted: Vec<(String, &String)> = field_map
        .iter()
        .map(|(k, v)| (k.trim().to_lowercase(), v))
        .collect();
    let mut mapped = Vec::new();
    let mut left = Vec::new();
    for f in &parsed.fields {
        let label = f.alias.clone().unwrap_or_else(|| f.name.clone());
        let values: Vec<_> = parsed
            .extended
            .iter()
            .filter(|e| e.field_id == f.field_id)
            .collect();
        let key = wanted.iter().find(|(w, _)| {
            f.alias.as_deref().is_some_and(|a| a.to_lowercase() == *w)
                || f.name.to_lowercase() == *w
                || f.field_id == *w
        });
        match key {
            Some((_, key)) => mapped.extend(values.iter().map(|e| Mapped {
                uid: e.uid,
                key: (*key).clone(),
                raw: e.value.clone(),
            })),
            None if values.is_empty() => {}
            None => left.push(LeftOut {
                field: label,
                values: values.len(),
                reason: "no mapping to a custom field was given for it".to_string(),
            }),
        }
    }
    (mapped, left)
}

/// The JSON values a field's text could stand for, most literal first: the text, then a whole number, then a
/// boolean. The caller keeps the first that the custom field's type accepts.
#[must_use]
pub fn candidates(raw: &str) -> Vec<Value> {
    let t = raw.trim();
    let mut out = vec![Value::String(t.to_string())];
    if let Ok(n) = t.parse::<i64>() {
        out.push(Value::from(n));
    }
    match t.to_lowercase().as_str() {
        "yes" | "true" | "1" => out.push(Value::Bool(true)),
        "no" | "false" | "0" => out.push(Value::Bool(false)),
        _ => {}
    }
    out
}

/// A project file's name reduced to what two spellings of it share: the last path segment, without its extension,
/// lower case. `C:\Plans\Beta.mpp`, `Beta.xml` and `beta` are the same project.
#[must_use]
pub fn project_key(name: &str) -> String {
    let last = name.rsplit(['\\', '/']).next().unwrap_or(name).trim();
    let stem = match last.rsplit_once('.') {
        Some((stem, ext)) if matches!(ext.to_lowercase().as_str(), "mpp" | "xml" | "mspdi") => stem,
        _ => last,
    };
    stem.trim().to_lowercase()
}

/// A cross-project link that names a task in another file of the job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolved {
    /// Index of the project holding the successor.
    pub successor_project: usize,
    /// The successor's `UID` in that file.
    pub successor: i64,
    /// Index of the project holding the predecessor.
    pub predecessor_project: usize,
    /// The predecessor's `UID` in its file.
    pub predecessor: i64,
    /// The link kind.
    pub kind: Kind,
    /// Business days of lag.
    pub lag_days: i64,
}

/// A cross-project link that could not be placed, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unresolved {
    /// The project holding the successor.
    pub project: String,
    /// The successor's `UID`.
    pub successor: i64,
    /// Why.
    pub reason: String,
}

/// Place each cross-project link of each file: the file it names must be in the job (matched by `project_key` to the
/// label or the project's own name), and the predecessor's `UID` must be a task of that file.
#[must_use]
pub fn resolve_cross_links(projects: &[(String, &Parsed)]) -> (Vec<Resolved>, Vec<Unresolved>) {
    let keys: Vec<Vec<String>> = projects
        .iter()
        .map(|(label, p)| {
            let mut k = vec![project_key(label)];
            k.extend(p.name.as_deref().map(project_key));
            k
        })
        .collect();
    let (mut ok, mut bad) = (Vec::new(), Vec::new());
    for (i, (label, p)) in projects.iter().enumerate() {
        for l in &p.cross_links {
            let wanted = project_key(&l.project);
            let hits: Vec<usize> = (0..projects.len())
                .filter(|j| *j != i && keys[*j].contains(&wanted))
                .collect();
            let reason = match hits.as_slice() {
                [] => Some(format!(
                    "{:?} is not among the files of this job",
                    l.project
                )),
                [j] if projects[*j].1.tasks.iter().any(|t| t.uid == l.predecessor) => {
                    ok.push(Resolved {
                        successor_project: i,
                        successor: l.successor,
                        predecessor_project: *j,
                        predecessor: l.predecessor,
                        kind: l.kind,
                        lag_days: l.lag_days,
                    });
                    None
                }
                [j] => Some(format!(
                    "task {} is not in {:?}",
                    l.predecessor, projects[*j].0
                )),
                _ => Some(format!(
                    "{:?} names more than one file of this job",
                    l.project
                )),
            };
            if let Some(reason) = reason {
                bad.push(Unresolved {
                    project: label.clone(),
                    successor: l.successor,
                    reason,
                });
            }
        }
    }
    (ok, bad)
}

/// What the files together say a resource is paid and works to. A value two files give differently is a conflict:
/// it is not taken, and it is listed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Terms {
    /// The resource, as in the pool.
    pub name: String,
    /// Minor currency units per hour, standard.
    pub standard: Option<i64>,
    /// Minor currency units per hour, overtime.
    pub overtime: Option<i64>,
    /// Per-use cost in minor units (reported, not stored).
    pub cost_per_use: Option<i64>,
    /// The resource's own calendar.
    pub calendar: Option<MsCalendar>,
    /// The currency the files agree on.
    pub currency: Option<String>,
    /// Which of the above the files disagree on.
    pub conflicts: Vec<String>,
    /// Rates that could not be converted, and similar.
    pub notes: Vec<String>,
}

fn agree<T: Clone + PartialEq>(values: &[T], what: &str, conflicts: &mut Vec<String>) -> Option<T> {
    let first = values.first()?;
    if values.iter().all(|v| v == first) {
        Some(first.clone())
    } else {
        conflicts.push(what.to_string());
        None
    }
}

/// Merge each resource's pay and calendar across the files, in pool order.
#[must_use]
pub fn merge_terms(projects: &[(String, &Parsed)]) -> Vec<Terms> {
    let pool = merge_resources(projects);
    pool.into_iter()
        .map(|entry| {
            let key = normalize(&entry.name);
            let mut standard = vec![];
            let mut overtime = vec![];
            let mut per_use = vec![];
            let mut calendars = vec![];
            let mut currencies = vec![];
            let mut notes = vec![];
            for (_, p) in projects {
                for r in p
                    .resource_terms
                    .iter()
                    .filter(|r| normalize(&r.name) == key)
                {
                    standard.extend(r.standard_per_hour_minor);
                    overtime.extend(r.overtime_per_hour_minor);
                    per_use.extend(r.cost_per_use_minor);
                    calendars.extend(r.calendar.clone());
                    notes.extend(r.rate_notes.clone());
                    if r.standard_per_hour_minor.is_some() || r.overtime_per_hour_minor.is_some() {
                        currencies.extend(p.currency.clone());
                    }
                }
            }
            notes.sort();
            notes.dedup();
            let mut conflicts = Vec::new();
            Terms {
                name: entry.name,
                standard: agree(&standard, "standard rate", &mut conflicts),
                overtime: agree(&overtime, "overtime rate", &mut conflicts),
                cost_per_use: agree(&per_use, "cost per use", &mut conflicts),
                calendar: agree(&calendars, "calendar", &mut conflicts),
                currency: agree(&currencies, "currency", &mut conflicts),
                conflicts,
                notes,
            }
        })
        .collect()
}

/// The custom-field type that fits every value: `integer`, `date`, `bool`, else `text`.
#[must_use]
pub fn infer_type<'a>(values: impl IntoIterator<Item = &'a str>) -> &'static str {
    let vals: Vec<&str> = values.into_iter().map(str::trim).collect();
    if vals.is_empty() {
        return "text";
    }
    if vals.iter().all(|v| v.parse::<i64>().is_ok()) {
        "integer"
    } else if vals.iter().all(|v| {
        chrono::NaiveDate::parse_from_str(v.get(..10).unwrap_or(v), "%Y-%m-%d").is_ok()
            && v.len() >= 10
    }) {
        "date"
    } else if vals
        .iter()
        .all(|v| matches!(v.to_lowercase().as_str(), "yes" | "no" | "true" | "false"))
    {
        "bool"
    } else {
        "text"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mspdi::parse;

    fn project(
        name: &str,
        resources: &[(i64, &str)],
        assignments: &[(i64, i64)],
        fields: &str,
        task_ext: &str,
    ) -> String {
        use std::fmt::Write;
        let mut res = String::new();
        for (u, n) in resources {
            write!(res, "<Resource><UID>{u}</UID><Name>{n}</Name></Resource>").unwrap();
        }
        let mut asg = String::new();
        for (t, r) in assignments {
            write!(
                asg,
                "<Assignment><TaskUID>{t}</TaskUID><ResourceUID>{r}</ResourceUID></Assignment>"
            )
            .unwrap();
        }
        format!(
            "<Project><Name>{name}</Name><ExtendedAttributes>{fields}</ExtendedAttributes>\
             <Tasks><Task><UID>1</UID><Name>Build</Name><OutlineLevel>1</OutlineLevel>{task_ext}</Task></Tasks>\
             <Resources>{res}</Resources><Assignments>{asg}</Assignments></Project>"
        )
    }

    #[test]
    fn a_resource_named_in_several_files_is_listed_once_with_every_project_that_uses_it() {
        let a = parse(&project(
            "A",
            &[(1, "Ana  Lopez"), (2, "Bo")],
            &[(1, 1), (1, 2)],
            "",
            "",
        ))
        .unwrap();
        let b = parse(&project("B", &[(7, "ana lopez")], &[(1, 7)], "", "")).unwrap();
        let c = parse(&project("C", &[(3, "Cy")], &[], "", "")).unwrap();
        let pool = merge_resources(&[("A".into(), &a), ("B".into(), &b), ("C".into(), &c)]);
        let names: Vec<&str> = pool.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(
            names,
            ["Ana Lopez", "Bo", "Cy"],
            "the first spelling, spaces collapsed"
        );
        assert_eq!(pool[0].projects, ["A", "B"]);
        assert_eq!(pool[0].assignments, 2);
        assert_eq!(pool[1].projects, ["A"]);
        assert_eq!(pool[2].assignments, 0);
    }

    #[test]
    fn fields_are_mapped_by_alias_name_or_id_and_the_rest_are_left_out_with_their_value_count() {
        let fields = "<ExtendedAttribute><FieldID>111</FieldID><FieldName>Enterprise Text1</FieldName><Alias>Cost centre</Alias></ExtendedAttribute>\
                      <ExtendedAttribute><FieldID>222</FieldID><FieldName>Text2</FieldName></ExtendedAttribute>\
                      <ExtendedAttribute><FieldID>333</FieldID><FieldName>Text3</FieldName></ExtendedAttribute>";
        let ext = "<ExtendedAttribute><FieldID>111</FieldID><Value>CC-100</Value></ExtendedAttribute>\
                   <ExtendedAttribute><FieldID>222</FieldID><Value>x</Value></ExtendedAttribute>";
        let p = parse(&project("A", &[], &[], fields, ext)).unwrap();
        assert_eq!(p.fields.len(), 3);
        let map: BTreeMap<String, String> =
            [("COST CENTRE".to_string(), "cost_centre".to_string())].into();
        let (mapped, left) = map_fields(&p, &map);
        assert_eq!(
            mapped,
            [Mapped {
                uid: 1,
                key: "cost_centre".into(),
                raw: "CC-100".into()
            }]
        );
        assert_eq!(left.len(), 1, "Text3 has no values so it is not reported");
        assert_eq!((left[0].field.as_str(), left[0].values), ("Text2", 1));
        let by_id: BTreeMap<String, String> = [("222".to_string(), "t2".to_string())].into();
        assert_eq!(map_fields(&p, &by_id).0[0].key, "t2");
    }

    #[test]
    fn a_value_is_offered_as_text_then_number_then_boolean() {
        assert_eq!(candidates(" 12 "), [Value::from("12"), Value::from(12)]);
        assert_eq!(candidates("Yes"), [Value::from("Yes"), Value::Bool(true)]);
        assert_eq!(candidates("1").len(), 3);
        assert_eq!(candidates("plain"), [Value::from("plain")]);
    }

    #[test]
    fn a_cross_project_link_is_placed_when_its_file_and_task_are_in_the_job() {
        let link = "<PredecessorLink><PredecessorUID>1</PredecessorUID><Type>1</Type><CrossProject>1</CrossProject><CrossProjectName>C:\\Plans\\Alpha.mpp</CrossProjectName></PredecessorLink>";
        let a = parse(&project("Alpha", &[], &[], "", "")).unwrap();
        let b_xml = project("Beta", &[], &[], "", "").replace("</Task>", &format!("{link}</Task>"));
        let b = parse(&b_xml).unwrap();
        assert_eq!(b.cross_links.len(), 1);
        let (ok, bad) = resolve_cross_links(&[("Alpha".into(), &a), ("Beta".into(), &b)]);
        assert!(bad.is_empty(), "{bad:?}");
        assert_eq!(
            (
                ok[0].successor_project,
                ok[0].predecessor_project,
                ok[0].predecessor
            ),
            (1, 0, 1)
        );
        // Without the file, or with a task it does not have, the link is listed with the reason.
        let (ok, bad) = resolve_cross_links(&[("Beta".into(), &b)]);
        assert!(
            ok.is_empty() && bad[0].reason.contains("not among the files"),
            "{bad:?}"
        );
        let b_missing = parse(&b_xml.replace("<PredecessorUID>1<", "<PredecessorUID>99<")).unwrap();
        let (ok, bad) = resolve_cross_links(&[("Alpha".into(), &a), ("Beta".into(), &b_missing)]);
        assert!(
            ok.is_empty() && bad[0].reason.contains("task 99"),
            "{bad:?}"
        );
    }

    #[test]
    fn project_names_match_across_paths_and_extensions() {
        assert_eq!(project_key("C:\\Plans\\Beta.mpp"), "beta");
        assert_eq!(project_key("/x/y/Beta.XML"), "beta");
        assert_eq!(project_key(" Beta "), "beta");
        assert_eq!(
            project_key("v1.2 plan"),
            "v1.2 plan",
            "a dot that is not an extension stays"
        );
    }

    fn with_pay(name: &str, rate: &str, fmt: &str, extra: &str) -> String {
        format!(
            "<Project><Name>{name}</Name><CurrencyCode>GBP</CurrencyCode><MinutesPerDay>480</MinutesPerDay>\
             <Calendars><Calendar><UID>5</UID><IsBaseCalendar>0</IsBaseCalendar><WeekDays>\
               <WeekDay><DayType>6</DayType><DayWorking>0</DayWorking></WeekDay></WeekDays></Calendar></Calendars>\
             <Tasks><Task><UID>1</UID><Name>t</Name><FixedCost>250.5</FixedCost><FixedCostAccrual>2</FixedCostAccrual></Task></Tasks>\
             <Resources><Resource><UID>1</UID><Name>Ana</Name><StandardRate>{rate}</StandardRate>\
               <StandardRateFormat>{fmt}</StandardRateFormat><CalendarUID>5</CalendarUID>{extra}</Resource></Resources></Project>"
        )
    }

    #[test]
    fn rates_are_converted_to_minor_units_per_hour_and_files_that_disagree_conflict() {
        let a = parse(&with_pay("A", "50", "2", "")).unwrap();
        let day = parse(&with_pay("B", "400", "3", "")).unwrap();
        assert_eq!(a.resource_terms[0].standard_per_hour_minor, Some(5000));
        assert_eq!(
            day.resource_terms[0].standard_per_hour_minor,
            Some(5000),
            "400 a day over 8 hours"
        );
        assert_eq!(a.fixed_costs[0].amount_minor, 25050);
        assert_eq!(a.fixed_costs[0].accrual, "end");
        assert_eq!(
            a.resource_terms[0].calendar.as_ref().unwrap().weekdays,
            "1111000",
            "Friday (day type 6) is off"
        );
        let t = merge_terms(&[("A".into(), &a), ("B".into(), &day)]);
        assert_eq!(
            (t[0].standard, t[0].currency.as_deref()),
            (Some(5000), Some("GBP"))
        );
        assert!(t[0].conflicts.is_empty());
        let other = parse(&with_pay("C", "60", "2", "")).unwrap();
        let t = merge_terms(&[("A".into(), &a), ("C".into(), &other)]);
        assert_eq!(t[0].standard, None);
        assert_eq!(t[0].conflicts, ["standard rate"]);
        let week = parse(&with_pay("D", "1000", "4", "")).unwrap();
        assert_eq!(week.resource_terms[0].standard_per_hour_minor, None);
        assert!(week.resource_terms[0].rate_notes[0].contains("week, month or year"));
    }

    #[test]
    fn a_field_type_is_inferred_from_all_its_values() {
        assert_eq!(infer_type(["1", " 22 "]), "integer");
        assert_eq!(infer_type(["2026-11-02T08:00:00", "2026-12-01"]), "date");
        assert_eq!(infer_type(["Yes", "no"]), "bool");
        assert_eq!(infer_type(["1", "x"]), "text");
        assert_eq!(infer_type([]), "text");
    }
}
