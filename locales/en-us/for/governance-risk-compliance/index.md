# Governance, Risk, Compliance

*For the people who run governance, risk and compliance: what the software helps you evidence, what it enforces, and what you must supply. Not legal advice.*

## Governance

- **Decisions are recorded.** Change requests need a named approver who is not the requester, a stated reason of at least 10 characters, and are audited by actor; the database refuses an approval without a decider and reason.
- **A frozen monthly director pack.** Drafted once, issued by a person, immutable once issued; corrections are a reissue with a note. Read access is by audience.
- **Delivery confidence and exceptions.** Stated confidence is append-only and shown beside computed status, never blended. A red or amber-red rating opens an exception with a 48-hour clock (configurable) that only a recorded decision closes.
- **Project governance of the software itself:** [GOVERNANCE.md](../../../../GOVERNANCE.md), [change control](../../../../spec/18-change-control.md), [AI statement](../../../../AI_STATEMENT.md).

## Risk

- Risks, issues and change requests per plan; an issues register across readable plans.
- Missing data is shown as "not assessed", never as green.
- The project's own open risks (no penetration test, partial retention, single maintainer, draft translations) are in [production readiness](../../../../governance/production-readiness.md). Verdict: ready for a controlled pilot on non-clinical, non-secret data, not for unsupervised production.

## Compliance

| Area | Where | State |
|---|---|---|
| UK GDPR / EU GDPR | [DPIA](../../../../governance/dpia.md), [record of processing](../../../../governance/record-of-processing.md), [personal-data inventory](../../../../governance/personal-data-inventory.csv), [retention schedule](../../../../governance/retention-schedule.md), [data flow](../../../../governance/data-flow.md) | Pre-filled; `‹buyer›` markers are yours; retention only partly enforced |
| NHS DTAC | [DTAC checklist](../../../../governance/dtac-checklist.md) | Answered section by section; buyer and supplier rows open |
| Accessibility | [statement](../../../../governance/accessibility-statement.md) | WCAG 2.2 AA automated; no manual test |
| Language duties | [language statement](../../../../governance/language-statement.md) | Translations are drafts |
| ISO/IEC 27001 | [compliance spec](../../../../spec/12-compliance.md) | Deployment-side controls; not certified |
| HIPAA, EHDS, ONC/HTI, IEC 62304 | [compliance spec](../../../../spec/12-compliance.md) §12.4 | Not engaged (no patient data); stated, not manufactured |

## Evidence and assurance

The [assurance checklist](../../../../governance/assurance-checklist.md) lists 14 items an assessor will ask for, with owner and state. Most are open and belong to the deployer: penetration test, accessibility audit, disaster-recovery exercise, hosting assurance, identity-provider configuration, access recertification, incident process, records management. The audit trail refuses change and deletion in the database, but a database administrator can defeat it; there is no hash chain.

## Decisions that are yours

Lawful basis and retention periods; exception window; marking level for the deployment; who reviews access and how often; whether a legal-hold procedure is needed (none is built).

## Read next

[Governance pack index](../../../../governance/README.md) · [SECURITY](../../../../SECURITY.md) · [Compliance spec](../../../../spec/12-compliance.md)
