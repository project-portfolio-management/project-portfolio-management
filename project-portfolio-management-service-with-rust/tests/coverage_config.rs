//! The two cargo-tarpaulin configurations (`tarpaulin.toml`, without a database, and `tarpaulin-db.toml`, with the
//! database suites) leave the same files out of the denominator, and every file or directory they name exists, so
//! the local figure and CI's measure the same code and an exclusion cannot outlive what it excluded. No database.

use std::path::Path;

/// The sections of a tarpaulin file and each one's lines, `[name]` then everything up to the next section.
fn sections(text: &str) -> Vec<(String, Vec<String>)> {
    let mut out: Vec<(String, Vec<String>)> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            out.push((name.to_string(), Vec::new()));
        } else if let Some((_, lines)) = out.last_mut() {
            lines.push(line.to_string());
        }
    }
    out
}

/// The quoted entries of a section's `exclude-files = [ … ]`, in order.
fn excluded(lines: &[String]) -> Vec<String> {
    let start = lines
        .iter()
        .position(|l| l.starts_with("exclude-files"))
        .expect("an exclude-files list");
    let mut out = Vec::new();
    for line in &lines[start..] {
        for part in line.split('"').skip(1).step_by(2) {
            out.push(part.to_string());
        }
        if line.ends_with(']') {
            break;
        }
    }
    out
}

fn read(name: &str) -> String {
    std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(name))
        .unwrap_or_else(|e| panic!("read {name}: {e}"))
}

#[test]
fn both_tarpaulin_files_exclude_the_same_files_and_every_one_exists() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut reference: Option<Vec<String>> = None;
    let mut runs = 0;
    for file in ["tarpaulin.toml", "tarpaulin-db.toml"] {
        let text = read(file);
        for (name, lines) in sections(&text) {
            if name == "report" {
                assert!(
                    lines.iter().any(|l| l.starts_with("output-dir")),
                    "{file} [report] names its output directory"
                );
                continue;
            }
            runs += 1;
            for setting in [
                "engine = \"Llvm\"",
                "skip-clean = true",
                "target-dir = \"target/tarpaulin\"",
            ] {
                assert!(
                    lines.iter().any(|l| l == setting),
                    "{file} [{name}] sets {setting}"
                );
            }
            let list = excluded(&lines);
            assert!(!list.is_empty(), "{file} [{name}] excludes something");
            match &reference {
                None => reference = Some(list),
                Some(first) => assert_eq!(
                    &list, first,
                    "{file} [{name}] excludes what tarpaulin.toml [unit] does"
                ),
            }
        }
    }
    assert_eq!(
        runs, 6,
        "two runs in tarpaulin.toml, four in tarpaulin-db.toml"
    );
    // `target/*` is the build output, which need not exist when this runs (a different `CARGO_TARGET_DIR`).
    for pattern in reference
        .expect("at least one run")
        .into_iter()
        .filter(|p| p != "target/*")
    {
        let literal = pattern.trim_end_matches('*').trim_end_matches('/');
        assert!(
            dir.join(literal).exists(),
            "the excluded {pattern} names something that exists"
        );
    }
}
