//! The `AsyncAPI` 3.0 description of what this service sends: the per-plan live server-sent events and the outbound
//! webhooks (NC-11). Hand-written like the `OpenAPI` document, and kept honest by tests: the file `asyncapi.json` is
//! current, every reference resolves, and every live event kind the code publishes is described here.

use serde_json::{Value, json};

/// The live event kinds a plan's stream publishes, with what each carries. `resync` is sent by the stream itself.
const LIVE_KINDS: &[(&str, &str)] = &[
    (
        "task_created",
        "A task was created: `task_pid`, `title`, `status`.",
    ),
    (
        "task_updated",
        "A task was edited: identifiers and the changed fact.",
    ),
    (
        "task_moved",
        "A task changed status: identifiers, the old and new status.",
    ),
    (
        "raci_updated",
        "A task's RACI assignment changed: identifiers and the changed fact.",
    ),
    (
        "comment_added",
        "A comment was added: identifiers, no more.",
    ),
    (
        "resync",
        "Sent by the stream itself when the client is owed more than it remembers (`reason` is `gap` or `lagged`): refetch.",
    ),
];

/// The document.
#[must_use]
#[expect(clippy::too_many_lines, reason = "one JSON document literal")]
pub fn spec() -> Value {
    let mut messages = serde_json::Map::new();
    for (kind, text) in LIVE_KINDS {
        messages.insert(
            format!("live_{kind}"),
            json!({
                "name": kind,
                "title": kind,
                "summary": text,
                "payload": { "$ref": "#/components/schemas/LiveEvent" },
            }),
        );
    }
    let live_refs: Vec<Value> = LIVE_KINDS
        .iter()
        .map(|(kind, _)| json!({ "$ref": format!("#/channels/planLive/messages/live_{kind}") }))
        .collect();
    let live_channel_messages: serde_json::Map<String, Value> = LIVE_KINDS
        .iter()
        .map(|(kind, _)| {
            (
                format!("live_{kind}"),
                json!({ "$ref": format!("#/components/messages/live_{kind}") }),
            )
        })
        .collect();
    let mut doc = json!({
        "asyncapi": "3.0.0",
        "info": {
            "title": "Project Portfolio Management Service events",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "What the service sends: a server-sent event stream per plan, and outbound webhooks. Webhook deliveries are signed with HMAC-SHA256 over `<X-Mxi-Timestamp>.` and the exact request body bytes (header `X-Mxi-Signature-V2`; a receiver should refuse a timestamp more than 300 seconds from its clock), and, for one more release, over the body alone (`X-Mxi-Signature`); a receiver deduplicates on `X-Mxi-Event-Id`. A target chooses `native`, `cloudevents_structured` or `cloudevents_binary`."
        },
        "defaultContentType": "application/json",
        "servers": {
            "service": {
                "host": "localhost:5150",
                "protocol": "http",
                "description": "The service itself; the stream is `GET /api/plans/{pid}/live`."
            }
        },
        "channels": {
            "planLive": {
                "address": "/api/plans/{pid}/live",
                "title": "A plan's live events (server-sent events)",
                "description": "`id:` is a monotonic number, so `Last-Event-ID` resumes; the stream says `resync` when the gap is too large. Query: `limit`, `idle_ms`.",
                "parameters": { "pid": { "description": "The plan's pid." } },
                "messages": live_channel_messages,
            },
            "webhook": {
                "address": "{receiver url}",
                "title": "Outbound webhook (POST to each configured target)",
                "description": "Best effort, retried on 5xx and transport failure, not on 4xx.",
                "messages": {
                    "native": { "$ref": "#/components/messages/webhookNative" },
                    "cloudeventsStructured": { "$ref": "#/components/messages/webhookCloudEventsStructured" },
                    "cloudeventsBinary": { "$ref": "#/components/messages/webhookCloudEventsBinary" }
                }
            }
        },
        "operations": {
            "receivePlanLive": {
                "action": "send",
                "channel": { "$ref": "#/channels/planLive" },
                "summary": "The service sends these to a client reading a plan's stream.",
                "messages": live_refs,
            },
            "deliverWebhook": {
                "action": "send",
                "channel": { "$ref": "#/channels/webhook" },
                "summary": "The service POSTs the event to each configured webhook target.",
                "messages": [
                    { "$ref": "#/channels/webhook/messages/native" },
                    { "$ref": "#/channels/webhook/messages/cloudeventsStructured" },
                    { "$ref": "#/channels/webhook/messages/cloudeventsBinary" }
                ]
            }
        },
        "components": {
            "messages": {
                "webhookNative": {
                    "name": "native",
                    "contentType": "application/json",
                    "headers": { "$ref": "#/components/schemas/WebhookHeaders" },
                    "payload": { "$ref": "#/components/schemas/Envelope" }
                },
                "webhookCloudEventsStructured": {
                    "name": "cloudevents_structured",
                    "contentType": "application/cloudevents+json",
                    "headers": { "$ref": "#/components/schemas/WebhookHeaders" },
                    "payload": { "$ref": "#/components/schemas/CloudEvent" }
                },
                "webhookCloudEventsBinary": {
                    "name": "cloudevents_binary",
                    "summary": "The `ce-specversion`, `ce-id`, `ce-source`, `ce-type` and `ce-subject` headers carry the CloudEvents attributes; the body is the envelope.",
                    "contentType": "application/json",
                    "headers": { "$ref": "#/components/schemas/CloudEventsBinaryHeaders" },
                    "payload": { "$ref": "#/components/schemas/Envelope" }
                },
            },
            "schemas": {
                "LiveEvent": {
                    "type": "object",
                    "required": ["plan_pid", "kind", "data"],
                    "properties": {
                        "plan_pid": { "type": "string", "format": "uuid" },
                        "kind": { "type": "string" },
                        "data": { "type": "object", "description": "Identifiers and the changed fact, no more." }
                    }
                },
                "Envelope": {
                    "type": "object",
                    "required": ["event_id", "schema_version", "entity", "kind", "pid", "seq", "name"],
                    "properties": {
                        "event_id": { "type": "string", "format": "uuid", "description": "The deduplication key." },
                        "schema_version": { "type": "integer" },
                        "entity": { "type": "string" },
                        "kind": { "type": "string", "enum": ["created", "updated", "deleted", "merged"] },
                        "pid": { "type": "string", "format": "uuid" },
                        "seq": { "type": "integer" },
                        "actor": { "type": ["string", "null"] },
                        "name": { "type": "string" }
                    }
                },
                "WebhookHeaders": {
                    "type": "object",
                    "required": ["X-Mxi-Event-Id", "X-Mxi-Signature"],
                    "properties": {
                        "X-Mxi-Event-Id": { "type": "string", "format": "uuid" },
                        "X-Mxi-Delivery-Id": { "type": "string", "format": "uuid", "description": "One delivery, all its attempts. A redelivery has a new one; the event id stays." },
                        "X-Mxi-Timestamp": { "type": "integer", "description": "Unix seconds when this attempt was signed. Refuse a delivery more than 300 seconds from your clock." },
                        "X-Mxi-Signature": { "type": "string", "description": "`<scheme>.<key id>:<hex>`, HMAC-SHA256 over the exact body bytes. Kept for one release; verify `X-Mxi-Signature-V2`." },
                        "X-Mxi-Signature-V2": { "type": "string", "description": "The same MAC over `<X-Mxi-Timestamp>.` followed by the exact body bytes." }
                    }
                },
                "CloudEventsBinaryHeaders": {
                    "type": "object",
                    "required": ["X-Mxi-Event-Id", "X-Mxi-Signature", "ce-specversion", "ce-id", "ce-source", "ce-type"],
                    "properties": {
                        "X-Mxi-Event-Id": { "type": "string", "format": "uuid" },
                        "X-Mxi-Signature": { "type": "string" },
                        "ce-specversion": { "type": "string", "enum": ["1.0"] },
                        "ce-id": { "type": "string" },
                        "ce-source": { "type": "string", "format": "uri-reference" },
                        "ce-type": { "type": "string" },
                        "ce-subject": { "type": "string" },
                        "ce-time": { "type": "string", "format": "date-time" }
                    }
                },
                "CloudEvent": {
                    "type": "object",
                    "required": ["specversion", "id", "source", "type"],
                    "properties": {
                        "specversion": { "type": "string", "enum": ["1.0"] },
                        "id": { "type": "string" },
                        "source": { "type": "string", "format": "uri-reference" },
                        "type": { "type": "string", "description": "`org.project-portfolio-management.<entity>.<kind>`" },
                        "subject": { "type": "string" },
                        "time": { "type": "string", "format": "date-time" },
                        "datacontenttype": { "type": "string", "enum": ["application/json"] },
                        "data": { "$ref": "#/components/schemas/Envelope" }
                    }
                }
            }
        }
    });
    if let Some(m) = doc["components"]["messages"].as_object_mut() {
        m.extend(messages);
    }
    doc
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The document as a file for tools that read one. Regenerate with
    /// `UPDATE_ASYNCAPI_JSON=1 cargo test --lib asyncapi_json_file`.
    #[test]
    fn asyncapi_json_file_is_current() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("asyncapi.json");
        let mut text = serde_json::to_string_pretty(&spec()).expect("serialise");
        text.push('\n');
        if std::env::var_os("UPDATE_ASYNCAPI_JSON").is_some() {
            std::fs::write(&path, &text).expect("write asyncapi.json");
        }
        let have = std::fs::read_to_string(&path).unwrap_or_default();
        assert!(
            have == text,
            "asyncapi.json is stale; regenerate with UPDATE_ASYNCAPI_JSON=1 cargo test --lib asyncapi_json_file"
        );
    }

    fn refs(v: &Value, out: &mut Vec<String>) {
        match v {
            Value::Object(m) => {
                for (k, x) in m {
                    if k == "$ref" {
                        out.push(x.as_str().unwrap_or_default().to_string());
                    }
                    refs(x, out);
                }
            }
            Value::Array(a) => a.iter().for_each(|x| refs(x, out)),
            _ => {}
        }
    }

    /// A structural lint (the official `AsyncAPI` linter is not run here): version 3, the required
    /// top-level members, every `$ref` resolves, every operation names a declared channel and
    /// messages that channel declares.
    #[test]
    fn the_document_is_structurally_sound_and_every_reference_resolves() {
        let doc = spec();
        assert_eq!(doc["asyncapi"], "3.0.0");
        for key in ["info", "channels", "operations", "components"] {
            assert!(doc.get(key).is_some(), "{key}");
        }
        assert!(doc["info"]["title"].is_string() && doc["info"]["version"].is_string());
        let mut all = Vec::new();
        refs(&doc, &mut all);
        assert!(!all.is_empty());
        for r in &all {
            let pointer = r
                .strip_prefix('#')
                .unwrap_or_else(|| panic!("local ref {r}"));
            assert!(doc.pointer(pointer).is_some(), "{r} does not resolve");
        }
        for (name, op) in doc["operations"].as_object().unwrap() {
            assert!(
                matches!(op["action"].as_str(), Some("send" | "receive")),
                "{name}"
            );
            let channel = op["channel"]["$ref"].as_str().unwrap();
            for m in op["messages"].as_array().unwrap() {
                let m = m["$ref"].as_str().unwrap();
                assert!(
                    m.starts_with(&format!("{channel}/messages/")),
                    "{name}: {m}"
                );
            }
        }
    }

    /// Every live event kind the code publishes is described, and nothing is described that is not published.
    #[test]
    fn every_published_live_event_kind_is_described() {
        let mut published = std::collections::BTreeSet::new();
        let mut stack = vec![std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(dir).unwrap().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|e| e == "rs")
                    && !path.ends_with("live.rs")
                    && !path.ends_with("asyncapi.rs")
                {
                    let text = std::fs::read_to_string(&path).unwrap();
                    for chunk in text.split("live::publish(").skip(1) {
                        // The kind is the first string literal after the first comma.
                        let after = &chunk[chunk.find(',').unwrap() + 1..];
                        let start = after.find('"').unwrap() + 1;
                        let end = after[start..].find('"').unwrap();
                        published.insert(after[start..start + end].to_string());
                    }
                }
            }
        }
        let described: std::collections::BTreeSet<String> = LIVE_KINDS
            .iter()
            .map(|(k, _)| (*k).to_string())
            .filter(|k| k != "resync")
            .collect();
        assert_eq!(published, described);
    }
}
