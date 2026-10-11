# For the Chief Risk Officer

*Chief Risk Officer: the risks the software helps you see, and the risks of using it.*

## Risk it helps you manage

- **Risks, issues and change requests** recorded per plan; an issues register across readable plans; change requests with separation of duties.
- **Delivery confidence and exceptions:** stated confidence kept separate from computed RAG, flagged when they diverge by two steps, with an append-only history, and a 48-hour exception clock that does not restart when a rating worsens and is closed only by a recorded decision.
- **Honest absence:** missing data shows as "not assessed" or null with a reason, never as green or zero.
- **A frozen, versioned monthly pack,** so what was told to directors can be shown later exactly as issued.

## Risk of using it (the project's own register)

| Risk | State |
|---|---|
| No penetration test, third-party audit or certification | Open: buyer's to commission ([assurance checklist](../../../../governance/assurance-checklist.md)) |
| Retention enforced for only part of the data | Open; purge must be scheduled |
| Data-subject tool does not search free text, attachments or backups | Open |
| Audit trail can be defeated by a database administrator; no hash chain | Stated in the [DPIA](../../../../governance/dpia.md) |
| Handling marks hide and label, they do not encrypt | Stated |
| Attachment scanning needs a deployer-run scanner | Open |
| Single maintainer, no SLA | Open |
| Translations are drafts | Open |
| Non-clinical only; no clinical safety case | Do not enter clinical content |

## Controls the software enforces

Login by default, rate limiting, strict security headers, an idle session timeout (30 minutes) and a hard session limit, outbound destinations restricted to public `https`, signature-checked attachments, immutable audit rows, break-glass with audit. Each is covered by a test cited in [production readiness](../../../../governance/production-readiness.md).

## Decisions that are yours

Risk appetite for a pilot; the exception window (default 48 hours); which marking level the deployment is assured for; who reviews the access list and how often.

## Read next

[Production readiness](../../../../governance/production-readiness.md) · [DPIA](../../../../governance/dpia.md) · [Compliance](../../../../spec/12-compliance.md) · [SECURITY](../../../../SECURITY.md)
