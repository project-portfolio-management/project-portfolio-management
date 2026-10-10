# Record of processing

The record a controller keeps under UK GDPR / EU GDPR Article 30, pre-filled with
what the **software** does. Fields marked `‹buyer›` are the controller's. If you
act as a processor for someone else, keep the processor version of the record
(categories of processing carried out for each controller) instead.

## 1. Controller

- Name and contact: ‹buyer›
- Data protection officer: ‹buyer›
- Joint controller or representative, if any: ‹buyer›

## 2. Processing activities

| # | Activity | Purpose | Categories of people | Categories of data | Lawful basis | Recipients |
|---|---|---|---|---|---|---|
| 1 | Maintaining the portfolio of programmes, projects and their tasks, risks, budgets and milestones | Deliver and govern the organization's change portfolio | Staff, contractors and stakeholders named as leads, owners, assignees, sponsors, approvers | Opaque person references; free text in descriptions, notes and comments | ‹buyer› (public task or legitimate interests are the usual candidates) | Other users of the service, as the access policy allows |
| 2 | Planning capacity: allocations, availability, leave and non-working periods, resource roles | Match people to work and avoid over-commitment | Staff and contractors | Person reference; booked percentages and date ranges; short notes on availability | ‹buyer› | Resource managers and, for their own records, the people concerned |
| 3 | Recording time and costs: timesheets, time entries, cost rates and the costs derived from them | Account for effort and cost; capitalization | Staff and contractors | Person reference; hours; cost rates (finance-restricted); notes | ‹buyer› (and legal obligation for financial records) | Finance holders and managers; never guests |
| 4 | Audit trail | Accountability, security, assurance | Everyone who changes a record | Actor reference; a snapshot of what changed; per-row integrity digests | ‹buyer› (legal obligation or legitimate interests) | Auditors; administrators |
| 5 | Notifications, announcements, comments, reviews and approvals | Collaborate and decide | Staff, contractors, stakeholders | Person references; message text; **the e-mail address or chat webhook each user chooses for their own notifications** | ‹buyer› | The recipients and watchers; the mail server; any chat service a user chooses (Slack, Mattermost, Discord, Telegram) |
| 6 | Joiner, mover and leaver cases | Control access and handover | Staff and contractors | Person reference; checklist notes | ‹buyer› | The people managers and onboarding owners concerned |
| 7 | Sign-in and access control | Authenticate people; authorize actions | Everyone who signs in | Subject and display name from the identity provider; attributes that drive the access policy | ‹buyer› | The identity provider; the service |
| 8 | Backups | Recover from loss | As above | A copy of everything | ‹buyer› | Your backup storage provider |

The full column-level list is in the [personal-data inventory](personal-data-inventory.csv):
every column that can hold personal data, each with a class and a retention class.

## 3. Special-category and criminal-offence data

None is collected by design: the service has fields for plans, tasks, money and
people **references**, not for health, beliefs or similar. Free-text fields
(notes on leave or availability, comments) could be used to write such data; the
operator guidance is that they must not be, and there is no technical block.
‹buyer: state how you tell your users, and what you do if it happens›

## 4. Transfers outside the UK or EEA

The software makes none by itself. Transfers depend on **where you host** the
service, the database, the backups, the mail server and the telemetry collector,
and on your identity provider (Microsoft Entra ID, Keycloak, or the
authentication service). **A user who sets a chat webhook as a notification
destination sends notification text to that chat service**, which may be
outside the UK or EEA. ‹buyer: list each and the safeguard›

## 5. Retention

Per the [retention schedule](retention-schedule.md), classes `AUD`, `OPS`, `PPL`,
`COM`, `DOC`, `GEN`, `CRD`, `EVT`, `DRV`. The software enforces only some of it
today (see the schedule's known gaps).

## 6. Security measures (Article 32)

Login enforcement on by default; role- and attribute-based access control with
deny by default; private plans hidden from non-members; masking of person
references on export; an audit trail with per-row integrity digests and an optional keyed MAC; security headers and a strict
content security policy; per-caller rate limiting; short-lived tokens verified
offline; no secrets in the container image; backups with a tested restore
([operations pack](../operations/README.md)); dependency, licence and advisory
checks in the build (`cargo deny`), and a software bill of materials with each
release. Encryption in transit (TLS) and at rest (disk, database, backups) is
provided by **your** environment. ‹buyer: confirm›

## 7. Processors and sub-processors

None is built into the software. Anything you use is yours to list: hosting,
database, backups, mail, telemetry, identity provider. ‹buyer›
