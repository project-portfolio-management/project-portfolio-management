# Worldwide equity audit (WW-1)

Audit of the repository against [`spec/02-scope.md` §2.0](../spec/02-scope.md): the target is worldwide, and no
standard, protocol, convention or locale may gate one country or region. Done 2026-10-29 by three read-only passes
(service and matcher, operator interface, spec and documents).

**How to read this.** Each finding has an id, the place, what it assumes, and a **decision**: *configure* (make it a
setting with no country default), *extend* (add the missing coverage), or *limit* (kept on purpose, with the reason).
**Accepted by the product owner, 2026-10-31:** all 25 decisions, as written below. Each row's status says what has
been changed in code since; a row with no status is accepted and not yet done.

**What the audit is not.** The passes read files with pattern searches and spot reads. Line numbers come from search
output and were not each re-checked in context. Nothing was run. The lists of what each pass did not search are at the
end. The first four findings are the ones that most clearly break §2.0 ("the same features, the same quality of text,
the same tests").

## A. Highest priority

| Id | Finding | Decision |
| --- | --- | --- |
| WW-1.1 | **One locale has a completeness guarantee and tests; the other eight do not.** Only Welsh (`cy-001`) has the page-level catalogue (`i18n-pages.cy.ts`), a "renders fully" e2e test (`welsh.spec.ts`), a review record and NFR-19. Arabic, German, Spanish, French, Hindi and Chinese fall back to English for most page text, silently (`i18n.svelte.ts` `rawString`), with no coverage report. | extend: apply NFR-19 to every supported locale (a per-locale parity test, a review record, a marker for machine-written text); until then record the English fallback as a listed limit. **Status 2026-10-31: done** (accepted decision): a page catalogue for every locale (`i18n-pages.<ar|de|es|fr|hi|zh>.ts`, 4,008 keys each, typed complete; `de-de` and `zh-cn` share their language's), the unit guards for each (keys, placeholders, copied English: none left in Arabic, Hindi or Chinese beyond names and code, a falling count in German, Spanish and French), a browser test that opens every page route in every locale (`every-locale.spec.ts`), and a review record and working file per locale (`governance/translation-reviews/`). Every translation is a machine draft, **unreviewed**. Not done: plural groups hold only `_one` and `_other` in every language; the `lily-labels` packs are unchanged (WW-1.17). |
| WW-1.2 | **Compliance mapping names UK, EU and US-health law only** (`spec/12-compliance.md`; `governance/record-of-processing.md`, `dpia.md` use "UK GDPR / EU GDPR", "outside the UK or EEA"). No coverage for Asia-Pacific, Africa, Latin America, the Middle East, Canada or US state laws. | extend: reword to "the applicable data-protection law" with a per-region table (law, trigger, what it drives: erasure, residency, transfer, breach notice); keep HIPAA, NHS and ONC "not engaged" rows as deliberate limits. **Status 2026-10-31: done** (accepted decision): `spec/12-compliance.md` §12.1 and §12.2 say "the applicable data-protection law", new §12.2a maps 13 regions (law, trigger, erasure, residency and transfer, breach notice) and keeps HIPAA, NHS and ONC as stated limits; the record of processing and the DPIA name the applicable law and cross-border transfers instead of "UK GDPR / EU GDPR" and "outside the UK or EEA". Written from general knowledge, not checked against each law's text; not legal advice. |
| WW-1.3 | **Money.** GBP is a built-in default in `GET /api/tpc` (`tpc.rs`, documented in `openapi.rs`) and the starting value of five forms (costs, governance, cost-rates, scenarios, proposals). The UI `money()` is fixed to `en-GB` and a `.` decimal mark; amounts are scaled with `/100` and `*100`, which is wrong for JPY (0 decimals) and KWD (3); inputs reject "1,5". The service `exponent()` list is partial; `money()` in the director-pack text fixes `,` and `.`. `pack-view.ts` already does it right (`Intl.NumberFormat`, `exponentOf`). | configure: no built-in currency (a per-deployment or per-plan setting, or a required field); use `exponentOf` and one locale-aware formatter everywhere; parse amounts by locale; use the full ISO 4217 table. Add JPY, KWD and a decimal-comma case to tests. **Status 2026-10-29: fixed in part** (formatter, forms, `GET /api/tpc`, validator and ISO table); not done: the Welsh strings are machine-written and unreviewed, other locales fall back to English for the new text, key-result (OKR) currency values, the service text outputs still use a fixed `,` and `.`, and no database test could be run. See the root `CHANGELOG.md`. |
| WW-1.4 | **Working week and day.** `portfolio_resources.rs` and `work_profile.rs` assume Monday to Friday and ignore the plan's calendar, which elsewhere supports any weekday mask; the day is a fixed 480 minutes (`portfolio_resources.rs`, `scheduling_rules.rs`, `mspdi_batch.rs`); MS Project export writes `1111100` when a plan has no calendar (`mspdi.rs`). Grids and timesheets are Monday-first in code and UI, the calendar widget Sunday-first. | configure: read the calendar's mask and a minutes-per-day setting; add a week-start setting with a locale default; export the real calendar. Keep ISO 8601 weeks as the stated default, as a *limit* with the reason. |

## B. Service and matcher

| Id | Finding | Decision |
| --- | --- | --- |
| WW-1.5 | Financial-year start defaults to April (`financial_year.rs`); configurable only by one process-wide env var. | configure: no country default (January, or require a value); consider a per-deployment setting. |
| WW-1.6 | Small-cell floor `MIN_GROUP_PEOPLE = 5` and `DEFAULT_SUPPRESSION_FLOOR_MINUTES = 2400` are constants; the second is named "default" but nothing overrides it (`controllers/effort.rs`). No law is named for either. | configure per deployment, and state the value used and why (§2.0). |
| WW-1.7 | Retention defaults (`retention.rs`) are in code; the scheduled job passes no overrides, so a deployment cannot change them by configuration. Audit-record retention ("6 years") is in `governance/retention-schedule.md` only and is not enforced in code. | configure; name the basis of each default; record the unenforced audit retention as a known gap. |
| WW-1.8 | Data-subject export and erasure are named for GDPR (`privacy.rs`, `GET /api/plans/{pid}/export`); erasure is pseudonymisation only (`subjects.rs`), one fixed policy. | extend: neutral name for the export, other regimes listed as mappings; let a deployment choose pseudonymise or delete. |
| WW-1.9 | The director-pack PDF is Latin-1 only (`pdf.rs`): any other character becomes `?`; its fixed words are English (`director_pack_text.rs`). Welsh, Chinese and Hindi cannot be carried. | extend (embed a Unicode font with shaping and right-to-left), or record as a named limit with the HTML view as the route for other scripts. |
| WW-1.10 | Phonetic matching is American Soundex over ASCII letters (`phonetic.rs`), and `normalize::code()` keeps ASCII alphanumerics only, so a plan code in another script collapses to empty and would break deterministic matching. Search does no CJK word segmentation; `to_lowercase` is not locale-aware (Turkish I). | extend `code()` to Unicode alphanumerics with a test; make phonetic matching script-aware or a named limit; record the others as limits. |
| WW-1.11 | Handling-mark codes allow ASCII only (`handling_marks.rs`), so a classification scheme in another script cannot be used. | extend to Unicode letters. |
| WW-1.12 | Identity: PASETO by default; Keycloak is the only OIDC option, off by default (`auth/verifier.rs`). | extend: any compliant OIDC identity provider, so other federations are not forced through Keycloak; state that the set is not country-specific. (The external verifier crate was not audited.) |
| WW-1.13 | Smaller: `tz.rs` examples name `Europe/London`; `docs.rs` hard-codes `lang="en"`; currency validators disagree (`financials.rs`, `tpc.rs` accept any ASCII letters, the rest require three upper-case); GBP in error text and tests. | extend: neutral and varied examples, one currency validator, a non-GBP non-2-decimal test. **Status 2026-10-29: the currency items are fixed** (one validator, neutral examples in error text and a JPY, KWD and CLF test); the `tz.rs` and `docs.rs` items are not started. |

## C. Operator interface

| Id | Finding | Decision |
| --- | --- | --- |
| WW-1.14 | Ten locales, hard-coded in `locales.ts` (adding one touches five tables). No African language; no Japanese, Portuguese, Korean, Indonesian, Bengali, Swahili, Turkish, Russian, Persian or Urdu. Region fallback is by first match: `es-MX`, `fr-CA`, `en-IN` all collapse to the one world locale, and `zh-TW`/`zh-HK` silently get Simplified Chinese. | extend: a stated criterion for adding a locale; add `zh-Hant` or record Simplified-only as a limit; make the fallback explicit and tested. Candidates are in `spec/locales-for-global-sharing-with-svelte/locales.tsv`. |
| WW-1.15 | Right-to-left: only `ar-001`; four places use physical CSS (`ObjectiveCard`, `MilestoneCard`, `flow`, `resources`) that breaks mirroring; the font stack names no script-specific fonts; `CumulativeFlow` month labels are always English. | extend: logical CSS properties; month labels follow the locale; fonts as a limit or extended. **Status 2026-10-30: harmonized in part** (no decision needed): the `CumulativeFlow` day labels are now the locale's (`formatDayMonth`). Remains for the owner: the physical CSS in four places, script-specific fonts, any further locale. |
| WW-1.16 | Dates are ISO-sliced or built "day month year" by hand; some views call `toLocaleDateString()` with no locale; Gregorian only, no fiscal-year or non-Gregorian calendar. | extend: one shared formatter taking the UI locale and a time zone; Gregorian-only as a stated limit. **Status 2026-10-30: harmonized in part** (no decision needed): the UI now formats numbers, percentages and dates as the workforce UI does, through one module (`src/lib/locale-format.ts`, a port of WPM's) in the page's locale, civil dates in UTC and timestamps in the reader's zone; about 60 hand-rolled places replaced and a scan test (`tests/unit/locale-format.test.ts`) fails on new ones. Display only. Remains: dates the scan does not see (table columns, bucket headers, the network diagram's labels, "→" ranges in tables); Gregorian only and no fiscal-year calendar (a limit for the owner to state); the time-zone choice is the browser's, not a setting. See the UI spec "Numbers, dates and plurals" and the root `CHANGELOG.md`. |
| WW-1.17 | The `lily-labels` packs for the eight non-Welsh/English locales are machine-drafted and unread by a speaker. | limit, with a review record per locale (see WW-1.1). |
| WW-1.18 | Sign-in is by email magic link only. | limit: no phone or national-identity sign-in; say so. |

## D. Documents

| Id | Finding | Decision |
| --- | --- | --- |
| WW-1.19 | `spec/locales.md` says the repository has no locale directories; `/locales` has ten, and they do not match the supported ten (`en-gb`, `en-gb-oxendict`, `en-us`, `cy-gb`, `hi-in` extra; `ar`, `de`, `es`, `fr` missing). The persona documents name UK GDPR and the NHS DTAC. | reconcile the two lists; add the missing personas or record the gap; add non-UK equivalents. |
| WW-1.20 | `governance/language-statement.md` gives Welsh a dedicated statement and review record and the others one line. | extend (see WW-1.1). |
| WW-1.21 | `docs/` has UK folders only for national bodies (MHRA, NHS DTAC) and the `governance/dtac-checklist.md` is the only assurance checklist. The product is not a medical device (ISO 14971 page). | limit the UK folders as named worked examples; extend with a generic assurance mapping (ISO 27001, SOC 2, and national schemes); state the `docs/<country-or-body>/` convention. |
| WW-1.22 | PRINCE2 is the only national-origin method with its own spec chapter; terms such as tolerances, escalations and gate stages may be PRINCE2 vocabulary. | limit: label PRINCE2 as one example; extend with an ISO 21502 map and a PMI/IPMA map; check the terms. |
| WW-1.23 | `spec/05-domain-model.md` uses "£2m"; UI help text says "Currency (e.g. GBP)" in six keys. | generalise to a neutral code or several; regenerate the Welsh review CSV. |
| WW-1.24 | The specification has no policy on number, date, name and address formats or on right-to-left layout. | extend: an NFR tied to the locale. **Status 2026-10-30: harmonized in part** (no decision needed): the UI picks plural forms with `Intl.PluralRules` (`tn`, as WPM's) instead of `n === 1`, the five "(s)" strings are plural groups, and numbers and dates follow the locale (WW-1.16). Remains for the owner: the NFR itself (a policy on number, date, name and address formats and right-to-left layout); Welsh has only one and other for each plural group; 30 catalogue strings still put "%" after a number; names and addresses are untouched. |
| WW-1.25 | `governance/accessibility-statement.md` cites WCAG 2.2 AA (international) but maps no national law (EN 301 549, Section 508, AODA and others). | extend lightly or record as a limit. |

## E. Found to be neutral

Currency is any ISO 4217 code and money is never converted; the time zone is an IANA name with UTC as the default and
the service never guesses one; calendars are per plan and per person with a configurable weekday mask and holidays (none
bundled); the handling-mark scheme is configurable; the OIDC settings in the interface are issuer-agnostic; there is
no address, phone, postcode or name-field validation; people are opaque references; WCAG and the BCP 47 `in_language`
check are international.

## What was not searched

- Service: `tests/`, `fuzz/`, `benches/`, generated entities, the search-index data, the external
  `authentication-verifier` crate, SQL column defaults in the migrations (only a keyword search), `starter_templates.rs`
  and `frameworks.rs` for method bias, and most of the 124 controller files line by line.
- Interface: the `@lilydesignsystem/svelte-locale-translations` package (whether its Chinese is Simplified or
  Traditional), email and magic-link text, `static/`, and whether each translated string is present in each locale.
- Documents: most of `spec/06` to `18`, `agents/`, `ci/`, `scripts/`, the bodies of the persona documents and several
  governance files, and the root files `RFC.md`, `CONTRIBUTING.md`, `INSTALL.md`, `COMPARISONS.md`, `AI_STATEMENT.md`,
  `NEWS.md` and `plan.md` (hits were counted, not read).
- Nothing was run or rendered; git history and external links were not checked.
- The three passes did not search for several things §2.0 names: bidirectional text beyond Arabic, name and address
  formats in free text, and holiday data by region.
