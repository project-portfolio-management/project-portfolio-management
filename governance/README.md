# Information governance pack

Documents a buyer's information governance lead can **adapt rather than write
from scratch**. They are pre-filled with what this service actually does, drawn
from the code and checked by tests where a test can check it, and they leave the
decisions that belong to the buyer clearly marked.

| Document | What it is |
|---|---|
| [DPIA template](dpia.md) | A data protection impact assessment, filled in for this service as far as the software can know, with the buyer's parts marked |
| [Data-flow diagram](data-flow.md) | Who and what talks to whom, where personal data is stored, and where it leaves |
| [Record of processing](record-of-processing.md) | The Article 30-style record: purposes, categories, recipients, transfers, retention, security |
| [Retention schedule](retention-schedule.md) | A retention class for every kind of record, with a starting period, and what the code enforces today |
| [Personal-data inventory](personal-data-inventory.csv) | Every column that can hold personal data, with class, purpose and retention class (kept true to the schema by a test) |
| [Accessibility statement](accessibility-statement.md) | WCAG 2.2 AA: what is tested, what is not, known gaps, how to ask |
| [Language statement](language-statement.md) | Which languages the interface is available in and how complete each is |
| [Production readiness](production-readiness.md) | An honest assessment of readiness for major medical and governmental organizations: what is strong, what is not ready |
| [Assurance checklist](assurance-checklist.md) | What an assessor will ask for, who owns each item, and what the project can supply |
| [DTAC checklist](dtac-checklist.md) | Answers to the Digital Technology Assessment Criteria, section by section, with the evidence |
| [Welsh translation review record](welsh-translation-review.md) | Where the translator's review of the Welsh text is recorded |

## How to use it

1. Read the [DPIA](dpia.md) and the [record of processing](record-of-processing.md); replace every
   `‹buyer›` marker with your own entry. Each marker is a decision that is
   yours: your organization's name, your lawful basis, your retention periods,
   your processors, your sign-off.
2. Agree the [retention schedule](retention-schedule.md)'s periods with your records manager.
3. Complete the [DTAC checklist](dtac-checklist.md): rows marked **Buyer to complete** are about your
   organization, not the software.
4. Publish the accessibility and language statements with your own contact
   details, after your own checks.

## What is checked automatically

`tests/personal_data_inventory.rs` fails when a migration adds a column that can
hold personal data that the inventory does not name, when the inventory names a
column that no longer exists, and when a retention class is missing from the
schedule. So the pack cannot silently go out of date as the schema grows.

## What this pack is not

It is not legal advice and it does not make you compliant; compliance is a
property of how you run the service. It is the software's side of the story,
stated honestly, including the gaps (see "Known gaps" in each document).
