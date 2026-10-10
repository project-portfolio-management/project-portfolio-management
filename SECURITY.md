# Security policy

## Reporting a vulnerability

Please report a suspected vulnerability **privately**, not in a public issue.

- Email **joel@joelparkerhenderson.com** with the subject `SECURITY: project-portfolio-management`.
- Say what you found, how to reproduce it, which version or commit, and what you
  think the impact is. A proof of concept helps; please do not use it against
  anyone's live system.
- If the report involves personal data, describe it without attaching real
  personal data.

There is one maintainer (see [MAINTAINERS.md](MAINTAINERS.md)). The intent is
to acknowledge a report within **5 business days** and to tell you what will be
done within **15 business days**; these are targets, not guarantees. A fix is
released as a patch version and noted in the changelog, crediting you if you
wish. No bug bounty is offered.

## Supported versions

Only the latest release, and the `main` branch, receive security fixes. The
project is pre-1.0 and changes quickly.

## What the software already does

The defaults are described in [`spec/07-non-functional-requirements.md`](spec/07-non-functional-requirements.md)
(NFR-12 to NFR-14) and checked by tests:

- Login enforcement on `/api/*` is **on** by default; the service refuses to
  start without verification keys, and refuses `production` with login off.
- Security headers and a strict Content Security Policy on the UI; a CORS
  allow-list on the API; per-caller rate limiting.
- Tokens from an OpenID Connect sign-in stay on the server; the browser holds
  one opaque `httpOnly` cookie.
- `cargo deny` (advisories, licences, bans, sources) runs in CI; Dependabot
  watches the dependencies.

## What it does not claim

No third party has audited this software, and no certification is claimed. The
[information governance pack](governance/README.md) states known gaps (for
example, the audit trail uses digests and an optional MAC, and is not a hash
chain). A deployer is responsible for their own configuration, keys, backups
and assessment.

## Scope

In scope: the code in this repository and its default configuration. Out of
scope: your deployment's infrastructure, third-party services (an identity
provider, a database host), and denial-of-service by volume.
