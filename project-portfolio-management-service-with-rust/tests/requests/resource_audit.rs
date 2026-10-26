//! Planning rules audit (BR-7): the gaps the audit found, each pinned by a test.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

/// CF-6: resolving a placeholder is a compare and set. Several resolvers that all saw the same placeholder race;
/// exactly one wins, the others are told it already names a person, and the person stored is the winner's.
#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn racing_resolvers_of_one_placeholder_leave_exactly_one_person() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let plan: Value = request.post("/api/plans").json(&json!({ "kind": "Project", "name": "Race" })).await.json();
        let plan = plan["pid"].as_str().unwrap().to_string();
        let ph: Value = request
            .post(&format!("/api/plans/{plan}/allocations"))
            .json(&json!({ "placeholder": true, "role": "dev", "percent": 50, "start_date": "2026-10-05", "end_date": "2026-10-11" }))
            .await
            .json();
        let ph = ph["pid"].as_str().unwrap().to_string();

        let people: Vec<String> = (1..=6u8).map(|n| format!("person:00000000-0000-4000-8000-0000000001{n:02}")).collect();
        let url = format!("/api/plans/{plan}/allocations/{ph}/resolve");
        let go = |i: usize| {
            let (request, url, p) = (&request, &url, people[i].clone());
            async move { (p.clone(), request.post(url).json(&json!({ "person_ref": p, "confirm": true })).await.status_code().as_u16()) }
        };
        let (r0, r1, r2, r3, r4, r5) = tokio::join!(go(0), go(1), go(2), go(3), go(4), go(5));
        let results = vec![r0, r1, r2, r3, r4, r5];

        let winners: Vec<&String> = results.iter().filter(|(_, s)| *s == 200).map(|(p, _)| p).collect();
        assert_eq!(winners.len(), 1, "exactly one resolver wins: {results:?}");
        assert!(results.iter().filter(|(_, s)| *s != 200).all(|(_, s)| *s == 409), "the rest are told 409: {results:?}");

        let listed: Value = request.get(&format!("/api/plans/{plan}/allocations")).await.json();
        let stored = listed.as_array().unwrap().iter().find(|a| a["pid"] == ph.as_str()).expect("the allocation");
        assert_eq!(stored["person_ref"], winners[0].as_str(), "the winner's person is the one stored");
    })
    .await;
}
