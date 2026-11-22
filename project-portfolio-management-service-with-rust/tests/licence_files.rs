//! The repository's licence files and manifests agree (HD-1).
//!
//! Every licence text lives in the root `LICENSES/` directory, one
//! `<lowercase-spdx-id>/index.md` per SPDX identifier in the expression; every manifest declares the same
//! expression; the root `LICENSE.md` explains the choice and names every
//! licence in the expression; and no other licence file sits anywhere else. No database: these
//! run in a normal `cargo test`.

use std::path::{Path, PathBuf};

/// The one SPDX expression for the whole repository.
const EXPRESSION: &str = "MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only OR OGL-UK-3.0 OR CC-BY-NC-SA-4.0";

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

/// The directory name for an SPDX identifier: its lower-case form.
fn directory_name(id: &str) -> String {
    id.to_lowercase()
}

#[test]
fn every_licence_in_the_expression_has_an_index_in_the_licences_directory() {
    for id in EXPRESSION.split(" OR ") {
        let file = root()
            .join("LICENSES")
            .join(directory_name(id))
            .join("index.md");
        let text = std::fs::read_to_string(&file)
            .unwrap_or_else(|e| panic!("{} is missing: {e}", file.display()));
        assert!(
            text.contains("```text"),
            "{} holds no full text in a text fence",
            file.display()
        );
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
fn the_licences_directory_holds_only_the_expression_s_licences() {
    let ids: Vec<String> = EXPRESSION.split(" OR ").map(directory_name).collect();
    for entry in std::fs::read_dir(root().join("LICENSES")).expect("LICENSES/ exists") {
        let entry = entry.expect("a directory entry");
        let name = entry.file_name().to_string_lossy().to_string();
        assert!(
            entry.path().is_dir() && ids.contains(&name),
            "LICENSES/{name} is not a licence directory in the expression"
        );
        let mut files: Vec<String> = std::fs::read_dir(entry.path())
            .expect("read the licence directory")
            .map(|e| {
                e.expect("an entry")
                    .file_name()
                    .to_string_lossy()
                    .to_string()
            })
            .collect();
        files.sort();
        // `README.md` may be there as a symlink to `index.md`, so a viewer that opens a folder's README shows the
        // text. It is the same file, not a second licence text.
        if files == ["README.md", "index.md"] {
            let link = std::fs::read_link(entry.path().join("README.md"));
            assert_eq!(
                link.ok().as_deref(),
                Some(Path::new("index.md")),
                "LICENSES/{name}/README.md must be a symlink to index.md"
            );
            continue;
        }
        assert_eq!(
            files,
            ["index.md"],
            "LICENSES/{name} holds more than index.md (and a README.md symlink to it)"
        );
    }
}

#[test]
fn the_old_licence_directory_is_gone() {
    assert!(
        !root().join("LICENSE").exists(),
        "LICENSE/ was replaced by LICENSES/<id>/index.md and must not return"
    );
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
fn no_licence_file_lives_outside_the_licences_directory() {
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
                    ".git" | "target" | "node_modules" | ".svelte-kit" | "build" | "licenses"
                ) {
                    walk(&path, found);
                }
            } else if is_licence_name(&name) && path != root().join("LICENSE.md") {
                // The one file allowed outside `LICENSES/`: the root `LICENSE.md`
                // that explains the choice (and is not a licence text).
                found.push(path);
            }
        }
    }
    // What git would commit: tracked files and untracked ones it does not ignore. That leaves out a local checkout's
    // ignored folders (agent worktrees, editor state), which hold other copies of this repository and are not ours.
    // Without git (a source archive), walk the tree instead.
    let listed = std::process::Command::new("git")
        .args([
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ])
        .current_dir(root())
        .output()
        .ok()
        .filter(|out| out.status.success());
    let mut found = Vec::new();
    if let Some(out) = listed {
        for relative in out.stdout.split(|b| *b == 0).filter(|p| !p.is_empty()) {
            let path = root().join(String::from_utf8_lossy(relative).as_ref());
            let in_licence_dir = path.starts_with(root().join("LICENSES"));
            let name = path
                .file_name()
                .map_or_else(String::new, |n| n.to_string_lossy().to_lowercase());
            if !in_licence_dir && is_licence_name(&name) && path != root().join("LICENSE.md") {
                found.push(path);
            }
        }
    } else {
        walk(&root(), &mut found);
    }
    assert!(
        found.is_empty(),
        "licence files outside LICENSES/: {found:?}"
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
            text.contains(&format!("LICENSES/{}/index.md", directory_name(id))),
            "LICENSE.md does not link LICENSES/{}/index.md",
            directory_name(id)
        );
    }
}
