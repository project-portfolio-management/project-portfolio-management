# For the Chief Legal Officer

*Chief Legal Officer: licence, data protection, accessibility, language duties and what the project does not warrant. This is not legal advice.*

## Licence

Offered under any one of MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only or GPL-3.0-only, or custom terms by arrangement. The choice and the texts are in [`LICENSE.md`](../../../LICENSE.md) and `LICENSE/`; a test keeps every manifest consistent with them.

## Data protection (UK GDPR / EU GDPR)

Pre-filled documents you adapt; every `‹buyer›` marker is a decision that is yours (controller, lawful basis, retention periods, processors, sign-off):

- [DPIA template](../../../governance/dpia.md) and [record of processing](../../../governance/record-of-processing.md)
- [Personal-data inventory](../../../governance/personal-data-inventory.csv): every column that can hold personal data
- [Retention schedule](../../../governance/retention-schedule.md): proposals, with what the code enforces today
- [Data-flow diagram](../../../governance/data-flow.md)

The software holds people as opaque references plus recorded time, availability and cost rates, which is monitoring-adjacent and so warrants a DPIA. It offers a data-subject export and pseudonymisation (administrators only, preview first). Frozen director packs and append-only confidence history are blanked of the person who drafted or assessed, never deleted, so an erasure request can be met without breaking the record. It does **not** find names typed into free text, and backups hold records the application has deleted.

## Frameworks

HIPAA and the US health-data certification regimes are **not engaged** (no patient data); the clinical-device regime is not engaged. NHS DTAC answers are in the [DTAC checklist](../../../governance/dtac-checklist.md). Nothing is certified.

## Accessibility and language

WCAG 2.2 AA is tested automatically on every route; no manual or assistive-technology test has been done ([statement](../../../governance/accessibility-statement.md)). Welsh and other translations are drafts awaiting a professional translator ([language statement](../../../governance/language-statement.md)); relevant if you have statutory language duties.

## Supplier due diligence

Single maintainer, no contractual support, no warranty claimed, AI tools used in development and disclosed in the [AI statement](../../../AI_STATEMENT.md). The product contains no AI model. Vulnerability reporting: [SECURITY.md](../../../SECURITY.md).

## Decisions that are yours

Lawful basis and retention periods; whether you need a legal-hold procedure (none is built); contract and support terms with the maintainer.

## Read next

[Assurance checklist](../../../governance/assurance-checklist.md) · [Governance pack index](../../../governance/README.md)
