//! Sending email: the sender trait and the rules around it. The database side (one row per recipient) is
//! [`crate::controllers::email_deliveries`].
//!
//! Everything that sends email goes through a [`Sender`], so the transport can be swapped without touching the
//! callers: the SMTP transport the deployment configures (`SMTP_*`, which can be the bundled Stalwart server,
//! see `operations/email-with-stalwart.md`), nothing at all when no mailer is configured (a failure that
//! says so), or a recording one for tests. A sender reports an attempt as sent, **transient** (try again) or
//! **permanent** (do not).

#![expect(clippy::missing_panics_doc)]

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, OnceLock, RwLock};

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};

/// One message to one recipient.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// The sender's address.
    pub from: String,
    /// The one recipient.
    pub to: String,
    /// The subject line (no line breaks).
    pub subject: String,
    /// The plain-text body.
    pub text: String,
}

/// Why an attempt did not deliver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SendError {
    /// Worth trying again (a connection failed, the server was busy).
    Transient(String),
    /// Not worth trying again (no mailer, a refused address).
    Permanent(String),
}

/// A way of sending one message.
#[async_trait]
pub trait Sender: Send + Sync {
    /// A short name recorded with the attempt (`smtp`, `none`, `recording`).
    fn name(&self) -> &'static str;
    /// Try to send. `Ok` means the transport accepted it.
    async fn send(&self, message: &Message) -> Result<(), SendError>;
}

/// The mailer the deployment configured (loco's SMTP transport).
pub struct SmtpSender {
    mailer: loco_rs::mailer::EmailSender,
}

impl SmtpSender {
    /// Wrap the configured mailer.
    #[must_use]
    pub fn new(mailer: loco_rs::mailer::EmailSender) -> Self {
        Self { mailer }
    }
}

#[async_trait]
impl Sender for SmtpSender {
    fn name(&self) -> &'static str {
        "smtp"
    }

    async fn send(&self, m: &Message) -> Result<(), SendError> {
        let email = loco_rs::mailer::Email {
            from: Some(m.from.clone()),
            to: m.to.clone(),
            reply_to: None,
            subject: m.subject.clone(),
            text: m.text.clone(),
            html: String::new(),
            bcc: None,
            cc: None,
            headers: None,
        };
        self.mailer
            .mail(&email)
            .await
            .map_err(|e| SendError::Transient(e.to_string()))
    }
}

/// No mailer is configured: every attempt fails for good, and says why.
pub struct NoSender;

#[async_trait]
impl Sender for NoSender {
    fn name(&self) -> &'static str {
        "none"
    }

    async fn send(&self, _: &Message) -> Result<(), SendError> {
        Err(SendError::Permanent(
            "no mailer is configured (set SMTP_HOST and the other SMTP settings)".to_string(),
        ))
    }
}

/// A sender for tests: it keeps what it was asked to send and can be told to fail the next attempts.
#[derive(Default)]
pub struct RecordingSender {
    sent: Mutex<Vec<Message>>,
    failures: Mutex<VecDeque<SendError>>,
}

impl RecordingSender {
    /// A new, empty recorder.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Fail the next attempt with `e` (queued, in order).
    pub fn fail_next(&self, e: SendError) {
        self.failures.lock().expect("lock").push_back(e);
    }

    /// What was sent successfully.
    #[must_use]
    pub fn sent(&self) -> Vec<Message> {
        self.sent.lock().expect("lock").clone()
    }
}

#[async_trait]
impl Sender for RecordingSender {
    fn name(&self) -> &'static str {
        "recording"
    }

    async fn send(&self, m: &Message) -> Result<(), SendError> {
        if let Some(e) = self.failures.lock().expect("lock").pop_front() {
            return Err(e);
        }
        self.sent.lock().expect("lock").push(m.clone());
        Ok(())
    }
}

fn slot() -> &'static RwLock<Option<Arc<dyn Sender>>> {
    static SLOT: OnceLock<RwLock<Option<Arc<dyn Sender>>>> = OnceLock::new();
    SLOT.get_or_init(|| RwLock::new(None))
}

/// Replace the sender for the whole process (`None` restores the configured one). For tests.
pub fn override_sender(s: Option<Arc<dyn Sender>>) {
    *slot().write().expect("lock") = s;
}

/// The sender in force: the override if one is set, else the configured SMTP mailer, else [`NoSender`].
#[must_use]
pub fn sender_for(ctx: &loco_rs::app::AppContext) -> Arc<dyn Sender> {
    if let Some(s) = slot().read().expect("lock").clone() {
        return s;
    }
    match ctx.mailer.clone() {
        Some(m) => Arc::new(SmtpSender::new(m)),
        None => Arc::new(NoSender),
    }
}

/// Most attempts at one delivery.
pub const MAX_ATTEMPTS: i32 = 5;

/// A recipient and a sender address are plausible (one address, no line breaks, an `@` with something either
/// side). A check against injection into a header, not a validation of deliverability.
#[must_use]
pub fn address_ok(a: &str) -> bool {
    let a = a.trim();
    !a.is_empty()
        && a.len() <= 254
        && !a.contains(['\r', '\n', ',', ';', ' ', '<', '>'])
        && a.split_once('@').is_some_and(|(l, d)| {
            !l.is_empty() && d.contains('.') && !d.starts_with('.') && !d.ends_with('.')
        })
}

/// What an attempt leaves the delivery as: its new status and, for a retry, when to try again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct After {
    /// `sent`, `retry` or `failed`.
    pub status: &'static str,
    /// The next attempt, for a retry.
    pub next: Option<DateTime<Utc>>,
}

/// The state after an attempt, given its result and how many attempts have now been made. A transient
/// failure is retried after 1, 5, 25, 125 minutes (capped at six hours) until [`MAX_ATTEMPTS`]; a permanent
/// one, or the last, ends the delivery as failed.
#[must_use]
pub fn after(result: &Result<(), SendError>, attempts: i32, now: DateTime<Utc>) -> After {
    match result {
        Ok(()) => After {
            status: "sent",
            next: None,
        },
        Err(SendError::Permanent(_)) => After {
            status: "failed",
            next: None,
        },
        Err(SendError::Transient(_)) if attempts >= MAX_ATTEMPTS => After {
            status: "failed",
            next: None,
        },
        Err(SendError::Transient(_)) => {
            let minutes = 5_i64
                .saturating_pow(u32::try_from(attempts.max(1) - 1).unwrap_or(0))
                .min(360);
            After {
                status: "retry",
                next: Some(now + Duration::minutes(minutes)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_with_line_breaks_lists_or_names_are_refused() {
        assert!(address_ok("ada@example.com"));
        assert!(!address_ok("ada@example.com\r\nBcc: x@y.z"));
        assert!(!address_ok("a@example.com, b@example.com"));
        assert!(!address_ok("Ada <ada@example.com>"));
        assert!(!address_ok("ada@localhost"));
        assert!(!address_ok(""));
    }

    #[test]
    fn a_transient_failure_retries_with_backoff_then_gives_up() {
        let now = Utc::now();
        let t = Err(SendError::Transient("down".into()));
        let one = after(&t, 1, now);
        assert_eq!(
            (one.status, one.next),
            ("retry", Some(now + Duration::minutes(1)))
        );
        assert_eq!(after(&t, 2, now).next, Some(now + Duration::minutes(5)));
        assert_eq!(after(&t, 3, now).next, Some(now + Duration::minutes(25)));
        assert_eq!(
            after(&t, MAX_ATTEMPTS, now).status,
            "failed",
            "the last attempt ends it"
        );
    }

    #[test]
    fn a_permanent_failure_ends_at_once_and_success_is_sent() {
        let now = Utc::now();
        assert_eq!(
            after(&Err(SendError::Permanent("no".into())), 1, now).status,
            "failed"
        );
        assert_eq!(
            after(&Ok(()), 1, now),
            After {
                status: "sent",
                next: None
            }
        );
    }

    #[tokio::test]
    async fn the_recorder_keeps_what_it_sends_and_fails_on_request() {
        let r = RecordingSender::new();
        let m = Message {
            from: "a@x.org".into(),
            to: "b@x.org".into(),
            subject: "s".into(),
            text: "t".into(),
        };
        r.fail_next(SendError::Transient("busy".into()));
        assert_eq!(r.send(&m).await, Err(SendError::Transient("busy".into())));
        assert!(r.send(&m).await.is_ok());
        assert_eq!(r.sent(), [m]);
        assert!(matches!(
            NoSender
                .send(&Message {
                    from: String::new(),
                    to: String::new(),
                    subject: String::new(),
                    text: String::new()
                })
                .await,
            Err(SendError::Permanent(_))
        ));
    }
}
