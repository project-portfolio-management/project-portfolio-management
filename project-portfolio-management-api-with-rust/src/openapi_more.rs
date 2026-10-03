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
    merge(&mut all, access_paths());
    merge(&mut all, bulk_task_paths());
    merge(&mut all, timesheet_paths());
    merge(&mut all, template_paths());
    merge(&mut all, live_paths());
    merge(&mut all, planner_paths());
    merge(&mut all, attachment_paths());
    merge(&mut all, bulk_paths());
    merge(&mut all, guest_paths());
    merge(&mut all, link_paths());
    merge(&mut all, pdf_paths());
    merge(&mut all, sink_paths());
    merge(&mut all, attachment_search_paths());
    merge(&mut all, report_schedule_paths());
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

/// Private plans (RM-8).
fn access_paths() -> Value {
    let pid = vec![uuid_param("pid")];
    json!({
        "/api/plans/{pid}/access": {
            "get": op("access", "Whether a plan is private, and its members",
                "A plan is public unless someone made it private. This route is itself behind the privacy guard: a non-member gets 404 here too.",
                pid.clone(), &[("200", "plan_pid, private, members"), ("404", "Unknown plan, or a private plan the caller cannot see")]),
            "put": op("access", "Make a plan private (naming its members) or public again",
                "Members are user ids (UUIDs, the token sub). Making a plan private always adds the caller as a member, so it can never be locked away from the person who locked it. Every change is audited with the before and after. WHAT IS ENFORCED: any request addressed to /api/plans/{pid} or beneath it answers 404 (never 403, so existence is not disclosed) to a non-member who is not an admin; plan lists, search and duplicate checks omit it; discussion, watchers, approvals, custom fields, task relations and My work resolve their subjects through the same check. A private plan with no identity at all is hidden (fail closed). WHAT IS NOT ENFORCED: cross-plan aggregate views (at-a-glance, executive, board, financials, reports, capacity, scenarios) are computed over every plan and still include a private plan's figures.",
                pid, &[("200", "plan_pid, private, members"), ("401", "Missing or invalid bearer token"), ("404", "Unknown plan, or a private plan the caller cannot see"), ("422", "Invalid members, or an identity that is not a user id")])
        }
    })
}

/// Bulk task operations (RM-9).
fn bulk_task_paths() -> Value {
    let results = "Every requested task gets its own result (ok or refused, with the reason or a note), so one bad item never aborts the rest and the counts reconcile: ok + refused equals requested after de-duplication. At most 200 ids. Each task goes through the same read check as any task access, and every change is audited.";
    let r = |tail: &str| format!("{results} {tail}");
    json!({
        "/api/tasks/bulk": {
            "post": op("tasks-bulk", "Set the same fields on many tasks",
                &r("Settable: assignee_ref, due_date, component_pid (null clears). status is deliberately not bulk-editable: a status change must pass the workflow, WIP, role and blocker rules that belong to the board move. A component must belong to each task's own plan, and a due date that would break a precedes relation refuses that task. Assigning notifies and auto-watches like a single assignment."),
                vec![], &[("200", "requested, ok, refused, results"), ("422", "No ids, too many, an unknown or malformed field, or an empty set")])
        },
        "/api/tasks/copy": {
            "post": op("tasks-bulk", "Copy tasks into a plan",
                &r("Copies get fresh pids, the target workflow's initial status and no transition history; the assignee is kept only with keep_assignee. A copied_from relation links each copy to its source, and the source's custom-field values are carried so a required field is never left empty."),
                vec![], &[("200", "requested, ok, refused, results"), ("422", "No ids, too many, or a target plan the caller cannot see")])
        },
        "/api/tasks/move": {
            "post": op("tasks-bulk", "Move tasks into another plan",
                &r("Re-validated against the target plan's workflow: a status the target lacks maps to its initial state and the result says so. Component, sprint and parent links are plan-specific and are cleared. A task with subtasks is refused until they are moved or detached; a task already in the target plan is refused. Earlier transition history keeps its original plan."),
                vec![], &[("200", "requested, ok, refused, results"), ("422", "No ids, too many, or a target plan the caller cannot see")])
        }
    })
}

/// Timesheets (LT-11) and time reports (RM-10).
fn timesheet_paths() -> Value {
    let units = "All figures are whole minutes, and billable and non-billable are always carried apart (a combined total only ever appears beside the split). A figure with no evidence is null with a reason (no_entries), never zero.";
    json!({
        "/api/timesheets": {
            "get": op("timesheets", "One person's week by day and plan, and its approval state",
                &format!("{units} The person defaults to the caller; reading someone else's week needs the approver right (access=admin or the timesheet_approver role). Plans the reader cannot see are left out and counted in omitted_unreadable. status is open until the week is submitted."),
                vec![query("person", false, "A person:/worker: URN; defaults to the caller"), query("week", true, "An ISO week such as 2026-W40")],
                &[("200", "person, week_start, status, grid, omitted_unreadable"), ("401", "Missing or invalid bearer token"), ("403", "Not the person and not an approver"), ("422", "Invalid week or person")])
        },
        "/api/timesheets/submit": {
            "post": op("timesheets", "Submit your own week",
                "Only the person a week belongs to may submit it. A week with no entries cannot be submitted (422), and a week that is already submitted or approved is 409. A rejected week can be resubmitted.",
                vec![], &[("200", "The week's state"), ("401", "Missing or invalid bearer token"), ("403", "Not your week"), ("409", "Already submitted or approved"), ("422", "No entries, or invalid week")])
        },
        "/api/timesheets/decide": {
            "post": op("timesheets", "Approve or reject a submitted week",
                "Needs the approver right and never applies to the approver's own week. A rejection must carry a comment. Approved is final, and an approved week is locked: a new time entry against it is refused with 409. Audited.",
                vec![], &[("200", "The week's state"), ("401", "Missing or invalid bearer token"), ("403", "Not an approver, or their own week"), ("409", "The week is not submitted"), ("422", "Unknown decision, or a rejection without a comment")])
        },
        "/api/time-reports": {
            "get": op("timesheets", "Time grouped by person, plan, category, billable or month",
                &format!("{units} Dates are calendar dates (no zone); the default window is the last 90 days. JSON, or CSV with format=csv. Plans the caller cannot read are omitted and counted; at most 20000 entries are read and the response says truncated if more exist."),
                vec![query("group_by", true, "person, plan, category, billable or month"), query("from", false, "YYYY-MM-DD"), query("to", false, "YYYY-MM-DD"), query("plan", false, "Narrow to one plan"), query("person", false, "Narrow to one person URN"), query("format", false, "json (default) or csv")],
                &[("200", "groups, split, total_minutes, reason, omitted_unreadable, truncated (or text/csv)"), ("422", "Unknown group_by, or from after to")])
        }
    })
}

/// Plan templates (LT-2).
fn template_paths() -> Value {
    let rules = "A template is a self-contained snapshot: the plan and its readable descendants (at most 200 plans, 2000 tasks, six levels), their tasks, components, milestones and each plan's own task workflow. People and identity are never captured (leads, assignees, external identifiers, same_as, codes, alternate names are dropped), every date is stored as an offset from a base date, and tasks are re-created at the workflow's initial state with no history.";
    json!({
        "/api/plans/{pid}/save-as-template": {
            "post": op("templates", "Save a plan tree as a reusable template",
                &format!("{rules} A descendant the caller cannot read is left out with its subtree and counted in omitted_unreadable. Template names are unique, case-insensitively."),
                vec![uuid_param("pid")], &[("200", "pid, name, plans, tasks, milestones, needs_start_date, omitted_unreadable"), ("404", "Unknown plan, or one the caller cannot see"), ("409", "A template with this name exists"), ("422", "Blank name, or the tree exceeds the caps")])
        },
        "/api/plans/from-template/{tpl}": {
            "post": op("templates", "Create a new plan tree from a template",
                "Every plan is created through the same path as POST /api/plans, so events and audit are written exactly as for a single create. start_date is required when the template captured any date (each date becomes start_date plus its offset). No person reference is copied; tasks start at the workflow's initial state. Not one transaction: if it fails part-way the plans already created are soft-deleted and the failure is reported, so a half-built tree is never left behind.",
                vec![uuid_param("tpl")], &[("200", "root_pid, plans, tasks_created, milestones_created"), ("404", "Unknown template"), ("422", "start_date missing for a dated template, an unreadable snapshot, an invalid plan, or a parent the caller cannot see")])
        },
        "/api/plan-templates": {
            "get": op("templates", "Saved templates (name, description, size)",
                "Plain list of stored templates without their snapshots.", vec![], &[("200", "Templates")])
        },
        "/api/plan-templates/{pid}": {
            "get": op("templates", "A template's full snapshot", rules, vec![uuid_param("pid")], &[("200", "The template"), ("404", "Unknown template")]),
            "delete": op("templates", "Soft-delete a template",
                "Plans already created from it are untouched.", vec![uuid_param("pid")], &[("200", "Deleted"), ("404", "Unknown template")])
        },
        "/api/plan-templates/{pid}/delete": {
            "post": op("templates", "Soft-delete a template (POST twin)",
                "The POST twin of DELETE: under the default policy DELETE needs the admin attribute. Same behaviour.", vec![uuid_param("pid")], &[("200", "Deleted"), ("404", "Unknown template")])
        }
    })
}

/// Live plan updates (PL-3).
fn live_paths() -> Value {
    json!({
        "/api/plans/{pid}/live": {
            "get": op("live", "A text/event-stream of changes to a plan's board",
                "Events: task_created, task_moved, task_updated, comment_added (identifiers and the changed fact only). The plan is read-checked once at connect: one the caller cannot see (record-level policy or a private plan) or that no longer exists is 404 before any event. Each event carries an id; a reconnecting client sends Last-Event-ID and is replayed what it missed from a bounded in-memory ring, or receives a resync event when the gap is larger than the ring remembers, so a gap is never silent. A subscriber that falls behind the bounded channel gets a resync event and is disconnected rather than buffered without limit. Optional limit closes the stream after that many events and idle_ms after that long with nothing to send, for clients that cannot hold a connection. Limits: the hub is per process, so with several replicas an event reaches only the clients connected to the replica that handled the write.",
                vec![uuid_param("pid"), query("limit", false, "Close after this many events"), query("idle_ms", false, "Close after this many milliseconds idle")],
                &[("200", "text/event-stream"), ("404", "Unknown plan, or one the caller cannot see")])
        }
    })
}

/// Personal planner (PL-4).
fn planner_paths() -> Value {
    json!({
        "/api/me/planned-slots": {
            "post": op("planner", "Block time for a task",
                "Requires a bearer token and the ability to read the task. A slot runs forward and is at most 12 hours; instants are UTC. It never changes the task's due date. Overlapping slots are allowed (reported as clashes by GET /api/me/plan, never refused). At most 2000 live slots per person.",
                vec![], &[("200", "The slot"), ("401", "Missing or invalid bearer token"), ("404", "Unknown task, or one the caller cannot read"), ("422", "ends_at not after starts_at, over 12 hours, or too many slots")])
        },
        "/api/me/planned-slots/{pid}": {
            "delete": op("planner", "Remove one of your own slots",
                "Another person's slot is 404, identically, so a slot's existence is not disclosed.", vec![uuid_param("pid")], &[("200", "Removed"), ("401", "Missing or invalid bearer token"), ("404", "Unknown, or not yours")])
        },
        "/api/me/planned-slots/{pid}/delete": {
            "post": op("planner", "Remove one of your own slots (POST twin)",
                "The POST twin of DELETE: under the default policy DELETE needs the admin attribute. Same behaviour.", vec![uuid_param("pid")], &[("200", "Removed"), ("401", "Missing or invalid bearer token"), ("404", "Unknown, or not yours")])
        },
        "/api/me/plan": {
            "get": op("planner", "The caller's slots for one local calendar day, in their zone",
                "day is a calendar date and tz an IANA zone (fixed offsets are refused). The window is built from local midnights, so it is 23 or 25 hours across a daylight-saving change. Each slot carries its UTC instants and the same instants rendered in tz. clashes lists groups of overlapping slots (transitively; back-to-back is not a clash). due_on_day lists the caller's open tasks due that day, i.e. what the day still owes. A slot on a task the caller can no longer read is omitted and counted.",
                vec![query("day", true, "YYYY-MM-DD"), query("tz", true, "An IANA zone")],
                &[("200", "day, tz, window, slots, clashes, due_on_day, omitted_unreadable"), ("401", "Missing or invalid bearer token"), ("422", "Invalid day or tz")])
        }
    })
}

/// File attachments (LT-5).
fn attachment_paths() -> Value {
    let subject = vec![
        query(
            "subject_kind",
            true,
            "plan, task, risk, proposal, idea, canvas_item, budget_line or scenario",
        ),
        query("subject_pid", true, "The subject pid"),
    ];
    let mut upload = subject.clone();
    upload.push(query(
        "filename",
        true,
        "The display name; UTF-8 is preserved, path components are dropped",
    ));
    json!({
        "/api/attachments": {
            "post": op("attachments", "Attach a file to a subject (the raw request body)",
                "Send the file bytes as the request body with its Content-Type. Refused before anything is stored: a content type outside the allow-list (pdf, png, jpeg, gif, webp, txt, csv, markdown, json, docx, xlsx, pptx; never html, svg, archives or executables) or a size over the limit (10 MiB by default) is 422, and an empty file is 422. The stored bytes carry a SHA-256. The subject's record-level read check and private-plan membership apply.",
                upload, &[("200", "pid, filename, content_type, size_bytes, sha256"), ("404", "Unknown subject, or one the caller cannot see"), ("413", "Over the transport ceiling"), ("422", "Disallowed type, empty, over the limit, or unknown subject_kind")]),
            "get": op("attachments", "A subject's attachments",
                "Name, type, size, SHA-256, uploader and time for each live attachment.", subject,
                &[("200", "Attachments"), ("404", "Unknown subject, or one the caller cannot see")])
        },
        "/api/attachments/{pid}/download": {
            "get": op("attachments", "Download an attachment",
                "Re-authorised against the SUBJECT's policy, not just the attachment's own row, so losing access to the plan loses the file. The bytes are re-verified against the stored SHA-256 and never served if they differ. Served with X-Content-Type-Options: nosniff, Cache-Control: private, no-store and Content-Disposition: attachment carrying both an ASCII fallback and the RFC 5987 UTF-8 name, so a non-Latin filename survives and is never doubled.",
                vec![uuid_param("pid")], &[("200", "The file"), ("404", "Unknown attachment, or its subject is not visible"), ("500", "The stored file failed its integrity check")])
        },
        "/api/attachments/{pid}": {
            "delete": op("attachments", "Remove an attachment (soft-delete)",
                "The uploader or an admin may. The bytes remain until an artifact sweep.", vec![uuid_param("pid")],
                &[("200", "Removed"), ("403", "Not the uploader or an admin"), ("404", "Unknown attachment")])
        },
        "/api/attachments/{pid}/delete": {
            "post": op("attachments", "Remove an attachment (POST twin)",
                "The POST twin of DELETE: under the default policy DELETE needs the admin attribute. Same behaviour.", vec![uuid_param("pid")],
                &[("200", "Removed"), ("403", "Not the uploader or an admin"), ("404", "Unknown attachment")])
        }
    })
}

/// Bulk import / export of plans (T-8).
fn bulk_paths() -> Value {
    let job = vec![uuid_param("id")];
    let formats = "Formats: jsonl (one plan object per line, the lossless reference) and csv (RFC 4180 with every repeated or nested field as compact JSON in its cell, so it round-trips exactly as jsonl does).";
    json!({
        "/api/plans/import": {
            "post": op("bulk", "Queue an import of the request body (202 with a job id)",
                &format!("{formats} Send the file as the request body; format=jsonl|csv and on_keyless=review|create are query parameters, and an Idempotency-Key header makes a retried request return the original job (200, replayed true). Each row goes through the same validation, cycle check, events and audit as POST /api/plans. A row is matched to an existing plan by its stable key, in this order: pid; a globally-unique external identifier (Jira key, Asana gid, Trello id, MS Project id, GitHub project id, Linear id, URI, UUID); owner_org_id plus code (a bare code is not a key). A keyed row is created, updated, or reported unchanged (re-importing the same file changes nothing). A pid that names no plan is an error row, never a silent create. A keyless row is checked for duplicates and, if it looks like one, held in the review queue (provenance import) rather than created or merged (on_keyless=create skips the check). One bad row never aborts the load, and the counts always reconcile: created + updated + unchanged + queued_for_review + failed equals total. Import is a destructive action at the guard; per-row record-level stage policy is not applied because the background worker has no claims."),
                vec![query("format", false, "jsonl (default) or csv"), query("on_keyless", false, "review (default) or create")],
                &[("202", "{ job_id, status }"), ("200", "A replay of an earlier Idempotency-Key"), ("422", "Empty body, over the size limit, or an unknown format")])
        },
        "/api/plans/import/{id}": {
            "get": op("bulk", "An import job's status, counts and links",
                "status is queued, running, completed, completed_with_errors or failed (failed means the job itself could not run, with failure_reason). counts carries created, updated, unchanged, queued_for_review and failed, and reconciles states whether they sum to total. errors_url appears when rows failed, review_url when rows await review. Visible to the requester and to an admin; anyone else gets 404.",
                job.clone(), &[("200", "The job"), ("404", "Unknown job, or not yours")])
        },
        "/api/plans/export": {
            "post": op("bulk", "Queue an export of the plans the caller may read (202 with a job id)",
                &format!("{formats} Authorisation is evaluated now, with the caller's identity: the job carries only the plans the caller may read (record-level policy and private plans applied), and rows_omitted reports how many were left out. masking_profile masked (default) hides people references (the lead is dropped, owner organisation fields are partly hidden); full carries them and, like include_soft_deleted, needs the destructive right (403 otherwise). Optional filters: kind and name_like. Rows are ordered by id, so two exports of unchanged data are byte-identical. EVERY export is audited, including one that matches zero rows."),
                vec![], &[("202", "{ job_id, status }"), ("403", "full masking or soft-deleted rows without the destructive right"), ("422", "Unknown format or masking_profile, or more than 100000 plans")])
        },
        "/api/plans/export/{id}": {
            "get": op("bulk", "An export job's status and download link",
                "download_url appears once the job has completed; expires_at is when the file will be swept (24 hours by default).",
                job.clone(), &[("200", "The job"), ("404", "Unknown job, or not yours")])
        },
        "/api/plans/export/{id}/download": {
            "get": op("bulk", "Download the export file",
                "Served with nosniff, private no-store caching and an attachment disposition; the bytes are re-verified against the stored SHA-256. 404 after the file has expired.",
                job.clone(), &[("200", "The file"), ("404", "Unknown job, not yours, or expired")])
        },
        "/api/plans/bulk-jobs": {
            "get": op("bulk", "The caller's bulk jobs, newest first",
                "Filter by kind and status. An admin sees every job; anyone else only their own. Capped at 200.",
                vec![query("kind", false, "import or export"), query("status", false, "queued, running, completed, completed_with_errors or failed")],
                &[("200", "Jobs")])
        },
        "/api/plans/bulk-jobs/{id}/errors": {
            "get": op("bulk", "The per-row error report of a job",
                "CSV by default (format=jsonl for JSONL) with the columns row_number, source_line, field, code, message. row_number is the 1-based data-row ordinal; source_line is the physical line, which differs for multi-line CSV cells. The report names the offending field and never echoes the source value, so it can be shared without leaking personal data.",
                {let mut p = job.clone(); p.push(query("format", false, "csv (default) or jsonl")); p},
                &[("200", "The report"), ("404", "Unknown job, not yours, or no rows failed")])
        },
        "/api/plans/bulk-jobs/sweep": {
            "post": op("bulk", "Delete expired artifact files and re-queue abandoned jobs",
                "Admin only. Deletes the files of artifacts past their expiry (the rows remain as tombstones) and re-queues running jobs untouched for ten minutes; processing a job is idempotent, so a re-run is safe.",
                vec![], &[("200", "artifacts_deleted, jobs_requeued"), ("403", "Not an admin")])
        },
        "/api/plans/review-queue": {
            "get": op("bulk", "Imported rows held for a human decision",
                "Keyless imported rows that the matcher scores as probable duplicates are held here with their candidate matches, never created or merged automatically. Filter by status (pending by default) and by job.",
                vec![query("status", false, "pending (default), confirmed or rejected"), query("job", false, "A bulk job id")],
                &[("200", "Items")])
        },
        "/api/plans/review-queue/{pid}/confirm": {
            "post": op("bulk", "It is a new plan after all: create it",
                "Creates the held candidate through the normal create path. A decided item is 409. Merging into a match is not offered here: reject the item and use the merge endpoint.",
                vec![uuid_param("pid")], &[("200", "The item, now confirmed"), ("404", "Unknown item"), ("409", "Already decided"), ("422", "The candidate is no longer valid")])
        },
        "/api/plans/review-queue/{pid}/reject": {
            "post": op("bulk", "It is a duplicate: discard the candidate",
                "Nothing was created for it, so nothing is undone. A decided item is 409.",
                vec![uuid_param("pid")], &[("200", "The item, now rejected"), ("404", "Unknown item"), ("409", "Already decided")])
        }
    })
}

/// Guest access (PL-7).
fn guest_paths() -> Value {
    let pid = vec![uuid_param("pid")];
    let mut with_grant = pid.clone();
    with_grant.push(uuid_param("grant"));
    let confined = "A guest is a caller whose token carries role=guest. A guard layer confines them: they may read only a plan they hold a live grant on - its MASKED record (no people references), its milestones and its components - read and (with comment access) add comments on that plan, and call their own /api/me endpoints. Everything else, including listing plans, search, reports, tasks, risks and uploads, is 404, so a guest cannot enumerate. Grants expire and are revocable; both take effect on the guest's next request.";
    json!({
        "/api/plans/{pid}/guests": {
            "post": op("guests", "Grant an outside collaborator access to this plan",
                &format!("{confined} access is read or comment (comment implies read); expires_at, when given, must be in the future. Audited. The caller needs write access to the plan."),
                pid.clone(), &[("200", "The grant"), ("401", "Missing or invalid bearer token"), ("404", "Unknown plan, or one the caller cannot see"), ("422", "Unknown access, or an expiry in the past")]),
            "get": op("guests", "A plan's guest grants, live and not",
                "Each grant shows its guest, access level, expiry, who granted it, when it was revoked, and whether it is live now.",
                pid, &[("200", "Grants"), ("404", "Unknown plan, or one the caller cannot see")])
        },
        "/api/plans/{pid}/guests/{grant}/revoke": {
            "post": op("guests", "Revoke a guest grant",
                "Effective on the guest's very next request (the guard reads the grants every time). Idempotent and audited.",
                with_grant, &[("200", "Revoked"), ("401", "Missing or invalid bearer token"), ("404", "Unknown plan or grant")])
        }
    })
}

/// Typed links on a plan (LT-8).
fn link_paths() -> Value {
    let pid = vec![uuid_param("pid")];
    let mut with_link = pid.clone();
    with_link.push(uuid_param("lpid"));
    json!({
        "/api/plans/{pid}/links": {
            "post": op("links", "Link a plan to where its real documents live",
                "Kinds: docs, repo, ticket, design, chat, other. The service stores the link and never fetches or renders it. Only absolute http and https URLs with a host are accepted - never javascript:, data:, file: or a URL with embedded credentials. A URL can be linked once per plan, and a plan holds at most 100 links.",
                pid.clone(), &[("200", "The link"), ("404", "Unknown plan"), ("409", "Already linked"), ("422", "Unknown kind, an unsafe or over-long URL, a blank label, or too many links")]),
            "get": op("links", "A plan's links",
                "Plain list of the plan's live links.", pid, &[("200", "Links"), ("404", "Unknown plan")])
        },
        "/api/plans/{pid}/links/{lpid}": {
            "delete": op("links", "Remove a link (soft-delete)", "Audited.", with_link.clone(),
                &[("200", "Removed"), ("404", "Unknown plan or link")])
        },
        "/api/plans/{pid}/links/{lpid}/delete": {
            "post": op("links", "Remove a link (POST twin)",
                "The POST twin of DELETE: under the default policy DELETE needs the admin attribute. Same behaviour.", with_link,
                &[("200", "Removed"), ("404", "Unknown plan or link")])
        }
    })
}

/// Printable output (PL-10).
fn pdf_paths() -> Value {
    let as_of = query(
        "as_of",
        false,
        "An RFC 3339 instant to stamp the document with; without it, now",
    );
    let rules = "A PDF is never more permissive than its JSON, is deterministic when stamped (pass as_of and two renders of unchanged data are byte-identical), and is NOT signed: signing needs a named signer and a key policy, which is an open decision. Text outside WinAnsi (Latin-1) is written as a question mark rather than guessed.";
    json!({
        "/api/board/pack.pdf": {
            "get": op("pdf", "The board pack as a PDF document",
                &format!("Built from the same function as GET /api/board/pack, behind the same oversight authorisation, so a caller denied the JSON is denied the PDF. The pack carries no people references. {rules}"),
                vec![query("from", false, "Window start"), query("to", false, "Window end"), as_of.clone()],
                &[("200", "application/pdf"), ("403", "Not authorised for oversight reads"), ("422", "Malformed as_of")])
        },
        "/api/plans/{pid}/summary.pdf": {
            "get": op("pdf", "A one-plan summary as a PDF document",
                &format!("Read-checked like the plan itself (record-level policy, private plans). Rendered from the MASKED record and counts only: no lead, owner organisation, assignee or risk owner ever reaches the text layer. Shows kind, status, phase, stage, dates, keywords, goals, task counts by status, the number of risks recorded and the milestones. {rules}"),
                vec![uuid_param("pid"), as_of],
                &[("200", "application/pdf"), ("404", "Unknown plan, or one the caller cannot see"), ("422", "Malformed as_of")])
        }
    })
}

/// Notification delivery sinks (LT-6, RM-11).
fn sink_paths() -> Value {
    let pid = vec![uuid_param("pid")];
    let rules = "Kinds: webhook (the family envelope, signed with X-Mxi-Signature over the exact body), slack, mattermost, discord, telegram (needs settings.chat_id) and email. Delivery is never in the notification's path: a notification only enqueues a delivery, and a dispatcher sends it later with bounded retries (1, 5, 25, 125 minutes, then given up), so a slow or dead destination cannot delay or fail the notification. The person's channel preferences (webhook, chat, email) are honoured per notification kind. A chat or email message carries only the notification's own text, never more than the in-app inbox shows.";
    json!({
        "/api/me/notification-sinks": {
            "post": op("sinks", "Register a delivery destination for your own notifications",
                &format!("{rules} The destination must be an https URL to a PUBLIC host: never credentials in the URL, plain http, localhost, a private, link-local or metadata address, or an .internal or .local name (the name is resolved again at send time and refused if any answer is private; redirects are not followed). It is a secret and only a masked form is ever returned. At most ten per person; an identity that is not a user id is refused."),
                vec![], &[("200", "The sink, destination masked"), ("401", "Missing or invalid bearer token"), ("422", "Unknown kind, an unacceptable destination, a telegram sink without chat_id, too many sinks, or an identity that cannot be addressed")]),
            "get": op("sinks", "Your own sinks, destinations masked",
                "Each sink with its kind, channel, enabled flag and a masked destination (scheme and host plus the last four characters, or the email domain). Never another person's, and never a full secret.",
                vec![], &[("200", "Sinks"), ("401", "Missing or invalid bearer token")])
        },
        "/api/me/notification-sinks/{pid}": {
            "delete": op("sinks", "Remove one of your own sinks",
                "Another person's sink is 404, identically. Pending deliveries to it are skipped.", pid.clone(),
                &[("200", "Removed"), ("401", "Missing or invalid bearer token"), ("404", "Unknown, or not yours")])
        },
        "/api/me/notification-sinks/{pid}/delete": {
            "post": op("sinks", "Remove one of your own sinks (POST twin)",
                "The POST twin of DELETE: under the default policy DELETE needs the admin attribute. Same behaviour.", pid.clone(),
                &[("200", "Removed"), ("401", "Missing or invalid bearer token"), ("404", "Unknown, or not yours")])
        },
        "/api/me/notification-sinks/{pid}/deliveries": {
            "get": op("sinks", "Recent deliveries to one of your own sinks",
                "Status (pending, sent, failed, skipped), the number of attempts, the last error, and when the next attempt is due, for the newest fifty.", pid,
                &[("200", "Deliveries"), ("401", "Missing or invalid bearer token"), ("404", "Unknown, or not yours")])
        },
        "/api/notification-sinks/dispatch": {
            "post": op("sinks", "Send the deliveries that are due now",
                "Admin only. A ticker does this on its own every few seconds (PROJECT_PORTFOLIO_MANAGEMENT_SINKS_TICK_SECS, 0 turns it off); this is for operators and tests. Returns how many were sent, are retrying, failed or were skipped.",
                vec![], &[("200", "sent, retrying, failed, skipped"), ("403", "Not an admin")])
        }
    })
}

/// Attachment text search (PL-8).
fn attachment_search_paths() -> Value {
    json!({
        "/api/attachments/search": {
            "get": op("attachments", "Find attachments by their text",
                "Full-text search over text extracted at upload: plain text, CSV, Markdown and JSON as UTF-8, and .docx, .xlsx and .pptx from their XML. PDFs and images are NOT indexed (no extractor, no OCR) and are recorded unindexed with the reason; unindexed_attachments says how many, so no result is not mistaken for not there. Access follows the subject: a hit the caller may not read is dropped entirely (not a title, not a count). Soft-deleted attachments never match. Up to 50 hits with a snippet.",
                vec![query("q", true, "Search text, 2 to 200 characters")],
                &[("200", "results and unindexed_attachments"), ("401", "Missing or invalid bearer token"), ("422", "q is too short or too long")])
        }
    })
}

/// Scheduled report runs (T-28e).
fn report_schedule_paths() -> Value {
    let pid = vec![uuid_param("pid")];
    json!({
        "/api/reports/{pid}/schedule": {
            "post": op("reports", "Run a saved report on a cadence",
                "cadence is daily, weekly or monthly (monthly keeps the day, clamped in short months); format is csv (default) or json; start is the first UTC date (default today). Each due run goes through the same code as GET /api/reports/{pid}/run, is stored as an artifact that expires (30 days by default) and is audited like an export, even at zero rows. A missed run runs once and then moves to the next future date; a failure is recorded on the schedule and the date still advances. At most 20 per person.", pid.clone(),
                &[("200", "The schedule"), ("401", "Missing or invalid bearer token"), ("404", "Unknown report"), ("422", "Bad cadence, format or start, or too many schedules")])
        },
        "/api/report-schedules": {
            "get": op("reports", "Your scheduled report runs",
                "Each with its next run date, last status, last row count (0 is a real zero) and whether an output exists. Only your own.",
                vec![], &[("200", "Schedules"), ("401", "Missing or invalid bearer token")])
        },
        "/api/report-schedules/sweep": {
            "post": op("reports", "Run the scheduled reports that are due now",
                "Admin only. The scheduler ticker does this too; this is for an external cron and tests. Claim-based, so running both cannot run a schedule twice.",
                vec![], &[("200", "ran, failed, already_claimed"), ("403", "Not an admin")])
        },
        "/api/report-schedules/{pid}": {
            "delete": op("reports", "Stop a scheduled report", "Another person's schedule is 404, identically.", pid.clone(),
                &[("200", "Stopped"), ("401", "Missing or invalid bearer token"), ("404", "Unknown, or not yours")])
        },
        "/api/report-schedules/{pid}/delete": {
            "post": op("reports", "Stop a scheduled report (POST twin)", "The POST twin of DELETE (the default policy reserves DELETE for admins).", pid.clone(),
                &[("200", "Stopped"), ("401", "Missing or invalid bearer token"), ("404", "Unknown, or not yours")])
        },
        "/api/report-schedules/{pid}/latest": {
            "get": op("reports", "Download the latest scheduled output",
                "Owner only. 404 before the first run and after the artifact has expired. Served as an attachment with nosniff and private no-store.", pid,
                &[("200", "The report file"), ("401", "Missing or invalid bearer token"), ("404", "No output, expired, unknown, or not yours")])
        }
    })
}
