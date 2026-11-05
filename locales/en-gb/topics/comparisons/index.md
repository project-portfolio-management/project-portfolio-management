# Comparisons

*A summary of how the software relates to other products.*

The products below serve different purposes and are not scored. These notes are based on each project's public material. **No other product was installed or run**, and each may have changed since. Current product documentation should be checked before a decision is made.

| Project | What it is | Where this project differs |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Project management for people who are not project managers | This project also includes the scheduling, cost and governance functions used by a portfolio office |
| [Redmine](https://www.redmine.org/) | A configurable general-purpose tracker, with organisation-defined issue types, statuses and roles | Issue types, statuses and workflows are organisation-defined. Roles carry issue permissions, held plan by plan, within what a person's sign-in allows. See the [Redmine comparison](../../../en-001/compare/redmine/index.md) |
| Huly Platform (through its fork, Platform Collective) | A business application framework on many services | Huly is a multi-service platform. This project is one service over PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | A desktop alternative to Microsoft Project, for one manager and one project | This project is a hosted, multi-user service with access control |

## Product comparisons

Individual comparison pages: [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Interoperability

- **Jira and Asana:** CSV import, with a preview and a stated reason for everything left out.
- **Microsoft Project:** import and export of MSPDI XML, covering tasks, outline, dates, durations, constraints, dependencies and the calendar. Assignees, resources and baselines are excluded, and the file records this.

## Design differences

- **Stated reasons.** A missing figure is shown with a reason, for example "no forecast, because…".
- **Explainable duplicate detection.** Matching works across all plan types and shows its evidence.
- **Security defaults.** Login is on by default, and security material is included in the repository.
- **Explicit durations.** Each duration is stated in calendar days or business days.
- **Recorded limits.** The backlog records the limits of each item.

## Known limitations

- No mobile app or desktop application.
- No real-time co-editing.
- One maintainer, a limited production history and no third-party audit.
- The published load test ran on one machine. It indicates an order of magnitude and is not a capacity plan.
- The translations are drafts awaiting review.
- No screen for drafting or deciding change requests.
- No plugin or marketplace ecosystem.

Further detail is in the [comparison notes](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). Corrections can be submitted as an issue with a source.
