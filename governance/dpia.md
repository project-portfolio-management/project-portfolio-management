# Data protection impact assessment (template)

Follows the usual DPIA structure (describe the processing; consider consultation;
assess necessity and proportionality; identify and assess risks; identify
measures; record the outcome). It is **pre-filled with what the software does**,
so you start from facts. Everything marked `‹buyer›` is yours to complete, and
the assessment is not finished until a person with authority has signed it.

*Version:* ‹date› · *Assessor:* ‹buyer› · *Data protection officer's advice:* ‹buyer›

## 1. Why a DPIA

The service is a workplace system that holds **references to identifiable staff
and contractors** together with their assignments, recorded time, availability,
leave notes, cost rates and the audit trail of what they did. That combination
is monitoring-adjacent and so warrants an assessment even though no
special-category data is collected by design.

## 2. Describe the processing

**Nature.** Users enter and edit portfolio records (programmes, projects,
tasks, risks, budgets, milestones). People are recorded as opaque references,
not as names or contact details. The service derives views (capacity, workload,
utilization, schedules, forecasts) from those records. Every change is written
to an audit trail whose rows carry integrity digests. Users may attach files and may set a personal
notification destination (an e-mail address or a chat webhook).

**Scope.** The categories and every column are listed in the
[personal-data inventory](personal-data-inventory.csv) and summarized in the
[record of processing](record-of-processing.md). *Volumes, the number of people
and the period covered:* ‹buyer›.

**Context.** Staff and contractors are the data subjects. They are in an
employment or engagement relationship with the controller, so there is a power
imbalance and consent is rarely the right lawful basis. They would expect
assignment and time data to be recorded; they would not expect it to be used as
a productivity score (see risk 1). *Prior concerns, trade-union or staff
representative views:* ‹buyer›.

**Purposes.** Governing and delivering the change portfolio; planning capacity;
accounting for effort and cost; accountability and assurance. *Benefit to the
organization and to individuals:* ‹buyer›.

**Data flows.** See the [data-flow diagram](data-flow.md).

## 3. Consultation

*Who was consulted (staff representatives, the DPO, information security, the
supplier) and what they said:* ‹buyer›.

## 4. Necessity and proportionality

| Question | Answer from the software | Buyer |
|---|---|---|
| Lawful basis | Not decided by the software; see the record of processing | ‹buyer› |
| Does it achieve the purpose? | Yes: assignments, time and cost are needed to plan and account for work | ‹buyer› |
| Is there a less intrusive way? | The design minimizes: people are **references** held elsewhere, not copies of their details; per-person detail is masked from people who may not manage people | ‹buyer› |
| Data minimization | No names, e-mail addresses (other than a user's own notification destination), dates of birth, or special-category fields exist in the schema | ‹buyer› |
| Accuracy | Records are corrected in place with the old value kept in the audit trail; timesheets are submitted and decided by the people concerned | ‹buyer› |
| Storage limitation | See the [retention schedule](retention-schedule.md); **a purge task covers read notifications, delivery logs and trend snapshots (`cargo loco task retention_purge`, dry run by default); the personal records themselves (plans, tasks, time, documents) are not purged, which is the buyer's decision** (gap) | ‹buyer› |
| Transparency | Provide a privacy notice that names this processing | ‹buyer› |
| Individuals' rights | See section 6 | ‹buyer› |
| Processors | None built in; list yours | ‹buyer› |
| International transfers | None by the software; depends on your hosting, backups, mail, chat webhooks and identity provider | ‹buyer› |

## 5. Risks and measures

Rate each risk's likelihood and severity for **your** organization
(remote / possible / probable; minimal / significant / severe) and choose an
outcome (eliminated / reduced / accepted). The "Measures in the software" column
is what exists today; every claim points at a test, a route or a document.

| # | Risk to individuals | Measures in the software | Residual gap | Likelihood · Severity · Outcome |
|---|---|---|---|---|
| 1 | **Monitoring and profiling**: time, workload and utilization figures used to rank or discipline people | Recorded effort never becomes a per-person productivity metric and never replaces calendar time in a flow ratio (spec §2.1); per-person utilization is served only under five stated obligations and is never the sole ranking key (`spec/time-based-analysis.md`); an absent figure is `null` with a reason, never zero; a caller who may not manage people sees roles and totals only | Policy and training; the obligations bind the software, not how managers use a screenshot | ‹buyer› |
| 2 | **Excessive access** to people-linked data (timesheets, availability, cost rates) | Deny-by-default access policy; finance and people data are separate policy areas; private plans are invisible to non-members in every aggregate (tested across ~55 routes, `tests/private_aggregates.rs`); guests are confined to one plan; another person's availability record is `404`, not `403` | The policy must be configured to your roles | ‹buyer› |
| 3 | **Special-category data written into free text** (for example a health reason in a leave note) | Operator guidance forbids it; export masks person references by default; notes are plain text with no structure that invites it | **No technical block.** Train users; consider disabling the note fields' use | ‹buyer› |
| 4 | **Unauthorized access** through stolen credentials or a vulnerable page | Login enforced by default and refused-off in production; short-lived tokens verified offline; strict content security policy and security headers; per-caller rate limiting with a tighter bucket for failed sign-ins; every change audited | TLS, network isolation, MFA at your identity provider, penetration testing are yours | ‹buyer› |
| 5 | **Loss of data or of availability** | Backups of the database and artifact store; a tested restore (CI restores a dump and runs the test suite on it); stated recovery targets ([operations pack](../operations/recovery-objectives.md)) | Backup storage, encryption and its access control are yours | ‹buyer› |
| 6 | **Keeping data too long** | A retention class on every personal-data column; the schedule states periods | **No purge task; soft-deleted rows and expired files remain.** Operate manually until built | ‹buyer› |
| 7 | **Cannot meet a data-subject request** (access, rectification, erasure, restriction, portability) | Access and portability: the audited per-plan export (`GET /api/plans/{pid}/export`), masked unless elevated; rectification: edit in place with history; restriction: soft delete hides a record | **Erasure is pseudonymisation, not deletion**: an administrator can replace a person's reference with a stable pseudonym everywhere it is held, audit trail included (`POST /api/admin/subjects/{ref}/pseudonymise`, a preview unless `?apply=true`; the original is not written to the audit row). Records stay linked to each other but no longer to the person. **Not found automatically:** names or e-mail addresses typed into free text, the content of uploaded files, audit snapshot JSON, notification destinations, backups. Set `PSEUDONYM_SALT` to a secret so the pseudonym cannot be recomputed from a known identifier. Decide your policy and document it | ‹buyer› |
| 8 | **Disclosure through exports, reports, feeds or notifications** | Exports and PDFs follow the same authorization as the screen and mask references by default; feed tokens are per-user, hashed, revocable; webhook targets are operator-configured and loopback is refused by default | Chat webhooks a user sets send notification text to a third-party service; forbid at your network edge if unacceptable | ‹buyer› |
| 9 | **Tampering or denial of actions** | Each audit row carries a SHA-256 and a SHA3-256 digest and, when an audit key is configured, a keyed MAC; an edited row is detected by verification (`GET /api/compliance/records/verify` and `GET /api/compliance/audit/verify`) | A database trigger refuses any `UPDATE` of an audit row and any `DELETE` outside the retention task (migration 51), so an application bug or a casual statement cannot alter or remove one. **It does not stop a database administrator**, who can drop the trigger; and there is no hash chain, so a deleted row would not show in verification. Set the audit key, restrict database administration, keep backups out of administrators' reach | ‹buyer› |
| 10 | **Automated decisions about people** | There are none: scores rank **plans** (the Smart Score discloses its inputs), not people; no model runs in the service | If you add decision-making on people, reassess | ‹buyer› |
| 11 | **Transfers outside the UK or EEA** | None by the software | Depends on hosting, backups, mail, telemetry, chat webhooks and the identity provider | ‹buyer› |
| 12 | **Emergency access used wrongly**: a person opens a private plan without a real need | A private plan can be opened only by a person the identity provider marks `breakglass=eligible` (or an administrator), for at most four hours, with a stated reason of at least 20 characters; a second person is told in the same transaction or the grant is not made; every request it opens is audited (`break_glass_access`); grants are never deleted, can be revoked by the holder or an administrator, and are limited to five a day | Visibility only, and aggregate views opened by a grant are not audited per request; someone must read the grants (`GET /api/break-glass`, with the access review) | ‹buyer› |
| 13 | **A frozen pack keeps something it should not**: an issued director pack carries a title that names a person, or goes to an audience with no access to the live estate | A pack is computed only over plans that are neither private nor marked; it holds no person reference or cost rate; it is read only by the audiences named in the token (`pack_audience`), and a partner reader confined by `role=pack_reader` can reach nothing else; an issued pack cannot be changed or deleted by anyone, the database refusing it | Free text in issue and change-request titles is copied into the pack and cannot be edited afterwards; a correction is a reissue, which leaves the old version readable, so removing a title from an issued pack needs a decision of the controller and a database operation | ‹buyer› |

*Add risks specific to your organization:* ‹buyer›

## 6. Individuals' rights, and how to meet each

| Right | How |
|---|---|
| Be informed | Your privacy notice; this assessment and the record of processing as the source |
| Access | Export the person's records: the per-plan export, the auditors' trail explorer filtered by actor, and the person's own pages (`/me`) |
| Rectification | Edit the record; the audit trail keeps the earlier value |
| Erasure | Subject to your retention policy and legal duties; see risk 7 and the retention schedule's known gaps |
| Restrict | Soft delete removes a record from every view; it remains in the database |
| Portability | JSON and CSV exports |
| Object | Process to be defined by you; where the basis is public task or legitimate interests |
| Automated decisions | Not applicable (risk 10) |

## 7. Sign-off and review

| | Name | Date |
|---|---|---|
| Measures approved by | ‹buyer› | |
| Residual risks approved by | ‹buyer› | |
| DPO advice provided | ‹buyer› | |
| Consultation with the regulator needed? (high residual risk) | ‹buyer› | |
| Next review (at least yearly, and on any new feature that processes people's data) | ‹buyer› | |

## Keeping this current

`tests/personal_data_inventory.rs` fails the build when a migration adds a column
that can hold personal data that the inventory does not name. When it does,
add the row, decide its retention class, and **re-read the risk table above**.
