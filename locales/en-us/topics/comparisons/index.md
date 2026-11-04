# Comparisons

*How it relates to other tools, for anyone deciding whether it fits.*

Different products solve different problems, so nothing here is scored. These notes come from reading each project's public material. **No competitor was installed or run**, and all of them have moved on since, so please check their current documentation.

| Project | What it is | Where this project differs |
| --- | --- | --- |
| [Leantime](https://github.com/leantime/leantime) | Project management for people who are not project managers | This project also has the full scheduling, cost and governance set that a portfolio office needs |
| [Redmine](https://www.redmine.org/) | A configurable generalist, with your own issue types, statuses and roles | Here the vocabulary is mostly fixed in code: custom fields exist, but there is no per-type workflow matrix |
| Huly Platform (through its fork, Platform Collective) | A business application framework on many services | That is a platform; this is one service over PostgreSQL |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) | A desktop alternative to Microsoft Project, for one manager and one project | This is a hosted, multi-user service with access control |

## Product by product

Read a page on each: [Trello](../../../en-001/compare/trello/index.md), [Jira](../../../en-001/compare/jira/index.md), [Asana](../../../en-001/compare/asana/index.md), [Microsoft Project](../../../en-001/compare/microsoft-project/index.md), [Planview](../../../en-001/compare/planview/index.md), [Smartsheet](../../../en-001/compare/smartsheet/index.md), [Leantime](../../../en-001/compare/leantime/index.md), [Redmine](../../../en-001/compare/redmine/index.md), [ProjectLibre](../../../en-001/compare/projectlibre/index.md).

## Works alongside the tools you have

- **Jira and Asana:** CSV import, with a preview and a stated reason for everything left out.
- **Microsoft Project:** import and export of MSPDI XML, covering tasks, outline, dates, durations, constraints, dependencies and the calendar. Assignees, resources and baselines are not included, and the file says so.

## Where it is deliberately different

- **It says why.** A missing figure is "no forecast, because…", never a guess presented as a number.
- **Duplicate detection is explainable,** and it works across every kind of plan.
- **Secure by default.** Security is part of the repository, not an extra.
- **Plain durations.** Every duration is in calendar days or business days.
- **Honest status.** The backlog says what each item does not do.

## Where it is behind

- No mobile app and no desktop application.
- No real-time co-editing.
- One maintainer and a young codebase, with no long production history and no third-party audit.
- A published load test, but on one machine, so it is an order of magnitude and not a capacity plan.
- The translations are drafts awaiting review.
- No screen yet for drafting or deciding change requests.
- No plugin or marketplace ecosystem like those of mature products.

For a fuller account, read [the full comparison notes](https://github.com/project-portfolio-management/project-portfolio-management/blob/main/COMPARISONS.md). If you find a mistake, please open an issue with a source.
