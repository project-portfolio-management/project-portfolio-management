//! `OpenAPI` entries for the capabilities added after the first waves
//! (backlog items LT-/RM-/PL-n in the repo `tasks.md`). Kept apart from
//! [`crate::openapi`] so that file does not keep growing; its `paths()` is
//! merged into the main document, and the two-way route-parity test there
//! covers these entries exactly as it covers the rest.

// Builder helpers take owned `Value`s so call sites read as a literal tree.
#![allow(clippy::needless_pass_by_value)]

use serde_json::{Value, json};

/// A required path parameter named `name`, a UUID.
#[must_use]
pub fn uuid_param(name: &str) -> Value {
    json!({ "name": name, "in": "path", "required": true,
        "schema": { "type": "string", "format": "uuid" } })
}

/// A query parameter.
#[must_use]
pub fn query(name: &str, required: bool, description: &str) -> Value {
    json!({ "name": name, "in": "query", "required": required, "description": description,
        "schema": { "type": "string" } })
}

/// Shorthand for one operation. `responses` is `(status, description)`.
#[must_use]
pub fn op(
    tag: &str,
    summary: &str,
    description: &str,
    parameters: Vec<Value>,
    responses: &[(&str, &str)],
) -> Value {
    let mut resp = serde_json::Map::new();
    for (code, text) in responses {
        resp.insert((*code).to_string(), json!({ "description": text }));
    }
    json!({
        "tags": [tag],
        "summary": summary,
        "description": description,
        "parameters": parameters,
        "responses": Value::Object(resp),
    })
}

/// Merge `src`'s top-level keys into `dst`, merging per-path method maps.
pub fn merge(dst: &mut Value, src: Value) {
    let (Some(d), Some(s)) = (dst.as_object_mut(), src.as_object()) else {
        return;
    };
    for (path, item) in s {
        match d.get_mut(path) {
            Some(existing) => {
                if let (Some(e), Some(i)) = (existing.as_object_mut(), item.as_object()) {
                    for (m, v) in i {
                        e.insert(m.clone(), v.clone());
                    }
                }
            }
            None => {
                d.insert(path.clone(), item.clone());
            }
        }
    }
}

/// All the entries in this module.
#[must_use]
pub fn paths() -> Value {
    let mut all = json!({});
    // Feature path functions are merged here as they land.
    merge(&mut all, component_paths());
    merge(&mut all, custom_field_paths());
    merge(&mut all, watcher_paths());
    merge(&mut all, approval_paths());
    merge(&mut all, relation_paths());
    merge(&mut all, rollup_paths());
    merge(&mut all, feed_paths());
    merge(&mut all, self_service_paths());
    all
}

/// Task components (PL-11).
fn component_paths() -> Value {
    let pid = uuid_param("pid");
    let cpid = uuid_param("c_pid");
    json!({
        "/api/plans/{pid}/components": {
            "post": op("components", "Create a component (a named grouping of the plan's tasks)",
                "Names are unique within a plan, case-insensitively; a plan holds a bounded number.",
                vec![pid.clone()],
                &[("200", "Created"), ("404", "Unknown plan"), ("409", "A component with this name already exists"), ("422", "Blank or over-long name, malformed lead_ref, or the plan is full")]),
            "get": op("components", "A plan's live components",
                "Plain list of stored components; the per-component task counts are in /components/summary.",
                vec![pid.clone()], &[("200", "Components"), ("404", "Unknown plan")])
        },
        "/api/plans/{pid}/components/summary": {
            "get": op("components", "Open, done and total tasks per component, with an unassigned bucket",
                "Every component is reported, empty ones at zero, plus a final unassigned bucket for tasks with no component (or whose component no longer exists), so the bucket totals always sum to the plan's task total and grouping never loses work.",
                vec![pid.clone()], &[("200", "buckets, total"), ("404", "Unknown plan")])
        },
        "/api/plans/{pid}/components/{c_pid}": {
            "put": op("components", "Rename a component or change its lead", "Same name rules as create.",
                vec![pid.clone(), cpid.clone()],
                &[("200", "Updated"), ("404", "Unknown plan or component"), ("409", "Name already used"), ("422", "Invalid name or lead_ref")]),
            "delete": op("components", "Delete a component",
                "Refused with 409 while any live task still belongs to it; tasks are never silently orphaned.",
                vec![pid, cpid], &[("200", "Deleted (soft)"), ("404", "Unknown plan or component"), ("409", "Tasks still belong to it")])
        }
    })
}

/// Typed custom fields (RM-1).
fn custom_field_paths() -> Value {
    let subject = vec![
        query("applies_to", true, "plan or task"),
        query("subject_pid", true, "The plan or task pid"),
    ];
    json!({
        "/api/custom-fields": {
            "post": op("custom-fields", "Declare a typed custom field for plans or tasks",
                "Types: text, date (YYYY-MM-DD), bool, integer, list (one of options), multi_list (a distinct subset of options). A plan field may be scoped to certain kinds and may be required. Values are stored apart from the plan payload, so a custom value can never reach the matcher.",
                vec![], &[("200", "Declared"), ("409", "A live field with this key already exists for this subject type"), ("422", "Invalid key, label, type, applies_to, kinds or options")]),
            "get": op("custom-fields", "Live field definitions",
                "Plain list of declared definitions, optionally narrowed by applies_to; withdrawn definitions are not listed.",
                vec![query("applies_to", false, "plan or task")], &[("200", "Definitions")])
        },
        "/api/custom-fields/{pid}": {
            "delete": op("custom-fields", "Withdraw a definition",
                "Soft-delete only: values already stored are kept and stay readable, marked withdrawn. New values for a withdrawn key are refused.",
                vec![uuid_param("pid")], &[("200", "Withdrawn"), ("404", "Unknown definition")])
        },
        "/api/custom-fields/values": {
            "put": op("custom-fields", "Set or clear a subject's custom values",
                "Atomic: one invalid value refuses the whole call and nothing is written. A null clears a value, except for a required field. A kind-scoped field is refused on a plan of another kind. Applies the owning plan's record-level read check.",
                vec![], &[("200", "The subject's values"), ("403", "Owning plan not readable"), ("404", "Unknown subject"), ("422", "Unknown key, wrong type, value outside the options, kind mismatch, or clearing a required field")]),
            "get": op("custom-fields", "A subject's custom values and the required fields still unset",
                "Includes values whose definition has since been withdrawn (marked withdrawn), so withdrawing a field never hides stored data. missing_required lists the live required fields with no value yet.",
                subject, &[("200", "values, missing_required"), ("403", "Owning plan not readable"), ("404", "Unknown subject"), ("422", "applies_to not plan or task")])
        },
        "/api/custom-fields/search": {
            "get": op("custom-fields", "Subjects whose custom field equals a value",
                "Equality after the same normalisation a write gets (multi_list matches when the list contains the value). Subjects whose owning plan the caller cannot read are omitted. At most 5000 stored values are scanned and the response says truncated if more exist.",
                vec![query("applies_to", true, "plan or task"), query("key", true, "The field key"), query("value", true, "The value to match")],
                &[("200", "count, pids, truncated"), ("422", "Unknown field")])
        }
    })
}

/// Watchers (RM-3).
fn watcher_paths() -> Value {
    let subject = vec![
        query(
            "subject_kind",
            true,
            "plan, task, risk, proposal, idea or canvas_item",
        ),
        query("subject_pid", true, "The subject pid"),
    ];
    json!({
        "/api/watchers": {
            "post": op("watchers", "Follow a subject (idempotent)",
                "Requires a bearer token and the ability to read the subject (the owning plan's record-level check applies). A watcher joins the notification targets of a comment and, for a task, of a status move: each told exactly once, never the person who made the change. Watching grants no access and the notification text is generic.",
                vec![], &[("200", "{ watching, created }"), ("401", "Missing or invalid bearer token"), ("403", "Owning plan not readable"), ("404", "Unknown subject"), ("422", "Unknown subject_kind")]),
            "get": op("watchers", "How many follow a subject, and whether the caller does",
                "Other watchers are never named: only a count and the caller's own state are returned.",
                subject.clone(), &[("200", "{ count, mine }"), ("401", "Missing or invalid bearer token"), ("404", "Unknown subject")]),
            "delete": op("watchers", "Stop following a subject (idempotent)",
                "Removes only the caller's own watch.", subject, &[("200", "Removed (or already absent)"), ("401", "Missing or invalid bearer token")])
        },
        "/api/me/watching": {
            "get": op("watchers", "What the caller follows, newest first",
                "Private to the caller; capped at 500 entries.", vec![],
                &[("200", "{ count, items }"), ("401", "Missing or invalid bearer token")])
        }
    })
}

/// Approval requests (PL-1).
fn approval_paths() -> Value {
    json!({
        "/api/approvals": {
            "post": op("approvals", "Ask a named person to approve a subject",
                "Subjects: plan, task, risk, proposal, idea, canvas_item, budget_line, scenario. Names exactly one approver (a person: or worker: reference). The approver is notified; the owning plan's record-level read check applies. Requires a bearer token.",
                vec![], &[("200", "The pending request"), ("401", "Missing or invalid bearer token"), ("403", "Owning plan not readable"), ("404", "Unknown subject"), ("422", "Unknown subject_kind, or a malformed approver_ref")]),
            "get": op("approvals", "Requests involving the caller, or attached to one subject",
                "Without subject_kind and subject_pid it lists requests where the caller is the approver or the requester (narrow with mine=approver|requester); with them it lists that subject's requests, subject to the read check. Capped at 500, newest first.",
                vec![query("mine", false, "approver or requester"), query("status", false, "pending, approved, rejected or withdrawn"), query("subject_kind", false, "With subject_pid"), query("subject_pid", false, "With subject_kind")],
                &[("200", "Requests"), ("401", "Missing or invalid bearer token"), ("422", "Invalid mine or status")])
        },
        "/api/approvals/{pid}/decide": {
            "post": op("approvals", "The named approver decides",
                "Only the named approver may decide (403 otherwise); a rejection must carry a comment (422); a decision is final, so a request that is not pending is 409. The decision and comment are audited and the requester is notified.",
                vec![uuid_param("pid")], &[("200", "The decided request"), ("401", "Missing or invalid bearer token"), ("403", "Not the named approver, or the owning plan is not readable"), ("404", "Unknown request"), ("409", "Already decided or withdrawn"), ("422", "Unknown decision, a rejection without a comment, or an over-long comment")])
        },
        "/api/approvals/{pid}/withdraw": {
            "post": op("approvals", "The requester withdraws a pending request",
                "Writes no decision. A request that is already decided or withdrawn is 409.",
                vec![uuid_param("pid")], &[("200", "The withdrawn request"), ("401", "Missing or invalid bearer token"), ("403", "Not the requester"), ("404", "Unknown request"), ("409", "Not pending")])
        }
    })
}

/// Task relations (LT-10).
fn relation_paths() -> Value {
    let p = vec![uuid_param("pid"), uuid_param("t_pid")];
    let mut with_r = p.clone();
    with_r.push(uuid_param("r_pid"));
    json!({
        "/api/plans/{pid}/tasks/{t_pid}/relations": {
            "post": op("task-relations", "Relate this task to another (related, blocks, precedes, duplicates, copied_from)",
                "From is this task, to is to_task_pid; cross-plan relations are allowed. blocks: from blocks to, and to cannot be finished while from is open. precedes: from precedes to, and to must be due after from when both are dated; this is enforced again when either due date is edited. duplicates: from is a duplicate of to, and finishing to closes from. related and copied_from are informational. blocks and precedes must stay acyclic. Every refusal names the relation that caused it.",
                p.clone(), &[("200", "The relation"), ("404", "Unknown plan or task"), ("409", "The relation already exists (related is symmetric)"), ("422", "Unknown kind, a self-relation, a target that is not a live task, a cycle, or a precedes that the due dates already violate")]),
            "get": op("task-relations", "A task's relations in both directions",
                "Each carries its direction (outgoing or incoming) and a summary of the other task. A relation whose other task sits in a plan the caller cannot read is omitted and counted in hidden; a relation to a deleted task is not shown.",
                p, &[("200", "relations, hidden"), ("404", "Unknown plan or task")])
        },
        "/api/plans/{pid}/tasks/{t_pid}/relations/{r_pid}": {
            "delete": op("task-relations", "Remove a relation",
                "Soft-delete; the tasks themselves are untouched.", with_r,
                &[("200", "Removed"), ("404", "Unknown plan, task or relation")])
        }
    })
}

/// Subtask roll-up (RM-5).
fn rollup_paths() -> Value {
    json!({
        "/api/plans/{pid}/tasks/{t_pid}/rollup": {
            "get": op("task-tree", "A parent task's figures derived from its subtasks",
                "Derived on read, never stored, so it cannot drift: due is the latest descendant due date; estimate is the sum of descendants' story points; progress is estimate-weighted (basis points) over the descendants that have an estimate, which are the only ones weighed; those without an estimate are named in excluded_unestimated and never counted as zero; spent is recorded minutes across the parent and its descendants. A figure that cannot be derived is null with a reason (no_children, no_estimates, no_dated_children), never zero. Parents are set with parent_task_pid on task create or update: same plan, no cycle, at most ten deep.",
                vec![uuid_param("pid"), uuid_param("t_pid")],
                &[("200", "task_pid, rollup"), ("404", "Unknown plan or task")])
        }
    })
}

/// Feeds (LT-7, RM-6).
fn feed_paths() -> Value {
    let token = vec![query(
        "token",
        true,
        "A feed token from POST /api/me/feed-token",
    )];
    json!({
        "/api/me/feed-token": {
            "post": op("feeds", "Mint a feed token",
                "Requires a bearer token. The token is shown once in this response; only its SHA-256 is stored. It lets a calendar app or feed reader fetch the caller's own feeds without a bearer header, and nothing else.",
                vec![], &[("200", "token, calendar_url, atom_url"), ("401", "Missing or invalid bearer token")]),
            "delete": op("feeds", "Revoke all of the caller's feed tokens",
                "Takes effect on the next fetch; every feed URL minted so far stops working.",
                vec![], &[("200", "Revoked"), ("401", "Missing or invalid bearer token")])
        },
        "/api/feeds/calendar.ics": {
            "get": op("feeds", "The owner's due tasks, milestones and sprint dates as an iCalendar feed",
                "Authenticated by the token alone (this path is exempt from the bearer guard). Scope is the owner's own work: open tasks assigned to them with a due date, plus the open milestones and sprint dates of the plans those tasks sit in. RFC 5545: CRLF line ends, lines folded at 75 octets, all-day events with the exclusive next-day end, and stable UIDs (task-, milestone-, sprint-start- and sprint-end- plus the pid) so a calendar app updates an item instead of duplicating it. A token that is unknown, malformed or revoked is 401.",
                token.clone(), &[("200", "text/calendar"), ("401", "Unknown, malformed or revoked token")])
        },
        "/api/feeds/me.atom": {
            "get": op("feeds", "The owner's notifications as an Atom feed",
                "Authenticated by the token alone. The newest 100 notifications for the token's owner, with stable urn:uuid entry ids; the feed's updated time follows its newest entry, so an unchanged feed is byte-identical and If-None-Match returns 304. A token that is unknown, malformed or revoked is 401.",
                token, &[("200", "application/atom+xml"), ("304", "Not modified"), ("401", "Unknown, malformed or revoked token")])
        }
    })
}

/// POST twins of the self-service DELETEs. The default policy reserves DELETE
/// for admins, so a write-level user could not otherwise undo their own
/// follow, reaction, preference, comment or feed token.
fn self_service_paths() -> Value {
    let note = "The POST twin of the DELETE route: under the default policy DELETE needs the admin attribute, but a person must always be able to undo their own action with ordinary write access. Same behaviour, idempotent where the DELETE is.";
    json!({
        "/api/me/feed-token/revoke": {
            "post": op("feeds", "Revoke all of the caller's feed tokens (POST twin)", note, vec![],
                &[("200", "Revoked"), ("401", "Missing or invalid bearer token")])
        },
        "/api/watchers/unwatch": {
            "post": op("watchers", "Stop following a subject (POST twin)", note, vec![],
                &[("200", "Removed (or already absent)"), ("401", "Missing or invalid bearer token")])
        },
        "/api/reactions/remove": {
            "post": op("discussion", "Remove the caller's own reaction (POST twin)", note, vec![],
                &[("200", "Removed (or already absent)"), ("422", "Unknown kind")])
        },
        "/api/me/notification-prefs/reset": {
            "post": op("me", "Restore the default for one notification kind (POST twin)", note, vec![],
                &[("200", "Reset"), ("401", "Missing or invalid bearer token"), ("422", "Unknown kind or channel")])
        },
        "/api/comments/{pid}/delete": {
            "post": op("discussion", "Soft-delete a comment, author only (POST twin)", note,
                vec![uuid_param("pid")],
                &[("200", "Deleted"), ("403", "Not the author"), ("404", "Unknown comment")])
        }
    })
}
