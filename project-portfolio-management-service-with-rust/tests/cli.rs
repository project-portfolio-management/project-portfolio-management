//! Exercise the installed operator entry points, rather than only calling
//! the same implementation through the test router. Each command gets a
//! fresh process, as it does in production, and inherits LLVM's profile
//! destination when coverage is running.

use std::process::{Command, Output};

fn run(args: &[&str], search_path: &std::path::Path) -> Output {
    let executable = env!("CARGO_BIN_EXE_project-portfolio-management");
    Command::new(executable)
        .args(args)
        .env("APP_ENV", "test")
        .env("OTLP_ENDPOINT", "")
        .env(
            "DB_URL",
            std::env::var("DB_URL").expect("DB_URL for the database CLI test"),
        )
        .env(
            "PROJECT_PORTFOLIO_MANAGEMENT_SEARCH_INDEX_PATH",
            search_path,
        )
        .output()
        .unwrap_or_else(|error| panic!("run {executable} {args:?}: {error}"))
}

fn assert_ok(output: &Output, command: &str) {
    assert!(
        output.status.success(),
        "{command} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
fn operator_cli_checks_migrations_lists_tasks_and_rebuilds_search() {
    let search_path = std::env::temp_dir().join(format!("ppm-cli-search-{}", std::process::id()));

    let check = run(&["check"], &search_path);
    assert_ok(&check, "check");

    let migrate = run(&["db", "migrate"], &search_path);
    assert_ok(&migrate, "db migrate");
    let status = run(&["db", "status"], &search_path);
    assert_ok(&status, "db status");

    let tasks = run(&["task"], &search_path);
    assert_ok(&tasks, "task");
    assert!(String::from_utf8_lossy(&tasks.stdout).contains("search_reindex"));

    let invalid = run(&["task", "search_reindex", "batch:nope"], &search_path);
    assert!(!invalid.status.success());
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("batch must be a positive integer"));

    let reindex = run(&["task", "search_reindex", "batch:1"], &search_path);
    assert_ok(&reindex, "task search_reindex");
    assert!(String::from_utf8_lossy(&reindex.stdout).contains("reindexed"));

    let _ = std::fs::remove_dir_all(search_path);
}

/// `check` as production: without a deployment name it refuses, naming the setting, before reading anything else; with
/// a bad one it refuses for the name's form; with a good one it gets past this check (and may stop on a later one).
#[test]
fn production_refuses_to_start_without_a_deployment_name() {
    let check = |name: Option<&str>| {
        let mut command = Command::new(env!("CARGO_BIN_EXE_project-portfolio-management"));
        command
            .arg("check")
            .env("APP_ENV", "production")
            .env("OTLP_ENDPOINT", "")
            .env_remove("PROJECT_PORTFOLIO_MANAGEMENT_DEPLOYMENT_ID");
        if let Some(name) = name {
            command.env("PROJECT_PORTFOLIO_MANAGEMENT_DEPLOYMENT_ID", name);
        }
        let output = command.output().expect("run check");
        let said = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        (output.status.success(), said)
    };
    let (ok, said) = check(None);
    assert!(
        !ok && said.contains("PROJECT_PORTFOLIO_MANAGEMENT_DEPLOYMENT_ID is required"),
        "{said}"
    );
    let (ok, said) = check(Some("Acme Example"));
    assert!(!ok && said.contains("lowercase letters"), "{said}");
    let (_, said) = check(Some("acme-example"));
    assert!(
        !said.contains("PROJECT_PORTFOLIO_MANAGEMENT_DEPLOYMENT_ID"),
        "{said}"
    );
}
