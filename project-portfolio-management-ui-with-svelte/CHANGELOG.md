# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> See also: [spec/index.md](./spec/index.md), [README.md](./README.md), [AGENTS.md](./AGENTS.md).

## [Unreleased]

### Fixed — money is scaled by the ISO 4217 minor unit, as the service scales it (2026-10-30)

`src/lib/money.ts` took the exponent from `Intl` (CLDR), which differs from ISO 4217 for 16 codes (AFN, ALL, COP, HUF, IDR, IQD, IRR, KPW, LAK, LBP, MGA, MMK, PKR, SOS, SYP, YER), so a forint amount was stored at a hundredth of what the service reads (a thousandth for IQD). The exponent now comes from `MINOR_UNITS`, the service's `src/currency.rs` table, with `Intl` as the fallback for a code not in it; `money()` sets `minimumFractionDigits` and `maximumFractionDigits` to the exponent. The header comment names the three family `money.ts` files that share the rule. Tests: `tests/unit/money.test.ts` (the table against `currency.rs`, the 16 codes, the fallback, round trips for HUF, IDR, COP, JPY, KWD, CLF). **Not done:** no migration of amounts already stored (see the root `CHANGELOG.md` for what an operator should check); WPM's copy not changed. **Not verified:** against a running service.

### Added — the capacity commitment form and the send outcome (FB-6 completed, 2026-10-30)

`/capacity-commitments`: the form for `PUT /api/capacity-commitments` (skill pool, programme, supply or demand, optional partner organization, month, committed and optional delivered full-time equivalents in the reader's own number format through `parseAmount`, note), every wrong field named before anything is sent, 404 and 422 in the catalogue's words. After saving a demand commitment the page says in plain words what happened to the demand sent to the workforce service (`peer_demand_send`), including the withdrawal at zero (FB-5); "Recent sends to the workforce service" lists the latest send per programme, pool and month from `GET /api/capacity-commitments`. Reached from `/capacity-heatmap` and from a programme's plan page (`?plan=`); the heatmap's "no form" note is gone. `PpmClient.capacityCommitments` / `setCapacityCommitment`. English and machine-written Welsh (unreviewed, in `governance/welsh-translation-review.csv`). Tests: `tests/unit/capacity-commitments-view.test.ts` (9), `tests/e2e/capacity-commitments.spec.ts` (3, axe, Welsh). **Not built:** no edit or delete of an existing commitment from a list (setting the same pool, programme, organization, kind and month again replaces it); the programme list is every plan the reader can list (the list carries no kind), so a plan that is not a programme is refused by the service (422) rather than left out; not in the top navigation; the send's answer is seen by pressing Refresh, not pushed. **Not verified:** against a running service or WPM (the browser tests stub the service; FB-8); a screen reader; CI. `.banner.success` fails colour contrast under the default theme (found here; this page uses the neutral banner, the others are not changed).

### Changed — removing a pool's link says its demand figures are withdrawn (FB-5, 2026-10-30)

The confirmation on `/skill-pools` said demand already sent to the workforce service stays there; the service now withdraws it (sends zero for each programme and month), so the confirmation says so, and after the removal the page says how many withdrawals the service started (`peer_demand_withdrawals`). English and machine-written Welsh (unreviewed, in `governance/welsh-translation-review.csv`). Test: `tests/e2e/wpm-bridge.spec.ts` (the removal test). **Not built:** the outcome of each withdrawal is not shown on this page. **Not verified:** against a running service or WPM (the browser test stubs the service).

### Added — the skill-pool link and the capacity heatmap screens (FB-6, 2026-10-30)

`/skill-pools`: every pool with the workforce service's pool it is linked to, and a per-row "Change link" form that sets or removes it (`PUT /api/skill-pools/{pid}/peer-link`). The id is checked on the page as a lower-case hyphenated UUID before anything is sent; 409 (already linked) and 422 (refused) are said in the catalogue's words; removing is confirmed first; the page says the link is set on both sides by a person who can see both and is never matched by name. `/capacity-heatmap`: `GET /api/capacity-heatmap` pool by month, each square with the service's word and shape as well as a tone, figures in the reader's locale, and for a linked pool the source, the reading (live, last received with its age, hidden, unknown with the reason, never zero) and "Entered here" beside it, never added; every figure also in a `DataTable`. Both in the top navigation. `src/lib/capacity-heatmap-view.ts`, `PpmClient.setSkillPoolPeerLink` and `capacityHeatmap`, `SkillPool.peer_pool_ref`, the `supply unknown` label (which the director pack now also uses for that state instead of the bare word), `tests/unit/capacity-heatmap-view.test.ts` (10), `tests/e2e/wpm-bridge.spec.ts` (5, with axe); the two routes are also in the axe-over-every-route and Welsh-every-route runs. Welsh is machine-written and unreviewed (listed in `governance/welsh-translation-review.csv`); other locales fall back to English; the service's reasons are shown in English. **Not built:** the demand send outcome (`peer_demand_send`): no screen sets a capacity commitment, so there is nowhere to show it; pools are not created or edited here. **Not verified:** against a real service or workforce service (the browser tests stub the service); not checked in a screen reader; the heatmap is a wide table that scrolls sideways on a phone.

### Added — people impact on a programme's plan page (FB-4, 2026-10-30)

`PeopleImpact.svelte` on `/plans/[pid]` for a plan of kind Program: asks `GET /api/plans/{pid}/people-impact` on a button press and shows WPM's answer as it came (as-of date, end date sent or why not, counts by pool, hidden as "hidden (fewer than N)", unknown parts with WPM's reason, no total), or only the reason when `unknown`. `src/lib/people-impact.ts`, `tests/unit/people-impact.test.ts`, `tests/e2e/people-impact.spec.ts` (axe). Welsh is machine-written and unreviewed; other locales fall back to English; WPM's reasons are shown in English. Not verified: against a real service and WPM (the e2e stubs the service); not checked in a screen reader.

### Added — the API client is checked against the service's routes (CQ-9, part, 2026-10-22)

`tests/unit/client-routes.test.ts` reads every `this.http.*` call in `src/lib/api/*.ts` (292), works out each path it can produce (choices written in the call are expanded, and so is a parameter typed as a union of words such as `action: "submit" | "approve"`), and requires each to be in the service's `api-routes.txt` with the same method, a value only where the route has a parameter. All 292 match today; a misspelt path and an invented action both fail it. Not done: request and response bodies and types are not checked (types are still hand-written); `ppm.ts` is not split.

### Changed — one rule for error text (CQ-20, part, 2026-10-22)

159 copies of `err instanceof Error ? err.message : t("…")` in 88 files are now `errorText(err, "…")` (`src/lib/error-text.ts`). One change of behaviour: a server fault (5xx) shows the page's own catalogue message instead of the server's English "Internal Server Error", so a Welsh reader sees Welsh; a refusal the service explains (4xx) still shows the service's reason. Unit test `tests/unit/error-text.test.ts`; check, vitest and the full Playwright run pass. Not done: 15 variants that fall back to `""` or `String(err)` were left; 4xx reasons are still the service's English; the `resource()` load helper is not built.

### Fixed — best-effort reads say when they fail (CQ-19, 2026-10-22)

Four pages swallowed a failed read: the WBS link counts, the plan page's tool list (every tool shown), the Gantt working calendar (no shading) and the announcements page, which fell back to the public feed on **any** error. They now stay quiet only for a 404 or 403 (`src/lib/absent.ts`, also used by the handling-mark banner) and otherwise show a note; announcements fall back only when the full list is refused, and show the error otherwise. `hooks.server.ts` set no security headers when a response's headers were immutable; it now copies the response and sets them on the copy. Browser tests (`tests/e2e/best-effort.spec.ts`, all four fail on the old pages) and a unit test for the headers (fails on the old hook). Welsh machine-written, unreviewed.

### Fixed — a key result's delete question in the reader's language (CQ-23, 2026-10-21)

`ObjectiveCard` asked `Delete “…”?` in English whatever the locale; it now uses `c.ObjectiveCard.delete_the_key_result` (Welsh machine-written, unreviewed). The catalogue test walks only template text, so it could not see a dialog's argument; a new check fails on any `confirm`, `alert` or `prompt` called with a literal (it found this one when the old line was put back). Not done: the native dialogs are still native (no Lily dialog); unused catalogue keys and the `as never` key casts were not touched.

### Fixed — the plan page follows a link to another plan (CQ-18, 2026-10-21)

Following a "Potential duplicates" link from `/plans/A` to `/plans/B` reuses the page, which had read the plan id once and loaded in `onMount`, so plan A stayed on screen under plan B's address. The plan page, its edit page, the sprint board and the flow page now take the id reactively and reload when it changes (the board also forgets the last plan's sprint; the edit form is rebuilt). `tests/e2e/plan-navigation.spec.ts` follows a duplicate link and expects the second plan; it fails on the old page. Not done: 25 other pages under a `[pid]` route still load in `onMount`; none of them links to another plan of the same route, which is why they were left, and they were not tested.

### Fixed — a filter change loads once, and a late answer cannot win (CQ-17, 2026-10-21)

The delivery calendar (`/calendar`) and the flow page (`/plans/[pid]/flow`) loaded both from an `$effect` that tracked the filter and from the filter's change handler, so each change sent every request twice and the two could land out of order. The handler call is gone, an older answer is dropped when a newer one is on its way, and the error clears on success. The sprint board's six reads run in two parallel batches instead of one after another. `tests/e2e/load-once.spec.ts` counts the requests per change (and fails, 3 for 2, with the old handler call put back). Not done: the other pages that load in `onMount` were not reviewed for the same race (CQ-18, CQ-20).

### Removed — unused Lily picker pins (CQ-16, 2026-10-21)

`svelte-link-picker`, `svelte-locale-picker`, `svelte-search-picker`, `svelte-text-size-picker` and `svelte-theme-picker` were direct dependencies that no file imports: the pickers on screen come through `svelte-picker-bar`, which carries its own (older) copies, so the pins only made the bundle look newer than it is. `svelte-headless` stays (a peer of the grid, calendar, Gantt and board) and `themes` stays (`static/assets/themes` links into it). Verified: check, vitest, Playwright, build. Not done: the lockfile still resolves three `svelte-headless` versions, because `svelte-picker-bar` 0.4.0 pins older pickers; a newer picker-bar would collapse them.

### Security — a failed handling-mark lookup says so (CQ-19, 2026-10-21)

On a plan page, a failed `GET /api/plans/{pid}/handling-mark` used to show no banner, which reads as "unmarked". A 404 or 403 still shows none (the plan is not there for this reader), but any other failure now shows "Handling mark: could not be checked. Treat this plan as marked until it is." (`p._layout_svelte.handling_mark_unknown`; Welsh machine-written, unreviewed). The RG-7 browser test that asserted no banner on a 500 now asserts the notice, and a new one covers 404. Not done: the other silent failures in CQ-19 (link counts, modules, calendar shading, announcements, security headers).

### Security — the proxy refuses cross-site writes and the browser's own bearer (CQ-5, 2026-10-21)

`/api/proxy` forwarded every browser header except four, so a page script without a session could send its own `Authorization` to the service, and nothing but the SameSite=Lax cookie stood between another site and a state-changing call. It now drops `Authorization` from the browser, refuses with 403 a `POST`/`PUT`/`PATCH`/`DELETE` whose `Origin` is not the app's (or, with no `Origin`, whose `Sec-Fetch-Site` names another site), and answers 502 `bad_gateway` instead of an unhandled 500 when the service is unreachable. AGENTS.md and the spec claimed an `X-CSRF-Token` the client never sent; corrected. Unit tests (`proxy-guard.test.ts`) and two Playwright tests against the real proxy route. Not done: the 502 path has no browser test (a test would need the service address to be known-closed, which a shared machine cannot promise); a write with neither header passes, as non-browser clients do.

### Fixed — the proxy relays the response headers the client reads (CQ-4, 2026-10-21)

`/api/proxy` copied only `content-type`, so in the browser `getPage` totals fell back to the page length and `ETag`, `Retry-After`, `Location`, `Content-Disposition` and `X-Handling-Mark` were lost. An allow-list in `src/lib/server/proxy-headers.ts` now relays them; unit tests for what passes and what does not (`set-cookie`, hop-by-hop, unnamed). Not done: no browser test goes through the real proxy (the e2e suite stubs the API in the browser), so the fix is proven at the function, not end to end.

### Added — link counts on the work breakdown (LT-8, 2026-10-21)

`/plans/[pid]/wbs` shows "· N links" beside a task and a Links column, from `taskLinkCounts`. Browser test stubs the counts. Not on the board. Welsh machine-written, unreviewed.

### Added — "Raised automatically" on escalations (FR-54, 2026-10-21)

The plan escalations page and the register mark an escalation raised by the `tolerance_escalations` task. `automaticText` unit test and register browser test. Welsh is machine-written, unreviewed.

### Changed — framework description editable (FR-52, 2026-10-21)

A Description field on the ways-of-working and governance forms. Welsh machine-written, unreviewed.

### Changed — shared frameworks editable and deletable (FR-52, 2026-10-21)

Edit a shared record after confirming how many other plans it reaches; delete one nobody follows. Fixed number inputs crashing the framework form check. Welsh machine-written, unreviewed.

### Changed — framework lists are editable (FR-52, 2026-10-21)

Ceremonies, definitions of ready/done, decision rights and forums can now be edited on the framework pages. Not built: editing shared records, the description. Welsh machine-written, unreviewed.

### Added — schedule tolerance (FR-53, 2026-10-21)

The tolerance pages and the escalation form know the `schedule` dimension ("Schedule against baseline"). Welsh machine-written, unreviewed.

### Added — task page with links (LT-8, 2026-10-21)

`/plans/[pid]/tasks/[tpid]` shows a task and its links (shared `LinksPanel` component). Not built: editing the task, links on the board. Welsh machine-written, unreviewed.

### Added — ways of working, governance framework and links pages (FR-52, LT-8, 2026-10-20)

`/plans/[pid]/ways-of-working`, `/governance-framework` and `/links`, from the plan page. Not built: editing ceremonies, definitions, decision rights and forums (kept on save, not editable); editing shared records. Welsh machine-written, unreviewed.

### Added — escalate red tolerances (FR-54, 2026-10-20)

A button on a plan's tolerances page raises an escalation for every red dimension that has none. One browser test with an axe audit; Welsh strings machine-written, unreviewed.

### Added — escalations (FR-54, 2026-10-20)

`/plans/[pid]/escalations` raises an escalation (title, why, which tolerance, level, who it goes to, the person it is for, an answer-by date) and works the ones raised: acknowledge, take it up a level (a reason of ten characters is required), resolve (an outcome and a written response) or withdraw, each with its history. `/escalations` lists every escalation on the plans you can read, with a filter for those still open and for the overdue. A red tolerance with no escalation says so and links to a pre-filled form. Each state is a word and a shape; overdue is said in words. Tests: 3 unit groups, 3 e2e (one with an axe audit of each page). Welsh strings are machine-written, unreviewed. Not done: it does not suggest the level or the body from the governance path (the service fills those in when left blank); no notification.

### Added — tolerances with red / amber / green (FR-53, 2026-10-20)

`/tolerances` lists every plan you can read that has a tolerance, worst first, with its overall colour and the colour of time, cost, risk and resources; `/plans/[pid]/tolerances` shows each dimension's limit (set on the plan or inherited), what was measured, how, and the colour, and lets you set or remove the plan's own limit (cost is typed in percent, stored in basis points). A colour is always a word and a shape as well; "not measurable" (a limit that could not be measured) and "no limit set" are their own states and never green. Linked from the navigation and from each plan. Tests: 4 unit groups, 2 e2e with axe. Welsh strings are machine-written, unreviewed. Not done: a tolerance cannot be set from the board; the how-it-is-measured sentences come from the service in English only; nothing here raises an escalation when a dimension is red.

### Added — approval processes page (PL-2, 2026-10-20)

`/approval-processes` (the path `/processes` is the plan-kind collection): the defined processes, a form to define one (up to 20 approve, to-do and notify steps, checked in words before it is sent), a form to start a run on a plan, task or other subject by its id, and your runs with each step's state and the log, with the buttons for marking your to-do step done, cancelling, and checking an approval again. The service decides who may do what; a refusal is shown as a message. Tests: 3 unit groups, 2 e2e (one with axe). Welsh strings are machine-written, unreviewed. Not done: editing or deleting a definition, choosing a subject from a list, and showing which approval request a waiting step is blocked on.

### Added — drill-down from a role to its tasks (PJ-12, 2026-10-20)

On `/resources/rbs`, a role whose people you may see has a button that opens the open tasks of those people, each linked to its task page, with a count of tasks in plans you cannot read and a note if the list was cut off. Tests: 1 unit, 1 e2e with axe. Welsh strings are machine-written, unreviewed. Not done: skills and the unclassified row have no drill-down; there is no per-person button.

### Added — reorder siblings in the work breakdown (PJ-12, 2026-10-20)

Each task in `/plans/{pid}/wbs` has move-up and move-down buttons (disabled at the edge; none for a "Restricted item"). A move sends the full sibling order to `PUT /api/plans/{pid}/task-order` and reloads, so the codes follow the service's order; a refusal (409, someone else changed the siblings) is shown as a message. Siblings sort by `sibling_position`, then by age. Tests: 3 unit (`siblingMove`, order), 1 e2e with axe. Welsh strings are machine-written, unreviewed. Not done: buttons only, no drag and drop; moving a task to another parent is still the task's own edit; the buttons do not repeat for a hidden sibling, which makes the service refuse (409) rather than reorder blind.

### Added — outcomes and de-escalation in the director pack page (EX-10, 2026-10-11)

A section with the sponsor's escalation level, the run of met quarters against the rule, and each quarter's state with the reason when it is missing. English and Welsh.

### Added — cutover decisions in the director pack page (EX-6, 2026-10-11)

A section with the cutovers decided in the period and counts of go, no-go and delay for safety. English and Welsh.

### Added — triage and thresholds in the director pack page (EX-7, 2026-10-11)

A section with each programme's latest triage, the period's decisions and the overrides in force. English and Welsh.

### Added — key people in the director pack page (EX-11, 2026-10-11)

A section with the counts of single points of failure and a table of the contingent share per skill pool. English and Welsh.

### Added — watermelon measures in the director pack page (EX-9, 2026-10-11)

A table of the four measures with a word and a shape for each, or why one is not measured. English and Welsh.

### Added — register completeness in the director pack page (EX-1, 2026-10-11)

The data-quality section shows how many programmes have a complete register and a table of the incomplete ones with each missing field named. English and Welsh.

### Added — external asks in the director pack page (EX-13, 2026-10-11)

A section listing the asks raised in the period and what each displaces, or that none is named yet. English and Welsh.

### Added — burn-up and flow for product programmes in the pack page (EX-14, 2026-10-11)

The earned-value section shows a table of tasks created, finished and open, throughput and the weekly finished counts for each product programme. English and Welsh.

### Added — the delivery model in the director pack page (EX-2, 2026-10-11)

The portfolio map table shows each programme's delivery model, owner and whether the organization delivers it. English and Welsh.

### Added — milestone sign-off in the director pack page (EX-4, 2026-10-11)

A section for milestones that need partner sign-off: delivered, met and the gap, each state a word and a shape as well as a tone. English and Welsh.

### Added — the critical chain, buffers and the drum (CC-6, 2026-10-11)

`/plans/[pid]/critical-chain` (the chain and its buffers as labelled bars with tables, beside the critical path), `/plans/[pid]/buffers` (a fever chart per buffer, an alert naming a red chain, recovery actions, snapshots) and `/pipeline` (the drum and the proposed staggered starts). A zone is a word and a shape (circle, diamond, triangle) as well as a colour; every chart has a table that gives the same figures. Axe passes in `en-001`, `cy-001` and `zh-cn`.

### Added — the director pack pages (DP-11, 2026-10-11)

`/director-packs` and `/director-packs/[pid]`: every section of an issued pack in English and Welsh (other locales show English words), readings as a word and a symbol as well as a colour, money in the reader's own format with its currency, a link to the service's English PDF. Axe passes in `en-001`, `cy-001` and `zh-cn`.

### Added — the API client sends an Idempotency-Key on every POST (2026-10-11)

A fresh key per call, reused while an identical POST (same path and body) is still in flight, so a double click is one request to the service. A caller can pass its own `idempotencyKey` to reuse across a retry, or `false` for none; a caller's own `Idempotency-Key` header wins. The proxy already forwards request headers. **Be clear about what it buys today:** the service replays on this key only for bulk imports; elsewhere it ignores it, so a double submit of another POST is still two writes at the service. The key is what makes server-side replay possible; the service side is not built for other routes.

### Added — the change requests page (IR-4, 2026-10-10)

`/change-requests` lists change requests across plans with a status summary and filters, in English and Welsh (draft). Read-only: drafting and deciding are through the API.

### Added — the handling mark banner (RG-7, 2026-10-10)

A plan page shows its effective handling mark as words in a bordered bar (English and Welsh draft) when the deployment has a scheme; nothing otherwise, and nothing when the lookup fails.

### Added — a shared session store (RG-12, 2026-10-10)

`OIDC_SESSION_STORE=redis` with `REDIS_URL` keeps sign-in sessions and login transactions in Redis, so any front-end replica serves any request and a restart signs nobody out. Dependency-free RESP client; atomic single-use login states; idle timeout and refreshed tokens shared; Redis down means signed out. The default stays in-process.

### Added — idle session timeout (RG-2, 2026-10-10)

`OIDC_IDLE_MINUTES` (default 30; `0` off) ends an OpenID Connect session after that long without a request; a page request afterwards returns to the sign-in page with a sentence saying why.

### Added — the issues register page (IR-2, 2026-10-10)

`/issues` lists the issues across plans with severity, status, owner, age in calendar days and an overdue flag, and filters by status, severity, open-only and minimum age. All text is in the catalogue with Welsh (draft); codes show as words through `enumLabel()`. The page lists and filters; raising and editing issues from the page is not built.

### Added — every page in Welsh (HD-13, 2026-10-10)

All text that a page or a component prints now comes from a catalogue: 2,327 keys in `src/lib/i18n-pages.ts` (English) and `src/lib/i18n-pages.cy.ts` (Welsh, typed as a complete record, so a missing Welsh string fails the type check). Page keys use plain `{name}` interpolation, not message-format syntax, so a value prints exactly as before; a count that changes the wording is two keys (`…_one`, `…_other`). Code values the service sends (task status, plan kind, proposal status, score band) are shown through `enumLabel()` (`src/lib/enum-labels.ts`), and the common API error codes through `localizeApiMessage()` in the client. New `src/routes/+error.svelte` replaces SvelteKit's English-only error page. Guards: `tests/unit/catalogue-coverage.test.ts` (no literal English in templates; Welsh complete, placeholders kept, not a copy of the English) and `tests/e2e/welsh.spec.ts` (every route rendered in English and Welsh; a line identical in both fails). **The Welsh is a draft: it has not been reviewed by a professional translator**; see `governance/welsh-translation-review.md`. Other locales fall back to English for the new page text.

### Added — security headers and a strict Content Security Policy (HD-4, 2026-10-09)

Every response carries `X-Content-Type-Options`, `X-Frame-Options`, `Referrer-Policy`, `Permissions-Policy`, `Cross-Origin-Opener-Policy`, `Cross-Origin-Resource-Policy` and (over HTTPS only) `Strict-Transport-Security` (`src/lib/security-headers.ts`, applied in `hooks.server.ts`). `kit.csp` in `vite.config.ts` sets `script-src 'self'` with SvelteKit nonces and hashes and no `'unsafe-inline'` for scripts or `<style>` elements; inline style *attributes* are allowed because the SVAR widgets set computed `style="…"`. SVAR's icon font is now self-hosted under `static/svar/` and its themes are mounted with `fonts={false}`, so no third-party stylesheet loads. `tests/e2e/security-headers.spec.ts` fails on any CSP violation across representative routes in three locales; the e2e `settle()` helper uses a constructed stylesheet because an injected `<style>` tag is now (rightly) blocked.

### Changed — Oxford spelling and explicit day units (NFR-10, NFR-11, 2026-10-09)

English text and identifiers use Oxford (`-ize`) spelling (`normalizeLocale`, the `/prioritization` page); durations in days say "calendar days" or "business days" (the compact `2.3d` forms are spelled out). The package licence matches the Rust crates (`MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only`).

### Removed — two-letter locale forwarding

`/en/…`, `/cy/…`, `/de/…` and the other bare two-letter prefixes no longer redirect (301) to `/en-001/…`, `/cy-001/…`, `/de-001/…`. A two-letter code is not a locale: such a path is an ordinary bare path (it gains the reader's own locale prefix and then finds no page). Only the full `<language>-<region>` codes are URL prefixes (spec NFR-4a). The earlier "Added — localized routes" entry below describes the old behaviour.

### Added — display preferences and an accessibility audit (LT-12)

`/display`: **simple view** (advanced destinations behind "More"; nothing removed; on by default for the worker role, an opt-in otherwise) and a **dyslexia-friendly font** (OpenDyslexic). Kept on the device and, signed in, on the account. Reduced motion is honoured everywhere. An axe audit now covers every route, and fixed: muted and danger text contrast, and the SVAR grid's invalid `aria-rowindex`.

### Added — forms for the Monte Carlo inputs

On the governance page: a risk's simulation inputs (one probability, the task it delays, the cost it adds) and a budget line's low / high range. On the critical-path page: a task's optimistic / pessimistic range. Mistakes are refused in words before anything is sent. Fixed: the Escalate button's text contrast, and two input-handling bugs found by the browser tests.

### Added — cost-risk and joint-risk pages

`/plans/{pid}/cost-risk`, `/cost-risk` (portfolio) and `/plans/{pid}/joint-risk`: histograms with labelled P50 / P80 / P90 and budget lines and a table twin, the joint confidence beside both marginals with a quadrant table and scatter, and every assumption stated.

### Added — scheduling views (SC-11) and a browser accessibility audit

`/plans/{pid}/schedule-health`, `/plans/{pid}/leveling` (proposal, then an explicit confirmed apply), `/resources` (role heatmap with a table twin) and `/plans/{pid}/schedule-risk` (histogram with P50 / P80 / P90 and a table twin). `tests/e2e/scheduling.spec.ts` runs axe over them and the milestone, KPI and OKR pages. Fixed two defects it found: links in running text are now underlined, and muted text meets the 4.5:1 contrast minimum.

### Added — milestones page

`/milestones`: every milestone with status, forecast from the schedule, slip and how it was achieved; complete, reopen, reschedule with a reason, link tasks, and see the slip history.

### Added — KPI and OKR pages

`/okrs` and `/kpis`: create, check in or read, edit and delete; derived scores and statuses, with "Not measured" and "No data yet" as their own states, and status as a word and shape rather than colour alone.

### Added — float on My work (SC-10)

Each task on `/me` says "On the critical path", "N business days of float", or, for negative float, "This is already late for the plan"; a task with no schedule data shows nothing. Plain text, not colour alone.

### Added — localized routes and Sveltia I18n

Every page is served under a locale prefix: `/en-001/plans`, `/cy-001/plans` (also `ar-001`, `de-001`, `de-de`, `es-001`, `fr-001`, `hi-001`, `zh-001`, `zh-cn`; `de-001` and `zh-001` start as copies of `de-de` and `zh-cn`). A bare two-letter alias redirects (301) to its locale — `/en/…` to `/en-001/…`, `/cy/…` to `/cy-001/…`, and `de`, `zh` to `de-001`, `zh-001`. A bare, unprefixed URL (a bookmark, the magic-link email) redirects (302) — on the server, and in the root layout's `load` for client-side navigations — to the locale in the `locale` cookie, else `Accept-Language`, else `en-001`. The routes themselves did not move: the universal `reroute` hook strips the prefix. The API, `/_app`, `/assets` and static files are never prefixed. Messages are now formatted by Sveltia I18n (`@sveltia/i18n`, MessageFormat 2) behind the same `t(key)`, which also takes `{name}` values; `localeHref(path)` prefixes every in-app link, and the language picker navigates to the same page under the chosen prefix. `<html lang>` and `dir` are set by the server per request.

### Added — EVM analytics (SC-5)

On the earned-value page: EAC variants each named by formula with a stated recommendation, ETC, VAC, TCPI and traffic lights, an S-curve (earned value and cost only where recorded), and a per-currency roll-up beneath a plan.

### Added — earned value (SC-4)

`/plans/[pid]/earned-value`: planned value, earned value and actual cost per currency, schedule and cost performance and earned-schedule SPI(t) in plain sentences; how percent complete is counted is stated, and a manual percent is always flagged.

### Added — RACI matrix

`/plans/[pid]/raci`: tasks down, people across, R/A/C/I toggles per cell, one Accountable per task, rows with gaps named in words, load by person, and a spreadsheet export.

### Added — schedule baselines and variance (SC-3)

Take a numbered, append-only baseline on the critical-path page; the Gantt shows an outline where each task was baselined; a variance table separates slipped, pulled in, added and removed in business days.

### Added — availability and proposed bookings (PJ-3, PJ-4)

`/capacity` draws firm load against capacity in force with the proposed bookings as a separate hatched series and a strip of over-capacity days; `/me/calendar` edits availability ranges; allocations can be proposed and committed (with a confirm when it would overload).

### Added — critical path and calendars (SC-1, SC-2, PJ-12)

`/plans/[pid]/critical-path` (Gantt with float tails, precedence network diagram, table, and a schedule editor that reports what moved), `/plans/[pid]/calendar` and `/me/calendar` (weekly pattern, holidays, absences, with symbols as well as colour). Pure helpers in `src/lib/cpm-view.ts`.

### Added — accessible Gantt and Kanban tables (Lily headless)

`/gantt` and `/plans/[pid]/board` gain an "Accessible table" view built on `GanttTable*` / `KanbanTable*` from `@lilydesignsystem/svelte-headless`, with pure helpers in `src/lib/lily-grids.ts`: named cells, arrow / Home / End navigation, a labelled Move control per card, undated and unknown-status items listed rather than hidden.

### Added — role-tailored navigation and landing page (T-28f)

The nav and `/`'s landing route now respond to a deployment-declared
`view` ABAC attribute (e.g. `view=executive` reorders the nav and
lands `/` on `/executive`), read via a new `currentUser()` BFF call to
the auth service's `GET /api/auth/me` — which itself needed a small
upstream fix (`authentication-service-with-loco`'s `CurrentResponse`
carried no `attrs` field at all; see that crate's own `CHANGELOG.md`).
Presentation only: authorization stays with the service's ABAC, and
every route stays reachable by direct URL regardless of `view`.
Absent/unmatched `view` ⇒ today's nav, unchanged. New pure module
`src/lib/nav.ts`. See `spec/index.md` §13 T-28f.

### Added — phone-viewport responsive audit (T-28k)

A new `mobile` Playwright project (390×844) runs `tests/e2e/mobile.spec.ts`
across all 35 real routes, asserting no horizontal body scroll and a
visible primary heading per route. Found and fixed four real overflow
sources: the SVAR `FilterBar` on `/plans` (fixed-width internal
input, plus hiding it alongside the grid under the mobile breakpoint
— filtering a plain name list has little value), an `<input
size="40">` on `/ideas` (removed), and a bare `<table>` on `/reviews`.
`/scenarios` also overflowed but needed no route-local edit: the
global `input { max-width: 100% }` rule added to `src/app.css` covers
it without touching its own `size="50"`/`size="12"`/`size="4"`
attributes. `/plans`'s `Grid` and `/gantt`'s `Gantt` now degrade to a
plain, read-only `<ul>` under a 600px breakpoint (CSS-only, no JS
viewport detection); `src/app.css` also gained global `table` overflow
containment covering every other route with the same shape of bug.
Fixed a real regression this surfaced in `tests/e2e/smoke.spec.ts`
(the mobile-list fallback duplicated the seeded plan's name in the
DOM, making an existing `getByText` assertion ambiguous — rescoped to
the grid cell). See `spec/index.md` §13 T-28k.

### Added — operator onboarding guide (T-28p)

`/onboarding` — a role-by-role "first hour" walkthrough (executive,
PMO, resource manager), each step naming the real page it lands on.
Pure static content (`src/lib/onboarding.ts`), no API call, so nothing
on the page can fail to load. Deliberately makes no
time-to-productivity claim — nobody has measured how long a first
hour with the guide actually takes.

`tests/unit/onboarding.test.ts` (new) walks every step and asserts,
via `fs.existsSync`, that its named route resolves to a real
`+page.svelte` — the guide cannot drift from the route tree
undetected. See `spec/index.md` §13 T-28p and the service crate's own
`spec/13-tasks.md` T-28p, where this gap was first identified.

### Changed — PickerBar now uses Lily's default themes and text sizes, not app-specific lists

Removed the hand-maintained `THEMES`/`THEME_LABELS` (39 slugs — a subset
of Lily's own 45, missing `adobe-spectrum`, `mozilla-protocol`, and the
UK/US government themes) and `SIZES`/`SIZE_LABELS` (a custom 4-step
`small`/`medium`/`large`/`x-large` scale) arrays from `+layout.svelte`.
`PickerBar` now gets no `themes`/`sizes` prop at all, so it uses its own
`DEFAULT_THEMES` (45, alphabetical, UK/US government themes grouped at
the end) and `DEFAULT_SIZES` (the 7-step `largest`…`smallest` scale)
directly — a superset of what was offered before, not a behavioural
narrowing.

`src/app.css`'s `[data-text-size]` rules rewritten for the new 7-step
scale (was 3 rules for `small`/`large`/`x-large`; the custom scale's
`medium` default is now Lily's `normal`, still unscaled).

### Changed — chrome consolidated onto Lily `PickerBar`; locale picker restored

The top-bar chrome (theme, text-size, share pickers) previously wired
`ThemePicker`/`TextSizePicker`/`SharePicker` individually in
`+layout.svelte`. Replaced with the single Lily
`lily-design-system-svelte-picker-bar` component, which composes all
four pickers — theme, locale, text-size, share — as one row.

This restores a locale picker to the chrome, since `PickerBar` bundles
all four unconditionally with no way to omit one. Wired with
`applyDir={false}` and `onChange={(code) => i18n.set(code)}`, since
this app's own `i18n.svelte.ts` store already reflects `lang`/`dir`
onto `<html>`; the `<html lang>` effect now hyphenates the locale
(`i18n.locale.replace("_", "-")`) to stay valid BCP47, agreeing with
what `LocalePicker` itself writes via its own `bcp47LocaleTag`.

`SHARE_TARGETS` gained an `email` (`mailto:`) target and now orders
Email / LinkedIn / Reddit / Bluesky / Mastodon; the Mastodon target
now points at `mastodonshare.com` instead of `mastodon.social/share`
(a generic sharer rather than one specific instance).

New dependencies: `lily-design-system-svelte-locale-picker`,
`lily-design-system-svelte-picker-bar` (both `file:` deps on the
sibling Lily checkout, same pattern as the existing four).

Also added `en_US` ("English (United States)") to
`src/lib/i18n.svelte.ts`'s `LOCALES`/`LOCALE_LABELS`/`STRINGS` (a
verbatim duplicate of `en`'s copy) and taught `normalizeLocale` to
match a region variant like `en_US`/`en-US` exactly before falling
back to stripping to its primary subtag (previously `en_US` would have
silently collapsed to `en`, since only the primary subtag was ever
checked).

### Fixed — `/verify` crashed with a raw 500 when the authentication service was unreachable (FE-5)

`src/routes/verify/+page.server.ts` called `await verifyMagicLink(fetch,
token)` with no `try`/`catch`. A network-level failure (the
authentication service unreachable, timed out, connection reset) makes
`fetch` throw rather than resolve — uncaught, that propagated out of
`load` and SvelteKit rendered its generic 500 error page instead of
this route's own friendly UI. The same bug class was found and fixed
first in `place-front-end-with-svelte` (T-26) and
`thing-front-end-with-svelte` (T-23); ported here: a `try`/`catch`
around the call, a new `"serviceUnavailable"` error variant, and its
message in `+page.svelte`. New `tests/unit/verify.test.ts` unit-tests
the `load` function directly (missing token / service unavailable /
invalid token), verified to fail with the `try`/`catch` reverted and
pass with it restored. See spec §13 FE-5.

### Fixed — the delivery calendar never showed a milestone event

`@svar-ui/calendar-store` requires an all-day event's `end` to be
strictly after `start`, but `/calendar`'s `+page.svelte` passed
`end: day` — the same `Date` object as `start` — so the SVAR Calendar
widget silently dropped every milestone it was ever asked to show. The
existing e2e test never caught this because it asserted only the
fallback `<ul>` list, a separate render path from the calendar widget
itself. The identical bug was already found and fixed in
worker-front-end's and person-front-end's `/expiry` calendars. Fixed
by computing the following calendar day as `end`; the test now also
asserts the widget itself renders the event, with the stubbed
milestone's due date computed relative to the actual test-run date so
it stays inside the widget's default "today's month" view.

### Added — the time-based-analysis view (TBA-8)

`/plans/{pid}/flow`, linked from the plan detail page: the delivery
board read through the lens of time rather than of activity.

- **The service level expectation first.** "85% of items finish within
  11 calendar days", with the sample size beside it — the number a team can
  actually quote. Below the service's minimum sample it renders the
  refusal, not a figure computed from noise.
- **Aging work in progress** next, because it is the only thing on the
  page whose outcome can still be changed. Items past the expectation
  are flagged; cycle time, throughput and yield are all history.
- **A cumulative flow diagram** (`src/lib/components/CumulativeFlow.svelte`):
  an inline SVG stacked area chart, no charting dependency. Bands stack
  with `done` at the bottom, so the vertical gap above it is work in
  progress and the horizontal gap is roughly the cycle time — Little's
  Law read off the picture.
- **Constraints, flow rates and distributions** below, with cycle time
  always shown beside lead time.

Three things worth keeping:

1. **A low flow efficiency is captioned as normal.** 5–15% is typical
   for knowledge work, so a bare "7.1%" invites the wrong reaction. The
   tile says which band the figure sits in — and calls a suspiciously
   *high* one out as a stale board rather than as excellence.
2. **A null ratio renders as an em-dash, never as 0%.** An item that
   never started has no flow efficiency; "0%" would read as
   catastrophically inefficient rather than as not-applicable. Pinned by
   a unit test.
3. **The chart carries a table view.** Three of the five palette slots
   sit below 3:1 against the light surface, so the numbers must be
   reachable without reading colour — the palette's relief rule, pinned
   by an e2e test rather than left as intent.

The palette is the validated categorical set, assigned to bands in a
fixed order and never recycled when a band goes empty. It was run
through the validator in both modes before use (worst adjacent CVD ΔE
9.1 light / 8.4 dark). The figure paints its own surface **and its own
ink**: the first dark-mode render lost the legend entirely, because a
container that sets a background while inheriting the host page's text
colour is only legible while the two themes agree.

Nothing in the view is per-person. Assignees appear on an aging item so
a reader knows who to ask, never as a ranked comparison.

### Added — wire `/plans` itself to `listPage()` (2026-08-05)

The sibling gap the entry directly below flagged and left open:
`PlanRepository.listPage()` existed (and was tested at the repository
level) but `src/routes/plans/+page.svelte` still called the unpaginated
`list()`. Now it calls `listPage()` and shows the same `shown / total`
count as `/organizations`, `/automations`, and `/reviews`.

- **`src/routes/plans/+page.svelte`** — `onMount` calls `repo.listPage()`
  instead of `repo.list()`; a `.count` paragraph shows `shown / total`
  (hidden when the list is empty), reusing the exact style/markup
  pattern from the organization front-end's `/organizations` list route.
- No `?parent=` roll-up wiring added — the list route never took a
  parent scope before this change either.
- 3 new `tests/unit/plans.test.ts` cases (65 → 68 vitest total) pinning
  `listPage()`'s URL, header parsing, and the header-absent fallback.
  No new Playwright coverage needed: the existing `list page renders the
  seeded plan` smoke test already exercises the now-paginated path.
- `svelte-check` 0/0; `pnpm run build` green; 23/23 Playwright pass.

### Added — pagination on the automation + review capability views (repo tasks.md PG-1, 2026-08-05)

The service's `automations`, `automations/runs`, `scheduled-actions`,
and `reviews` list endpoints gained real `?limit=&offset=` pagination
(`X-Total-Count`/`X-Limit`/`X-Offset`), closing the last PG-1 sub-bullet
left open on 2026-08-01. This wires the two consuming routes to it.

- **`src/lib/api/capabilities.ts`** — `listAutomationsPage`, `runsPage`,
  `listScheduledPage`, `listReviewsPage`, `inboxPage`: thin
  `ApiClient.getPage()` wrappers alongside their existing unpaginated
  siblings, same pattern as `PlanRepository.listPage()` next to `list()`.
- **`/automations`** — the rules table, the deadline queue, and the run
  log each show a `shown / total` count (hidden when the list is empty),
  matching the organization service's front-end `/organizations`
  convention.
- **`/reviews`** — the delegation list gets the same count.
- **Notifications stay unwired.** `inboxPage()` exists for contract
  parity, but no route calls `inbox()`/`inboxPage()` — a documented gap
  since the 2026-07-22 capability-views entry, and building a
  notifications inbox page is new UI scope this task did not take on.
- **No Prev/Next pager was added**, matching the family's headers-plus-
  count convention (`agents/share/restful.md`) — deep paging past the
  default page still works via `?limit=&offset=`, just not from a UI
  control yet.
- 3 new `tests/unit/capabilities.test.ts` cases (62 → 65 vitest total),
  reusing `tests/unit/client.test.ts`'s `getPage` header-stub pattern.
  No new Playwright coverage: `/plans`'s `listPage()` has none either
  (it is not wired into `/plans/+page.svelte`), so there was no
  established e2e depth for this change to match.
- `svelte-check` 0/0; `pnpm run build` green.

### Fixed — doc/code reconciliation + e2e route staleness (2026-08-04, DOC-4 audit)

- **`.env.example`** documented the decommissioned client-held-token
  model's vars (`PUBLIC_API_BASE_URL`, `VITE_AUTH_FRONTEND_URL`, and a
  leftover "Case Service" header comment copy-pasted from the case
  front-end) — zero references in `src/`. Rewritten to the real
  server-side-only vars `PROJECT_PORTFOLIO_MANAGEMENT_API_URL` /
  `AUTH_API_URL` (`src/lib/server/config.ts`), matching README/AGENTS.md
  which already described them correctly for the BFF proxy but not for
  the file operators actually copy.
- **`tests/e2e/smoke.spec.ts` and `tests/e2e/ppm.spec.ts` were
  navigating to `/projects*` routes that have not existed since the
  2026-07-20 `/plans` collection unification**, plus both stubs matched
  request paths without stripping the BFF proxy prefix
  (`/api/proxy/api/...`, from `ad95088e`). Neither `pnpm run check` nor
  `pnpm test` (vitest) nor `pnpm run build` exercises Playwright, so this
  was invisible until `pnpm test:e2e` was actually run — which had
  apparently not happened since the route unification landed. Fixed
  both: route paths (`/projects` → `/plans`, including the task-board
  route), stub match paths, and the proxy-prefix strip. All 23
  Playwright tests now pass (was 21 failing of 23).
- **`spec/index.md`, `AGENTS.md`, `README.md`, `index.md`** described
  several capabilities as built that have no repository method, type, or
  route: a server-round-trip name search, a recent-activity feed, a
  per-plan audit timeline, a per-component match-score breakdown visual,
  and the plan-detail child-plan roll-up. Also corrected: the auth model
  description (this app's own `/signin` + `/verify` BFF pages, not a
  redirect to a central authentication front-end — `signInUrl()` and
  `src/lib/auth.svelte.ts` do not exist), stale `src/` tree entries
  (`src/lib/i18n/` is one file, not a directory; `PlanRepository` does
  not carry `audit`/`recentEvents`/sub-resource methods), and several
  stale test-suite descriptions and counts. See `spec/index.md` §6, §8,
  §9, §11, §13 and `AGENTS.md`'s Status note for the corrected detail.

### Added — record-merge page (2026-08-03)

- **`/plans/merge`** — an operator page for folding a confirmed-duplicate
  plan into a survivor: survivor pid + duplicate pid + optional reason,
  an optional side-by-side preview, a native `confirm()` before the
  destructive call (the duplicate is soft-deleted), and a recent-merge
  history table. Reachable from a new **Merge** entry in the top-bar nav.
- **`PlanRepository.recentMerges()`** — `GET /api/plans/merges/recent`,
  the service's merge history (newest first, service cap 100).
- **`MergeRequest` / `MergeResponse` / `MergeRecordRow`** wire types.
  This service's merge response is `{main_pid, duplicate_pid, main}` —
  there is deliberately **no** `merge_record` wrapper the way the person
  service has one, so the page reads the history row back separately.
- **`src/lib/components/merge-validation.ts`** — the pre-merge guard as a
  pure, unit-testable function returning an i18n key: both pids required,
  and they must differ (the service answers `422` on a self-merge, so
  catching it here saves the round trip and states the reason in the
  operator's own language).
- 24 new i18n keys across all 13 locales.

### Changed

- **`PlanRepository.merge()`** now takes a single `MergeRequest` object
  rather than three positional arguments, so the call site reads as the
  wire body it becomes.

### Added — paged collection reads (2026-08-01)

- **`ApiClient.getPage()`** returns `{ items, total, limit, offset }`,
  reading the service's `X-Total-Count` / `X-Limit` / `X-Offset` headers.
  The plain `get()` throws response headers away, which is fine for one
  record and useless for a collection. A service that predates the
  headers still works: the page length is the fallback.
- **`PlanRepository.listPage()`** wraps it; `list()` is unchanged for callers
  that just want the default page.
- `listPage()` keeps the optional `parent` scope, which the service
  applies to its count as well as its page.


### Added

- 2026-07-22 — **Capability views for collaborative review, workflow
  automation, prioritization, and bird's-eye visibility**, consuming
  the service's new endpoints (service spec §9.4a):
  - `src/lib/api/capabilities.ts` — `CapabilityClient` plus the wire
    types (`Review`, `Consensus`, `Automation`, `AutomationRun`,
    `ScheduledAction`, `Notification`, `SmartScore`, `RankedPlan`,
    `LifecycleFunnel`, `PlanLifecycle`, `AssigneeLoad`). One method per
    endpoint; unset query parameters are omitted rather than sent blank.
  - `/prioritization` — the Smart Score queue with an **Explain**
    toggle showing each component's weight, value, and points, plus the
    per-plan evidence coverage. Unscored plans read as `unscored`, never
    as a low score.
  - `/lifecycle` — the challenge funnel: live and stalled counts for
    every phase (empty phases included) and any items in an
    unrecognized phase.
  - `/reviews` — delegate an item to an internal or external expert,
    accept/decline, submit a verdict, withdraw, and read the consensus
    (which shows outstanding invitations and reports a tie as a tie).
  - `/automations` — configure rules (trigger + action, with the action
    fields switching on the chosen action kind), enable/disable/delete,
    the pending deadline queue with a manual sweep, and the run log
    including skipped and failed runs.
  - Nav entries for the four new pages; 6 vitest cases pinning the
    client's endpoint paths.
  - **Not built:** these four pages are English-first (the 13-locale
    catalogues are not yet extended to them, matching the other PPM
    catalogue views), and there is no dedicated notifications inbox page
    — the inbox endpoint is wired in the client only.

- 2026-07-22 — **Five new `PlanKind` values: `Practice`, `Process`,
  `Purpose`, `Pathway`, `Proposal`** in `src/lib/api/types.ts`, so the
  `PlanForm` kind `<select>` (driven by `ALL_KINDS`) offers them. The
  legacy `COLLECTIONS` segments gain `practices` / `processes` /
  `purposes` / `pathways` for the proposal / idea / report kind-target
  selects; `proposals` is deliberately **not** added there, because that
  segment already names the separate proposals catalogue route.

### Changed — unify the four collections into one recursive `/api/plans` collection + rename to `plan` (2026-07-20)

- **One recursive collection.** The four PPM REST collections
  (portfolios / projects / products / programs) are unified into a single
  recursive `/api/plans` collection: every record is a **plan** under
  `/api/plans`, and matching runs across the whole collection rather than
  being gated by kind.
- **Entity renamed `work item` → `plan`.** `src/lib/api/types.ts` renames
  `WorkItem` → `Plan` (and `WorkItemKind` → `PlanKind`, `WorkItemEvent` →
  `PlanEvent`, `WorkItemIdentifier` → `PlanIdentifier`, `WorkItemRef` →
  `PlanRef`); `kind` is now **optional** (a descriptive label —
  Portfolio / Project / Product / Program — not a collection selector);
  `portfolio_ref` → `parent_ref` (any plan may contain any other).
- **Routes.** The dynamic `[collection]` route directory is collapsed to a
  static `plans/` directory (`/plans`, `/plans/new`, `/plans/[pid]`,
  `/plans/[pid]/{edit,board,schedule,governance}`). The nav's collection
  switcher is gone — there is a single **Plans** destination.
- **API client.** `src/lib/api/work-items.ts` → `src/lib/api/plans.ts`
  exporting `PlanRepository` (no longer collection-bound; all paths under
  `/api/plans`, with the `?parent=` child roll-up and
  `/api/plans/{pid}/schedule`).
- **Form.** `WorkItemForm.svelte` → `PlanForm.svelte`; `kind` is now an
  optional `<select>` (a plan may have no kind); `parent_ref` is an
  optional plan picker for any plan.
- **Tests.** `tests/unit/work-items.test.ts` → `tests/unit/plans.test.ts`
  and `tests/unit/work-item-form.test.ts` →
  `tests/unit/plan-form.test.ts`. `svelte-check` is 0 errors / 0 warnings
  and the vitest suite (45 tests) is green.

### Added — engineering moderate views (2026-07-20)

- Board gains story-point input (+pt on cards), the per-sprint
  retro/feedback notes panel with convert-to-task, and the velocity
  table (team-local note shown). `/engineering` gains the DevOps
  metrics tiles (derivation shown; unresolved incidents counted, never
  timed) and the release register. Six typed client methods + pins;
  e2e stubs + assertions.

### Added — engineering-team views (2026-07-20)

- `/{collection}/{pid}/board` — the per-item task Kanban (drag = the
  PATCH move; the service owns the flow stamps) with sprint
  create/select, the honest burndown table (server derivation shown),
  and the standup digest; linked from the item detail page.
- `/calendar` — the estate milestone calendar (SVAR Calendar) with
  kind filter; `/engineering` — blocked-work aging + MoSCoW scope +
  delivery links. Nav + keys ×13 locales; eleven typed client methods
  (ApiClient gains `patch`); path pins; three stubbed Playwright specs.

### Added — oversight areas (2026-07-20)

- `/board`, `/auditor`, `/compliance`, `/risk`, `/security`,
  `/regulator` pages over the thirteen oversight endpoints (board pack
  + investments + take-snapshot trends; findings + filterable trail +
  evidence-pack CSV; registers; heatmap matrix with honest no-appetite
  note; coarse regulator extract). Nav + `ppm.nav.*` keys ×13 locales;
  twelve typed client methods with path pins; six stubbed Playwright
  specs.

### Added — executive moderate fits (2026-07-19)

- `/executive` gains the strategic-alignment section (coverage per
  collection, unaligned spend, largest unaligned items); `/technology`
  gains the technical-debt register and delivery-flow metrics;
  `/scenarios` gains a side-by-side compare panel (pick two, see
  per-currency deltas and feasibility). Client gains four typed
  methods with vitest path pins; e2e stubs + three extended specs.

### Added — executive areas (2026-07-19)

- `/executive`, `/financials`, `/technology` — CEO / CFO / CTO views
  over the service's new insight endpoints. All numbers are
  server-derived (RAG, variance, realization ratios, radar rings); the
  client formats with `money()` and displays the server's derivation /
  no-FX notes verbatim. Nav + `ppm.nav.*` keys in all 13 locales;
  client gains seven typed methods with vitest path pins; three
  stubbed Playwright specs.

### Added

- 2026-07-19 — SVAR strong fit: new **/gantt** route (nav-linked): the selected portfolio's
  schedule (PPM-6) in the SVAR Gantt — dated items as task bars,
  dependency edges as links, the critical path tinted, undated items
  listed honestly below rather than invented onto the timeline.
  Read-only in v1. One new i18n key (`ppm.nav.gantt`) x 13.

- 2026-07-19 — SVAR component seams: **@svar-ui/svelte-calendar**,
  **@svar-ui/svelte-kanban**, **@svar-ui/svelte-gantt**, and
  **@svar-ui/svelte-filemanager** are installed (no routes yet —
  candidate features are catalogued per project; see the roadmap).

- 2026-07-19 — SVAR DataGrid + Filter: the `/[collection]` index routes (**/portfolios, /projects,
  /products, /programs**) now render the SVAR DataGrid with a
  FilterBar above it (client-side name filtering over the loaded
  refs); row selection opens the work-item detail route.

- 2026-07-19 — Lily Design System: the hand-rolled locale `<select>` is replaced by the Lily
  **LocaleSelect** (wired to the i18n store; `applyDir` off), and
  the **Lily headless** component library is now a dependency
  alongside the existing ThemeSelect.

- 2026-07-18 — **13-locale i18n for the PPM views** (closing the
  English-first follow-up from the catalogue-views round): 98 new
  `ppm.*` keys — nav entries, view titles, table headers, buttons,
  chips, and empty-state lines for the dashboard, intake board,
  ideas, scenarios, objectives, capacity, reports, the governance
  panel, and the schedule view — translated across all 13 locales
  (en, cy, es, fr, de, ar, ru, hi, zh, bn, pt, id, ur) and wired
  through `t()`. The full-coverage parity test enforces every locale
  covers every key; new spot-checks pin PPM strings in zh/ar/hi/ur.
  Wire tokens (statuses, gate ids, currency codes, URN placeholders)
  stay verbatim by design, matching the untranslated collection
  segments in the nav.


### Added

- 2026-07-18 — **PPM catalogue views** (over service Phases A–C,
  PPM-1..12; English-first — extending the 13-locale catalogues to
  these strings is a follow-up). New `PpmClient`
  (`src/lib/api/ppm.ts`, the single source of PPM endpoint paths) and
  routes: `/dashboard` (site tiles + per-collection RAG/stage
  rollups), `/proposals` (intake board: pipeline actions,
  matcher-backed duplicate hits, promote-to-work-item), `/ideas`
  (capture/vote/convert), `/scenarios` (create/evaluate/commit with
  violation chips), `/objectives` (registry + alignment rollups),
  `/capacity` (per-person load meters, over-allocation flags),
  `/reports` (definitions, JSON preview runs, CSV download), the
  per-item **governance panel**
  (`/[collection]/[pid]/governance`: summary strip, gate journey +
  next-gate review form, risks + escalate, budget lines +
  record-actual, benefits + realize + ROI, OKR mappings, milestones,
  allocations), and the portfolio **schedule view**
  (`/portfolios/[pid]/schedule`: CSS Gantt bars, critical-path
  badges, finish-start violation banners). Nav + detail-page links.
  Tests: PpmClient path-mapping + `money` vitest suite; 3 Playwright
  PPM specs over a stubbed API. Verified live end-to-end through the
  BFF proxy against the running service.

### Fixed

- 2026-07-18 — The Playwright smoke suite was a stale copy of the
  case front-end's (asserting "Cases" headings, stubbing `title`
  instead of the work-item `name`, using the case app's detail
  routes) and had never passed against this app; rewritten against
  the real contract — all 8 e2e specs green.

### Changed

- 2026-07-18 — **Subproject renamed**: `portfolio` →
  `project-portfolio-management` (directory, crate/package name, lib
  ident, env-var prefix `PORTFOLIO_*` → `PROJECT_PORTFOLIO_MANAGEMENT_*`,
  database names). The **domain language is unchanged**: the work-item
  kinds (portfolio / project / product / program), the `work_items`
  table, the API routes, and the matcher's `WorkItem` type keep their
  names — the rename repositions the *subproject* as a project
  portfolio management (PPM) product; see the feature roadmap in
  `../spec/15-roadmap.md`.


### Changed

- **De-versioned API URLs.** Dropped the `/api/v1` segment from the work-item client and tests (now `/api/{collection}/…`); the BFF proxy negotiates the API version via the `Accepts-version: 1.0` request header instead (see `agents/share/api-versioning.md`).

### Fixed

- Prettier formatting drift across `src/` (left behind by recent
  BFF/auth-era edits) broke the `pnpm lint` (`prettier --check src`)
  gate. Reformatted with `pnpm format`; no behavioural change —
  `svelte-check` and the vitest suite are unchanged and green.

## [0.1.0] — 2026-06-18

### Added

- **Inaugural spec + docs (spec-only).** First deliverable for
  `project-portfolio-management-ui-with-svelte`: the living `spec/index.md` (§1–§18)
  and the doc-set (`README.md`, `CLAUDE.md`, `AGENTS.md`, `CHANGELOG.md`,
  `index.md`). No `src/` yet — code is tracked as the spec §13 build
  queue.
  - **Stack decision.** SvelteKit 2 · Svelte 5 runes only
    (`$state` / `$derived` / `$effect` / `$props` / `$bindable`; no
    `export let`, no `$:`) · TypeScript strict (`noUncheckedIndexedAccess`)
    · SPA (`ssr = false`) · SVAR Svelte DataGrid · Lily Design System
    Svelte Headless. Per-project drift accepted — own copy of
    `src/lib/api/{types,client,work-items}.ts` + form primitives; no
    shared package.
  - **Scope.** Consumes the portfolio service REST API under
    `/api/v1/{portfolios,projects,products,programs}/...` — **four
    matchable collections**, one per `WorkItemKind` (Portfolio / Project /
    Product / Program); matching is within a collection only. Identity
    surface per collection: work-item list (SVAR DataGrid), create / edit
    form, detail, name search, duplicate-check with a per-component
    **MatchBreakdown** visual (name, goals, code, owner org, portfolio,
    timeframe, keywords, relationships, tags — `kind` is a hard match
    gate, not a scored component), merge UI, and an audit timeline. A
    Portfolio detail page rolls up its child work items (by
    `portfolio_ref`).
  - **Project-management views.** Kanban task board (Todo / InProgress /
    InReview / Done / Blocked; drag = status change), issues list
    (kind / severity / status), Gantt / timeline view (goal milestones +
    task date ranges), burndown chart (remaining estimate over time), and
    a goals panel. (Posts / comments / members are not part of the v1
    portfolio sub-resource set — roadmap-only.)
  - **Layout shell.** Top navigation bar with a **leftmost hamburger**
    menu (NOT a left sidebar); full-width main content; a collection
    switcher in the chrome. Full theme catalogue via
    `lily-design-system-svelte-theme-select` (selecting a theme restyles
    the whole site). 13-locale i18n (en, cy, es, fr, de, ar, ru, hi, zh,
    bn, pt, id, ur) via `lily-design-system-svelte-locale-select`
    (selecting a locale switches the language; RTL for `ar` / `ur`).
  - **Auth.** Backend-For-Frontend (BFF) + httpOnly cookie session +
    CSRF: **Sign in** runs the central magic-link, which establishes a
    server-side session and sets an httpOnly `__Host-mxi_session` cookie;
    the browser holds **no token** (no `localStorage`, no
    `mxi_access_token`, no URL-fragment handoff). This app's SvelteKit
    server exchanges the session for a short-lived **PASETO v4.public**
    token to call the portfolio service server-side
    (`PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH`, off by default — family contract
    `agents/share/jwt-enforcement.md`). RS256/JWKS not used. Source of
    truth: `agents/share/authentication-sessions.md`.
  - **Testing plan.** vitest unit (client, repository per collection, BFF
    auth integration, `signInUrl`, `WorkItemForm`, i18n, and a `+layout`
    render test asserting the hamburger toggles the nav) + Playwright e2e
    smoke.
  - **Out of scope (stated).** No FHIR; no consent UI; no finance /
    budgeting UI; no posts / comments feed; no member / role panel; no
    login screen (SSO delegated).

### Configuration

- `PUBLIC_API_BASE_URL` (default `http://localhost:5150`).
- `VITE_AUTH_FRONTEND_URL` (default `http://localhost:5173`).
