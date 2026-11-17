#![warn(clippy::pedantic)]

//! Criterion benchmarks for the `project-portfolio-management-service`
//! pure request-path work: payload validation, record merge, and the
//! Tantivy retrieval that backs duplicate detection.
//!
//! Run with `cargo bench`. Nothing here touches Postgres — these are the
//! CPU-bound halves of a request, which is exactly the part a database
//! benchmark would hide behind I/O.
//!
//! What the groups are for:
//!
//! - **`validation`** — runs on every create and update. The
//!   `oversized_arrays` case exercises the SEC-M1 input caps: rejecting
//!   an abusive payload must be cheap.
//! - **`merge`** — a whole-record fold; the scaling case shows the cost
//!   sits in the collections it unions.
//! - **`search`** — `index_one_document` is what every create, update,
//!   and merge pays synchronously. The `kind` filter is benchmarked
//!   with and without a value because it is a **search** filter only:
//!   it narrows retrieval and never gates matching, so its cost belongs
//!   here and nowhere near the matcher.
//! - **`capacity_rollups`** (T-28d) — the pure arithmetic behind
//!   `GET /capacity` (`visibility::summed_percent`) and `GET
//!   /capacity/utilization` (`effort::utilization`), scaled over the
//!   input sizes the DB-gated `tests/requests/scale.rs` seeds. This is
//!   the CPU half; that test proves the *I/O* half (the query count
//!   feeding these) does not grow with plan/people count.
//! - **`cpm`** — the critical-path calculation behind `GET …/cpm`, on a chain with a skip dependency every fifth
//!   task (the load test's seed), from 100 to 12,667 tasks: the size `BENCHMARKS.md` measured over HTTP, so the
//!   calculation's share of that request can be read off.
//! - **`auth`** — what the guard does on every request: verify a PASETO `v4.public` token and decide the ABAC policy
//!   (`auth::enforce`), against refusing a request with no token and passing a public path.
//! - **`cursor`** — reading and checking a signed list cursor (`pagination::page_with`), which every paged list does.
//! - **`public_roadmap`** — the roadmap rules on a write: a slug, a public id, a translation's source hash, the
//!   translations shown for a full item, and the checks on tags and details.
//! - **`dhcw_import`** — reading DHCW's example spreadsheet (`dhcw_import::parse` and the dry run's report).

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use project_portfolio_management::{
    auth, cpm, effort, merge,
    search::{SearchEngine, SearchMode},
    validation, visibility,
};
use project_portfolio_management_matcher::{
    Goal, IdentifierScheme, Plan, PlanIdentifier, PlanKind, PlanStatus,
};
use uuid::Uuid;

fn goal(title: &str) -> Goal {
    Goal {
        title: title.into(),
        description: None,
        target_date: None,
        status: None,
    }
}

/// A fully populated plan — every validated field present, so validation
/// runs its whole rule set.
fn populated() -> Plan {
    let mut p = Plan::new("Patient Records Modernization");
    p.kind = Some(PlanKind::Program);
    p.alternate_names = vec!["PRM".into()];
    p.code = Some("PRM-2024".into());
    p.owner_org_id = Some("org-42".into());
    p.owner_org_name = Some("Example NHS Trust".into());
    p.lead_ref = Some("worker:0c4f1e2a".into());
    p.status = Some(PlanStatus::Active);
    p.goals = vec![
        goal("Retire the legacy record store"),
        goal("Cut record retrieval time by half"),
    ];
    p.start_date = Some("2024-01-08".into());
    p.target_date = Some("2026-03-31".into());
    p.keywords = vec!["records".into(), "modernization".into()];
    p.tags = vec!["strategic".into(), "digital".into()];
    p.identifiers = vec![PlanIdentifier {
        scheme: IdentifierScheme::JiraProjectKey,
        value: "PRM".into(),
    }];
    p.same_as = vec!["https://tracker.example/PRM".into()];
    p
}

/// A payload violating several rules at once — the controller collects
/// every problem in one pass, so this is the realistic bad-request shape.
fn invalid() -> Plan {
    let mut p = populated();
    p.name = "   ".into();
    p.start_date = Some("2024-13-45".into());
    p.keywords = vec![String::new(), "records".into()];
    p.identifiers = vec![PlanIdentifier {
        scheme: IdentifierScheme::JiraProjectKey,
        value: String::new(),
    }];
    p
}

/// Deliberately over the SEC-M1 caps: the shape an attacker sends to buy
/// O(n·m) scoring work with one cheap request.
fn oversized() -> Plan {
    let mut p = populated();
    p.name = "x".repeat(64 * 1024);
    p.goals = (0..10_000).map(|i| goal(&format!("goal-{i}"))).collect();
    p.keywords = (0..10_000).map(|i| format!("keyword-{i}")).collect();
    p
}

/// The `idx`-th distinct plan. `kind` cycles on purpose: it is a label
/// and a search filter, never a matching gate.
fn variant(idx: usize) -> Plan {
    let stems = ["Records", "Estates", "Workforce", "Analytics", "Networks"];
    let kinds = [
        PlanKind::Portfolio,
        PlanKind::Project,
        PlanKind::Product,
        PlanKind::Program,
        PlanKind::Practice,
    ];
    let mut p = Plan::new(format!("{} Modernization {idx}", stems[idx % stems.len()]));
    p.kind = Some(kinds[idx % kinds.len()]);
    p.code = Some(format!("PRM-{:04}", idx % 500));
    p.owner_org_id = Some(format!("org-{}", idx % 20));
    p.goals = vec![goal(&format!("Deliver milestone {}", idx % 60))];
    p.keywords = vec!["records".into(), format!("theme-{}", idx % 30)];
    p.tags = vec!["digital".into()];
    p
}

fn bench_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation");
    let valid = populated();
    let invalid = invalid();
    let oversized = oversized();

    group.bench_function("valid_payload", |b| {
        b.iter(|| validation::problems(black_box(&valid)));
    });
    group.bench_function("several_problems", |b| {
        b.iter(|| validation::problems(black_box(&invalid)));
    });
    group.bench_function("oversized_arrays", |b| {
        b.iter(|| validation::problems(black_box(&oversized)));
    });
    group.finish();
}

fn bench_merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("merge");
    let main = populated();
    let duplicate = variant(7);
    group.bench_function("typical_pair", |b| {
        b.iter(|| merge::merge_plans(black_box(&main), black_box(&duplicate)));
    });

    // Collection union is where merge's cost lives, so scale that.
    let mut big_main = populated();
    big_main.keywords = (0..500).map(|i| format!("keyword-{i}")).collect();
    big_main.tags = (0..500).map(|i| format!("tag-{i}")).collect();
    let mut big_dup = populated();
    big_dup.keywords = (250..750).map(|i| format!("keyword-{i}")).collect();
    big_dup.tags = (250..750).map(|i| format!("tag-{i}")).collect();
    group.bench_function("many_keywords_half_overlapping", |b| {
        b.iter(|| merge::merge_plans(black_box(&big_main), black_box(&big_dup)));
    });
    group.finish();
}

fn bench_search(c: &mut Criterion) {
    let dir = tempfile::tempdir().expect("temp dir");
    let engine = SearchEngine::new(dir.path()).expect("index opens");
    for i in 0..500 {
        engine
            .index_plan(Uuid::new_v4(), &variant(i))
            .expect("index write");
    }

    let mut group = c.benchmark_group("search");
    // Its own index, deliberately: measured against the populated one
    // the number would be dominated by accumulated segment state rather
    // than by what a write costs.
    let write_dir = tempfile::tempdir().expect("temp dir");
    let write_engine = SearchEngine::new(write_dir.path()).expect("index opens");
    group.bench_function("index_one_document", |b| {
        let plan = populated();
        b.iter(|| {
            write_engine
                .index_plan(black_box(Uuid::new_v4()), black_box(&plan))
                .expect("index write");
        });
    });

    for (label, mode) in [
        ("exact", SearchMode::Exact),
        ("fuzzy", SearchMode::Fuzzy),
        ("phonetic", SearchMode::Phonetic),
    ] {
        group.bench_function(label, |b| {
            b.iter(|| {
                engine
                    .search_page(black_box("Records Modernization"), mode, None, 20, 0)
                    .expect("search")
            });
        });
    }
    // The same query narrowed by `kind`, so the filter's cost is visible
    // rather than folded into the unfiltered number.
    group.bench_function("exact_kind_filtered", |b| {
        b.iter(|| {
            engine
                .search_page(
                    black_box("Records Modernization"),
                    SearchMode::Exact,
                    Some("project"),
                    20,
                    0,
                )
                .expect("search")
        });
    });

    let probe = populated();
    for &limit in &[10usize, 50] {
        group.throughput(Throughput::Elements(limit as u64));
        group.bench_with_input(
            BenchmarkId::new("duplicate_candidates", limit),
            &limit,
            |b, &limit| {
                b.iter(|| {
                    engine
                        .candidates(black_box(&probe), limit)
                        .expect("candidates")
                });
            },
        );
    }
    group.finish();
}

/// Capacity/utilization rollups (T-28d): the pure arithmetic scales
/// with the number of *allocations* / *people* summed, not with plan
/// count directly, but plan count is what drives that in practice
/// (roughly one allocation per plan per person). 40/400 brackets the
/// scale `tests/requests/scale.rs` seeds (40 people) and an order of
/// magnitude beyond it.
fn bench_capacity_rollups(c: &mut Criterion) {
    let mut group = c.benchmark_group("capacity_rollups");

    for &people in &[40usize, 400] {
        let allocations: Vec<(
            i32,
            Option<project_portfolio_management::datetime::Date>,
            Option<project_portfolio_management::datetime::Date>,
        )> = (0..people)
            .map(|i| {
                let pct = 10 + i32::try_from(i % 90).unwrap_or(0);
                (pct, None, None)
            })
            .collect();
        let window = (
            project_portfolio_management::datetime::Date::from_ymd_opt(2026, 1, 1)
                .expect("valid date"),
            project_portfolio_management::datetime::Date::from_ymd_opt(2026, 1, 31)
                .expect("valid date"),
        );
        group.throughput(Throughput::Elements(people as u64));
        group.bench_with_input(
            BenchmarkId::new("summed_percent", people),
            &allocations,
            |b, allocations| {
                b.iter(|| visibility::summed_percent(black_box(allocations), window.0, window.1));
            },
        );
    }

    for &people in &[40usize, 400] {
        let facts: Vec<effort::CapacityFact> = (0..people)
            .map(|i| effort::CapacityFact {
                actor_ref: format!("worker:{i}"),
                declared_minutes: 2400,
                non_working_minutes: i64::try_from(i % 5).unwrap_or(0) * 60,
                effort_minutes: i64::try_from(i % 30).unwrap_or(0) * 100,
            })
            .collect();
        group.throughput(Throughput::Elements(people as u64));
        group.bench_with_input(
            BenchmarkId::new("utilization", people),
            &facts,
            |b, facts| {
                b.iter(|| {
                    facts
                        .iter()
                        .map(|f| effort::utilization(black_box(f), 60))
                        .collect::<Vec<_>>()
                });
            },
        );
    }

    group.finish();
}

/// A chain of `n` one-to-nine-day tasks, the first dated, with a skip dependency every fifth task.
fn chain(n: u128) -> (Vec<cpm::Node>, Vec<cpm::Edge>) {
    let start = project_portfolio_management::datetime::Date::from_ymd_opt(2026, 11, 2)
        .expect("valid date");
    let id = |i: u128| Uuid::from_u128(i + 1);
    let nodes = (0..n)
        .map(|i| cpm::Node {
            pid: id(i),
            duration: Some(1 + i64::try_from(i % 9).unwrap_or(0)),
            planned_start: (i == 0).then_some(start),
            ..cpm::Node::default()
        })
        .collect();
    let mut edges = Vec::new();
    let edge = |k: u128, a: u128, b: u128| cpm::Edge {
        pid: Uuid::from_u128(1 << 64 | k),
        pred: id(a),
        succ: id(b),
        kind: cpm::Kind::Fs,
        lag: 0,
    };
    for i in 1..n {
        edges.push(edge(i, i - 1, i));
        if i % 5 == 0 && i >= 3 {
            edges.push(edge(n + i, i - 3, i));
        }
    }
    (nodes, edges)
}

fn bench_cpm(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpm");
    group.sample_size(20);
    let calendar = cpm::WorkCalendar::new([true, true, true, true, true, false, false], [])
        .expect("a working week");
    for n in [100_u128, 1_000, 4_000, 12_667] {
        let (nodes, edges) = chain(n);
        group.throughput(Throughput::Elements(u64::try_from(n).unwrap_or(u64::MAX)));
        group.bench_with_input(
            BenchmarkId::new("analyze", n),
            &(nodes, edges),
            |b, (nodes, edges)| {
                b.iter(|| {
                    cpm::analyze(
                        black_box(nodes),
                        black_box(edges),
                        &calendar,
                        cpm::Options::default(),
                    )
                    .expect("a schedule")
                });
            },
        );
    }
    group.finish();
}

#[cfg(feature = "paseto")]
fn bench_auth(c: &mut Criterion) {
    use authentication_verifier::{Claims, Policy, Verifier};
    use axum::http::{HeaderMap, Method, header::AUTHORIZATION};
    use base64::Engine;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use ed25519_dalek::SigningKey;
    use rusty_paseto::core::{
        Footer, Key, Paseto, PasetoAsymmetricPrivateKey, Payload, Public, V4,
    };
    use sha2::{Digest, Sha256};

    // A throwaway key, as in the auth unit tests; never a secret.
    const SEED: [u8; 32] = [5; 32];
    let public = SigningKey::from_bytes(&SEED).verifying_key().to_bytes();
    let kid = URL_SAFE_NO_PAD.encode(Sha256::digest(public));
    let keys = serde_json::json!({ "keys": [{ "kty": "OKP", "crv": "Ed25519", "use": "sig", "kid": kid,
                                              "x": URL_SAFE_NO_PAD.encode(public) }] });
    let verifier =
        Verifier::from_paseto_keys_value(&keys, "authentication-service", "main-x-service")
            .expect("a verifier");
    let claims = Claims {
        sub: "11111111-1111-1111-1111-111111111111".into(),
        email: "alice@example.com".into(),
        name: "Alice".into(),
        iss: "authentication-service".into(),
        aud: "main-x-service".into(),
        exp: 4_000_000_000,
        iat: 1_700_000_000,
        nbf: None,
        sid: "bench".into(),
        scope: Vec::new(),
        roles: Vec::new(),
        attrs: [("access".to_string(), vec!["write".to_string()])].into(),
    };
    let key = Key::<64>::from(SigningKey::from_bytes(&SEED).to_keypair_bytes());
    let private = PasetoAsymmetricPrivateKey::<V4, Public>::from(&key);
    let payload = serde_json::to_string(&claims).expect("claims");
    let footer = format!(r#"{{"kid":"{kid}"}}"#);
    let mut builder = Paseto::<V4, Public>::builder();
    builder.set_payload(Payload::from(payload.as_str()));
    builder.set_footer(Footer::from(footer.as_str()));
    let token = builder.try_sign(&private).expect("sign");
    let mut signed = HeaderMap::new();
    signed.insert(
        AUTHORIZATION,
        format!("Bearer {token}").parse().expect("header"),
    );
    let none = HeaderMap::new();
    let policy = Policy::default_policy();

    let mut group = c.benchmark_group("auth");
    group.bench_function("enforce_valid_token_read", |b| {
        b.iter(|| {
            auth::enforce(
                true,
                &Method::GET,
                "/api/plans",
                black_box(&signed),
                &verifier,
                &policy,
            )
        });
    });
    group.bench_function("enforce_valid_token_write", |b| {
        b.iter(|| {
            auth::enforce(
                true,
                &Method::POST,
                "/api/plans",
                black_box(&signed),
                &verifier,
                &policy,
            )
        });
    });
    group.bench_function("refuse_no_token", |b| {
        b.iter(|| {
            auth::enforce(
                true,
                &Method::GET,
                "/api/plans",
                black_box(&none),
                &verifier,
                &policy,
            )
        });
    });
    group.bench_function("public_path", |b| {
        b.iter(|| {
            auth::enforce(
                true,
                &Method::GET,
                "/_health",
                black_box(&none),
                &verifier,
                &policy,
            )
        });
    });
    group.finish();
}

#[cfg(not(feature = "paseto"))]
fn bench_auth(_c: &mut Criterion) {}

fn bench_cursor(c: &mut Criterion) {
    use project_portfolio_management::controllers::pagination;
    let key = axa::cursor_pagination::CursorKey::new(&[9; 32]).expect("a key");
    let first: axum::http::Uri = "/api/plans?limit=50".parse().expect("uri");
    let page = pagination::page_with(&key, &first, 200, pagination::MAX_LIMIT).expect("page");
    let next = page.next_link().expect("a next link");
    let next: axum::http::Uri = next
        .to_str()
        .expect("text")
        .trim_start_matches('<')
        .split_once('>')
        .expect("a link")
        .0
        .parse()
        .expect("uri");
    let forged: axum::http::Uri =
        "/api/plans?limit=50&cursor=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
            .parse()
            .expect("uri");
    let mut group = c.benchmark_group("cursor");
    group.bench_function("first_page", |b| {
        b.iter(|| pagination::page_with(&key, black_box(&first), 200, pagination::MAX_LIMIT));
    });
    group.bench_function("signed_next_page", |b| {
        b.iter(|| pagination::page_with(&key, black_box(&next), 200, pagination::MAX_LIMIT));
    });
    group.bench_function("refuse_forged_cursor", |b| {
        b.iter(|| {
            pagination::page_with(&key, black_box(&forged), 200, pagination::MAX_LIMIT).is_err()
        });
    });
    group.bench_function("next_link", |b| b.iter(|| black_box(&page).next_link()));
    group.finish();
}

fn bench_public_roadmap(c: &mut Criterion) {
    use project_portfolio_management::public_roadmap::{self as rules, Detail};
    use serde_json::json;
    let tags: Vec<String> = (0..20).map(|i| format!("Service {i}")).collect();
    let details: Vec<Detail> = (0..30)
        .map(|i| Detail {
            text: format!("Line {i} of the details"),
            level: u8::try_from(i % 3).unwrap_or(0),
        })
        .collect();
    let sources = std::collections::BTreeMap::from([
        ("title", json!("Online booking for everyone")),
        (
            "description",
            json!("What the work is, in a sentence or two."),
        ),
        ("tags", json!(tags)),
        ("details", json!(details)),
    ]);
    let stored = json!({ "cy": {
        "title": rules::stored_translation(&sources["title"], &json!("Archebu ar-lein i bawb")),
        "description": rules::stored_translation(&sources["description"], &json!("Beth yw'r gwaith.")),
        "tags": rules::stored_translation(&sources["tags"], &json!(tags)),
        "details": rules::stored_translation(&sources["details"], &json!(details)),
    }});
    let mut group = c.benchmark_group("public_roadmap");
    group.bench_function("slug_from_title", |b| {
        b.iter(|| rules::slug_from(black_box("Online booking for GP appointments — Wales 2027")));
    });
    group.bench_function("public_id", |b| {
        b.iter(|| rules::public_id_from(black_box(&[7; 16])));
    });
    group.bench_function("source_hash_30_detail_lines", |b| {
        b.iter(|| rules::source_hash(black_box(&sources["details"])));
    });
    group.bench_function("shown_translations_full_item", |b| {
        b.iter(|| rules::shown_translations(black_box(&stored), &sources));
    });
    group.bench_function("tag_and_detail_checks", |b| {
        b.iter(|| {
            let mut out = Vec::new();
            rules::tag_problems(black_box(&tags), &mut out);
            rules::detail_problems(black_box(&details), &mut out);
            out
        });
    });
    group.finish();
}

fn bench_dhcw_import(c: &mut Criterion) {
    use project_portfolio_management::dhcw_import::{self, Input};
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dhcw-roadmaps");
    let read = |name: &str| std::fs::read_to_string(dir.join(name)).expect("fixture");
    let input = Input {
        index: read("Roadmaps.csv"),
        tabs: ["ProductA", "ProductB", "ProductC"]
            .into_iter()
            .map(|t| (t.to_string(), read(&format!("{t}.csv"))))
            .collect(),
        known_codes: vec!["SBUHB".into(), "National".into()],
        known_categories: vec!["GP & Community Services".into()],
    };
    let mut group = c.benchmark_group("dhcw_import");
    group.bench_function("parse_example_spreadsheet", |b| {
        b.iter(|| dhcw_import::parse(black_box(&input)));
    });
    group.bench_function("dry_run_example_spreadsheet", |b| {
        b.iter(|| dhcw_import::dry_run(black_box(&input)));
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_validation,
    bench_merge,
    bench_search,
    bench_capacity_rollups,
    bench_cpm,
    bench_auth,
    bench_cursor,
    bench_public_roadmap,
    bench_dhcw_import
);
criterion_main!(benches);
