# For the Chief Security Officer

*Chief Security Officer: what the software enforces, what the project has verified, and what you must test and operate yourself.*

## What the software enforces

| Area | Behaviour |
|---|---|
| Authentication | Login is on by default. The service will not start without token-verification keys, and will not start in production with login off. Sign-in through an identity provider such as Microsoft Entra ID; multi-factor authentication stays with the provider ([sign-in guide](../../../../operations/sign-in-with-entra.md)) |
| Authorisation | Attribute-based policy on token attributes. Private plans are kept out of every aggregate for everyone; handling marks hide plans above a caller's clearance, administrators included |
| Break-glass | Access to private plans needs eligibility, a stated reason, at most four hours, and a second person told; every request is audited |
| Sessions | Idle timeout (30 minutes by default) and a hard session limit |
| Transport and browser | Security headers, a strict Content Security Policy, a CORS allow-list, per-caller rate limiting |
| Outbound | User-supplied destinations are restricted to public `https` addresses |
| Uploads | Content must match the declared type; programs are refused; an optional `clamd` scan fails closed |
| Audit | The audit trail refuses change and deletion in the database |
| Frozen records | Issued director packs and delivery-confidence assessments are immutable or append-only by database trigger |
| Supply chain | `cargo deny`, a software bill of materials, build provenance on release, a [security policy](../../../../SECURITY.md) |

## What is not done

- No penetration test, third-party audit, independent code review or certification.
- No bundled malware scanner; the `clamd` hook was tested only against a stand-in.
- The audit trail can be defeated by a database administrator; there is no hash chain.
- Handling marks hide and label; they do not encrypt, and a mark is not stamped into generated files.
- Sessions are held in memory unless Redis is configured.
- The data-subject tool finds references, not names typed into free text, uploaded files or backups.

## Operating it

Backups have a tested restore, optional encryption and stated recovery targets ([operations pack](../../../../operations/README.md)). The [runbook](../../../../operations/runbook.md) gives the technical incident steps; the incident and breach process is yours. Report a vulnerability privately as [SECURITY.md](../../../../SECURITY.md) describes; the targets are acknowledgement in 5 business days, not guarantees.

## Decisions that are yours

Who performs the penetration test and when; the identity provider's conditional-access and role mapping; the marking level this deployment is assured for; the scanner behind the attachment hook; how often access is recertified.

## Read next

[Assurance checklist](../../../../governance/assurance-checklist.md) · [Production readiness](../../../../governance/production-readiness.md) · [Data flow](../../../../governance/data-flow.md)
