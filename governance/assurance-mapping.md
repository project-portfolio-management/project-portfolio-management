# Assurance mapping

How the [assurance checklist](assurance-checklist.md) lines up with the general schemes an assessor anywhere is likely
to use (WW-1.21). The product is built for any country (`spec/02-scope.md` §2.0), so no national scheme is the
default: the UK NHS [DTAC checklist](dtac-checklist.md) and the UK folders under `docs/` are **worked examples** of
one country's schemes, kept because they were the first to be answered, not because the product is for the UK.

**Read this first.** Written on 2026-10-31 from general knowledge of the schemes. **The standards' texts were not
read for this mapping and the control references were not checked against them**; the scheme lists are examples, not
complete or current. Nothing here is certified, and nothing in this repository is evidence of conformity: the
evidence is the deployer's (the checklist's owner column). Use it to find where to start, then check the scheme.

## By checklist item

ISO/IEC 27001 references are to the themes of the 2022 edition's Annex A (organizational 5, people 6, physical 7,
technological 8); SOC 2 references are to the Trust Services Criteria categories (the common criteria CC1 to CC9,
availability A, confidentiality C, privacy P).

| # | Checklist item | ISO/IEC 27001:2022 (Annex A theme, example controls) | SOC 2 (criteria) |
|---|---|---|---|
| 1 | Penetration test | technological: security testing, technical vulnerabilities | CC4 monitoring, CC7 system operations |
| 2 | Accessibility audit | not covered by either; national accessibility law (see the [accessibility statement](accessibility-statement.md)) | not covered |
| 3 | Clinical safety, if in scope | not covered; sector schemes (for example ISO 14971, IEC 62304; the UK's DCB0129 and DCB0160) | not covered |
| 4 | Data protection impact assessment | organizational: privacy and protection of personal data | P privacy |
| 5 | Disaster-recovery exercise | technological: backup, redundancy; organizational: continuity | A availability |
| 6 | Hosting assurance | organizational: supplier relationships, cloud services | CC9 risk mitigation (vendors) |
| 7 | Identity provider configuration | technological: secure authentication; organizational: access control, identity | CC6 logical access |
| 8 | Supplier due diligence | organizational: supplier relationships | CC9 risk mitigation (vendors) |
| 9 | Independent code review | technological: secure development, security testing | CC8 change management |
| 10 | Load and capacity test | technological: capacity management | A availability |
| 11 | Access recertification | organizational: access rights | CC6 logical access |
| 12 | Incident response and breach notice | organizational: incident management | CC7 system operations |
| 13 | Language-duty review | not covered by either; national language law where it applies (see the [language statement](language-statement.md)) | not covered |
| 14 | Records management | organizational: protection of records, privacy | C confidentiality, P privacy |

## National and sector schemes (examples)

Each of these asks, in its own form, for most of the items above. Where a deployment must meet one, answer it from the
checklist and record the answers as the DTAC checklist does: a file per scheme under `governance/`, and the scheme's
own published material, if the project keeps a copy, under `docs/<country-or-body>/<body>/<scheme>/<version>/`.

| Where | Examples |
|---|---|
| International | ISO/IEC 27001 and 27701; SOC 2; CSA STAR; ISO/IEC 42001 (if a model is ever tuned, `spec/12-compliance.md`) |
| European Union | ENISA's cloud certification work (EUCS); NIS2 obligations of the deployer; Germany's BSI C5; France's SecNumCloud |
| United Kingdom | NHS DTAC (answered: [DTAC checklist](dtac-checklist.md)); Cyber Essentials; the NHS Data Security and Protection Toolkit |
| United States | FedRAMP and StateRAMP; NIST SP 800-53 and the CSF; HIPAA (not engaged, `spec/12-compliance.md`) |
| Canada | the Cyber Centre's ITSG-33 and its cloud profile |
| Australia and New Zealand | IRAP and the Essential Eight; New Zealand's NZISM |
| Asia | Japan's ISMAP; Singapore's MTCS; India's MeitY cloud empanelment; South Korea's ISMS-P |
| Latin America, Africa, Middle East | national data-protection authorities' guidance and public-sector cloud rules (see `spec/12-compliance.md` §12.2a); for example Saudi Arabia's NCA controls |

## The `docs/` convention

`docs/<country-or-body>/<body>/<scheme>/<version>/` holds a scheme's own published material when the project keeps it
for reference: `docs/united-kingdom/national-health-service/digital-technology-assessment-criteria/2.0/`,
`docs/united-kingdom/medicines-and-healthcare-products-regulatory-agency/…` and
`docs/international-organization-for-standardization/iso-14971/`. A folder for another country's or body's scheme is
added the same way; the presence of a folder says only that material was kept, not that the product meets the scheme.
