# A starting hazard analysis for PPM, in the manner of ISO 14971

For an organization that gives PPM a **medical purpose** and so has to run a risk management process on it. It
follows the shape of [ISO 14971 as scoped on this project](index.md) and is modelled on the sibling project
CRM's `spec/compliance/risk-management.md`. Written on **2026-10-28** for the service 0.3.0, the matcher 0.2.0 and
the web client 0.1.0.

> ⚠️ **This is not a risk management file, claims no conformity with ISO 14971 or ISO/TR 24971, and invents no
> severity or probability.** It was written by the maintainer's AI coding assistant; **the standard itself was not
> read** and nobody has reviewed this against it or against any clinical use. It names hazards a care use would add
> to and the controls the software has. Severity, probability and acceptability are the integrator's.

## 1. Status

- **As shipped, PPM is not a medical device.** It has no clinical purpose, on the reading in the
  [MHRA scoping assessment](../../united-kingdom/medicines-and-healthcare-products-regulatory-agency/index.md).
  The standard does not bind this project.
- **If an integrator gives it a medical purpose** (for example, ranking patients' treatment or planning clinic
  capacity with it), the integrator becomes the manufacturer of that product and owns the risk management file.
- **Evidence rule, as in CRM:** a control is **Implemented** only when code or configuration here enforces it **and a
  test I found pins it**; **Deployer** when the organization running PPM must supply it; **Gap** when neither is
  true. A test is cited by file and name so a reader can run it. **I did not run the database tests on this
  machine; CI runs them** (the last green run is on the commit that added the page).

## 2. Foreseeable misuse

- **Care use.** Planning or prioritising clinical work in PPM and relying on its schedule, score or capacity figure.
  Nothing stops it or detects it, and there is no clinical-use warning in the product (hazard H13).
- **Reading a figure as more certain than it is.** A schedule, cost or utilization result computed from estimates.
- **Treating the demo or seeded data as real.**

## 3. Hazards a care use would add to

| # | Hazardous situation | Possible harm in a care use | Control in the software and the test that pins it | Status |
| --- | --- | --- | --- | --- |
| H1 | A **private plan, or one marked above a reader's clearance, is visible** to someone not entitled to it | Disclosure of a sensitive plan or its contents | Private plans answer 404 to non-members; a marked plan "does not exist" to a reader below its clearance; a mask obligation redacts reads and exports. Tests: `a_private_plan_is_404_to_non_members_and_visible_to_members_and_admins` (`tests/plan_access.rs`), `a_plan_marked_above_your_clearance_does_not_exist_for_you` (`tests/handling_marks.rs`), `mask_obligation_redacts_reads_and_exports` (`tests/masking.rs`) | Implemented |
| H2 | The **privacy lookup fails** and the request goes through anyway | A non-member reads a private plan during a database fault | Fails closed: a failed lookup under a list answers 503, counted. Test: `a_failed_lookup_under_a_list_answers_503` (`tests/requests/privacy_lookup_failure.rs`). **Writes are not converted** (the write has happened); work spawned onto another task does not carry the mark; only the list route is tested | Implemented for lists; **Gap** for the rest, recorded in `tasks.md` |
| H3 | A **score or ranking is wrong** | The wrong item is done first | The RICE arithmetic and the clamping of outsized inputs are tested: `rice_is_reach_times_impact_times_confidence_over_effort` (`src/scoring_models.rs`), `outsized_inputs_are_clamped_never_overflowing_the_scale` (`src/prioritization.rs`). Which model and which inputs is not something software can check | Implemented for the arithmetic; the choice is **Deployer** |
| H4 | **Money summed across currencies** | A wrong cost decision | The roll-up keeps currencies apart. Test: `the_rollup_never_merges_two_currencies_into_one_sum` (`tests/requests/financials.rs`). PPM does not convert between currencies | Implemented |
| H5 | **Utilization shown as 0%** when no capacity is declared, or a **small group** identifies a person | A wrong staffing conclusion; disclosure of who is in a rare group | No declared capacity reports `unavailable`, not zero; a team or plan of fewer than 5 people is withheld (`MIN_GROUP_PEOPLE`). Tests: `no_declared_capacity_is_not_zero_utilization` and `a_team_under_five_is_withheld_and_a_team_of_five_is_reported` (`tests/requests/effort.rs`). **Primary suppression only**; other aggregates were not audited for small-cell risk | Implemented for utilization; **Gap** elsewhere |
| H6 | Two **different records are merged as duplicates**, or a real one is dropped | A record about one thing is attached to another | Duplicate detection is explainable and a keyless duplicate on import is held for review, never created or merged: `a_keyless_duplicate_is_held_for_review_and_never_created_or_merged` (`tests/requests/bulk.rs`). **I did not find a test of the plan-merge page's own path** | Partly: the import path only |
| H7 | A **link or feed token is replayed** or forwarded | A stranger reads a person's calendar feed | Feed tokens are random, hashed at rest, shown once, revocable, expire after a year and rotate on reissue. Test: `feeds_authenticate_by_token_are_stable_and_revocable` (`tests/feeds.rs`, which also asserts rotation and expiry). **Feeds are reusable by design** (calendar apps poll one address) | Implemented, with that stated exception |
| H8 | A **non-English screen says something different** from the English | A person misreads an instruction | A catalogue-parity test keeps every locale to the same keys (`tests/unit/catalogue-coverage.test.ts`). **The Welsh strings are machine-written and no speaker has reviewed them** ([register](../../../governance/welsh-translation-review.md)) | Partly; the **review is a Gap** |
| H9 | An **error message** leaks detail or misleads | Confusing or unsafe text shown to a person | No page shows the service's error text; a fixed localized sentence by status. Test: `tests/unit/no-api-text.test.ts` | Implemented |
| H10 | A **milestone or forecast with no data is read as "on track"** | A late item is believed to be fine | The milestones page states that a milestone with no forecast says so and is never "on track" by default. **I found no test that pins it** | **Gap** |
| H11 | The data is **unavailable or lost** | A record cannot be read when it is needed | Backup and restore are in the [runbook](../../../operations/runbook.md); CI runs a restore drill that runs the whole test suite against a restored copy (`scripts/restore-test.sh`). **No clinical availability target exists** | Implemented for restore; the target is **Deployer**; **Gap** for any clinical target |
| H12 | A **software change breaks a control** unnoticed | Any of the above | CI runs fmt, clippy `-D warnings`, unit, database, restore, UI, axe and security scans on every push, and changes are spec, code and test together ([AGENTS.md](../../../AGENTS.md)). A clinical change-control process is not supplied | Implemented; clinical change control is **Deployer** |
| H13 | **Clinical use goes unnoticed** | Every hazard above, with nobody having assessed it | Nothing detects or warns. The maintainer receives no telemetry and has no access to any deployment | **Gap** |
| H14 | **Personal data kept longer than needed**, or erased while still needed | A record needed for a dispute is gone, or kept without cause | A [retention schedule](../../../governance/retention-schedule.md) and a personal-data inventory with a test (`tests/personal_data_inventory.rs`) exist. **I did not find a test of erasure itself** | Partly; erasure is a **Gap** |

## 4. Risks the controls themselves create

- **A 404 for a private plan hides that it exists.** A clinician who should have been told of a plan cannot learn it
  is there; that is the point, and also a cost.
- **Failing closed makes reads unavailable.** During a database fault a list answers 503 rather than risk a leak.
- **Withholding a small group can make a table look complete.** The answer says the figure is unavailable
  (`small_group`), but a reader can still skim past it.
- **Fixed error messages hide the cause.** The cause is in the audit and the logs, not on screen.
- **Rotating a feed token breaks the old subscription.** Reissuing a token revokes the previous one, so a calendar app
  stops until it is given the new address.

## 5. What an integrator would still have to do

1. Decide whether the product has a medical purpose and, in Great Britain, whether it is a device, with the regulator
   if in doubt.
2. If it is: appoint responsible people, write the risk management plan and file, set acceptability criteria,
   estimate severity and probability for the table above and for what the integrator adds, evaluate overall residual
   risk and review before release.
3. Close the gaps above that a clinical use needs: H2 (writes), H10, H13, H14 and the Welsh review (H8).
4. Supply a clinical change-control process, an availability target and a post-production loop of complaints,
   incidents and audit entries.
5. Obtain the NHS clinical-safety documentation (DCB0129 for the manufacturer, DCB0160 for the deploying
   organisation). The [DTAC assessment](../../united-kingdom/national-health-service/digital-technology-assessment-criteria/2.0/index.md)
   records that no DCB0129 documentation exists.

## What is not done

- No severity, probability or acceptability is assigned.
- The tests cited were found by name and read for their titles; **I did not re-read each one to confirm it asserts all
  that its row says**, and I ran none of the database tests here.
- H10, H13, and the erasure and plan-merge paths have no test I could find.
- The standard was not read; nobody with clinical-safety or regulatory expertise has reviewed this.
