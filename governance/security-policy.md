# Information security policy

The policy behind the [vulnerability disclosure page](../SECURITY.md). [`SECURITY.md`](../SECURITY.md) says how to
report a vulnerability and what to expect; this document says what the project requires of itself, and where in the
repository each requirement is enforced or checked.

## Document control

| Field | Value |
| --- | --- |
| Title | Information security policy |
| Owner | The maintainer role ([`MAINTAINERS.md`](../MAINTAINERS.md)) |
| Version | 1.0 |
| Effective | 2026-11-17 |
| Next review | By 2027-11-17, and after any significant incident (section 18) |
| Approval | Approved by the maintainer; not reviewed by a third party |
| Applies to | The repository, its default configuration and its release artifacts (section 2) |

**Conventions.** The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY and
OPTIONAL are to be read as described in BCP 14 ([RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) and
[RFC 8174](https://www.rfc-editor.org/rfc/rfc8174)) when, and only when, they appear in capitals. Durations name their
kind, calendar days or business days (spec NFR-11).

**What "evidence" means here.** Each rule names the file, test or CI job that enforces or checks it, as the repository
stood on the effective date. Evidence proves what it tests and no more. Where nothing enforces a rule, the rule says so
and the gap is listed in section 17. Nothing here is a certification, an audit or evidence of a deployment's
conformity.

## 1. Purpose

1.1. This policy sets the security rules the project follows when it writes, reviews, builds and releases the
software, so that a contributor knows what is required and a deployer or assessor can see what the project does and
does not do.

1.2. It sits between the disclosure page ([`SECURITY.md`](../SECURITY.md)), the security requirements in the
specification ([`spec/07-non-functional-requirements.md`](../spec/07-non-functional-requirements.md), chiefly NFR-5,
NFR-6 and NFR-12 to NFR-14 and NFR-21) and the [information governance pack](README.md). Where this policy and the
specification disagree, the specification governs behaviour of the software and this policy MUST be corrected.

## 2. Scope

2.1. **In scope:**

- the source in this repository: the service (`project-portfolio-management-service-with-rust`), the matcher library
  (`project-portfolio-management-matcher-with-rust`), the operator UI (`project-portfolio-management-ui-with-svelte`),
  the scripts, the CI workflows and the documentation;
- the default configuration the repository ships (`compose.yaml`, the Dockerfiles, the service's defaults);
- the release artifacts: the service container image published to GitHub Container Registry
  (`ghcr.io/project-portfolio-management/project-portfolio-management`), and the CycloneDX SBOMs and release notes
  attached to the GitHub release ([`release.yml`](../.github/workflows/release.yml)).

2.2. **Out of scope.** A deployer's infrastructure and hosting, its identity provider (including multi-factor
authentication and conditional access), its database host, its network, its keys and secrets, its backups, its
monitoring and its incident process. The deployer's own information security policy governs those. The
[assurance checklist](assurance-checklist.md) lists what only the deployer can do. Third-party services and
denial of service by volume are out of scope, as [`SECURITY.md`](../SECURITY.md#scope) says.

2.3. The UI container image is built and smoke-tested in CI but is **not** a release artifact; a deployer who uses it
builds it themselves.

## 3. Roles and responsibilities

3.1. **Maintainer** (one person; [`MAINTAINERS.md`](../MAINTAINERS.md), [`GOVERNANCE.md`](../GOVERNANCE.md)). Owns this
policy. MUST review and merge every change, cut every release, receive and triage security reports, decide exceptions
(section 16) and review this policy (section 18). Is accountable for every merged change, whatever tool produced it
([`AI_STATEMENT.md`](../AI_STATEMENT.md)).

3.2. **Contributors.** MUST follow [`CONTRIBUTING.md`](../CONTRIBUTING.md) and [`AGENTS.md`](../AGENTS.md), MUST NOT
commit secrets or real personal data, MUST report suspected vulnerabilities privately as
[`SECURITY.md`](../SECURITY.md) says and not in a public issue, and MUST state in the pull request what was not done
or not verified ([pull request template](../.github/pull_request_template.md)).

3.3. **Reviewers.** The maintainer is the only human reviewer. Four reviewer agents in
[`.claude/agents/`](../.claude/agents/) (`security-reviewer`, `database-reviewer`, `rust-reviewer`, `ui-reviewer`)
check a change against [`agents/review-checklist.md`](../agents/review-checklist.md), which is the one list of what a
reviewer checks; they add no rule of their own and MUST NOT approve or merge. Their output is advice to the
maintainer, not an independent review (section 17).

3.4. **Deployers.** Responsible for everything out of scope in section 2.2: configuration, keys, the identity
provider, TLS, backups, monitoring, their own assessment and their own incident and breach process. SHOULD run the
latest release (only it and `main` receive fixes) and SHOULD complete the [assurance checklist](assurance-checklist.md)
before holding real data.

## 4. Authentication and session handling

4.1. Login enforcement on `/api/*` MUST be on by default. Only the health probes, the OpenAPI documents, Swagger UI and
the per-user calendar feeds are public; `/metrics.prom` MUST need a token (spec NFR-12).
*Evidence:* `src/auth/enforcement.rs`; unit tests `resolve_require_auth_is_on_unless_explicitly_off`,
`enforce_on_allows_public_paths`, `the_old_health_paths_are_not_public` and `metrics_requires_a_token` in
`src/auth/tests.rs`; request test `enforcement_on_gates_the_real_router` (`tests/enforcement.rs`); the CI `container`
job's step "Login enforcement is on (anonymous API call is refused)".

4.2. The service MUST refuse to start when enforcement is off outside `development`, `test` or `container-dev`, and
when enforcement is on with no token verifier and no key URL (fail closed; spec NFR-12).
*Evidence:* `startup_verdict` and `check_startup` in `src/auth/enforcement.rs`; unit test
`startup_verdict_refuses_the_unsafe_postures`.

4.3. Bearer tokens MUST be verified offline: PASETO `v4.public` (Ed25519) signature, issuer, audience and expiry,
against the published key set (spec NFR-5). Keycloak and OIDC verifiers are feature-gated alternatives.
*Evidence:* `src/auth/verifier.rs` (the `authentication-verifier` crate); unit tests `valid_token_yields_claims`,
`empty_verifier_rejects_everything`.

4.4. A token's scope MUST only narrow access: `scope=readonly` and `scope=capture` are decided before the policy file,
which cannot widen them (spec NFR-5; review checklist, Security 7).
*Evidence:* `tests/read_only_scope.rs` (`a_read_only_token_reads_and_is_refused_every_write`), `tests/capture_scope.rs`
(`a_capture_token_posts_three_things_and_is_refused_the_rest`).

4.5. Failed authentication MUST be rate limited: a client address that earns 20 `401`s by default is locked out
until its window ends; every lockout is counted and logged without the address (spec NFR-14).
*Evidence:* `src/rate_limit.rs` and its unit tests; `tests/hardening.rs`
(`the_real_router_carries_headers_cors_and_rate_limits`); alert rule in
[`operations/prometheus/alerts.yml`](../operations/prometheus/alerts.yml).

4.6. In the UI, tokens from an OpenID Connect sign-in MUST stay on the server. The browser holds one opaque session
cookie that is `httpOnly`, `Secure`, `SameSite=Lax` and host-locked (`__Host-` prefix). The sign-in is the
authorization-code flow with PKCE (S256). The browser MUST NOT hold a token or send `Authorization` (spec NFR-18;
review checklist, Security 8).
*Evidence:* UI `src/lib/server/session.ts`, `src/lib/server/oidc-session.ts`, `src/lib/server/oidc.ts`; unit tests
`tests/unit/oidc.test.ts`; end-to-end `tests/e2e/oidc-signin.spec.ts` (run in the CI `ui`
job twice, the second time with sessions in Redis).

4.7. A UI session MUST end after an idle period, 30 minutes by default and configurable (`OIDC_IDLE_MINUTES`; spec
NFR-21).
*Evidence:* UI `src/lib/server/config.ts`, `src/lib/server/oidc-session.ts`, `src/hooks.server.ts`; unit tests in
`tests/unit/session-store.test.ts` and `tests/unit/oidc.test.ts`.

4.8. Multi-factor authentication and conditional access are the identity provider's, and so the deployer's
(section 2.2; [sign-in guide](../operations/sign-in-with-entra.md)).

## 5. Authorization and least privilege

5.1. Access MUST be decided from token attributes (attribute-based access): a token with no attributes reads but
cannot write; `access=write` writes but cannot do destructive actions; `access=admin` can; a configured deny beats a
later allow.
*Evidence:* `src/auth/policy.rs`; unit tests `abac_empty_attrs_reads_but_cannot_write`,
`abac_access_write_writes_but_not_destructive`, `abac_access_admin_allows_destructive`,
`abac_configured_deny_beats_later_allow`, `derive_action_matrix` in `src/auth/tests.rs`.

5.2. **Private plans.** A private plan MUST be `404` to a non-member, through every route: every
`/api/plans/{pid}/…` route goes through the plan guard, and every route outside it MUST be declared in
`plan-guard-census.txt` with how it treats a plan the caller cannot read (review checklist, Security 1).
*Evidence:* `src/controllers/plan_access.rs` (census test `every_route_outside_the_guard_is_declared`);
`tests/plan_access.rs`; `tests/private_routes.rs` (for example `a_field_of_the_body_is_no_way_into_a_private_plan`,
`a_reference_is_no_way_into_a_private_plan`, `what_you_were_told_goes_when_your_access_does`);
`tests/requests/security_regression.rs`.

5.3. A view that adds up, lists or counts across plans MUST route its rows through `HiddenScope`, so a private plan
contributes to no total, and every new parameterless `GET` MUST be named in the aggregate census (review checklist,
Security 2).
*Evidence:* `tests/private_aggregates.rs` (`census_of_what_a_product_plan_touches`,
`a_private_plan_is_in_no_aggregate_for_a_non_member`).

5.4. Elevated access to a private plan MUST be break-glass only: eligible people, a stated reason, at most four
hours, a named second person, every request audited (spec NFR-21). Handling marks MUST hide a plan above a caller's
clearance, administrators included.
*Evidence:* `domain/src/break_glass.rs`, `tests/break_glass.rs`; `src/handling_marks.rs`, `tests/handling_marks.rs`.

5.5. The UI's server-side proxy MUST refuse a cross-site write, and the service MUST refuse a cross-site write whose
origin is not on the allow-list.
*Evidence:* UI `src/lib/server/proxy-guard.ts`, `tests/unit/proxy-guard.test.ts`, `tests/e2e/proxy-guard.spec.ts`;
service `tests/requests/cross_site.rs`.

5.6. In the repository and CI, workflows MUST default to `permissions: contents: read` and grant more only to the job
that needs it (the release `image` job's `packages: write`, `id-token: write` and `attestations: write`; the `release`
job's `contents: write`). Checkouts SHOULD set `persist-credentials: false`.
*Evidence:* [`ci.yml`](../.github/workflows/ci.yml), [`release.yml`](../.github/workflows/release.yml); the `pins`
job runs `zizmor` over `.github`.

5.7. The service container MUST run as a non-root user.
*Evidence:* `USER portfolio` in `project-portfolio-management-service-with-rust/Dockerfile`.

## 6. Input validation and output encoding

6.1. Request bodies MUST be validated by the service and refused with every problem at once (`422`).
*Evidence:* `domain/src/validation.rs` and its unit tests; fuzz targets `validate_json` and `validate_built`
(`project-portfolio-management-service-with-rust/fuzz/`).

6.2. Free text from a request MUST NOT be put into SQL. Every value MUST be bound; a table, column or condition in a
format string MUST come from a constant, and every statement MUST run through `src/db.rs` (review checklist,
Security 5 and Database 6; spec §8.3b).
*Evidence:* the census tests `the_raw_pool_is_recognised` and `nothing_outside_this_file_takes_the_raw_pool` in
`src/db.rs`; reviewer checklist. No automated check proves that every format string's parts are constants; that is
left to review (section 17).

6.3. Code that parses a path in a guard MUST percent-decode first and fail closed (review checklist, Security 3).

6.4. Uploaded files MUST match their declared type; programs MUST be refused whatever they claim to be; an optional
`clamd` scan MUST fail closed when configured. Upload and import routes MUST carry a body limit.
*Evidence:* `src/content_check.rs` and its unit tests (`a_program_is_refused_whatever_it_claims_to_be`,
`arbitrary_bytes_never_panic`); `tests/requests/attachments.rs`; `body_limit` in `src/controllers/attachments.rs` and
`src/controllers/mspdi_import.rs`.

6.5. A user-supplied outbound destination (webhook, sink) MUST be `https` to a public address: no credentials, no
loopback, private, link-local or internal names, checked again on the resolved address (server-side request forgery).
*Evidence:* `domain/src/sinks.rs` (`destination_url_ok`, `resolved_ok`) and its unit tests; `tests/sinks.rs`.

6.6. **Output encoding.** Every API response MUST carry the security headers of spec NFR-13 (`nosniff`, a restrictive
CSP, `X-Frame-Options: DENY`, `Referrer-Policy`, `Permissions-Policy`, COOP, CORP, and HSTS over TLS). The UI MUST set
a strict CSP with no `'unsafe-inline'` for scripts or style elements, so computed styles use Svelte's `style:`
directive, never a `style="…"` attribute. CORS MUST be an exact allow-list with no wildcards.
*Evidence:* `src/http_security.rs` (unit tests `cors_list_is_exact_and_refuses_wildcards`,
`the_permissions_policy_is_no_weaker_than_axas_default`); `tests/hardening.rs`; UI `src/lib/security-headers.ts` and
`tests/e2e/security-headers.spec.ts`; the CI `container` job's header check; the CI `zap` job (ZAP baseline, every
finding fails except the two reviewed in [`.zap/rules.tsv`](../.zap/rules.tsv)).

## 7. Cryptography and secrets

7.1. The project MUST NOT implement its own cryptographic primitives. What the code uses, as of the effective date:

| Purpose | Mechanism | Where |
| --- | --- | --- |
| Token verification | PASETO `v4.public` (Ed25519) | `authentication-verifier` crate; `src/auth/verifier.rs` |
| Keyed integrity of plan and audit rows | HMAC-SHA256 with HKDF domain separation, off unless `PORTFOLIO_INTEGRITY_MAC_KEY` (or `…_KEY_FILE`) is set | `integrity-mac` crate; `src/compliance/mac.rs` |
| Unkeyed content digests | SHA-256 and SHA3-256 | `src/compliance/record_integrity.rs`, `src/compliance/audit_integrity.rs` |
| Stored feed and link tokens | random token; only its SHA-256 is stored | `domain/src/feeds.rs`, `domain/src/link_token.rs` |
| Transport | rustls for outbound HTTP, SMTP and Redis; PostgreSQL over TLS | `Cargo.toml`; spec [`08-architecture.md`](../spec/08-architecture.md) |
| Encrypted backups | GnuPG: symmetric AES-256 with a passphrase file, or a public-key recipient | [`scripts/backup.sh`](../scripts/backup.sh) |

7.2. In production, a database URL that leaves the machine MUST set `sslmode` (enforced by `axa`; spec
[`08-architecture.md`](../spec/08-architecture.md)). A database outside the Compose stack SHOULD use `verify-full`;
the shipped `compose.yaml` uses `require`, which encrypts but does not check the certificate (section 17).

7.3. **No secrets in the repository.** Secrets, keys, tokens and passwords MUST NOT be committed; the service and UI
MUST read them from the environment or from a `…_FILE` path (spec NFR-5). Test keys and fixtures MAY be committed only
when they protect nothing, and each such finding MUST be listed with its reason in `.gitleaksignore`.
*Evidence:* the CI `secrets` job runs gitleaks 8.30.1 (its release checksum verified) over the whole history; the
`container` and `zap` jobs boot with a public-only test key set (`scripts/smoke-keys.sh`); `compose.yaml` refuses to
start without `POSTGRES_PASSWORD`.

7.4. Key rotation and custody are the deployer's. The integrity MAC supports retired keys
(`PORTFOLIO_INTEGRITY_MAC_KEYS_RETIRED`) so a key can be rotated without losing verification of older rows.

## 8. Logging, audit and monitoring

8.1. Every write MUST be audited in the same transaction as the change (who, what, when, old and new JSON); if the
audit row cannot be written, the write MUST be refused. A sensitive read (a data-subject export, the access review,
a break-glass request) MUST NOT be served unaudited (spec NFR-6; review checklist, Security 4). Named exceptions are
logged and counted, and a census keeps their list from growing.
*Evidence:* `tests/requests/audit_failure.rs` (`a_write_whose_audit_row_cannot_be_written_is_refused`,
`a_failed_audit_write_is_counted_not_dropped`); `tests/outbox_audit.rs`.

8.2. The database MUST refuse an `UPDATE` or `DELETE` of an audit row except from the data-subject tool or the
retention task (spec NFR-21).
*Evidence:* migration `migration/migrations/20261010000051_audit_logs_protected.up.sql`;
`tests/requests/audit_protection.rs` (`audit_rows_cannot_be_changed_or_deleted_without_the_flags`). The limits are in
section 17.

8.3. Logs SHOULD be structured (JSON in production) and MUST NOT contain tokens, secrets or client addresses for
lockouts (spec NFR-7, NFR-14). No automated test checks log content for tokens or personal data (section 17).

8.4. Alert rules MUST ship with the service and MUST name only metrics the service registers: audit rows not written,
visibility lookups failing closed, lockouts, a high `5xx` rate, failed scheduled jobs and relay lag (spec NFR-7).
*Evidence:* [`operations/prometheus/alerts.yml`](../operations/prometheus/alerts.yml); unit test
`every_metric_an_alert_rule_names_is_registered` (`src/metrics.rs`); the CI `docs` job runs `promtool check rules` and
the rules' unit tests. Thresholds are starting points, not measured; wiring them to an alerting system is the
deployer's.

## 9. Data protection and privacy

9.1. The project MUST keep an accurate inventory of every column that can hold personal data, with its class, purpose
and retention class.
*Evidence:* [`personal-data-inventory.csv`](personal-data-inventory.csv); `tests/personal_data_inventory.rs`, which
fails when a migration adds a column the inventory does not name, and when an identifier column is not covered by the
data-subject tools.

9.2. The project MUST maintain the pre-filled [DPIA](dpia.md), [data-flow diagram](data-flow.md),
[record of processing](record-of-processing.md) and [retention schedule](retention-schedule.md) (spec NFR-17), and MUST
state in them what the software does not do. Completing and signing them is the deployer's, as controller.

9.3. Retention purges MUST be dry runs unless told otherwise and MUST audit what they remove; the data-subject tool
MUST be administrator-only and preview first (spec NFR-21).
*Evidence:* `src/retention.rs`, `tests/requests/retention.rs`; `domain/src/subjects.rs`, `tests/requests/subjects.rs`.

9.4. Exports MUST mask personal data by default, and a policy's `mask` obligation MUST redact the record on the
ordinary endpoint and in the export.
*Evidence:* `tests/masking.rs`; `tests/requests/bulk.rs`
(`export_masks_by_default_and_a_zero_row_export_is_still_audited`).

9.5. Contributors MUST NOT put real personal data in issues, tests, fixtures or reports
([`SECURITY.md`](../SECURITY.md),
[`PRIVACY.md`](../PRIVACY.md)).

## 10. Secure development lifecycle

10.1. **Specification first.** A behavioural change MUST be one commit with a specification edit, a code edit and a
test edit ([`agents/spec-driven-development.md`](../agents/spec-driven-development.md),
[`spec/18-change-control.md`](../spec/18-change-control.md)).

10.2. **Review.** Every change MUST be reviewed by the maintainer against
[`agents/review-checklist.md`](../agents/review-checklist.md); a change to authentication, authorization, private
plans, audit, SQL or secrets SHOULD also be put through the `security-reviewer` agent.

10.3. **Gates.** Before a change is pushed, the gates in [`AGENTS.md`](../AGENTS.md#gates) MUST pass:
`cargo fmt --check`,
`cargo clippy --all-targets --all-features -- -D warnings`, the unit tests, the database tests on PostgreSQL 18, and
the UI's lint, type check, unit and Playwright tests (spec NFR-9). CI runs them on every push to `main` and every pull
request, and the single aggregate job `ci-ok` fails if any job fails, is cancelled or is skipped
([`ci.yml`](../.github/workflows/ci.yml)).

10.4. A guard or census test MUST NOT be weakened to get green; the cause is fixed or the reason recorded
([`AGENTS.md`](../AGENTS.md); review checklist, Rust 7).

10.5. **Security testing.** The project runs, besides the gates:

- a passive ZAP baseline scan of the running stack on every CI run (the `zap` job);
- a restore drill of a real backup, plain and encrypted (the `restore` job);
- mutation testing of the matcher: the changed lines on every CI run (the `mutants` job, a merge gate) and every
  mutant weekly ([`mutants.yml`](../.github/workflows/mutants.yml)); the service is not yet mutation tested
  ([`BENCHMARKS.md`](../BENCHMARKS.md#mutation-testing-measured-2026-11-17));
- weekly fuzzing of five targets in the service and the matcher ([`fuzz.yml`](../.github/workflows/fuzz.yml));
- a weekly chaos test of database and service faults under load ([`chaos.yml`](../.github/workflows/chaos.yml),
  `scripts/chaos-test.sh`);
- line coverage of the service's workspace by every test suite, the database suites included, measured with
  cargo-tarpaulin on every CI run (the `coverage` job) and reported, with no threshold yet
  ([`COVERAGE.md`](../COVERAGE.md) has the baseline and the places tests are most needed).

The fuzz and chaos runs are not merge gates; a red run MUST be read and either fixed or recorded in
[`tasks.md`](../tasks.md).

10.6. Every `CHANGELOG.md` entry MUST say what was not done and what was not verified ([`AGENTS.md`](../AGENTS.md)).

## 11. Dependency and supply-chain management

11.1. **Lockfiles.** Every Rust crate and every pnpm project MUST commit its lockfile, and CI MUST build from it
(`cargo build --locked`, `pnpm install --frozen-lockfile`; the `fresh-clone` and `ui` jobs).

11.2. **Rust policy.** `cargo deny check` (advisories, licences, bans, sources) MUST pass for the service and the
matcher on every CI run and before every release (the `deny` jobs in [`ci.yml`](../.github/workflows/ci.yml) and
[`release.yml`](../.github/workflows/release.yml)). An ignored advisory MUST carry its reason in the crate's
`deny.toml`. Unused dependencies are refused by the `machete` job.

11.3. **Updates.** Dependabot MUST watch every ecosystem the repository uses (cargo, npm, GitHub Actions, Dockerfiles
and Compose files), weekly, with a cooldown of 7 calendar days before a new release is proposed
([`.github/dependabot.yml`](../.github/dependabot.yml)). pnpm's default minimum release age (one day) applies to the
UI; a package taken inside it MUST be named in `minimumReleaseAgeExclude` in the UI's `pnpm-workspace.yaml`, with the
reason in the changelog. Cargo has no equivalent setting in this repository (section 17).

11.4. **Pinning.** Every GitHub Action MUST be pinned to a full commit SHA, and every container image (workflow
services, Compose `image:`, Dockerfile `FROM`) to a `sha256` digest.
*Evidence:* `scripts/check-actions-pinned.py` and `zizmor`, both in the CI `pins` job.

11.5. **Toolchain.** The Rust toolchain MUST be pinned exactly in [`rust-toolchain.toml`](../rust-toolchain.toml)
(1.100.0), and the CI workflows' `dtolnay/rust-toolchain` input MUST equal it (checked by the `fresh-clone` job). The
fuzz workflow uses a dated nightly. Downloaded tools SHOULD be pinned by version with `--locked`, or by checksum (as
`cargo-fuzz`, `rumdl` and gitleaks are); `cargo-cyclonedx` in the release `sbom` job is installed `--locked` but with
no version (section 17).

11.6. A dependency held back on purpose (for example `axum-test` and `jsonwebtoken`, which follow `axa` and the
authentication code; TypeScript 7) MUST be named in `dependabot.yml` with its reason and is an exception under
section 16.

## 12. Vulnerability management

12.1. **Sources.** Reports under [`SECURITY.md`](../SECURITY.md) (GitHub private vulnerability reporting, on since
2026-11-17, or email); RustSec advisories through `cargo deny`; Dependabot;
findings of the ZAP scan, fuzzing, chaos test and review.

12.2. **Severity.** A confirmed vulnerability MUST be scored with CVSS v4.0 (base, and threat metrics where known) and
classed by score. The maintainer MAY raise a class for a reason recorded with the fix (for example, a private plan
exposed to non-members is treated as at least High).

12.3. **Targets.** The project has one maintainer, so these are targets, not guarantees, consistent with
[`SECURITY.md`](../SECURITY.md) (acknowledge within 5 business days; say what will be done within 15 business days).
Each is measured from triage, the point at which the maintainer confirms the vulnerability.

| Class | CVSS v4.0 score | Target for a fixed release or a published mitigation |
| --- | --- | --- |
| Critical | 9.0 to 10.0 | 15 business days |
| High | 7.0 to 8.9 | 30 calendar days |
| Medium | 4.0 to 6.9 | 90 calendar days |
| Low | 0.1 to 3.9 | The next planned release, or accepted with a reason under section 16 |

12.4. A fix MUST be released as a patch version of the latest release and noted in the changelog. A missed target
MUST be recorded in [`tasks.md`](../tasks.md) with the reason and a new date. An advisory in a dependency that cannot
be fixed at this level MUST be handled as an exception (section 16).

## 13. Incident response

13.1. A security incident in scope is a confirmed or suspected compromise of the repository, the CI, a release
artifact or a signing identity, or a vulnerability being exploited. An incident in a deployment is the deployer's
(section 2.2); the [runbook](../operations/runbook.md) gives the technical steps and its Escalation section points to
the deployer's own incident and breach process.

13.2. The maintainer MUST handle an in-scope incident in these steps, recording the time and what was done at each:

1. **Detect and assess**: confirm what happened and its class (section 12.2).
2. **Contain**: for example revoke a leaked token or key, disable a workflow, delete or deprecate an affected image
   tag, or pull a release.
3. **Eradicate**: remove the cause (fix the code, rotate every credential the attacker could have reached, remove a
   compromised dependency).
4. **Recover**: release a fixed version from a clean build and confirm the gates pass.
5. **Disclose**: publish an advisory and a changelog entry saying what was affected, which versions, what a deployer
   should do and what was not verified; credit the reporter if they wish.
6. **Learn**: add a test or a gate that would have caught it, and review this policy (section 18).

13.3. **Coordinated disclosure.** The default disclosure window is 90 calendar days from the report, or the release
of a fix if sooner. The maintainer and the reporter MAY agree a different date; a vulnerability being exploited MAY be
disclosed sooner, with a mitigation.

## 14. Release integrity

14.1. A release MUST be a `vX.Y.Z` tag whose version equals the service crate and UI package versions, and whose
release notes exist in the service changelog; the workflow MUST refuse otherwise (the `verify` job).

14.2. A release MUST pass `cargo deny` before an image is built, and the image MUST be built from a pinned base image
by digest with `--locked`.

14.3. A released image MUST carry build provenance and an SBOM: BuildKit provenance (`mode=max`) and SBOM, and a
GitHub build-provenance attestation pushed to the registry (`actions/attest-build-provenance`, signed through the
workflow's OIDC identity). CycloneDX SBOMs for both crates and the UI MUST be attached to the GitHub release, each
checked against its lockfile (`scripts/check-sbom.py`).

14.4. A deployer SHOULD pull the image by digest (the release notes give it) and MAY verify its attestation with
`gh attestation verify`.

14.5. What release integrity does not yet include is in section 17 (no re-run of the tests in the release workflow,
no signed tags, a build cache in the image job).

## 15. Backup and recovery

15.1. The repository MUST ship backup and restore scripts for the database and the artifact store, written recovery
targets and an upgrade guide (spec NFR-16): [backup and restore](../operations/backup-and-restore.md),
[recovery objectives](../operations/recovery-objectives.md) (RPO 24 hours and RTO 4 hours for scheduled logical
backups), [upgrade and migration](../operations/upgrade-and-migration.md), [deployment](../operations/deployment.md).

15.2. A restore MUST be tested automatically: the CI `restore` job backs up a database holding real rows, restores it
into a fresh database, compares the two and runs the request suite against the copy, plain and with an encrypted
backup.

15.3. Backups that leave the machine SHOULD be encrypted (`BACKUP_PASSPHRASE_FILE` or `BACKUP_GPG_RECIPIENT`), with
the key kept outside the backup location. Running, scheduling, monitoring (`scripts/check-backup-age.sh`) and testing
backups on a real host is the deployer's.

15.4. Released migrations MUST NOT be edited; their checksums are checked, and a change is a new migration
([`AGENTS.md`](../AGENTS.md)).

## 16. Exceptions

16.1. A departure from a MUST or SHOULD in this policy, or from a gate, is an exception. An exception MUST be decided
by the maintainer and MUST be recorded, in the same change that introduces it, with:

- what is excepted and why;
- what limits the risk meanwhile;
- an expiry date or the event that ends it (for example "when `loco-gen` is removed").

16.2. The record MUST be in [`tasks.md`](../tasks.md) and in the changelog entry of the change, and, where the tool
has one, next to the setting: `deny.toml` `ignore` entries, `.gitleaksignore`, [`.zap/rules.tsv`](../.zap/rules.tsv),
`ignore` entries in [`.github/dependabot.yml`](../.github/dependabot.yml), `minimumReleaseAgeExclude` in the UI's
`pnpm-workspace.yaml`.

16.3. Expired exceptions MUST be reviewed at each release and at each policy review, and either removed or renewed
with a new reason and date. The exceptions present on the effective date predate this policy and carry reasons but no
expiry dates (section 17); they MUST be given one at the first review.

16.4. **Exceptions recorded under this policy.**

| Exception | Reason and what limits the risk | Expiry |
| --- | --- | --- |
| `RUSTSEC-2026-0543` (chrono), `RUSTSEC-2026-0542` (humantime, only with the `fluvio` feature) ignored in the service's `deny.toml` | Declared unmaintained, not vulnerable; no safe upgrade. The service moved to `jiff` on 2026-11-17: its own code, its `sqlx` `chrono` and `schemars` `chrono04` features and `chrono-tz` are gone, so the `RUSTSEC-2026-0555` (chrono-tz) exception is **cleared** and removed. `chrono` now arrives only through `axum-test`'s `expect-json` (a dev-dependency: tests only, never in a built service) and `fluvio` (the optional `fluvio` feature, not in the default build or the image); `cargo tree -i chrono -e normal` without features prints nothing. A vulnerability advisory in any of them is not ignored and still fails `cargo deny`. Added 2026-11-17; narrowed 2026-11-17. | 2027-05-17, or when `axum-test` and `fluvio` drop `chrono`, whichever is sooner |

## 17. Known gaps

What this policy requires or a reader might assume, and the repository does not do, as of the effective date. Each is
an open item, not a hidden one.

**Assurance and people**

1. No penetration test, third-party audit, independent code review or certification
   ([production readiness](production-readiness.md); [assurance checklist](assurance-checklist.md) items 1 and 9).
2. One maintainer: there is no separation of duties. The person who writes or accepts a change also reviews, merges and
   releases it. The reviewer agents are automated advice, not an independent reviewer.
3. No security training or background checks; nothing in this repository can attest the maintainer's account security
   (for example multi-factor authentication on GitHub), and none is claimed.

**Repository settings** (observed through the GitHub API on 2026-11-17; they are settings, not files, so no test keeps
them true)

1. The `main` branch has no branch protection and no ruleset, so `ci-ok` is not a required check and a push to `main`
   is not blocked by a failing gate.
2. GitHub secret scanning, push protection and Dependabot security updates are off. Secrets are caught by gitleaks in
   CI after the push, not before; a RustSec advisory fails `cargo deny` in CI, but no security-update pull request is
   raised, and version updates wait for the weekly run and the 7-day cooldown.
3. Commit signing is the maintainer's local practice, not enforced; the release tag `v0.2.0` is annotated but not
   signed.

**Release**

1. The release workflow does not re-run the tests and does not check that the tagged commit passed CI; with no branch
   protection, nothing ensures it did.
2. The release `image` job uses the GitHub Actions build cache (`cache-from: type=gha`), although the `sbom` job's own
   comment says a release job must not use one, because a poisoned cache must not reach a published artifact.
3. Only the service image is published; the UI image is not a release artifact, so it has no provenance attestation.
4. `cargo-cyclonedx` is installed in the release `sbom` job without a pinned version.
5. No cosign or maintainer-key signature on the image beyond the GitHub attestation; the SBOMs on the release are not
   themselves attested.

**Supply chain**

1. No advisory scan of npm dependencies (UI, site, root tooling) runs in CI; `cargo deny` covers Rust only.
2. No static analysis beyond clippy and ESLint (no CodeQL or similar).
3. No publish-age cooldown for Cargo outside Dependabot: a manual `cargo update` takes a release published today.
4. `deny.toml` treats unknown registries and git sources, yanked crates, wildcards and duplicate versions as warnings,
   not failures.
5. The existing exceptions (`deny.toml` advisory ignores, `.gitleaksignore`, `.zap/rules.tsv`, Dependabot ignores,
   about 28 `minimumReleaseAgeExclude` entries) have reasons but no expiry dates.

**Software**

1. The audit trail is not tamper-proof against a database administrator: the trigger can be dropped, `TRUNCATE` is
   not blocked, there is no hash chain, and the keyed MAC is off until a key is configured (`src/compliance/mod.rs`).
2. The shipped `compose.yaml` connects to PostgreSQL with `sslmode=require`, which does not verify the certificate.
3. UI sessions are in memory unless Redis is configured, and rate-limit and lockout state is per process.
4. No automated test checks that logs exclude tokens, secrets or personal data.
5. Proof that SQL format strings contain only constants is by review, not by a test.
6. The data-subject tool finds references, not names typed into free text, files, audit snapshots or backups; the
   retention purge covers only part of the data ([retention schedule](retention-schedule.md#known-gaps)).
7. Attachment checks are signature-level; no virus scanner is bundled, and the `clamd` hook was tested only against
   a stand-in.
8. A calendar feed's token travels in its address (`/api/feeds/calendar.ics?token=…`), as calendar apps need, so it can
   be recorded wherever the address is: a deployer's reverse proxy or load-balancer access log, the subscriber's
   calendar settings, a shared screenshot. Checked 2026-11-18: the service's request span records only the method and
   status (`src/observability.rs`, `trace_mw`), and the UI's proxy (`/api/proxy`) logs no address. Only the token's
   hash is stored, it expires after 365 days, a new token replaces the old, and the owner can revoke it
   (`src/controllers/feeds.rs`). A deployer should keep query strings out of its access logs for that path.

**Process**

1. No incident has been handled under section 13 and no incident exercise has been run; the steps are untested.
2. The vulnerability targets in section 12.3 have not been measured against any real report.
3. Fuzzing and the chaos test are weekly and not merge gates.

## 18. Compliance and policy review

18.1. Compliance with this policy is checked mainly by the CI gates and census tests named in each section, and by
the maintainer's review against [`agents/review-checklist.md`](../agents/review-checklist.md). A rule with no named
evidence is checked only by review, and section 17 says so.

18.2. The maintainer MUST review this policy at least once every 12 months, after any significant incident, and when
the architecture, the release process or the scope changes. The review MUST check that every cited path, test and job
still exists and still does what is claimed, update section 17, give each exception an expiry, and record the result
in the changelog with a new version number.

18.3. A change to this policy is a documentation change made under
[`spec/18-change-control.md`](../spec/18-change-control.md).
A change that weakens a MUST MUST say why in the changelog.

18.4. Legal and regulatory obligations of a deployment (data protection law, sector rules) are the deployer's;
[`spec/12-compliance.md`](../spec/12-compliance.md) states what the software was designed against.

## 19. Mapping to ISO/IEC 27001:2022 Annex A

As in the [assurance mapping](assurance-mapping.md), this table was written from general knowledge of the standard;
**the standard's text was not read for it and the control numbers were not checked against it**. It shows where an
assessor might look, not conformity: the controls of an information security management system are the deployer's,
and nothing here is certified. The themes are organizational (5), people (6), physical (7) and technological (8).

| Section of this policy | ISO/IEC 27001:2022 Annex A controls (examples) | Assurance checklist item |
| --- | --- | --- |
| Document control, 1 Purpose, 18 Review | 5.1 Policies for information security; 5.36 Compliance with policies, rules and standards; 5.35 Independent review (not done) | — |
| 2 Scope | 5.23 Information security for use of cloud services; 5.19 Supplier relationships (the deployer's side) | 6, 8 |
| 3 Roles and responsibilities | 5.2 Information security roles and responsibilities; 5.3 Segregation of duties (a gap); 5.4 Management responsibilities | 8 |
| 4 Authentication and session handling | 5.16 Identity management; 5.17 Authentication information; 8.5 Secure authentication | 7 |
| 5 Authorization and least privilege | 5.15 Access control; 5.18 Access rights; 8.2 Privileged access rights; 8.3 Information access restriction; 8.4 Access to source code | 7, 11 |
| 6 Input validation and output encoding | 8.26 Application security requirements; 8.28 Secure coding; 8.7 Protection against malware (attachments) | 9 |
| 7 Cryptography and secrets | 8.24 Use of cryptography; 5.17 Authentication information | — |
| 8 Logging, audit and monitoring | 8.15 Logging; 8.16 Monitoring activities; 5.28 Collection of evidence | 12 |
| 9 Data protection and privacy | 5.34 Privacy and protection of PII; 5.33 Protection of records; 8.10 Information deletion; 8.11 Data masking | 4, 14 |
| 10 Secure development lifecycle | 8.25 Secure development life cycle; 8.27 Secure system architecture and engineering principles; 8.29 Security testing in development and acceptance; 8.32 Change management; 8.33 Test information; 5.8 Information security in project management | 1, 9, 10 |
| 11 Dependency and supply chain | 5.19 Supplier relationships; 5.21 Managing information security in the ICT supply chain; 8.9 Configuration management | 8 |
| 12 Vulnerability management | 8.8 Management of technical vulnerabilities; 5.7 Threat intelligence; 6.8 Information security event reporting | 1 |
| 13 Incident response | 5.24 Incident management planning and preparation; 5.25 Assessment and decision on events; 5.26 Response to incidents; 5.27 Learning from incidents; 6.8 Event reporting | 12 |
| 14 Release integrity | 8.32 Change management; 8.31 Separation of development, test and production environments; 5.21 ICT supply chain | — |
| 15 Backup and recovery | 8.13 Information backup; 8.14 Redundancy; 5.29 Information security during disruption; 5.30 ICT readiness for business continuity; 5.37 Documented operating procedures | 5 |
| 16 Exceptions | 5.1 Policies (exceptions); 5.36 Compliance with policies | — |
| 17 Known gaps | 5.35 Independent review; 5.3 Segregation of duties | 1, 9 |

The people (6) and physical (7) themes are largely out of scope: the project has no staff, premises or equipment of its
own, and a deployment's are the deployer's.
