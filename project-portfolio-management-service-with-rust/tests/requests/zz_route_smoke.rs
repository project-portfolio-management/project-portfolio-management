//! Router-wide refusal smoke test. Every mounted API handler receives an inert
//! path and, for methods with bodies, an empty JSON object. This complements
//! the feature suites' successful cases by proving malformed and missing
//! inputs are refused at the real routing boundary without an internal error.

use axa_test::prelude::*;
use project_portfolio_management::app::App;
use serde_json::{Map, Value, json};
use serial_test::serial;
use std::collections::BTreeSet;

const NIL: &str = "00000000-0000-0000-0000-000000000000";

fn inert_path(template: &str) -> String {
    let mut path = String::with_capacity(template.len() + 64);
    let mut rest = template;
    while let Some((before, after_open)) = rest.split_once('{') {
        path.push_str(before);
        let Some((name, after)) = after_open.split_once('}') else {
            path.push_str(after_open);
            return path;
        };
        path.push_str(match name {
            "index" | "n" | "line" => "0",
            "version" => "1",
            "lang" => "en",
            "kind" | "dimension" | "key" | "slug" | "subject" | "gate" => "invalid",
            _ => NIL,
        });
        rest = after;
    }
    path.push_str(rest);
    path
}

fn seeded_path(template: &str, plan: &str, task: &str) -> String {
    let mut path = String::with_capacity(template.len() + 64);
    let mut rest = template;
    while let Some((before, after_open)) = rest.split_once('{') {
        path.push_str(before);
        let Some((name, after)) = after_open.split_once('}') else {
            path.push_str(after_open);
            return path;
        };
        path.push_str(match name {
            "pid" => plan,
            "t_pid" | "task_pid" | "tpid" => task,
            "index" | "n" | "line" => "0",
            "version" => "1",
            "lang" => "en",
            "kind" | "dimension" | "key" | "slug" | "subject" | "gate" => "invalid",
            _ => NIL,
        });
        rest = after;
    }
    path.push_str(rest);
    path
}

fn resolve_schema<'a>(document: &'a Value, schema: &'a Value) -> &'a Value {
    schema
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|reference| reference.strip_prefix('#'))
        .and_then(|pointer| document.pointer(pointer))
        .unwrap_or(schema)
}

fn minimal_json(document: &Value, schema: &Value, depth: usize) -> Value {
    if depth > 12 {
        return Value::Null;
    }
    let schema = resolve_schema(document, schema);
    if let Some(value) = schema.get("example").or_else(|| schema.get("default")) {
        return value.clone();
    }
    if let Some(value) = schema
        .get("enum")
        .and_then(Value::as_array)
        .and_then(|v| v.first())
    {
        return value.clone();
    }
    if let Some(parts) = schema.get("allOf").and_then(Value::as_array) {
        let mut object = Map::new();
        for part in parts {
            if let Value::Object(values) = minimal_json(document, part, depth + 1) {
                object.extend(values);
            }
        }
        return Value::Object(object);
    }
    if let Some(part) = schema
        .get("oneOf")
        .or_else(|| schema.get("anyOf"))
        .and_then(Value::as_array)
        .and_then(|v| v.iter().find(|v| v.get("type") != Some(&json!("null"))))
    {
        return minimal_json(document, part, depth + 1);
    }
    let kind = schema
        .get("type")
        .and_then(|value| match value {
            Value::String(kind) => Some(kind.as_str()),
            Value::Array(kinds) => kinds
                .iter()
                .filter_map(Value::as_str)
                .find(|kind| *kind != "null"),
            _ => None,
        })
        .unwrap_or_else(|| {
            if schema.get("properties").is_some() {
                "object"
            } else {
                "string"
            }
        });
    match kind {
        "object" => {
            let properties = schema
                .get("properties")
                .and_then(Value::as_object)
                .cloned()
                .unwrap_or_default();
            let required = schema
                .get("required")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str);
            Value::Object(
                required
                    .filter_map(|name| {
                        properties.get(name).map(|child| {
                            (name.to_string(), minimal_json(document, child, depth + 1))
                        })
                    })
                    .collect(),
            )
        }
        "array" => Value::Array(vec![minimal_json(
            document,
            schema.get("items").unwrap_or(&Value::Null),
            depth + 1,
        )]),
        "integer" => json!(
            schema
                .get("minimum")
                .and_then(Value::as_i64)
                .unwrap_or(1)
                .max(1)
        ),
        "number" => json!(
            schema
                .get("minimum")
                .and_then(Value::as_f64)
                .unwrap_or(1.0)
                .max(1.0)
        ),
        "boolean" => json!(false),
        "null" => Value::Null,
        _ => match schema.get("format").and_then(Value::as_str) {
            Some("uuid") => json!(NIL),
            Some("date") => json!("2026-01-01"),
            Some("date-time") => json!("2026-01-01T00:00:00Z"),
            Some("uri" | "url") => json!("https://example.invalid/"),
            Some("email") => json!("test@example.invalid"),
            _ => json!("coverage-smoke-value"),
        },
    }
}

fn request_body(document: &Value, method: &str, template: &str) -> Value {
    document["paths"][template][method.to_ascii_lowercase()]["requestBody"]["content"]
        ["application/json"]["schema"]
        .as_object()
        .map_or_else(|| json!({}), |_| {
            minimal_json(
                document,
                &document["paths"][template][method.to_ascii_lowercase()]["requestBody"]["content"]
                    ["application/json"]["schema"],
                0,
            )
        })
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn every_mounted_api_handler_refuses_inert_input_without_an_internal_error() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let document: Value = request.get("/api-docs/openapi.json").await.json();
        let routes: BTreeSet<_> = App::routes(&ctx)
            .mounted()
            .into_iter()
            .filter(|(_, path)| path == "/api" || path.starts_with("/api/"))
            .collect();
        assert!(routes.len() > 500, "route census unexpectedly small");

        let mut internal_errors = Vec::new();
        for (method, template) in routes {
            let path = inert_path(&template);
            let body = request_body(&document, &method, &template);
            let response = match method.as_str() {
                "GET" => request.get(&path).await,
                "POST" => request.post(&path).json(&body).await,
                "PUT" => request.put(&path).json(&body).await,
                "PATCH" => request.patch(&path).json(&body).await,
                "DELETE" => request.delete(&path).await,
                // The service declares no method-agnostic API handler today.
                other => panic!("unhandled mounted method {other} {template}"),
            };
            if response.status_code() == 500 {
                internal_errors.push(format!(
                    "{method} {template} via {path}: {}",
                    response.text()
                ));
            }
        }
        assert!(
            internal_errors.is_empty(),
            "inert requests reached internal errors:\n{}",
            internal_errors.join("\n")
        );
    })
    .await;
}

#[tokio::test]
#[serial]
#[ignore = "requires PostgreSQL (DB_URL); run with `cargo test -- --ignored`"]
async fn every_mounted_get_handler_accepts_a_live_plan_without_an_internal_error() {
    super::isolate_search_index();
    request::<App, _, _>(|request, ctx| async move {
        let plan: Value = request
            .post("/api/plans")
            .json(&json!({ "kind": "Project", "name": "Router coverage plan" }))
            .await
            .json();
        let plan = plan["pid"].as_str().unwrap().to_string();
        let task: Value = request
            .post(&format!("/api/plans/{plan}/tasks"))
            .json(&json!({
                "title": "Router coverage task",
                "start_date": "2026-01-05",
                "duration_days": 5
            }))
            .await
            .json();
        let task = task["pid"].as_str().unwrap().to_string();

        let routes: BTreeSet<_> = App::routes(&ctx)
            .mounted()
            .into_iter()
            .filter(|(method, path)| {
                method == "GET"
                    && (path == "/api" || path.starts_with("/api/"))
                    && path != "/api/plans/{pid}/live"
            })
            .collect();
        assert!(routes.len() > 150, "GET route census unexpectedly small");

        let mut internal_errors = Vec::new();
        for (method, template) in routes {
            let path = seeded_path(&template, &plan, &task);
            let response = request.get(&path).await;
            if response.status_code() == 500 {
                internal_errors.push(format!(
                    "{method} {template} via {path}: {}",
                    response.text()
                ));
            }
        }
        assert!(
            internal_errors.is_empty(),
            "live-plan GETs reached internal errors:\n{}",
            internal_errors.join("\n")
        );
    })
    .await;
}
