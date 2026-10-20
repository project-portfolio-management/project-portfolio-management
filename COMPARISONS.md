# Comparisons

How this project relates to others, for someone deciding whether it fits. Three
cautions first:

- **Evidence is limited and stated.** The notes below come from the research in
  [`plan.md`](plan.md) (dated 2026-10-02 to 2026-10-03), which says for each
  source what was read and what was not. Mostly that was a README, release
  notes, directory listings and constants files; **no competitor was installed or
  run**, and the products have moved on since.
- **Ideas only.** Where a source's licence is restrictive (copyleft, an
  attribution clause, a network-use clause), the plan took ideas and copied no
  code, text or resources.
- **No scoring.** Different products solve different problems. Treat module and
  feature names as pointers to verify against the product's current
  documentation.

## What this project is

A **registry of plans** (one recursive collection; any plan may contain any
other; `kind` is only a label) with **explainable duplicate matching**, plus a
workspace: tasks, critical-path scheduling, cost and earned value, resources,
risks and issues, and governance. It is one Rust binary over PostgreSQL with a
SvelteKit front end, **secure by default**, and available in several languages.

## Projects looked at

| Project | What it is, from the research | What this project took from it | Where it differs |
|---|---|---|---|
| [Leantime](https://github.com/leantime/leantime) (PHP, MySQL; AGPLv3) | A project-management system aimed at people who are not project managers | The individual worker's path: a personal task view, plain-language labels, one clear primary action per screen | This project also has the full scheduling, cost and governance set a PMO needs |
| [Redmine](https://www.redmine.org/) | A configurable generalist: the organization defines issue types, statuses, custom fields and roles | Configurability ideas: custom fields, per-plan modules, watchers, approvals | Here the vocabulary is mostly fixed in code; custom fields exist but there is no per-type workflow matrix |
| Platform Collective, a fork of Huly Platform (TypeScript, Svelte; EPL-2.0) | A business-application framework (CRM, HR, chat, documents and a tracker) on 30+ services and five infrastructure systems | Product ideas (documents, requests and approvals, process) | It is a platform; this is one binary over PostgreSQL. This project takes the ideas, not the shape |
| [ProjectLibre](https://sourceforge.net/p/projectlibre/) (desktop; CPAL 1.0) | An open-source desktop alternative to Microsoft Project for one manager and one project | The scheduling vocabulary (the eight MS Project constraint types, calendars, work contours, hammock activities, resource levelling) | It is a hosted multi-user service with access control, not a desktop application |

Each of these is covered at length, with what was *not* read, in the plan's
sections 1 to 31.

## Compatibility rather than competition

- **Jira and Asana:** CSV import with a preview and a stated reason for everything
  left out.
- **Microsoft Project:** import of the MSPDI XML, and export to it (tasks, outline,
  dates, durations, constraints, dependencies, calendar; not assignees, resources, baselines or
  anything beyond done/not-done progress, which the file says).

## Where this project is deliberately different

- **It says why.** A missing figure is `no forecast, because ...`, never a guess
  presented as a number.
- **Duplicate detection is explainable** and kind-agnostic: the same initiative
  chartered twice under different names is a finding with its evidence.
- **Secure by default:** login enforced, security headers, rate limiting, a
  recovery plan with stated targets, and an information governance pack are
  part of the repository, not an extra.
- **Plain durations:** every duration is in "calendar days" or "business days".
- **Honest status:** the backlog says what each item does **not** do.

## Where it is behind

- No mobile app and no desktop application.
- No real-time co-editing.
- A single maintainer and a young codebase: no long production history, no
  third-party audit, and no published load test ([BENCHMARKS.md](BENCHMARKS.md)).
- The Welsh and other translations are drafts awaiting review.
- Change requests are specified but not built ([`plan.md`](plan.md) §57).
- Plugin and marketplace ecosystems that mature products have.

## Correcting this page

If something here is wrong or out of date, open an issue with the correction and
a source. See [CONTRIBUTING.md](CONTRIBUTING.md).
