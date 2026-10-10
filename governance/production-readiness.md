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
| **No data-subject export or erasure tool** | A subject-access or erasure request is handled by hand | RG-6 |
| **Retention covers only part of the data** | Plans, tasks, time records and documents are kept until an operator deletes them | RG-4 follow-up; buyer decision |
| **No handling marks** (security classification or sensitivity labels) | Governments and health bodies label records and exports | RG-7 |
| **No attachment content checks or malware-scan hook** | Uploaded files are stored as sent | RG-8 |
| **No access-review report; no break-glass procedure** | Periodic recertification is a common control | RG-9, RG-10 |
| **No load or soak test** | Capacity is unknown ([`BENCHMARKS.md`](../BENCHMARKS.md)) | RG-11 |
| **Sessions are in memory** | Several front-end replicas need sticky sessions | RG-12 |
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
