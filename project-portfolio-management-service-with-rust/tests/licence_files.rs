//! The repository's licence files and manifests agree (HD-1).
//!
//! Every licence file lives in the root `LICENSE/` directory, one file per
//! SPDX identifier in the expression; every manifest declares the same
//! expression; the root `LICENSE.md` explains the choice and names every
//! licence in the expression; and no other licence file sits anywhere else. No database: these
//! run in a normal `cargo test`.

use std::path::{Path, PathBuf};

/// The one SPDX expression for the whole repository.
const EXPRESSION: &str = "MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only";

/// The repository root (this crate's parent directory).
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crate sits inside the repository")
        .to_path_buf()
}

/// The `license` value of a manifest line such as `license = "…"` or
/// `"license": "…",`.
fn declared(manifest: &Path) -> String {
    let text = std::fs::read_to_string(manifest)
        .unwrap_or_else(|e| panic!("read {}: {e}", manifest.display()));
    let line = text
        .lines()
        .find(|l| {
            let t = l.trim_start();
            t.starts_with("license ") || t.starts_with("license=") || t.starts_with("\"license\"")
        })
        .unwrap_or_else(|| panic!("{} declares no license", manifest.display()));
    let value = line.split_once(['=', ':']).expect("a key and a value").1;
    value
        .trim()
        .trim_end_matches(',')
        .trim_matches('"')
        .to_string()
}

#[test]
fn every_licence_in_the_expression_has_a_file_in_the_licence_directory() {
    for id in EXPRESSION.split(" OR ") {
        let file = root().join("LICENSE").join(format!("{id}.txt"));
        let text = std::fs::read_to_string(&file)
            .unwrap_or_else(|e| panic!("{} is missing: {e}", file.display()));
        // MIT and BSD carry the copyright line in the licence itself; the GPL
        // and Apache placeholders sit in their "how to apply" appendices and
        // are part of the published text.
        if matches!(id, "MIT" | "BSD-3-Clause") {
            assert!(
                !text.contains("<year>")
                    && !text.contains("<owner>")
                    && !text.contains("<copyright holders>"),
                "{} still has a template placeholder",
                file.display()
            );
        }
    }
}

#[test]
fn the_licence_directory_holds_only_the_expression_s_licences() {
    let ids: Vec<String> = EXPRESSION
        .split(" OR ")
        .map(|id| format!("{id}.txt"))
        .collect();
    for entry in std::fs::read_dir(root().join("LICENSE")).expect("LICENSE/ exists") {
        let name = entry.expect("a directory entry").file_name();
        let name = name.to_string_lossy();
        assert!(
            ids.iter().any(|id| *id == name),
            "LICENSE/{name} is not in the expression"
        );
    }
}

#[test]
fn every_manifest_declares_the_one_expression() {
    for manifest in [
        "project-portfolio-management-service-with-rust/Cargo.toml",
        "project-portfolio-management-matcher-with-rust/Cargo.toml",
        "project-portfolio-management-ui-with-svelte/package.json",
    ] {
        assert_eq!(declared(&root().join(manifest)), EXPRESSION, "{manifest}");
    }
}

/// `LICENSE`, `LICENCE.md`, `COPYING`, `LICENSE-MIT`, `license.txt` and the
/// like (lower-cased); not a file that merely mentions licences in its name.
fn is_licence_name(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or(name);
    ["license", "licence", "copying"]
        .iter()
        .any(|base| stem == *base || stem.starts_with(&format!("{base}-")))
}

#[test]
fn no_licence_file_lives_outside_the_licence_directory() {
    fn walk(dir: &Path, found: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if path.is_dir() {
                // Build output, dependencies and git internals are not ours.
                if !matches!(
                    name.as_str(),
                    ".git" | "target" | "node_modules" | ".svelte-kit" | "build" | "license"
                ) {
                    walk(&path, found);
                }
            } else if is_licence_name(&name) && path != root().join("LICENSE.md") {
                // The one file allowed outside `LICENSE/`: the root `LICENSE.md`
                // that explains the choice (and is not a licence text).
                found.push(path);
            }
        }
    }
    let mut found = Vec::new();
    walk(&root(), &mut found);
    assert!(
        found.is_empty(),
        "licence files outside LICENSE/: {found:?}"
    );
}

#[test]
fn the_root_explainer_names_every_licence_and_links_its_text() {
    let text = std::fs::read_to_string(root().join("LICENSE.md")).expect("LICENSE.md exists");
    assert!(
        text.contains(EXPRESSION),
        "LICENSE.md must state the SPDX expression"
    );
    for id in EXPRESSION.split(" OR ") {
        assert!(
            text.contains(&format!("LICENSE/{id}.txt")),
            "LICENSE.md does not link LICENSE/{id}.txt"
        );
    }
}
