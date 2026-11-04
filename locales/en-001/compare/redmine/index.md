# Project Portfolio Management and Redmine

*Two configurable issue models, and where they differ.*

**Basis of this comparison.** Redmine was not installed or run for this comparison. The description is general and may be out of date, and Redmine's current documentation should be checked before a decision is made. Products are not scored. Statements about this project are taken from its repository.

## About Redmine

A long-established, configurable, open-source project tracker in which the organization defines issue types, statuses, custom fields and roles.

## When Redmine may be the better fit

- Roles need to carry permissions inside the tool, assigned to members project by project
- Fields need to be read-only or required by role and status
- A long production history and a large plugin ecosystem are required

## When this project may be the better fit

- Issue types, statuses and role-restricted moves are needed alongside a portfolio registry
- Portfolio views, critical path, earned value and governance are required
- Explainable duplicate matching is required

## Issue configuration compared

| Concept | Redmine | This project |
| --- | --- | --- |
| Issue types | Trackers, defined by an administrator and enabled per project | Issue types, defined by an administrator for the whole deployment. A type is retired, never deleted |
| Statuses | A shared list of issue statuses | The states of a workflow. Each state declares what it means (not started, active, waiting or done), and totals and overdue figures use that meaning |
| Roles | Roles carry permissions and are given to members per project | Role names only. A name can restrict a status move, and a role listed here grants nothing: permissions come from the identity provider |
| Workflow | Allowed status changes for each combination of role and tracker | Allowed moves for a workflow, each open to anyone or to named roles. A workflow can apply to one issue type, or to one plan through the API |
| Field rules | Fields can be read-only or required by role and status | Not available. A terminal status requires a resolution |

## Migration

There is **no Redmine importer**. This project adopted ideas from Redmine but no code: custom fields, per-plan modules, watchers and approvals.

## Project status

This project is at an early stage, with one maintainer and no third-party audit. Its own assessment is that it is ready for a controlled pilot on non-clinical, non-secret data, and not ready for unsupervised production use. It has no mobile app and no real-time co-editing. See [known limitations](../../topics/comparisons/index.md#known-limitations).

Corrections can be submitted as an [issue](https://github.com/project-portfolio-management/project-portfolio-management/issues) with a source.

Redmine is the name of its owner's product. This project is not affiliated with or endorsed by its owner.
