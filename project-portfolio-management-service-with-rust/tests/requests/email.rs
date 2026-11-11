//! Email through the sender trait, one delivery row per recipient: success, a transient failure that is
//! retried, a permanent failure (no mailer at all is unit-tested: the test config has a stub mailer), an address that is refused before any attempt, no mailer at all, and a
//! repeated dispatch that does not send twice.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`. The sender and the sender address are
//! process-wide, so the test restores both.

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::controllers::email_deliveries::{Delivered, deliver, run_due};
use project_portfolio_management::email::{self, RecordingSender, SendError};
use serde_json::Value;
use serial_test::serial;
use sqlx::Row;
use uuid::Uuid;

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn each_recipient_has_a_delivery_row_and_a_failure_is_recorded_not_lost() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let rec = RecordingSender::new();
        email::override_sender(Some(rec.clone()));
        // SAFETY: serial test; the setting is restored below.
        unsafe { std::env::set_var("PROJECT_PORTFOLIO_MANAGEMENT_EMAIL_FROM", "ppm@example.org") };
        let subject = Uuid::new_v4();
        let status = |who: String| {
            let db = project_portfolio_management::db::pool(&ctx);
            async move {
                let r = sqlx::query(sqlx::AssertSqlSafe(format!("SELECT status, attempts, sender, coalesce(last_error, '') AS err FROM email_deliveries WHERE recipient = '{who}'"))).fetch_optional(db).await.unwrap().unwrap();
                (r.try_get::<String, _>("status").unwrap(), r.try_get::<i32, _>("attempts").unwrap(), r.try_get::<Option<String>, _>("sender").unwrap(), r.try_get::<String, _>("err").unwrap())
            }
        };

        // Three recipients, three rows; one succeeds, one is down for now, one is refused for good.
        assert_eq!(deliver(&ctx, "test", subject, "ada@example.com", "Hello", "Body").await.unwrap(), Delivered::Sent);
        rec.fail_next(SendError::Transient("connection refused".into()));
        assert_eq!(deliver(&ctx, "test", subject, "bob@example.com", "Hello", "Body").await.unwrap(), Delivered::Retry("connection refused".into()));
        rec.fail_next(SendError::Permanent("550 no such user".into()));
        assert_eq!(deliver(&ctx, "test", subject, "cy@example.com", "Hello", "Body").await.unwrap(), Delivered::Failed("550 no such user".into()));
        assert_eq!(status("ada@example.com".into()).await, ("sent".into(), 1, Some("recording".into()), String::new()));
        assert_eq!(status("bob@example.com".into()).await.0, "retry");
        assert_eq!(status("cy@example.com".into()).await.0, "failed");
        assert_eq!(rec.sent().len(), 1, "only Ada's was sent");

        // Asking again for a message already sent sends nothing more.
        assert_eq!(deliver(&ctx, "test", subject, "ADA@example.com", "Hello", "Body").await.unwrap(), Delivered::Sent);
        assert_eq!(rec.sent().len(), 1, "no second email, even with the address in another case");

        // Bob's retry is not due yet; make it due and run the retry pass.
        assert_eq!(run_due(&ctx, 10).await.unwrap(), 0);
        sqlx::raw_sql("UPDATE email_deliveries SET next_attempt_at = now() - interval '1 minute' WHERE status = 'retry'").execute(project_portfolio_management::db::pool(&ctx)).await.unwrap();
        assert_eq!(run_due(&ctx, 10).await.unwrap(), 1);
        assert_eq!(status("bob@example.com".into()).await, ("sent".into(), 2, Some("recording".into()), String::new()), "two attempts; the error belongs to the last attempt, which succeeded");
        assert_eq!(rec.sent().len(), 2);

        // A malformed address is refused before any attempt, and says so.
        let bad = deliver(&ctx, "test", subject, "dee@example.com\r\nBcc: x@y.z", "Hello", "Body").await.unwrap();
        assert!(matches!(&bad, Delivered::Failed(w) if w.contains("malformed")), "{bad:?}");
        assert_eq!(rec.sent().len(), 2);

        // Without a sender address nothing is attempted.
        unsafe { std::env::remove_var("PROJECT_PORTFOLIO_MANAGEMENT_EMAIL_FROM") };
        let nofrom = deliver(&ctx, "test", Uuid::new_v4(), "fay@example.com", "Hello", "Body").await.unwrap();
        assert!(matches!(&nofrom, Delivered::Failed(w) if w.contains("EMAIL_FROM")), "{nofrom:?}");

        // The list is for administrators; auth is off in this test, so the caller is one.
        let list: Value = request.get("/api/email-deliveries?status=failed").await.json();
        assert!(list["deliveries"].as_array().unwrap().len() >= 3, "{list}");
        request.get("/api/email-deliveries?status=lost").await.assert_status(axum::http::StatusCode::UNPROCESSABLE_ENTITY);
        email::override_sender(None);
    })
    .await;
}
