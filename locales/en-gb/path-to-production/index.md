# Path to production

*What "production" means for this software, and how far it is from it, item by item.*

"Production" here means that an organisation runs the software for real work. Real plans, real people's names and
real decisions depend on it, an outage or a leak has consequences, and someone is answerable for it. At a large health
body or a government department, production also means that the organisation's own assurance has been passed: a
security test, a data protection impact assessment, an accessibility audit and, in the National Health Service, the
Digital Technology Assessment Criteria (DTAC).

## Where it stands

- No certification, approval or audit is claimed. This page is the project's own estimate.
- "Medical" means a **non-clinical** portfolio and project tool: no patient records and no clinical decision support.
- The project suggests starting with a pilot on non-clinical, non-secret data, run by an organisation that completes
  its own assurance.

## Readiness, item by item

Each figure is the project's estimate on 13 November 2026, not a measurement. Each slider starts at that estimate.
Moving a slider records your own view while you read; nothing is saved or sent.

- **Independent security assurance** — 0%. No penetration test, no third-party audit and no certification. A buyer or a supplier has to commission them (RG-13).
- **Accessibility** — 40%. An automated WCAG 2.2 AA audit runs on every route on every change. No manual test, and no test with assistive technology or with disabled people, has been done (RG-13).
- **Data-subject requests** — 50%. Administrators can export and pseudonymise a person across every reference in the schema, with a preview first (RG-6). Names typed into titles, comments or documents, uploaded files, audit snapshots and backups are not found automatically.
- **Retention** — 35%. A purge, dry run by default, covers notifications, delivery logs, idempotency keys and trend and adoption snapshots, with periods set per deployment (RG-4). Plans, tasks, time records, documents and the audit trail are kept until an operator deletes them.
- **Handling marks** — 60%. A configurable classification hides plans above a caller's clearance, administrators included, and labels every response (RG-7). The mark is not stamped into generated files, and the clearance is only as good as the identity provider's attribute.
- **Attachment safety** — 50%. Content must match its declared type, programs are refused, and an optional `clamd` scan fails closed (RG-8). No scanner is bundled, and the hook has been tested only against a stand-in.
- **Access review and emergency access** — 45%. A report of standing grants supports recertification (RG-9), and emergency access is time-limited and needs a stated reason (RG-10). The report shows what was recorded, not what the identity provider enforces; no route records a reviewer's decision; and an emergency-access notice can name someone who receives nothing while the grant still stands.
- **Performance and capacity** — 40%. A load and soak test on one machine is in [`BENCHMARKS.md`](../../../BENCHMARKS.md): ordinary requests answer in tens of milliseconds, a plan of about 12,700 tasks is slow to schedule, and the process grows to about 1 GB. There is no capacity plan, no test with many users and no run across several machines.
- **Sessions across several front-end servers** — 70%. Sessions are held in memory unless Redis is configured (RG-12); with Redis, signing in depends on it.
- **Audit trail integrity** — 50%. Writes are audited, and a database trigger refuses changes and deletions, with stated exceptions (RG-3). A database administrator can drop the trigger, and there is no hash chain.
- **Backup and recovery** — 80%. A restore is tested in continuous integration, plain and encrypted, against stated recovery targets (RG-5). Encryption is optional, and the drill runs on the project's own CI, not on the deployer's hosting.
- **Support and continuity** — 10%. One maintainer and no service-level agreement ([`GOVERNANCE.md`](../../../GOVERNANCE.md)).
- **Translations** — 20%. Text in languages other than English is machine-written until a speaker reviews it, and statutory language duties need reviewed text ([language statement](../../../governance/language-statement.md)).
- **Clinical use** — 0%. No clinical safety case has been written, and clinical use is out of scope.

## What a first deployment should do

1. Complete the [assurance checklist](../../../governance/assurance-checklist.md), starting with a penetration test and an accessibility audit.
2. Fill every "Buyer to complete" row in the [DTAC checklist](../../../governance/dtac-checklist.md) and the markers in the [DPIA](../../../governance/dpia.md).
3. Agree retention periods with the records manager and schedule the purge.
4. Turn on encrypted backups and run the restore drill on the deployment's own hosting.
5. Keep the data non-clinical and below the marking level the deployment is assured for.
6. Plan for the gaps above, or fund their removal.

The evidence for each item is in the [assessment](../../../governance/production-readiness.md).
