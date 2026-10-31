//! Request tests for **recorded effort** (FR-28) and **utilization**
//! (FR-35), including per person.
//!
//! The load-bearing test here is
//! [`someone_on_leave_reports_null_not_zero_percent`]: leave leaves the
//! denominator rather than sitting in it, so a person absent for the
//! whole window is *unavailable*, not *idle*. Reporting 0% there would
//! be the most defamatory number this service could publish, and it is
//! the obligation most easily lost in a refactor.
//!
//! There is deliberately **no test asserting a per-person cycle time**,
//! because no endpoint serves one: the 2026-08-25 decision narrowed the
//! family refusal to utilization alone.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

macro_rules! create_plan {
    ($request:expr, $name:expr) => {{
        let created: Value = $request
            .post("/api/plans")
            .json(&json!({ "name": $name }))
            .await
            .json();
        created["pid"].as_str().expect("plan pid").to_string()
    }};
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// Effort rolls up per plan, task and assignee, and every roll-up is
// labelled **asserted** — it is typed by a person, unlike a transition
// timestamp. Uncategorized effort stays visible rather than being
// folded into `opex`, which would flatter the capitalizable share.
async fn effort_rolls_up_and_is_labelled_asserted() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan = create_plan!(request, "Effort plan");
        for (actor, minutes, category) in [
            (
                "person:aaaaaaaa-0000-0000-0000-000000000001",
                120,
                Some("capex"),
            ),
            (
                "person:aaaaaaaa-0000-0000-0000-000000000001",
                60,
                Some("opex"),
            ),
            ("person:aaaaaaaa-0000-0000-0000-000000000002", 30, None),
        ] {
            let body = category.map_or_else(
                || json!({ "actor_ref": actor, "spent_on": "2026-08-20", "minutes": minutes }),
                |c| {
                    json!({ "actor_ref": actor, "spent_on": "2026-08-20",
                            "minutes": minutes, "category": c, "billable": true })
                },
            );
            let created = request
                .post(&format!("/api/plans/{plan}/time-entries"))
                .json(&body)
                .await;
            assert_eq!(created.status_code(), 200);
        }

        let rollup: Value = request
            .get(&format!("/api/plans/{plan}/effort"))
            .await
            .json();
        assert_eq!(rollup["plan"]["minutes"], 210);
        assert_eq!(rollup["plan"]["capex_minutes"], 120);
        assert_eq!(rollup["plan"]["opex_minutes"], 60);
        assert_eq!(
            rollup["plan"]["unclassified_minutes"], 30,
            "uncategorized effort must stay visible"
        );
        assert_eq!(rollup["plan"]["asserted"], true);
        assert_eq!(
            rollup["by_assignee"].as_array().expect("by_assignee").len(),
            2
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// A day cannot hold more than 1440 minutes, and an actor must be a real
// EntityRef. Both are refused at write rather than producing a roll-up
// nobody can reconcile.
async fn an_impossible_entry_is_refused() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan = create_plan!(request, "Refusal plan");
        let too_long = request
            .post(&format!("/api/plans/{plan}/time-entries"))
            .json(&json!({
                "actor_ref": "person:aaaaaaaa-0000-0000-0000-000000000001",
                "spent_on": "2026-08-20", "minutes": 2_000
            }))
            .await;
        assert_eq!(too_long.status_code(), 422);

        let bad_actor = request
            .post(&format!("/api/plans/{plan}/time-entries"))
            .json(&json!({
                "actor_ref": "someone", "spent_on": "2026-08-20", "minutes": 60
            }))
            .await;
        assert_eq!(bad_actor.status_code(), 422);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// **The obligation-2 test.** Somebody on leave for the whole window
// reports `null` with a reason, **never 0%** — leave is absence of
// capacity, not failure to use it. And the figure always ships its own
// denominator, so nobody has to assume it was 100%.
async fn someone_on_leave_reports_null_not_zero_percent() {
    super::isolate_search_index();
    // Relative to "now", not a fixed calendar date: the read side is
    // `window_days=28` from the moment of the request, so a hardcoded
    // `spent_on` ages out of the window and the test starts failing on
    // whatever date is more than 28 calendar days past it — found 2026-09-18,
    // when a `spent_on` written for an August run finally aged out.
    let today = chrono::Utc::now().date_naive();
    let spent_on = (today - chrono::Duration::days(5)).to_string();
    let leave_starts_on = (today - chrono::Duration::days(90)).to_string();
    let leave_ends_on = (today + chrono::Duration::days(90)).to_string();
    request::<App, _, _>(|request, _ctx| async move {
        let plan = create_plan!(request, "Utilization plan");
        let worker = "person:bbbbbbbb-0000-0000-0000-000000000001";
        let absent = "person:bbbbbbbb-0000-0000-0000-000000000002";

        // A generous declared capacity, so the suppression floor is
        // cleared and the *reason* under test is leave, not smallness.
        let declared = request
            .post("/api/working-time")
            .json(&json!({ "minutes_per_day": 480, "working_days_per_week": 5 }))
            .await;
        assert_eq!(declared.status_code(), 200);

        request
            .post(&format!("/api/plans/{plan}/time-entries"))
            .json(&json!({
                "actor_ref": worker, "spent_on": spent_on, "minutes": 480
            }))
            .await;

        // The absent person has no effort rows at all.
        let leave = request
            .post("/api/non-working")
            .json(&json!({
                "person_ref": absent, "starts_on": leave_starts_on,
                "ends_on": leave_ends_on, "kind": "leave"
            }))
            .await;
        assert_eq!(leave.status_code(), 200);

        let view: Value = request
            .get("/api/capacity/utilization?by=person&window_days=28")
            .await
            .json();
        let rows = view["utilization"].as_array().expect("utilization rows");

        let absentee = rows
            .iter()
            .find(|r| r["actor_ref"] == absent)
            .expect("a person on leave must still be reported, not silently absent");
        assert!(
            absentee["basis_points"].is_null(),
            "leave must never read as 0% utilization"
        );
        assert_eq!(absentee["unavailable"], "all_non_working");
        assert_eq!(
            absentee["available_minutes"], 0,
            "leave left the denominator"
        );
        assert!(absentee["declared_minutes"].as_i64().unwrap_or(0) > 0);

        let present = rows
            .iter()
            .find(|r| r["actor_ref"] == worker)
            .expect("the working person is reported");
        // The figure always ships its own working.
        assert_eq!(present["effort_minutes"], 480);
        assert!(present["available_minutes"].as_i64().unwrap_or(0) > 0);
        assert_eq!(present["asserted"], true);
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// With no declared capacity at all the denominator is **unknown**, which
// is a different answer from zero and from being on leave — so it has
// its own reason.
async fn no_declared_capacity_is_not_zero_utilization() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let view: Value = request
            .get("/api/capacity/utilization?by=team&window_days=28")
            .await
            .json();
        let utilization = &view["utilization"];
        assert!(utilization["basis_points"].is_null());
        assert!(
            utilization["unavailable"] == "no_declared_capacity"
                || utilization["unavailable"] == "all_non_working"
                || utilization["unavailable"] == "small_group",
            "an unknown denominator is never reported as 0%"
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
// **Small-cell suppression.** A team figure over fewer than five people is
// withheld whole (no figure, and no numerator or denominator to make one
// from); the fifth person makes it reportable. The per-person view is the
// owner's carve-out and is not suppressed at any size.
async fn a_team_under_five_is_withheld_and_a_team_of_five_is_reported() {
    super::isolate_search_index();
    let today = chrono::Utc::now().date_naive();
    let spent_on = (today - chrono::Duration::days(5)).to_string();
    request::<App, _, _>(|request, _ctx| async move {
        let plan = create_plan!(request, "Small cell plan");
        // Declared capacity large enough that the minutes floor is cleared,
        // so the reason under test is the size of the group.
        let declared = request
            .post("/api/working-time")
            .json(&json!({ "minutes_per_day": 480, "working_days_per_week": 5 }))
            .await;
        assert_eq!(declared.status_code(), 200);

        let person = |n: u32| format!("person:cccccccc-0000-0000-0000-00000000000{n}");
        let team = || async {
            let view: Value = request
                .get("/api/capacity/utilization?by=team&window_days=28")
                .await
                .json();
            view
        };
        for n in 1..=4 {
            let entry = request
                .post(&format!("/api/plans/{plan}/time-entries"))
                .json(&json!({
                    "actor_ref": person(n), "spent_on": spent_on, "minutes": 480
                }))
                .await;
            assert_eq!(entry.status_code(), 200);
        }

        let four = team().await;
        assert_eq!(four["people_counted"], 4);
        let u = &four["utilization"];
        assert!(u["basis_points"].is_null(), "four people: no figure");
        assert_eq!(u["unavailable"], "small_group");
        assert_eq!(u["effort_minutes"], 0, "the numerator is withheld too");
        assert_eq!(u["available_minutes"], 0, "and the denominator");
        // WW-1.6: the answer names the thresholds it used (the deployment's, here the stated defaults).
        assert_eq!(four["suppression"]["min_group_people"], 5);
        assert_eq!(four["suppression"]["floor_minutes"], 2_400);

        // The per-person view is unchanged at this size.
        let people: Value = request
            .get("/api/capacity/utilization?by=person&window_days=28")
            .await
            .json();
        assert_eq!(people["utilization"].as_array().expect("rows").len(), 4);

        let fifth = request
            .post(&format!("/api/plans/{plan}/time-entries"))
            .json(&json!({
                "actor_ref": person(5), "spent_on": spent_on, "minutes": 480
            }))
            .await;
        assert_eq!(fifth.status_code(), 200);

        let five = team().await;
        assert_eq!(five["people_counted"], 5);
        let u = &five["utilization"];
        assert!(u["unavailable"].is_null(), "five people: reportable");
        assert!(u["basis_points"].as_i64().is_some_and(|b| b > 0));
        assert_eq!(u["effort_minutes"], 5 * 480);
        assert!(u["available_minutes"].as_i64().unwrap_or(0) > 0);
    })
    .await;
}
