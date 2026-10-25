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

use crate::mspdi::Parsed;

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
}
