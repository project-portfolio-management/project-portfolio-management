# DHCW Roadmap Editor: lessons for this repository

Research date: 2026-11-14. Source: the repository
[DHCW-Digital-Health-and-Care-Wales/roadmaps-editor](https://github.com/DHCW-Digital-Health-and-Care-Wales/roadmaps-editor),
read at commit `cc81478` (2026-11-14). It has 72 commits, all from 10 to 14 November 2026. It is a static React 19 and
TypeScript app on GitHub Pages, MIT-licensed, copyright Digital Health and Care Wales. With it, DHCW Product Leads
edit the public Now / Next / Later roadmaps of their products.

**Second reading, 2026-11-23.** The repository was read again at `1da2008` (23 November): 302 commits, 230 of them
since the first reading, and renamed `roadmaps`. It is no longer only an editor. It is a pnpm workspace of the editor and
the public site, with a **PostgreSQL API designed but not yet built** (ADR-015 to ADR-020 accepted, 6 placeholder stories,
no `apps/api`). The new lessons, 12 to 19, are in [Second reading](#second-reading-2026-11-23); advice and tasks are in
[plan.md §94](../../../plan.md) and `RE-16` to `RE-20`. Lessons 1 to 11 are unchanged.

This file is a **map, not a transcription**. It covers what the editor does that bears on a portfolio service, what
this repository already has, and what to take. It follows the pattern of [brainitive lessons](../brainitive/index.md).
The decisions that came out of it are in [plan.md §88 to §92](../../../plan.md), and the tasks `RE-1` to `RE-14` are in
[tasks.md](../../../tasks.md).

**Licence.** MIT permits reuse with the notice kept. The stacks differ (React and Google Sheets there; Rust, PostgreSQL
and Svelte here), so in practice this repository takes ideas, and no code is copied.

**Evidence base.** Read: the README, `AGENTS.md`, the ADR index and ADR-008 and ADR-012 in full,
`docs/operations/replacing-google-sheets.md`, the Product Lead persona, the domain types `roadmap.ts`,
`roadmap-item.ts` and `roadmap-placement.ts`, the CI workflow, the head of `build/design-token-lint.ts`, the e2e
fixture header, and the file and backlog listings. **Not read:** most of the source, the unit tests, the public site
(a separate app, not in this repository), and the archived epics. Nothing was run. Every statement about behaviour
comes from the repository's own documents and types. The GitHub API refused metadata (the `nhs-wales` enterprise
enforces SAML), so the repository was read from a public `git clone`.

## What it is, and how it differs from this service

- **Two apps, one data source.** The editor is internal and used only by DHCW staff. A separate public site reads the
  same Google spreadsheet, rebuilds nightly at 02:00 UTC, and shows the published roadmaps. So **saving is
  publishing**, and the spreadsheet's format is a contract with the public site.
- **No backend.** The browser gets a Google token through Google Identity Services (token model, about an hour, kept
  in memory only) and calls the Sheets API directly. What a person can see is decided entirely by Google's sharing
  settings.
- **Sheets is temporary by design.** ADR-002 puts everything behind a `RoadmapRepository` port. The operations guide
  [replacing-google-sheets.md](https://github.com/DHCW-Digital-Health-and-Care-Wales/roadmaps-editor/blob/main/docs/operations/replacing-google-sheets.md)
  plans a **PostgreSQL-backed HTTP API** in its place, and lists the operations, error mapping, contract test and
  CORS that API needs.
- **Vocabulary trap.** Their "roadmap" is a **public product roadmap**: a short, outcome-worded list of what a
  product is doing now, next and later, written for health boards, Welsh Government and the public. This
  repository's "roadmap" (spec §15) is the project's own feature plan, and its `/roadmap` screen (NC-10) is an
  impact and effort matrix over ideas. Neither is the same as a task schedule or a Gantt chart. Tasks here call the
  new thing a **public product roadmap** throughout.

## Lessons

### 1. A roadmap item is not a task (take: RE-1)

An item has a title, a description, an **expected outcome** ("what changes, and for whom"), a phase, free tags,
the organizations the work is for, and details nested up to three levels. It also has a **placement**: one of three
horizons (Now, Next, Later) or one of three sections around them (Recently delivered, Delivered this year, Not
doing). An item with no recognised placement is "unplaced", and the public site skips it. An item should fit on one
phone-sized card; the editor does not enforce a limit.
*Here:* a product plan (`PlanKind::Product`) has tasks, milestones, goals and objectives, but no outcome-worded,
public-facing summary of direction. NC-10 places ideas on Now / Next / Later from ratings kept only in the browser's
`localStorage`, so they are not shared.

### 2. Publishing is one all-or-nothing change set, refused on a co-lead's change (take: RE-2)

Edits stay pending in memory until **Publish**. Then every pending change to one roadmap is written in one atomic
`batchUpdate`, or none is. Each pending edit keeps its **base value**, the field's value when it was first edited.
A refresh keeps pending edits on top of what was re-read, and a co-lead's change to the same field is shown beside
the Product Lead's and never overwritten. Before publishing, a dialog shows what will change. If the write's answer
is lost, the next read decides whether it landed. Text is stored exactly as typed and trimmed, never parsed as a
number, date or formula.
*Here:* `If-Match` exists on a few resources (goals, handover, the sandbox), not on plans or tasks as a whole. There
is no multi-record change set with per-field bases and a preview.

### 3. Freshness is a stated rule, set only on purpose (take: RE-3)

A roadmap untouched for **six weeks** counts as stale (persona answer A7). `Last updated` is a calendar date that
publishing does **not** change, unless the Product Lead ticks "Mark as updated today". That way, fixing a typo does
not make an old roadmap look current.
*Here:* NC-4 has a status cadence, but there is no "reviewed on" date for a public statement, and no rule that keeps
an edit from refreshing it.

### 4. A published flag that the public reader obeys (take: RE-4)

A roadmap has `Enabled` (published or not), and its own publish and unpublish action (STY-035). The public site shows
only published roadmaps, from a fixed set of columns, and ignores columns it does not know. Each roadmap and each item
has a stable public id: the roadmap's URL slug, and an `Item ID` of eight random lower-case letters and digits that
starts with a letter.
*Here:* every read needs a token. Guest grants are per plan, and BR-9 (public share links) was judged risky and left
to the owner.

### 5. A port with a written contract (take: RE-5, only if wanted)

The repository port's doc comments are the contract, and a shared contract test runs against each adapter. The
replacement guide maps HTTP statuses to error kinds (401 unauthenticated, 403 forbidden, 404 not-found, 429
rate-limited, 5xx retryable, unparsable body invalid-response). It also says the API needs CORS from the Pages origin
with an `Authorization` header, and either accepts Google tokens or brings its own sign-in.
*Here:* this service is a PostgreSQL-backed HTTP API. Serving their port is a mapping exercise. It is also a decision
for DHCW and the owner, not something to build on speculation.

### 6. Bilingual fields, and never writing Welsh from English (take: RE-6)

Every display field is `LocalisedText` (English and Welsh). The editor writes English only and never writes a Welsh
cell. An item whose tags or details already hold Welsh **cannot have them edited**, because a changed English list
could no longer be paired with the Welsh. A publish that finds Welsh written since the item was read writes nothing.
The interface is in English and Welsh, typed so that a missing Welsh key or plural form fails the type check, and the
Welsh is marked machine-assisted until a fluent speaker reviews it.
*Here:* issues already carry a map of language to text (`src/issues.rs`), and the UI has 11 locales with a review
record. No stored content marks a translation as out of date when its source changes.

### 7. Reference lists defined once, linked by id, with drift shown (take: RE-7)

Families and categories each live in their own tab with an id, English and Welsh names and a colour. A roadmap links
to them by id, and also keeps copies of the names, because the public site reads the copies. A copy that differs from
the list is flagged **out of step**, with a "Make it match" action. A family not in the list is flagged too. There is
also a list of organizations: NHS Wales health boards by code, and `National` (All Wales) first. An unknown code is
kept as written, but not shown.
*Here:* `organizations` exist, but none has a short code or a "national" scope, and portfolios have no shared,
bilingual category list.

### 8. A persona with numbered owner answers (take: RE-12)

`docs/backlog/personas/product-lead.md` records who the user is, how long they have each week (30 minutes), their
devices and pain points. Each fact is tied to an owner answer, A1 to A13, and stories cite those answers ("A9, A10").
It also says what is still to be checked with real users.
*Here:* `spec/03-stakeholders-and-users.md` lists roles, with no numbered answers for stories to cite.

### 9. Tests that look at structure and refuse the outside world (take: RE-9, RE-10, RE-11)

- **Accessibility-tree snapshots** (34 `*.aria.yml` files) for each important state, in English and Welsh: start,
  empty, failed, expired, the publish dialog, a conflict review, the not-found page. *Here:* there are none; we use axe
  and text assertions.
- **A hermetic e2e page:** the only outside world is a fake Google Identity Services script and a fake Sheets API.
  Any request or WebSocket to another origin is blocked on the browser context, popups included, and fails the test,
  as do uncaught page errors and service workers. *Here:* the e2e fixtures stub `**/api/plans**` and some
  other routes, and an unstubbed request is not refused.
- **A design-token lint:** with Tailwind's defaults removed, ESLint rejects any colour, type size, radius, shadow or
  focus style that does not come from a token, and a fixture proves each rule's message. A contrast test checks the
  token pairs. *Here:* the UI forbids `style="…"` (the CSP blocks it), but nothing stops a raw colour in a class or
  stylesheet.

### 10. Pending work survives an expired session, and leaving asks first (take: RE-13)

A token that expires does not lose pending edits; **Reconnect** keeps them. Disconnect with edits pending asks:
keep them for the next connection, discard them, or cancel. Closing or reloading the tab asks first. Nothing pending
is written to browser storage. *Here:* the UI's long forms have not been checked against an expired session or a
closed tab.

### 11. Small working practices (take: RE-14, optional)

ADRs in Nygard format that are amended, never deleted, with each amendment dated in the header. Every story has a
UX analysis and HTML prototypes before code. Each pull request ends with a retro, and the lessons go into the agent
skills. A pull request template states what is written to the shared data source. *Here:* the spec plays the ADR's
role, and CHANGELOG entries state what was not done.

## Second reading, 2026-11-23

**What changed.** 230 commits since the first reading, from 7 October to 23 November (34 on 19 November and 41 on 22
November, the busiest days). By type: 38 `feat(roadmaps)`, 27 `docs(backlog)`, 23 `docs(retro)`, 11 `docs(agents)`,
10 `docs(analysis)`, 10 `test(e2e)`, and a few `ci`, `build`, `fix` and `chore`. The work that matters here is in three places: a **design for the database and the API** (the
accepted ADR-015 to ADR-020, 1,497 lines), a **workspace and CI move** (STY-085 to STY-088), and a **test-slimming run**
(CHO-036 to CHO-043). The repository's own records of the last are the best view of how it is delivered.

**Evidence base.** Read in full: ADR-015 (the database), and the decisions of ADR-016 to ADR-020; the delivery-run summary
for CHO-036 to CHO-042; the CHO-031 supply-chain commit; the EPC-022 and EPC-023 epics' status lines; the commit log. **Not
read:** the editor's source since 14 November, the public site, the 23 retro commits' text, or the identity analysis. **Not
run.** Nothing here describes running code for the API: `apps/` holds only the editor and the public site, and the API
stories (STY-090 to STY-094) are placeholders. These are designs, not behaviour, and they may change when built.

### 12. A change set, a snapshot and a change summary for every write (take: RE-18, assess first)

ADR-015 gives every write to a draft, a list or access one **change set**: who, when, its scope, its kind, a note, the
revision it was based on and produced, and a client-supplied key that makes a retry return the first result. A
**database function builds the snapshot itself** after the write, as one JSONB document with its schema version, so past
content is a read, never reconstructed from rows. Each change set also keeps a **change summary**, one row per field with its
old and new value, so "who changed what" and "who last changed this field" are queries. Triggers refuse a write to a working
table outside a completed change set, after its sign-off, or outside its scope; the application's role cannot update or
delete history. **Restoring writes forward** as a new change set. **Redaction** is the one exception: an audited procedure
replaces named content with `[redacted]` everywhere it is held and records who, when and why, never the content.
*Here:* `audit_logs` records the act, with an idempotency layer (`src/idempotency.rs`) and a tamper-evidence design (SA-7,
not built). It does not keep a past version of a plan to view, a per-field summary, or a restore. Erasure is a policy (WW-1.8),
not a redaction procedure across history. See RE-18.

### 13. A private scratch, a shared draft and a publication that pins a snapshot (take: none; compare with RE-2)

Unsaved edits live in a **private scratch on the server**, one per person per roadmap, visible to its owner alone, never
history, deleted on save, discard, end of assignment or expiry. Saving promotes it to the roadmap's **one shared draft** and
merges **field by field** against the snapshot of the save's base revision under the roadmap lock: a field the save changes
and nobody changed since is written; one changed since to another value is a conflict; a reorder has its own version.
A **publication pins a snapshot**, so unpublishing is a change set and the public read is of exactly what was published.
*Here:* RE-2 publishes one change set and refuses a co-lead's change (server-side, from the first reading). The scratch, the
merge by field and the pinned snapshot are finer than RE-2 and not taken: this service has no shared draft of a roadmap.

### 14. Forced row-level security, set per transaction, failing closed (take: nothing; one idea in RE-19)

Every tenant-owned table carries `tenant_id`, with `UNIQUE (tenant_id, id)` on parents and tenant-keyed foreign keys, and
**row-level security that is enabled and forced from the first migration**: the policy matches `current_setting('app.tenant_id',
true)` and reads nothing when it is unset. The tenant and the person are set **per transaction only** (`set_config(…, true)`),
so neither leaks through a pooled connection; views are `security_invoker`; the few `SECURITY DEFINER` functions pin their
`search_path`; forced RLS binds the migration role too. Tests run as the application's own role against a real PostgreSQL 18.
*Here:* this service is **one deployment per customer** (SA-1, SA-R13) and keeps tenancy out on purpose, so none of this is
taken as tenancy. The part that transfers is a backstop for **private plans**: today `HiddenScope` is applied in application
code; a forced policy keyed on a per-transaction setting would make a missed filter return nothing. Not a task now (RE-19
asks whether to try it on one table).

### 15. The API runs sign-in; sessions are server-side; invitations are single-use links (take: none)

The API is a confidential OpenID Connect client for a **fixed allow-list of issuers in code**; ID tokens are validated by a
maintained library; the mock provider can never be admitted in a deployed environment. A person is `(issuer, subject)`, never
matched by email; **a person record exists only after an invitation is redeemed**, a single-use link plus a matching verified
email that the admin copies and sends themselves. A session is an opaque 256-bit row in PostgreSQL behind a `__Host-`, `Secure`,
`HttpOnly`, `SameSite=Strict` cookie, ending after 60 minutes idle or 10 hours; every request re-reads the session and
refuses a disabled person. Security events go to the structured log, never the database, and carry IDs, never tokens or emails.
*Here:* this service takes bearer tokens from the organization's own identity provider and has no sessions, no invitations
and no browser cookie (SA-D2: OIDC only). The editor needs a server session because it has no gateway; this service does not.
Not taken.

### 16. Every route declares one policy; permissions are a catalogue in code; access is reviewed (take: RE-19, check)

ADR-019: the **permission catalogue is a typed enumeration in the API's code**; **every route declares exactly one policy**
(public; signed in; a tenant permission; a platform one) and a test fails a route that declares none; permission names are
referenced only through the enumeration (a lint); there are **three shipped roles**, and a tenant cannot edit roles yet; a
**tenant keeps at least one admin** (the API refuses to end the last); an **access review every six months** shows each
member's roles and last sign-in. **Platform operations** are one-command Cloud Run jobs only named people can run, and who ran
one comes from Cloud Audit Logs, not from the command line; there is **no operator web surface and no impersonation**.
*Here:* there is an ABAC layer and an access-review module (`access_review`), and support grants are planned (SA-4). The
two ideas worth checking, not checked in this reading: that a test fails a route with no declared policy, and that the last
administrator cannot be removed.

### 17. One origin, no CORS, edge rate limits, an OpenAPI document that CI keeps honest (take: RE-19)

ADR-020: the editor and the API **share one origin**, so the editor's endpoints send **no CORS headers at all**; a
state-changing request needs same-origin proof (`Sec-Fetch-Site: same-origin` or a matching `Origin`); rate limits live at the
edge (Cloud Armor), and an environment without the load balancer holds no real data. ADR-017: **the OpenAPI document is
committed and CI fails when it drifts from the code**; **every refusal is `{detail, code, params}` with a stable `code`**, and
the document lists every code, so the editor's adapter maps each. *Here:* the OpenAPI document is committed and checked, and
the service has its own rate limiter and a refusal shape; whether the codes are **enumerated in the document**
and a test fails a code the document omits is not checked. See RE-19.

### 18. A delivery that records itself, and tests that hold each rule once (take: RE-17, RE-20)

The repository is delivered by agents, one fresh agent and one pull request per item, and each run leaves a **delivery-run
record**: models used, review lanes and rounds, findings fixed and declined, timing, escapes found after review, rework,
human interventions, bot status and the retro outcome, with an "evidence limits" line saying what the record cannot show.
Each pull request ends in a **retro** whose lessons go into the agent skills (23 retro commits; for example "an item's stage
timings come from its progress log", and "observe an event-gated CI condition before merge", which followed DEF-031, a fix so that
the editor deploys from `main` again). The seven chores of CHO-014 then **slimmed the end-to-end suite so that each rule is asserted
once, at its lowest seam**: one spec's runs fell from 133 to 83 and its lines from 4,602 to 3,067, another from 128 to 60.
A standing owner rule (23 November) adds an axe scan to each page's brief. *Here:* the UI has about 615 Playwright tests and
a CI run takes about 45 minutes, with one Playwright job near 20 minutes; the first reading's RE-9 to RE-11 added structure
checks without asking where a rule is asserted twice. A flaky axe test on 23 November (a page scanned before its title
existed) is the cost of waiting for `networkidle` instead of for the thing the scan needs. See RE-17.

### 19. Supply-chain checks on every pull request (take: RE-16)

CHO-031: **GitHub's dependency-review action** and an **OSV-Scanner audit of every lockfile**, with a config file and
pnpm's release-age setting, run on every pull request; workflow files are linted with actionlint and zizmor; `main` is
protected by a ruleset applied to the whole workspace; hooks install on every fresh install (DEF-028). A retro from that run
holds that **a repository setting only the owner can turn on is an owner-only scenario** to be named in the story, not
discovered at merge. *Here:* `cargo deny`, `cargo machete`, gitleaks, zizmor, a two-day publish-age rule and a container
scan (SA-15, not built) exist; **dependency review and an OSV audit of the UI's and the matcher's lockfiles do not.** See RE-16.

### What the second reading says about the first

- **RE-5 and OQ-15 are settled by DHCW.** The first reading found a guide that planned a PostgreSQL API "to replace Google
  Sheets"; the project asked whether to offer this service as that API. DHCW has since accepted its own: Python 3.15,
  FastAPI, SQLAlchemy, a tenant-scoped schema, its own OpenAPI contract. The decision of 2026-11-16 (not offered, the path kept
  open) stands, and the path is now narrower: RE-5 stays as a **mapping and a one-time dry-run import** of the spreadsheet
  format, and is not extended into an adapter.
- **The vocabulary trap is unchanged**, and now there is a second reader: the editor's "tenant" is an NHS Wales organization,
  not a customer deployment in the sense of SA-R13.
- **None of the first reading's lessons were withdrawn.** Lesson 3 (freshness) and 6 (bilingual text that is never written
  from English) still hold; ADR-008's two-language rule is carried into ADR-015 as an English and a Welsh column per field.

### Not taken from the second reading

- **Multi-tenancy, a tenant on every row, tenant-scoped uniqueness and a platform job to create a tenant.** SA-1 and SA-R13
  chose one deployment per customer, and a census test keeps it so.
- **Server-side sessions, invitations and an Entra and Google sign-in in the API.** This service does not sign anyone in.
- **A private scratch and a field-level merge** (lesson 13). The service has no shared draft to merge.
- **Python, FastAPI and SQLAlchemy, Cloud SQL and Cloud Armor.** Other stack and hosting choices.
- **Treating the designs as evidence of behaviour.** They are accepted decisions, not running code.

## Not taken

- **Google Sheets as the store, and Google Identity Services in the browser.** We are one service over PostgreSQL,
  behind the organization's own identity provider. Sheets access is also coarse: anyone who can edit the sheet can
  edit every roadmap.
- **No backend, and visibility decided by Google.** That stands against private plans, handling marks and audit.
- **Locking a field because it holds Welsh.** It is the right call for a spreadsheet the editor cannot reason about.
  With a database, the better rule is to keep the Welsh and mark it out of date (RE-6).
- **A nightly public rebuild as the publish step.** A published change here should be visible to the public reader
  at once, and the reader should carry an `ETag`.
- **React, shadcn and Base UI, TanStack Router with hash history.** Other stack choices.
- **Free-text "Product Lead" names on a public page.** This service holds people as references, not names. A public
  roadmap names a role or a team, never a person, unless the person agrees (RE-4).
