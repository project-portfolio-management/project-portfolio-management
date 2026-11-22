//! `project-portfolio-management-service` binary entrypoint: parses the command line and runs
//! [`App::run`]. See the crate library docs for the service overview.

#![forbid(unsafe_code)]
// Production code does not panic (spec/rust-checked-arithmetic, RCA-P-1 and RCA-NONE-1): an `Option` or a `Result` is handled, never unwrapped. Tests may.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable,
        clippy::todo,
        clippy::unimplemented
    )
)]
#![deny(missing_docs)]

#[cfg(target_env = "musl")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use project_portfolio_management::app::App;

#[tokio::main]
async fn main() -> axa::Result<()> {
    App::run(axa::cli::parse(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION"),
    ))
    .await
}
