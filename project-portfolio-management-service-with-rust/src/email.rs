//! Sending email: the sender trait and the rules around it. The database side (one row per recipient) is
//! [`crate::controllers::email_deliveries`].
//!
//! Everything that sends email goes through a [`Sender`], so the transport can be swapped without touching the
//! callers: the SMTP transport the deployment configures (`SMTP_*`, which can be the bundled Stalwart server,
//! see `operations/email-with-stalwart.md`), nothing at all when no mailer is configured (a failure that
//! says so), or a recording one for tests. A sender reports an attempt as sent, **transient** (try again) or
//! **permanent** (do not).

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, OnceLock, PoisonError, RwLock};

use crate::datetime::{Duration, Timestamp};
use async_trait::async_trait;

/// The longest wait between two attempts to send an email, in minutes.
const MAX_BACKOFF_MINUTES: i64 = 360;

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

/// Everything a stub mailer has been asked to send.
#[derive(Debug, Clone, Default)]
pub struct Deliveries {
    /// How many messages it accepted.
    pub count: usize,
    /// Each message, rendered as subject then text.
    pub messages: Vec<String>,
}

/// The mailer the deployment configured.
#[derive(Clone)]
pub enum Mailer {
    /// A real SMTP transport.
    Smtp(lettre::AsyncSmtpTransport<lettre::Tokio1Executor>),
    /// Keeps what it was given, for tests.
    Stub(Arc<Mutex<Deliveries>>),
}

impl std::fmt::Debug for Mailer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Smtp(_) => "Mailer::Smtp",
            Self::Stub(_) => "Mailer::Stub",
        })
    }
}

impl Mailer {
    /// The mailer for this environment. Tests get a stub. Development talks to a local catcher on port 1025 in the
    /// clear. Everything else uses `SMTP_HOST` and `SMTP_PORT` with STARTTLS, with `SMTP_USER` and `SMTP_PASSWORD` when set.
    ///
    /// # Errors
    ///
    /// When the SMTP host is not usable.
    pub fn from_env(environment: &axa::Environment) -> axa::Result<Self> {
        Self::from_lookup(environment, |name| std::env::var(name).ok())
    }

    /// [`Mailer::from_env`] with the settings read through `lookup`, so a test need not change the process's
    /// environment.
    ///
    /// # Errors
    ///
    /// When the SMTP host is not usable.
    pub fn from_lookup(
        environment: &axa::Environment,
        lookup: impl Fn(&str) -> Option<String>,
    ) -> axa::Result<Self> {
        use lettre::transport::smtp::authentication::Credentials;
        use lettre::{AsyncSmtpTransport, Tokio1Executor};
        match environment {
            axa::Environment::Test => Ok(Self::Stub(Arc::default())),
            axa::Environment::Development => Ok(Self::Smtp(
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("localhost")
                    .port(1025)
                    .build(),
            )),
            _ => {
                let host = lookup("SMTP_HOST").unwrap_or_else(|| "localhost".to_string());
                let port = lookup("SMTP_PORT")
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(587);
                let mut builder = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&host)
                    .map_err(axa::Error::wrap)?
                    .port(port);
                let user = lookup("SMTP_USER").unwrap_or_default();
                if !user.is_empty() {
                    let password = lookup("SMTP_PASSWORD").unwrap_or_default();
                    builder = builder.credentials(Credentials::new(user, password));
                }
                Ok(Self::Smtp(builder.build()))
            }
        }
    }

    /// What a stub mailer has been given; empty for a real one.
    #[must_use]
    pub fn deliveries(&self) -> Deliveries {
        match self {
            Self::Smtp(_) => Deliveries::default(),
            Self::Stub(d) => d.lock().unwrap_or_else(PoisonError::into_inner).clone(),
        }
    }
}

/// Sends through the deployment's [`Mailer`].
pub struct SmtpSender {
    /// The deployment's configured mailer.
    mailer: Mailer,
}

impl SmtpSender {
    /// Wrap the configured mailer.
    #[must_use]
    pub fn new(mailer: Mailer) -> Self {
        Self { mailer }
    }
}

#[async_trait]
impl Sender for SmtpSender {
    fn name(&self) -> &'static str {
        "smtp"
    }

    async fn send(&self, m: &Message) -> Result<(), SendError> {
        use lettre::AsyncTransport;
        let refused = |e: String| SendError::Permanent(format!("not a sendable message: {e}"));
        let email = lettre::Message::builder()
            .from(
                m.from
                    .parse()
                    .map_err(|e: lettre::address::AddressError| refused(e.to_string()))?,
            )
            .to(m
                .to
                .parse()
                .map_err(|e: lettre::address::AddressError| refused(e.to_string()))?)
            .subject(m.subject.clone())
            .body(m.text.clone())
            .map_err(|e| refused(e.to_string()))?;
        match &self.mailer {
            Mailer::Smtp(transport) => transport
                .send(email)
                .await
                .map(|_| ())
                .map_err(|e| SendError::Transient(e.to_string())),
            Mailer::Stub(deliveries) => {
                let mut d = deliveries.lock().unwrap_or_else(PoisonError::into_inner);
                d.count += 1;
                d.messages.push(format!("{}\n{}", m.subject, m.text));
                Ok(())
            }
        }
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
    /// Messages accepted so far, in order.
    sent: Mutex<Vec<Message>>,
    /// Queued errors to return from the next attempts, front first, before any message is accepted.
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
        self.failures
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push_back(e);
    }

    /// What was sent successfully.
    #[must_use]
    pub fn sent(&self) -> Vec<Message> {
        self.sent
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

#[async_trait]
impl Sender for RecordingSender {
    fn name(&self) -> &'static str {
        "recording"
    }

    async fn send(&self, m: &Message) -> Result<(), SendError> {
        if let Some(e) = self
            .failures
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .pop_front()
        {
            return Err(e);
        }
        self.sent
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(m.clone());
        Ok(())
    }
}

/// The process-wide sender override, empty by default; only tests set it.
fn slot() -> &'static RwLock<Option<Arc<dyn Sender>>> {
    static SLOT: OnceLock<RwLock<Option<Arc<dyn Sender>>>> = OnceLock::new();
    SLOT.get_or_init(|| RwLock::new(None))
}

/// Replace the sender for the whole process (`None` restores the configured one). For tests.
pub fn override_sender(s: Option<Arc<dyn Sender>>) {
    *slot().write().unwrap_or_else(PoisonError::into_inner) = s;
}

/// The sender in force: the override if one is set, else the configured SMTP mailer, else [`NoSender`].
#[must_use]
pub fn sender_for(ctx: &axa::AppContext) -> Arc<dyn Sender> {
    if let Some(s) = slot()
        .read()
        .unwrap_or_else(PoisonError::into_inner)
        .clone()
    {
        return s;
    }
    match ctx.shared_store.get::<Mailer>() {
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
    pub next: Option<Timestamp>,
}

/// The state after an attempt, given its result and how many attempts have now been made. A transient
/// failure is retried after 1, 5, 25, 125 minutes (capped at six hours) until `MAX_ATTEMPTS`; a permanent
/// one, or the last, ends the delivery as failed.
#[must_use]
pub fn after(result: &Result<(), SendError>, attempts: i32, now: Timestamp) -> After {
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
            // 5^(attempt - 1) minutes, capped at 360 (six hours): a power past `i64` is past the cap.
            let minutes = 5_i64
                .checked_pow(u32::try_from(attempts.max(1) - 1).unwrap_or(0))
                .unwrap_or(MAX_BACKOFF_MINUTES)
                .min(MAX_BACKOFF_MINUTES);
            After {
                status: "retry",
                next: Duration::try_minutes(minutes).and_then(|wait| now.checked_add_signed(wait)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn each_environment_gets_its_mailer() {
        let none = |_: &str| None;
        let test = Mailer::from_lookup(&axa::Environment::Test, none).unwrap();
        assert_eq!(format!("{test:?}"), "Mailer::Stub");
        let development = Mailer::from_lookup(&axa::Environment::Development, none).unwrap();
        assert_eq!(format!("{development:?}"), "Mailer::Smtp");
        // A real transport keeps no record of what it sent.
        assert_eq!(development.deliveries().count, 0);

        // Production: the defaults, then every setting (a bad port falls back to 587).
        let production = axa::Environment::Production;
        assert!(Mailer::from_lookup(&production, none).is_ok());
        let settings = |name: &str| {
            Some(
                match name {
                    "SMTP_HOST" => "smtp.example.invalid",
                    "SMTP_PORT" => "not-a-port",
                    "SMTP_USER" => "relay",
                    _ => "secret",
                }
                .to_string(),
            )
        };
        let configured = Mailer::from_lookup(&production, settings).unwrap();
        assert_eq!(format!("{configured:?}"), "Mailer::Smtp");
    }

    #[tokio::test]
    async fn a_relay_that_cannot_be_reached_is_a_transient_failure() {
        // Port 1 on the loopback refuses the connection at once.
        let transport =
            lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::builder_dangerous("127.0.0.1")
                .port(1)
                .timeout(Some(std::time::Duration::from_secs(2)))
                .build();
        let sender = SmtpSender::new(Mailer::Smtp(transport));
        let message = Message {
            from: "service@example.com".into(),
            to: "ada@example.com".into(),
            subject: "Hello".into(),
            text: "Body".into(),
        };
        assert!(matches!(
            sender.send(&message).await,
            Err(SendError::Transient(_))
        ));
    }

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
        let now = Timestamp::now();
        let t = Err(SendError::Transient("down".into()));
        let one = after(&t, 1, now);
        assert_eq!(
            (one.status, one.next),
            (
                "retry",
                Some(now.checked_add_signed(Duration::minutes(1)).unwrap())
            )
        );
        assert_eq!(
            after(&t, 2, now).next,
            Some(now.checked_add_signed(Duration::minutes(5)).unwrap())
        );
        assert_eq!(
            after(&t, 3, now).next,
            Some(now.checked_add_signed(Duration::minutes(25)).unwrap())
        );
        assert_eq!(
            after(&t, MAX_ATTEMPTS, now).status,
            "failed",
            "the last attempt ends it"
        );
    }

    #[test]
    fn a_permanent_failure_ends_at_once_and_success_is_sent() {
        let now = Timestamp::now();
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

    #[tokio::test]
    async fn stub_smtp_sender_builds_messages_and_rejects_header_addresses() {
        let mailer = Mailer::from_env(&axa::Environment::Test).unwrap();
        assert_eq!(format!("{mailer:?}"), "Mailer::Stub");
        assert_eq!(mailer.deliveries().count, 0);
        let sender = SmtpSender::new(mailer.clone());
        assert_eq!(sender.name(), "smtp");
        let message = Message {
            from: "service@example.org".into(),
            to: "person@example.org".into(),
            subject: "Subject".into(),
            text: "Body".into(),
        };
        sender.send(&message).await.unwrap();
        assert_eq!(mailer.deliveries().messages, ["Subject\nBody"]);

        let mut invalid = message.clone();
        invalid.from = "not an address".into();
        assert!(matches!(
            sender.send(&invalid).await,
            Err(SendError::Permanent(_))
        ));
        invalid.from = message.from.clone();
        invalid.to = "still not an address".into();
        assert!(matches!(
            sender.send(&invalid).await,
            Err(SendError::Permanent(_))
        ));
        assert_eq!(NoSender.name(), "none");
        assert_eq!(RecordingSender::default().name(), "recording");
    }

    #[test]
    fn address_boundaries_and_long_retry_backoff_are_explicit() {
        assert!(address_ok(" a@b.com "));
        for bad in ["a@.example", "a@example.", "a;b@example.com"] {
            assert!(!address_ok(bad), "{bad:?}");
        }
        let too_long = format!("{}@example.com", "a".repeat(254));
        assert!(!address_ok(&too_long));
        let now = Timestamp::now();
        let transient = Err(SendError::Transient("down".into()));
        assert_eq!(
            after(&transient, -100, now).next,
            Some(now.checked_add_signed(Duration::minutes(1)).unwrap())
        );
        assert_eq!(after(&transient, 100, now).status, "failed");
    }
}
