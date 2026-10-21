//! Per-recipient email deliveries: every email goes to one recipient at a time through the [`Sender`] in
//! force ([`crate::email`]), and each attempt is written to `email_deliveries`.
//!
//! * [`deliver`] records and attempts one message to one recipient. Repeating it for the same message and
//!   recipient does not send twice (a delivery already `sent` stays so);
//! * [`run_due`] retries what is waiting after a transient failure;
//! * `GET /api/email-deliveries` (administrators only; anyone else gets 404) lists them, newest first.

use axum::http::StatusCode;
use chrono::Utc;
use loco_rs::prelude::*;
use sea_orm::{PaginatorTrait, QueryOrder, QuerySelect};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use super::db_err;
use crate::auth::MaybeAuthUser;
use crate::email::{self, Message, SendError};
use crate::models::_entities::email_deliveries as rows;

/// The routes.
pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/email-deliveries", get(list))
}

/// How a delivery attempt ended, for a caller that keeps its own record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delivered {
    /// The transport accepted it (now, or on an earlier attempt).
    Sent,
    /// Failed for now; try again later.
    Retry(String),
    /// Failed for good.
    Failed(String),
}

/// The sender address: `PROJECT_PORTFOLIO_MANAGEMENT_EMAIL_FROM`.
#[must_use]
pub fn from_address() -> Option<String> {
    std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_EMAIL_FROM")
        .ok()
        .filter(|f| !f.trim().is_empty())
}

/// Record and attempt one message to one recipient. The message is identified by `kind` and `subject_ref`:
/// asking again for the same pair and recipient updates the one row and never sends a message that was
/// already sent.
///
/// # Errors
///
/// When the database cannot be read or written; a failure to send is the returned [`Delivered`], recorded.
pub async fn deliver(
    ctx: &AppContext,
    kind: &str,
    subject_ref: Uuid,
    recipient: &str,
    subject: &str,
    text: &str,
) -> Result<Delivered> {
    let recipient = recipient.trim();
    let existing = rows::Entity::find()
        .filter(rows::Column::Kind.eq(kind))
        .filter(rows::Column::SubjectRef.eq(subject_ref))
        .filter(sea_orm::sea_query::Expr::cust_with_values(
            "lower(recipient) = lower($1)",
            [recipient.to_string()],
        ))
        .one(&ctx.db)
        .await
        .map_err(db_err)?;
    if existing.as_ref().is_some_and(|e| e.status == "sent") {
        return Ok(Delivered::Sent);
    }
    let attempts = existing.as_ref().map_or(0, |e| e.attempts) + 1;
    let from = from_address();
    let sender = email::sender_for(ctx);
    let result = match (&from, email::address_ok(recipient)) {
        (None, _) => Err(SendError::Permanent(
            "PROJECT_PORTFOLIO_MANAGEMENT_EMAIL_FROM is not set".to_string(),
        )),
        (Some(f), _) if !email::address_ok(f) => Err(SendError::Permanent(
            "the sender address is malformed".to_string(),
        )),
        (_, false) => Err(SendError::Permanent(
            "the recipient address is malformed".to_string(),
        )),
        (Some(f), true) => {
            sender
                .send(&Message {
                    from: f.clone(),
                    to: recipient.to_string(),
                    subject: subject.replace(['\r', '\n'], " "),
                    text: text.to_string(),
                })
                .await
        }
    };
    let now = Utc::now();
    let after = email::after(&result, attempts, now);
    let error = result.as_ref().err().map(|e| match e {
        SendError::Transient(m) | SendError::Permanent(m) => m.clone(),
    });
    match existing {
        Some(e) => {
            let mut a: rows::ActiveModel = e.into();
            a.status = ActiveValue::set(after.status.to_string());
            a.attempts = ActiveValue::set(attempts);
            a.last_error = ActiveValue::set(error.clone());
            a.sender = ActiveValue::set(Some(sender.name().to_string()));
            a.next_attempt_at = ActiveValue::set(after.next.map(Into::into));
            a.sent_at = ActiveValue::set((after.status == "sent").then(|| now.into()));
            a.update(&ctx.db).await.map_err(db_err)?;
        }
        None => {
            rows::ActiveModel {
                pid: ActiveValue::set(Uuid::new_v4()),
                kind: ActiveValue::set(kind.to_string()),
                subject_ref: ActiveValue::set(subject_ref),
                recipient: ActiveValue::set(recipient.to_string()),
                subject: ActiveValue::set(subject.replace(['\r', '\n'], " ")),
                body_text: ActiveValue::set(text.to_string()),
                status: ActiveValue::set(after.status.to_string()),
                attempts: ActiveValue::set(attempts),
                last_error: ActiveValue::set(error.clone()),
                sender: ActiveValue::set(Some(sender.name().to_string())),
                next_attempt_at: ActiveValue::set(after.next.map(Into::into)),
                sent_at: ActiveValue::set((after.status == "sent").then(|| now.into())),
                ..Default::default()
            }
            .insert(&ctx.db)
            .await
            .map_err(db_err)?;
        }
    }
    Ok(match (after.status, error) {
        ("sent", _) => Delivered::Sent,
        ("retry", e) => Delivered::Retry(e.unwrap_or_default()),
        (_, e) => Delivered::Failed(e.unwrap_or_default()),
    })
}

/// Retry what is due after a transient failure (at most `limit`), except what a caller retries itself (the
/// notification sinks keep their own schedule and call [`deliver`] again). Returns how many were sent.
///
/// # Errors
///
/// When a database read fails.
pub async fn run_due(ctx: &AppContext, limit: u64) -> Result<usize> {
    let due = rows::Entity::find()
        .filter(rows::Column::Status.eq("retry"))
        .filter(rows::Column::Kind.ne("sink"))
        .filter(rows::Column::NextAttemptAt.lte(Utc::now()))
        .order_by_asc(rows::Column::Id)
        .limit(limit)
        .all(&ctx.db)
        .await
        .map_err(db_err)?;
    let mut sent = 0;
    for d in due {
        if deliver(
            ctx,
            &d.kind,
            d.subject_ref,
            &d.recipient,
            &d.subject,
            &d.body_text,
        )
        .await?
            == Delivered::Sent
        {
            sent += 1;
        }
    }
    Ok(sent)
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    limit: Option<u64>,
}

/// `GET /api/email-deliveries[?status=][&limit=]` — administrators only; anyone else gets 404.
#[debug_handler]
async fn list(
    State(ctx): State<AppContext>,
    caller: MaybeAuthUser,
    Query(q): Query<ListQuery>,
) -> Result<Response> {
    let admin = caller.holds_or_open(|c| crate::auth::has_attr(c, "access", "admin"));
    if !admin {
        return Err(Error::NotFound);
    }
    let limit = q.limit.unwrap_or(100).clamp(1, 500);
    let mut find = rows::Entity::find().order_by_desc(rows::Column::Id);
    if let Some(s) = &q.status {
        if !["queued", "retry", "sent", "failed"].contains(&s.as_str()) {
            return Err(Error::CustomError(
                StatusCode::UNPROCESSABLE_ENTITY,
                loco_rs::controller::ErrorDetail::new(
                    "unprocessable_entity",
                    "status must be queued, retry, sent or failed",
                ),
            ));
        }
        find = find.filter(rows::Column::Status.eq(s.as_str()));
    }
    let list = find.limit(limit).all(&ctx.db).await.map_err(db_err)?;
    let count = |s: &str| {
        let ctx = ctx.clone();
        let s = s.to_string();
        async move {
            rows::Entity::find()
                .filter(rows::Column::Status.eq(s))
                .count(&ctx.db)
                .await
                .unwrap_or(0)
        }
    };
    format::json(json!({
        "deliveries": list.iter().map(|d| json!({
            "pid": d.pid, "kind": d.kind, "subject_ref": d.subject_ref, "recipient": d.recipient,
            "subject": d.subject, "status": d.status, "attempts": d.attempts, "last_error": d.last_error,
            "sender": d.sender, "next_attempt_at": d.next_attempt_at, "sent_at": d.sent_at,
        })).collect::<Vec<_>>(),
        "counts": { "retry": count("retry").await, "sent": count("sent").await, "failed": count("failed").await },
    }))
}
