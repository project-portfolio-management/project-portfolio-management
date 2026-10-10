# DTAC v2.0 assessment of Project Portfolio Management (PPM)

An assessment of this software against the questions in [checklist.md](checklist.md) (Digital Technology
Assessment Criteria, form v2.0, 24 February 2026). It is the answers a manufacturer would give, written from the
repository as it stood on **2026-10-10**, for the **service 0.3.0**, the **matcher library 0.2.0** and the **web
client 0.1.0**.

> ⚠️ **This is a self-assessment, written by the maintainer's AI coding assistant, and nobody independent has
> reviewed it.** It is **not** a submitted DTAC form and does not say the product passes. Every answer cites the
> file, requirement or test it rests on; where it cannot be answered from the repository, it says
> **manufacturer to complete** rather than invent a company, a certificate or a date. An answer without evidence
> is not "met". The project's own earlier, shorter mapping to the DTAC themes is
> [governance/dtac-checklist.md](../../../../../governance/dtac-checklist.md); this page answers the v2.0 form question
> by question.

## How to read it

| Assessment | Meaning |
| --- | --- |
| **Met** | The repository holds the evidence cited. |
| **Partly met** | Some of it is evidenced; the gap is stated. |
| **Not met** | The requirement is not satisfied, and this says so plainly. |
| **Not applicable** | The question does not apply to the product as built, with the reason (a commissioner may challenge it). |
| **Manufacturer to complete** | A fact about a company, a certificate or a date that only the manufacturer holds. |

## The product in one paragraph

PPM is a **portfolio and project management** system for an organization's own work: a registry of plans (a
portfolio, a programme, a project, with an optional label and any plan able to contain any other) with
explainable duplicate detection, and a workspace for tasks, critical-path scheduling, cost and earned value,
resources, risks and issues, and governance. It holds **no patient data, takes no clinical input and gives no
clinical output.** It is open-source software the buyer deploys and operates (a Rust service and a SvelteKit web
client over PostgreSQL); the maintainer does not host it, has no access to any deployment, and receives no
telemetry. It is **young software with one maintainer**: its own [readiness assessment](../../../../../governance/production-readiness.md)
says it is **not ready for unsupervised production use** at a large medical or governmental organization, and
**ready for a controlled pilot on non-clinical, non-secret data**.

## Where it stands

| Section | Result |
| --- | --- |
| A. Company information | **Mostly manufacturer to complete.** There is no company: the maintainer is one individual. |
| B. Value proposition | Described; the benefits are **not validated**. |
| C1. Clinical safety | Answered **not in scope** (portfolio and project records, no direct care). That answer can be challenged, and **no DCB0129 documentation exists**. |
| C2. Data protection | The manufacturer holds and processes **no** deployed data. A DPIA template, data-flow diagram, record of processing and retention schedule are provided; **no ICO registration evidence, no privacy notice**; storage location is the deployer's. |
| C3. Technical security | **Would not pass as it stands:** no Cyber Essentials certificate, the Cyber Security Charter is unsigned, and **no independent penetration test** has been done. Several controls are built and tested. |
| C4. Interoperability | JSON/OpenAPI API; **no patient-identity exchange** (no NHS number, PDS or NHS login), so the identity questions fall away. |
| D1. Usability and accessibility | Scored, not pass/fail: an automated WCAG 2.2 AA audit of every page runs on every change; **no manual or assistive-technology audit, no user testing**, no availability figure. |

One further fact a reader should know: **the project's continuous integration runs on GitHub and, at the time of
writing, five of its jobs fail** (the restore drill, the generated-documents check, `cargo deny` for the service,
the ZAP baseline scan and the PostgreSQL integration tests; the other jobs pass). That is being investigated and
is not hidden here; see C3.4.

## A. Company information (non-assessed)

| Code | Answer | Assessment | Evidence or gap |
| --- | --- | --- | --- |
| A1 | None. PPM is maintained by an individual, Joel Parker Henderson, as sole maintainer | Manufacturer to complete | [MAINTAINERS.md](../../../../../MAINTAINERS.md). No company is named anywhere in the repository |
| A2 | Project Portfolio Management (PPM): the service and its web client, with the matcher library | Met | [README.md](../../../../../README.md) |
| A3 | Service 0.3.0, matcher 0.2.0, web client 0.1.0. Pre-1.0: no release has been cut from the root release workflow | Met | `Cargo.toml`, `package.json`, [CHANGELOG.md](../../../../../CHANGELOG.md) |
| A4 | **Other:** open-source software that the buyer deploys and operates (service, web client, PostgreSQL). The maintainer does not offer it as software as a service | Met | [INSTALL.md](../../../../../INSTALL.md), [operations/deployment.md](../../../../../operations/deployment.md) |
| A5 | Joel Parker Henderson, sole maintainer | Met | [MAINTAINERS.md](../../../../../MAINTAINERS.md) |
| A6 | joel@joelparkerhenderson.com | Met | [SECURITY.md](../../../../../SECURITY.md) |
| A7 | Not published | Manufacturer to complete | |
| A8 | Not published | Manufacturer to complete | |
| A9 | Not published | Manufacturer to complete | |
| A10 | None known. No company or charity number is recorded | Manufacturer to complete | |
| A11 | Not applicable: the maintainer provides no regulated care activity. **The manufacturer must confirm** | Not applicable | |
| A12 | Not applicable | Not applicable | |

## B. Value proposition (non-assessed)

| Code | Answer | Assessment | Evidence or gap |
| --- | --- | --- | --- |
| B1 | **Other:** portfolio and project management for an organization's own programmes of work. It is none of patient care, diagnostics or clinical support. (It is not workforce management either, although it plans people's time against tasks) | Met | [spec/01-purpose-and-vision.md](../../../../../spec/01-purpose-and-vision.md) |
| B2 | A registry of plans with explainable duplicate detection, plus tasks, critical-path scheduling and what-if analysis, schedule and cost risk by simulation, cost and earned value, resource levelling and capacity, risks and an issues register, milestones, key measures and objectives, controlled documents and approvals, and a CEO dashboard. Available in several languages, including Welsh (a draft awaiting a translator's review) | Met | [README.md](../../../../../README.md), [spec/index.md](../../../../../spec/index.md) |
| B3 | Intended users: project and portfolio managers, and the individual workers whose tasks they plan. **The benefits are not validated:** there has been no evaluation, trial or user study | Not met | No evaluation exists ([RFC.md](../../../../../RFC.md) asks for feedback) |
| B4 | A data-flow diagram and its table are provided. **A user-journey map is not provided**; the product has a role-by-role first-hour walkthrough in the interface, which is guidance, not a pathway map | Partly met | [governance/data-flow.md](../../../../../governance/data-flow.md) |

## C1. Clinical safety (assessed)

| Code | Answer | Assessment | Evidence or gap |
| --- | --- | --- | --- |
| C1.1.1 | **No.** No component has a medical purpose: it holds project and portfolio records, takes no clinical input and produces no clinical output. This is the manufacturer's statement, not a regulator's view. An integrator who gives it a medical purpose changes the answer | Met (as a statement) | [AI_STATEMENT.md](../../../../../AI_STATEMENT.md) section 2; [governance/dtac-checklist.md](../../../../../governance/dtac-checklist.md) |
| C1.1.2 | **No** | Met (as a statement) | As C1.1.1 |
| C1.2 | **No** | Met (as a statement) | |
| C1.2.1 | PPM schedules and reports on an organization's projects. It carries no patient information and does not influence, support or manage direct care. **The caveat a commissioner may press:** it plans people's capacity against tasks (resource levelling, a capacity view, a handover tool). Those outputs are advisory, are computed from planned work and not from a live feed, and a person decides. **An organization that uses them to staff a clinical service in near real time should treat that use as in scope and ask for a hazard assessment, which does not exist.** | Partly met | [spec/01-purpose-and-vision.md](../../../../../spec/01-purpose-and-vision.md) |
| C1.2.2 to C1.2.5 | Not answered, because C1.2 is No. **For completeness: none of this exists.** There is no DCB0129 clinical risk management system, no Clinical Safety Case Report, no hazard log and no Clinical Safety Officer. **If a commissioner decides DCB0129 applies, C1 is Not met** | Not applicable (see C1.2.1) | [governance/assurance-checklist.md](../../../../../governance/assurance-checklist.md) item 3 |

## C2. Data protection (assessed)

| Code | Answer | Assessment | Evidence or gap |
| --- | --- | --- | --- |
| C2.1 | **No access to patient data or national NHS System.** The maintainer has no access to any deployment: no hosting, no remote support, no telemetry. The software's only outbound requests are to addresses the deployer configures: token signing keys, an optional virus scanner, webhooks, chat and e-mail notification destinations that users enter, and an optional worker-skills service | Met | [governance/data-flow.md](../../../../../governance/data-flow.md) |
| C2.2 | **Yes, the product processes personal data**: references to people (task owners, assignees, approvers, authors), free text that may name people, an audit trail of who did what, time records and notification destinations. The manufacturer does not operate or host it and has no means of access, so the deployer is the controller. (The form lets a manufacturer in that position answer No; this answers Yes so that nothing is understated) | Met | [governance/record-of-processing.md](../../../../../governance/record-of-processing.md), [governance/personal-data-inventory.csv](../../../../../governance/personal-data-inventory.csv) (every column that can hold personal data, kept true to the schema by a test) |
| C2.2.1 | **Not provided.** The maintainer is not a controller or processor of any deployment. Whether the maintainer pays an ICO fee is not recorded | Manufacturer to complete | |
| C2.2.2 | **Provided as a template**, pre-filled for this service; the deployer completes, rates and signs it. Coverage of the required content: see below | Partly met | [governance/dpia.md](../../../../../governance/dpia.md) |
| C2.2.3 | **Not provided.** A deployer writes their own from the record of processing | Not met | [governance/README.md](../../../../../governance/README.md) |
| C2.2.4 | Not applicable as a user-data agreement: the software is licensed under open-source licences and the manufacturer processes no user data. It is **not** a substitute for the buyer's own terms with their staff | Not applicable | [LICENSE.md](../../../../../LICENSE.md) |
| C2.2.5 | **The deployer's choice.** The software stores data only in the PostgreSQL database and the file directory the deployer connects it to. The web client loads nothing from a third party at run time (scripts, styles, fonts and icons are served from the same origin, enforced by a strict Content Security Policy that a test checks on representative pages) | Manufacturer to complete | [operations/deployment.md](../../../../../operations/deployment.md); `tests/e2e/security-headers.spec.ts` |
| C2.2.6 | Not answerable by the manufacturer: depends on where the deployer hosts, and on which optional destinations users enter (a chat service they choose is a transfer the deployer must assess). The software makes no transfer of its own | Manufacturer to complete | [governance/data-flow.md](../../../../../governance/data-flow.md) |

### How the DPIA template covers what C2.2.2 requires

| Required content | Covered? | Where |
| --- | --- | --- |
| Summary of the product and how it processes data | Yes | dpia.md sections on the service and the processing |
| List of data fields required | Yes: every column that can hold personal data, with its class, purpose and retention class | [personal-data-inventory.csv](../../../../../governance/personal-data-inventory.csv) |
| How data flows into, within and out of the product | Yes | [data-flow.md](../../../../../governance/data-flow.md) |
| End-user security controls: on/off-boarding and access limits | Yes: sign-in enforced by default, attribute-based access, an onboarding and offboarding tool with an access ledger, an access-review report | [spec/07-non-functional-requirements.md](../../../../../spec/07-non-functional-requirements.md) NFR-12, NFR-21 |
| Measures for data in transit and at rest | Partly: TLS ends at the deployer's proxy and HSTS is sent over TLS; backups can be encrypted by the script; **encryption of the database and disks at rest is the deployer's** | [operations/deployment.md](../../../../../operations/deployment.md), [operations/backup-and-restore.md](../../../../../operations/backup-and-restore.md) |
| Countries where data is stored or flows | The deployer's | |
| Whether the manufacturer's staff can access personal data | Yes: they cannot | See C2.1 |
| Who is controller for each element | The deployer for all of it | [record-of-processing.md](../../../../../governance/record-of-processing.md) |
| Retention and disposal | Partly: a schedule per class; a purge task covers read notifications, delivery logs and trend snapshots; **plans, tasks, time records and documents are not purged** (the buyer decides) | [retention-schedule.md](../../../../../governance/retention-schedule.md) |
| Processors and sub-processors, with binding agreements | The deployer's | |
| Confidentiality, availability and integrity risks and mitigations | Yes | [dpia.md](../../../../../governance/dpia.md) risk table |
| How the product supports data subject rights | Partly: an administrator-only export of everything held about a person reference, and pseudonymisation of it (not deletion) everywhere, audit trail included. **Names typed into free text, uploaded files and backups are not found automatically** | [dpia.md](../../../../../governance/dpia.md) risk 7; `GET /api/admin/subjects/{ref}` |

## C3. Technical security (assessed)

| Code | Answer | Assessment | Evidence or gap |
| --- | --- | --- | --- |
| C3.1 | **No Cyber Essentials certificate.** The maintainer is an individual and has none | **Not met** | |
| C3.2 | **No**, the Cyber Security Charter for Suppliers to the NHS is not signed | Not met | So C3.3 to C3.6 are answered below |
| C3.3 | **Not provided. No independent penetration test has been done.** A passive OWASP ZAP baseline scan is written in CI and **its job currently fails on GitHub**, so there is no clean scan result either. The project's own documents are not a substitute | **Not met** | [governance/assurance-checklist.md](../../../../../governance/assurance-checklist.md) item 1; `.github/workflows/ci.yml` job `zap` |
| C3.4 | **Unable to confirm.** Practices that map to the Code's four themes exist, and the gaps are named. *Secure design and development:* login enforced by default, attribute-based access, a security-headers and CSP test, `#![forbid(unsafe_code)]`, clippy with warnings denied. *Build environment:* continuous integration on GitHub; actions are pinned by tag, not commit; no secret scanning is configured. *Deployment and maintenance:* the service refuses to start without verification keys and refuses `production` with login off; a tested restore drill; encrypted backups; Dependabot; `cargo deny`; a software bill of materials and build provenance on release. *Communication with customers:* a security policy with a private reporting route, a changelog, a governance and a readiness document. **Gaps:** five CI jobs currently fail (restore drill, docs check, `cargo deny` for the service, ZAP, PostgreSQL integration tests), no release has been cut, and the practice has **not been independently assessed against the Code** | Partly met | [SECURITY.md](../../../../../SECURITY.md), [.github/workflows/ci.yml](../../../../../.github/workflows/ci.yml), [.github/workflows/release.yml](../../../../../.github/workflows/release.yml), [governance/production-readiness.md](../../../../../governance/production-readiness.md) |
| C3.5 | **Yes, by identity federation.** PPM implements no passwords or accounts of its own: it verifies tokens issued by the deployer's identity provider (Microsoft Entra ID, directly or through Keycloak, or a PASETO issuer), so multi-factor authentication is enforced there for every account type. The plan is documented | Partly met | [operations/sign-in-with-entra.md](../../../../../operations/sign-in-with-entra.md). MFA itself is the identity provider's and was **not exercised** by this repository's tests; sign-in is tested against a mock provider and a real Keycloak |
| C3.5.1 | Not applicable: the supplier has no access to any deployment | Not applicable | See C2.1 |
| C3.6 | **Yes.** Every mutation writes an audit entry naming the actor; each entry carries digests and, when a key is set, a keyed MAC, and the database refuses to change or delete an audit row outside the retention task. **Limits:** it is not a hash chain, so a deleted row would not show in verification, and a database administrator can drop the protection. Reads are audited only for sensitive operations (data-subject export, access review), not every read | Partly met | [governance/dpia.md](../../../../../governance/dpia.md) risk 9; `tests/requests/audit_protection.rs`; `GET /api/compliance/audit/verify` |

## C4. Interoperability (assessed)

| Code | Answer | Assessment | Evidence or gap |
| --- | --- | --- | --- |
| C4.1 | **Yes:** a JSON REST API, signed webhooks and calendar feeds, for the buyer's own systems. None is aimed at health or social care systems in particular | Met | `/api-docs/openapi.json` |
| C4.1.1 | HTTP and JSON; OpenAPI; bearer tokens (PASETO or OIDC JWT); ISO 8601 dates; ISO 4217 currencies; HMAC-signed webhooks; iCalendar feeds. Import of Jira and Asana CSV and Microsoft Project (MSPDI XML). **No health-care standard is used** (no HL7 FHIR) because the product holds no clinical records; the specification records this as deliberate | Met | [spec/02-scope.md](../../../../../spec/02-scope.md); `src/openapi.rs`, `src/openapi_more.rs` |
| C4.1.2 | **Cannot confirm** conformance with GDS Open API best practice; the API is openly documented and the code is public | Not met (cannot confirm) | The OpenAPI document is assembled by hand and its coverage is checked by tests, not generated, so it can lag the routes |
| C4.1.3 | The basis on which third parties can build integrations: the source and the OpenAPI document are public under open-source licences; there is no commercial gate or partner agreement | Met | [LICENSE.md](../../../../../LICENSE.md) |
| C4.2 | **No.** PPM does not share or receive data with national or local care systems where patient identity is relevant: it holds no patient identity | Met | C2.1 |
| C4.2.1 to C4.2.6 | Not asked, because C4.2 is No. For information: **PPM does not use the NHS number, does not integrate with the Personal Demographics Service and does not use NHS login.** People sign in through the deployer's identity provider | Not applicable | [operations/sign-in-with-entra.md](../../../../../operations/sign-in-with-entra.md) |

## D1. Usability and accessibility (scored, not pass/fail)

| Code | Answer | Assessment | Evidence or gap |
| --- | --- | --- | --- |
| D1.1 | **Not provided** as a user journey fitted into a care pathway. The specification describes users and screens, and the interface has a first-hour walkthrough per role | Not met | [spec/03-stakeholders-and-users.md](../../../../../spec/03-stakeholders-and-users.md) |
| D1.2 | **No.** No testing with intended users has been done | Not met | [RFC.md](../../../../../RFC.md) asks for it |
| D1.3 | **Cannot confirm** that the Accessible Information Standard has been read and considered. It concerns communications with patients, which PPM has none of | Not met (cannot confirm) | |
| D1.4 | **Yes**, a web application | Met | `project-portfolio-management-ui-with-svelte` |
| D1.4.1 | **No**, not on the evidence the form asks for. What exists: an **automated** audit (axe-core, WCAG 2.2 AA) of **every page route** on every change, which finds no serious or critical violation; target sizes, status shown in words and shapes and not colour alone, a simple mode, a dyslexia-friendly typeface, reduced motion, keyboard operation. What does not: **no manual review, no assistive-technology testing and no testing with disabled users**, which automation cannot replace. There is a plan (an assistive-technology audit) but **no timeline** | Partly met (stated as No) | [governance/accessibility-statement.md](../../../../../governance/accessibility-statement.md); `tests/e2e/a11y.spec.ts`; [governance/assurance-checklist.md](../../../../../governance/assurance-checklist.md) item 2 |
| D1.4.2 | **No timescale has been set** | Not met | |
| D1.4.3 | The statement is [governance/accessibility-statement.md](../../../../../governance/accessibility-statement.md). It says plainly what is and is not tested. It is a template for a deployer's own; **it is not published at a public address** | Partly met | |
| D1.5 | **Not applicable and not measured:** the manufacturer operates no service, so there is no availability figure. A deployer measures their own | Not applicable | |

## Supporting documents

| Code | Document | Status |
| --- | --- | --- |
| A12 | CQC report | Not applicable |
| B4 | User journeys and data flows | Data flow: [data-flow.md](../../../../../governance/data-flow.md). **User journeys: not provided** |
| C1.1.1 | Pre-acquisition questionnaire | Not applicable (C1.1.1 is No) |
| C1.2.3, C1.2.4 | Clinical risk management system, Clinical Safety Case Report, Hazard Log | **Do not exist** |
| C2.2.1 | ICO registration | **Not provided** |
| C2.2.2 | DPIA | Template: [dpia.md](../../../../../governance/dpia.md) |
| C2.2.3 | Transparency information | **Not provided** |
| C2.2.4 | Terms on use of user data | Not applicable: [LICENSE.md](../../../../../LICENSE.md) |
| C3.1 | Cyber Essentials certificate | **Not provided** |
| C3.3 | External penetration test summary | **Not provided** |
| D1.1 | User journeys | **Not provided** |

## What it would take to pass

The assessed sections (C1 to C4) must all be met. As submitted, **C3 fails on three points the maintainer cannot
close in code**, and C2 and C1 each depend on a decision for someone else.

1. **C3.1 Cyber Essentials.** A certificate for the manufacturer (or the hosting supplier, if the buyer buys it as
   a service). Needs an organization to certify.
2. **C3.3 Independent penetration test** of a deployed instance covering the OWASP Top 10, with no finding at or
   above CVSS 7.0 ([assurance checklist](../../../../../governance/assurance-checklist.md) item 1). Alternatively
   **C3.2:** sign the Cyber Security Charter for Suppliers to the NHS, which removes C3.3 to C3.6.
3. **Make the project's own CI green.** Five jobs fail on GitHub today; C3.4 cannot honestly be confirmed while the
   restore drill, the ZAP scan and the integration tests fail.
4. **C2.2.1 and C2.2.3.** ICO registration evidence if the manufacturer is a controller or processor of any
   deployment (not if only the deployer is), and a transparency notice the deployer completes from the record of
   processing.
5. **C1.** Settle with the commissioner whether any use of the capacity or levelling views staffs a clinical
   service in near real time. If it does, DCB0129 applies and the clinical risk management system, safety case,
   hazard log and Clinical Safety Officer must exist; none does.
6. **D1.** An independent assistive-technology audit and user testing, for the score.
7. **A.** A legal entity, address and phone number, or a statement that the buyer is contracting with an
   individual.

Separately, the product's own [readiness assessment](../../../../../governance/production-readiness.md) records that
it is not ready for unsupervised production use in a large medical or governmental organization, whatever DTAC
says.

## Limits of this assessment

- Written from the repository only. It did not see a deployment, a hosting arrangement, a certificate or a
  contract.
- It rests on the maintainer's own tests and documents. No external party has reviewed either.
- It follows the checklist as converted in [checklist.md](checklist.md). The original form, and its guidance on
  evidence, were not available to the assessor; the pass criteria are as that file states them.
- Where the checklist links to a published standard (DCB0129, the Software Security Code of Practice, DAPB3051, the
  Accessible Information Standard), the standard itself was not read in full for this assessment.
- The statement about CI is from the GitHub run list on 2026-10-10; it will change.
