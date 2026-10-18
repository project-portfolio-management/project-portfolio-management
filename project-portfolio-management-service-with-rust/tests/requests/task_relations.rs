//! Task-relation request tests (LT-10): shape and duplicate refusals,
//! acyclic `blocks`, `blocks` stopping a close, `precedes` on due dates
//! (at create and at edit), `duplicates` closing the twin, and removal.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use loco_rs::testing::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Value, json};
use serial_test::serial;

async fn plan(request: &axum_test::TestServer) -> String {
    let created: Value = request
        .post("/api/plans")
        .json(&json!({ "kind": "Project", "name": "Apollo" }))
        .await
        .json();
    created["pid"].as_str().expect("pid").to_string()
}

async fn task(request: &axum_test::TestServer, p: &str, title: &str, due: Option<&str>) -> String {
    let mut body = json!({ "title": title });
    if let Some(d) = due {
        body["due_date"] = json!(d);
    }
    let t: Value = request
        .post(&format!("/api/plans/{p}/tasks"))
        .json(&body)
        .await
        .json();
    t["pid"].as_str().expect("task pid").to_string()
}

async fn relate(
    request: &axum_test::TestServer,
    p: &str,
    from: &str,
    kind: &str,
    to: &str,
) -> axum_test::TestResponse {
    request
        .post(&format!("/api/plans/{p}/tasks/{from}/relations"))
        .json(&json!({ "kind": kind, "to_task_pid": to }))
        .await
}

async fn mv(request: &axum_test::TestServer, p: &str, t: &str, status: &str) -> u16 {
    request
        .patch(&format!("/api/plans/{p}/tasks/{t}"))
        .json(&json!({ "status": status }))
        .await
        .status_code()
        .as_u16()
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (config/test.yaml); run with `cargo test -- --ignored`"]
async fn relations_have_the_behaviours_redmine_proved() {
    super::isolate_search_index();
    request::<App, _, _>(|request, _ctx| async move {
        let p = plan(&request).await;
        let a = task(&request, &p, "A", None).await;
        let b = task(&request, &p, "B", None).await;
        let c = task(&request, &p, "C", None).await;

        // Shape: a self-relation and an unknown kind are refused; an exact
        // duplicate is a 409, and `related` is symmetric.
        assert_eq!(
            relate(&request, &p, &a, "related", &a).await.status_code(),
            422
        );
        assert_eq!(
            relate(&request, &p, &a, "nope", &b).await.status_code(),
            422
        );
        relate(&request, &p, &a, "related", &b)
            .await
            .assert_status_ok();
        assert_eq!(
            relate(&request, &p, &a, "related", &b).await.status_code(),
            409
        );
        assert_eq!(
            relate(&request, &p, &b, "related", &a).await.status_code(),
            409,
            "symmetric"
        );

        // `blocks` must stay acyclic.
        relate(&request, &p, &a, "blocks", &b)
            .await
            .assert_status_ok();
        relate(&request, &p, &b, "blocks", &c)
            .await
            .assert_status_ok();
        assert_eq!(
            relate(&request, &p, &c, "blocks", &a).await.status_code(),
            422
        );

        // `blocks` stops the blocked task finishing while the blocker is open.
        let refused = request
            .patch(&format!("/api/plans/{p}/tasks/{b}"))
            .json(&json!({ "status": "done" }))
            .await;
        assert_eq!(refused.status_code(), 422);
        assert!(
            refused.text().contains("blocks"),
            "names the relation: {}",
            refused.text()
        );
        assert_eq!(
            mv(&request, &p, &a, "done").await,
            200,
            "the blocker itself is free to finish"
        );
        assert_eq!(
            mv(&request, &p, &b, "done").await,
            200,
            "unblocked once the blocker is done"
        );

        // Removing a relation lifts it.
        let rels: Value = request
            .get(&format!("/api/plans/{p}/tasks/{c}/relations"))
            .await
            .json();
        let incoming = rels["relations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["kind"] == "blocks")
            .expect("C is blocked by B");
        assert_eq!(incoming["direction"], "incoming");
        // B is done already, so C is movable; prove removal on a fresh pair.
        let x = task(&request, &p, "X", None).await;
        let y = task(&request, &p, "Y", None).await;
        let rel: Value = relate(&request, &p, &x, "blocks", &y).await.json();
        assert_eq!(mv(&request, &p, &y, "done").await, 422);
        request
            .delete(&format!(
                "/api/plans/{p}/tasks/{x}/relations/{}",
                rel["pid"].as_str().unwrap()
            ))
            .await
            .assert_status_ok();
        assert_eq!(mv(&request, &p, &y, "done").await, 200);

        // `precedes`: the successor must be due strictly after the predecessor.
        let pa = task(&request, &p, "PA", Some("2026-10-05")).await;
        let pb = task(&request, &p, "PB", Some("2026-10-05")).await;
        assert_eq!(
            relate(&request, &p, &pa, "precedes", &pb)
                .await
                .status_code(),
            422,
            "same day"
        );
        let pc = task(&request, &p, "PC", Some("2026-10-06")).await;
        relate(&request, &p, &pa, "precedes", &pc)
            .await
            .assert_status_ok();
        let edit = |t: String, due: &'static str| {
            let (request, p) = (&request, p.clone());
            async move {
                request
                    .put(&format!("/api/plans/{p}/tasks/{t}"))
                    .json(&json!({ "title": "x", "due_date": due }))
                    .await
                    .status_code()
                    .as_u16()
            }
        };
        assert_eq!(
            edit(pc.clone(), "2026-10-04").await,
            422,
            "successor moved before predecessor"
        );
        assert_eq!(
            edit(pa.clone(), "2026-10-07").await,
            422,
            "predecessor moved after successor"
        );
        assert_eq!(
            edit(pa.clone(), "2026-10-03").await,
            200,
            "earlier predecessor is fine"
        );

        // `duplicates`: closing the primary closes the duplicate.
        let primary = task(&request, &p, "Primary", None).await;
        let dup = task(&request, &p, "Dup", None).await;
        relate(&request, &p, &dup, "duplicates", &primary)
            .await
            .assert_status_ok();
        assert_eq!(mv(&request, &p, &primary, "done").await, 200);
        let list: Value = request.get(&format!("/api/plans/{p}/tasks")).await.json();
        let dup_row = list["tasks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["pid"] == dup)
            .expect("dup");
        assert_eq!(dup_row["status"], "done");
        assert!(dup_row["done_at"].is_string(), "closed, with a done_at");
        let tr: Value = request
            .get(&format!("/api/plans/{p}/tasks/{dup}/transitions"))
            .await
            .json();
        assert!(
            format!("{tr}").contains("done"),
            "a transition row records the close: {tr}"
        );
    })
    .await;
}
