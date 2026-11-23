//! Every setting that names an outbound host is in the deployment guide (SA-2, spec SA-R2).
//!
//! `operations/deployment.md` § "Outbound hosts" lists each setting by which the service or the operator interface
//! reaches another host, and a second table lists settings named like hosts that are not contacted. This test fails
//! when
//!
//! - the service's source holds a string literal named like a host setting (ending `_URL`, `_ENDPOINT`, `_ADDR`,
//!   `_HOST` or `_ISSUER`) that neither table names;
//! - the operator interface's source reads such a setting (`env.NAME`) that neither table names;
//! - either table names a setting that nothing reads any more.
//!
//! Three settings hold hosts without a host-like name or are read by a library under a prefix; [`NAMED_BY_HAND`]
//! lists them with the text that shows each is still read.
//!
//! **Not covered:** a host setting whose name has none of the endings above, unless it is added to
//! [`NAMED_BY_HAND`]; settings read by axa itself (only `DB_URL` reaches a host, and it is listed); the scripts in
//! `scripts/`. No database: this runs in a normal `cargo test`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Endings that mark a setting's name as naming a host.
const HOST_ENDINGS: &[&str] = &["_URL", "_ENDPOINT", "_ADDR", "_HOST", "_ISSUER"];

/// Settings the scan cannot find by name, each with text in the service's source that shows it is read.
const NAMED_BY_HAND: &[(&str, &str)] = &[
    (
        "PROJECT_PORTFOLIO_MANAGEMENT_KEYCLOAK_URL",
        "KeycloakSettings::from_env(\"PROJECT_PORTFOLIO_MANAGEMENT\")",
    ),
    (
        "PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS",
        "\"PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS\"",
    ),
    (
        "PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS_FILE",
        "\"PROJECT_PORTFOLIO_MANAGEMENT_WEBHOOKS_FILE\"",
    ),
];

fn service_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn repo_root() -> PathBuf {
    service_root()
        .parent()
        .expect("the service sits in the repository")
        .to_path_buf()
}

fn files_under(dir: &Path, extensions: &[&str], out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files_under(&path, extensions, out);
        } else if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| extensions.contains(&e))
        {
            out.push(path);
        }
    }
}

fn is_setting_char(c: char) -> bool {
    c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'
}

fn names_a_host(name: &str) -> bool {
    name.starts_with(|c: char| c.is_ascii_uppercase())
        && HOST_ENDINGS.iter().any(|ending| name.ends_with(ending))
}

/// Host-like names written as a whole string literal: `"NAME"`.
fn quoted_host_names(text: &str) -> BTreeSet<String> {
    text.match_indices('"')
        .filter_map(|(at, _)| {
            let after = &text[at + 1..];
            let name: String = after.chars().take_while(|&c| is_setting_char(c)).collect();
            (after[name.len()..].starts_with('"') && names_a_host(&name)).then_some(name)
        })
        .collect()
}

/// Host-like names read as `env.NAME` (which also matches `process.env.NAME`).
fn env_read_host_names(text: &str) -> BTreeSet<String> {
    text.match_indices("env.")
        .map(|(at, _)| {
            text[at + 4..]
                .chars()
                .take_while(|&c| is_setting_char(c))
                .collect::<String>()
        })
        .filter(|name| names_a_host(name))
        .collect()
}

fn service_source() -> String {
    let mut files = Vec::new();
    files_under(&service_root().join("src"), &["rs"], &mut files);
    files_under(&service_root().join("domain/src"), &["rs"], &mut files);
    assert!(!files.is_empty(), "no service source found");
    files
        .iter()
        .map(|f| std::fs::read_to_string(f).expect("read source"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn ui_source() -> String {
    let mut files = Vec::new();
    files_under(
        &repo_root().join("project-portfolio-management-ui-with-svelte/src"),
        &["ts", "js", "svelte"],
        &mut files,
    );
    assert!(!files.is_empty(), "no operator-interface source found");
    files
        .iter()
        .filter(|f| {
            let name = f.file_name().and_then(|n| n.to_str()).unwrap_or("");
            !name.contains(".test.") && !name.contains(".spec.")
        })
        .map(|f| std::fs::read_to_string(f).expect("read source"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The backticked names in the first column of each table row in the guide's "Outbound hosts" section.
fn listed_in_guide() -> BTreeSet<String> {
    let guide = std::fs::read_to_string(repo_root().join("operations/deployment.md"))
        .expect("read operations/deployment.md");
    let start = guide
        .find("\n## Outbound hosts\n")
        .expect("operations/deployment.md has an \"Outbound hosts\" section");
    let rest = &guide[start + 1..];
    let section = &rest[..rest[3..].find("\n## ").map_or(rest.len(), |end| end + 3)];
    let mut names = BTreeSet::new();
    for line in section.lines().filter(|l| l.starts_with("| `")) {
        let first_cell = line.split('|').nth(1).unwrap_or("");
        for name in first_cell.split('`').skip(1).step_by(2) {
            names.insert(name.to_owned());
        }
    }
    assert!(
        names.len() >= 10,
        "the outbound-hosts tables look empty: {names:?}"
    );
    names
}

#[test]
fn every_outbound_host_setting_is_in_the_deployment_guide() {
    let service = service_source();
    let ui = ui_source();
    let listed = listed_in_guide();

    let mut read: BTreeSet<String> = quoted_host_names(&service);
    read.extend(env_read_host_names(&ui));
    for (name, evidence) in NAMED_BY_HAND {
        assert!(
            service.contains(evidence),
            "{name} is listed by hand, but the service no longer holds {evidence}"
        );
        read.insert((*name).to_owned());
    }

    let unlisted: Vec<_> = read.difference(&listed).collect();
    assert!(
        unlisted.is_empty(),
        "read but not in operations/deployment.md \"Outbound hosts\" (add each, or to the not-contacted table): {unlisted:?}"
    );
    let stale: Vec<_> = listed.difference(&read).collect();
    assert!(
        stale.is_empty(),
        "listed in operations/deployment.md \"Outbound hosts\" but read by nothing: {stale:?}"
    );
}

#[test]
fn the_scan_finds_what_it_should() {
    assert_eq!(
        quoted_host_names(r#"var("SMTP_HOST"); "PORT"; "a_URL"; "X_URL""#),
        BTreeSet::from(["SMTP_HOST".to_owned(), "X_URL".to_owned()])
    );
    assert_eq!(
        env_read_host_names("env.REDIS_URL ?? process.env.PUBLIC_SITE_URL; env.OIDC_SCOPES"),
        BTreeSet::from(["PUBLIC_SITE_URL".to_owned(), "REDIS_URL".to_owned()])
    );
}
