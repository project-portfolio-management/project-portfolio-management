# Supplier assurance for a managed, hosted deployment (proposed 2026-11-20)

> **Status: proposed. None of SA-1 to SA-21 is built or done.** Tasks are in [`tasks.md`](../tasks.md). The decisions
> SA-D1 to SA-D4 belong to the owner. The first buyer in view is NHS Wales, so the names here are NHS Wales names, but
> every software requirement applies to any deployment.

## What it is for

A supplier that hosts this service in a UK cloud, supplies it under a licence and supports it for a health board, trust or
Digital Health and Care Wales (DHCW) becomes the **data processor** under UK GDPR Article 28. It is also a **third-party
supplier**. The buyer assesses the supplier's organization as well as the open-source project. This spec separates three
kinds of requirement:

- **what the software must do** (SA-R1 to SA-R13), which this repository can build and test;
- **what the supplier must be or hold** (SA-O1 to SA-O9), such as contracts, certificates, vetting and tests by a third
  party. No commit can meet these;
- **what the buyer keeps** (the shared-responsibility matrix below).

The source is a pasted procurement briefing on NHS Wales supplier onboarding. It was **not checked** against the current
text of the Welsh Information Governance Toolkit (WIGTK), Welsh Health Circular WHC (2017) 025, the NHS Wales Records
Management Code of Practice or any NHS Wales contract template. Read those before anything here is quoted to a buyer
(SA-20).

## This is not an HR system

The briefing was written for an HR application that holds sickness, disciplinary and pay records. **This service holds
none of those.** People are opaque references (§12.3), and the workforce service holds personal detail (§2.0, §8.5a).
What comes closest here:

| Briefing's category | Nearest data here | Where |
| --- | --- | --- |
| Pay | Cost rates per person (`cost_rate_tables.person_ref`, `cost_rates.per_hour_minor`) and the costed time stored with each entry (`time_entry_costs`) | PJ-2. Rates sit in the finance ABAC area and are never shown to guests |
| Sickness and absence | Work calendars (weekdays and holidays). A person's absence reason is **not stored** | `work_calendars` |
| Disciplinary | Nothing. Free text (issue text, time-entry notes, lessons) could carry it, which operator guidance forbids | §12.1 |
| Performance | Per-person time and utilization (`time_entries.actor_ref`), served only under the five obligations of `spec/time-based-analysis.md` | |

So the briefing's "field-level RBAC for sickness, disciplinary and pay" becomes two requirements here. Per-person rates
and costs must be separated from what plan roles can read (SA-R5), and special-category HR fields must never be added
(SA-R6).

## Software requirements

Each requirement states what is already true, checked against the code on 2026-11-20, and what is missing.

- **SA-R1 — Single tenant per deployment.** One deployment serves one customer organization, with its own database, its
  own artifact store and its own backups. No table has a tenant column, and none may gain one. A supplier serving several
  customers runs several deployments. *Today:* true by design; no test states it. *Missing:* a census test that fails when
  a migration adds a `tenant`, `organization_id` or similar discriminator column (SA-1), and a sentence in the deployment
  guide. SA-R13 makes sure each service opens its own customer's database.
- **SA-R13 — One customer, one database: the service knows which database is its own.** When a supplier hosts several
  customers, only the `DB_URL` setting ties a running service to its database. A configuration copied from the wrong
  deployment, or a typing mistake in a deploy script, could start customer B's service against customer A's database.
  Customer B's staff would then read and change customer A's plans, people references, time and costs. SA-R1 keeps the
  data of different customers apart. SA-R13 makes sure each service opens the right database. The rule:
  1. Each deployment is given a name with `PROJECT_PORTFOLIO_MANAGEMENT_DEPLOYMENT_ID`, for example `acme-example`: 1
     to 63 lowercase letters, digits and hyphens, starting with a letter. Any other value stops the start.
  2. **First start.** The database has no name yet. The service writes the configured name into a new one-row table,
     `deployment` (`id` fixed at 1, `deployment_id`, `recorded_at`), writes an audit row, then starts as normal.
  3. **Every later start.** The service compares the configured name with the stored one before it runs a migration,
     starts a scheduled job or relay, or serves a request. If they are equal it starts. If they differ, it **refuses to
     start**. It exits non-zero with an error that names both values and the database host and name, but not the
     password. It reads no other row and writes nothing.
  4. **Name not set.** In production (NFR-12) the service refuses to start. In development and test it starts with a
     warning and writes no name, so local and CI databases need no setting.
  5. **Renaming** is deliberate. An operator task (`cargo run -- task deployment_rename`) takes the old and the new name,
     changes the row only when the old name matches, and writes an audit row. No HTTP route can read or change the name.
  A database restored from a backup keeps the name, which is correct: it holds the same customer's data. A restore into a
  scratch database for the restore drill needs the same setting, or development mode. *Today:* nothing checks this; the
  wrong `DB_URL` starts the service against the other customer's database. *Missing:* everything (SA-21). Limits: a
  database that existed before this check is named by the first start after the upgrade, so that start trusts its
  configuration. The operator should check that start by hand. The check also cannot catch two deployments given the
  same name. The deployment guide should state that names are unique per supplier.
- **SA-R2 — Data stays where it is put.** The service makes no outbound transfer of its own (§12.3). Every outbound target
  is set by the operator: webhooks, the WPM bridge, the email relay, the event relay, the virus-scan hook and the OIDC
  issuer. Strict mode (NFR-21, `sinks.rs`) holds operator targets to public `https` addresses. It does not hold them to a
  region. *Missing:* a single list in the
  deployment guide of every setting that names an outbound host, so a supplier can show the buyer that each one is in the
  UK or belongs to the buyer (SA-2). A test keeps that list complete as settings are added.
- **SA-R3 — No local passwords and no vendor backdoor.** The service keeps no passwords and has no built-in account. Every
  caller is a verified token from the configured identity provider (NFR-5, NFR-18), and the service will not start in
  production with login off (NFR-12). *Missing:* a test asserting that no route accepts a password or issues its own
  credential, and that no subject is hard-coded with access; a documented Entra ID sign-in with multi-factor authentication
  enforced at the provider (SA-3). NADEX is reached through Entra ID or a Keycloak realm. **SAML 2.0 is not supported
  directly:** a Keycloak realm brokers it (SA-D2).
- **SA-R4 — Supplier support access is granted, limited in time and recorded.** A support engineer acting in the
  application gets access only through a grant that a customer administrator approves. The grant names its reason and
  expires within a stated number of hours. Every request served under it is audited, as break-glass access is today
  (NFR-21). *Today:* break-glass covers private plans only, and it is self-granted with a witness. No grant needs approval
  from a second party who belongs to the customer. *Missing:* a support-grant kind that requires customer approval before it
  takes effect, and an audit-feed view of every request it served (SA-4). Infrastructure access (shell, database, cloud
  console) cannot be controlled by this software. SA-O5 covers it.
- **SA-R5 — Per-person rates and costs separated from plan access.** A token that can read a plan does not, for that
  reason alone, read per-person cost rates or the per-person cost of time. Only the finance area grants those, and a token
  that holds `access=admin` without the finance area does not either. *Missing:* a request test for each route that returns
  `per_hour_minor`, `cost_minor` or a per-person cost total, and a census of those routes like CF-8's (SA-5). Per-person
  utilization keeps its five obligations.
- **SA-R6 — No special-category HR fields.** No migration adds a column for absence or sickness reasons, occupational
  health, disciplinary matters, grievances or pay grades. The personal-data inventory census (NFR-17) refuses such column
  names, and the inventory states the rule. *Missing:* the name list and the refusal (SA-6). Operator guidance on free text
  stays as it is.
- **SA-R7 — Audit trail that shows tampering.** The audit trail is append-only, and the database refuses changes (NFR-21).
  *Missing:* a hash chain over `audit_logs` (`prev_hash`, `hash`) with checkpoints taken by an outside witness, which the
  sibling person and case services already have. The `src/compliance` module records that this is absent. A verify route
  must report the first broken link (SA-7). Reads of per-person rates and costs and of break-glass or support-grant data
  are audited (SA-R4, SA-R5).
- **SA-R8 — Audit export to the customer's SIEM.** The customer's security information and event management system (SIEM)
  can collect every audit row continuously, without a person running a report. *Today:* `GET /api/auditor/trail` and
  `GET /api/auditor/evidence-pack` are pulled by a person, and their own use is audited. *Missing:* a cursor-paged audit
  feed for a machine reader holding a read-only token (NC-15), with rows in a documented JSON schema that a SIEM collector
  can poll, or a relay of audit rows through the outbox (SA-8). Retention follows the retention schedule, and the customer's
  SIEM keeps its own copy for as long as its own rules require.
- **SA-R9 — Encryption.** TLS 1.2 or 1.3 at the edge (NFR-5). The service sends `Strict-Transport-Security` (NFR-13).
  Storage encryption (AES-256) belongs to the hosting layer, and backups can be encrypted (NFR-21). *Missing:* a statement
  in the deployment guide of which layer encrypts what (SA-9), and a decision on **column-level encryption** for per-person
  rates and costs (SA-D3). Handling marks hide data but do not encrypt it (`src/handling_marks.rs`). Column-level encryption
  is **not built**, and it adds key management that the deployer would run.
- **SA-R10 — Exit: a complete structured export.** When the contract ends, the customer receives everything the deployment
  holds, in documented formats. *Today:* a `pg_dump` backup with its artifact archive and manifest (NFR-16), a per-plan
  export and a bulk plan export (§9.6). *Missing:* a whole-deployment export (an `operator export` task) that writes every
  table as CSV and JSON, with a manifest of counts and SHA-256 digests and the artifact files. A test reconciles its counts
  with the database (SA-10).
- **SA-R11 — Exit: erasure with a record.** After the customer confirms that the export arrived, the supplier destroys every
  copy: database, artifact store, backups and their keys. *Missing:* a runbook procedure that destroys the backup encryption
  keys (cryptographic erasure) and the hosted volumes, and writes a signed erasure record listing what was destroyed and when
  (SA-11). The software cannot certify the destruction of a cloud volume. The record states what the supplier did.
- **SA-R12 — Retention aligned with the buyer's code.** The retention schedule and the purge task (NFR-21) let the deployer
  set a period for each kind of record. *Missing:* a column in the retention schedule for the matching NHS Wales Records
  Management Code of Practice entry, left blank until someone reads the code (SA-12). Legal hold is not built
  (assurance checklist item 14).

## Supplier obligations (outside the code)

These belong to whichever organization supplies the service. The project can supply evidence for some of them (shown in
brackets) but can meet none of them.

- **SA-O1 — Data processing agreement** under UK GDPR Article 28 with the controller. It covers scope, sub-processors and
  breach notice within the contract's period (the briefing says 24 to 48 hours; SA-D4). *(DPIA, record of processing and
  data-flow diagram are in `governance/`.)*
- **SA-O2 — WIGTK supplier onboarding** through DHCW, completed before go-live.
- **SA-O3 — Cyber Essentials Plus**, the briefing's stated minimum, and **ISO/IEC 27001** covering the hosting and support
  operation. *(The assurance mapping lines up controls; certification is of an organization.)*
- **SA-O4 — Sub-processor register.** Cloud provider, email relay and backup storage, each with location and attestation
  (ISO/IEC 27001 or SOC 2). All are UK-only, including disaster-recovery replicas.
- **SA-O5 — UK-based, vetted support staff.** Every engineer with production access is UK-based and vetted to BPSS (or DBS
  where the buyer requires it). Privileged infrastructure access is just-in-time, uses multi-factor authentication, is
  recorded and is approved by the customer.
- **SA-O6 — Independent IT health check** each year by a CHECK or CREST tester. It covers the hosted application, the API
  and the cloud estate, and it tests role separation, including the supplier's own support access. Critical and High
  findings are fixed, and the fix is recorded. *(The CI ZAP scan is passive and is no substitute.)*
- **SA-O7 — Patch commitment.** A stated time from an upstream advisory to a deployed fix: **critical within 14 calendar
  days** in the briefing (SA-D1). *(`cargo deny` advisories in CI and release, Dependabot, CycloneDX SBOMs with each release.
  `SECURITY.md` states acknowledgement targets only, not a patch time, and no container image scan runs.)*
- **SA-O8 — Incident response**, staffed 24 hours a day, every day, and tested, with notice to the customer inside the
  contract's period. *(The runbook gives technical steps; alert rules ship, NFR-7.)*
- **SA-O9 — Licence statement.** The repository is offered under MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only or
  GPL-3.0-only (NFR-15). A supplier choosing MIT or Apache-2.0 has no duty to disclose source for hosted use. None of the
  five is the AGPL. The customer's configuration and data are never part of the source.

## Shared responsibility

| Area | Supplier | Customer (health board, trust or DHCW) |
| --- | --- | --- |
| DPIA and controller duties | Technical inputs (the governance pack, SA-R1 to SA-R13) | Completes and signs the DPIA (DPO and SIRO) |
| Hosting and backups | UK-only hosting, hardening, monitoring, backups, sub-processors | Approves location; reviews evidence through WIGTK |
| Code and patching | Tracks advisories, builds releases, patches (SA-O7), commissions the health check | Reviews summaries; approves maintenance windows |
| Identity | Maintains the OIDC integration and the ABAC role map | Accounts, multi-factor authentication and leavers in Entra ID or NADEX; assigns roles |
| Sensitive data | Enforces ABAC, rate separation (SA-R5) and export masking | Decides who may see rates and costs; audits use |
| Incidents | Monitors, contains, notifies within the contract's period | Notifies the ICO and Welsh Government; informs staff |
| Retention and exit | Purge tools, full export, erasure record (SA-R10 to SA-R12) | Sets retention periods; checks the export on exit |

## Decisions for the owner

- **SA-D1** — Whether the project itself states a patch time for critical advisories in `SECURITY.md`, or leaves it to each
  supplier. The project has one maintainer.
- **SA-D2** — Whether direct SAML 2.0 is in scope, or brokering through Keycloak is the answer.
- **SA-D3** — Whether to build column-level encryption for per-person rates and costs, given the key-management burden.
- **SA-D4** — The breach-notice period the project's documents assume: 24 hours, 48 hours, or left blank for the contract.

## Honest limits

- The briefing is secondary. No NHS Wales document was read for this spec.
- Most of what an NHS Wales assessor asks for is organizational (SA-O1 to SA-O9). Building every SA-R item would still not
  get a supplier through onboarding.
- The HR categories map onto this service only loosely. An assessor reading an HR checklist should be told that early, with
  the table above.
