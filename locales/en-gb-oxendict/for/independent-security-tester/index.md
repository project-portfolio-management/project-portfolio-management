# Independent security testing

*For a buyer, an assessor or a tester: what independent security testing of this software should cover, what the project supplies to help, what has and has not been done, and what is left to you. This is guidance, not a test report and not legal advice.*

## Where things stand

**No independent security test has been done.** No penetration test, third-party audit, independent code review or certification exists for this software, and the project claims none ([production readiness](../../../../governance/production-readiness.md), [SECURITY](../../../../SECURITY.md)). A passive scan runs in the project's continuous integration (rules in `.zap/rules.tsv`), and the project's own tests check the controls listed below; neither is a substitute for a tester who did not write the software.

This is item 1 and item 9 of the [assurance checklist](../../../../governance/assurance-checklist.md). It belongs to the deployer, with a certified tester, against **your** deployment: the project cannot test your hosting, identity provider, network or keys.

## What to test

Test the deployment as it will run, with login enforced and your identity provider in front of it. Suggested scope, from the controls the software claims:

| Area | What the software claims | What a tester should try |
|---|---|---|
| Authentication | Login is on by default; the service will not start without verification keys or, in production, with login off; tokens are verified offline against published keys | Forged, expired, replayed and wrongly signed tokens; wrong issuer or audience; key rotation; a missing key set |
| Authorization | Attribute-based policy; record-level rules | Horizontal and vertical privilege escalation on every route; changing identifiers in paths and bodies; attributes a caller should not be able to add |
| Private plans and handling marks | A private plan, and a plan above a caller's clearance, is in no list, count, total, search result, feed or export for that caller, administrators included | Any route that leaks existence, a name, a count or a total of a plan the caller cannot read; error messages and timing that differ between "hidden" and "absent" |
| Break-glass | Eligible people only, a stated reason, at most four hours, a second person told | Use without eligibility; extending a grant; acting without the second person being told |
| Frozen and append-only records | The audit trail, issued director packs, delivery-confidence history, programme status history and project buffers refuse change and deletion in the database | Updating or deleting through the API, through bulk import, and through any route that touches the same rows |
| Separation of duties | A requester cannot approve their own change request | Approving as the requester, as an administrator, and by changing the approver after submission |
| Uploads and attachments | Content must match the declared type; programs are refused; an optional virus-scan hook fails closed | Polyglot files, renamed programs, oversized and malformed files, path tricks in names, and whether your scanner is really called |
| Outbound requests | User-supplied destinations are limited to public `https` addresses | Server-side request forgery to private, link-local and metadata addresses, redirects, DNS rebinding |
| Sessions and the browser | Idle timeout, a hard limit, an `httpOnly` cookie, a strict Content Security Policy, a CORS allow-list | Session fixation and reuse, cross-site request forgery, injection into the interface, clickjacking |
| Rate limiting | Per-caller limits | Bypass by changing identity, header or path; whether limits survive your proxy |
| Data-subject tools | Export and pseudonymization, administrators only, preview first | Use by a non-administrator; completeness against the [personal-data inventory](../../../../governance/personal-data-inventory.csv) |
| Backups | Optional encryption, a tested restore | Access to backup files and keys; restore into a clean environment |

## What the project supplies

- The source, the specification and the tests, so a code review can start from the claims.
- A machine-readable description of the API (`/api-docs/openapi.json`), with request and response schemas for the director-pack work; the list of routes is checked against the code by tests.
- A throw-away configuration with login off (`dev-insecure`, see the [README](../../../../README.md)) **for the tester's own machine only**, and a script that generates a test key set (`scripts/smoke-keys.sh`) so tokens with chosen attributes can be minted against a test deployment.
- The [data-flow diagram](../../../../governance/data-flow.md), the [runbook](../../../../operations/runbook.md) and the [recovery objectives](../../../../operations/recovery-objectives.md).
- A private way to report what is found: [SECURITY.md](../../../../SECURITY.md).

## Rules of engagement

Agree these in writing before any test; the project cannot set them for your environment.

- **Environment.** Test a copy with realistic but not real data. Do not test production unless you have decided to, in writing.
- **Authorization.** The tester has your written permission, a named contact, and a stated time window.
- **Out of bounds.** Denial of service by volume, social engineering of staff, and third-party services (your identity provider, your host) are out of scope unless the owner of each agrees. The project's own scope is in [SECURITY](../../../../SECURITY.md).
- **Data.** Real personal data is not used, not copied out, and not put in the report.
- **Keys.** Test keys are separate from production keys and are destroyed afterwards.

## Reporting and retest

- Findings about this software go to the maintainer privately, as [SECURITY](../../../../SECURITY.md) says, with how to reproduce, the version or commit, and the impact. The project's targets are acknowledgement in 5 business days and a plan in 15; they are targets, not guarantees. There is one maintainer and no service-level agreement.
- Findings about your deployment go to your own owner.
- Ask the tester to rate each finding, to say what was **not** tested, and to retest the fixes. A report that lists only what was found does not tell you what was missed.
- Keep the report, the scope, the dates and the retest with your own records. The project cannot hold them for you.

## Limits of this document

It lists areas to cover, not a complete test plan, and the claims in the table are the project's own. It does not certify anything, and a clean test is evidence about one deployment on one date.

## Decisions that are yours

Who tests, when, against what, with what data and under what rules; how findings are rated and by when they must be fixed; and whether the result is shared with your assessors.

## Read next

[Assurance checklist](../../../../governance/assurance-checklist.md) · [Production readiness](../../../../governance/production-readiness.md) · [Chief Security Officer](../chief-security-officer/index.md) · [Governance, Risk, Compliance](../governance-risk-compliance/index.md)
