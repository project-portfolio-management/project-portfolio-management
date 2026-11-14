# DHCW Roadmap Editor: lessons for this repository

Research date: 2026-11-14. Source: the repository
[DHCW-Digital-Health-and-Care-Wales/roadmaps-editor](https://github.com/DHCW-Digital-Health-and-Care-Wales/roadmaps-editor),
read at commit `cc81478` (2026-11-14). It has 72 commits, all from 10 to 14 November 2026. It is a static React 19 and
TypeScript app on GitHub Pages, MIT-licensed, copyright Digital Health and Care Wales. With it, DHCW Product Leads
edit the public Now / Next / Later roadmaps of their products.

This file is a **map, not a transcription**. It covers what the editor does that bears on a portfolio service, what
this repository already has, and what to take. It follows the pattern of [brainitive-lessons](../brainitive-lessons/index.md).
The decisions that came out of it are in [plan.md §88 to §92](../../plan.md), and the tasks `RE-1` to `RE-14` are in
[tasks.md](../../tasks.md).

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
the organisations the work is for, and details nested up to three levels. It also has a **placement**: one of three
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
also a list of organisations: NHS Wales health boards by code, and `National` (All Wales) first. An unknown code is
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
