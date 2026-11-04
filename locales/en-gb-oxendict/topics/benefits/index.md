# Benefits

*The main benefits for an organization, by area.*

## Portfolio visibility

Portfolio information is often held in several places. In this software, every portfolio, programme, project and product is one record type, called a **plan**. Any plan can contain any other, to any depth, so the structure can match the organization. A label such as "Project" is descriptive and places no restriction on a plan.

## Duplicate reduction

Two departments can charter the same initiative under different names. The matcher compares the attributes that identify a plan: name, goals, owner, sponsor, parent, timeframe, keywords and identifiers. It works across all plan types and shows the evidence for each match, and a person makes the decision. Two records of one plan can be merged.

## Reporting quality

- A missing figure is shown with a reason, for example "no forecast, because…".
- A plan without an assessment is shown as "not assessed".
- Delivery confidence recorded by people is shown beside the calculated status, and the two are kept separate.
- Private plans are excluded from all totals and counts for all users. Tests check this rule.
- Each duration states whether it is in calendar days or business days.

## Decision records

Every change is audited, and the audit trail cannot be altered through the service. The person who raises a change request cannot approve it. A monthly director pack is drafted once, issued by a named person and then frozen by the database. A correction is issued as a new version with a note.

## Security

Login is on by default, and the service does not start in production with login turned off. Security headers, rate limiting, a recovery plan with stated targets and an information governance pack are included in the repository.

## Control and portability

- **Licensing.** The software is open source, under a choice of MIT, Apache-2.0, BSD-3-Clause, GPL-2.0 or GPL-3.0.
- **Hosting.** The software is self-hosted: one Rust service over PostgreSQL, with a SvelteKit interface.
- **Data import.** CSV files from Jira and Asana can be imported, with a preview and a stated reason for anything excluded. Microsoft Project XML can be imported and exported.
- **Languages.** The interface is available in ten locales, including right-to-left Arabic. The translations are drafts awaiting review by speakers.

## Related pages

See also [features](../features/index.md), [comparisons](../comparisons/index.md) and the [tutorial](../tutorial/index.md).
