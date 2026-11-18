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

```text
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
                             raise from a risk (NC-1 slice 1); read one
                             as stored, remove one (slice 12 part c)
/plans/[pid]/change-requests the plan's change requests: draft, edit a
                             draft, submit, review, decide, withdraw
                             (NC-1 slice 2); read one, implement, the
                             issues and risks it answers, the what-if
                             forecast, a baseline (slice 12 part c)
/plans/[pid]/tasks/[tpid]    one task: edit its fields, add and remove
                             its links to other tasks (NC-1 slice 3);
                             follow it, react, comment, attach files
                             (NC-1 slice 5); raise an issue from it
                             while blocked (slice 12 part c); each
                             board card's title links here
/plans/[pid]/modules         the plan's tools: which feature modules it
                             shows (NC-1 slice 8); linked from the plan
                             page
/canvases                    strategy canvases of every scope, and a new
                             free-standing one (NC-1 slice 9)
/plans/[pid]/canvases        the plan's canvases, and a new one on it
                             (NC-1 slice 9); linked from the plan page
/canvases/[pid]              one canvas: every box, its items, add,
                             edit, move, remove, turn into an idea, an
                             objective or a risk (NC-1 slice 9)
/plan-templates              saved plan templates and the built-in
                             starters (NC-1 slice 9)
/plan-templates/[pid]        one template: what it holds; start plans
                             from it; remove a saved one (NC-1 slice 9)
/plans/[pid]/save-as-template save the plan as a template (NC-1 slice
                             9); linked from the plan page
/plans/[pid]/export          export the plan to Microsoft Project as an
                             MSPDI file (NC-1 slice 10); linked from the
                             plan page
/director-packs/schedules    the monthly pack schedules: add, move,
                             switch off, run what is due (NC-1 slice 11)
/plans/[pid]/confidence      the plan's delivery confidence: what the
                             map shows, its history, add one (NC-1 slice
                             11); linked from the plan page
/exceptions                  the delivery exceptions register: complete
                             and decide exceptions (NC-1 slice 11)
/portfolio-map               the live portfolio map, strategic aims and
                             each programme's primary aim (NC-1 slice 11)
/plans/[pid]/programme       planned gate dates and programme status
                             (NC-1 slice 11); linked from the plan page
/funding                     funding sources, a plan's lines' sources,
                             money by funding source (NC-1 slice 11)
/plans/[pid]/benefits        a plan's benefits: classification and
                             validation (NC-1 slice 11); linked from the
                             plan page
/deescalation                the de-escalation tracker and the
                             scorecards it reads (NC-1 slice 11)

Also built, described in the sections below (the list above is not
every page; `find src/routes -name +page.svelte` gives all 124):
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
/register-completeness /reviews /search /start-queue /tolerances /tour
/issue-settings

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
`<html>`.
Theme stylesheets are served from `static/assets/themes/` (a symlink
to the shared design-system themes).

**The UI uses the browser's light/dark setting** (owner's rule, 2026-11-15):
with no theme the person picked, the theme is `light` or `dark` as the
browser's `prefers-color-scheme` says, on every visit, and it changes when the
setting changes while the page is open. A theme the person picks that is not
the browser's own is kept (`localStorage`, key `ppm-theme-choice`) and wins
over the setting; picking the theme the browser asks for clears it, so the UI
follows the browser again. The picker is given no storage key, because it
stores every theme it applies, its guess from the browser included, which
made the setting count only on a first visit (`src/lib/theme-choice.ts`). A
value under the old key `lily-theme` is read once: `light` or `dark` there
(most likely that guess) is dropped, any other theme is kept as the choice.
`tests/e2e/theme-follows-browser.spec.ts` and `tests/unit/theme-choice.test.ts`. The app's own colour tokens (`src/app.css`) are
derived from the theme's so that text meets WCAG 2 AA (4.5:1) under both
the `light` and the `dark` theme: `--mxi-color-link` (every link, and text
written in the primary colour) is the theme's primary mixed 70 / 30 with
its text colour (the dark theme's primary as is measured 3.4:1 as link
text); `--mxi-color-primary` (the fill behind primary-content text: primary
buttons, the sign-in button) is the primary mixed 80 / 20 with the theme's
neutral (4.14:1 as is under dark); the danger and success colours are mixed
with the text colour, and the error and success banners sit on a tint of
the page background. `tests/e2e/a11y.spec.ts` audits every page route
under both system preferences.

**Contrast rule (owner's decision, 2026-11-15; RE-11, RE-15).** The `light`
and `dark` themes **must** meet WCAG 2.2 AA (4.5:1) for every text pair of
the app's tokens: a failure there fails the build. Every other theme in the
catalogue stays offered **even where it does not meet 4.5:1**, because a
person who picks one has chosen it; nobody gets one by default. Those themes
are measured and reported, not gated. `tests/e2e/token-contrast.spec.ts`
resolves nine token pairs (text, muted text and links on the page and on
surfaces; text on the primary fill; the danger and success colours) in the
browser for every theme: it fails on `light` or `dark` under 4.5:1 and
records each other theme's failing pairs as a test annotation. The
accessibility statement says which themes are tested.

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

**Unsaved work (RE-13).** The plan form (new and edit) keeps everything typed when a save is refused with `401`: it
says the session ended and links to sign-in in a new tab, so this tab and its contents stay, and the person saves
again. Nothing is written to browser storage. Leaving the form with changes not yet saved asks first (the browser's
own prompt on close or reload, a confirm for a link). Rules: `src/lib/unsaved.ts`; tests: `tests/unit/unsaved.test.ts`,
`tests/e2e/unsaved.spec.ts`.

**Accessibility-tree snapshots (RE-9).** `tests/e2e/structure.spec.ts` holds the `main` region's accessibility tree
for the plan list, a plan's page, the plan form's blank-name error, the not-found page and sign-in, in `en-001` and
`cy-001`. A change to a label, heading, landmark or alert there fails until the snapshot is updated on purpose.

**Hermetic e2e (RE-10).** Every spec takes `test` from `tests/e2e/test.ts`. Its automatic fixture routes the browser
context (popups included) and blocks any request or WebSocket to an origin other than the app and the mock identity
provider (`localhost:4180`), unless a test faked it, then fails the test with the blocked addresses; service workers
are blocked. `tests/e2e/hermetic.spec.ts` proves the rule, that a faked origin passes, and, with `test.fail`, that an
unfaked one fails. The app's own Content Security Policy already refuses a script's `fetch` to another origin, so the
guard matters for navigations, popups, images and anything the policy allows.


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
  owner is a typed `person:` or `worker:` reference). (Deleting and raising from a task were built in slice 12
  part c, below.)
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
  change (`…/baseline`); a confirmation before withdrawing; picking the approver from a list of people. (All but
  attaching what-if edits, the confirmation before withdrawing and the approver picker were built in slice 12 part c,
  below.)
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

## A plan's tools (NC-1 slice 8, RM-7)

`/plans/[pid]/modules` chooses which feature modules a plan shows, over `GET` and `PUT /api/plans/{pid}/modules`. It
is linked from the plan page beside "Edit", always: the route belongs to no module, so turning every tool off cannot
hide the way back. `src/lib/module-settings.ts` holds the rules; the page only wires them.

- **What it says.** Whether the plan has its own choice or follows the deployment's default, each module by a name in
  the reader's language (tasks, time, risks and issues, financials, milestones, governance, the public roadmap), and
  what the personal set and the default hold. A module that is off is hidden from the plan page, and the service
  answers its routes with 404; nothing is deleted.
- **Three changes, as the service takes them.** A chosen set (`{ modules }`, in the service's order; none at all is
  a choice, and the page says what it means before it is sent); the personal set (`{ preset: "personal" }`); and
  back to the default (`{ reset: true }`, offered only when the plan has its own). Choosing exactly the default again
  on a plan that follows it sends nothing and says so, since saving it would stop the plan following a later change to
  the default. The page shows the plan as the service answers.
- **Refusals.** The page does not know whether the caller may change the plan; the service decides, and a refusal or
  a failed load is the catalogue's sentence, never the service's text.
- **Issues follow risks.** The plan page's Issues link now follows the `risks` module, as the service gates
  `/api/plans/{pid}/issues`; it followed `governance` before, so it could show a page the service refused, or hide one
  it served. A unit test checks each link names a module the service has.
- **Not built in this slice:** hiding the page from people who cannot change it; showing which pages each module
  holds (the service's `segments` are API paths, not pages); a setting for the deployment's default (an environment
  setting, `PROJECT_PORTFOLIO_MANAGEMENT_DEFAULT_MODULES`).
- **Tests:** `tests/unit/module-settings.test.ts`, `tests/unit/modules.test.ts`, and `tests/e2e/plan-modules.spec.ts`
  (the link, a chosen set, the personal set and the default, nothing sent for the default again, no tools at all, a
  refusal and a failed load, the plan page hiding a module's links with issues following risks, Welsh, axe). The
  Welsh and the six other languages are machine-written and unreviewed.

## Strategy canvases (NC-1 slice 9, LT-1)

`/canvases` lists every canvas the caller may read and starts a free-standing one; `/plans/[pid]/canvases` lists one
plan's and starts one on it (linked from the plan page, always: canvases belong to no module); `/canvases/[pid]` is one
canvas. They use `GET /api/canvas-templates` and the `/api/canvases` routes. `src/lib/canvas-view.ts` holds the rules,
and `CanvasList.svelte` is shared by the two lists.

- **Names in the reader's language.** The seven built-in templates and their 37 box names come from the catalogue,
  not the service's English titles; a template or box the service adds later shows the service's title. A unit test
  reads the service's `TEMPLATES` table and fails when a template or box has no catalogue name, or the catalogue names
  one the service does not have.
- **A canvas.** Every box of its template, in order, empty ones too ("Nothing here yet": an empty box is itself the
  finding), each with its items in the service's order. An item is added at the end of a box; edited (text) or moved
  to another box, sending only what changed and nothing when nothing did; removed after a question. An item becomes an
  idea, an objective or, on a plan's canvas only, a risk on that plan with a probability and an impact from 1 to 5,
  never defaulted; once only, after which it links to what it became. Removing the canvas asks first, removes its
  items, and goes back to the list it came from; what items became stays.
- **Checks before sending, as the service makes them.** A title that is blank or over 200 characters, a description or
  item over 2,000, blank item text, a risk without both scores, and a full canvas (500 items, when the add forms go).
  Text is sent trimmed. A refusal or a failed load is the catalogue's sentence, never the service's text.
- **Not built in this slice:** reordering items within a box (the service takes a `position`); setting an item's owner
  (`owner_ref` is shown when the service has one, as the reference); editing a canvas's title or description (the
  service has no route); a link from an idea, objective or risk back to the item it came from; plan templates (LT-2),
  built in the same slice, below.
- **Tests:** `tests/unit/canvas-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and `tests/e2e/canvases.spec.ts`
  (starting a free-standing canvas and one on a plan from the plan page, problems said with nothing sent, every box
  shown, add, an unchanged edit sending nothing, a move sending only the box, removing after a question, an idea once,
  a risk only on a plan's canvas and only with both scores, a refusal and a failed load, removing a canvas, Welsh,
  axe). The Welsh and the six other languages are machine-written and unreviewed.

## Plan templates (NC-1 slice 9, LT-2)

`/plan-templates` lists the saved templates the caller may see and the built-in starters (BR-13), each with its number
of plans and whether starting from it needs a start date. `/plan-templates/[pid]` is one template;
`/plans/[pid]/save-as-template`, linked from the plan page, saves a plan as one. They use `GET /api/plan-templates`,
`GET` and `DELETE /api/plan-templates/{pid}`, `POST /api/plans/{pid}/save-as-template` and
`POST /api/plans/from-template/{tpl}`; the plan picker reads `GET /api/plans`. `src/lib/template-view.ts` holds the
rules.

- **What a template holds.** Its plans as an outline in the service's order (a plan whose parent is missing is put at
  the top rather than lost), each with its number of tasks and its milestones as "day N" after the start. The page
  says no person is kept. A starter is marked as built in and offers no removal; a saved one gives its version.
- **Starting plans from it.** A name for the top plan (left out when blank, so the service uses the template's own,
  shown as the placeholder), a start date only when the template carries dates (required then, and checked as a real
  calendar day), and an optional plan to put it under (none means a new top-level plan). The answer counts the plans,
  tasks and milestones made and links to the new top plan.
- **Saving a plan as a template.** A name (needed, at most 120 characters, as the service checks) and an optional
  description. The page says what is kept and that later changes to the plan do not change the template. The answer
  counts what was kept, says how many plans under it were left out because the caller cannot read them, and links to
  the template. A taken name (409) has its own sentence; other refusals are the catalogue's sentences.
- **Removing.** A saved template is removed after a question, and the page goes back to the list; plans already made
  from it stay.
- **Not built in this slice:** revising a template from its plan and the plans made from it (NC-12's
  `POST /api/plan-templates/{pid}/revise` and `GET …/plans`); the starters' names and descriptions are the service's
  English text, shown as it sends them; a task list inside the outline (counts only).
- **Tests:** `tests/unit/template-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/plan-templates.spec.ts` (the two lists; what a template holds; a start date needed and the body sent with a
  name and a parent; a starter sending nothing extra and offering no removal; removing after a question; saving from
  the plan page with a blank name said first and a taken name in the catalogue's words; what was kept and left out;
  Welsh; axe). The Welsh and the six other languages are machine-written and unreviewed.

## Export to Microsoft Project (NC-1 slice 10, PJ-10)

`/plans/[pid]/export`, linked from the plan page beside Import (the service files `export` under no module, so it is
always on; the link sits with import under the tasks module, as import does), offers the plan as an MSPDI XML file
from `GET /api/plans/{pid}/export/mspdi`.

- **What it says.** What the file carries (tasks with their outline, start, duration in working days, milestones,
  constraints, deadline and notes; done or not; the links between tasks with kind and lag; the plan's calendar) and
  what it leaves out, as the service writes into the file itself (people and resources, status other than done,
  sprints and components, effort, custom fields, comments, baselines, links other than "precedes", elapsed-time lags);
  that a plan of more than 5,000 tasks cannot be exported; and that the file has been read back only by this
  service's own importer, not opened in Microsoft Project.
- **The file.** A plain link to the service's own download through the proxy, which relays its
  `Content-Disposition: attachment` and file name, so the browser saves it and the page never draws it. No `download`
  attribute, as for attachments: the service names the file.
- **Not built in this slice:** a readable answer when the service refuses the export (a plan over 5,000 tasks answers
  413, and following the link then shows the service's error, not a catalogue sentence); a choice of what to include.
- **Tests:** the link in `tests/unit/modules.test.ts`, and `tests/e2e/plan-export.spec.ts` (the plan page link, what
  the page says, a download named by the service's header with the page staying put, the link back to import, Welsh,
  axe). The Welsh and the six other languages are machine-written and unreviewed.

## Director pack drafting and issue (NC-1 slice 11, DP-1, DP-12)

`/director-packs` gains a form to draft a pack, and `/director-packs/[pid]` the actions on it, over
`POST /api/director-packs`, `POST …/{pid}/review`, `POST …/{pid}/issue` and `POST …/{pid}/reissue`. The pages do not know
the caller's pack rights (`directorpack=editor` or `issuer`, or `access=admin`); the service decides, and each refusal
is a sentence for that action. `src/lib/pack-actions.ts` holds the rules. This is the first part of NC-1's last slice,
the director-pack and escalation inputs; the rest are listed in `tasks.md` under NC-1.

- **Drafting.** A period chosen as a month, an audience (the service's four, held to its list by a unit test, and
  each active organization as `org:<pid>`), and an optional cut-off, not in the future and not more than 400 days
  old (left out, the service uses now). Every problem is said at once before anything is sent. The page says that
  nothing reaches the audience until someone issues the pack, and opens the new draft.
- **On a pack.** A draft says whether the portfolio office has reviewed it, whether the monthly schedule made it and
  whether it is stale. It offers "Mark as reviewed" (until reviewed) and "Issue the pack"; a scheduled draft not yet
  reviewed has issue disabled and says why. The page says issuing freezes the pack, tells its audience and cannot be
  undone. An issued pack offers a reissue with a change note of at least ten characters, said first, and opens the
  new version.
- **Refusals.** A 403 says which right the action needs (issuer to issue; editor for the rest); a 409 says what it
  means for that action (a pack already exists for the period and audience; already issued or waiting for review;
  not the latest version; already reviewed). Anything else is the general sentence; never the service's text.
- **Not built in this slice:** the monthly schedules, the sweep and audience subscriptions (built next, below);
  discarding a draft (the service has no route); showing audiences in words in the existing list and heading, which
  still show the stored value.
- **Tests:** `tests/unit/pack-actions.test.ts`, the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/director-pack-actions.spec.ts` (drafting with every problem first and an organization as audience; a
  manual draft reviewed and issued; a scheduled draft waiting for review; a reissue with a short note refused first;
  a 403 for issue and for review and a 409 for a draft; Welsh; axe). The Welsh and the six other languages are
  machine-written and unreviewed.

## Director pack schedules and subscriptions (NC-1 slice 11, DP-12)

`/director-packs/schedules`, linked from `/director-packs`, holds the monthly schedules (`GET` and `POST
/api/director-pack-schedules`, `PUT …/{pid}`, `POST …/sweep`); `/director-packs` gains the reader's subscriptions (`GET`
and `PUT /api/director-pack-subscriptions`).

- **Schedules.** Each audience's schedule (one per audience) says whether it is on, its day and its next run, and its
  last run's result in the reader's language when it is one the page knows ("drafted" or "skipped" a period, as the
  service writes them); any other result text is not shown, since it would be the service's English. A schedule is
  added for an audience that has none (a day from 1 to 28, checked by the page; the form has `novalidate`, so a day out
  of range is the page's sentence, not the browser's bubble), moved to another day (nothing is sent for the same day),
  and switched off and on. "Run the due schedules" drafts what is due and says how many were drafted and skipped; the
  page says that a schedule never issues a pack. A reader without editor rights is told which right the page needs.
- **Subscriptions.** On `/director-packs`, a box per audience (the four and each active organization) to be told,
  with a link and never the pack, when a pack for it is issued. A refusal (an audience the reader may not read) is said
  in the catalogue's words and the box is put back, since nothing changed.
- **Also fixed:** the canvas page's promote form (slice 9) now has `novalidate` too: a probability or impact of 6 was
  stopped by the browser's own bubble, in the browser's language, before the page's check could say it.
- **Not built in this slice:** subscribing someone else (an editor may, by `person`); a list of who is subscribed.
- **Tests:** `tests/unit/pack-actions.test.ts` (the day, the last result, the refusals), the paths in
  `tests/unit/ppm.test.ts`, `tests/e2e/director-pack-rhythm.spec.ts` (the schedules with next run and last result; an
  audience with a schedule not offered again; a day of 29 said first and nothing sent; adding, moving, switching off,
  running what is due; a 403 on the list; subscribing and unsubscribing, and a refusal putting the box back; Welsh;
  axe), and `tests/e2e/canvases.spec.ts` (a probability of 6). The Welsh and the six other languages are
  machine-written and unreviewed.

## Delivery confidence (NC-1 slice 11, DP-2)

`/plans/[pid]/confidence`, linked from the plan page among the governance links (the service files
`delivery-confidence` under no module, so the route is always on; the link follows governance), over `GET` and `PUT
/api/plans/{pid}/delivery-confidence`. `src/lib/confidence-view.ts` holds the rules.

- **What it says.** What the portfolio map shows: the latest independent assessment, else the latest self-assessment,
  else "not assessed", which the page says is not the same as green. Each value is its word and a symbol (the director
  pack's), never a colour alone; whether it is independent or self-assessed and when; and whether it is stale. The
  page says a stated value is kept beside the RAG worked out from the plan's data and never mixed with it.
- **Adding one.** A value from the service's five-point scale (held to it by a unit test), who says it (self or
  independent), why (at least ten characters) and an optional date, not in the future; every problem is said at once
  before anything is sent (`novalidate`). Choosing amber/red or red says first that it opens an exception, or worsens
  the open one; the answer then says within how many hours it is to be completed (a plural, as the window is set by
  the deployment). An assessment never replaces one; the history follows, newest first, with each rationale.
- **Refusals.** An independent assessment without `confidence=independent` or admin (403) says so and that a
  self-assessment is open; anything else is the general sentence.
- **Not built in this slice:** the exceptions register (built next, below); the assessor shown by name (the service keeps the
  actor string, which the page does not show); the derived RAG beside it on this page. The date check uses the
  browser's today; the service uses the deployment's, so a date on the edge can be refused by the service.
- **Tests:** `tests/unit/confidence-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, the link in
  `tests/unit/modules.test.ts`, and `tests/e2e/plan-confidence.spec.ts` (the link and "not assessed"; every problem
  first; amber/red's warning and the answer's hours; the history newest first; a refused independent assessment;
  Welsh; axe). The Welsh and the six other languages are machine-written and unreviewed.

## Delivery exceptions register (NC-1 slice 11, DP-3)

`/exceptions`, in the navigation and linked from each plan's confidence page, over `GET /api/exceptions` (with
`status` and `breached`), `PUT /api/exceptions/{pid}` and `POST …/{pid}/decision`. `src/lib/exception-view.ts` holds the
rules; plan names come from `GET /api/plans` (best effort: without them an exception names "a plan" and still links).

- **What it says.** Each exception across the plans the reader can read, newest first, open ones by default: the
  plan (linked to its confidence page), the worst value as a word and a symbol, how often it worsened, when it opened
  and closed, a breach (not completed within the deployment's window, a plural), and while open whether it is complete
  or what is still needed, in the reader's words. The page says an exception is closed only by a recorded decision,
  never by improving confidence. Filters (open, closed or every one; breached only) ask the service.
- **Completing.** The cause, the decision needed, an owner (a person reference) and the owner's role, and a decide-by
  date, filled from what is there. Only parts that changed are sent, trimmed; a blank is never sent, since the service
  would keep what it had; nothing changed sends nothing. A text part of one or two characters is said first not to
  count, as the service judges it.
- **Deciding.** A decision of at least ten characters closes the exception. One a threshold opened (EX-7) closes
  another way, by a threshold decision (see "Triage, thresholds and threshold decisions"); a 422 on a checked decision
  is still read as that case. A 409 is one already closed.
- **Not built in this slice:** the threshold decision (part k); the owner shown by name; a notice when an exception
  breaches.
- **Tests:** `tests/unit/exception-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and `tests/e2e/exceptions.spec.ts`
  (the list with plan, worst value, worsening, breach and what is needed; the filters as sent; completing with only
  what changed, nothing for no change, a short part said first; a decision closing one and a short one said first; a
  threshold refusal; Welsh; axe). The Welsh and the six other languages are machine-written and unreviewed.

## Portfolio map and strategic aims (NC-1 slice 11, DP-4)

`/portfolio-map`, in the navigation, over `GET /api/portfolio-map` (with `period`), `PUT /api/objectives/{pid}/strategic-aim`
and `PUT /api/plans/{pid}/primary-aim`; the objectives come from `GET /api/objectives`. `src/lib/aims-view.ts` holds the
rules.

- **The map.** The live map has the shape of a director pack's `portfolio_map`, so the page draws it with the pack's
  own renderer (`buildPackView`, its map section only): each programme once under its primary aim, confidence a word
  and a symbol beside the derived RAG, and the group of programmes with no primary aim in the catalogue's words, never
  the service's "no strategic aim". A month (or none, for this one) is asked of the service.
- **Strategic aims.** A box per objective, ticked when the map lists it as an aim (every aim is a group of the map, with
  or without programmes, so the map is the list). The page says that unmarking an aim also stops it being any
  programme's primary aim. A refusal is the catalogue's sentence and the box is put back.
- **Primary aim.** A programme and an aim, both from the map; choosing the programme's current primary is said first
  and nothing is sent; the page says a new primary makes the earlier one an ordinary link.
- **Not built in this slice:** the other aims a programme serves (an ordinary link) set here; the trend, which only
  issued packs carry.
- **Tests:** `tests/unit/aims-view.test.ts` (aims and programmes from the map, the checks, the map drawn by the pack's
  renderer with the no-aim group from the catalogue), the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/portfolio-map.spec.ts` (the map, a period asked of the service, marking and unmarking an aim, a primary
  set and its current one said first, a refusal putting the box back, Welsh, axe). The Welsh and the six other
  languages are machine-written and unreviewed.

## Gates and programme status (NC-1 slice 11, DP-5)

`/plans/[pid]/programme`, linked from the plan page among the governance links (`planned-gates` and `programme-status`
are owned by no module), over `GET /api/plans/{pid}/planned-gates`, `PUT …/planned-gates/{gate}` and `GET` and `POST
/api/plans/{pid}/programme-status`; a decision may cite one of the plan's gate reviews (`GET …/gate-reviews`) or
exceptions (`GET /api/exceptions`). `src/lib/programme-view.ts` holds the rules.

- **Planned gates.** A date for each of the six gates (held to the service's list by a unit test), each named in the
  reader's language. Only dates that changed and are set are sent, one call per gate; nothing changed sends nothing.
  The page says a set date can be moved but not cleared (there is no route to clear one).
- **Programme status.** Only a plan of kind Program has one; any other plan says so and keeps its gate dates. The
  status in words; the page says a programme is active until a recorded decision pauses or stops it, and a pause flags
  allocations and deletes none. While paused or stopped it lists what the programme still holds (allocations flagged,
  money planned and actual per currency, benefits; plurals). The changes offered never include the current status, and
  none once stopped, which the page says is final. Choosing stop warns first. A decision needs a reason of at least ten
  characters and may cite a gate review or an exception of the programme. The decisions follow, newest first, with what
  each cited.
- **Refusals.** A 403 says the programme decider right is needed; a 409 that the programme has that status or is
  stopped; a 422 that only a programme has a status and a cited record must be its own.
- **Not built in this slice:** clearing a gate date (no route); the cited review or exception named in the history
  (the page says only which kind it was).
- **Project buffer (DP-7, part f).** For a programme, the page also declares its project buffer over `GET` and `PUT
  /api/plans/{pid}/project-buffer`: a whole number of business days from 1 to 2000, or the rule (half the baselined
  critical work, rounded up), with an optional note; the page says a declaration never replaces an earlier one and
  that a critical-chain buffer plan is used instead where there is one. Its fever-chart reading now is an entry of a
  director pack's `buffer_fever.programmes`, so the pack's renderer draws it (no point is said in the catalogue's words
  for its reason). A refusal by the rule (422) says it needs a schedule baseline with critical work. The declarations
  follow, newest first (plurals). Tests: more in `tests/unit/programme-view.test.ts` and `tests/unit/ppm.test.ts`, and a
  buffer test in `tests/e2e/plan-programme.spec.ts`.
- **Tests:** `tests/unit/programme-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, the link in
  `tests/unit/modules.test.ts`, and `tests/e2e/plan-programme.spec.ts` (the link; gate dates sending only what changed;
  a plan that is not a programme; pausing with a cited review, what is held, stopping and its finality; a 403; Welsh;
  axe). The Welsh and the six other languages are machine-written and unreviewed.

## Funding sources (NC-1 slice 11, DP-6)

`/funding`, in the navigation, over `GET` and `POST /api/funding-sources`, `PUT …/{pid}`, `PUT
/api/plans/{pid}/budget-lines/{line}/funding-source` and `GET /api/portfolio-money` (with `fy` and `combine_classes`); a
plan's lines come from `GET /api/plans/{pid}/budget-lines`, whose rows already carried `funding_source_pid`.
`src/lib/funding-view.ts` holds the rules.

- **Sources.** The owner-supplied list, active and retired. A new one needs a name of 2 to 120 characters, not already
  on the list in any case; both are said first. A source is retired or restored; the page says a retired one can no
  longer be given to a line but the lines that have it keep it. A taken name (409) has its own sentence.
- **A plan's lines.** Choose a plan; each budget line (description, capital or revenue, planned money) has a source
  picker: every active source, and the line's own retired one, marked so, only on that line; "no funding source"
  clears it. A refused choice (422, a retired source) is said and the picker put back to what the line has.
- **Money by funding source.** The service's money section has the shape of a director pack's `funding`, so the page
  draws it with the pack's renderer: capital and revenue apart unless combined, currencies never summed, lines with no
  source counted. A financial year (the year it starts in, 2000 to 2100, or empty for this one) is checked first and
  asked of the service. It is read again after a line's source changes.
- **Fixed on the way:** saving a line's source reloaded the lines, and the reload cleared the notice it had just set;
  only choosing another plan clears it now (the test caught it).
- **Not built in this slice:** renaming a source (no route); a line's source set from the governance page; the
  funding allocations, cost codes and reconcile (EX-12, built next, below).
- **Tests:** `tests/unit/funding-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and `tests/e2e/funding.spec.ts`
  (adding with a taken and a short name said first, retiring and restoring; a line's source set and cleared, a retired
  one only on its line; a refusal put back; the money drawn with a year checked and asked; Welsh; axe). The Welsh and
  the six other languages are machine-written and unreviewed.

## The funding ledger's writes (NC-1 slice 11, EX-12)

`/ledger` (already showing each allocation and what it bought) gains `LedgerWrites.svelte`, over `POST
/api/funding-allocations`, `PUT /api/plans/{pid}/budget-lines/{line}/funding-allocation`, `GET` and `PUT /api/cost-codes` and
`POST /api/ledger/reconcile`; sources from `GET /api/funding-sources`, a plan's lines from `GET …/budget-lines`, whose rows
already carried `funding_allocation_pid`. `src/lib/ledger-actions.ts` holds the rules.

- **Recording an allocation.** A source (active ones), an amount read in the reader's own number format and sent in
  minor units of a three-letter currency (above 0), a date not after today, and an optional reference of at most 120
  characters; every problem is said at once. The page says allocations are kept as recorded and never changed.
- **A plan's lines.** Each line shows its funding source and offers only the allocations of that source and currency,
  as the service requires; a line with no source says it needs one first (set on `/funding`), and "draws on no
  allocation" detaches it. A refused draw (422) is said and the choice put back.
- **Cost codes.** The codes mapped to lines the reader can read, each with its line's description when the line's plan
  is the one chosen above; a code (1 to 60 characters) is mapped to a line of that plan, and the page says mapping a
  code already mapped moves it.
- **Reconciling.** A finance export chosen as a file or pasted, sent as CSV text, after the page checks it is not empty,
  is under 1 MB and names `cost_code`, `amount_minor` and `currency` in its first line. The report says whether it
  reconciled and how many rows were read (a plural), with each matched code's export and register amounts and their
  difference, each unmatched code and amount, and each row in another currency than its line; the page says nothing is
  written. A refused file (422; the service lists its problems by line, in English) is the catalogue's sentence.
- **Why money is untraced** (NC-1 slice 12 part a): the trace's banner is the catalogue's sentence for its
  `untraced_reason_code` (no budget line the reader may read draws on the allocation; money not yet planned on a
  line; lines planning more than was allocated), a general sentence when money is untraced with no code the page
  knows, and nothing when every unit is traced. The service's English `untraced_reason` is no longer shown (slice 11
  showed it). Tests: `tests/unit/ledger-actions.test.ts`, `tests/e2e/ledger.spec.ts` (each code, Welsh).
- **Not built in this slice:** each unmatched code mapped from the report; the service's per-line problems with a file
  shown in the reader's language.
- **Tests:** `tests/unit/ledger-actions.test.ts`, the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/ledger-writes.spec.ts` (recording with every problem first and minor units sent; a line's choices by source
  and currency and one with no source; a refused draw put back; a code mapped; an export's header checked, sent as text
  and its report; Welsh; axe); `tests/e2e/ledger.spec.ts` unchanged and passing. The Welsh and the six other languages
  are machine-written and unreviewed.

## Benefit classification and validation (NC-1 slice 11, DP-9)

`/plans/[pid]/benefits`, linked from the plan page with the financial links (`benefits` is owned by no module), over
`PUT /api/plans/{pid}/benefits/{benefit}/classification`, `GET` and `POST …/validations`; the benefits come from `GET
/api/plans/{pid}/benefits`, whose rows already carried the classification fields, and a plan's KPIs from `GET
/api/kpis?plan=`. `src/lib/benefit-view.ts` holds the rules.

- **What it says.** Each benefit, financial ones with their target and realized (claimed) money. The page says a
  realized amount is a claim, validated only by a recorded validation from someone outside the programme, each
  stating the amount validated to date, the latest counting.
- **Classifying.** Whether it recurs, its yearly amount (shown only for a recurring, financial benefit, read in the
  reader's own number format), its classification and, for a non-financial one, the KPI that measures it; the words
  come from the existing `pk_recur` and `pk_cls` labels, held to the service's lists by a unit test. A field left out
  is cleared by the service, so the page sends the whole classification and says so. Problems are said first.
- **Validating.** A financial benefit's standing in words (not claimed, claimed, partly validated, validated), the
  latest validated amount and the history, newest first; a new validation states the amount to date (not below 0) and
  an optional note. A non-financial benefit says it is measured by its KPI and offers no validation. A validator who is
  a member of the programme (403) is told to ask someone outside it.
- **Not built in this slice:** evidence as an attachment (`evidence_attachment_pid`); the portfolio's benefits view
  (`GET /api/portfolio-benefits`).
- **Tests:** `tests/unit/benefit-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and `tests/e2e/plan-benefits.spec.ts`
  (the link; classifying a financial benefit with the whole classification sent and a yearly amount only when
  recurring; validating with a bad amount said first, the standing and history; a non-financial benefit's KPI; a
  refused validator; Welsh; axe). The Welsh and the six other languages are machine-written and unreviewed.

## Adding and changing skill pools (NC-1 slice 11, DP-10)

`/skill-pools` (already showing every pool and its workforce-service link) gains `SkillPoolWrites.svelte`, over `POST
/api/skill-pools` and `PUT /api/skill-pools/{pid}`. `src/lib/skill-pool-view.ts` holds the rules.

- **Adding a pool.** A name (2 to 120 characters, no other pool of that name in any case), and roles and skills typed
  with commas or new lines, kept as the service keeps them (trimmed, blanks dropped, a repeat once in any case). The
  page says a pool groups roles and skills so the capacity heatmap counts their demand and supply once, and that a
  role or skill is in at most one pool; a role or skill another pool already holds is named, with that pool, before
  anything is sent, from the pools already on the page.
- **Changing a pool.** Choose a pool; its roles and skills are filled in; only a list that changed is sent (the service
  keeps a list left out), and nothing when neither did. A pool is retired or restored; the page says a retired pool
  stays on the record with its roles and skills.
- **Refusals.** The service answers 409 for a taken name and for a role or skill in another pool alike, so the page's
  sentence names both; it is rare, as both are checked first.
- **Not built in this slice:** renaming a pool (the service has no route).
- **Tests:** `tests/unit/skill-pool-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/skill-pool-writes.spec.ts` (adding with a taken name and a clash said first, the lists sent as kept;
  changing only the changed list, nothing for no change; retiring and restoring; a 409; Welsh; axe). The Welsh and the
  six other languages are machine-written and unreviewed.

## A programme's delivery model and reporting method (NC-1 slice 11, EX-2, EX-14)

`/plans/[pid]/programme`, for a plan of kind Program, gains "Delivery model and reporting", over `GET`/`PUT
/api/plans/{pid}/delivery-model` and `PUT /api/plans/{pid}/reporting-method`.

- **Delivery model.** One of the three models (central product; central platform, local deployment; local programme,
  central standards; labelled as the director pack labels them) or not stated, and an owning organization from the
  active organizations, or none. The two are saved together, because the service clears a field left out of the body;
  nothing is sent when neither changed.
- **Reporting method.** Earned value (the default) or burn-up and flow for a product programme; the pack reports the
  programme the chosen way. Nothing is sent when it did not change.
- **Refusals** (a plan that is not a programme, an organization made inactive since the page loaded) are said in the
  catalogue's words.
- **Tests:** the client paths in `tests/unit/ppm.test.ts`; `tests/e2e/plan-programme.spec.ts` (saving the model and
  owner together, only active organizations offered, the method changed, a refusal; the page's Welsh and axe test).
  The translations are machine-written and unreviewed.

## Changing a draft cutover's stop criteria (NC-1 slice 11, EX-6)

`/plans/[pid]/cutover` gains, on a decision still in draft, "Change the stop criteria", over `PUT
/api/plans/{pid}/cutover-decisions/{did}/criteria`. `src/lib/cutover-view.ts` holds the rules.

- **The criteria** are edited one per line, filled in with the draft's own; they are kept as the service keeps them
  (trimmed, blank lines dropped) and sent whole, in the order written. Nothing is sent when the list did not change.
- **The rules are said first,** here and on the new-draft form: 1 to 30 criteria, each 5 to 500 characters (counted as
  characters, not bytes), naming the first line that breaks it.
- **A decision published meanwhile** answers 409; the page says its criteria are frozen and that a new version changes
  them, in the catalogue's words.
- **Not built in this slice:** choosing which organizations must sign a milestone off (EX-4,
  `PUT …/milestones/{m}/signoff-required`). That route replaces the whole set, but no read returns the set: a
  milestone's `signoff.outstanding` drops an organization once it has signed. A screen could not show what is there and
  would drop signed organizations on save. The service will first return the required organizations, with their
  sign-off dates; the screen follows.
- **Tests:** `tests/unit/cutover-view.test.ts`, the path in `tests/unit/ppm.test.ts`,
  `tests/e2e/cutover-criteria.spec.ts` (the rules said first, nothing for no change, the list sent whole and in order,
  a 409, the new-draft form's rule; Welsh; axe on the open editor). The translations are machine-written and unreviewed.

## Triage, thresholds and threshold decisions (NC-1 slice 11, EX-7)

A programme's triage and thresholds are set on `/plans/[pid]/programme` (`ProgrammeThresholds.svelte`), and the
exceptions a threshold opens are answered on `/exceptions`. `src/lib/threshold-view.ts` holds the rules. Nothing is
paused, reset or stopped automatically; every page says so.

- **Triage** (`GET`/`PUT /api/plans/{pid}/triage`): finish, fix, pause or stop, with a reason of at least 10
  characters (said first); the latest and the earlier ones are listed. A triage is a view and changes nothing.
- **Thresholds** (`GET`/`PUT /api/plans/{pid}/thresholds`): each of the three kinds is set, said as a sentence, or
  removed: confidence red in N issued packs in a row (N a whole number from 1 to 24; a number input's value is read as
  a number once typed in), the forecast finish past a date, the P80 cost above a cap (an amount above zero, typed in the
  reader's locale, in an ISO 4217 currency, sent in minor units). Nothing is sent for no change.
- **The sweep** (`POST /api/threshold-sweep`), from the exceptions register: the notice counts the thresholds checked
  and the exceptions opened, and lists each crossed threshold with what happened, in the catalogue's words (opened,
  already open, added to the open exception, not opened under an override); the service's note is not shown.
- **A threshold exception** (its `opened_value` is `threshold`) says it was opened by a threshold, instead of a
  confidence value (which it does not have), and its decision form records pause, reset, stop, or an override with an
  expiry in the future and at most 366 days out (said first; an expiry is sent only with an override).
- **Which threshold** (NC-1 slice 12 part a): every exception now carries `threshold_kind`. A threshold exception
  names its kind in the catalogue's words ("Opened because the programme crossed its threshold: P80 cost above a
  cap."); a confidence exception a threshold joined while it was open says so with the kind; one delivery confidence
  alone opened names none; a threshold exception with no kind (an older answer) keeps the plain sentence.
  `exceptionThreshold` in `src/lib/threshold-view.ts`.
- **Tests:** `tests/unit/threshold-view.test.ts`, the paths in `tests/unit/ppm.test.ts`;
  `tests/e2e/plan-programme.spec.ts` (triage and the three thresholds; the page's Welsh and axe test covers the
  sections), `tests/e2e/threshold-exceptions.spec.ts` (the exception's sentence, an override's expiry rules, a pause
  without an expiry, the sweep; Welsh; axe). The translations are machine-written and unreviewed.

## Scorecards and de-escalation (NC-1 slice 11, EX-10)

`/deescalation`, in the navigation, holds the de-escalation tracker (`DeescalationTracker.svelte`, over `GET
/api/deescalation`, `PUT /api/deescalation/level`, `POST /api/deescalation/criteria` and `PUT
/api/deescalation/criteria/{pid}/evidence`) and the scorecards (`Scorecards.svelte`, over `GET` and `POST
/api/scorecards`, `GET /api/scorecards/{pid}`, `PUT …/{pid}/kpis`, `GET /api/scorecard-template` and `POST
…/{pid}/from-template`; the KPIs to choose from `GET /api/kpis`). `src/lib/deescalation-view.ts` holds the rules. A
change to a scorecard reads the tracker again.

- **The level.** Shown as the sponsor set it, with its date, the sponsor's note and the earlier levels. The page says,
  in its own words, that the level is never worked out here and that nothing on the page moves it (the service's
  `level_note` is not shown). Who set the level and each earlier one (`set_by`, NC-1 slice 12 part a) is shown as the
  service returns it, a reference to a person and not a name, as `decided_by` is elsewhere; a level with no one
  recorded gives only its date. Recording it says first that only the sponsor (role `sponsor`) or an administrator may,
  and that a level is 1 to 60 characters with an optional note; the same level and note is not sent again; a 403 is the
  catalogue's sentence.
- **The run.** The consecutive finished quarters met against the number the rule asks for, and whether the rule is
  satisfied, offered to the person who decides; a deployment setting the service could not use is said, with the
  default it applied. The scorecard the tracker reads is named, or its absence said. Key measures on plans the reader
  may not read are counted, not shown.
- **Each quarter** (a `DataTable`): its state as a word and a symbol (met ✓, not met ✗, nothing recorded ?), never a
  colour alone, and why. The service's `because` sentences are English and are not shown: the page names each key
  measure and criterion not met, and each with nothing recorded, from the detail (`kpis[].quarters[].met` and
  `criteria[].quarters[].met`, null being nothing recorded). The tracker's scorecard is drawn by quarter below
  (`KpiQuarterTable.svelte`: the target as "at least" or "at most" by direction, each quarter's state and reading).
- **Criteria and evidence** are added, never edited or retired; the latest record for a quarter counts (said first). A
  criterion is 5 to 500 characters. Evidence names a criterion and a finished quarter (offered from the tracker's
  quarters, and a quarter not yet finished refused before sending), met or not met (chosen, never assumed), a note of at
  least 5 characters and an optional link (http or https, no spaces, at most 500 bytes). Evidence the latest record
  already says is not sent again. The evidence form is disabled until the tracker has loaded. Each quarter's evidence
  says who recorded it and when (`recorded_by`, `recorded_on`, NC-1 slice 12 part a), the person as a reference.
- **Scorecards.** Listed with the one the tracker reads. A new one has a name of 3 to 120 characters not already used
  in any case, and may be the one the tracker reads only when no other is (both said first). The service's two 409s
  have a code each (NC-1 slice 12 part a): `scorecard_name_taken` is the taken-name sentence and
  `tracker_scorecard_taken` says another scorecard is already the tracker's; a 409 with neither code is the generic
  sentence. A chosen scorecard shows its key measures by quarter and each quarter's state with the reasons built the
  same way. Its key measures are chosen from the KPI list (at most 60; nothing sent for the same set, in any order;
  one on a plan the reader may not read stays on the card, as the service keeps it). The checkboxes are disabled until
  the scorecard and the KPI list have loaded.
- **The template** has no targets and is not applied by reading it. Each measure is named from the catalogue by its
  key (the service's English name is not shown), with its direction and unit; the reader picks measures and sets each
  target and red line as whole numbers (a number input's value read as a number once typed in). The direction rule is
  said first: where higher is better the red line is below the target, where lower is better above it. A KPI made
  from the template carries its `template_key` (NC-1 slice 12 part a) and is named from the catalogue by that key
  wherever this page shows it (the tracker's and a scorecard's tables by quarter, the reasons for a quarter, the key
  measures to choose); a KPI without a key, or with one the catalogue does not know, keeps its stored name. A rename of
  a template KPI is therefore not shown here, as the service spec says. Other pages (`/kpis`) still show the stored
  name.
- **Not built:** retiring or renaming a scorecard, choosing how many quarters a scorecard shows (the service's default
  of four is used), and editing or retiring a criterion (no routes). The template's six keys are named in the catalogue;
  a key the catalogue does not know is shown as its key. **Left:** people are shown by reference, not by name (the
  service stores references); KPIs made from the template before migration `20261118000116` have no key and keep
  their English name (the service does not backfill them).
- **Tests:** `tests/unit/deescalation-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, `tests/e2e/deescalation.spec.ts`
  (the level, run and reasons from the detail with no service text; the level's rule, no change, a 403; criteria and
  evidence with their rules, no change and a 422; a scorecard added, its key measures chosen and measures added from
  the template with the direction rule; Welsh; axe with the forms open). The translations are machine-written and
  unreviewed.

## A plan's phase and a programme's start (NC-1 slice 11, FR-30, EX-8)

`/plans/[pid]/programme` holds the phase section (`PlanPhase.svelte`, over `GET /api/plans/{pid}/phase-history`, `PUT
/api/plans/{pid}/phase` and, for a programme, `GET /api/plans/{pid}/start-check`), beside the gates, the programme
status, the delivery model and the thresholds: it is the page about where a plan is in its life, and until now no
page moved a phase at all. Every plan has a phase, so the section is shown for every plan; only a plan of kind
Program has the start check. `src/lib/phase-view.ts` holds the rules.

- **The phase.** The current phase in words (or that none is recorded yet), and the time in each phase entered, in
  whole days, with how many times it was entered and which is now. The moves offered are the ones the service
  allows: one step forward (only Initiating for a plan with no phase; none after Closing) and every earlier phase. A
  move back needs a reason (said when one is chosen, and refused before sending without one); a reason given for a
  move forward is sent and recorded too. Nothing is sent before a phase is chosen; the current phase is never offered.
  The page says the phase never stops work being recorded.
- **The start check** (a programme only). What a start would meet now, read without moving anything: the places for
  programmes in active delivery taken of the limit (or that no limit is set, or that the deployment's setting could
  not be used), the constraint pool (or that none is identified), whether a start now would go ahead or be refused
  and why (each refusal built from its structured fields: the limit and the count, the pool and its short months in
  the reader's month names), the programme's place in the start queue when it is waiting, with a link to
  `/start-queue`, and its firm demand by skill pool and month in full-time equivalents (a `DataTable`; a dash where a
  pool has none that month). The rule is said first: a refused start joins the queue and keeps its place, and can go
  ahead with an override reason of 10 or more characters, recorded and audited. "Check again" reads it again.
- **The start.** Moving a programme into Executing is its start. Choosing it says first whether the check expects a
  refusal (and that, sent without an override, the programme joins the queue) or that the start may still be refused
  when sent. When the check expects a refusal, or after one, an override reason field is shown; an override shorter
  than 10 characters after trimming is refused before sending (the service would ignore it and refuse the start), and
  an override is sent only on a programme's move into Executing.
- **Refusals.** A refused start (422 `start_refused`) is said in the catalogue's words with the programme's place in
  the queue and the reasons, read from the fields the refusal carries (`errors.refusals`, each tagged by `reason` with
  its numbers, and `errors.queue_position`; NC-1 slice 12 part a, `startRefusedFields`). The start check is shown
  before sending and is not read again after a refusal; a reason of a kind the page cannot word is left out, and a
  refusal without the fields says only that the start was refused. The English `detail` is never shown. Any other 422
  says the three phase rules (one step forward, a reason to go back, not the current phase); a 404 that the plan no
  longer exists; anything else is the generic sentence.
- **Not built:** the transitions themselves, each with its reason (the page shows the time in each phase only); the
  phase anywhere but this page (the plan page does not show it); withdrawing from the queue here (that is
  `/start-queue`); demand set against free supply (the check gives this programme's demand, not the pool's free
  supply).
- **Tests:** `tests/unit/phase-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, and
  `tests/e2e/plan-programme.spec.ts` (the check said first with its reasons and demand; nothing sent before a phase is
  chosen; a short override refused first; a refused start in the catalogue's words with its place and no service
  text; an override that starts it; a start with room; a move back with its reason said first, no start check on a
  project, a 422 in the catalogue's words; a plan with no phase; Welsh; axe with the override field open). The
  translations are machine-written and unreviewed.

## The named trade-off on a new ask (NC-1 slice 11, EX-13)

`/proposals` (over `GET` and `POST /api/proposals`, the pipeline actions and `PUT /api/proposals/{pid}/tradeoff`).
`src/lib/tradeoff-view.ts` holds the rules.

- **Tagging.** A new proposal can be tagged as an external ask from the sponsor; it may name its trade-off (what it
  displaces) at once, in 10 to 2000 characters, or leave it to be named before approval. An ordinary proposal sends
  neither field. The form is `novalidate` and says a missing title before sending.
- **The board** says the rule above the table (an external ask must name a trade-off of 10 or more characters before
  it can be approved) and, in its own column, whether each proposal is an external ask and its trade-off, or that none
  is named yet (words and a ⚠ symbol, never a colour). The pipeline buttons are now the catalogue's words (they were
  the service's action names), each named with its proposal.
- **Naming and changing it.** An external ask not yet approved or rejected offers "Name the trade-off" or "Change the
  trade-off", which opens a form below the table with the rule said first (10 to 2000 characters, until the proposal
  is decided, each change recorded); the same text again sends nothing and says so.
- **Approving.** Approving an external ask without a named trade-off is refused before anything is sent, naming the
  proposal. The service's pipeline 422s have a code each (NC-1 slice 12 part a): `proposal_status_refused` says only
  a proposal in review can be approved (on approving) or that the step does not follow from the status (submitting,
  reviewing, rejecting), and `tradeoff_required` says an external ask needs a named trade-off of 10 or more characters
  first; a pipeline 422 with neither code is the generic sentence. A 422 on the trade-off and on creating a proposal
  says its rules; a 404 that the proposal no longer exists.
- **Not built:** tagging an existing proposal as an external ask, or untagging one (that is the draft update, `PUT
  /api/proposals/{pid}`, which no page calls); the pack's external-asks section is shown by the director pack pages,
  not here.
- **Tests:** `tests/unit/tradeoff-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, `tests/e2e/proposals.spec.ts`
  (the board's column and rule; an approval refused first; naming a trade-off with its rule, nothing for the same text,
  then approving; a new external ask with and without its trade-off; a 422 in the catalogue's words and no service
  text; Welsh; axe with both forms open), and `tests/e2e/ppm.spec.ts` (the board as before). The translations are
  machine-written and unreviewed.

## Who must sign a milestone off (NC-1 slice 11, EX-4)

On `/plans/[pid]/governance`, under the milestones, `MilestoneSignoffRequired.svelte` lists each milestone's partner
sign-off: every required organization and whether it has signed (with the date), or that none is needed. "Change who
must sign" offers the active organizations, and any inactive one the milestone still requires, ticked. Saving calls
`PUT /api/plans/{pid}/milestones/{m}/signoff-required` with the whole set, built from the milestone's
`signoff.required` (on `GET /api/milestones?plan=`), never from `outstanding`, which drops an organization once it signs.
`src/lib/signoff-view.ts` holds the rules, said first: at most 20; an inactive organization cannot be required, so it
must be unticked on purpose (it is not dropped silently); nothing is sent for no change; an empty set clears the
requirement, and the page says so. `/milestone-signoffs` shows and records the sign-offs and now points here.

- **Fixed on the way:** the governance page's new-milestone date input had no accessible name (axe, critical); it
  has one.
- **Tests:** `tests/unit/signoff-view.test.ts`, the path in `tests/unit/ppm.test.ts`,
  `tests/e2e/milestone-signoff-required.spec.ts` (a signed organization kept when another is added; no change sends
  nothing; an inactive one unticked on purpose; clearing; Welsh; axe). The translations are machine-written and
  unreviewed.

## The service's new codes and fields, read (NC-1 slice 12 part a)

The service now answers with a code or a field for each refusal and fact slice 11 could only read as English (service
spec, "Codes and fields a screen reads (NC-1 slice 12, part a)"). Each workaround above is replaced, and no page shows
the service's text:

- `/exceptions` names the threshold that opened or joined an exception from `threshold_kind`.
- `/deescalation` gives each of the two 409s on a new scorecard its own sentence (`scorecard_name_taken`,
  `tracker_scorecard_taken`), shows who set the level and each earlier one (`set_by`) and who recorded each piece of
  evidence and when (`recorded_by`, `recorded_on`), and names a KPI made from the template by its `template_key`.
- `/proposals` gives each pipeline 422 its own sentence (`proposal_status_refused`, `tradeoff_required`).
- `/plans/[pid]/programme` words a refused start from the refusal's `errors` (its reasons and queue place) and no
  longer reads the start check a second time.
- `/ledger` says why money is untraced from `untraced_reason_code`.

**Tests:** `tests/unit/threshold-view.test.ts`, `tests/unit/deescalation-view.test.ts`, `tests/unit/tradeoff-view.test.ts`,
`tests/unit/phase-view.test.ts`, `tests/unit/ledger-actions.test.ts`; `tests/e2e/threshold-exceptions.spec.ts` (each
kind, a joined one, none, an older answer; Welsh), `tests/e2e/deescalation.spec.ts` (who set and recorded, a template
KPI by its key, each 409 code), `tests/e2e/proposals.spec.ts` (each 422 code), `tests/e2e/plan-programme.spec.ts`
(the refusal's own reasons and place, the check not read again), `tests/e2e/ledger.spec.ts` (each code; Welsh). Every
stub carries `"service text"` where the service would put English, and each test checks it is not shown. 13 keys in
eight catalogues and two removed (the sentences that covered two refusals each), machine-written and unreviewed.
**Not verified:** against a real service.

## Status updates on a cadence, with their evidence (NC-1 slice 12 part b, NC-4, BR-2)

`/plans/[pid]/status`, linked from the plan page (`PLAN_LINKS` key `status`, with the governance links: the service
files the status routes under no module, so they are always on), over `GET /api/plans/{pid}/status`, `GET` and `PUT
/api/plans/{pid}/status-cadence`, `GET` and `POST /api/plans/{pid}/status-updates` and `GET /api/status-metrics`; the
records a claim may cite come from the plan's tasks, risks, milestones and issues (`GET …/tasks`, `…/risks`,
`…/milestones`, `…/issues`, each best effort: a kind that cannot be read is not offered). `/dashboard` lists the plans
whose update is due from `GET /api/status-updates/due`. `src/lib/status-view.ts` holds the rules;
`StatusUpdateForm.svelte` posts an update. A status update is a person's call; the page says nothing works out a score.

- **Where the plan stands.** The latest update: its call as a word and a symbol (on track ✓, at risk !, off track ✗;
  never a colour alone), when it was posted and by whom (a person's reference as the service returns it, not a name),
  and its narrative; or that none has been posted. Against the cadence: how often an update is expected (a plural)
  and the service's standing, said as overdue by so many days (a plural), due today, or the date the next is due; a
  plan with no cadence says no update is ever due. The date is the service's `as_of`, in the deployment's time zone.
  Nothing is worked out here.
- **The cadence.** Said first: 1 to 92 days; setting it restarts the clock from today, so the same cadence is not sent
  again; a cadence cannot be removed once set. Who set it and when are shown. The number input binds a number and is
  disabled until the cadence has loaded (a 404 is a plan with none); a value outside 1 to 92 or not whole is refused
  before sending; a 422 is the same sentence.
- **Posting an update.** Said first: the call and a narrative of up to 4000 characters; an update cannot be changed or
  removed once posted, and posting one restarts the clock; up to 10 claims, each a sentence of up to 500 characters
  citing up to 5 sources, each checked by the service when posted. A claim's sources are the four published numbers
  (checkboxes) and the plan's records (a list grouped by kind, with "Add the record"; each chosen record can be
  removed). Every problem is said at once before sending, naming the claim it is on; an empty claim (no sentence, no
  source) is left out; the narrative and claims are sent trimmed and each source once. The form is disabled until the
  plan's status has loaded. A 422 says a source may no longer be a record of this plan (the page checks everything else
  first); a 404 that the plan no longer exists.
- **The history**, newest first: each update as above, with its claims and each source as the service resolves it for
  the reader now: a record's kind and its own title, a number's value now ("unknown (not zero)" when it cannot be
  known), or "a source that is no longer available". The page says updates are never changed and that sources are
  read as they are now.
- **How the numbers are counted.** Each number the service publishes is named and defined in the catalogue's words
  by its key (the "due soon" window from the service's `due_soon_days`); the service's English `label`, `definition`
  and `note` are not shown, nor a number's `definition` inside a claim.
- **The due list** on `/dashboard`: each plan the reader may read whose update is due, most overdue first, linked to
  its status page, overdue by so many days or due today, with its last call and date or that it has none. A 403 or
  404 shows nothing; any other failure is a short sentence.
- **Decisions taken here (the plainest option):** the page offers tasks, risks, milestones and issues as sources, not
  change requests or cutover decisions (the service accepts them; a claim already citing one is shown as a record);
  records are chosen from a list rather than typed as `kind:<pid>`; the link sits with the governance links.
- **Not built:** removing a cadence (no route, NC-4); claims in the director pack; reminders (`POST
  /api/status-updates/remind` is a sweep, part p); the people as names. **Service gaps:** every 422 on posting an
  update is `unprocessable` with English text, whichever rule it is (the call, the narrative, the claims, a source not
  a record of the plan), so the page checks the rules first and reads a 422 as the one it cannot check; the
  numbers' labels and definitions are English only (the page uses its own words by key).
- **Tests:** `tests/unit/status-view.test.ts`, the paths in `tests/unit/ppm.test.ts`, `tests/unit/modules.test.ts` (the
  link), `tests/e2e/plan-status.spec.ts` (the link from the plan page; where the plan stands and the history with each
  kind of source and no service text; the cadence's rule first, nothing for the same cadence, out of range refused,
  sent as a number; a plan with no cadence and a 422; an update's rules first, a claim's sentence needed, numbers and a
  record cited, the body sent; a 422 in the catalogue's words; the dashboard's due list; Welsh; axe with a claim open).
  The translations are machine-written and unreviewed.

## Issues, change requests and risks, the rest (NC-1 slice 12 part c, IR-3, IR-4)

The issue, change-request and risk routes that still had no screen, on the pages they belong to. Every rule is said
before anything is sent; refusals are the catalogue's sentences by route and status; the service's text is never shown.

- **`/plans/[pid]/issues`.** "Edit" reads the issue as stored now (`GET /api/plans/{pid}/issues/{issue_pid}`) before
  filling the form, because the save replaces every field; above the form it says when it was raised and by whom (a
  reference, not a name), the risk it was raised from (by the risk's own title), a link to the blocked task it was
  raised from, a link to the change requests when one answers it, and when it was resolved. "Remove" (`DELETE …`) asks
  first (Cancel sends nothing); the page says first that removing cannot be undone here and that, under a workflow that
  gates moves or fields, only an administrator removes one: on a plan whose issue actions are not role-checked, a
  caller who is not an administrator is told so for such an issue and nothing is sent. The button is offered only to a
  caller holding `issue_delete` when the options say. A 403 says who may remove; a 404 says the issue is gone and reads
  the list again.
- **`/plans/[pid]/tasks/[tpid]`.** A section raises an issue from the task (`POST
  /api/plans/{pid}/issues/from-task/{task_pid}`). Said first: only a blocked task, once while that issue is open; the
  issue is a blocker of medium severity owned by the task's assignee; the task is not changed. The plan's issues are
  read (best effort) so an open issue already raised from the task is named instead of the button; a task not blocked
  says so and offers nothing; the button is disabled until the issues have been read. A 409 (one already open) reads
  the issues again; a 422 (no longer blocked) and a 404 read the task again; a 403 says the caller may not raise issues.
- **`/plans/[pid]/change-requests`.** "Steps" reads the one request as stored (`GET …/change-requests/{cr}`, with the
  issues and risks it answers) and shows its stated impact and implementation date. An approved request is marked
  implemented (`POST …/implement`; the state machine's step, offered beside withdraw). **What it answers** (`PUT
  …/links`): the plan's issues and risks as checkboxes, read once and disabled until read; said first: only while the
  request is open (draft, submitted, under review, approved), an issue answers one change request at most, up to 200
  issues and 200 risks, and saving replaces the whole set. An issue already answering another request cannot be ticked;
  the sets are sent once each; nothing is sent for the sets the request already has. A closed request says it cannot be
  relinked. **The forecast** (`GET …/preview`): offered only when the request carries what-if edits
  (`has_forecast_edits`; otherwise said, with no call); shown labelled a forecast in which nothing was changed: the
  plan's finish now and with the change, the tasks whose finish moves (their own titles, furthest first, in business
  days, a plural), and the planned labour cost's change per currency or why it is not shown (no access to rates, no
  costed work). The service's `label` and `note` are English and are not shown. **A baseline** (`POST …/baseline`):
  said first that it is a person's choice, one per change, and cannot be undone; asked before sending; offered only for
  an approved or implemented request with none taken; the version taken is said, then that one has been taken.
- **`/plans/[pid]/governance`.** Each risk row has "Change", which opens `RiskEditForm.svelte` below the table (`PUT
  /api/plans/{pid}/risks/{risk_pid}`): status, likelihood, impact, mitigation and review date. Said first: likelihood
  and impact are whole numbers from 1 to 5; the mitigation is at most 1024 bytes (the service counts bytes); a review
  date can be changed but not removed; only what changes is sent; "materialized" is recorded by Escalate (which stamps
  when), so it is offered only for a risk already materialized. Every problem is said at once before sending; the score
  inputs bind numbers; nothing is sent for no change. A 422 is one sentence (the service's 422 here has one code for
  every rule); a 404 says the risk is gone. The risk table now names each status in words (`l.enum.risk_status.*`).
- **Decided (the plainest option):** the change request's links are checkboxes, not a search; the forecast lists the
  tasks whose finish moves rather than every date; removing an issue needs a confirmation, implementing does not;
  "materialized" stays Escalate's.
- **Not built:** attaching or editing a request's what-if edits; a confirmation before withdrawing; the people as
  names; showing the baseline itself (it is on `/plans/[pid]/schedule`). **Service gaps:** the issue raised from a task
  is titled by the service in English (`Blocked:` and the task's title), so that word is shown in every language; the
  two 409s on relinking (a closed request, an issue answering another) share the code `conflict`, as do the two on the
  baseline (not approved, already taken), so the page checks both first and one sentence covers both; every 422 on
  changing a risk is `validation` with English text; no route clears a risk's review date or mitigation to nothing
  (an empty mitigation is sent as empty text).
- **Tests:** `tests/unit/issue-view.test.ts`, `tests/unit/change-request-view.test.ts`, `tests/unit/risk-edit-view.test.ts`,
  the paths in `tests/unit/ppm.test.ts`; `tests/e2e/plan-issues.spec.ts` (editing reads the stored issue and says where
  it came from, with axe; removing after a question, Cancel sending nothing; the administrator-only rule said first,
  nothing sent; a 403; an issue gone; Welsh), `tests/e2e/plan-task-edit.spec.ts` (a blocked task raises one issue, the
  rule first, with axe; a task not blocked; a 409 and a 422; Welsh), `tests/e2e/plan-change-requests.spec.ts`
  (implement and a baseline after a question; the links' rules first, one answering elsewhere disabled, nothing for no
  change, the sets sent; a refused relink and baseline; the forecast labelled, no service text, with axe; a request
  with no edits asks nothing; a refused forecast; Welsh), `tests/e2e/risk-edit.spec.ts` (the rules first, nothing for
  no change or out of range; only the changed fields sent, a score as a number; a 422 and a 404; Welsh; axe). Every
  stub carries `"service text"` where the service would put English, and each test checks it is not shown. The
  translations are machine-written and unreviewed. **Not verified:** against a real service.

## Escalation follow-through and register completeness (NC-1 slice 12 part d, CF-4, EX-1)

CF-4's records of an escalation's acknowledgement and resolution, on the page that already works escalations, and a
page for EX-1's register completeness. Every rule is said before anything is sent; refusals are the catalogue's
sentences by action and status; the service's text is never shown.

- **`/plans/[pid]/escalations`.** Acknowledging and resolving now go through the records (`PUT
  /api/plans/{pid}/escalations/{epid}/acknowledgement` with `{note?}`, `PUT …/resolution` with `{outcome, response}`)
  instead of the older `POST …/acknowledge` and `…/resolve` (which stay in the client and the service; taking it up a
  level and withdrawing still use theirs). Opening an escalation reads its records as well as its history: the
  acknowledgement (`GET …/acknowledgement`) when it is acknowledged, resolved or withdrawn, and the resolution (`GET
  …/resolution`) when it is resolved; an open one has no acknowledgement at its level, so nothing is read for it. The
  acknowledgement says the level, who (a reference, not a name; "not recorded" when the service has none), when and
  the note; one not acknowledged at its current level says so. The resolution says the outcome in words, who, when
  and the response, and that it is a record that is not changed. Said first, above the actions: acknowledging needs
  the right to acknowledge escalations and resolving the right to resolve them (an administrator holds both), and the
  page cannot tell beforehand who holds them; an acknowledgement counts at the current level, and taking it up a level
  clears it; a resolution is a record, never overwritten. The acknowledgement takes an optional note (a blank note is
  not sent). A resolution's response is 10 to 10,000 characters and its outcome one of the three; both are checked
  before sending, and the page asks before recording it (Cancel sends nothing). Acknowledge is offered only while it
  is open and the resolution form only while it is live, so a repeat is never sent. A 403 says which right is needed;
  a 409 says it has moved on (resolved or withdrawn meanwhile, or, for a resolution, already resolved differently) and
  reads it again; a 422 says the outcome or response was not accepted; a 404 says it is gone and reads the list again.
  A record that cannot be read says so in the catalogue's words.
- **`/register-completeness`**, linked from `/director-packs` and `/exceptions`, over `GET /api/register-completeness`:
  each programme the caller can read (a plan of kind Program), incomplete ones first and the least complete first;
  its state as a word and a shape; the share of applicable fields on record as a percentage and as a count ("7 of 9");
  every missing field named and every field not asked of it named, in the catalogue's words (the ten field keys of
  `fields`; one the page does not know is "another field"); the name linking to the plan. Said first: what the ten
  fields are, that the programme's lead stands for the senior responsible owner, that partners are asked only where
  the delivery model has partners and a funding source only where there is a budget line. Whether "no register, no
  money" is on is said from `enforced`: on, releasing a budget line or booking a firm allocation on an incomplete
  programme is refused, naming the fields; off, the action goes ahead and the warning is kept in the audit trail. A
  count says how many of the programmes are complete (a plural). The service's `note` is English and is not shown.
- **Decided (the plainest option):** the records replace the older acknowledge and resolve calls on this page rather
  than sitting beside them; the records are read when an escalation is opened, not for every row; the register page is
  read-only and is not in the navigation (two links reach it).
- **Not built:** offering or hiding acknowledge and resolve by the caller's rights (no service read says what the
  policy allows); the people as names; a sentence of its own for the "no register, no money" refusal on the pages that
  release a budget line or book a firm allocation (they show the general 422 sentence); a filter on the register page
  beyond the table's own. **Service gaps:** a 404 on reading an acknowledgement or a resolution has the code
  `not_found` both for "none yet" and for an escalation or plan that is gone, so the page reads a 404 there as "none"
  and relies on the history read just before it; the 409s on resolving (withdrawn, already resolved differently) and
  on acknowledging share `conflict`, and every 422 is `unprocessable` with English text, so one sentence covers each;
  the register's `note` is English.
- **Tests:** `tests/unit/escalation-view.test.ts` (which records to read, the resolution's rules, the note's body, the
  refusal keys, the records' words), `tests/unit/register-completeness-view.test.ts`, the paths in
  `tests/unit/ppm.test.ts`; `tests/e2e/escalations.spec.ts` (acknowledging with a note and the record shown, with axe;
  the resolution's rules first and nothing sent, the question, Cancel sending nothing, the record shown and no form
  after; each refusal in the catalogue's words; a record that cannot be read; Welsh) and
  `tests/e2e/register-completeness.spec.ts` (the switch on and off, the fields named, the order, the count, the links
  from both pages, with axe; a failed read; none; Welsh). Every stub carries `"service text"` where the service would
  put English, and each test checks it is not shown. The translations are machine-written and unreviewed. **Not
  verified:** against a real service.
