# Assurance checklist

What an assessor at a health body or a government department, in any country, will ask for, who
owns each item, what the project can supply, and what only the deployer can do.
Nothing here is done by the software alone. Items marked **open** have not been
done for this project.

| # | Assurance | Owner | What the project supplies | State |
|---|---|---|---|---|
| 1 | Penetration test of your deployment (web, API, authentication, file upload) | Deployer, with a certified tester | A passive scan runs in CI (`.zap/`); the security policy ([SECURITY.md](../SECURITY.md)); the threat-relevant defaults ([NFR-12 to NFR-14](../spec/07-non-functional-requirements.md)) | **open** |
| 2 | Accessibility audit with assistive technology (screen reader, magnification, voice control, keyboard-only) and users with disabilities | Deployer or supplier | An automated WCAG 2.2 AA audit of every route; the [accessibility statement](accessibility-statement.md) | **open** |
| 3 | Clinical safety assessment, if any clinical content could ever be entered | Deployer's clinical safety officer | A statement that the software is non-clinical ([DTAC checklist](dtac-checklist.md)) | **open**, only if clinical content is in scope |
| 4 | Data protection impact assessment, signed by the controller | Deployer's data protection officer | A pre-filled [DPIA](dpia.md), [record of processing](record-of-processing.md), [data-flow diagram](data-flow.md), [retention schedule](retention-schedule.md) | **open** |
| 5 | Disaster-recovery exercise on your hosting, against your recovery targets | Deployer | Backup and restore scripts, [recovery objectives](../operations/recovery-objectives.md), a CI drill | **open** for your environment |
| 6 | Hosting assurance (certified data centre, data location, transfers) | Deployer | None built in; the software makes no transfers by itself | **open** |
| 7 | Identity provider configuration: multi-factor authentication, conditional access, role mapping | Deployer | [Sign-in guide](../operations/sign-in-with-entra.md) | **open** |
| 8 | Supplier due diligence: who is the supplier, what is the support commitment | Deployer | [`MAINTAINERS.md`](../MAINTAINERS.md), [`GOVERNANCE.md`](../GOVERNANCE.md), [AI statement](../AI_STATEMENT.md), software bill of materials and build provenance with each release | **open** |
| 9 | Independent code review or security audit of the source | Deployer or funder | The source, the specification, the tests | **open** |
| 10 | Load and capacity test at your volumes | Deployer, or the project (RG-11) | `scripts/load-test.mjs` and the recorded results in [`BENCHMARKS.md`](../BENCHMARKS.md) (one machine, login off, order of magnitude only) | **open** for your volumes |
| 11 | Access recertification process (who reviews who has what, how often) | Deployer | Access records; `GET /api/access-review` lists the standing grants as a spreadsheet (what people recorded, not what the identity provider enforces) | **open** |
| 12 | Incident response and breach notification process | Deployer | The [runbook](../operations/runbook.md) gives the technical steps | **open** |
| 13 | Language-duty review (Welsh or others) by a professional translator | Deployer or supplier | [Review record](welsh-translation-review.md) and a CSV export for the translator | **open** |
| 14 | Records management: retention periods agreed, purge scheduled, legal hold procedure | Deployer's records manager | Retention schedule; the purge task; no legal-hold feature | **open** |

The [assurance mapping](assurance-mapping.md) lines these items up with ISO/IEC 27001, SOC 2 and national schemes
(WW-1.21).

Keep the evidence an assessor sees (reports, dates, who signed) with your own
records. The project cannot hold it for you.
