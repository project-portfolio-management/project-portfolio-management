# Project Portfolio Management and Redmine

*Two configurable issue models, and where they differ.*

**Basis of this comparison.** The Redmine column of the table below was checked on 5 November 2026 against Redmine's own documentation: [roles and permissions](https://www.redmine.org/projects/redmine/wiki/RedmineRoles), [issue tracking setup](https://www.redmine.org/projects/redmine/wiki/RedmineIssueTrackingSetup) and [project settings](https://www.redmine.org/projects/redmine/wiki/RedmineProjectSettings). Redmine was not installed or run, and the rest of this page is a general description. Products are not scored. Statements about this project are taken from its repository.

## About Redmine

A long-established, configurable, open-source project tracker in which the organization defines issue types, statuses, custom fields and roles.

## When Redmine may be the better fit

- Roles need to govern more than issue actions, such as notes, watchers, saved queries and which issues a member can see
- Project members, not only administrators, need to manage their own project's members and trackers
- Groups need to hold roles, or field rules need to cover custom fields
- A long production history and a large plugin ecosystem are required

## When this project may be the better fit

- Issue types, statuses, roles and field rules are needed alongside a portfolio registry
- Portfolio views, critical path, earned value and governance are required
- Explainable duplicate matching is required

## Issue configuration compared

| Concept | Redmine | This project |
| --- | --- | --- |
| Issue types | Trackers, defined by an administrator. Each project selects the ones it uses, and project members with the permission can make that selection | Issue types, defined by an administrator for the whole deployment. An administrator can limit a plan to some of them. A type is retired, never deleted |
| Statuses | A shared list of statuses. More than one can be marked closed, and each can set a percent done | The states of a workflow. Each state declares what it means (not started, active, waiting or done), and totals and overdue figures use that meaning |
| Roles | A named set of permissions across issues, notes, watchers, queries and more. A role can also narrow which issues its holder sees | A named set of four issue permissions: add, edit, move and remove. Permissions can only narrow what a person's sign-in allows, and what they can see is decided by the plan's privacy |
| Members | A user or a group holds one or more roles on a project, and the permissions combine | A person holds one or more roles on a plan, and the permissions combine. A plan is unaffected until it has its first member. Only an administrator changes membership, and groups cannot hold a role |
| Workflow | Allowed status changes for each combination of role and tracker | Allowed moves for a workflow, each open to anyone or to named roles. A workflow can apply to one issue type, to one plan, or to both |
| Field rules | Per status, a field can be read-only or required, for each role and tracker. This includes custom fields | Per status, one of seven standard fields can be read-only or required, for everyone or named roles. Custom fields are not covered. An administrator is not bound by read-only |
| Translated names | Not checked for this comparison | An organization can give each issue type, role and status a name per language |

## Migration

There is **no Redmine importer**. This project adopted ideas from Redmine but no code: custom fields, per-plan modules, watchers and approvals.

## Project status

This project is at an early stage, with one maintainer and no third-party audit. What production use would take, item by item with an estimate for each, is on the [path to production](../../path-to-production/index.md) page. It has no mobile app and no real-time co-editing. See [known limitations](../../topics/comparisons/index.md#known-limitations).

Corrections can be submitted as an [issue](https://github.com/project-portfolio-management/project-portfolio-management/issues) with a source.

Redmine is the name of its owner's product. This project is not affiliated with or endorsed by its owner.
