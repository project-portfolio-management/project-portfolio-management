## 12. Compliance

The Main X Index targets worldwide public governmental systems.
Plans are **operational / business data** rather than clinical or
patient data, so the technology-compliance posture leads, with the
data-protection posture engaged by the **people references** a plan
carries. Frameworks:
`agents/share/compliance-for-technology.md` (shared family guidance, not in this repository)
and
`agents/share/compliance-for-healthcare.md`.

### 12.1 Data classification

A plan's thin record (name, goals, code, optional `kind` label, parent
plan, timeframe, identifiers) is **business data**, not personal data.
Two angles keep the data-protection posture in force:

- **People references are personal data.** `lead_ref` and task
  `assignee_ref`s are `EntityRef`s to **people** (person / worker /
  authentication identities). Who leads or staffs a plan — and the
  audit trail of who changed what — is personal data about identifiable
  individuals. **The applicable data-protection law applies** to these
  references and to the audit log: whichever law governs the deployer and
  the people concerned (§12.2a lists the common ones by region; GDPR and the
  UK GDPR are two of them, not the default).
- **Free-text fields can leak.** `keywords`, `tags`, goal titles, and
  issue / task text MUST never carry special-category or unnecessary
  personal data; this is an operator-training point and is why export
  defaults to masked (§9.6).

There is **no patient data** anywhere in the model; the NHS / HIPAA
clinical posture does not lead, but the audit-grade trail it implies
is retained for governmental information governance.

### 12.2 Frameworks

| Framework | Application to this entity |
|---|---|
| The applicable data-protection law (§12.2a; for example the EU GDPR, the UK GDPR and DPA 2018, Brazil's LGPD, India's DPDP Act) | Applies to the people references (`lead_ref`, task assignees) and operator identities in audit data; lawful-basis and accountability documentation deployment-side; soft delete supports retention policy; bulk export of these references is a compliance event (§9.6) |
| US HIPAA | Not engaged — no PHI; HIPAA-grade audit trails (who/what/when) are still the bar for registry + sub-resource operations as a governance baseline |
| UK NHS Act 2006 s251 / Common Law Duty of Confidentiality | Not engaged — no confidential patient information by design |
| ISO/IEC 27001 | ISMS operational controls (deployment-side): access control, backups, logging |
| ISO/IEC 42001:2023 | AIMS controls if matcher weights/thresholds are ever ML-tuned (today they are hand-set constants, §7 NFR-8) |

### 12.2a Data-protection law by region (WW-1.2)

The service is built for any country (§2.0), so no one law is the default. The deployer names the law that applies to
it in its record of processing (`governance/record-of-processing.md`); the table maps the common ones to what each
drives in this service. It is a map, not legal advice, written from general knowledge on 2026-10-31 and **not checked
against each law's current text**; a deployer confirms its own obligations. Where the table says "deployment", the
software does not do it for the deployer.

| Region | Law (examples) | Trigger here | Erasure | Residency and transfer | Breach notice |
|---|---|---|---|---|---|
| European Union and EEA | GDPR (Reg. (EU) 2016/679) | people references and audit identities | right to erasure: the subject erasure path (§9.6; WW-1.8) | deployment hosts in region; transfers under Chapter V | to the supervisory authority within 72 hours (deployment) |
| United Kingdom | UK GDPR and Data Protection Act 2018 | the same | the same | the same, under the UK's own adequacy rules | to the ICO within 72 hours (deployment) |
| Switzerland | Federal Act on Data Protection (revised, 2023) | the same | the same | transfers to countries the Federal Council lists, or with safeguards | to the FDPIC as soon as possible (deployment) |
| Canada | PIPEDA; the federal Privacy Act for federal bodies; provincial laws such as Québec's Law 25 | the same | correction and deletion requests | provincial public-sector laws may require hosting in Canada (deployment) | to the regulator and the people affected where there is a real risk of significant harm (deployment) |
| United States | no general federal law; the Privacy Act of 1974 for federal agencies; state laws such as California's CCPA/CPRA; every state's breach-notification law | the same, where a state law reaches the deployer | deletion requests under state laws | no general residency rule; agency rules (for example FedRAMP) are deployment matters | under each state's law (deployment) |
| Brazil | LGPD (Law 13.709/2018) | the same | deletion and anonymization | international transfers under the LGPD's mechanisms | to the ANPD and the people affected (deployment) |
| Rest of Latin America | for example Argentina's Law 25.326, Mexico's federal personal-data laws, Chile's Law 21.719 | the same | deletion requests | each law's transfer rule (deployment) | each law's rule (deployment) |
| India | Digital Personal Data Protection Act 2023 and its rules | the same | erasure on withdrawal of consent or when the purpose ends | transfers allowed except to countries the government restricts | to the Data Protection Board and the people affected (deployment) |
| China | Personal Information Protection Law (PIPL) | the same | deletion | cross-border transfer needs a security assessment, standard contract or certification; some operators must store in China | to the authorities and the people affected (deployment) |
| Japan, South Korea, Singapore | APPI; PIPA; PDPA | the same | deletion or suspension of use | each law's transfer rule (deployment) | to the PPC; to the PIPC; to the PDPC (deployment) |
| Australia and New Zealand | Privacy Act 1988 (Australian Privacy Principles); Privacy Act 2020 | the same | correction; deletion when no longer needed | cross-border disclosure rules (APP 8; IPP 12) | under the notifiable data breach schemes (deployment) |
| Africa | for example South Africa's POPIA, Nigeria's NDPA 2023, Kenya's Data Protection Act 2019 | the same | deletion | each law's transfer rule (deployment) | to the regulator (Information Regulator, NDPC, ODPC) (deployment) |
| Middle East and Türkiye | for example Saudi Arabia's PDPL, the UAE's federal PDPL, Türkiye's KVKK | the same | deletion | transfer rules, some with localisation (deployment) | to the regulator (SDAIA, UAE Data Office, KVKK Board) (deployment) |

What the software gives every row alike: masked export by default and a per-subject export (§9.6), a subject
erasure path whose policy the deployment chooses (pseudonymise, the default, or delete the person's own rows and
replace every other reference with an unlinkable one: `POST /api/admin/subjects/{ref}/erase`, WW-1.8), an append-only audit log, retention purges
(`retention.rs`) and no transfer of its own (§12.3). Residency, transfer safeguards and breach notices are
the deployer's. The rights are named differently by each law (the right of access under the GDPR and its
neighbours, the right to know under the CCPA, confirmation and access under the LGPD, access under the PIPL and the
DPDP Act; erasure, deletion or anonymization); every one of them maps to the same two routes, the subject export and
the subject erasure, and the per-plan export (`GET /api/plans/{pid}/export`), which is named for no one law. **Kept as deliberate limits:** the HIPAA, NHS (s251) and ONC rows of §12.2 and §12.4 stay, as "not
engaged" with their reasons, because they are the regimes a health-sector buyer asks about first; they are not a sign
that the service is for one country.

### 12.3 Information-governance posture

- **People-reference minimization.** The model stores `EntityRef`s
  (opaque URNs), not duplicated personal detail; the personal data
  itself lives in the person / worker / authentication entities. This
  keeps the portfolio entity a *referrer*, not a second copy of
  personal data — re-assess masking the moment any denormalized
  personal field appears.
- **Masking on export.** Bulk export defaults to masked, with the
  people references revealed only under elevated authorization (§9.6);
  the `case ↔ person`-style sensitivity does **not** apply (plans
  are not case data), but lead / assignee references are still personal
  data and masked by default.
- **Auditability.** Full audit log + event stream of every CRUD **and
  sub-resource write** per
  `agents/share/auditability.md`
  — including who was assigned what — is the production bar; the
  durable event bus is roadmap (§15).
- **Access control.** Production deployments MUST sit behind SSO
  (central authentication entity, PASETO v4 public token verification;
  `agents/share/authentication-sessions.md`,
  supersedes the RS256-JWT model) and TLS. **ABAC** authorization over
  the token's `attrs` claim gates write vs read vs destructive actions
  (per
  `agents/share/authorization-attributes.md`;
  delivered, supersedes the earlier role-based sketch); default-off
  until the coordinated SSO activation (§15).
- **Explainability for accountability.** Per-component match
  breakdowns give auditors a replayable rationale for every duplicate
  decision; keep this property (NFR-8).

### 12.4 Extended frameworks

Four frameworks impose obligations beyond §12.2. Regime detail:
`agents/share/compliance-for-healthcare.md`
§2; repository-wide status and the reference implementation:
`spec/compliance` §8 (shared family guidance, not in this repository). Consistent with
§12.1 — plans are business data, and there is no patient data anywhere in
the model — two of the four are **not engaged**, and the table says so
rather than manufacturing relevance.

| Framework | Engagement here | What it drives |
|---|---|---|
| **HIPAA (US)** — §164.312(b) audit controls, §164.312(c) integrity | **Not engaged** — no PHI, consistent with §12.2. | Nothing HIPAA-specific. The **tamper-evident audit chain** (a SHA-256 chain over `audit_logs`) is still adopted on governmental-information-governance and ISO/IEC 27001 grounds — the same reasoning §12.2 already uses to keep an audit-grade trail without a HIPAA trigger. |
| **GDPR / EU EHDS** — Reg. (EU) 2025/327 | **GDPR engaged**, via the people references §12.1 names (`lead_ref`, task `assignee_ref`) and the operator identities in audit data. **EHDS not engaged** — no health data. | An **erasure path that survives immutable history** (redact content, keep chain linkage) — which here means erasing a person's assignment history without destroying the plan's own record; a declared **data residency** and **lawful basis**; and bulk export of people references beyond the region recorded as a **Ch. V transfer**, reinforcing §9.6's masked-by-default rule. |
| **ONC / HTI certification (US)** — 45 CFR Part 170 §170.315(g)(10) | **Not engaged.** Portfolio is explicitly out of FHIR scope (`agents/share/fhir.md` §3) — no meaningful resource models a plan, and the service mounts no `/fhir` surface. | Nothing. Do **not** add a FHIR surface, a profile claim, or SMART discovery to this entity; there is nothing to conform to and advertising conformance would be misleading. |
| **IEC 62304 / SaMD** (with ISO 14971) | Not a device, and no clinical hazard. The engagement is **supply-chain and configuration evidence** on general engineering grounds. | A **SOUP register + CycloneDX SBOM**; **machine-checked requirement→test traceability**, notably over the containment cycle check and the date arithmetic (SEC-M4's overflow panic is the archetype of what traceability catches); and **signed, reproducible builds** — all feeding the ISO/IEC 27001 configuration-management controls §12.2 claims. |

### 12.5 Honest limits

- **Two of the four do not apply, and that is the finding** — not an
  oversight to be filled in later. Recording "not engaged" with the
  reason is the useful output.
- **Read-auditing is optional here.** Without PHI, the §164.312(b)
  argument does not carry; audit reads only if a deployment's own
  governance rules require it.
- **Every extended control is unimplemented in this service today.** The
  reference implementation is the
  care-pathway service (sibling repository, not in this repository).
