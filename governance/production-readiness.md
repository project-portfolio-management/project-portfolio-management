# Production readiness for major medical and governmental organizations

An honest assessment, dated **10 October 2026**, of whether this software can be
put into production at a large health body or a government department. The
reasoning and the evidence are in [`plan.md`](../plan.md) §58 to §63; the work is in
[`tasks.md`](../tasks.md) (items `RG-n`).

## Verdict

**Not ready for unsupervised production use. Ready for a controlled pilot** on
non-clinical, non-secret portfolio and project data, by an organization that
completes its own assurance (see the [assurance checklist](assurance-checklist.md)).

"Medical" here means a health organization using this as a **non-clinical**
portfolio and project tool: no patient records, no clinical decision support. A
clinical use would need a clinical safety case that this project has not written.

## What is strong

| Area | Evidence |
|---|---|
| Login enforced by default; the service refuses to start without keys | NFR-12; tests |
| Sign-in through an identity provider (including Microsoft Entra ID), with multi-factor authentication left to the provider | NFR-18; [sign-in guide](../operations/sign-in-with-entra.md) |
| Attribute-based access; private plans kept out of every aggregate | tests |
| Security headers, strict Content Security Policy, CORS allow-list, rate limiting | NFR-13, NFR-14 |
| Idle session timeout (30 minutes by default) and a hard session limit | RG-2; unit tests |
| User-supplied outbound destinations restricted to public `https` addresses; optional strict mode for operator targets | `sinks.rs`; RG-1 |
| Audit trail refuses change and deletion in the database | RG-3; request test |
| Data-subject export and pseudonymisation of a person across the schema (administrators only; preview first) | RG-6; request test; a test keeps the column list in step with the inventory |
| Access review: the grants that stand, oldest first, as a spreadsheet for recertification (administrators only) | RG-9; request test |
| Attachments: content must match the declared type, programs refused, optional `clamd` scan that fails closed | RG-8; unit and request tests |
| Break-glass access to private plans: eligible people only, a stated reason, at most four hours, a second person told or no grant, every request audited | RG-10; end-to-end test with real tokens |
| Handling marks: a configurable classification scheme, plans above your clearance do not exist for you (administrators too), the mark on every response | RG-7; end-to-end test with real tokens |
| Backup with a **tested restore**, stated recovery targets, optional encryption | [operations pack](../operations/README.md); RG-5; CI drill (plain and encrypted) |
| Retention purge for notifications, delivery logs and trend snapshots (dry run by default) | RG-4; request test |
| Dependency policy, software bill of materials, build provenance, a security policy | `cargo deny`; `release.yml`; [`SECURITY.md`](../SECURITY.md) |
| Accessibility: automated WCAG 2.2 AA audit of every route | [statement](accessibility-statement.md) |
| Information governance documents a buyer can adapt | this folder |

## What is not ready

| Gap | Why it matters | Item |
|---|---|---|
| **No penetration test, no third-party audit, no certification** | Assessors will ask | RG-13 (buyer or supplier) |
| **No manual or assistive-technology accessibility test** | Public-sector accessibility regulations expect it | RG-13 |
| **Data-subject tool covers references, not free text** | Names typed into titles, comments or documents, uploaded files, audit snapshot JSON and backups are not found automatically | RG-6 follow-up |
| **Retention covers only part of the data** | Plans, tasks, time records and documents are kept until an operator deletes them | RG-4 follow-up; buyer decision |
| **Handling marks hide and label, they do not encrypt** | A configured scheme hides plans above a caller's clearance (administrators too) and labels every response; a mark is not stamped into the bytes of a generated file, and the clearance is only as good as the identity provider's attribute | RG-7 |
| **Attachment checks are signature-level; no scanner is bundled** | A program renamed `.pdf` is refused and a `clamd` hook exists, but macros and exploits in valid documents need a real scanner, which the deployer must run (the hook was tested only against a stand-in) | RG-8 |
| **The access review reports what was recorded, not what the identity provider enforces** | Recertification must compare the two | RG-9 |
| **Performance is known only roughly, and a very large plan is slow** | A load and soak test exists and its results are in [`BENCHMARKS.md`](../BENCHMARKS.md): ordinary requests answer in tens of milliseconds, but a plan of 12,667 tasks is slow to schedule (about 0.3 s alone, about 0.5 s with four at once, for a 10.8 MB answer that is now refused above 5,000 tasks unless asked for; the task list now pages and answers a page in about 6 ms) and the process grows to ~1 GB. No capacity plan exists; run the script on your own hardware | RG-15, RG-16 (done as far as stated) |
| **Sessions are in memory unless Redis is configured** | The default needs one front-end replica or sticky sessions; `OIDC_SESSION_STORE=redis` removes that, and makes Redis a dependency of sign-in | RG-12 |
| **The audit trail is not tamper-proof against an administrator** | A database administrator can drop the trigger; there is no hash chain | stated in the DPIA |
| **Single maintainer; no service-level agreement** | Continuity and support risk | [`GOVERNANCE.md`](../GOVERNANCE.md) |
| **The Welsh and other translations are drafts** | Statutory language duties | [language statement](language-statement.md) |

## What a first regulated deployment should do

1. Complete the [assurance checklist](assurance-checklist.md), starting with a penetration test and an accessibility audit.
2. Fill every "Buyer to complete" row in the [DTAC checklist](dtac-checklist.md) and the markers in the [DPIA](dpia.md).
3. Agree retention periods with the records manager and schedule the purge.
4. Turn on encrypted backups and run the restore drill on your hosting.
5. Keep the data non-clinical and below the marking level the deployment is assured for.
6. Plan for the gaps above, or fund their removal.

## What this document does not claim

No certification, approval or audit is claimed. This is a self-assessment by the
project, and the checks listed under "strong" prove what they test.
