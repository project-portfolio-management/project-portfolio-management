//! The standard files of a public repository are present and say what they must (NFR-20, PF-1).
//!
//! The list is the one in `spec/special-files-for-public-repos/index.md`. This test does not judge the
//! prose; it keeps a file from going missing or empty, and keeps the facts other documents rely
//! on (who the maintainer is, how to report a vulnerability, how to cite) from being dropped.
//! No database: it runs in a normal `cargo test`.

use std::path::{Path, PathBuf};

const MAINTAINER_EMAIL: &str = "joel@joelparkerhenderson.com";

const FILES: &[&str] = &[
    "README.md",
    "LICENSE.md",
    "CITATION.cff",
    "NEWS.md",
    "COMPARISONS.md",
    "BENCHMARKS.md",
    "INSTALL.md",
    "CONTRIBUTING.md",
    "RFC.md",
    "CODEOWNERS",
    "MAINTAINERS.md",
    "CHANGELOG.md",
    "AI_STATEMENT.md",
    "GOVERNANCE.md",
    "SECURITY.md",
];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crate sits inside the repository")
        .to_path_buf()
}

fn read(name: &str) -> String {
    std::fs::read_to_string(root().join(name)).unwrap_or_else(|e| panic!("read {name}: {e}"))
}

#[test]
fn every_special_file_exists_and_has_content() {
    for name in FILES {
        let text = read(name);
        // CODEOWNERS is legitimately one line.
        let least = if *name == "CODEOWNERS" { 20 } else { 100 };
        assert!(
            text.trim().len() > least,
            "{name} is missing or nearly empty"
        );
    }
}

#[test]
fn the_maintainer_and_the_security_contact_are_stated() {
    for name in [
        "MAINTAINERS.md",
        "CODEOWNERS",
        "SECURITY.md",
        "CITATION.cff",
    ] {
        assert!(
            read(name).contains(MAINTAINER_EMAIL),
            "{name} does not name {MAINTAINER_EMAIL}"
        );
    }
    assert!(read("MAINTAINERS.md").contains("Joel Parker Henderson"));
}

#[test]
fn codeowners_owns_everything() {
    assert!(
        read("CODEOWNERS")
            .lines()
            .any(|l| l.trim_start().starts_with("* ") && l.contains(MAINTAINER_EMAIL)),
        "CODEOWNERS has no `* <owner>` line"
    );
}

#[test]
fn the_citation_file_has_its_required_fields() {
    let text = read("CITATION.cff");
    for field in [
        "cff-version:",
        "title:",
        "message:",
        "authors:",
        "family-names:",
        "given-names:",
    ] {
        assert!(text.contains(field), "CITATION.cff lacks {field}");
    }
}

#[test]
fn the_ai_statement_states_the_practice_as_it_is() {
    let text = read("AI_STATEMENT.md");
    assert!(text.contains("ai-generated"), "levels are not stated");
    assert!(
        text.contains("Co-Authored-By"),
        "commit trailers are not disclosed"
    );
    assert!(text.contains("ships-ai-system: false"));
    assert!(
        text.contains("not yet done"),
        "the Welsh translation review status is not disclosed"
    );
}

#[test]
fn the_readme_links_to_every_special_file() {
    let readme = read("README.md");
    for name in FILES
        .iter()
        .filter(|n| !["README.md", "CODEOWNERS", "LICENSE.md"].contains(n))
    {
        assert!(readme.contains(name), "README.md does not link {name}");
    }
}
