# Brainitive: lessons for this repository

Research date: 2026-10-25. Subject: [armendzekjiri/brainitive](https://github.com/armendzekjiri/brainitive)
(PHP / Laravel 12 + Inertia + React; AGPL-3.0; 20 stars; created 2026-02-23; pushed the day of this research).
An open-source team workspace: Kanban and Scrum boards, docs, Excalidraw whiteboards, Gantt plans, a helpdesk,
lead forms, billing and an MCP server, as one self-hosted monolith.

This file is a **map, not a transcription**: what Brainitive does that bears on a portfolio service, what this
repository already has, and what to take. The pattern follows [prince2](../prince2/index.md) and
[total-project-control](../total-project-control/index.md). The decisions that came out of it are in
[plan.md §71 to §76](../../plan.md) and the tasks `BR-1` to `BR-10` in [tasks.md](../../tasks.md).

**Licence.** AGPL-3.0 is strong copyleft. This repository takes **ideas only**; no code, text or schema is to be
copied from it.

**Evidence base.** Read: the README, `AGENTS.md`, the head of the CHANGELOG, `docs/ai/README.md`, the two skills
`track-work-in-brainitive` and `add-mcp-tool`, the headings of `KNOWLEDGE.md` and `CONVENTIONS.md` and the
webhook and insights sections of `KNOWLEDGE.md`, 18 of the roughly 80 per-branch plan files in `docs/ai/tasks/`
(project health, workload planning, quick capture, backlinks, meeting notes to tasks, project templates, releases,
plan-issue links, duplicate issues, secure sharing, AI project context, the security review, the CI test gate and the
e2e plan), three middleware files and the file listing of `app/`. **Not read:** most source code, any test, the UI,
the deployment files. Every statement below about how Brainitive behaves comes from its own documents and is
**unverified by running it**.

## What it is, and how it differs from this service

Brainitive is a workspace for small teams. It has no portfolio, no critical path, no earned value, no stage gates
and no governance records; its boards, docs and helpdesk are the product. This service is a registry and
decision-support service for portfolios. So the overlap is narrow: **boards, plans, templates, time, webhooks, an
MCP server, status reporting and workload**. The value is not its features but how a small team documents and
hardens each one. The strongest material is in its per-branch plan files, which state decisions, definitions and
test plans precisely.

## Lessons

### 1. Writes an AI client may retry are idempotent by key (take: BR-1)

Every MCP write tool takes a required `request_id`; the write runs inside a helper that stores the result under
`(user, tool, request_id, payload)`. A retry with the same payload returns the stored result flagged as replayed;
the same id with a different payload is refused. Concurrently edited documents take an `expected_revision` and a
stale write is refused. The same helper guards the meeting-notes creation endpoint.
*Here:* `Idempotency-Key` exists only in the bulk import spec, and optimistic concurrency only on some resources.
NC-2 is read-only for now, but webhook consumers, integrations and a later write tool all retry.

### 2. A status report is a person's judgement over deterministic evidence (take: BR-2)

The project health page computes counts live, lists the items behind every number, and publishes the definitions
("how these numbers are counted"): due today is not overdue; a blocker only counts if open and readable by the
viewer; missing dates make a metric **Unknown**, never zero; there is **no computed health score**, the status is
chosen by a person. Saved reports keep claims with source references that are validated on save (an unknown
reference is a 422) and resolved for the *reader* on display: a source the reader cannot open shows as
"Unavailable source" with no title and no link. Reports store ids, never titles.
*Here:* NC-4 has the person-chosen status and the cadence, but a narrative is free text with no checked evidence.
The "Unknown, not zero" and "no score" rules match this repository's doctrine and are worth stating wherever a
number can be missing.

### 3. The model proposes, the server verifies, a person creates (take: BR-3, without a model)

Meeting notes become proposed issues in three steps; nothing is written before the last. Every proposal must quote
an excerpt that is found in the notes; one that is not is dropped and counted. An assignee is matched only against
project members; several matches are shown as ambiguous and a person chooses. A due date is accepted only inside a
window around the meeting date. Output is capped, never silently truncated, and creation is one transaction under an
idempotency key. The notes are passed between a random boundary and the prompt says they are data.
*Here:* `plan.md §16` refuses an AI assistant inside the service. The **verify and create half** needs no model and
fits NC-14: an external agent, or a person, proposes action items from minutes; the service checks and creates them
once. That keeps the refusal and gives the useful half.

### 4. Backlinks are one table, derived entries are written by hooks (take: BR-4)

A single `content_references` table (source, target, origin manual or derived) feeds a "Related content" and a
"Referenced by" section on every resource. Derived references are written by model hooks so every write path is
covered; rows are scoped to one project; visibility is applied in batch (one query per type) so a count or a picker
never leaks a private draft; a target the user cannot see answers the same 422 as one that does not exist.
*Here:* meetings (NC-14) and lessons (NC-8) point at plan records but nothing answers "which meetings mention this
risk". The no-existence-leak rule is the one to copy.

### 5. Outgoing webhooks: the rules that keep them from hurting the caller (take: BR-5)

Deliveries are created after the transaction commits, sent after the response, bounded (25 per request, 15 s, 5 s
per attempt), and a failure never breaks the request. The signature is HMAC-SHA256 over `timestamp + "." + body`,
with the timestamp in a header so a receiver can reject replays; the delivery id is a header and the envelope id
stays the same on redelivery; a "Redeliver" action exists; the secret is encrypted, hidden and returned once with
`no-store`; payloads carry ids, titles and names but no emails or rates; private-network targets are refused on save
and before every attempt.
*Here:* NC-11 signs the body only, has no timestamp and no redelivery route. The SSRF guard and payload minimisation
already exist.

### 6. A security review is a list of verified findings, each with a test that fails on the old code (take: BR-6)

An external AI review gave 3 high findings and 3 leads. Each was verified in the code, the same pattern was looked
for nearby, and every fix got a regression test written to **fail against the unfixed code**. The classes found:
uploads that kept the client's file name (the container then executed `*.php`: remote code execution); a token that
covered every ticket of a contact across workspaces; rich text rendered raw; nested route ids not tied to their
parent (cross-workspace edit and delete); a shared global setting editable per project; SSRF through link previews
and AI endpoints (private ranges, redirects); search and an MCP `fetch` tool that returned records by workspace
membership alone; a missing suspended-account check on token APIs; a path-traversal in a temporary file name.
*Here:* the patterns map onto attachments, sinks, search and the future MCP adapter. A sweep is cheap and the
"test must fail on the old code" discipline is the part worth adopting.

### 7. Planning rules that stop numbers lying (audit: BR-7)

Workload planning states its rules: an allocation is explicit minutes for one person in one week and is never spread
automatically; a parent and its subtasks cannot both carry allocations (no double counting); logged time lowers the
remaining estimate once; a changed estimate or reassigned issue **flags** an allocation for review and never
overwrites it; an override beats a default; no capacity declared is *not set*, not zero; weeks are Monday dates so a
DST change cannot move them; writes carry the value the planner saw and a mismatch is refused.
*Here:* `allocations` and the capacity heatmap exist. They have not been read against this list.

### 8. One source of truth for agents, thin pointers per tool (take: BR-8)

`docs/ai/` holds architecture, conventions, workflow, recipes and decisions; the files each tool reads
(`CLAUDE.md`, Copilot, Cursor, `AGENTS.md`) are pointers. Reviewer subagents are thin wrappers around **one**
review checklist, so the checklist is the only copy. Every task has a per-branch plan file linked to a board card;
the last step of each task is "update the doc that owns it", which they say is what stopped the docs going stale.
*Here:* `AGENTS.md`, `agents/` and a project skill exist; there are no reviewer subagents and no single checklist.

### 9. MCP details that cost them debugging time (amend NC-2)

A tool may only do what the signed-in user could do in the interface. Every write returns a location. A declared
string output field must never come back null (strict clients reject the whole response). Read-only and idempotent
annotations on tools. All tools in one page because some clients ignore pagination. A model-facing description per
tool and a server instructions block. They kept a temporary request logger to find out why a client saw "no tools"
and say to remove it afterwards.

### 10. Shared state between tests (found: BR-10)

Their e2e suite runs specs in parallel against one workspace, so numbers are asserted per project the spec itself
creates. This repository hit the same thing on 2026-10-25: tests in one process share one search index, an extra
plan named "Apollo" pushed the real one off a fuzzy page, and the restore drill failed. See CHANGELOG.

## Small facts worth keeping

- Duplicating an issue copies planning fields, resets logged time, and never copies comments, attachments, watchers,
  work logs or history; subtasks are an option. A decision to state for any "copy" feature here.
- Releases: one fix version per issue ("single fix version keeps reporting simple"); releasing moves unfinished work to
  a chosen release or clears it, with no default; release notes are a deterministic draft from done items, private
  until published as a snapshot under an unguessable token; unpublish removes access.
- Share links: expiring, revocable, a whitelisted payload, `X-Robots-Tag: noindex`, `410` once revoked, expired or
  deleted, with a view count.
- Insight pages assert a fixed number of queries per page in tests, and keep each definition in one SQL place with a
  note that two places must change together.
- A project is the unit of visibility and of moves between workspaces; rows hang off `project_id` so a move needs
  nothing.

## What not to take

Helpdesk and ticket portal, lead forms, billing and invoicing, whiteboards, the document editor, Google Calendar
sync, a browser extension, a personal capture inbox and focus timer, web push, bring-your-own-key AI writing helpers
and AI extraction inside the service, SQLite-by-default, and the Laravel/Inertia monolith shape. They serve a
different product. The commercial model (cloud plus AGPL self-host with a commercial licence) is theirs.

## Advice

Do first, in this order: **BR-6** (security sweep; cheap, and the findings are generic), **BR-1** (idempotency keys),
**BR-5** (signed timestamps and redelivery), **BR-10** (stop the test coupling). Then **BR-3** and **BR-2**, which
extend NC-14 and NC-4 with the "evidence, then a person decides" pattern. **BR-4**, **BR-7** and **BR-8** are small and
can wait. **BR-9** (share links) only if an owner asks, because public links to portfolio data are a disclosure risk.
