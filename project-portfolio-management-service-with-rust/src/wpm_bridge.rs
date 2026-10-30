//! The bridge to the workforce service (WPM, which is the worker service), FB-1 to FB-3 and the entity spec §8.8.
//!
//! The family contract (2026-10-30): a PPM plan is named in WPM as `thing:<plan pid>`; a PPM skill pool may carry
//! `peer_pool_ref`, WPM's id for the pool that means the same skills, set by a person and never matched by name;
//! for a linked pool **supply is WPM's and demand is PPM's**, and the two apps' figures are never added together.
//!
//! This module holds the pure rules (the event id, FTE in hundredths, the reading of a supply figure) and the
//! two outbound calls:
//!
//! - `GET {base}/api/capacity/peer-supply?pool=<wpm pool>&from=YYYY-MM-01&months=N`: WPM's supply for a linked
//!   pool, by month, in hundredths of an FTE, a month `null` (hidden) under WPM's small-cell floor.
//! - `POST {base}/api/capacity/peer-demand` with `{pool_pid, programme_ref, month, fte_centi, peer_event_id}`.
//!
//! Both go out **under the caller's own bearer token** (WPM-D135: no shared service account), with a short
//! timeout and no redirects, to the one operator-configured base URL ([`crate::workers_client::BASE_URL_ENV`],
//! the same worker service). **Stub-first:** with no base URL, no token, or a pool that is not linked, nothing
//! is called and the answer says why. A PPM request never fails because WPM is down or slow.
//!
//! Supply figures received are kept **in this process** (not in the database) so that, when WPM cannot be
//! reached, the last figure is shown with its age, or `unknown` when there is none. They are lost at restart,
//! and one caller's figure is shown to the next caller of this PPM (stated in the spec as a limit).

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use uuid::Uuid;

/// How long PPM waits for WPM on any one call.
pub const PEER_TIMEOUT: Duration = Duration::from_secs(2);

/// The base URL of the worker service (WPM), read from [`crate::workers_client::BASE_URL_ENV`]; `None` is stub
/// mode.
#[must_use]
pub fn base_url() -> Option<String> {
    std::env::var(crate::workers_client::BASE_URL_ENV)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

/// The caller's bearer token from the request, to forward to WPM unchanged (WPM-D135).
#[must_use]
pub fn bearer_of(headers: &axum::http::HeaderMap) -> Option<String> {
    let h = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    let t = h
        .strip_prefix("Bearer ")
        .or_else(|| h.strip_prefix("bearer "))?
        .trim();
    (!t.is_empty()).then(|| t.to_string())
}

/// Whether `text` is a WPM pool id as WPM writes it: a lower-case hyphenated UUID (the same rule WPM applies to
/// its own `peer_pool_ref`).
#[must_use]
pub fn peer_pool_ref_ok(text: &str) -> bool {
    text.len() == 36 && Uuid::parse_str(text).is_ok_and(|u| u.hyphenated().to_string() == text)
}

/// How a PPM plan is named in WPM.
#[must_use]
pub fn programme_ref(plan_pid: Uuid) -> String {
    format!("thing:{plan_pid}")
}

/// Thousandths of an FTE to hundredths, rounded half up (`round(committed_fte × 100)`).
#[must_use]
pub fn centi_of_milli(milli: i64) -> i64 {
    (milli.max(0) + 5) / 10
}

/// Hundredths of an FTE to thousandths.
#[must_use]
pub fn milli_of_centi(centi: i64) -> i64 {
    centi.saturating_mul(10)
}

/// The id of one version of a programme's demand on a pool for a month: the pool, the plan, the month and the
/// moment of the write that made it. A retry of the same send carries the same id (WPM then changes nothing);
/// a new write is a new id.
#[must_use]
pub fn peer_event_id(
    pool: Uuid,
    plan: Uuid,
    month: NaiveDate,
    written_at: DateTime<Utc>,
) -> String {
    format!(
        "ppm:{pool}:{plan}:{}:{}",
        month.format("%Y-%m"),
        written_at.timestamp_micros()
    )
}

/// What PPM does with a programme's demand on a linked pool for a month, decided before any call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DemandPlan {
    /// Send this many hundredths of an FTE.
    Send(i64),
    /// Do not send, with the outcome to record and why.
    Skip(&'static str, &'static str),
}

/// Decide what to send for a programme's total demand (thousandths of an FTE) on a linked pool, with or without
/// a base URL and a caller token. **Zero is not sent**: WPM takes 1 to 1 000 000 hundredths and has no route to
/// withdraw a figure, so WPM keeps the last figure sent until it has one (recorded as `not_sent_zero`).
#[must_use]
pub fn plan_demand(total_milli: i64, configured: bool, has_token: bool) -> DemandPlan {
    let centi = centi_of_milli(total_milli);
    if centi < 1 {
        return DemandPlan::Skip(
            "not_sent_zero",
            "WPM takes no zero and has no route to withdraw a figure; WPM keeps the last figure sent",
        );
    }
    if centi > 1_000_000 {
        return DemandPlan::Skip(
            "not_sent_too_large",
            "WPM takes at most 1 000 000 hundredths of an FTE",
        );
    }
    if !configured {
        return DemandPlan::Skip(
            "not_configured",
            "the worker service URL is not set, so nothing was sent",
        );
    }
    if !has_token {
        return DemandPlan::Skip(
            "not_sent_no_token",
            "the request carried no bearer token to forward, and WPM is called only under the caller's own",
        );
    }
    DemandPlan::Send(centi)
}

/// One month of supply as PPM shows it for a linked pool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupplyReading {
    /// `live` (just received), `last_received` (WPM could not be reached now), `hidden` (WPM hides the month under
    /// its floor) or `unknown` (nothing received).
    pub status: &'static str,
    /// Thousandths of an FTE; `None` when hidden or unknown.
    pub milli: Option<i64>,
    /// When WPM took the figure.
    pub as_of: Option<DateTime<Utc>>,
    /// Why there is no live figure.
    pub reason: Option<String>,
}

/// One month as received from WPM: hundredths, or `None` when hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Received {
    /// Hundredths of an FTE; `None` when WPM hides the month.
    pub centi: Option<i64>,
    /// When WPM took the figure.
    pub as_of: DateTime<Utc>,
}

/// Read one month: a live figure when this call succeeded, else the last one received with its time, else
/// `unknown`. Never zero for a figure that is not there.
#[must_use]
pub fn read_month(
    live: Option<Received>,
    last: Option<Received>,
    why_not_live: Option<&str>,
) -> SupplyReading {
    let (r, fresh) = match (live, last) {
        (Some(r), _) => (r, true),
        (None, Some(r)) => (r, false),
        (None, None) => {
            return SupplyReading {
                status: "unknown",
                milli: None,
                as_of: None,
                reason: Some(
                    why_not_live
                        .unwrap_or("no figure received from WPM")
                        .to_string(),
                ),
            };
        }
    };
    let reason = (!fresh).then(|| {
        why_not_live
            .unwrap_or("WPM could not be reached")
            .to_string()
    });
    match r.centi {
        None => SupplyReading {
            status: "hidden",
            milli: None,
            as_of: Some(r.as_of),
            reason: Some(reason.unwrap_or_else(|| {
                "WPM hides a month made up of fewer workers than its floor".to_string()
            })),
        },
        Some(c) => SupplyReading {
            status: if fresh { "live" } else { "last_received" },
            milli: Some(milli_of_centi(c)),
            as_of: Some(r.as_of),
            reason,
        },
    }
}

// ---- the in-process store of figures received ---------------------------------------------------------------

type Store = Mutex<HashMap<(String, NaiveDate), Received>>;

fn store() -> &'static Store {
    static STORE: OnceLock<Store> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn remember(peer_pool_ref: &str, month: NaiveDate, r: Received) {
    let mut g = store()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    g.insert((peer_pool_ref.to_string(), month), r);
}

fn recall(peer_pool_ref: &str, month: NaiveDate) -> Option<Received> {
    let g = store()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    g.get(&(peer_pool_ref.to_string(), month)).copied()
}

// ---- the calls ----------------------------------------------------------------------------------------------

fn client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(PEER_TIMEOUT)
            .connect_timeout(PEER_TIMEOUT)
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("build the non-redirecting WPM client")
    })
}

async fn allowed(base: &str) -> Result<(), String> {
    if !crate::webhooks::target_allowed(base, crate::webhooks::outbound_strict()) {
        return Err(
            "the worker service URL is not https:// (or http:// on loopback), or is not public in strict mode"
                .to_string(),
        );
    }
    match crate::webhooks::check_resolution(base).await {
        Ok(()) => Ok(()),
        Err(true) => {
            Err("the worker service resolves to a non-public address (strict mode)".to_string())
        }
        Err(false) => Err("the worker service name could not be resolved".to_string()),
    }
}

#[derive(Deserialize)]
struct SupplyBody {
    as_of: DateTime<Utc>,
    months: Vec<SupplyMonth>,
}

#[derive(Deserialize)]
struct SupplyMonth {
    month: NaiveDate,
    supply_centi: Option<i64>,
}

/// Why a supply read failed. A refusal is WPM's answer about *this* caller, so it must not be met with a figure
/// another caller's token fetched (WPM-D135); only a peer that could not be reached falls back to the last figure.
#[derive(Debug, PartialEq, Eq)]
enum Fetch {
    Unreachable(String),
    Refused(String),
}

async fn fetch_supply(
    base: &str,
    bearer: &str,
    peer_pool_ref: &str,
    from: NaiveDate,
    months: u32,
) -> Result<Vec<(NaiveDate, Received)>, Fetch> {
    allowed(base).await.map_err(Fetch::Unreachable)?;
    // Every part is a UUID, a date or a number, so nothing needs escaping.
    let url = format!(
        "{}/api/capacity/peer-supply?pool={peer_pool_ref}&from={}&months={months}",
        base.trim_end_matches('/'),
        from.format("%Y-%m-%d")
    );
    let response = client()
        .get(&url)
        .bearer_auth(bearer)
        .send()
        .await
        .map_err(|e| {
            Fetch::Unreachable(if e.is_timeout() {
                "WPM did not answer in time".to_string()
            } else {
                "WPM could not be reached".to_string()
            })
        })?;
    let status = response.status();
    if status.is_client_error() {
        return Err(Fetch::Refused(format!(
            "WPM refused this caller ({})",
            status.as_u16()
        )));
    }
    if !status.is_success() {
        return Err(Fetch::Unreachable(format!(
            "WPM answered {}",
            status.as_u16()
        )));
    }
    let body: SupplyBody = response
        .json()
        .await
        .map_err(|_| Fetch::Unreachable("WPM's answer could not be read".to_string()))?;
    Ok(body
        .months
        .into_iter()
        .map(|m| {
            (
                m.month,
                Received {
                    centi: m.supply_centi,
                    as_of: body.as_of,
                },
            )
        })
        .collect())
}

/// Whether to call WPM for supply on this read.
#[derive(Debug, Clone)]
pub enum SupplyRead {
    /// Show only figures already received (the director pack, the drum, the start check): no call.
    Kept,
    /// Ask WPM now under this caller's token (or none, which makes no call).
    Live(Option<String>),
}

/// The supply of each linked pool for each month of the window, keyed by PPM pool pid. Calls go out in
/// parallel, each bounded by [`PEER_TIMEOUT`]; every failure is a reason on the reading, never an error.
pub async fn supply_for(
    linked: &[(Uuid, String)],
    months: &[NaiveDate],
    read: &SupplyRead,
) -> HashMap<Uuid, Vec<SupplyReading>> {
    let base = base_url();
    let mut set = tokio::task::JoinSet::new();
    let first = months.first().copied();
    let n = u32::try_from(months.len()).unwrap_or(u32::MAX);
    for (pool, peer) in linked {
        let (pool, peer, base, read) = (*pool, peer.clone(), base.clone(), read.clone());
        set.spawn(async move {
            let live = match (&read, base.as_deref(), first) {
                (SupplyRead::Kept, _, _) => Err(Fetch::Unreachable(
                    "figures kept from earlier reads only on this view".to_string(),
                )),
                (SupplyRead::Live(_), None, _) => Err(Fetch::Unreachable(
                    "the worker service URL is not set".to_string(),
                )),
                (SupplyRead::Live(None), Some(_), _) => Err(Fetch::Unreachable(
                    "the request carried no bearer token to forward to WPM".to_string(),
                )),
                (SupplyRead::Live(Some(_)), Some(_), None) => Ok(Vec::new()),
                (SupplyRead::Live(Some(t)), Some(b), Some(f)) => {
                    fetch_supply(b, t, &peer, f, n).await
                }
            };
            (pool, peer, live)
        });
    }
    let mut out = HashMap::new();
    while let Some(Ok((pool, peer, live))) = set.join_next().await {
        let (got, why, refused): (HashMap<NaiveDate, Received>, Option<String>, bool) = match live {
            Ok(rows) => {
                for (m, r) in &rows {
                    remember(&peer, *m, *r);
                }
                (rows.into_iter().collect(), None, false)
            }
            Err(Fetch::Unreachable(e)) => (HashMap::new(), Some(e), false),
            Err(Fetch::Refused(e)) => (HashMap::new(), Some(e), true),
        };
        let readings = months
            .iter()
            .map(|m| {
                let why = why.as_deref().or(if got.is_empty() {
                    None
                } else {
                    Some("WPM gave no figure for this month")
                });
                let last = if refused { None } else { recall(&peer, *m) };
                read_month(got.get(m).copied(), last, why)
            })
            .collect();
        out.insert(pool, readings);
    }
    out
}

/// The demand to send for one write, with everything the send needs (the token is held in memory only).
#[derive(Debug, Clone)]
pub struct DemandSend {
    /// The worker service's base URL.
    pub base: String,
    /// The caller's own bearer token, forwarded unchanged.
    pub bearer: String,
    /// WPM's pool id.
    pub peer_pool_ref: String,
    /// The PPM plan (the programme), sent as `thing:<pid>`.
    pub plan_pid: Uuid,
    /// The first day of the month.
    pub month: NaiveDate,
    /// Hundredths of an FTE.
    pub fte_centi: i64,
    /// The id of this version of the figure.
    pub peer_event_id: String,
}

/// Send one programme's demand to WPM. Returns the outcome to record (`sent`, `duplicate`, `refused`,
/// `unreachable`) and a detail.
pub async fn send_demand(s: &DemandSend) -> (&'static str, String) {
    if let Err(e) = allowed(&s.base).await {
        return ("unreachable", e);
    }
    let url = format!("{}/api/capacity/peer-demand", s.base.trim_end_matches('/'));
    let body = serde_json::json!({
        "pool_pid": s.peer_pool_ref,
        "programme_ref": programme_ref(s.plan_pid),
        "month": s.month.format("%Y-%m-%d").to_string(),
        "fte_centi": s.fte_centi,
        "peer_event_id": s.peer_event_id,
    });
    let response = match client()
        .post(&url)
        .bearer_auth(&s.bearer)
        .json(&body)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) if e.is_timeout() => {
            return ("unreachable", "WPM did not answer in time".to_string());
        }
        Err(_) => return ("unreachable", "WPM could not be reached".to_string()),
    };
    let status = response.status();
    if !status.is_success() {
        return ("refused", format!("WPM answered {}", status.as_u16()));
    }
    let duplicate = response
        .json::<serde_json::Value>()
        .await
        .ok()
        .and_then(|v| v["duplicate"].as_bool())
        .unwrap_or(false);
    if duplicate {
        ("duplicate", "WPM already had this version".to_string())
    } else {
        ("sent", "WPM took the figure".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn fte_hundredths_round_half_up_and_never_go_negative() {
        assert_eq!(centi_of_milli(1500), 150, "1.5 FTE");
        assert_eq!(centi_of_milli(1005), 101, "1.005 rounds up");
        assert_eq!(centi_of_milli(1004), 100);
        assert_eq!(centi_of_milli(4), 0, "0.004 FTE is no hundredth");
        assert_eq!(centi_of_milli(-10), 0);
        assert_eq!(milli_of_centi(150), 1500);
    }

    #[test]
    fn the_programme_is_a_thing_ref_and_the_event_id_is_stable_per_version() {
        let (pool, plan) = (Uuid::nil(), Uuid::from_u128(7));
        assert_eq!(programme_ref(plan), format!("thing:{plan}"));
        let at = DateTime::from_timestamp(1_800_000_000, 123_000).unwrap();
        let a = peer_event_id(pool, plan, d("2026-11-01"), at);
        assert_eq!(
            a,
            peer_event_id(pool, plan, d("2026-11-01"), at),
            "the same version, the same id"
        );
        let later = at + chrono::Duration::microseconds(1);
        assert_ne!(
            a,
            peer_event_id(pool, plan, d("2026-11-01"), later),
            "a new write is a new id"
        );
        assert!(a.len() <= 128, "WPM takes at most 128 characters: {a}");
    }

    #[test]
    fn peer_pool_refs_are_lower_case_hyphenated_uuids() {
        assert!(peer_pool_ref_ok("3f2c6b7e-0000-4000-8000-0000000000a1"));
        assert!(!peer_pool_ref_ok("3F2C6B7E-0000-4000-8000-0000000000A1"));
        assert!(!peer_pool_ref_ok("3f2c6b7e00004000800000000000000a1"));
        assert!(!peer_pool_ref_ok("Engineering"));
    }

    #[test]
    fn demand_is_sent_only_with_a_figure_a_url_and_a_token() {
        assert_eq!(plan_demand(1500, true, true), DemandPlan::Send(150));
        assert!(matches!(
            plan_demand(0, true, true),
            DemandPlan::Skip("not_sent_zero", _)
        ));
        assert!(matches!(
            plan_demand(4, true, true),
            DemandPlan::Skip("not_sent_zero", _)
        ));
        assert!(matches!(
            plan_demand(1500, false, true),
            DemandPlan::Skip("not_configured", _)
        ));
        assert!(matches!(
            plan_demand(1500, true, false),
            DemandPlan::Skip("not_sent_no_token", _)
        ));
        assert!(matches!(
            plan_demand(20_000_000, true, true),
            DemandPlan::Skip("not_sent_too_large", _)
        ));
    }

    #[test]
    fn a_month_is_live_then_last_received_with_its_time_then_unknown_never_zero() {
        let at = DateTime::from_timestamp(1_800_000_000, 0).unwrap();
        let r = Received {
            centi: Some(250),
            as_of: at,
        };
        let live = read_month(Some(r), None, None);
        assert_eq!(
            (live.status, live.milli, live.as_of),
            ("live", Some(2500), Some(at))
        );
        let kept = read_month(None, Some(r), Some("WPM did not answer in time"));
        assert_eq!(
            (kept.status, kept.milli, kept.as_of),
            ("last_received", Some(2500), Some(at))
        );
        assert_eq!(kept.reason.as_deref(), Some("WPM did not answer in time"));
        let none = read_month(None, None, Some("WPM could not be reached"));
        assert_eq!(
            (none.status, none.milli, none.as_of),
            ("unknown", None, None),
            "never zero"
        );
        let hidden = read_month(
            Some(Received {
                centi: None,
                as_of: at,
            }),
            None,
            None,
        );
        assert_eq!((hidden.status, hidden.milli), ("hidden", None));
    }

    /// A stand-in WPM on a local port: answers peer-supply with `supply`, or waits `delay` first.
    async fn serve(supply: serde_json::Value, delay: Duration) -> String {
        let app = axum::Router::new().route(
            "/api/capacity/peer-supply",
            axum::routing::get(move || {
                let body = supply.clone();
                async move {
                    tokio::time::sleep(delay).await;
                    axum::Json(body)
                }
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        format!("http://{addr}")
    }

    #[tokio::test]
    async fn a_slow_wpm_is_abandoned_within_the_timeout_and_reads_as_unknown() {
        let base = serve(serde_json::json!({}), Duration::from_secs(10)).await;
        let started = std::time::Instant::now();
        let r = fetch_supply(&base, "t", &Uuid::new_v4().to_string(), d("2026-11-01"), 2).await;
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "bounded by the timeout"
        );
        assert_eq!(
            r,
            Err(Fetch::Unreachable("WPM did not answer in time".to_string()))
        );
    }

    #[tokio::test]
    async fn a_refusal_is_told_apart_from_a_peer_that_cannot_be_reached() {
        // A refused caller must not be shown the last figure another caller's token fetched (WPM-D135).
        let app = axum::Router::new().route(
            "/api/capacity/peer-supply",
            axum::routing::get(|| async { axum::http::StatusCode::FORBIDDEN }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let peer = Uuid::new_v4().to_string();
        let refused = fetch_supply(&format!("http://{addr}"), "t", &peer, d("2026-11-01"), 1).await;
        assert!(matches!(refused, Err(Fetch::Refused(_))), "{refused:?}");
        let gone = fetch_supply("http://127.0.0.1:9", "t", &peer, d("2026-11-01"), 1).await;
        assert!(matches!(gone, Err(Fetch::Unreachable(_))), "{gone:?}");
    }

    #[tokio::test]
    async fn a_supply_answer_is_read_with_hidden_months_kept_as_none() {
        let peer = Uuid::new_v4().to_string();
        let base = serve(
            serde_json::json!({ "as_of": "2026-10-30T09:00:00Z", "months": [
                { "month": "2026-11-01", "supply_centi": 300, "hidden": false },
                { "month": "2026-12-01", "supply_centi": null, "hidden": true } ] }),
            Duration::ZERO,
        )
        .await;
        let rows = fetch_supply(&base, "t", &peer, d("2026-11-01"), 2)
            .await
            .unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].1.centi, Some(300));
        assert_eq!(rows[1].1.centi, None);
    }
}
