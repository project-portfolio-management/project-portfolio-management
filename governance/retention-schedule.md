# Retention schedule

How long each kind of record is kept, and what the software does about it.

**Read this first.** The periods below are **starting proposals** for a public
body; they are not legal advice. Agree them with your records manager and
replace the "Proposed" column. The software currently **enforces only some** of
them (the "Enforced today?" column says which); the rest are policy that
needs a purge task to carry out (see "Known gaps").

Every record in the personal-data inventory has a retention class; this table
defines the classes. A test keeps the two in step.

| Class | Covers | Proposed retention | Enforced today? |
|---|---|---|---|
| `AUD` | The audit trail and integrity records: who changed what, plan merges, phase and task state history, approval-process step history | 6 years after the record they describe is closed; never shorter than the longest period of any record they cover | **No.** Rows are append-only and never deleted. Retention is by policy. |
| `OPS` | Operational plan records: plans, tasks, risks, budgets, milestones, objectives, controls, templates, saved views and reports | The life of the plan plus 6 years after it is closed | **No.** Deletion is a soft delete (`deleted_at`); nothing purges soft-deleted rows. |
| `PPL` | People-linked working records: timesheets, recorded time, allocations, RACI, availability and leave notes, resource roles, onboarding cases, guest access, cost rates | 6 years after the period for time and cost records; 2 years after the range ends for availability, leave notes and onboarding checklists; leaver records per your HR policy | **No.** Soft delete only. Cost rates are append-only history by design. |
| `COM` | Communications: comments (and their edit history), announcements, notifications, approvals, reviews, retrospective notes, meetings (agendas, minutes, outcomes), reactions, watchers | 2 years after the record they refer to is closed; notifications 90 calendar days after they were read | **No.** |
| `DOC` | Controlled documents and their versions; attachments and the text extracted from them | A controlled document keeps every version for as long as it is in force plus 6 years; an attachment follows the record it is attached to | **Partly.** Attachments are removed from view with their record; expired generated files return `404`, but the file is not yet deleted (see Known gaps). |
| `GEN` | Generated files and jobs: bulk import and export jobs, exports, scheduled report runs, idempotency keys and their stored answers | 24 hours for a bulk export file; 30 calendar days for a scheduled report artifact | **Partly.** The expiry is set (bulk: `BULK_TTL_HOURS`, default 24 hours; reports: `REPORT_TTL_DAYS`, default 30 calendar days) and an expired file is not served; no task deletes it yet. |
| `CRD` | Credentials: per-user feed tokens (only a hash is stored) | Until the user revokes them or leaves | **Yes**, in the sense that a user can revoke a token at any time and a revoked token stops working at once; hashes are not deleted. |
| `EVT` | Outgoing event outbox and webhook delivery log | 7 calendar days after delivery for the outbox; 30 calendar days for the delivery log | **Outbox: yes** (published rows are purged after `EVENT_RETENTION_DAYS`, default 7 calendar days). **Delivery log: no.** |
| `DRV` | Derived records that can be rebuilt: estate snapshots for trends, adoption snapshots, the search index | The trend series for 3 years; the search index is rebuilt and has no retention | **No** for snapshots; the index is derived. |

## Backups

A backup holds a copy of everything above, **including records the application
has removed from view**. Personal data therefore lives on in backups until the
backup is deleted. State a backup retention (for example 35 calendar days of
daily backups plus 12 monthly) and delete on that schedule; an erasure request
is honoured in the live system immediately and in backups when they age out.
See [Recovery objectives](../operations/recovery-objectives.md).

## Known gaps

These are things the schedule needs and the software does not yet do. Each is a
decision for the buyer to plan around until it is built:

1. **The retention purge is partial.** `project-portfolio-management task retention_purge` (dry run
   unless `apply:true`; one audit row per table purged) removes read
   notifications (90 calendar days), the webhook and notification delivery logs
   (30 calendar days) and the trend snapshots (3 years); each period can be
   overridden on the command line or, for every run including the scheduled
   job, by the deployment setting `PROJECT_PORTFOLIO_MANAGEMENT_RETENTION_<RULE>_DAYS`
   (WW-1.7; for example `..._RETENTION_NOTIFICATIONS_DAYS=30`). The defaults
   are this service's working choices, each with its reason in
   `src/retention.rs` (`Rule::basis`); none was taken from a law. `OPS`, `PPL`, the rest of `COM`, `AUD` and `DOC` records still
   accumulate until an operator removes them: how long to keep a plan, a
   timesheet or a document is the buyer's decision, and each needs its own rule
   (RG-4 follow-up).
2. **Expired generated files are not deleted**, only hidden.
3. **Soft-deleted rows are never purged**, so "deleted" data still exists in the
   database.
4. **Erasure of one person's references is pseudonymisation.** People are stored
   as opaque references. Removing a person's account at the identity provider
   stops them signing in but leaves their reference in historical records. An
   administrator can export everything held about a person and replace the
   reference with a stable pseudonym everywhere, the audit trail included
   (`/api/admin/subjects/...`; see the DPIA, risk 7). Free text, uploaded files,
   audit snapshots and backups are not found automatically.
5. **The audit-trail retention (`AUD`, 6 years) is not enforced in code.** It
   is policy only: no rule in the retention purge covers `audit_logs`, the rows
   are append-only and are never deleted, so they are kept for longer than the
   proposed period, not shorter. The 6 years is a starting proposal, not a
   period any one law sets for every country (WW-1.7); a deployment that must
   delete audit records after a period has no setting for it yet.
