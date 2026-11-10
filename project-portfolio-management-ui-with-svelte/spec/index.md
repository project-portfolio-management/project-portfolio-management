# Portfolio Front-End — Specification

> **Single source of truth.** Code conforms to this spec. Behavioural
> change = spec + code + test. Live work queue is §13.
>
> Sibling service: [project-portfolio-management-service-with-rust](../../project-portfolio-management-service-with-rust/spec/index.md).
> Entity umbrella: [portfolio/spec](../../spec/index.md).

## 1. Purpose and vision

A SvelteKit SPA for portfolio managers, programme leads, and project
operators to register, browse, edit, and duplicate-check **plan**
identities in **one recursive collection** — **and** to run each plan as a
live project workspace: goals, a Kanban task board, issues, a timeline /
Gantt, and a burndown chart. It is a thin presentation layer over the
portfolio service's REST API (`/api/plans/...`); the Rust service is the
system of record.

The entity has two faces that share one record (entity spec §1): a
**matchable identity registry** (plan-level dedup across the whole
collection) and a **project-management tool** (operational
sub-resources). The front-end surfaces both — identity CRUD +
duplicate-check + merge on one side, the project-management views on the
other. The canonical matchable Rust type is **`Plan`** with an
**optional** `kind: PlanKind` descriptive label; a plan may contain any
other plan (it carries an optional `parent_ref` to its parent), so the
tree is recursive. Every record is a plan in **one collection / table**,
and matching runs across the whole collection — it is **not** gated by
kind.

## 2. Scope

In scope:

- The **identity routes** under the static `plans/` directory
  (`/plans`, with `/plans/new`, `/plans/[pid]`, `/plans/[pid]/edit`):
  the plan list (SVAR DataGrid), create/edit form, detail page. The nav
  has a single **Plans** destination (no collection switcher).
- The **API client** (`src/lib/api/{types,client,plans}.ts`), the
  plan form, a client-side name filter on the list, and a
  duplicate-check screen (score + confidence per candidate). A
  standalone record-merge page (`/plans/merge`, landed 2026-08-03) folds
  a confirmed duplicate into a survivor, with a recent-merge history
  table. **Not built**: a per-component match-score breakdown visual and
  a per-plan audit timeline (§13).
- The **child roll-up** (not yet built, §13): a plan detail page is meant
  to also list its child plans (those whose `parent_ref` is this plan's
  pid); the service supports it (`GET /api/plans?parent={pid}`, and
  `PlanRepository` carries the query param), but no route calls it yet.
- The **project-management views** on the detail page (or its
  sub-routes) for any plan: a **Kanban task board** (drag = status
  change), an **Issues** list, a **Gantt / timeline** view, a
  **burndown** chart, and a **Goals** panel.
- The **layout shell**: a top navigation bar with a leftmost hamburger
  menu, full-width content, a full theme catalogue selector, and a
  13-locale i18n selector (RTL for `ar` / `ur`).

Out of scope (stated): no FHIR surface (the service exposes none); no
consent-management UI; no finance / budgeting UI; **no posts / comments
feed, no member-role panel** (the portfolio entity carries only
tasks / goals / issues as sub-resources — see entity spec §5; posts /
comments / members are roadmap-only). There is **no password /
credential form** — sign-on is passwordless magic-link, run through this
app's own `/signin` + `/verify` BFF routes against the central
authentication-service (sibling repository, not in this repository), or, when
`OIDC_ISSUER` is set, the authorization-code flow against an OpenID Connect
provider (`/signin/oidc`, NFR-18). **Stated limit (WW-1.18):** those are the
only two ways in. There is no sign-in by phone (an SMS or voice code), and no
national identity scheme of its own; a national or sector eID that offers
OpenID Connect can be used through the OIDC path (the API accepts any OIDC
issuer, WW-1.12), but none has been tried. An e-mail address is therefore
needed for the magic link wherever no OIDC provider is configured.
Per the BFF + httpOnly-cookie model
(`../../../agents/share/authentication-sessions.md` (shared family guidance, not in this repository))
the browser holds only the `__Host-mxi_session` cookie and the SvelteKit
server attaches a short-lived PASETO server-side (calling the portfolio
service through the same-origin `/api/proxy` route) — no token in JS, no
`localStorage`.

## 3. Stakeholders and users

Portfolio managers and PMO analysts (dedup, roll-up across the plan
tree), programme / project leads (the workspace), team
contributors (tasks, issues, goals), and auditors (the audit timeline +
match breakdown).

## 4. Glossary

- **pid** — the plan's public id (route param).
- **Plan** — the `project_portfolio_management_matcher::Plan` payload (the
  matchable identity header; the API DTO = the matcher type, persisted as
  JSONB). Carries an **optional** `kind` (Portfolio / Project / Product /
  Program / Practice / Process / Purpose / Pathway / Proposal)
  descriptive label; every plan lives in one collection / table.
- **Collection** — the single recursive `/api/plans` collection holding
  every plan. One REST collection / list / CRUD; matching runs across the
  whole collection.
- **kind** — an **optional** descriptive label on a plan (Portfolio /
  Project / Product / Program / Practice / Process / Purpose /
  Pathway / Proposal). It is display metadata; matching is **not**
  gated by kind.
- **parent_ref** — the containing plan's `pid` carried by any plan
  (optional; any plan may contain any other). Drives the roll-up and is an
  exact-match supporting signal.
- **Sub-resource** — a `Goal` / `Task` / `Issue` owned by a plan,
  reached under `/api/plans/{pid}/…`. Not part of the matching
  surface (except goal titles).
- **check-duplicates** — POST the current record to find stored matches
  (across the whole collection).
- **MatchBreakdown** — the per-component score map returned by a match
  (name, goals, code, owner org, parent, timeframe, keywords,
  relationships, tags).
- **EntityRef** — an opaque `*_ref` into person / worker / auth-user /
  organization, stored verbatim and resolved by the front-end picker.
- **Derived view** — a read-only computed view (timeline, burndown),
  never canonical state.

## 5. Information architecture

```
/                            landing → links to /plans
/dashboard                   PPM dashboard (site tiles + RAG / stage rollups)
/plans                       list (SVAR DataGrid + FilterBar)
/plans/new                   create form
/plans/[pid]                 detail + delete + check-duplicates
/plans/[pid]/edit            edit form
/plans/[pid]/governance      governance panel (gates, risks, budget,
                             benefits + ROI, OKR mappings, milestones,
                             allocations)
/plans/[pid]/schedule        plan schedule (child timeframes)
/plans/[pid]/board           per-plan task Kanban + sprints +
                             honest burndown + standup digest; an
                             "Accessible table" view (Lily headless
                             KanbanTable) moves a card with a labelled
                             control, no drag needed
/plans/[pid]/critical-path   float, critical chains, Gantt + network
                             diagram + table, and a task schedule editor
                             that answers with what moved (SC-1/SC-2)
/plans/[pid]/calendar        the plan's working weekdays and holidays
/me/calendar                 my working weekdays and days off
/me/notifications            my inbox, what I am told about, and where
                             copies go (NC-1 slice 4); linked from /me
/me/timesheet                my week by day and plan, sent for approval,
                             and an approver's decision on someone
                             else's week (NC-1 slice 6); linked from /me
/gantt                       schedule Gantt (Lily GanttChart: a bar is a
                             run of named cells; dependencies as text,
                             critical path in colour and words, zoom,
                             CSV/Excel export; read-only)
/capacity                    resource capacity rollup
/capacity-heatmap            demand and supply by skill pool and month;
                             a linked pool's supply from the workforce
                             service with its reading (FB-6)
/skill-pools                 skill pools and their link to the
                             workforce service's pool (FB-6)
/capacity-commitments        set a capacity commitment; the outcome of
                             the demand sent to the workforce service,
                             the withdrawal at zero included (FB-6, FB-5)
/engineering                 blocked aging + MoSCoW + delivery links
/calendar                    estate milestone calendar (kind filter)
/executive                   CEO area: portfolio-health briefing,
                             decision log, benefits realization
/financials                  CFO area: budget variance + per-currency
                             exposure (server-derived; no FX)
/technology                  CTO area: technology radar + dependency
                             risk lens + tech-debt register + flow
                             metrics
/ideas                       idea board
/objectives                  OKR objectives + alignment rollups
/proposals                   work-intake proposal pipeline
/roadmap                     ideas and proposals on an impact / effort
                             matrix and in Now / Next / Later (NC-10)
/reports                     saved reports + CSV download
/scenarios                   scenario planning + side-by-side compare
/board                       board pack + investments + snapshot trends
/auditor                     audit explorer + SoD findings + evidence pack
/compliance                  compliance register + conformance findings
/risk                        CRO heatmap + posture + appetite
/security                    CISO register + late-stage heuristic
/security/access             the access review and emergency (break-glass)
                             access to a private plan (NC-1 slice 7);
                             linked from /security
/regulator                   coarse regulator extract (mask-aware)
/signin · /verify            BFF magic-link sign-in / verification

/plans/[pid]/issues          the plan's issues register: raise, edit,
                             raise from a risk (NC-1 slice 1)
/plans/[pid]/change-requests the plan's change requests: draft, edit a
                             draft, submit, review, decide, withdraw
                             (NC-1 slice 2)
/plans/[pid]/tasks/[tpid]    one task: edit its fields, add and remove
                             its links to other tasks (NC-1 slice 3);
                             follow it, react, comment, attach files
                             (NC-1 slice 5); each board card's title
                             links here

Also built, described in the sections below (the list above is not
every page; `find src/routes -name +page.svelte` gives all 106):
/announcements /approval-processes /automations /balance /ceo /change-
requests /cost-rates /cost-risk /demo /director-packs /director-
packs/[pid] /display /documents /documents/[pid] /email-deliveries
/escalations /issues /kanban /key-persons /kpis /ledger /lifecycle /me
/milestone-signoffs /milestones /okrs /onboarding /organizations
/pipeline /plans/[pid]/buffers /plans/[pid]/cost-risk /plans/[pid]/costs
/plans/[pid]/critical-chain /plans/[pid]/cutover
/plans/[pid]/deployment-moves /plans/[pid]/earned-value
/plans/[pid]/escalations /plans/[pid]/flow /plans/[pid]/governance-
framework /plans/[pid]/handover /plans/[pid]/import /plans/[pid]/joint-
risk /plans/[pid]/leveling /plans/[pid]/links /plans/[pid]/onboarding
/plans/[pid]/raci /plans/[pid]/schedule-health /plans/[pid]/schedule-
risk /plans/[pid]/tasks/[tpid] /plans/[pid]/tolerances
/plans/[pid]/ways-of-working /plans/[pid]/wbs /plans/[pid]/what-if
/plans/[pid]/work /plans/merge /prioritization /resources /resources/rbs
/reviews /search /start-queue /tolerances /tour /issue-settings

Roadmap (per-plan project-management sub-routes, §13/§15; not built):
/plans/[pid]/timeline        Gantt / timeline (goal milestones + task date ranges)
/plans/[pid]/burndown        burndown chart (remaining estimate over time)
/plans/[pid]/goals           goals panel
```

(The project-management views MAY be implemented as detail-page tabs
rather than discrete sub-routes. The spec fixes the *capabilities*, not
the URL shape. If sub-routes are used they share the `[pid]` layout.)

### Layout shell & navigation

Cross-cutting UI rule for every `*-front-end-with-svelte` app, applied
here:

- Global navigation MUST be a **top navigation bar** (header) spanning
  the full viewport width. There MUST NOT be a left-hand navigation
  sidebar / rail.
- The navigation toggle is a **hamburger menu** placed **leftmost** in
  the top bar; on narrow viewports the top-bar navigation collapses
  behind it. The hamburger is present (leftmost) regardless of viewport.
- The main content area MUST be **full-width** — never inset by a
  persistent side-navigation column.
- The nav has a single **Plans** destination (no collection switcher). A
  chrome utility area in the top bar carries the **theme selector**
  (`lily-design-system-svelte-theme-picker`) and the **locale selector**
  (`lily-design-system-svelte-locale-picker`), plus the session affordance
  (Sign in / Sign out).

### Theming

The app uses the **full shared Lily/DaisyUI theme catalogue** for
parity with the rest of the family. Selecting a theme via
`ThemePicker` changes the whole site look: it manages exactly one
`<link rel="stylesheet" data-lily-theme-picker="theme">` in
`document.head`, mutating its `href` and the `data-theme` attribute on
`<html>`. The choice persists to `localStorage` (key `portfolio:theme`).
Theme stylesheets are served from `static/assets/themes/` (a symlink
to the shared design-system themes).

With no stored choice the theme follows the system light / dark preference
(`detectFromSystem`). The app's own colour tokens (`src/app.css`) are
derived from the theme's so that text meets WCAG 2 AA (4.5:1) under both
the `light` and the `dark` theme: `--mxi-color-link` (every link, and text
written in the primary colour) is the theme's primary mixed 70 / 30 with
its text colour (the dark theme's primary as is measured 3.4:1 as link
text); `--mxi-color-primary` (the fill behind primary-content text: primary
buttons, the sign-in button) is the primary mixed 80 / 20 with the theme's
neutral (4.14:1 as is under dark); the danger and success colours are mixed
with the text colour, and the error and success banners sit on a tint of
the page background. `tests/e2e/a11y.spec.ts` audits every page route
under both system preferences. The other themes in the catalogue are not
audited.

### Money (WW-1.3)

An amount is an integer in the minor unit of the currency the record carries. One module (`src/lib/money.ts`) formats
it (`Intl.NumberFormat`, the reader's locale, the currency's own number of decimals: JPY 0, EUR 2, KWD 3, CLF 4),
reads what the reader types in that locale (so "1,5" and "1 234,50" work where the locale uses a decimal comma) and
scales it by the same exponent. The exponent is the ISO 4217 minor unit, from a table (`MINOR_UNITS`) that is the
service's `src/currency.rs` table; `tests/unit/money.test.ts` reads that file and fails if the two differ. `Intl`'s
digits are used only for a code the table does not hold (and 2 when `Intl` does not know it either), because CLDR,
where `Intl` takes its digits, differs from ISO 4217 for 16 codes (AFN, ALL, COP, HUF, IDR, IQD, IRR, KPW, LAK, LBP,
MGA, MMK, PKR, SOS, SYP, YER: 0 places in CLDR, 2 in ISO 4217, or 3 for IQD). Formatting sets both
`minimumFractionDigits` and `maximumFractionDigits` to the ISO exponent, so a forint amount shows its two places and
what is shown, typed and stored agree. The same rule holds in the contact relationship and workforce planning UIs'
`money.ts`; a fix to one is copied to the other two. No page divides or multiplies by a fixed 100, and no form starts with a currency: the
field is empty and required (an amount without a currency is refused before anything is sent), with the neutral hint
"for example EUR, JPY or GBP".

### Numbers, dates and plurals (WW-1.16, WW-1.24 in part)

Every number, percentage and date a person reads goes through `src/lib/locale-format.ts` in the page's locale (the one
in the URL): grouping and decimal mark, the locale's percent form ("25 %" in German), and a civil ISO date as the
locale's date ("30 Oct 2026", "30.10.2026", "30 Hyd 2026"), formatted in UTC so it is the same day everywhere; a
timestamp is shown in the reader's time zone. An unknown figure is "—", never 0. Display only: the API still sends and
takes ISO dates, basis points and fractions. The module is a port of the workforce planning management UI's
`locale-format.ts` (WPM-T375, WPM-T377) and shares its behaviour; the contact relationship management UI is to format
the same way (it had no such module on 2026-10-30). PPM adds `formatDayMonth` for chart axes. Gregorian only.

A string that fills a count is a plural group (`<base>_one`, `<base>_other`, and `_zero`, `_two`, `_few`, `_many` where
a language uses them) chosen by `tn(base, count, values)` with `Intl.PluralRules` in the page's locale, as WPM's `tn`;
code never picks `_one` with `n === 1`. A locale with no text for a group reads English by English's rules, as for any
missing key. `tests/unit/locale-format.test.ts` scans `src/` (not the catalogues) for `toFixed`, `toLocale*`, a percent
sign glued to a number, a timestamp sliced to a date and an ISO date field put straight into markup or a placeholder;
`tests/unit/plurals.test.ts` holds the plural forms and scans `src/` for a plural form chosen by hand.

**Tables and chart labels (WW-1.16, 2026-10-31).** `DataTable` shows any plain column whose value is an ISO civil date
as the locale's date (sorting and export keep the ISO value; a column with its own `format`, `cell` or `type` is left
alone); the critical-path Gantt's and the Gantt view's axis labels, the network diagram's early start and finish, the
variance table's ranges (`formatDateRange`), the leveling table's moves and the calendar's week heading are
locale-formatted, and every "→" meaning "becomes" or "then" is `arrow()`, which points along the writing direction.

**Stated limits (WW-1.16).** Gregorian calendar only: no Hijri, Persian, Hebrew, Buddhist, Japanese-era or Indian
national calendar display, and no fiscal-year calendar in the interface (the service's financial year, WW-1.5, is not
shown as a calendar). Timestamps are shown in the reader's browser time zone; there is no time-zone setting in the
interface (the person's zone the service asks for in `/api/me/tasks` is not used for display).

Not covered yet (for the owner, WW-1): catalogue strings that put "%" after a number placeholder (30 in English and in
Welsh, counted so the number can only fall); placeholder numbers are not locale-formatted by the page catalogue's
`interpolate`; Welsh holds only one and other for each plural group (Welsh also has zero, two, few and many); dates
built inside a catalogue string or a column with its own `format` were not each re-checked.

### Locale / i18n

`LocalePicker` (`lily-design-system-svelte-locale-picker`) sets `lang`
and `dir` on `<html>` and switches the active translation catalogue, so
selecting a locale changes the displayed language. Supported locales
(10, `src/lib/locales.ts`): `ar-001`, `cy-001`, `de-001`, `de-de`, `en-001`,
`es-001`, `fr-001`, `hi-001`, `zh-001`, `zh-cn`; `ar-001` is **RTL**. The
locale is the first path segment (`/de-001/plans`). Strings come from
`src/lib/i18n.svelte.ts` (navigation, forms) and one page catalogue per
language (`i18n-pages.ts` English, `i18n-pages.<cy|ar|de|es|fr|hi|zh>.ts`;
`de-de` and `zh-cn` share their language's). **Every locale is complete
(WW-1.1, NFR-19):** each page catalogue is typed as a complete record of the
English keys, `tests/unit/catalogue-coverage.test.ts` checks keys,
placeholders and copied English for each, and `tests/e2e/every-locale.spec.ts`
opens every page route in every locale. A missing key would still fall back
to English at run time. English and Welsh are bundled; the other six page
catalogues are loaded only for their own locale (`loadLocale`, awaited by
the root layout's load), so a page does not carry every language. Every translation is a machine draft, recorded
unreviewed per locale in `governance/translation-reviews/` (Welsh in
`governance/welsh-translation-review.md`).

**Right-to-left (WW-1.15).** `ar-001` sets `dir="rtl"`. Stylesheets use logical properties only
(`margin-inline-start`, `border-inline-start`, `text-align: start`, …) so the layout mirrors;
`tests/unit/logical-css.test.ts` fails on a physical `left`/`right` margin, padding, border, `text-align`, `float` or
`clear` in any `src/` stylesheet. **Limits:** chart and timeline geometry (`style:left` percentages on Gantt,
critical-chain, capacity and pipeline bars, and the SVG charts) runs left to right in every locale; the font stack names
system faces that cover Arabic, Devanagari and Simplified Chinese after the platform's UI font, and no font is bundled
or downloaded, so a system without one shows the browser's fallback. Arabic is the only right-to-left locale.

**First day of the week (WW-1.4).** The calendars (`/calendar`, the working calendars through `CalendarMonth`) start
the week on the page's locale's day (Monday for the `-001` world locales and `de-de` and `zh-cn`, by CLDR), or on the day
the person chooses on the Display page (`weekStart`: Monday, Sunday or Saturday, kept with the other display
preferences on the device and the account). `src/lib/week-start.ts` holds the rule; the choice reaches Lily's
CalendarView as the locale's `-u-fw-` extension. **Limits:** a browser without `Intl.Locale` week information (Firefox
at the time of writing) uses Lily's region table and ignores the choice; week numbers, timesheet weeks, the critical-path
Gantt's weekly ticks and the API's week buckets stay ISO 8601 weeks (Monday first), the stated default; the weekday
editors of a working pattern list Monday first because that is the order of the stored seven-day mask.

**How a reader's language is resolved (WW-1.14),** in `normalizeLocale` and tested case by case: an exact supported
code; Traditional Chinese tags (`zh-Hant…`, `zh-TW`, `zh-HK`, `zh-MO`) read `zh-001`, Simplified, by a stated rule (there
is no Traditional catalogue: a **limit**); any other region reads its language's world locale (`fr-CA` reads `fr-001`,
`en-IN` `en-001`, `de-AT` `de-001`, never another region's variant; regional spelling is a limit); a language with no
catalogue is no match, so the reader's next `Accept-Language` preference is tried and then English.

**When a locale is added (WW-1.14, the criterion).** A language is added when someone will carry it to the same bar as
every other (§2.0, NFR-19): a complete page catalogue and `STRINGS` entry (the typed record and
`catalogue-coverage.test.ts`), the `lily-labels` pack or a recorded fallback for it, a review record in
`governance/translation-reviews/` saying the text is a machine draft until a speaker reviews it, its `dir`, and a pass
of `every-locale.spec.ts`. Which language comes next is chosen by reach where the product is used and by whether a
reviewer can be found, not by region; the candidates are the 47 in
`spec/locales-for-global-sharing-with-svelte/locales.tsv`. Today's ten include no African language and none of
Japanese, Korean, Portuguese, Indonesian, Bengali, Swahili, Turkish, Russian, Persian or Urdu: a stated gap, not a
choice against them.

## 6. Functional requirements

1. **List** active plans (`GET /api/plans`) in a **SVAR DataGrid** (name,
   pid columns) with a **SVAR FilterBar** doing a client-side, contains-
   match filter over the already-loaded rows by name.
   - **Not built** (§13): a search-on-submit box hitting
     `GET /api/plans/search?q=` — `PlanRepository.search()` wraps the
     endpoint and is unit-tested, but no route calls it, so name search
     stays client-side over the current page. There is also no
     recent-activity feed (`GET /api/plans/events/recent`) — no
     repository method, type, or UI exists for it.
2. **Create** (`POST /api/plans`), redirect to the new detail page. The
   `kind` label is chosen on the form (optional).
3. **Detail**: render the stored `Plan`; offer edit, delete,
   check-duplicates, and entry points to the project-management views
   (governance, board, schedule). **Not built** (§13): an audit timeline
   and a child-plan roll-up (those whose `parent_ref` equals this plan's
   pid) on this page. Merge is a **separate, standalone page**
   (`/plans/merge`, item 7 below), not a detail-page action.
4. **Edit** (`PUT`), redirect back to detail.
5. **Delete** (`DELETE`, soft), redirect to the plans list.
6. **Check-duplicates** posts the current record and lists matches
   (name, score, confidence) **across the whole collection**, excluding
   the record itself. Matching is **not** gated by `kind`; there is no
   `plan_type` / kind component. **Not built** (§13): a visual
   per-component **MatchBreakdown** (name / goals / code / owner org /
   parent / timeframe / keywords / relationships / tags) — there is no
   such type or component; the UI shows the raw score and confidence
   only.
7. **Merge** (`/plans/merge`, standalone page, landed 2026-08-03 as
   FE-1): survivor pid + duplicate pid + optional reason, an optional
   side-by-side preview (`GET /api/plans/{pid}` for each), a native
   `confirm()` before the destructive `POST /api/plans/merge` call
   (`{main_pid, duplicate_pid, reason?}`), and a recent-merge history
   table (`GET /api/plans/merges/recent`). Equal pids are guarded
   client-side (the service `422`s on a self-merge); errors render as
   `"<status>: <message>"`. Reachable from a **Merge** entry in the
   top-bar nav.
8. **Audit timeline** — **not built** (§13). No `AuditEntry` type, no
   `PlanRepository` method, no UI. `GET /api/plans/{pid}/audit` exists on
   the service but nothing in this front-end calls it yet.
9. **The plan form** (create/edit) edits: `name` (required),
   `alternate_names`, `code` (owner-scoped), `owner_org_id` (**org
   picker** into the organization entity), `owner_org_name`, `lead_ref`
   (**person / worker picker**), `parent_ref` (**plan picker**; optional
   for any plan), `kind` (**optional** select — Portfolio / Project /
   Product / Program / Practice / Process / Purpose / Pathway /
   Proposal, or none), `status`, a **goals editor** (title +
   description + status + target date rows), `start_date` / `target_date`,
   `keywords`, `tags`, `identifiers` (scheme + value rows), `same_as`,
   `in_language`, and `relationships` (kind + target rows). Comma-list
   fields split on submit; blanks null; empty repeatable rows dropped.
10. **Kanban task board** (`/plans/[pid]/board`): columns **Todo /
    InProgress / InReview / Done / Blocked**; cards show task title,
    assignee, estimate, due date. **Drag a card to a column = status
    change** (`PATCH /api/plans/{pid}/tasks/{tid}`). Create a task
    inline; a card's title links to the task's page, where it is edited
    and linked to other tasks (NC-1 slice 3, section below).
11. **Issues list** (`/plans/[pid]/issues`): table of issues with
    kind (Bug / Risk / Blocker / Question / Improvement), severity
    (Low / Med / High / Critical), status (Open / InProgress / Resolved /
    Closed), reporter, assignee; create / edit; filter by status /
    severity.
12. **Gantt / timeline** (`/plans/[pid]/timeline`): renders
    `GET /api/plans/{pid}/timeline` — goal milestones (target dates) +
    task date ranges (start / due) as Gantt-shaped rows over the plan
    timeframe.
13. **Burndown chart** (`/plans/[pid]/burndown`): renders
    `GET /api/plans/{pid}/burndown` — remaining estimate over time as a
    series.
14. **Goals panel** (`/plans/[pid]/goals`): list / create / edit /
    delete goals (title, description, status, target date). Goal titles
    feed the match `Goals` component (display-only note) via the
    `data.goals[]` bridge.
15. **Session / auth (BFF + httpOnly cookie)**: the top bar carries a
    session affordance. **Sign in** leads to this app's own `/signin` +
    `/verify` pages (not a redirect to a central authentication
    front-end) for the passwordless magic-link; on success the
    authentication-service sets the `__Host-mxi_session` httpOnly cookie.
    The browser holds only that cookie — **no token in JS, no
    `localStorage`, no URL-fragment handoff**. The SvelteKit **server**
    (BFF: `hooks.server.ts` + `src/lib/server/{auth,session,config}.ts`)
    holds the session and attaches a short-lived PASETO server-side when
    calling the portfolio service through the same-origin
    `/api/proxy/[...path]` route; the browser never calls the service
    directly. Mutating browser→BFF calls from another origin are refused
    (`Origin` / `Sec-Fetch-Site`, CQ-5; there is no CSRF token); **Sign out**
    revokes the session. This lets operator traffic through once the
    service turns on blanket enforcement
    (`PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH`, on by default since HD-3). Per
    `../../../agents/share/authentication-sessions.md`.
16. **Layout shell**: global navigation is a full-width **top bar**
    (header) with a **leftmost hamburger** toggle — NOT a left sidebar —
    the main content area is **full-width**, the nav has a single **Plans**
    destination (no collection switcher), and the chrome area carries
    the theme + locale selectors.

## 7. Non-functional requirements

- **Svelte 5 runes only** (`$state` / `$derived` / `$effect` / `$props`
  / `$bindable`); no `export let`, no `$:`, events are callback props.
- **SvelteKit 3**, SPA (`ssr = false` / `prerender = false`).
- **TypeScript strict** with `noUncheckedIndexedAccess`; no `any`
  without a justifying comment.
- **Lily DataGrid** (`@lilydesignsystem/svelte-data-grid`, wrapped once
  as `DataTable`) for the plans list and any tabular sub-view; native
  HTML for simple lists. Every `DataTable` can be exported (see
  "Exporting a table").
- **Lily Design System Svelte Headless** for accessibility primitives
  (focus trap, listbox, combobox, dialog) and the theme / locale
  selectors; native HTML elsewhere.
- **No global stores** for HTTP state — construct a `PlanRepository`
  (all paths under `/api/plans`) per page/component.
- Drift accepted: own copy of API client / types / form primitives; no
  shared package (repo decision 2026-06-02).

## 8. Architecture

`ApiClient` (lean, raw-JSON, get/post/put/patch/delete + `ApiError`) →
`PlanRepository` (all paths under `/api/plans`) → routes. The
service is Axum and returns **raw JSON** (no envelope). Under the BFF
model (§6.15) the browser carries only the `__Host-mxi_session` cookie;
`ApiClient`'s base URL is the same-origin `/api/proxy`, and the
SvelteKit server (`src/routes/api/proxy/[...path]`) attaches the
short-lived PASETO server-side when forwarding to the service — no token
is read or attached in browser JS. On the way back it relays the status, the
body and an allow-list of response headers (`src/lib/server/proxy-headers.ts`,
CQ-4): `content-type`, `content-disposition`, `cache-control`, `etag`,
`last-modified`, `location`, `retry-after`, `link` (which carries a list's
next page), `x-handling-mark` and the
`ratelimit-*` trio; never `set-cookie` or a hop-by-hop header. Before
forwarding it refuses (403) a state-changing request whose `Origin` is not the
app's, or whose `Sec-Fetch-Site` names another site, drops a browser-supplied
`Authorization`, and answers 502 `bad_gateway` when the service cannot be
reached (`src/lib/server/proxy-guard.ts`, CQ-5).

On any page under `/plans/[pid]` the layout shows the plan's **handling mark**
(RG-7) when the deployment has a scheme, and nothing when it has none or when
the plan is not there for the reader (404, 403: a plan above the reader's
clearance is 404). Any other failed lookup shows "Handling mark: could not be
checked", never no banner, because no banner reads as unmarked (CQ-19). `PlanForm` builds a `Plan` from the
inputs (comma lists split, blanks nulled, goals / identifiers /
relationships as editable rows; empty rows dropped on submit; a seeded
`Custom` enum collapses to "—"; `kind` is an optional select,
`owner_org_id` / `lead_ref` / `parent_ref` are raw pid text inputs, not
typeahead pickers — §16 open question). The project-management views
(board, and the oversight/executive dashboard routes in §5) call their
sub-resource / dashboard endpoints via a separate `PpmClient`
(`src/lib/api/ppm.ts`), not `PlanRepository`. Layout chrome (Plans
destination, theme / locale selectors, hamburger nav, session affordance)
lives in `+layout.svelte`; `+layout.ts` sets the SPA toggles.

`types.ts` hand-mirrors `project_portfolio_management_matcher::Plan` and
the identity-surface DTO shapes actually in use: `Plan`, `PlanKind`,
`PlanStatus`, `Goal`, `GoalStatus`, `RelationKind`, `PlanRelationship`,
`PlanIdentifier`, `IdentifierScheme`, `PlanRef`, `ScoredRef`,
`MergeRequest`, `MergeResponse`, `MergeRecordRow`. It does **not** carry
`MatchBreakdown`, `AuditEntry`, `PlanEvent`, `TimelineRow`, or
`BurndownPoint` — those capabilities are not built (§13); `Task` /
`TaskStatus` / `Issue*` types live in `ppm.ts` instead, since the board
route uses `PpmClient`, not `PlanRepository`. Keep `types.ts` in lockstep
with any matcher-type change (entity spec §18).

## 9. API consumption

Every plan lives under `/api/plans` (one recursive collection). Rows
below are endpoints an actual route calls today.

| Route / action | Endpoint |
|---|---|
| list | `GET /api/plans` |
| create | `POST /api/plans` |
| detail load | `GET /api/plans/{pid}` |
| delete | `DELETE /api/plans/{pid}` |
| duplicates | `POST /api/plans/check-duplicates` (→ `ScoredRef[]`, score + confidence only) |
| merge (`/plans/merge`) | `POST /api/plans/merge` (`{main_pid, duplicate_pid, reason?}`) |
| merge history (`/plans/merge`) | `GET /api/plans/merges/recent` |
| edit | `PUT /api/plans/{pid}` |
| schedule | `GET /api/plans/{pid}/schedule` |
| board: list / move | `GET /api/plans/{pid}/tasks` · `PATCH /api/plans/{pid}/tasks/{tid}` (status) |
| board: create / edit | `POST /api/plans/{pid}/tasks` · `PUT /api/plans/{pid}/tasks/{tid}` |
| board: sprints / burndown | `GET / POST /api/plans/{pid}/sprints` · `GET /api/plans/{pid}/burndown?sprint=` |

**Not wired to any route** (no repository method, or a method exists but
nothing calls it — §13): search (`GET /api/plans/search?q=` — the
`PlanRepository.search()` method exists and is unit-tested; the list
page filters client-side instead), recent activity
(`GET /api/plans/events/recent`), a per-plan audit timeline
(`GET /api/plans/{pid}/audit`), and the child roll-up
(`GET /api/plans?parent={pid}`, exposed as `listPage()`'s `parent`
scope). The issues register has a screen (`/issues`, `/plans/[pid]/issues`) over the service's issues routes;
the goals sub-resource route exists in the service with no screen of its own; there is no `timeline` route.

(Sub-resource verbs/paths track the service spec §9; pin them in tests
when the controllers land.)

## 10. Persistence

None client-side beyond in-memory route state and two app-local
`localStorage` keys for chrome preferences (`portfolio:theme` /
`portfolio:locale`). The session lives only in the `__Host-mxi_session`
httpOnly cookie — no auth token in `localStorage` (BFF model, §6.15).

## 11. Testing strategy

`pnpm run check` (svelte-check strict, 0/0). **vitest** unit tests
(`tests/unit/`; the current count is the generated block in the entity spec's
[§14](../../spec/14-implementation-status.md), 76 files and 650 tests at 2026-10-30) cover, among others
(the list below is the first eight files, kept as a description, not as a census):

- `client.test.ts` — the `ApiClient` (verb / body / headers /
  cookie-credentials / error-classification / empty-body, incl.
  `PATCH`);
- `plans.test.ts` (12 tests) — every `PlanRepository` method's path +
  verb: `list()` (incl. the `?parent=` roll-up query, though no route
  passes it today), `search()` `/search?q=` URL-encoding,
  `checkDuplicates()`, `merge()` body `{main_pid, duplicate_pid,
  reason?}`, `recentMerges()`. `PlanRepository` has **no** `audit()` /
  `recentEvents()` / goals / timeline method (the issues and burndown
  calls are in `PpmClient`, `src/lib/api/ppm.ts`, tested in `ppm.test.ts`);
- `merge-validation.test.ts` (4 tests) — the pure pre-merge guard (both
  pids required, must differ);
- `capabilities.test.ts` / `ppm.test.ts` — the `CapabilityClient` /
  `PpmClient` endpoint paths used by the oversight/executive dashboard
  routes and the board;
- `plan-form.test.ts` (14 tests) — required-name guard blocks
  `onsubmit` on blank/whitespace; `build()` trims scalars, nulls blanks,
  splits comma lists, drops empty goal / identifier / relationship rows,
  collapses a `Custom` enum seed, leaves `kind` optional;
- `i18n.test.ts` (10 tests) — every key present in `en`; fallback to
  `en` for a missing key; RTL flag for `ar` / `ur`; merge-key coverage;
- `layout.test.ts` (1 test) — the hamburger toggles the nav.

Component tests run via `@testing-library/svelte` mounted client-side
by the `svelteTesting()` vite plugin.

**Playwright** tests (the count is in the same generated block; 67 spec files at 2026-10-30), with the API
stubbed via `page.route`, run against the production build
(`vite preview`) to avoid the `vite dev` cold-start module race:

- `tests/e2e/smoke.spec.ts` (7 tests) — the identity surface: the
  list / new / detail / edit pages render; the merge page renders its
  form and the recent-merges table; the nav exposes the merge link; and
  check-duplicates self-exclusion (the record does not list itself as a
  candidate).
- `tests/e2e/ppm.spec.ts` (16 tests) — the oversight/executive dashboard
  views (dashboard, proposals, ideas, executive, financials, technology,
  scenarios, board, auditor, compliance, risk, security, regulator,
  engineering, calendar) and the per-plan Kanban task board (columns,
  burndown, standup digest, sprint notes, velocity).

There is **no** Playwright coverage yet for an audit timeline,
recent-activity panel, child-plan roll-up, issues, timeline, or a
plan-level (non-sprint) burndown — those views are not built.

Both stub files match request paths against the service's bare
`/api/...` contract; since the request actually lands on the BFF proxy
at `/api/proxy/api/...` (`ad95088e`), both strip the `/api/proxy` prefix
before dispatching. (Fixed 2026-08-04, DOC-4 audit: both files also
still navigated to the pre-2026-07-20 `/projects*` routes, which do not
exist — neither `pnpm run check` nor `pnpm test` nor `pnpm run build`
runs Playwright, so 21 of 23 tests had been silently broken since the
`/plans` unification; see CHANGELOG.)

Run: `pnpm test` (vitest) and `pnpm test:e2e` (Playwright).

## 12. Compliance

Plans are business / portfolio artefacts; defer to the service's
controls for access / audit. Lead / assignee / author refs are opaque
`EntityRef`s into person / worker — the front-end displays them but does
not resolve PII beyond what the referenced services return. Cross-service
links are **never** a match signal (entity spec §1).

## 13. Tasks (live work queue)

> This is the build queue for the implemented app (MVP shipped); check off
> in three-part PRs (spec + code + test).

- [x] **2026-10-29 — NC-9: the portfolio balance page (`/balance`).** A bubble
  chart with one bubble per portfolio: across is planned cost, up is target
  benefit value, the bubble's area is open risk exposure; every axis starts at
  zero. Beside it, the imbalances in words, and the same figures as a table
  (`DataTable`), always on the page, which is the chart's text alternative.
  **Data (existing reads only, no service change):** cost from
  `GET /api/financials/variance` (`by_portfolio`, `planned_minor`), value from
  `GET /api/executive/benefits` (`target_minor`), risk from
  `GET /api/risk/heatmap` (`posture[].open_exposure`), joined on the
  portfolio's `pid` (plans under no portfolio form one "No portfolio" group).
  Money is shown in one currency at a time, chosen from the currencies the cost
  and value reads name, and is never converted or added across currencies; risk
  exposure is the service's score and has no currency. **Flags** (pure, in
  `src/lib/balance-view.ts`, thresholds written there and on the page): a
  portfolio holds at least 50% of the total planned cost, or of the total open
  risk (only with two or more groups); its share of cost, or of risk, is at
  least 20 percentage points above its share of value; or it has planned cost
  and no target value in the chosen currency. Shares are compared to the nearest
  tenth of a percent. A flag is a prompt to look, not a verdict. A flagged
  bubble has a heavy dashed outline and a "!" mark, and is named in the list;
  colour is never the only signal. Load failures show the catalogue's sentence
  (`resource()`), never the service's text. Strings are `p.balance.*`; the Welsh
  is machine-written and listed in `governance/welsh-translation-review.csv`.
  **Not built:** grouping by category or strategic aim (the service reports
  cost, value and risk together only by portfolio; budget lines carry a
  category and objectives carry a strategic aim, but no read gives all three
  per category or aim); risk and value in the same unit as cost; drill-through
  from a bubble to the portfolio. Tests: `tests/unit/balance-view.test.ts` (the
  flags and their thresholds, the join, the layout) and
  `tests/e2e/balance.spec.ts` (the chart, the table, currency switching, the
  empty and failed states, Welsh, axe).

- [x] **2026-10-02 — LT-3: the "My work" page (`/me`).** The signed-in
  person's open tasks in five due-date buckets (overdue / today / this week /
  later / no due date), computed **in the browser's own IANA zone**, sent
  explicitly as `?tz=` — the service never guesses a zone and refuses fixed
  offsets (service spec §7.x). `PpmClient.myTasks(tz, asOf?)`, the `MyWork`
  types, a nav entry, and `/me` in the mobile-audit route list. A deployment
  makes it a persona's landing page with `view=me` (T-28f); the default landing
  page is unchanged. English-first. **Not built:** a one-line quick-add (needs
  the caller's own `person:` URN in the browser, which the BFF does not expose).
  **Not run:** the Playwright mobile audit (no browser in this environment).
  Tests: `tests/unit/ppm.test.ts` (client path).

- [x] **2026-10-03 — RM-7: module-aware plan links.** `/plans/[pid]` hides the
  Governance / Board / Schedule / Flow links whose feature module is off for
  that plan, via the pure `src/lib/modules.ts` (`visiblePlanLinks`) over
  `PpmClient.planModules(pid)`. **Fails open**: an unknown module list (still
  loading, or the call failed) shows every link — hiding a feature because of a
  transient error would be worse than a link the service then refuses with 404.
  The sub-routes stay reachable by URL (the service answers `404
  module_disabled`). Tests: `tests/unit/modules.test.ts`.

- [ ] **FE-2 (M) Masked view + GDPR export UI for `/plans/[pid]`.** The
  service has carried `GET /api/plans/{pid}/masked` and
  `GET /api/plans/{pid}/export` since 2026-08-02 (this crate's own
  `AGENTS.md`/§9 "API consumption" table lists both under "Plan CRUD"),
  but `PlanRepository` has no `masked()`/`export()` method and no route
  calls either path — *(verified: `grep -n "masked\|export(" src/lib/api/plans.ts`
  returns nothing, and `AGENTS.md`'s own repository-method list —
  "list + listPage + search + get + create + update + remove +
  checkDuplicates + merge + recentMerges" — has no masked/export
  entry)*. Add `PlanRepository.masked(pid)` / `.export(pid)`, and a
  "View masked" / "Export (GDPR)" affordance on `/plans/[pid]` (a plain
  link/button rendering the JSON is enough to make the capability
  reachable; a dedicated view is a stretch goal).
  Three-part change: spec §9/§13 + `src/lib/api/plans.ts` +
  `src/routes/plans/[pid]/+page.svelte` + vitest coverage.
  **Acceptance:** new vitest cases pin the two new repository methods'
  URLs; a Playwright smoke test exercises the affordance from
  `/plans/[pid]`.

- [ ] **FE-3 (M) Decide and, if adopted, implement the `requireSignedIn`
  page-visit guard (PRO-H10).** Repo `tasks.md` WEB-1 found the guard
  rolled to 5 of 16 front-ends and explicitly left "the 5/11 roll-out
  question... still open" for the rest — *(verified:
  `grep -rn "requireSignedIn" src/` in this project returns zero
  matches, matching WEB-1's finding, which names "place, organization,
  care-pathway, case, portfolio and all five consumer apps" as
  unguarded)*. This project's auth model (BFF + httpOnly session
  cookie, §8) is structurally identical to the five front-ends that do
  have the guard on `/new`/`/merge`-style mutating routes. Either add
  it (following person's `requireSignedIn(locals)` pattern plus WEB-1's
  `SMOKE_STORAGE_STATE` Playwright stub-cookie fix so the smoke suite
  still renders guarded pages) to `/plans/new`, `/plans/merge`, and
  `/plans/[pid]/edit`, or record in this spec (§8/§13) why portfolio
  deliberately opts out.
  **Acceptance:** either `requireSignedIn` gates the mutating routes
  with a pinned anonymous-303 Playwright test (mirroring person's), or
  spec §8 states the explicit reason portfolio stays unguarded.

- [ ] **FE-4 (M) Consolidate ETag/conditional-GET handling for the
  oversight and executive dashboard views.** The service documents
  every `at-a-glance`/`executive/*`/`board/*`/`auditor/*`/
  `compliance/*`/`financials/*`/`technology/*`/`risk/*`/`security/*`/
  `regulator/*`/`scenarios/compare` read as "ETag-conditional" with an
  `as_of` freshness watermark (service `AGENTS.md`), but the front-end
  handles this on only two of the ~20 route directories that consume
  such views — *(verified: `grep -rln "ETag\|If-None-Match\|304"
  src/routes/ src/lib/` matches only `src/routes/dashboard/+page.svelte`
  and `src/routes/executive/+page.svelte`; `src/lib/api/client.ts` and
  `src/lib/api/ppm.ts` carry no shared `ETag`/`If-None-Match` handling
  at all, so `auditor/`, `board/`, `compliance/`, `financials/`,
  `technology/`, `risk/`, `security/`, `regulator/`, `scenarios/` each
  either reinvent it or, more likely given the grep, never send
  `If-None-Match` and always pay the full payload)*. Add one shared
  helper in `client.ts` (store the last `ETag` per URL, send
  `If-None-Match`, treat `304` as "reuse the cached body") and wire the
  remaining oversight/executive routes to it.
  Three-part change: spec §9/§13 + `src/lib/api/client.ts` + the
  consuming routes + vitest coverage of the 304-reuse path.
  **Acceptance:** a vitest case proves the shared helper reuses the
  cached body on a stubbed `304`; at least the `board`/`auditor`/
  `financials`/`technology` routes are wired to it.

- [x] **2026-08-05 — Wire `/plans` itself to `listPage()`.** The sibling
  gap the entry directly below flagged and deliberately left open:
  `PlanRepository.listPage()` existed and was tested at the repository
  level, but `src/routes/plans/+page.svelte` still called the
  unpaginated `list()`. Now it calls `listPage()` and shows the same
  `shown / total` count (hidden when the list is empty) as
  `/organizations`, `/automations`, and `/reviews`. No `?parent=`
  roll-up wiring added here — the list route never took a parent scope
  before this change either, so that stays out of scope (only
  `/plans/[pid]`'s detail page passes a `parentRef`, and not to this
  route). Tests: 3 new cases in `tests/unit/plans.test.ts` pinning
  `listPage()`'s URL, header parsing, and the header-absent fallback.
  No new Playwright coverage — the existing `list page renders the
  seeded plan` smoke test already exercises the paginated path (its
  stub has no pagination headers, so `total` falls back to the page
  length, and the test's assertions were unaffected). svelte-check 0/0,
  68 vitest pass (65 → 68), build green, 23/23 Playwright pass.

- [x] **2026-08-05 — Pagination on the four consuming capability-view
  lists (repo tasks.md PG-1's last sub-bullet).** The service's
  `automations` / `automations/runs` / `scheduled-actions` / `reviews`
  endpoints gained `?limit=&offset=` + `X-Total-Count`/`X-Limit`/
  `X-Offset` (service spec §9.1/§9.4a); this app wires the two routes
  that consume them:
  - `src/lib/api/capabilities.ts` — five new `*Page` methods
    (`listAutomationsPage`, `runsPage`, `listScheduledPage`,
    `listReviewsPage`, `inboxPage`), each a thin `ApiClient.getPage()`
    wrapper alongside its existing unpaginated sibling (same pattern as
    `PlanRepository.listPage()` next to `list()`).
  - `/automations` (`src/routes/automations/+page.svelte`) — all three
    of its lists (rules, the deadline queue, the run log) now call the
    `*Page` methods and show a `shown / total` count above each table
    (hidden when the list is empty; matches the organization service's
    front-end `/organizations` list-route convention).
  - `/reviews` (`src/routes/reviews/+page.svelte`) — the delegation
    list gets the same treatment.
  - **`/api/notifications` stays unconsumed by any route** (documented
    gap since the 2026-07-22 capability-views entry below); `inboxPage()`
    exists for API-contract parity with the other four but nothing
    calls it. No notifications inbox page was built here — that would
    be new UI scope, not a mechanical pagination follow-up.
  - **No pager controls (Prev/Next) were added.** The family convention
    (`agents/share/restful.md`) is deliberately headers-plus-count, not
    a cursor UI yet; `/organizations` and `/plans` show the same
    restraint. A caller past the default page still reaches deeper rows
    via `?limit=&offset=` directly.
  - Tests: 3 new cases in `tests/unit/capabilities.test.ts` pinning the
    five new methods' URLs and `Page<T>` parsing (reusing
    `tests/unit/client.test.ts`'s `getPage` header-stub pattern). No
    Playwright coverage added — `/plans`'s own `listPage()` had none
    either at the time (it was not yet wired into
    `/plans/+page.svelte`, a pre-existing gap this task did not expand
    its scope to close), so there was no established e2e depth to
    match. **Closed by the entry directly above** (same day).
  - *Verified:* svelte-check 0 errors/0 warnings; 65 vitest pass
    (62 → 65); `pnpm run build` green.

- [x] **2026-08-03 — FE-1: the record-merge page (`/plans/merge`).**
  A standalone operator page for folding a confirmed-duplicate plan into
  a survivor: survivor pid + duplicate pid + optional reason, an optional
  side-by-side preview (`GET /api/plans/{pid}` for each), a native
  `confirm()` before the destructive call, and a recent-merge history
  table read from `GET /api/plans/merges/recent`. New: `MergeRequest` /
  `MergeResponse` / `MergeRecordRow` in `src/lib/api/types.ts`;
  `PlanRepository.merge(request)` (signature changed from positional args
  to the request object, matching the wire body) and
  `PlanRepository.recentMerges()`; the pure guard
  `src/lib/components/merge-validation.ts` (both pids required, must
  differ — the service answers `422` on a self-merge); a `nav.merge` entry
  in the top-bar nav. **Wire shape note:** this service's merge response
  is `{main_pid, duplicate_pid, main}` with **no** `merge_record` wrapper
  (unlike the person service), so the page shows the survivor's pid and
  reads the history row back separately. Errors are rendered as
  `"<status>: <message>"` — this crate's `ApiError` carries no error
  code. 24 i18n keys added across all 13 locales. Tests:
  `tests/unit/merge-validation.test.ts` (4 cases), repository
  merge/recentMerges path pins in `tests/unit/plans.test.ts`, a merge-key
  coverage + placeholder test in `tests/unit/i18n.test.ts`, and two
  Playwright smoke pins. svelte-check 0/0, 62 vitest pass, build green.

- [x] **2026-07-22 — Capability views (service spec §9.4a).**
  `CapabilityClient` + wire types in `src/lib/api/capabilities.ts`, and
  four pages: `/prioritization` (Smart Score queue + per-component
  explanation), `/lifecycle` (the phase funnel), `/reviews`
  (delegation, verdicts, consensus), `/automations` (rules, the
  deadline queue + manual sweep, and the run log). Nav entries added;
  vitest pins every client path. English-first (locale catalogues not
  extended yet); no standalone notifications page.

- [x] **2026-07-22 — Offer the five new `kind` labels.** `PlanKind` /
  `ALL_KINDS` in `src/lib/api/types.ts` gain `Practice`, `Process`,
  `Purpose`, `Pathway`, `Proposal`, so the `PlanForm` kind `<select>`
  lists them. `COLLECTIONS` gains the matching plural segments except
  `proposals`, which already names the proposals catalogue route.

- [x] **Unify the four collections into one recursive `/api/plans`
  collection + rename the entity to `plan` (2026-07-20).** Collapsed the
  dynamic `[collection]` route directory to a static `plans/` directory
  (`/plans`, `/plans/new`, `/plans/[pid]`,
  `/plans/[pid]/{edit,board,schedule,governance}`); removed the collection
  switcher (one **Plans** destination). `WorkItemForm` → `PlanForm`
  (`kind` now an **optional** select); `src/lib/api/work-items.ts` →
  `src/lib/api/plans.ts` exporting `PlanRepository` (not collection-bound;
  all paths under `/api/plans`, `?parent=` roll-up,
  `/api/plans/{pid}/schedule`); `types.ts` renamed `WorkItem` → `Plan`,
  `kind` optional, `portfolio_ref` → `parent_ref`. Tests renamed to
  `tests/unit/plans.test.ts` + `tests/unit/plan-form.test.ts`. `kind` is
  an optional descriptive label; matching is **not** gated by kind.
  svelte-check 0/0, 45 vitest pass.
- [x] Scaffold the SvelteKit 2 / Svelte 5 (runes) SPA: `package.json`,
  `svelte.config.js`, `vite.config.ts`, `tsconfig` (strict +
  `noUncheckedIndexedAccess`), `src/app.html`, `src/app.css`. The
  original scaffold's `.env.example` (`PUBLIC_API_BASE_URL`,
  `VITE_AUTH_FRONTEND_URL`) named the client-held-token model's vars;
  it was superseded when the BFF (own `/signin`+`/verify`+`/api/proxy`)
  landed and now documents `PROJECT_PORTFOLIO_MANAGEMENT_API_URL` /
  `AUTH_API_URL`, read server-side only (fixed 2026-08-04, DOC-4 audit —
  the vars had drifted out of every doc, not just this one).
- [x] `src/lib/api/types.ts` — mirror `project_portfolio_management_matcher::Plan` +
  `PlanKind` (optional) + the identity-surface DTO shapes (per §8; not
  the sub-resource `Task`/`Issue` shapes, which live in `ppm.ts`).
- [x] `src/lib/api/client.ts` — lean fetch wrapper (get/post/put/patch/
  delete) + `ApiError`. Browser→BFF calls only; the SvelteKit server
  attaches the PASETO bearer server-side (no client-held token).
- [x] `src/lib/api/plans.ts` — `PlanRepository` (list + listPage +
  search + get + create + update + remove + checkDuplicates + merge +
  recentMerges; all paths under `/api/plans`). **Not built**: `audit()`,
  `recentEvents()`, or any issues/goals/timeline/burndown method — see
  §9.
- [x] Auth — adopt BFF + httpOnly cookie + CSRF: `hooks.server.ts` +
  `src/lib/server/{auth,session,config}.ts` read the
  `__Host-mxi_session` cookie, exchange it for a short-lived PASETO
  server-side, and attach it when calling the portfolio service via
  `src/routes/api/proxy/[...path]`; cross-site mutating browser→BFF calls
  refused by an `Origin` / `Sec-Fetch-Site` check (CQ-5, 2026-10-21; the CSRF
  token this line once claimed was never built).
  Sign-in is this app's own `signin/` + `verify/` routes, not a redirect
  to a central authentication front-end. No `mxi_access_token` /
  `localStorage` bearer, no fragment handoff (per
  `../../../agents/share/authentication-sessions.md`).
- [x] `src/lib/config.ts` — `API_BASE_URL` (same-origin `/api/proxy`,
  hardcoded, not env-driven). There is no client-side `signInUrl()`; the
  Sign in link is a plain `href="/signin"`.
- [x] `src/lib/i18n.svelte.ts` — 13-locale catalogues (en, cy, es, fr, de, ar,
  ru, hi, zh, bn, pt, id, ur) + RTL flags + `en` fallback (one file, not
  a directory).
- [x] `+layout.svelte` / `+layout.ts` — top-bar nav with **leftmost
  hamburger**, full-width content, Plans destination + theme + locale
  selectors, session affordance (Sign in / Sign out — BFF, no token
  paste); SPA toggles.
- [x] Plan identity routes: `/plans` (SVAR DataGrid list + client-side
  name filter), `/plans/new`, `/plans/[pid]` (detail + delete +
  check-duplicates, plain score/confidence), `/plans/[pid]/edit`,
  `/plans/merge` (FE-1, above). **Not built** on any of these: a
  server-round-trip search box, recent activity, a MatchBreakdown
  visual, a per-plan audit timeline, and the child-plan roll-up — see §9.
- [x] `PlanForm` component (goals / identifiers / relationships editors;
  `kind` optional select). `owner_org_id` / `lead_ref` / `parent_ref`
  are plain pid text inputs, not typeahead pickers (§16 open question,
  still open).
- [ ] `MatchBreakdown` visual component (per-component score bars incl.
  goals / parent / relationships / tags; no `plan_type` / kind gate).
  **Not built** — no type, no component; `check-duplicates` renders raw
  score + confidence.
- [x] Kanban task board (`/plans/[pid]/board`) — drag = status PATCH.
  Uses `PpmClient`/`Task` (`ppm.ts`), not `PlanRepository`.
- [ ] Issues list (`/plans/[pid]/issues`).
- [ ] Gantt / timeline view (`/plans/[pid]/timeline`).
- [ ] Burndown chart (`/plans/[pid]/burndown`) — the board's per-sprint
  burndown (`GET /api/plans/{pid}/burndown?sprint=`) is not this: it is
  scoped to one sprint, not the plan-level "remaining estimate over
  time" series this item describes.
- [ ] Goals panel (`/plans/[pid]/goals`).
- [x] vitest unit suite, 8 files / 62 tests: `client`, `plans`
  (`PlanRepository`), `merge-validation`, `capabilities`, `ppm`,
  `plan-form`, `i18n`, `layout` (hamburger-toggle render test). See §11.
- [x] Playwright e2e smoke, 25 tests across `tests/e2e/{smoke,ppm}.spec.ts`:
  the plan identity routes render, the merge page + recent-merges table,
  the nav's merge link, check-duplicates self-exclusion, every
  oversight/executive dashboard view, and the Kanban board (columns,
  burndown, standup, sprint notes, velocity). **Not covered**: recent
  activity, audit timeline, child-plan roll-up, issues, plan-level
  timeline/burndown, or a layout hamburger/theme/locale/RTL switch test
  — see §11. Fixed 2026-08-04 (DOC-4 audit): both spec files still
  navigated to pre-unification `/projects*` routes and neither stripped
  the BFF proxy prefix, so 21/23 tests had been failing since
  2026-07-20/`ad95088e` respectively, undetected because no other
  command in this project's test pyramid runs Playwright.
- [x] **Fix: `/calendar` never actually rendered a milestone event
  (2026-09-06).** `@svar-ui/calendar-store` requires an all-day event's
  `end` to be strictly *after* `start`; `+page.svelte` passed
  `end: day` — the same `Date` object as `start` — so the SVAR Calendar
  widget silently dropped every milestone it was ever asked to show.
  The existing `"delivery calendar lists milestones with kinds"` test
  never caught this because it asserted only the fallback `<ul>`
  (`milestone-list`), a separate render path from the calendar widget
  itself. The identical bug and fix already landed in worker-front-end
  and person-front-end's `/expiry` calendars (repo `tasks.md`). Fixed
  by computing the following calendar day as `end`; the test now also
  asserts the calendar widget (`milestone-calendar`) itself renders the
  event text, and the stubbed milestone's due date is computed relative
  to the actual test-run date (a fixed date would eventually scroll
  outside the widget's default "today's month" view). Verified the
  strengthened assertion actually fails without the fix before
  confirming it passes with it.

- [x] **FE-5: `/verify` crashed with a raw 500 when the authentication service was unreachable.** *(resolved 2026-09-06.)* `src/routes/verify/+page.server.ts` called `await verifyMagicLink(fetch, token)` with no `try`/`catch`. A network-level failure (the authentication service unreachable, timed out, connection reset) makes `fetch` throw rather than resolve — uncaught, that propagated out of `load` and SvelteKit rendered its generic 500 error page instead of this route's own friendly UI. The same bug class was found and fixed first in `place-front-end-with-svelte` (T-26) and `thing-front-end-with-svelte` (T-23); ported here.
  - **Resolved.** A `try`/`catch` around the call, a new `"serviceUnavailable"` error variant, and its message in `+page.svelte`.
  - **Acceptance:** `tests/unit/verify.test.ts` (new) unit-tests the `load` function directly — pinning `missingToken`, the new `serviceUnavailable` (fetch rejects), and `invalidToken` (non-ok response) branches — verified to fail with the `try`/`catch` reverted and pass with it restored. Three-part change: spec (here) + code + test.

- [x] **T-28p (S) Operator onboarding guide** *(landed 2026-09-18)* — a
  role-by-role "first hour" walkthrough (executive, PMO, resource
  manager) at the new `/onboarding` route, each step naming the page it
  lands on. Tracked at the family level in the service crate's own
  `spec/13-tasks.md` T-28p (the "no onboarding path beyond the README
  quick start" gap that checklist identified); implemented here since
  the guide is front-end content, not an API surface.
  - **Built.** `src/lib/onboarding.ts` — pure data, `ONBOARDING_ROLES`
    (three roles, 6–8 steps each, every step naming a real route);
    `src/routes/onboarding/+page.svelte` — a static page rendering it,
    no API call, so nothing here can fail to load; a plain-English
    `/onboarding` entry added to `navItems` in `+layout.svelte`
    (unlocalized, matching the existing `/prioritization` and
    `/lifecycle` nav entries' precedent — not every nav item goes
    through the 13-locale i18n catalogue).
  - **Deliberately makes no time-to-productivity claim** — nobody has
    measured how long a first hour with the guide actually takes, and
    a guide is the same category of derived-looking-but-invented
    number the family refuses to produce elsewhere from nothing.
  - **Acceptance:** `tests/unit/onboarding.test.ts` (new, 4 tests) —
    walks every role's every step and asserts (via `fs.existsSync`)
    that its named route resolves to a real `+page.svelte` in
    `src/routes`, rather than trusting the data file to stay in sync
    with the route tree by hand; also pins that all three required
    roles are present and that no step/summary text reads like a
    timing claim (a `/\d+\s*(minute|hour)s?\b/i` regex guard). Three-part
    change: spec (here) + code + test. Verified: `svelte-check` 0
    errors / 0 warnings, `vitest run` 83/83 (was 79), `pnpm run build`
    green (the new route appears in the build output), `prettier
    --check` clean.

- [x] **T-28f (M) Role-tailored navigation and landing page** *(landed
      2026-09-19)* — tracked at the family level in the service crate's
      own `spec/13-tasks.md` T-28f; implemented here since it is
      front-end presentation, not an API surface.
      **The task's own premise was factually wrong — corrected, not
      guessed around.** T-28f's text assumed "the front-end reads the
      `attrs` the BFF already gets from `/whoami`." Verified false by
      reading `authentication-service-with-loco/src/views/auth.rs`
      (`GET /api/auth/me`'s `CurrentResponse` carried no `attrs` field
      at all) and grepping every front-end's BFF (`grep -rln "attrs"
      */*-front-end-with-svelte/src/lib/server/*.ts` — no hits anywhere
      in the family). No front-end BFF decodes the PASETO token itself
      either (that would mean hand-rolling a second verifier in
      TypeScript — exactly what `authentication-verifier` exists to
      avoid), so `attrs` was never reachable client-side at all. Fixed
      at the source: `authentication-service-with-loco`'s
      `CurrentResponse` now carries `attrs` (the **live**
      `users.attributes`, not the PASETO claim's up-to-5-minute-stale
      snapshot) — its own `spec/index.md` §13 T-17 carries the full
      account.
      **Built.** `src/lib/server/auth.ts` gained `currentUser()` (session
      → bearer → `GET /api/auth/me` → `{pid, name, email, attrs}`).
      `src/lib/nav.ts` (new, pure, DB-free): `viewAttr(attrs)` reads the
      deployment-declared `view` attribute's first value (a vocabulary
      the deployment declares, **not** an enum this code owns — an
      unrecognized value is simply ignored, never an error);
      `orderNavForView(items, view)` moves the nav item whose `href` is
      `/${view}` to right after the brand/home link, leaving everything
      else in original order, and is a no-op when `view` is
      absent/unmatched (**default is today's full nav**, verified
      byte-for-byte via the unchanged-array test); `landingRouteForView`
      picks `/${view}` over the family-wide `/plans` default when
      `view` matches `KNOWN_NAV_HREFS` (a small, explicitly-documented,
      hand-kept-in-sync copy of `+layout.svelte`'s nav hrefs — needed
      because `+layout.server.ts` resolves the landing route
      **server-side**, before `navItems`'s i18n-dependent, client-side
      array exists).
      `+layout.server.ts` resolves `view`/`landingRoute` alongside the
      existing `signedIn` (signed-out ⇒ `view: null`, unchanged nav, no
      extra fetch cost beyond what a session already implies).
      `+layout.svelte` derives `orderedNavItems = $derived(orderNavForView(navItems,
      data.view))` and iterates that instead of `navItems` directly —
      `navItems` itself, and every existing label/i18n lookup inside
      it, is completely untouched. Root `+page.svelte` redirects to
      `data.landingRoute` instead of the hardcoded `"/plans"`.
      This is **presentation only**: authorization stays entirely with
      the service's ABAC (`agents/share/authorization-attributes.md`);
      every route stays reachable by direct URL navigation regardless
      of `view`, since `orderNavForView` reorders, it never removes.
      **Acceptance (matches the family-level task's own three bullets):**
      `attrs` absent ⇒ identical nav (`orderNavForView` unit-tested to
      return the input array, item-for-item, when `view` is
      null/undefined/empty/unmatched); `view=executive` lands on
      `/executive` (the literal acceptance example, pinned in
      `tests/unit/nav.test.ts`); every route stays reachable by URL
      regardless of the attribute (nav ordering never removes an item,
      and route access is unaffected — no new guard was added anywhere).
      **Verified:** `tests/unit/nav.test.ts` (new, 13 tests) +
      `tests/unit/server-auth.test.ts` (new, 4 tests) +
      `tests/unit/layout.test.ts` updated for the layout's new `data`
      shape; `pnpm run check` 0 errors/0 warnings; `pnpm exec vitest
      run` 100/100 (was 83, +17); `pnpm run build` green; `pnpm run
      lint` (`prettier --check`) clean. A real-browser check (Playwright)
      confirmed the signed-out default path — `/` → `/plans`, nav opens
      and renders unchanged — has zero regression, since a live
      backend was not available to exercise the signed-in `view` path
      end to end; that path is covered by the unit suite above plus
      the auth-service's own DB-gated `current_user_carries_live_abac_attrs`
      test proving `attrs` actually reaches the wire.

- [x] **T-28k (M) Responsive audit at a phone viewport** *(landed
      2026-09-19)* — tracked at the family level in the service crate's
      own `spec/13-tasks.md` T-28k; implemented here since it is a
      front-end layout concern.
      **The task's own route count was slightly off — corrected, not
      guessed around.** T-28k's text says "across all 34 routes";
      enumerating the real route tree (`find src/routes -name
      "+page.svelte"`, the same technique T-28p's onboarding test
      already uses to stay in sync with the tree rather than a
      hand-maintained list) gives **35**. Used the verified count.
      **Built.** A new `mobile` Playwright project
      (`playwright.config.ts`, 390×844 — the task's own viewport,
      matched exactly) running a new `tests/e2e/mobile.spec.ts` that
      iterates all 35 real routes, asserting per route (a) no
      horizontal scroll on the document (`scrollWidth <= clientWidth`)
      and (b) the page's `<h1>` heading is visible — a route-agnostic
      proxy for "the primary content is reachable" that does not
      require hand-picking a different specific "primary action"
      selector for 35 different pages. Each route is its own named
      Playwright test, per the task's own acceptance text ("each
      failing route is a named test, not a screenshot").
      **Real failures found and fixed, not guessed at.** Running the
      audit before any layout fix named four real, specific failures:
      `/plans` (642px), `/ideas` (396px), `/reviews` (480px),
      `/scenarios` (480px) — all against a 390px viewport. Root causes,
      each verified by inspecting actual `getBoundingClientRect()`
      output at 390px width, not assumed:
      - `/plans`: the SVAR **`FilterBar`** (not the `Grid`, which was
        already handled) rendered its internal text input at a fixed
        ~610px regardless of container width.
      - `/ideas`: a plain `<input size="40">` — the HTML `size`
        attribute sizes in characters, ignoring the container.
      - `/scenarios`: **corrected, not guessed** — `/scenarios` also
        carries `size="50"`/`size="12"`/`size="4"` inputs, but removing
        `/ideas`'s `size="40"` was the only edit that route needed; the
        global `input { max-width: 100% }` rule below (already
        sufficient for `/scenarios`'s specific layout, verified by
        re-running the full mobile suite) covers it without touching
        the attribute at all — `/scenarios`'s `size` attributes are
        untouched and it still passes.
      - `/reviews`: a bare `<table>` with no responsive handling at
        all.
      **The two SVAR-widget routes** (`/plans`'s `Grid`, `/gantt`'s
      `Gantt` — the family's own T-28k text names exactly these two)
      get a **read-only list fallback**: both markups render
      unconditionally; a single `@media (max-width: 600px)` rule hides
      the SVAR widget (and, on `/plans`, the `FilterBar` alongside it —
      filtering a plain name list has little value, and it was the
      actual overflow source) and shows a plain `<ul>` instead. No JS
      viewport detection, so nothing can get out of sync with SSR
      (this app has none — `ssr = false`, `+layout.ts`).
      **The `<input>`/`<table>` overflow got a global fix**, not two
      route-local patches: `src/app.css` gained `input, select,
      textarea { max-width: 100%; }` (only engages once a `size`-sized
      input's intrinsic width exceeds its container — no effect at a
      desktop viewport) and `table { display: block; overflow-x:
      auto; max-width: 100%; }` (23 of the 35 routes use a bare
      `<table>`; only `/reviews` happened to overflow at exactly
      390px, but the fix is global so a route this audit did not
      individually catch is covered too — the table's own internal
      scroll stays contained to the table, not the page body).
      **A real regression found and fixed in the same pass:**
      `tests/e2e/smoke.spec.ts`'s `"list page renders the seeded
      plan"` test broke once `/plans` rendered the seeded plan's name
      twice in the DOM (once in the SVAR grid cell, once in the new,
      CSS-hidden-at-desktop `.mobile-list`) — Playwright's strict-mode
      locator resolution does not consider CSS visibility before
      erroring on an ambiguous match. Fixed by scoping that assertion
      to `getByRole("gridcell", …)`, which is what the test actually
      meant to pin.
      **Acceptance (matches the family-level task's own text):** the
      e2e suite runs the mobile project in CI (`--project=mobile`);
      every one of the 35 routes is its own named test — a failure
      names the route, never a screenshot to eyeball.
      **Verified:** `pnpm exec playwright test` (all projects) 60/60
      (25 chromium, 35 mobile) — the mobile project was run against
      the *unfixed* layout first and confirmed to fail exactly the
      four named routes above, then again after each fix, closing one
      real cause at a time rather than changing several things and
      hoping; `pnpm run check` 0 errors/0 warnings; `pnpm exec vitest
      run` 100/100 (unchanged — this task touched no unit-tested
      logic); `pnpm run build` green; `pnpm run lint` (`prettier
      --check`) clean.

## 14. Implementation status

**Implemented (MVP, v0.1.0).** The SvelteKit app is built and verified (svelte-check clean, vitest + Playwright green): the routes in §5 are live against the sibling service via the BFF proxy, with SVAR grid / Kanban / Gantt views, Lily theme + locale chrome, and 13-locale i18n covering the original identity + merge surface (the later oversight/executive dashboard views are English-first — CHANGELOG 2026-07-22). Open §13 items — the plan-detail audit timeline, recent activity, MatchBreakdown, child roll-up, and the issues/timeline/goals sub-routes (the roadmap sub-list in §5) — remain unchecked.

## 15. Roadmap

- **v0.1** (here): spec + docs only.
- **v0.2**: scaffold + plan identity routes (list / create / detail /
  edit) + lean client + repository + BFF auth (own `/signin`+`/verify`,
  httpOnly cookie + CSRF, per
  `../../../agents/share/authentication-sessions.md`)
  + layout shell (top-bar hamburger, theme + locale
  selectors) + vitest + Playwright smoke.
- **v0.3**: duplicate-check (done) + merge (done, as the standalone
  `/plans/merge` page, FE-1 2026-08-03) + MatchBreakdown visual (not
  built) + audit timeline (not built) + child-plan roll-up (not built).
- **v0.4**: project-management views — Kanban board, issues, timeline /
  Gantt, burndown, goals.
- **v0.5**: full 13-locale translation catalogues + RTL polish.
- **Deferred / roadmap-only**: posts feed + threaded comments, member /
  role panel (not part of the v1 portfolio sub-resource set).

## 16. Open questions

- Project-management views as detail-page tabs or discrete sub-routes?
- Real-time duplicate warning on the create form?
- Which charting primitive for the Gantt / burndown (SVAR vs. native
  SVG vs. a Lily chart helper)?
- How rich should the org / person / worker / plan pickers be
  (typeahead against the sibling services vs. raw `pid` entry) for MVP?

## 17. References

- Sibling service spec: [project-portfolio-management-service-with-rust/spec](../../project-portfolio-management-service-with-rust/spec/index.md).
- Entity umbrella: [portfolio/spec](../../spec/index.md);
  [models](../../agents/models.md); [matching](../../agents/matching.md).
- Family layout shell + theming + locale conventions; SvelteKit 3 +
  Svelte 5 runes docs; SVAR Svelte DataGrid; Lily Design System Svelte
  Headless.
- Auth / sessions (source of truth): `agents/share/authentication-sessions.md`;
  blanket enforcement: `agents/share/jwt-enforcement.md`;
  locales: [`agents/share/locales.md`](../../spec/locales.md).

## 18. Change control

Update this spec with any behavioural change; a behavioural change is
one PR with three parts (spec + code + test). Bump `CHANGELOG.md`. Keep
`src/lib/api/types.ts` in lockstep with the matcher `Plan` type — if
a field changes in the service / matcher, fix it here in the same cycle.

## Exporting a table (CSV, TSV, Excel)

Every `DataTable` offers three buttons, **CSV**, **TSV** and **Excel**, in a
group named "Export" (the group and the buttons are the data grid's own,
switched on by giving it their labels; we write no export code).

- **What is exported:** every row the filter leaves, in the current sort,
  not only the shown page (`exportScope="view"`). Hidden columns and a
  column with `exportable: false` (an actions column, say) are left out.
  A cell exports its number, boolean or date as itself and anything else as
  the text shown, so a column's `format` applies.
- **File name:** `exportFilename` on `DataTable`, else the table's label,
  lower-cased and hyphenated; the grid adds `.csv`, `.tsv` or `.xlsx`.
- **Opt out:** `exportable={false}` for a table that is a layout aid rather
  than data (a two-row summary, say). Default on.
- **Safety:** a string starting with `=`, `+`, `-`, `@`, tab or CR is
  written with a leading `'`, so a spreadsheet never runs it as a formula
  (the grid's `safe` default; we do not turn it off). CSV and TSV carry a
  UTF-8 byte-order mark so Excel reads Welsh and Arabic correctly.
- **Words:** the button text and the "exported" announcement are in the
  page's language (`p.table.export*`); "CSV", "TSV" and "Excel" are the
  same in every language, so they are on the Welsh allow-list.
- **Access:** the export is done in the browser from rows already loaded; it
  reaches no new endpoint and shows nothing the page did not. A table that
  holds masked values exports the masked values.

The Gantt (`/gantt`), the delivery calendar (`/calendar`) and the plan task
board offer the same three formats, plus a **Copy** button that puts the rows
on the clipboard tab-separated for pasting into a spreadsheet (the widgets'
own, labelled from `src/lib/table-labels.ts`). `DataTable` also copies, sorts
text in the page's language with numbers in natural order ("Item 2" before
"Item 10"), names its pager as a landmark, and, when `searchable`, lets the
filter be limited to one column. The Gantt and the calendar have a search box
(every word, any order), the Gantt draws an arrow per dependency (the text
"Depends on" stays the accessible description), and a phone-width calendar
opens as the agenda.

The Gantt (`/gantt`) has a **Columns** group for its optional Kind, Stage, Start and End columns (Start and End by default; the choice is remembered in the browser).

A `DataTable` of five or more columns has a **Columns** control to hide and show columns (`hideable` overrides; `storageKey` remembers the choice). Shift+click on a second header adds a tie-break sort. Moving columns exists (`movable`) but only with `grid`, because the grid handles the keyboard move only in its grid model; no table uses `grid` yet.

**Not built:** server-side export of rows the page has not loaded (the
service's own CSV endpoints cover that), choosing columns at export time,
and a "selected rows only" option on the UI (the grid supports it; tables
here rarely select rows).

## Accessible table views (Lily headless Gantt and Kanban)

`@lilydesignsystem/svelte-headless` (already a dependency) ships `GanttTable*`
and `KanbanTable*`; there is **no separate Lily gantt or kanban npm package**.
`src/lib/lily-grids.ts` holds the pure helpers (`ganttGrid`, `kanbanGrid`,
`gridKeyTarget`), and `LilyGanttTable.svelte` / `LilyKanbanTable.svelte` render
them as a labelled `role="grid"` with arrow / Home / End navigation (the
WAI-ARIA grid keys the headless components leave to the consumer). They sit
**beside** the SVAR widgets behind a Chart / Table toggle; they do not replace
them, and the default stays the SVAR view. Rules: dates are zone-free and
handled as UTC day numbers (a DST change cannot shift a bar); a task with no
usable range is listed, never invented onto the timeline; a long range coarsens
the unit (day, week, month) and says so; a card with an unknown status is
listed, never dropped; the Kanban table only reports `onmove` and the service
stays the single writer. Tests: `tests/unit/lily-grids.test.ts`,
`tests/unit/lily-tables.test.ts`. Not done: an axe-core / Playwright audit of
these views (LT-12).

## Critical path and calendars (SC-1, SC-2, PJ-12)

Routes `/plans/[pid]/critical-path`, `/plans/[pid]/calendar` and `/me/calendar`
(links from the plan page and from *My work*; the plan links obey the RM-7
modules: critical path with `tasks`, calendar with `milestones`). The numbers
come from `GET /api/plans/{pid}/cpm`; the pure helpers are `src/lib/cpm-view.ts`.

- **Three views of one result:** a Gantt (`CpmGantt`: each bar from early start to
  early finish, then the task's **float as a hatched tail**, and a limit marker
  when float is negative), a precedence **network diagram** (`NetworkDiagram`:
  columns by longest predecessor chain, critical chains drawn heavier, non-trivial
  links labelled `SS +2 business days`), and a **table** (`CpmTable`, with free float,
  constraint, deadline and calendar).
- **State is never colour alone.** Every task has a marker (● critical, ◐
  near-critical, ○ float, ! cannot meet a constraint or deadline) *and* a
  sentence ("3 business days of float", "Cannot meet its constraint or deadline: 2
  business days short"). Negative float is said in words, never a bare minus.
  The diagram has a list twin; the Gantt has the table.
- **Honest about what is missing.** Tasks with no duration, or no start and no
  scheduled predecessor, are listed under *Not scheduled* with the reason.
  Assignees whose calendar shares no business day with the plan are named in a
  banner. Float is in business days; bars are drawn in calendar days (a weekend
  inside a bar is drawn, not counted).
- **The editor** sets start, duration (0 = milestone), a constraint and a
  deadline, and shows **what moved** across the network. A hard constraint
  (`mso`, `mfo`) asks for a reason; the service refuses it without the
  `scheduling=hard_constraints` (or admin) permission and the page shows that
  message. `alap` is not offered (the service does not support it).
- **Calendars:** `CalendarMonth` shows business days, off days (–), holidays (✕)
  and recorded absences (A) with a symbol each; editable, a business day is a
  button that toggles a holiday. A plan with no calendar says "every day works".
  A person's calendar narrows the plan's for their own tasks.
- **Not built:** a WBS tree with generated codes and an RBS (PJ-12's other
  halves), the S-curve and resource heatmap (they need earned value and
  availability), float in *My work* (SC-10), the MS Project import screen (the
  API client is JSON-only; the import is API-only for now), a browser (Playwright
  / axe) audit of these views (LT-12).

Tests: `tests/unit/cpm-view.test.ts` (masks, month grids, bars, float wording,
layout), `tests/unit/cpm-components.test.ts` (markers, table headers, keyboard,
calendar symbols), `tests/unit/ppm.test.ts` (paths and verbs).

## Capacity: availability and proposed bookings (PJ-3, PJ-4)

- **`/capacity`** now draws each person's load with `CapacityMeter`: the solid bar
  is what is **firm**, the capacity line is the capacity in force (it moves left
  for a part-time spell or leave), the **hatched extension is proposed and never
  added into the firm figure**, and a strip marks the days over capacity. A
  marker and a sentence say the same thing (○ within capacity, ◐ within, but
  committing the proposals would exceed it, ! over). `src/lib/capacity-view.ts`
  holds the pure model; `tests/unit/capacity-view.test.ts` pins it.
- **`/me/calendar`** also edits the caller's **availability ranges** (the most
  they can commit between two dates: 60 part-time, 0 leave). Capacity is read day
  by day, so a short spell is never averaged away. The service defaults the
  person to the caller.
- **Plan → governance → allocations**: an allocation can be created as *proposed*,
  shows a "proposed — not counted" chip, and has a **Commit** button. If the
  service answers `409 over_capacity` the page offers **Commit anyway**
  (`?confirm=true`), and the server audits it as an over-commitment.
- **Not built:** a resource-manager inbox for `requested` proposals, the demand
  vs capacity heatmap by role (SC-7), and a browser audit.

## Schedule baselines (SC-3)

On `/plans/[pid]/critical-path`: **Take baseline** (a reason is required),
a version picker, a *show the baseline under each bar* toggle (an outline under
the Gantt bar where the task was baselined to sit; `cpmBars` widens the span to
include it) and the **variance table** (`VarianceTable`). Rules the page keeps:
a baseline is **append-only** (the page offers no edit or delete, re-baselining
is the next version); variance is in **business days**; every row has a marker
and a sentence (▲ slipped, ▼ pulled in, = on baseline, + **added — no slip**, −
**removed — kept with its baselined dates**, ? cannot say); the summary states
the plan's own finish movement and warns when it improved only because tasks
were removed; with no baseline it says so instead of showing zero slip.
**Not built:** comparing two baselines with each other, and a baseline for
plan-level (non-task) nodes.

## RACI matrix

Route `/plans/[pid]/raci` (the plan page links to it with the `tasks` module):
tasks down, people across, and in each cell four toggle **buttons** — R
Responsible, A Accountable, C Consulted, I Informed — each with a sentence for
assistive technology ("Design: 3f2c6b7e… is Responsible and Accountable") and a
visible letter, so colour is never the only signal. A toggle saves the row.
Rules the page keeps (`src/lib/raci-view.ts`): **one Accountable per task**
(giving it to someone moves it and a status line says so); a person left with no
letter drops out of the matrix; a row missing an Accountable or a Responsible is
named in words (⚠), not hidden; a person is added as a column by their
`person:` / `worker:` reference; the summary counts reconcile with the rows;
**Export to a spreadsheet** is the service's CSV. People marked R, A or I are
auto-subscribed to the task by the service (unless they switched `auto_watch`
off). **Not built:** role columns (only people), deliverable-level rows (only
tasks), a copy-row-from-another-task shortcut.

## Earned value (SC-4)

Route `/plans/[pid]/earned-value` (the plan page links to it with the `financials`
module): one section **per currency** with `EvBars` (planned value, earned value
and actual cost as three bars against the budget-at-completion line, each with its
amount as text), schedule and cost performance and earned-schedule SPI(t) in
sentences with a marker (▲ ahead, ● on track, ▼ behind, ? unknown — a stated 5%
band is "on track"), and a table of every figure. Rules the page keeps
(`src/lib/ev-view.ts`): ratios are basis points shown to two decimals by integer
arithmetic; a ratio the service cannot state shows its **reason** (nothing planned
yet, no cost recorded), never zero or 1.00; an unknown actual cost is "unknown",
not 0; how percent complete was counted is said before the numbers (the plan's
method, equal or story-point weights); **any figure resting on a person-entered
percent carries a banner naming those tasks**; a plan with no baseline, or nothing
to weigh, says why instead of showing figures; budget lines in a currency with no
baseline are named, never added in. The page lets you set the plan's default method
and count one task another way (a manual percent needs a note). Measured as of
today. **Not built:** the S-curve over time and the EAC/TCPI analytics (SC-5),
actual cost from timesheets (needs cost rates, PJ-2).

## EVM analytics (SC-5)

On `/plans/[pid]/earned-value`, below the figures:

- **Where it is heading**: every forecast of the final cost **named by its formula**
  (the overrun so far is a one-off / today's cost efficiency continues / cost and
  schedule pressure both continue / the plan's own estimate), with ETC and VAC (an
  overrun is labelled). One is marked "the one to lead with" by a **stated rule**
  and the sentence under the table gives it; the others stay beside it. A TCPI
  sentence says what the remaining work needs, and says plainly when the money is
  already spent. Lights (● on track, ◐ slipping, ✖ off plan, ? not yet known) use
  the deployment's thresholds, shown on the page.
- **Over time** (`SCurve`): the planned-value line is the whole baseline; earned
  value (●) and actual cost (■) are **dots only on days they were recorded**, never
  joined or filled in; a table under the picture holds every number; a button records
  today's figures now (the scheduler records them daily).
- **This plan and everything beneath it**: totals per currency, SPI and CPI from the
  totals (no CPI when some plan's cost is unknown), and a count of plans with no
  figures.

**Not built:** a per-period history before snapshots began (it cannot be
reconstructed), a spreadsheet link for the series on the page (the CSV is in the API).

## Command palette (BR-14)

Ctrl or Cmd with K, or the "Search and commands" entry in the navigation menu (it is in the menu, not the top row: one more control in the top row wrapped the bar on a narrow screen and pushed the CEO dashboard off an iPad in portrait in CI), opens a modal palette. It is Lily's `Dialog`
(a real modal: focus trapped, Escape closes, focus returns to the opener) holding Lily's `Combobox` (the input has
`role="combobox"`, `aria-controls` and `aria-activedescendant`; the rows are `role="option"` with `aria-selected`).
Typing two or more letters searches plans by name through `GET /api/plans/search?q=` after a 200 ms pause, and only
the latest answer is shown; commands are matched at once. Rows are ranked by the whole label, a prefix, a word start,
a match inside, then letters in order (`src/lib/palette-view.ts`, pure and unit-tested); with nothing typed the recent
items come first. The first match is active, the arrow keys move (wrapping), Enter goes there, one Escape closes the palette (typed text
or not), and the rows stay shown when focus leaves the input and comes back. Lily's `Combobox` (from headless 0.7)
keys its own list, with no row active until an arrow key and Escape closing only the list; the palette takes the arrow
keys, Enter, Escape, Home and End (which still move the caret) in the capture phase before it, and puts the active row back after each keystroke. The
active row is marked by a bar and bold text as well as colour, and every row says what it is (Command, Plan, Recent).
Ctrl or Cmd with K is taken over everywhere in the app, text fields included, so the browser's own shortcut for it does not run. The last five items chosen are kept in this browser (`mxi.palette.recent`), never sent anywhere; a stored item with a
path that does not start with a single `/` is dropped. A failed search says so in words and the commands still work.
Commands today: New plan, My work, Escalations, Dashboard. Not built: a command per page; searching tasks, risks or
meetings; a "status page" command (the UI has no status page route); the shortcut shown in the button's text; other
locales than English and Welsh (the rest show English). Tests: `tests/unit/palette-view.test.ts`,
`tests/e2e/command-palette.spec.ts`.

## Paste rows from a spreadsheet (BR-15)

On the board (the task list), "Paste rows from a spreadsheet" takes cells copied from a spreadsheet: one task per line,
tab-separated, the columns Title, Deadline (`yyyy-mm-dd`) and Notes in that order; a first line that names the columns
is skipped (`src/lib/paste-view.ts`, pure and unit-tested; `src/lib/components/PasteTasks.svelte`). Every row is shown
with its status. A row with no title, a title over 300 characters, a date that is not `yyyy-mm-dd` (other forms are
refused, not guessed, because day-month and month-day cannot be told apart), notes over 5000 characters, a repeat of an
earlier row, or beyond the 200th row is shown with the reason and **left out**; only the good rows go on. "Check with
the service" sends them to the Asana-shaped import as a dry run, which writes nothing, and shows what it would do;
"Create N tasks" sends the same rows for real. Each row's id is made from its own title and date, so pasting the same
rows again updates the tasks it made and creates none (the stored reference reads `asana:paste-xxxxxxxx`). Changing the
text starts again. **The date is stored as the task's deadline, not its due date**: that is what the import does ("due
dates come across as deadlines"), so the column is called Deadline, the board does not show it, and the overdue counts
(which read the due date) do not see it. `GET /api/plans/{pid}/tasks` now returns `deadline`. Not built: more columns
(assignee, points, status); a due date; a deadline on the board; translations other than English and Welsh. Tests:
`tests/unit/paste-view.test.ts`, `tests/e2e/paste-tasks.spec.ts` and, in the service,
`tests/requests/paste_import.rs`.

## Hiding sections of the top bar (BR-17)

The Display page (`/display`) lists every section of the top bar with a tick; unticking one hides it from the bar for
that person. The choice is a list of paths (`hidden`) in the person's display preferences: kept on the device and, when
signed in, on the account like simple view and the font. **A default is invisible.** Nothing is stored until a section is
hidden, and showing the last one again removes the field, so a person who never changed anything follows the current
default and a section added later reaches everyone who did not hide it, including someone who hid another. The way home
(`/`) cannot be hidden, and the Display page is a link in the top bar's own end (not a section), so a choice can always
be undone. A hidden section is not removed: its page still works at its address. "Show all sections" clears the list.
Hidden sections are removed before simple view splits the bar, so they appear in neither the front nor "More".
Account writes go through one queue (`serialQueue`, `src/lib/preferences.ts`): one at a time, in order, each sending the
choices as they stand when its turn comes, so two quick changes cannot interleave their list, delete and create steps.
The bar's items are in `src/lib/nav-items.ts`, shared by the layout and the Display page; the eight collection entries now have words in English and Welsh (they were shown as raw lowercase English names in every language). **Not built:** per-plan
switches in the UI (the service has them, `GET`/`PUT /api/plans/{pid}/modules`, and the plan page already follows them,
but no screen sets them); hiding a plan's workspace links per person; hiding a section for everyone. Tests:
`tests/unit/hidden-sections.test.ts`, `tests/e2e/hidden-sections.spec.ts`.

## Roadmap: impact / effort matrix and Now / Next / Later (NC-10)

`/roadmap` (`src/routes/roadmap`, logic in `src/lib/roadmap-view.ts`) places **open ideas** and **proposals still
being decided** (not rejected, not promoted) on a 3 by 3 matrix and in Now / Next / Later lists.

- **Where the figures come from.** The service holds **no impact or effort** for an idea or a proposal (the
  RICE fields belong to plan scoring, not to these), and this change does not add any. A person rates each item
  here as Low, Medium or High impact and effort. Ratings are kept in this browser only (`localStorage`
  `ppm.roadmap.ratings`, keyed `idea:<pid>` / `proposal:<pid>`); they are **not** shared, not sent to the service,
  and do not follow the person to another browser. If storage is blocked the page still works for the visit.
  Nothing is guessed: an item with either rating missing is **Not yet rated** and is not placed.
- **Horizons** (a pure rule, `horizonOf`): rank Low 1, Medium 2, High 3; impact less effort is 1 or more means
  **Now**; 0 with at least Medium impact means **Next**; anything else rated means **Later**.
- **Keyboard alternative.** The matrix is a real `<table>` (row headers for impact, column headers for effort,
  every cell a list of items). The table "Rate the items" (`DataTable`) is the way to set and change ratings: each
  row has an impact and an effort `<select>` named after the item, so everything the matrix shows can be reached
  and changed with the keyboard alone. Status is never colour alone: horizons are words and lists.
- Linked from `/ideas` and `/proposals`; not in the top bar. Load failures show a fixed catalogue sentence
  (`resource()` and `errorText`), never the service's text. Strings are `p.roadmap.*`; Welsh is machine-written
  and listed in `governance/welsh-translation-review.csv`.
- **Tests.** `tests/unit/roadmap-view.test.ts` (rules, filtering, storage parsing); `tests/e2e/roadmap.spec.ts`
  (keyboard rating moves an item in the matrix and the lists, reload remembers, fixed error sentence, Welsh,
  axe in two locales); the route is also audited by `a11y.spec.ts`.
- **Not built.** Shared or server-saved ratings (needs a service field); dragging items on the matrix; sorting
  within a cell; a link in the top bar; non-English text beyond English and Welsh (other locales fall back to English).

## Cross-app bridges (links to contact-relationship-management and workforce-planning-management)

A bridge is a **plain link** in each direction, with no runtime coupling: this app never calls the
other apps, shares a session with them, or fails when they are down.

- **Outbound.** A deployment sets `CONTACTS_APP_URL` and `WORKFORCE_APP_URL` (http or https, no
  credentials; anything else is ignored). Where the UI shows a stored reference, a `worker:<id>` links to
  `{WORKFORCE_APP_URL}/workers/<id>` and a `contact:<id>` to `{CONTACTS_APP_URL}/contacts/<id>`, opened in a
  new tab with `rel="noopener noreferrer"`. With no URL set no link is shown. `person:<id>` is **not**
  linked: nothing says a person's id is the same in either app. Today the links appear on the RACI workload
  table and the key-persons successor column (`src/lib/bridge.ts`, `BridgeLink.svelte`).
- **Inbound.** The other apps may link to `/plans`, `/plans/{pid}`, `/plans/{pid}/work`,
  `/plans/{pid}/raci` and `/me` (with the locale prefix the app adds). These routes are part of the
  contract: `tests/unit/bridge.test.ts` fails if one stops existing.
- **People impact (FB-4).** On `/plans/{pid}` for a plan of kind Program, a "People impact (from the workforce
  service)" box asks the service's `GET /api/plans/{pid}/people-impact` when the reader presses its button (never
  on load), and shows the answer as WPM gave it: the date it is as of, the end date sent (or why none), each part's
  counts by pool, a count under WPM's floor as "hidden (fewer than N)", never zero, an unknown part with WPM's
  reason, and no total; when the answer is `unknown`, only the reason. The service, not the browser, calls WPM
  (`src/lib/people-impact.ts`, `PeopleImpact.svelte`; `tests/unit/people-impact.test.ts`,
  `tests/e2e/people-impact.spec.ts`).
- **The pool link and the heatmap (FB-6).** `/skill-pools` lists every pool (name, roles, skills, state, the
  workforce service's pool id it is linked to, or "not linked") in the `DataTable`, and a per-row "Change link"
  opens a form for that pool. The id is checked before it is sent: a lower-case hyphenated UUID, trimmed, the same
  rule as the service's (`isPeerPoolRef`); anything else is refused on the page and nothing is sent. The service's
  `409` (another pool here is already linked to that workforce pool) and `422` (the id was not accepted) are each
  said in the catalogue's own sentence; the service's text is never shown. Removing a link (`peer_pool_ref: null`)
  is asked first through `askConfirm`, which says that every demand figure already sent to the workforce service
  for the pool will be withdrawn there (FB-5); after it, the page says how many withdrawals the service started
  (`peer_demand_withdrawals`). The page says that the link must be set on both sides by a person who can
  see both, and is never matched by name. `/capacity-heatmap` draws `GET /api/capacity-heatmap[?from=YYYY-MM]
  [&months=N]` as a table, pool by month: each square has the service's word and a symbol for the service's
  `shape` as well as a border tone from its `colour`, so colour is never the only signal; demand, supply and gap
  are in the reader's locale (`Intl` with `currentLocale()`). For a pool with `supply_source: "wpm"` the square
  also says "Supply from the workforce service", the reading (`live` with when it was taken, `last_received` with
  its age and the reason it is not live, `hidden` under the workforce service's floor, `unknown` with the
  service's reason), and "Entered here" beside it, never added; a `supply_unknown` month says "Supply: unknown",
  never zero. Every figure is also in a `DataTable` under "Every figure as a table". Both are in the top
  navigation (`src/lib/nav-items.ts`). Words in `src/lib/capacity-heatmap-view.ts`
  (`tests/unit/capacity-heatmap-view.test.ts`); browser test `tests/e2e/wpm-bridge.spec.ts`. **Not built:** pools
  are not created or edited here. The service's reasons are English and shown as they come.
- **Capacity commitments (FB-6 completed, FB-5).** `/capacity-commitments` is the form for
  `PUT /api/capacity-commitments`: skill pool (active pools only; a linked one says so), programme (every plan the
  reader can list; `?plan=<pid>` chooses one, and a plan not in the list is fetched by id), supply or demand,
  optional partner organization, month (`<input type="month">`), committed and optional delivered full-time
  equivalents read in the reader's own number format through `parseAmount` (0 to 1000), and a note. Every wrong
  field is named under it (`aria-invalid`, `aria-describedby`) and nothing is sent until all are right; the
  service's 404 and 422 are said in the catalogue's own sentences. After a save of a demand commitment the page
  says what happened to the demand sent to the workforce service (`peer_demand_send`) in plain words: started (a
  figure, or the withdrawal at zero), taken, withdrawn, already there, refused, not reached, not configured, no
  sign-in, too large, an old zero not sent, or nothing sent because the pool is not linked; the service's
  English `detail` is never shown. "Recent sends to the workforce service" lists the latest send per programme,
  pool and month from `GET /api/capacity-commitments` (newest first, at most 20) in a `DataTable`, with a Refresh
  button, since a started send's answer arrives later. Reached from `/capacity-heatmap` ("Set a capacity
  commitment") and a programme's plan page ("Capacity commitments for this programme"); not in the top
  navigation. Words in `src/lib/capacity-commitments-view.ts` (`tests/unit/capacity-commitments-view.test.ts`);
  browser test `tests/e2e/capacity-commitments.spec.ts` (axe, Welsh). The messages use the neutral banner.
  The success banner (`.banner.success`, `src/app.css`, used by `/skill-pools`) once failed colour contrast under
  the light theme (1.87:1, success colour `#00d390`); its text is now the theme's success colour mixed half and half
  with the theme's text colour, on an 8 % success tint of the page background, so it passes 4.5:1 under the light and
  the dark theme (`tests/e2e/wpm-bridge.spec.ts`, axe `color-contrast` on the banner in both). `/skill-pools`,
  `/capacity-heatmap` (`tests/e2e/wpm-bridge.spec.ts`) and `/capacity-commitments`
  (`tests/e2e/capacity-commitments.spec.ts`) are each audited as populated with axe `color-contrast` over the whole
  page under both themes (see **Theming**).
- **Not built:** the other two apps do not yet link here (their repositories are not changed by this);
  there is no shared sign-in, so a reader signs in to each app; no data is copied between the apps.


## Plan issues: raise, edit, raise from a risk (NC-1 slice 1, IR-2, IR-3)

`/plans/[pid]/issues` is the screen for the issue routes that existed only at the API. It is linked from the
plan page (module `governance`) and from the title of each row in the all-plans register at `/issues`.

- **List.** `GET /api/plans/{pid}/issues`: the plan's issues, most serious first, in a `DataTable` with the
  kind, severity and status as words, the age in calendar days, and "Overdue" in words.
- **Raise and edit.** One form (`POST /api/plans/{pid}/issues`; `PUT …/issues/{issue_pid}` for an edit, which
  replaces every field, so the form always sends all of them). Title is required; a closed issue needs a
  resolution; a due date is not before the day raised. The page checks these three before sending
  (`src/lib/issue-view.ts`) and says which in the reader's language; the service remains the authority. An
  empty "raised on" is left out so the service uses today. The body type `IssueBody` is checked against
  `openapi.json` by `client-bodies.test.ts`.
- **Raise from a risk.** `POST …/issues/from-risk/{risk_pid}` from a list of the plan's risks. The service
  allows one open issue per risk; a second attempt is a 409, which the page shows as the catalogue's
  sentence for a conflict, never the service's text.
- **Not built in this slice:** deleting an issue; a "raise as issue" button on each risk row of the governance
  page; `source_task_pid` (raise from a task); `priority`; picking the owner from a list of people (the
  owner is a typed `person:` or `worker:` reference).
- **Tests:** `tests/unit/issue-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/plan-issues.spec.ts` (raise, edit, the refused close, raise from a risk, the 409, Welsh, axe).
  The Welsh is machine-written and listed in `governance/welsh-translation-review.csv`.

## Plan change requests: draft, submit, decide, withdraw (NC-1 slice 2, IR-4)

`/plans/[pid]/change-requests` is the screen for the change-request routes (IR-4) that existed only at the API.
It is linked from the plan page (module `governance`) and from the title of each row in the all-plans register at
`/change-requests`, which stays read-only.

- **List.** `GET /api/plans/{pid}/change-requests`: the plan's change requests, newest first, in a `DataTable`
  with the kind and status as words, the stated impact (business days, and money in its own currency's minor
  unit), the named approver and the age in calendar days.
- **Draft and edit.** One form (`POST /api/plans/{pid}/change-requests`; `PUT …/{cr}` for an edit). Only a draft
  can be edited, so the edit button is on draft rows only. Title, description and reason are required; the
  schedule impact is a whole number of business days from -3650 to 3650 (negative pulls the finish in); a cost
  needs a currency and a currency needs a cost; an empty impact is sent as null (not assessed), never zero. The
  page checks these before sending (`src/lib/change-request-view.ts`); the service remains the authority. The
  edit replaces every field, so what-if edits already attached to the request (`edits`) are sent back unchanged
  rather than cleared. The body type `ChangeRequestBody` is checked against `openapi.json` by
  `client-bodies.test.ts`.
- **Steps.** Choosing "Steps" on a row opens that request's steps, offered by its status as the service's state
  machine allows: a draft can be submitted (naming the approver as a `person:` or `worker:` reference, not the
  requester) or withdrawn; a submitted one can be put under review, decided or withdrawn; one under review can
  be decided or withdrawn; an approved one can be withdrawn. A decision is approve or reject with a reason of at
  least 10 characters. A rejected, implemented or withdrawn request has no steps; its decision, if any, is shown.
  The page does not know who the caller is, so it offers the steps by status only: the service refuses a step
  the caller may not take (only the requester edits, submits or withdraws, besides an administrator for
  withdraw; only the named approver reviews or decides).
- **Refusals.** A 403 says who may take that step, a 409 says the request is no longer in a state that allows it
  (and the list is read again), a 422 says what was not accepted, each in a catalogue sentence; the service's
  own text is never shown.
- **Not built in this slice:** marking an approved change implemented (`…/implement`); linking issues and risks
  (`…/links`); the what-if forecast (`…/preview`) and attaching what-if edits; taking a baseline for an approved
  change (`…/baseline`); a confirmation before withdrawing; picking the approver from a list of people.
- **Tests:** `tests/unit/change-request-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/plan-change-requests.spec.ts` (draft, refused draft, edit, submit, review, decide, withdraw, the
  409, Welsh, axe). The Welsh and the other six languages are machine-written and unreviewed.

## Task editing and links between tasks (NC-1 slice 3, LT-10)

`/plans/[pid]/tasks/[tpid]` (which already showed a task and its links to outside systems) is now where a task is
edited and linked to other tasks. Each card on the board (`/plans/[pid]/board`) links to it from its title; the page
links back to the board and to the work breakdown.

- **Edit.** One form over `PUT /api/plans/{pid}/tasks/{t_pid}`: title, description, assignee, story points and due
  date. The service replaces every field the body names and clears one left out, so the body always carries the
  task's sprint, component and parent task as they were loaded; it never carries a status (a status change is the
  board's move, `PATCH`, which keeps the flow stamps true). The page checks the service's rules before sending
  (`src/lib/task-edit-view.ts`): a title (at most 1024 bytes), an assignee that is a `person:` or `worker:`
  reference to a UUID or nobody, whole story points from 0 to 100 or none, a due date or none. The service remains
  the authority: a new due date that breaks a `precedes` link is its 422. The body type `UpdateTaskBody` is checked
  against `openapi.json` by `client-bodies.test.ts`. The task routes take no `If-Match` and answer no `ETag`, so the
  edit is last-writer-wins and there is no 412 to handle.
- **Links to other tasks.** `GET …/tasks/{t_pid}/relations` lists them from this task's side, in words: "Blocks" /
  "Blocked by", "Comes before" / "Comes after" (`precedes`), "Related to", "Duplicate of" / "Duplicated by",
  "Copied from" / "Copied to". Each names the other task (a link to its page), its status and due date, and has a
  Remove button (`DELETE …/relations/{r_pid}`; neither task is removed). Links to tasks in plans the caller cannot
  read are counted, never shown, as the service answers. A short note says what each kind does (`blocks` stops the
  other task finishing; `precedes` needs the other task due after; closing a task closes its duplicates).
- **Add a link.** `POST …/relations` with a kind and one of this plan's other tasks. The page refuses before asking:
  no task chosen; the same link again (`related` either way is the same link); a two-task loop of `blocks` or
  `precedes`; `precedes` when both tasks are dated and the other is not due strictly after this one. A longer loop,
  or a task that has gone, is the service's 422.
- **Refusals.** A 403 says the caller may not change this plan's tasks; a 404 says the task (or the link) has gone
  and reads it again; a 409 says the link exists and reads the links again; a 422 says what was not accepted, for
  an edit or a link. Each is a catalogue sentence; the service's own text is never shown.
- **Not built in this slice:** editing the sprint, component or parent task here (they are kept); a status change
  here (the board does it); a link to a task in another plan, or an incoming link made from this side (the add
  form links this task to another of the same plan); a confirmation before removing a link; showing links on the
  board's cards; editing the start date or duration (the schedule editor on `/plans/[pid]/critical-path` does);
  custom fields (`PUT /api/custom-fields/values`); a person picker for the assignee.
- **Tests:** `tests/unit/task-edit-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/plan-task-edit.spec.ts` (the board's link, the edit's body, refusals on the page, a 422 and a 403 on
  the edit, the links read both ways, add and remove, refusals on the page, a 409, a 422 and a 404 on the links,
  Welsh, axe). The Welsh and the other six languages are machine-written and unreviewed.

## My notifications: inbox, choices and destinations (NC-1 slice 4, PL-5, LT-6, RM-11)

`/me/notifications`, linked from `/me`, is the signed-in person's own notification settings. Everything on it is the
caller's alone: the service keys it by the token, and nobody (not an administrator) reads another person's.
`src/lib/notifications-view.ts` holds the rules; the page only renders.

- **Inbox.** `GET /api/me/inbox?tz=` with the browser's IANA zone, so a daily digest groups by the person's own day;
  "Show only unread" adds `unread=true`. Each line says read or unread in words and a shape (never colour alone), the
  kind in words, the message the service wrote, its time, and for a digest how many notifications it stands for.
  A line about a plan links to it; a line about anything else (a task, a review) has no link, as the line carries no
  plan to build one from. "Mark read" stamps every notification the line stands for
  (`POST /api/notifications/{pid}/read`, idempotent, through `CapabilityClient.markRead`, which existed), then reads
  the inbox again. The count above is the service's `unread_count`, which counts notifications, not lines.
- **What you are told about.** `GET /api/me/notification-prefs`: one group per kind the service lists
  (`known_kinds`), a box per channel it lists (in the inbox, by signed webhook, in chat, by email) and "Daily digest
  in the inbox". No stored row means the default (on, instant), as the service says in `default`. A box saves at
  once (`PUT /api/me/notification-prefs` with the kind, channel, on or off, and the digest kept; the channel is
  always sent, as the service reads a missing one as `in_app`); a digest is only sent as `daily` on the inbox
  channel, where the service reads it. "Back to the default" shows only for a kind with a stored choice and resets
  each stored channel through the `POST …/reset` twin (the default policy keeps `DELETE` for administrators).
  A refused change reads the choices again and redraws the boxes from them.
- **Where copies go.** `GET /api/me/notification-sinks`: each destination's kind ("Slack chat", "Signed webhook",
  "Email"), its address as the service masks it (scheme and host and the last four characters, or `***@domain`; the
  page never has more), and whether it is sending or paused. Add one (`POST`): a kind, the address and, for
  Telegram, the chat. The page checks the service's rules before sending: an email address shaped as `email_shaped`
  wants, otherwise an `https` address with no user name or password whose host is not `localhost`, a `.local`,
  `.internal` or `.localhost` name, or a loopback, private, link-local (the metadata address), carrier-grade NAT,
  documentation, multicast or unique-local literal; at most 2048 characters; at most ten destinations. A name that
  resolves inside the network is the service's to refuse when it sends. Once saved, the box is cleared, so the
  secret does not stay on the page. Remove one through the `POST …/{pid}/delete` twin. "Recent deliveries" lists the
  last 50 (`GET …/{pid}/deliveries`): the outcome in words, attempts, when sent or the next try. The delivery's
  `last_error` is **not shown**: it is the service's text, and a transport error can quote the destination's URL.
- **Refusals.** A 403 says the caller may not change these settings; a 404 on a read stamp says the notification has
  gone or is not theirs (the service answers both alike) and reads the inbox again; a 404 on a destination says it
  had already been removed and reads the list again; a 422 on a choice or a destination says what was not accepted;
  a 422 on reading any section says the sign-in identity is not one notifications can be addressed to (a token
  `sub` that is not a UUID). These routes answer no 409 and take no `If-Match`. Each is a catalogue sentence; the
  service's own text is never shown.
- **Not built in this slice:** pausing a destination (the service has no route to change `enabled`); a test send;
  marking every line read at once; paging the inbox past the service's 500 rows; a link from a line about a task or
  a review; a confirmation before removing a destination; the admin dispatch (`POST /api/notification-sinks/dispatch`);
  showing why a delivery failed; the other kinds' wording beyond the 17 the service sends today (a new kind shows
  its code).
- **Tests:** `tests/unit/notifications-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/me-notifications.spec.ts` (the link from `/me`, the inbox and a mark read, unread only, a choice off, a
  daily digest, back to the default, a destination refused on the page three ways, added, its deliveries read
  without the service's error text, and removed, a 404, 422, 403 and 404, an identity the service cannot address,
  Welsh, axe). The Welsh and the other six languages are machine-written and unreviewed.

## A task's discussion and files: following, reactions, comments, attachments (NC-1 slice 5, LT-4, LT-5, RM-3)

The task page `/plans/[pid]/tasks/[tpid]` ends with two sections drawn by `src/lib/components/Discussion.svelte`,
which takes a subject kind and pid (the service's generic `subject_kind` / `subject_pid`), so the same component can
later serve a plan, a risk or a proposal; this slice mounts it on the task page only. `src/lib/discussion-view.ts`
holds the rules; the component only renders. None of these routes takes an `If-Match`; none answers a 409.

- **Following (RM-3).** `GET /api/watchers` says how many follow the task and whether the caller does; nobody is
  named. "Follow" is `POST /api/watchers`, "Stop following" the `POST /api/watchers/unwatch` twin (the default policy
  keeps `DELETE` for administrators). These routes need a sign-in: a 401 on the status is shown as a sentence ("Sign
  in to follow this…") with no button and no alert. Commenting follows the thread unless the person switched
  `auto_watch` off on `/me/notifications`, so the status is read again after a comment.
- **Reactions (LT-4).** The service's closed set of five (thumbs up, heart, celebrate, looking at it, question), each
  a toggle button with `aria-pressed` for the caller's own, its count, and an accessible name in words ("Heart: 2
  people"); the symbol is drawn `aria-hidden`. Adding is `POST /api/reactions` (a repeat writes nothing), taking back
  the `POST /api/reactions/remove` twin.
- **Comments (LT-4).** `GET /api/comments`, oldest first; every page is read by following the `Link` header
  (`ApiClient.followPages`, at most 20 pages of 500). Each shows the writer (their token `sub`, as the service holds
  it; there is no name lookup), when, and "edited" with when; the text keeps its line breaks and is plain text (never
  HTML). "Earlier versions" (only on an edited comment) reads `GET /api/comments/{pid}/history`. Edit is
  `PUT /api/comments/{pid}`; Delete asks first (`askConfirm`) and is the `POST /api/comments/{pid}/delete` twin (a soft
  delete; earlier versions are kept). The page does not know who the caller is, so Edit and Delete show on every
  comment and the service's 403 says "Only the person who wrote a comment may change or delete it." Before sending,
  the page refuses a blank comment and one over 4000 characters, counted by code point as the service counts, and
  shows how many characters are left.
- **Files (LT-5).** `GET /api/attachments` (every page read): each file is a link to the service's own
  `GET /api/attachments/{pid}/download` through the proxy, which relays the service's `Content-Disposition:
  attachment`, `Content-Type` and `Cache-Control: private, no-store`, with the UI's own `nosniff`; so the browser saves
  the file. **Nothing uploaded is ever drawn in the page:** no preview, no `<img>`, no frame, no `target`. Beside the
  link: the type in words ("PDF document"), the size (bytes, KB or MB, 1024-based), who added it and when. Attaching
  sends the chosen file itself as the request body (`POST /api/attachments?subject_kind=&subject_pid=&filename=`,
  the file's type as `Content-Type`; not multipart, which the service does not take). The page refuses before sending:
  no file, an empty file, one over 10 MiB (the service's default cap; a deployment may set another, which the page
  cannot read, and the service decides), and a type outside the service's twelve (PDF, PNG, JPEG, GIF, WebP, plain
  text, CSV, Markdown, JSON, Word, Excel, PowerPoint; never HTML, SVG, archives or programs). When the browser gives no
  type for a `.md`, `.csv`, `.json` or `.txt` file, the type is taken from its name; a type the browser did name is
  never replaced. The file box's `accept` lists the twelve. Remove asks first and is the
  `POST /api/attachments/{pid}/delete` twin (the uploader or an administrator). The service also checks that the
  content is what the type says and may run a malware scan; its 422 and 503 are sentences.
- **Body size at the proxy.** adapter-node refuses a request body over 512 KiB unless `BODY_SIZE_LIMIT` is set, which
  would stop most files at the proxy. The container image sets `BODY_SIZE_LIMIT=10304K` (the service's own transport
  ceiling for the upload route: 10 MiB plus 64 KiB). It is one limit for the whole server, not only this route. A 413
  is shown as "The file is too large to send."
- **Refusals.** 401: sign in first; 403: the author rule (edit, delete), the uploader rule (remove a file), or "You may
  not read or change this"; 404: the comment or file has gone (the list is read again), or the subject is not found;
  413: too large to send; 422: what was not accepted, for a comment or a file; 503 on an upload: the malware scanner
  did not answer. Each is a catalogue sentence; the service's own text is never shown.
- **Not built in this slice:** the section on any page other than a task (plan, risk, proposal, idea, canvas item,
  budget line, scenario, document version); `GET /api/me/watching` (a list of what the caller follows); attachment
  search (`GET /api/attachments/search`, PL-8); people's names for a `sub`; mentions, threads or replies (the service
  has none); hiding Edit and Delete on other people's comments; live updates (the plan's `comment_added` event, PL-3);
  reading the deployment's attachment cap; drag and drop, several files at once, or upload progress; a preview of any
  file.
- **Tests:** `tests/unit/discussion-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/task-discussion.spec.ts` (thread, post, follow after commenting; blank and over-long refused on the page;
  edit, earlier versions, delete after asking; a 403 and a 404; a reaction added and taken back; follow and stop, and
  signed out; files as download links never drawn in the page, a Markdown file sent as itself with its type; a web
  page, an empty file and one over 10 MiB refused on the page; a 422, a 403 and a 503; remove after asking; Welsh;
  axe). `tests/e2e/plan-task-edit.spec.ts` now also answers these routes, empty. The Welsh and the other six languages
  are machine-written and unreviewed.

## My timesheet: a week by day and plan, submitting it, and an approver's decision (NC-1 slice 6, LT-11)

`/me/timesheet`, linked from `/me`, shows the signed-in person's recorded time for one week and lets them send it for
approval; below, a person with the approver right reads someone else's week and decides it. `src/lib/timesheet-view.ts`
holds the rules and `src/lib/components/TimesheetGrid.svelte` draws a week; the page only wires them.

- **Which week.** A week is the service's ISO 8601 week (`2026-W46`, Monday to Sunday), whatever the person's first
  day of the week on the Display page (WW-1.4): the week the service approves and locks is that set of days, so the
  grid does not reorder them. "This week" is the week of today in the browser's IANA time zone (computed with
  `Intl.DateTimeFormat`, so a person in Auckland on Monday morning sees the new week while London still has Sunday);
  the page says the date and the zone. "Previous week", "Next week" and a typed week (`2026-W46`, a lower-case `w` and
  spaces accepted) move it; a typed week the service would refuse (week 0, a 53rd week in a 52-week year, anything
  else) is refused on the page. Time entries carry a calendar date, not a time, so no other zone applies.
- **The grid.** `GET /api/timesheets?week=` (the caller's own; the service takes the person from the token). A row
  per plan, named by `GET /api/plans/{pid}` (a plan the page cannot read is shown as the start of its id, "name not
  shown"), linking to the plan; a column per day headed by weekday and date in the page's locale; then the row's
  total and its billable and not billable halves. The foot row is each day's total, the week's total and its halves,
  and a sentence below repeats the week's billable, not billable and total, so the two are never merged into one
  number without both being shown. Times are the service's whole minutes written as hours and minutes ("2 hrs 15
  mins"; the units are catalogue plurals, as some browsers have no Welsh units in `Intl`); a day with nothing on a
  plan's row is blank. An empty week (`total: null`, `reason: "no_entries"`) is a sentence, never a row of zeros. The
  entries left out because they are in plans the reader cannot see are counted (`omitted_unreadable`).
- **Status and submitting.** The status in words and a shape (open ○, sent for approval ◔, approved ✓, sent back ✗;
  never colour alone). "Send this week for approval" (`POST /api/timesheets/submit` with `{ week }`) shows on an open
  or sent-back week with time in it ("…again" after a rejection, with the approver's reason shown); a week with no
  time says it cannot be sent; a submitted week says it is waiting; an approved week says it is final and that no
  more time can be recorded in it (the service refuses a new entry against it with 409).
- **Someone else's week (approvers).** A `person:` or `worker:` reference with a UUID, checked on the page as the
  service reads it, then `GET /api/timesheets?week=&person=` for the week chosen above. A submitted week offers
  "Approve" or "Send back" and a note (`POST /api/timesheets/decide` with `person`, `week`, `decision` and, when not
  blank, `comment`); sending back with no reason is refused on the page, and so is deciding one's own week (the
  page compares the id with the `person` the service returned for the caller's own week, ignoring the prefix and
  case, as the service does). The approval is said to be final before it is sent. The decide button is not the
  page's primary action (one primary per screen: submitting one's own week).
- **Refusals.** A 403 on reading someone else's week says only the person or a timesheet approver may read it; on
  submit, that only the owner may send it; on decide, that it needs the approver right and nobody decides their own
  week. A 409 on submit says it was already sent or approved, and on decide that it no longer waits for a decision;
  both read the week again. A 422 on reading one's own week says the sign-in identity has no person reference (a
  token `sub` that is not a UUID); on someone else's, that the week or person was not accepted; on submit, that the
  week has no time in it; on decide, that sending back needs a reason. Each is a catalogue sentence; the service's own
  text is never shown. These routes take no `If-Match`.
- **Not built in this slice:** recording, editing or deleting time entries (`POST /api/plans/{pid}/time-entries` has
  no screen, and the service has no edit or delete route); a list of the weeks waiting for a decision (the service
  has no such query, so an approver must know the person); a person picker or people's names for a reference; time
  reports (`GET /api/time-reports`, RM-10); a link to a week from `?week=` in the address; hiding the approver section
  from people without the right (the page does not know the caller's rights; the service's 403 is shown in words);
  categories and notes of the entries behind a cell.
- **Tests:** `tests/unit/timesheet-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, the weekday heads in
  `tests/unit/locale-format.test.ts`, and `tests/e2e/me-timesheet.spec.ts` (the link from `/me`; the grid with plan
  names, an unnamed plan, the day totals and both halves, and a submit; weeks moved and typed, a refused typed week
  and an empty week; this week in Auckland; a 409, 422 and 403 on submit; a sent-back week and the reason; the
  caller's own sent-back week; a refused person, the caller's own week by another prefix, and an approval; a 403
  and 409 on decide and a 403 on reading; an identity with no person reference; Welsh; axe). The Welsh and the other
  six languages are machine-written and unreviewed.

## Access review and emergency access (NC-1 slice 7, RG-9, RG-10)

`/security/access`, linked from `/security`, has two sections: the access review for recertification, and emergency
("break-glass") access to a private plan. `src/lib/access-view.ts` holds the checks, bodies, states and refusal
sentences; the page only wires them.

- **The access review (RG-9).** Filters: a plan id (blank covers every plan the reader can read) and the review age in
  calendar days (365 when blank, a whole number, 0 or more; the page refuses anything else as the service would).
  `GET /api/access-review?plan=&older_than_days=` is read **only when the person asks** ("Show the review"), never on
  opening the page, because the service audits every read. The page says it is for administrators only, that each
  reading is recorded in the audit trail, and that it lists what people recorded in the access ledger, not what the
  sign-in service enforces. It shows the date (the deployment's today), how many grants stand and how many are due,
  then a table oldest first (person, access, plan named by `GET /api/plans/{pid}` or the start of its id, granted on,
  granted by, age in calendar days, due for review in words and a shape ⚑, note). The table offers no export of its
  own: the recertification artefact is the service's spreadsheet (`?format=csv`, a link through the proxy, formulas
  defused, with an empty keep or revoke decision column), whose read is audited like the page's. The service's
  English `note` is not shown; the page says the same in the catalogue's words.
- **Emergency access (RG-10).** Before the form, a bordered note (`role="note"`) says that every use is recorded and
  audited (opening, with who, the plan, the reason, the length and the person named; and every request the access
  alone lets through), that the person named is told at once and that the plan is not opened if they cannot be, that
  it grants seeing only (what one may change still follows one's own rights), and that it ends by itself, at most five
  a day and one at a time per plan. The form asks for the private plan's id, the reason (at least 20 characters once
  trimmed and at most 1000, counted as the service counts them, with a live count), the length in whole minutes (1 to
  240, 60 filled in, always sent), the person to tell, and a tick that the person understands it is recorded and
  audited and that the person named is told. Every problem is listed at once and nothing is sent until there are none.
  The page is **never looser than the service**: the person to tell must be `person:` or `worker:` with a UUID (the
  service takes any text after the prefix, but can deliver the notice and read the person's choices only for a UUID),
  and the acknowledgement is the page's own extra step. The button is not the page's primary action (the page has
  none), so the control is never the easiest thing on the screen. After `POST /api/break-glass` the page says the plan
  is open until the expiry **in the browser's time zone, which it names**, that the person has been told, and links to
  the plan; the reason and the tick are cleared, so a second use asks again.
- **Grants.** `GET /api/break-glass` (read on opening; it is not audited): the caller's own, or everyone's for an
  administrator, newest first, in the service's order (not sortable). State in words and a shape from the service's
  `live` (in force ●, ended early ■, ran out ○), plan (a link while in force), holder, opened, ends or ended (with who
  ended it), the reason and the person told; times in the browser's zone, named above the table. "End now" (`POST
  /api/break-glass/{pid}/revoke`) on a grant in force, with no confirmation, as ending early only takes access away;
  the list is read again after.
- **Refusals.** A 401 says to sign in; on the review a 403 says administrators only and a 400 or 422 that the plan or
  days were not accepted; on opening a 403 names the eligibility needed, 404 that no plan has that id, 409 that the
  caller already holds access to it (the list is read again), 422 the three service checks the page cannot make (the
  plan is not private, the person to tell is the caller, or that person cannot be told) and that no access was given,
  and 429 the daily limit; on ending a 404 that the grant is not found or not the caller's. Each is a catalogue
  sentence; the service's own text is never shown. These routes take no `If-Match`.
- **Not built in this slice:** recording a reviewer's keep or revoke decision back to the service (it has no route;
  the spreadsheet is the artefact); a link from the access review to the access ledger's own page or to revoking an
  access entry; a plan picker for a private plan (the person must know its id, as they cannot list it); a person
  picker or people's names for a reference; hiding either section from people without the right (the page does not
  know the caller's attributes; the 403 is shown in words); checking on the page that the person to tell is not the
  caller (the page does not know the caller's subject); a countdown or a live refresh as a grant runs out (the state
  is as the service said when the list was read); paging past the service's 500 newest grants; offering emergency
  access from a plan page that answered 404.
- **Tests:** `tests/unit/access-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/security-access.spec.ts` (the link from `/security`; the review read only when asked, two refused
  filters, the totals, the due shape and the spreadsheet link; a 403 on the review; the audited note before the form,
  not primary, every problem at once and nothing sent; opening with the body sent and the expiry in Auckland's time,
  the grant in force, and ending it early; a 403, 404, 409, 422 and 429 on opening, a 404 on ending and a 401 on the
  list; Welsh; axe). The Welsh and the other six languages are machine-written and unreviewed.
