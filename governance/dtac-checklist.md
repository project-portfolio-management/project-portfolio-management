# DTAC checklist

A checklist for the **Digital Technology Assessment Criteria (DTAC)**, the
baseline of clinical safety, data protection, technical security,
interoperability and usability criteria that digital health and care
technology is assessed against in England and widely used elsewhere in the UK
as a common checklist. Use it as a worked starting point: the answers below are
the software's; the rows about your organization are yours.

> **Check the numbering against the current published form.** The criteria are
> revised from time to time; this checklist follows its seven sections (A to G)
> by topic and does not reproduce the form's wording. Answer the form from the
> evidence here.

**Status key.** **Met**: the software does this and the evidence is cited.
**Partly**: it does some of it; the gap is stated. **Buyer to complete**: the
answer is about the organization or its deployment. **Supplier to complete**:
about the company supplying the software. **Not applicable**: with the reason.

*This service is a portfolio and programme management tool. It holds no patient
data and performs no clinical function.*

## A. Company information

| Topic | Status | Notes |
|---|---|---|
| Legal entity, registration, address, contacts | Supplier to complete | The repository's author is named in the manifests; a deploying organization needs the supplier's registered details |
| Data protection registration of the supplier | Supplier to complete | |
| Insurance, financial standing | Supplier to complete | |
| Named senior contacts | Supplier to complete | |

## B. Value proposition

| Topic | Status | Notes |
|---|---|---|
| What the product is for, who uses it | Met | [`README.md`](../README.md), [`spec/01-purpose-and-vision.md`](../spec/01-purpose-and-vision.md) |
| Evidence of benefit | Buyer to complete | State the outcomes you expect; the service can report benefits realized against plan |
| Costs | Buyer to complete | The software is offered under several licences ([`LICENSE.md`](../LICENSE.md)); support and hosting are separate |

## C. Clinical safety

| Topic | Status | Notes |
|---|---|---|
| Does the product have a clinical function, or hold patient data? | Not applicable | No. Plans are operational data; the model has no patient fields ([`spec/12-compliance.md`](../spec/12-compliance.md) §12.1) |
| Clinical risk management (the clinical risk management standards for manufacturers and deployers) | Buyer to complete | A **clinical safety officer** at the deploying organization should confirm in writing that none applies, and record the reasoning. If the service is used to record go/no-go decisions on clinical cutovers, treat that use as in scope for your own safety review even though the software is not a medical device |
| Medical device registration | Not applicable | Not a medical device: no diagnosis, treatment or monitoring of an individual |

## D. Data protection

| Topic | Status | Evidence |
|---|---|---|
| Lawful basis, roles of controller and processor | Buyer to complete | [Record of processing](record-of-processing.md) |
| Data protection impact assessment | Partly | [DPIA template](dpia.md), pre-filled; the buyer completes, rates and signs |
| Record of processing activities | Partly | [Record of processing](record-of-processing.md) |
| What personal data, and why | Met | [Personal-data inventory](personal-data-inventory.csv), kept true to the schema by a test |
| Data minimization | Met | People are opaque references; no names, birth dates or special-category fields in the schema |
| Retention and deletion | Partly | [Retention schedule](retention-schedule.md); a purge task covers read notifications, delivery logs and trend snapshots; **plans, tasks, time, documents and the audit trail are not purged** (the buyer decides; stated gap) |
| Data-subject rights | Partly | Access and portability: an administrator-only export of everything held about one person reference (`GET /api/admin/subjects/{ref}`), plus the per-plan export; rectification: edit in place with history; restriction: soft delete; erasure: the deployment's choice of **pseudonymisation** (the default) or **deletion** of the person's own rows with an unlinkable reference elsewhere (`POST …/erase`, preview first; WW-1.8); audit rows are never deleted, because they are the evidence they exist for. Names typed into free text, uploaded files and backups are not found automatically (DPIA risk 7) |
| Data location and transfers | Buyer to complete | Set by where you host; the software makes no transfers by itself ([data flow](data-flow.md)) |
| Sub-processors | Buyer to complete | None built in |
| Breach notification process | Buyer to complete | [Runbook](../operations/runbook.md) §"Investigate an incident" gives the technical steps |
| Data protection officer, privacy notice, regulator registration | Buyer to complete | |
| Does the product use personal data to train models, or for marketing? | Met | No. There is no model in the service, no analytics and no marketing use |

## E. Technical security

| Topic | Status | Evidence |
|---|---|---|
| Authentication | Met | Login enforced on every API route by default; PASETO v4 tokens or a Keycloak/Entra OIDC provider; production refuses to start without it ([`spec/07`](../spec/07-non-functional-requirements.md) NFR-12) |
| Access control and least privilege | Met | Attribute-based, deny-by-default policy; separate finance and people areas; private plans; guest confinement ([`tests/private_aggregates.rs`](../project-portfolio-management-service-with-rust/tests/private_aggregates.rs), `tests/enforcement.rs`) |
| Multi-factor authentication | Buyer to complete | Provided by your identity provider |
| Encryption in transit | Buyer to complete | TLS terminates at your proxy; HSTS is sent over TLS ([deployment guide](../operations/deployment.md)) |
| Encryption at rest | Buyer to complete | Disk, database and backup encryption are your environment's |
| Secure configuration | Met | Secure by default; a start-up gate refuses unsafe postures; security headers, strict CSP, CORS allow-list, rate limiting (NFR-12 to NFR-14) |
| Audit logging | Met | Every change recorded with the actor; per-row integrity digests and an optional keyed MAC; deletion of rows is not detected (stated in the DPIA) |
| Secure development | Met | Linting at deny-warnings, unit, database and browser tests, accessibility audits on every change; CI at the repository root (`.github/workflows/ci.yml`) |
| Vulnerability and dependency management | Partly | `cargo deny` (advisories, licences, bans) in CI, dependency updates automated; a software bill of materials with each release; a vulnerability-disclosure policy ([`SECURITY.md`](../SECURITY.md)) with private reporting and response-time targets (not guarantees) |
| Penetration testing | Buyer to complete | **None has been done by the project.** Commission one for your deployment. A passive ZAP baseline scan runs in CI |
| Certifications (Cyber Essentials Plus, ISO/IEC 27001) | Supplier to complete | The software supports the controls; certification is of an organization |
| Backups and recovery | Met | Scripts, a tested restore drill in CI, stated RPO and RTO ([operations pack](../operations/README.md)) |
| Incident response and business continuity | Buyer to complete | [Runbook](../operations/runbook.md) is the technical part |
| Hosting and its assurance | Buyer to complete | You choose and assure the hosting |

## F. Interoperability

| Topic | Status | Evidence |
|---|---|---|
| Open, documented interfaces | Met | A REST API with a published OpenAPI 3 document (`/api-docs/openapi.json`) and Swagger UI; every list pages the same way |
| Standard formats | Met | JSON, CSV, iCalendar and Atom feeds; Microsoft Project XML (MSPDI) import; Jira and Asana imports |
| Health data standards (FHIR, HL7) | Not applicable | No clinical data; a FHIR surface is deliberately not provided ([`spec/02-scope.md`](../spec/02-scope.md)) |
| National identity and NHS services (NHS login, the national spine) | Not applicable | Not a patient-facing or clinical service |
| Single sign-on standards | Met | OpenID Connect, directly or through Keycloak, including Microsoft Entra ID ([sign-in guide](../operations/sign-in-with-entra.md)) |

## G. Usability and accessibility

| Topic | Status | Evidence |
|---|---|---|
| Accessibility standard | Partly | [Accessibility statement](accessibility-statement.md): WCAG 2.2 AA checked by an automated audit of every page on every change; **manual and assistive-technology testing is the buyer's to record** |
| Languages | Partly | [Language statement](language-statement.md) |
| User research with the intended users | Buyer to complete | |
| Support for users | Buyer to complete | |
| Plain-language and cognitive accessibility | Partly | Status is words and shapes, not colour; forms say what is wrong in words; a simple mode and a dyslexia-friendly typeface exist; testing with users is the buyer's |

## How to complete your own assessment from this

1. Fill every **Buyer to complete** row (these are about your organization).
2. Ask the supplier for every **Supplier to complete** row.
3. Attach the evidence links; re-run the checks (`cargo test`, the browser audits,
   `scripts/restore-test.sh`) against your deployment and keep the output.
4. Commission the independent items (penetration test, manual accessibility
   review, clinical safety officer confirmation) and record their dates here.
