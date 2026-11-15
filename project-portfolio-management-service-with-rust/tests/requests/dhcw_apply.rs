//! The apply step after the DHCW dry run (RE-5): the example spreadsheet written onto Product plans.
//!
//! `#[ignore]`d: needs PostgreSQL; run with `cargo test -- --ignored`.

use std::collections::BTreeMap;

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use project_portfolio_management::dhcw_apply;
use project_portfolio_management::dhcw_import::{self, Input};
use serde_json::{Value, json};
use serial_test::serial;

async fn plan(request: &axum_test::TestServer, kind: &str, name: &str) -> String {
    let p: Value = request
        .post("/api/plans")
        .json(&json!({ "kind": kind, "name": name }))
        .await
        .json();
    p["pid"].as_str().expect("pid").to_string()
}

/// DHCW's example spreadsheet with this run's own sheet names, organization code and category, so a rerun on the
/// same database does not meet the slugs an earlier run made.
fn fixture(tag: &str) -> Input {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dhcw-roadmaps");
    let own = |text: String| {
        text.replace("ProductA", &format!("ProductA{tag}"))
            .replace("ProductB", &format!("ProductB{tag}"))
            .replace("ProductC", &format!("ProductC{tag}"))
            .replace("SBUHB", &format!("SB{tag}"))
            .replace("GP & Community Services", &format!("GP {tag}"))
    };
    let read = |name: &str| own(std::fs::read_to_string(dir.join(name)).expect("fixture"));
    Input {
        index: read("Roadmaps.csv"),
        tabs: ["ProductA", "ProductB", "ProductC"]
            .into_iter()
            .map(|t| (format!("{t}{tag}"), read(&format!("{t}.csv"))))
            .collect(),
        ..Default::default()
    }
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn the_example_spreadsheet_lands_on_the_plans_named_unpublished() {
    request::<App, _, _>(|request, ctx| async move {
        let tag = uuid::Uuid::new_v4().simple().to_string()[..6].to_uppercase();
        let a = plan(&request, "Product", &format!("Product A {tag}")).await;
        let project = plan(&request, "Project", &format!("Not a product {tag}")).await;
        let org: Value = request
            .post("/api/organizations")
            .json(&json!({ "name": format!("Swansea Bay {tag}"), "kind": "partner", "code": format!("SB{tag}") }))
            .await
            .json();
        let cat: Value = request
            .post("/api/roadmap-categories")
            .json(&json!({ "name": format!("GP {tag}") }))
            .await
            .json();
        let input = fixture(&tag);
        let parsed = dhcw_import::parse(&input).unwrap();
        let dry = dhcw_import::dry_run(&input);
        let plans: BTreeMap<String, uuid::Uuid> = [
            (format!("ProductA{tag}"), a.parse().unwrap()),
            (format!("ProductB{tag}"), project.parse().unwrap()),
            (format!("Elsewhere{tag}"), uuid::Uuid::new_v4()),
        ]
        .into();

        let report = dhcw_apply::apply(&ctx, &parsed, &plans).await.unwrap();
        assert_eq!(report["written"], true);
        assert_eq!(report["totals"]["roadmaps"], 3);
        assert_eq!(report["totals"]["roadmaps_made"], 1);
        assert_eq!(report["totals"]["roadmaps_skipped"], 2);
        assert_eq!(report["sheets_in_the_plans_file_not_read"], json!([format!("Elsewhere{tag}")]));
        let made = &report["roadmaps"][0];
        assert_eq!(made["outcome"], "made", "{made}");
        assert_eq!(made["enabled_in_sheet"], true);
        assert_eq!(made["published"], false, "never published by the import");
        assert_eq!(made["category"]["linked"], true);
        let mapped = dry["roadmaps"][0]["items"]["mapped"].as_u64().unwrap();
        assert_eq!(made["items_made"].as_u64().unwrap(), mapped, "as the dry run said");
        assert_eq!(made["items_skipped"], json!([]));
        assert!(report["roadmaps"][1]["reason"].as_str().unwrap().contains("not a Product plan"));
        assert!(report["roadmaps"][2]["reason"].as_str().unwrap().contains("names no plan"));

        let r: Value = request.get(&format!("/api/plans/{a}/roadmap")).await.json();
        assert_eq!(r["published"], false);
        assert_eq!(r["slug"], format!("producta{}", tag.to_lowercase()));
        assert_eq!(r["reviewed_on"], "2026-10-01");
        assert_eq!(r["category_pid"], cat["pid"]);
        let items = r["items"].as_array().unwrap();
        assert_eq!(items.len() as u64, mapped);
        let first = items.iter().find(|i| i["title"] == "Example card with every column").unwrap();
        assert_eq!(first["placement"], "now");
        assert_eq!(first["organization_pids"], json!([org["pid"]]), "the unknown code is left out");
        assert_eq!(first["tags"], json!(["Service X", "Service Y"]));
        assert_eq!(first["details"].as_array().unwrap().len(), 4);
        assert_eq!(first["translations"]["cy"]["title"]["value"], "Cerdyn enghreifftiol gyda phob colofn");
        assert_eq!(first["translations"]["cy"]["title"]["needs_review"], false);
        assert_eq!(first["translations"]["cy"]["tags"]["value"], json!(["Gwasanaeth X", "Gwasanaeth Y"]));
        assert!(items.iter().any(|i| i["placement"] == "not_doing"));
        let audit: Value = request.get(&format!("/api/plans/{a}/audit")).await.json();
        assert!(
            audit.as_array().unwrap().iter().any(|e| e["action"] == "public_roadmap_imported"
                && e["actor"] == dhcw_apply::ACTOR),
            "{audit}"
        );
        // Nothing was made on the project plan.
        request.get(&format!("/api/plans/{project}/roadmap")).await.assert_status_not_found();

        // A second run never changes the roadmap it made.
        let again = dhcw_apply::apply(&ctx, &parsed, &plans).await.unwrap();
        assert_eq!(again["totals"]["roadmaps_made"], 0);
        assert!(again["roadmaps"][0]["reason"].as_str().unwrap().contains("already has a roadmap"));
        let same: Value = request.get(&format!("/api/plans/{a}/roadmap")).await.json();
        assert_eq!(same["items"].as_array().unwrap().len() as u64, mapped);

        // Another plan cannot take the slug that roadmap has.
        let b = plan(&request, "Product", &format!("Product B {tag}")).await;
        let mut clash = parsed[0].clone();
        clash.items = Ok(Vec::new());
        let r = dhcw_apply::apply(&ctx, &[clash], &[(format!("ProductA{tag}"), b.parse().unwrap())].into())
            .await
            .unwrap();
        assert!(r["roadmaps"][0]["reason"].as_str().unwrap().contains("slug"), "{r}");
        request.get(&format!("/api/plans/{b}/roadmap")).await.assert_status_not_found();
    })
    .await;
}
