# Project Portfolio Management

*A registry of funded work, with duplicate detection and a project management workspace.*

Project Portfolio Management is open-source software for recording the plans an organisation funds and managing the work within them. It covers tasks, schedules, cost, resources, risks and governance. It compares plans to identify possible duplicates and shows the evidence for each match. Where a figure cannot be calculated, the software states the reason.

## Main benefits

- **Single portfolio record.** Portfolios, programmes, projects and products use one record type, and any plan can contain another. Reporting draws on one list across all departments.
- **Duplicate detection.** Matching compares plans recorded under different names and shows the evidence for each result, so that reviewers can act before both are funded.
- **Traceable figures.** A missing figure is shown with the reason it is missing. A plan without an assessment is shown as "not assessed".
- **Audit trail.** Every change is audited. The person who raises a change request cannot approve it.
- **Access control.** Login is on by default, and the service does not run in production with login turned off.
- **Deployment and licensing.** The software is self-hosted and offered under five open-source licences. Work can be imported from Jira, Asana and Microsoft Project.

[View all benefits](topics/benefits/index.md)

## Functional areas

- **Plans and matching.** Recursive registry, explainable duplicate detection and merging.
- **Delivery.** Kanban boards, Gantt charts, calendars, milestones, critical path and critical chain.
- **Finance.** Budgets, funding ledger, earned value, and cost and schedule risk.
- **Resources.** Skill pools, capacity heatmap and key people.
- **Strategy.** Work intake, ideas, objectives, goals and key results, and scenarios.
- **Governance.** Risks, issues, gate reviews, escalation and monthly director packs.

[View all features](topics/features/index.md)

## Intended users

- **Portfolio offices.** Reporting across programmes and projects.
- **Project and delivery managers.** Scheduling, cost and risk management.
- **Team members.** A personal view of assigned work.
- **Senior leaders.** The [role briefs](for/) set out what the software provides for each role, its limits, and the decisions required.

## Current status

The software is at an early stage. It has one maintainer, no third-party audit and no support agreement. What production use would take, item by item with an estimate for each, is on the [path to production](path-to-production/index.md) page. Known limitations are listed on the [comparisons page](topics/comparisons/index.md).

## Demonstration

A local demonstration uses a fictitious transport authority and requires Git and Podman. The steps are in the [tutorial](topics/tutorial/index.md).
