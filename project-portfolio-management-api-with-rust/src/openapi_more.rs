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
    merge(&mut all, cpm_paths());
    merge(&mut all, calendar_paths());
    merge(&mut all, mspdi_paths());
    merge(&mut all, availability_paths());
    merge(&mut all, booking_paths());
    merge(&mut all, baseline_paths());
    merge(&mut all, raci_paths());
    merge(&mut all, earned_value_paths());
    merge(&mut all, evm_paths());
    merge(&mut all, leveling_paths());
    merge(&mut all, portfolio_resource_paths());
    merge(&mut all, schedule_risk_paths());
    merge(&mut all, schedule_health_paths());
    merge(&mut all, kpi_okr_paths());
    merge(&mut all, milestone_paths());
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
                &r("Settable: assignee_ref, due_date, start_date, component_pid (null clears). status is deliberately not bulk-editable: a status change must pass the workflow, WIP, role and blocker rules that belong to the board move. A component must belong to each task's own plan, and a due date that would break a precedes relation refuses that task. Assigning notifies and auto-watches like a single assignment."),
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

/// The critical-path engine (SC-1, PJ-1, PJ-8).
fn cpm_paths() -> Value {
    json!({
        "/api/plans/{pid}/cpm": {
            "get": op("cpm", "Float, critical tasks and critical chains for a plan's tasks",
                "Forward and backward pass over the plan's task network: early and late start and finish, total float (how long before the project finish moves), free float (how long before a successor is delayed), critical (float 0) and near-critical (?near= working days, default 5), and every critical chain along driving edges. All four relationship types (fs, ss, ff, sf) with a lag or lead. Edges are the dependencies between the plan's tasks plus task precedes relations. A task's planned start is a lower bound: pushed_days says by how many working days its predecessors force it later than planned. A task with no duration, or with no start and no scheduled predecessor, is listed unscheduled with the reason, never given a float of zero. A dependency cycle is a 422. Working days use the plan's calendar (all seven days until SC-2).",
                vec![uuid_param("pid"), query("near", false, "Working days of float counted as near-critical, 0 to 3650")],
                &[("200", "nodes, edges, unscheduled, ignored_edges, critical_chains"), ("404", "Unknown plan"), ("422", "Bad near, or a dependency cycle")])
        },
        "/api/plans/{pid}/tasks/{t_pid}/schedule": {
            "put": op("cpm", "Set a task's planned start and working duration",
                "start_date is a calendar date and duration_days a number of working days (0 is a milestone); an absent or null field clears it. constraint_kind (snet, snlt, fnet, fnlt, mso, mfo) goes with constraint_date; deadline is a soft date that bounds the LATE dates only, so a threatened deadline gives negative float and never moves an early date. A HARD constraint (mso, mfo) overrides the schedule's logic: it needs a reason and, when authentication is on, access=admin or scheduling=hard_constraints (403 otherwise). alap is not supported yet. The answer carries what moved across the whole network (start, finish or both, in working days), not only the edited task: an empty list when nothing moved. Audited with the old and new values and the reason.",
                vec![uuid_param("pid"), uuid_param("t_pid")],
                &[("200", "task and moved"), ("403", "A hard constraint without the permission"), ("404", "Unknown plan or task"), ("422", "Out-of-range values, an unknown constraint kind, a kind without a date, or a hard constraint without a reason")])
        }
    })
}

/// Working calendars (SC-2).
fn calendar_paths() -> Value {
    let body = "weekdays is seven 0/1 characters, Monday first, with at least one working day (1111100 is Monday to Friday); holidays is a list of dates (at most 400). A plan with no calendar works all seven days, so nothing stored before calendars existed changes.";
    json!({
        "/api/plans/{pid}/calendar": {
            "get": op("cpm", "The plan's working calendar", "All seven days when none was set (source default).", vec![uuid_param("pid")],
                &[("200", "weekdays, holidays and source"), ("404", "Unknown plan")]),
            "put": op("cpm", "Set the plan's working calendar", body, vec![uuid_param("pid")],
                &[("200", "The calendar"), ("404", "Unknown plan"), ("422", "A malformed mask, no working day, duplicate or out-of-range holidays")])
        },
        "/api/me/calendar": {
            "get": op("cpm", "Your own working calendar", "Your weekly pattern and holidays. Recorded absences (POST /api/non-working) count as your holidays automatically. A task is scheduled on its plan's calendar and yours narrows it: your work happens only on days both work.", vec![],
                &[("200", "weekdays, holidays and source"), ("401", "Missing or invalid bearer token"), ("422", "This identity has no person reference")]),
            "put": op("cpm", "Set your own working calendar", body, vec![],
                &[("200", "The calendar"), ("401", "Missing or invalid bearer token"), ("422", "A malformed mask, or no person reference")])
        }
    })
}

/// MS Project (MSPDI XML) import (PJ-10, T-28n).
fn mspdi_paths() -> Value {
    json!({
        "/api/plans/{pid}/import/mspdi": {
            "post": op("cpm", "Import an MS Project XML (MSPDI) file into this plan",
                "The body is the XML itself (up to 5 MiB, 5000 tasks). Binary .mpp is NOT read: export as XML from MS Project. dry_run defaults to true and returns the report without writing; dry_run=false applies it. Tasks, outline parents, dates, working-day durations, constraints, deadlines, dependencies (FF 0, FS 1, SF 2, SS 3, with lag or lead) and the base calendar come across; summary tasks come without schedule fields. Idempotent: a task's MS Project UID is kept as external_ref, so a re-import updates in place and never duplicates, and never changes status. A hard constraint (mso, mfo) needs access=admin or scheduling=hard_constraints, otherwise the task is imported without it. Resources are NOT matched to people (names are kept in the task description), and baselines, resource cost rates, as-late-as-possible, links to other files and inactive tasks are not imported: every one is listed in `unmapped` with its reason. DTDs are refused. The file is parsed with a node cap.",
                vec![uuid_param("pid"), query("dry_run", false, "true (default) reports only")],
                &[("200", "Counts and the unmapped list"), ("404", "Unknown plan"), ("422", "Not MSPDI, not UTF-8 XML, a DTD, too large or too many tasks")])
        }
    })
}

/// Availability over time (PJ-3).
fn availability_paths() -> Value {
    let who = "A person may read and set their own ranges; a resource manager (role=resource_manager) or an admin may for anyone; another person's are 404, not 403. With authentication off there is no caller to refuse.";
    json!({
        "/api/availability": {
            "post": op("availability", "Set a person's capacity for a date range",
                &format!("max_percent is the most of their time they can commit between from and to, 0 to 100 (60 is a part-time spell, 0 is leave). Where no range applies capacity is 100, so nothing set before this existed changes. person_ref defaults to the caller's own. Ranges for one person may not overlap (409). GET /api/capacity reads capacity per day, never averaged. {who}"),
                vec![], &[("200", "The range"), ("404", "Not yours to set"), ("409", "Overlaps an existing range"), ("422", "A bad person_ref, to before from, or max_percent outside 0 to 100")]),
            "get": op("availability", "A person's availability ranges",
                &format!("person_ref defaults to the caller's own. {who}"),
                vec![query("person_ref", false, "Whose ranges (a person: or worker: URN)")],
                &[("200", "Ranges, earliest first"), ("404", "Not yours to read"), ("422", "No person_ref and no caller identity")])
        },
        "/api/availability/{pid}": {
            "delete": op("availability", "Remove an availability range", who, vec![uuid_param("pid")],
                &[("200", "Removed"), ("404", "Unknown, or not yours")])
        },
        "/api/availability/{pid}/delete": {
            "post": op("availability", "Remove an availability range (POST twin)", "The POST twin of DELETE (the default policy reserves DELETE for admins).", vec![uuid_param("pid")],
                &[("200", "Removed"), ("404", "Unknown, or not yours")])
        }
    })
}

/// Proposed vs committed bookings (PJ-4).
fn booking_paths() -> Value {
    json!({
        "/api/plans/{pid}/allocations/{a_pid}/commit": {
            "post": op("availability", "Commit a proposed booking",
                "POST /api/plans/{pid}/allocations takes booking (proposed or committed, default committed) and requested. A proposed booking is hoped for: it never counts toward committed utilisation or over-allocation, and GET /api/capacity reports it as its own series (proposed.percent, proposed.allocations, proposed.would_exceed_capacity), never merged into the firm figures. Committing it is allowed to the named person, a resource manager or an admin (others 404). If, with it firm, the person would be over their per-day capacity (availability, PJ-3) the answer is 409 over_capacity with the numbers, and only ?confirm=true commits it anyway (audited as an over-commitment). Committing an already committed booking is 409.",
                vec![uuid_param("pid"), uuid_param("a_pid"), query("confirm", false, "true to commit despite being over capacity")],
                &[("200", "pid, booking and whether it is over capacity"), ("404", "Unknown, or not yours to commit"), ("409", "Already committed, or over capacity without ?confirm=true")])
        }
    })
}

/// Schedule baselines (SC-3).
fn baseline_paths() -> Value {
    json!({
        "/api/plans/{pid}/schedule-baselines": {
            "post": op("cpm", "Take a schedule baseline",
                "Snapshots every task's early start, early finish and working duration (the critical-path engine's dates) as the next numbered version, with a required reason (up to 500 characters). Append-only: there is no update and no delete, a re-baseline is the next version and the earlier one stays. A bounded number are kept per plan (11 by default); at the limit a new one is refused 409 rather than overwriting history. A task that cannot be scheduled is recorded with no dates and the reason. Refused 422 when nothing is scheduled yet or the dependencies contain a cycle.",
                vec![uuid_param("pid")],
                &[("200", "The baseline"), ("404", "Unknown plan"), ("409", "At the per-plan limit, or another baseline was taken at the same moment"), ("422", "No reason, nothing scheduled, or a cycle")]),
            "get": op("cpm", "A plan's schedule baselines, newest first", "Version, reason, who, when, task count and the baselined project finish.",
                vec![uuid_param("pid")], &[("200", "Baselines"), ("404", "Unknown plan")])
        },
        "/api/plans/{pid}/schedule-variance": {
            "get": op("cpm", "How the schedule differs from a baseline",
                "Per task: start, finish and duration variance in working days of the plan's calendar (positive is later), measured against ?baseline=<version> (the latest by default). Standing is slipped, pulled_in, on_baseline, added, removed or unmeasurable. A task added since the baseline is `added`, not slip; one removed since stays listed with its baselined dates; a task unscheduled on either side has null variance with the reason, never zero. The summary counts reconcile (matched + removed = baseline tasks; matched + added = current tasks) and states the plan's own finish slip. With no baseline at all the answer is baseline null, variance null, reason no_baseline.",
                vec![uuid_param("pid"), query("baseline", false, "A baseline version (default: the latest)")],
                &[("200", "baseline, variance and note"), ("404", "Unknown plan, or no such baseline version")])
        }
    })
}

/// The RACI matrix.
fn raci_paths() -> Value {
    json!({
        "/api/plans/{pid}/raci": {
            "get": op("raci", "The plan's RACI matrix",
                "People across, tasks down, letters in the cells (R Responsible, A Accountable, C Consulted, I Informed; a person may hold several, e.g. AR). Each row lists what is missing (no_accountable, no_responsible, many_accountable); the summary counts reconcile with the rows (a task with none of that is complete); workload counts each person's R, A, C and I. A task with no assignments is still a row. ?format=csv returns a spreadsheet with a gaps column.",
                vec![uuid_param("pid"), query("format", false, "csv for a spreadsheet")],
                &[("200", "people, rows, summary, workload, legend (or CSV)"), ("404", "Unknown plan")])
        },
        "/api/plans/{pid}/tasks/{t_pid}/raci": {
            "put": op("raci", "Replace a task's RACI row",
                "Send {assignments: [{person_ref, roles: \"AR\"}]}; that becomes the row, atomically, and the old assignments are soft-deleted so history is kept. REFUSED 422: more than one Accountable person (the database enforces it too), a letter that is not R, A, C or I, a person twice, a person with no letters, a person_ref that is not a person:/worker: URN, more than 50 people. A row with no Accountable or no Responsible is allowed and reported in findings, because that is how a plan looks while it is being set up. People who are R, A or I are auto-subscribed to the task unless they turned auto_watch off; removing them from the row does not unsubscribe them.",
                vec![uuid_param("pid"), uuid_param("t_pid")],
                &[("200", "task_pid, cells and findings"), ("404", "Unknown plan or task"), ("422", "The row is refused (see above)")])
        }
    })
}

/// Earned value (SC-4).
fn earned_value_paths() -> Value {
    json!({
        "/api/plans/{pid}/earned-value": {
            "get": op("earned-value", "Earned value: PV, EV, AC, SV, CV, SPI, CPI and SPI(t), one block per currency",
                "Planned value follows the phased budget baseline (the latest version in each currency; linear within a period, the standard EVM convention). Earned value is the budget times the plan's PHYSICAL percent complete: each task is counted 0/100, 0/50/100, or by a person-entered percent (always flagged, always with a note), weighted equally or by story points (the plan default, PUT …/earned-value-settings). Actual cost is the plan's recorded budget-line actuals in that currency; timesheets x a rate is not offered until cost rates exist. Ratios are basis points (10000 = 1.0), money minor units, all integer. SPI(t) is earned schedule over actual time and, unlike SPI, still says so when a project finishes late. Undefined figures are null with a reason (no_planned_value_yet, no_actuals_recorded, no_planned_value), never zero or 1.0; with no baseline the answer is blocks null, absent no_baseline; with nothing to weigh, no_weighted_tasks. Measured as of today (UTC date): actuals carry no date, so the past is not reconstructed. Currencies never merge: budget lines in a currency with no baseline are listed in unbaselined_currencies.",
                vec![uuid_param("pid")],
                &[("200", "percent_complete and one block per currency (or an absent reason)"), ("404", "Unknown plan")])
        },
        "/api/plans/{pid}/earned-value-settings": {
            "put": op("earned-value", "How the plan's tasks are counted by default",
                "default_method is zero_hundred (finished tasks only, equal weights), fifty_fifty (started work counts half) or points (finished tasks, weighted by story points: the physical percent of points delivered; a task with no points carries no weight and is listed). A plan with no setting counts zero_hundred. Audited.",
                vec![uuid_param("pid")], &[("200", "The setting"), ("404", "Unknown plan"), ("422", "Unknown method")])
        },
        "/api/plans/{pid}/tasks/{t_pid}/percent-complete": {
            "put": op("earned-value", "Count one task another way",
                "method zero_hundred, fifty_fifty or manual overrides the plan default for this task; an absent or null method clears the override. manual needs percent (0 to 100) and a note saying why, is flagged wherever earned value is shown, and is audited with the old and new values.",
                vec![uuid_param("pid"), uuid_param("t_pid")],
                &[("200", "The task's method"), ("404", "Unknown plan or task"), ("422", "Unknown method, a manual percent without percent or note, or percent/note with another method")])
        }
    })
}

/// EVM analytics (SC-5).
fn evm_paths() -> Value {
    json!({
        "/api/plans/{pid}/earned-value/forecast": {
            "get": op("earned-value", "Where the plan is heading: EAC variants, ETC, VAC, TCPI and traffic lights",
                "Per currency: every EAC variant named by its formula (budget_rate: AC + (BAC - EV); cpi: AC + (BAC - EV) / CPI; cpi_spi: AC + (BAC - EV) / (CPI x SPI); bottom_up: AC + the plan's own estimate to complete from the budget forecast), each with ETC (EAC - AC) and VAC (BAC - EAC, negative is an overrun). None is presented as the answer: `recommended` is a stated rule (cost efficiency continues when the schedule is on track; both pressures when it is behind the amber line; the one-off reading when there is no CPI) and the others are returned beside it. TCPI to the budget and to the recommended forecast: above 1.00 the remaining work must be done more efficiently than the work so far; a target already spent is budget_exhausted, never infinity. A missing denominator is null with undefined_denominator, never zero. Traffic lights use thresholds from PROJECT_PORTFOLIO_MANAGEMENT_EV_THRESHOLDS (spi_cpi_amber, spi_cpi_red, tcpi_amber, tcpi_red, in basis points), shown in the response with their source. Currencies are never added together.",
                vec![uuid_param("pid")], &[("200", "forecasts, thresholds and a note (or an absent reason)"), ("404", "Unknown plan")])
        },
        "/api/plans/{pid}/earned-value/series": {
            "get": op("earned-value", "The S-curve data (planned value, earned value, actual cost over time)",
                "The planned-value curve is the whole baseline, per currency. Earned value and actual cost appear ONLY on the days a snapshot was recorded (daily by the scheduler, or POST …/earned-value/snapshot): they carry no date at source, so the past cannot be reconstructed and is never filled in between (null on every other date). ?format=csv gives currency,date,pv_minor,ev_minor,ac_minor.",
                vec![uuid_param("pid"), query("format", false, "csv for a spreadsheet")],
                &[("200", "curves per currency (or CSV)"), ("404", "Unknown plan")])
        },
        "/api/plans/{pid}/earned-value/snapshot": {
            "post": op("earned-value", "Record today's earned-value figures now",
                "One row per plan, currency and day; a second call the same day replaces the first. The scheduler does this daily for every plan with a baseline; this is for an immediate record. Returns how many currencies were recorded (0 when there is nothing to measure).",
                vec![uuid_param("pid")], &[("200", "as_of and snapshotted"), ("404", "Unknown plan")])
        },
        "/api/plans/{pid}/earned-value/rollup": {
            "get": op("earned-value", "Earned value for a plan and everything beneath it, per currency",
                "Sums BAC, PV, EV and AC over the plan and its descendants (to ten levels, 500 plans), per currency and never across currencies; SPI and CPI are computed from the totals. A partial cost (some plans with no recorded cost) gives no CPI rather than a flattering one. EAC and VAC appear only when every contributing plan has its own recommended EAC. Plans the caller cannot read (private, or denied by policy) are left out without a trace; plans with no figures are counted in plans_without_figures, not guessed at.",
                vec![uuid_param("pid")], &[("200", "currencies, plans_without_figures, thresholds"), ("404", "Unknown plan")])
        }
    })
}

/// Resource leveling (SC-6).
fn leveling_paths() -> Value {
    json!({
        "/api/plans/{pid}/leveling": {
            "get": op("scheduling", "Propose a resource leveling (advice only; changes nothing)",
                "Finds the days a person's unfinished, assigned, scheduled tasks ask for more than their capacity (availability, 100% when none; plus their committed bookings in other plans) and proposes start delays. The first over-allocated day is resolved by keeping tasks in priority order (cannot-move first, then least total float, then earliest start, then input order) and delaying the lowest-priority task left over; a delay is accepted only if the network re-run shows no negative float, no hard-constraint conflict, nothing newly unscheduled and, unless allow_critical=true, an unchanged project finish. A task already started or with a hard constraint is never moved; a critical task is never moved without allow_critical, and then it is marked and the finish impact shown. What cannot be fixed is listed under unresolved with a reason (fixed_tasks, critical_task, no_float, hard_constraint, did_not_move, limit). The same input always gives the same proposal. Each moved task is for a person: a caller who may not manage that person sees person-N labels, no person_ref and no percentages. load_percent (default 100) is what one task asks of its assignee per working day.",
                vec![uuid_param("pid"), query("allow_critical", false, "true to let a critical task move"), query("load_percent", false, "1-100, default 100")],
                &[("200", "moves, effects, over_allocated, unresolved, finish_before/after, finish_delay_days"), ("404", "Unknown plan"), ("422", "A dependency cycle, or a bad load_percent")])
        },
        "/api/plans/{pid}/leveling/apply": {
            "post": op("scheduling", "Apply moves from a leveling proposal (confirmed)",
                "Body: moves [{task_pid, start_date}], confirm (must be true), optional allow_critical / load_percent (as read) and reason. The proposal is recomputed and every move must be part of it at that date, otherwise 409 stale_proposal and nothing is changed. Each start date is set through the bulk task edit (so the same read checks apply) and writes exactly one task_leveled audit entry per moved task, with from, to, delay and finish impact. Returns one result per move.",
                vec![uuid_param("pid")],
                &[("200", "requested, ok, refused, results, finish impact"), ("404", "Unknown plan"), ("409", "A move is not part of the current proposal"), ("422", "confirm missing, no moves, or a bad value")])
        }
    })
}

/// Portfolio resource management (SC-7).
fn portfolio_resource_paths() -> Value {
    json!({
        "/api/resource-plan": {
            "get": op("resources", "Demand against supply by role or skill and period, in FTE",
                "Across every plan the caller can read. Demand is committed allocations (percent x days, summed as percent-days, then turned into FTE = percent-days / (100 x days in the period)); placeholders (role-only demand, no person yet) count toward their role and are shown apart as placeholder_fte; proposed bookings are a separate series and never enter demand or the gap. Supply is the people whose primary role it is (PUT /api/resource-roles), read per day from their availability. gap_fte is supply minus demand (negative is a shortage); headcount is a separate field and never mixed with FTE. Demand with no role, and people with no role, are the `unclassified` row, never dropped. group_by=skill is demand-only (a person's skills are resolved live from the worker service) and its rows overlap. ?scenario=<pid> limits demand to that scenario's plans (what-if). A caller without access=admin or role=resource_manager gets roles and totals only (masked: true, no people_by_role). The per-plan series sums to the per-period totals.",
                vec![query("from", false, "YYYY-MM-DD, default today"), query("to", false, "default from + 89 days"), query("period", false, "week or month (default)"), query("group_by", false, "role (default) or skill"), query("scenario", false, "a scenario pid")],
                &[("200", "periods with rows, by_plan, placeholders, people_by_role"), ("404", "Unknown scenario"), ("422", "A bad window, period or group_by")])
        },
        "/api/resource-plan/hours": {
            "get": op("resources", "Planned against actual hours per plan and person",
                "Planned is a named, committed allocation's percent of an 8-hour day on each Monday to Friday of the window (placeholder demand is reported apart); actual is recorded time entries, whether or not the week is approved. A plan with nothing recorded is null with actual_reason no_actuals, never zero hours. Time recorded by someone with no allocation on that plan in the window is listed under unplanned_actuals (per person for a manager, per plan with a count for anyone else). Plans the caller cannot read are left out.",
                vec![query("from", false, "default to - 27 days"), query("to", false, "default today")],
                &[("200", "plans, unplanned_actuals, totals"), ("422", "A bad window")])
        },
        "/api/resource-roles": {
            "get": op("resources", "Primary roles for capacity planning",
                "A manager (access=admin or role=resource_manager) sees everyone's; anyone else sees only their own.",
                vec![], &[("200", "person_ref and role")]),
            "put": op("resources", "Set or clear a person's primary role",
                "Body: person_ref and role (null clears it). One role per person: it is the role whose supply they count toward. A manager's call (403 otherwise); audited with old and new.",
                vec![], &[("200", "person_ref and role"), ("403", "Not a manager"), ("422", "A bad person_ref or role")])
        },
        "/api/plans/{pid}/allocations/{a_pid}/resolve": {
            "post": op("resources", "Give a placeholder a named person",
                "A placeholder is created with POST /api/plans/{pid}/allocations {placeholder: true, role, percent, ...} (no person_ref). Resolving sets the person; a manager's call. If it would put a committed booking's person over capacity on any day the answer is 409 over_capacity with the numbers, and only confirm=true goes ahead (audited as allocation_resolved with over_capacity).",
                vec![uuid_param("pid"), uuid_param("a_pid")],
                &[("200", "pid, person_ref, over_capacity"), ("403", "Not a manager"), ("404", "Unknown plan or allocation"), ("409", "Already names a person, or over capacity"), ("422", "A bad person_ref")])
        }
    })
}

/// Schedule-risk Monte Carlo (SC-8).
fn schedule_risk_paths() -> Value {
    json!({
        "/api/plans/{pid}/schedule-risk": {
            "get": op("scheduling", "Seeded schedule-risk simulation: P50 / P80 / P90 finish, criticality and sensitivity",
                "Each iteration draws a whole-working-day duration for every task from its optimistic / likely (duration_days) / pessimistic estimate (triangular, or PERT), lets each register risk with a task, an impact and a probability strike with that probability, and re-runs the critical-path analysis. Reports the finish percentiles (nearest rank), the share of iterations finishing by the deterministic CPM finish (cpm_finish_confidence_bp), a criticality index per task (share of iterations with no float), a sensitivity correlation per task, and a finish histogram. Seeded: the same seed gives the identical result. With no uncertainty every percentile equals the CPM finish. Every assumption is echoed, including that inputs are treated as independent (no correlation). A task with a likely duration but no estimate is simulated at that duration +/- fallback_band_percent (default 20) and marked source=fallback, never treated as certain; a risk missing a probability, task or impact is skipped and named under events.skipped. Iterations are capped (20 000, and iterations x tasks 5 000 000).",
                vec![uuid_param("pid"), query("iterations", false, "default 2000, max 20000"), query("seed", false, "default 1"), query("distribution", false, "triangular (default) or pert"), query("fallback_band_percent", false, "0-100, default 20")],
                &[("200", "assumptions, percentiles, histogram, tasks, events, unscheduled"), ("404", "Unknown plan"), ("422", "A cycle, nothing schedulable, or over the caps")])
        },
        "/api/plans/{pid}/tasks/{t_pid}/estimate": {
            "put": op("scheduling", "Set a task's optimistic and pessimistic duration",
                "Working days; the likely duration is the task's duration_days, which must be set first, and optimistic <= likely <= pessimistic. A field left out or null clears it. Audited with old and new.",
                vec![uuid_param("pid"), uuid_param("t_pid")],
                &[("200", "optimistic, likely and pessimistic days"), ("404", "Unknown plan or task"), ("422", "Out of order, out of range, or no likely duration")])
        },
        "/api/plans/{pid}/risks/{r_pid}/schedule-impact": {
            "put": op("scheduling", "Make a register risk a schedule event",
                "Names the task it threatens (a task of this plan), the working days it adds if it happens (1-365) and its probability in percent (0-100). All optional; the simulation skips, and names, a risk that has some of these but not all, or is not open or mitigating. Audited with old and new.",
                vec![uuid_param("pid"), uuid_param("r_pid")],
                &[("200", "the risk's schedule fields"), ("404", "Unknown plan or risk"), ("422", "A bad value or a task of another plan")])
        }
    })
}

/// Schedule health self-check (SC-9).
fn schedule_health_paths() -> Value {
    json!({
        "/api/plans/{pid}/schedule-health": {
            "get": op("scheduling", "Schedule health self-check: fourteen DCMA-style checks",
                "Runs fourteen checks over the plan's task network: missing logic, leads, lags, relationship types (finish-to-start share), hard constraints, high float, negative float, high duration, invalid dates, resources, missed tasks, the critical path test, the critical path length index (CPLI) and the baseline execution index (BEI). Each returns count, of, value, threshold, pass and offenders by pid, and names its denominator (incomplete tasks, relationships into incomplete tasks, or tasks baselined due by the status date). A check that cannot be computed (no baseline, a cycle, nothing to measure) has pass null and a reason and is never a pass; all_pass is true only when all fourteen were computed and passed. A self-check in the style of the DCMA 14-Point Assessment, not a DCMA compliance assessment. The preset (dcma-default) is echoed with its thresholds; PROJECT_PORTFOLIO_MANAGEMENT_SCHEDULE_HEALTH_THRESHOLDS (JSON) overrides any, and the response shows the source.",
                vec![uuid_param("pid"), query("as_of", false, "the status date, YYYY-MM-DD (default today)")],
                &[("200", "checks, counts, thresholds and preset"), ("404", "Unknown plan"), ("422", "A bad as_of")])
        }
    })
}

/// KPIs, and the OKR portfolio view / edit / delete.
fn kpi_okr_paths() -> Value {
    json!({
        "/api/kpis": {
            "post": op("kpis", "Define a KPI",
                "A standing measure: name, unit, direction (higher_better or lower_better), target_value, red_value (the line beyond which it is red, strictly on the worse side of the target), optional cadence (daily, weekly, monthly, quarterly), owner_ref, plan_pid (omit for a portfolio KPI) and objective_pid. Values are integers (scale them: basis points, minor units). Refused 422 when the red line is on the wrong side of the target.",
                vec![], &[("200", "the new KPI's pid"), ("404", "Unknown plan"), ("422", "A bad direction, thresholds, cadence, owner or objective")]),
            "get": op("kpis", "KPIs with their derived status",
                "Each KPI carries status (green at or better than the target, red at or worse than the red line, amber between, no_data when it has no reading — never red or green), the latest reading, trend (direction-aware, from the last two readings) and stale (a cadence KPI with no reading within twice the interval). Ordered red, amber, no_data, green. summary counts every visible KPI whatever the status filter. KPIs on plans the caller cannot read are left out.",
                vec![query("plan", false, "a plan pid"), query("objective", false, "an objective pid"), query("status", false, "green, amber, red or no_data"), query("as_of", false, "date staleness is judged against (default today)")],
                &[("200", "kpis, summary, as_of"), ("422", "A bad status")])
        },
        "/api/kpis/{pid}": {
            "get": op("kpis", "One KPI with its readings", "The definition, derived status, trend, staleness and the full reading history, newest first.",
                vec![uuid_param("pid")], &[("200", "the KPI and its history"), ("404", "Unknown KPI, or on a plan the caller cannot read")]),
            "put": op("kpis", "Edit a KPI",
                "Name, description, unit, target, red line, cadence, owner and objective. The direction and the plan are not editable (changing which way is good would silently re-colour every past reading); sending one is refused. Audited with old and new.",
                vec![uuid_param("pid")], &[("200", "the KPI"), ("404", "Unknown KPI"), ("422", "A bad field, thresholds, or an unknown field")]),
            "delete": op("kpis", "Delete a KPI", "Soft delete; its readings are kept.",
                vec![uuid_param("pid")], &[("200", "deleted"), ("404", "Unknown KPI")])
        },
        "/api/kpis/{pid}/readings": {
            "post": op("kpis", "Record a reading",
                "Append-only. observed_on defaults to today and may not be in the future. The latest by observed day (then entry order) is the one status rests on, so a correction is a newer reading for the same day. Returns the KPI with its new derived status.",
                vec![uuid_param("pid")], &[("200", "the reading and the KPI"), ("404", "Unknown KPI"), ("422", "A future or implausible date")]),
            "get": op("kpis", "Readings of a KPI", "Every reading, newest first; an earlier entry for a day that has a later one is marked superseded.",
                vec![uuid_param("pid")], &[("200", "readings"), ("404", "Unknown KPI")])
        },
        "/api/okrs": {
            "get": op("okrs", "Objectives with key results and derived scores",
                "Every objective (optionally one period) with each key result's derived progress or the reason it has none, its latest check-in with confidence, and the objective's score. Ordered least progressed first; an objective with no measurable key result is unmeasured and sorts last, never as zero. Confidence is shown beside each key result and never blended into a score.",
                vec![query("period", false, "an exact period, e.g. 2026-Q4")], &[("200", "objectives and as_of")])
        },
        "/api/objectives/{pid}": {
            "put": op("okrs", "Edit an objective", "Replaces title, description and period (null clears the last two). Audited with old and new.",
                vec![uuid_param("pid")], &[("200", "the objective"), ("404", "Unknown objective"), ("422", "A bad title or an unknown field")]),
            "delete": op("okrs", "Delete an objective", "Soft delete. Its key results are read through it, and plans stop counting it toward their score.",
                vec![uuid_param("pid")], &[("200", "deleted"), ("404", "Unknown objective")])
        },
        "/api/key-results/{pid}": {
            "put": op("okrs", "Edit a key result",
                "Title, target_value, tolerance, unit, owner_ref and due_date. The baseline (start_value), metric, direction and currency are not editable (progress from a moving baseline is not progress) and sending one is refused. The target may not equal the baseline. Audited with old and new.",
                vec![uuid_param("pid")], &[("200", "the key result"), ("404", "Unknown key result"), ("422", "A bad value or a field that cannot be edited")]),
            "delete": op("okrs", "Delete a key result", "Soft delete; scores that rest on it are recomputed without it.",
                vec![uuid_param("pid")], &[("200", "deleted"), ("404", "Unknown key result")])
        }
    })
}

/// Milestones as a concept: the portfolio view, links to tasks, reopen.
fn milestone_paths() -> Value {
    json!({
        "/api/milestones": {
            "get": op("milestones", "Milestones across the portfolio, with derived status, forecast and slip",
                "Every milestone in the plans the caller can read (others are left out without a trace). status is against the calendar: achieved, late (past due and not achieved), due_soon (within due_soon_days, default 14) or upcoming. A linked milestone is achieved when every task it waits for is done (and it waits for at least one); a manual one when marked done. schedule.state comes from the critical-path forecast of the tasks still open: forecast_late when they finish after the due date, at_risk when they finish within at_risk_days working days before it (default 5), on_track otherwise; a milestone with nothing to forecast from is no_forecast with a reason (no_linked_tasks, no_scheduled_tasks, cycle) and is never on_track. slip reads the append-only reschedule history: original_due, reschedules, later_moves and net_days. The summary counts every visible milestone whatever the status filter. Ordered by due date.",
                vec![query("plan", false, "a plan pid"), query("status", false, "achieved, late, due_soon or upcoming"), query("kind", false, "a milestone kind"), query("from", false, "due on or after"), query("to", false, "due on or before"), query("due_soon_days", false, "default 14"), query("at_risk_days", false, "default 5"), query("as_of", false, "the date today means (default today)")],
                &[("200", "milestones and a summary"), ("422", "A bad status, window or range")])
        },
        "/api/milestones/{pid}": {
            "get": op("milestones", "One milestone with its linked tasks and slip history",
                "Everything the portfolio view derives, plus links (each linked task with whether it is done, its early finish and float) and history (every reschedule: from, to, reason, who, when).",
                vec![uuid_param("pid")], &[("200", "the milestone"), ("404", "Unknown milestone, or in a plan the caller cannot read")])
        },
        "/api/plans/{pid}/milestones/{m_pid}/tasks": {
            "put": op("milestones", "Set the tasks a milestone waits for",
                "Body: task_pids (replaces the set; each a live task of this plan, up to 100, no repeats) and completion_mode (manual or linked; by default linked when there are tasks, else manual). A linked milestone must wait for at least one task and is achieved when they are all done. Audited with old and new.",
                vec![uuid_param("pid"), uuid_param("m_pid")], &[("200", "the mode and task_pids"), ("404", "Unknown plan or milestone"), ("422", "A task of another plan, a repeat, too many, or linked with no tasks")])
        },
        "/api/plans/{pid}/milestones/{m_pid}/reopen": {
            "post": op("milestones", "Reopen a milestone marked done",
                "Body: reason (required, up to 500 characters). Clears the manual completion; audited. A milestone achieved only because its linked tasks are all done cannot be reopened here (409): reopen a task instead.",
                vec![uuid_param("pid"), uuid_param("m_pid")], &[("200", "done and whether it is still achieved by its tasks"), ("404", "Unknown plan or milestone"), ("409", "Not marked done, or achieved by its tasks"), ("422", "No reason")])
        }
    })
}
