# For the Chief Operating Officer

*Chief Operating Officer: what you run day to day, what the software does for delivery, and what operating it takes.*

## What it does for operations

- **Delivery planning and control:** plans, tasks, dependencies, a critical-path calculation, milestones across plans, sprints and phases, burndown and flow metrics.
- **Capacity:** allocations, availability, leave and non-working periods, resource roles, derived workload and utilization. (A capacity heatmap by skill pool and month for the director pack is not built: `DP-10`.)
- **Time and cost:** timesheets, recorded time, cost rates (finance-restricted), budgets and derived cost.
- **Governance flow:** issues register, change requests with separation of duties (the requester cannot approve), approvals, reviews, joiner/mover/leaver cases.
- **Exceptions:** a red or amber-red delivery confidence opens an exception that needs a cause, the decision needed, a named owner and a decide-by date within 48 hours (window configurable, 1 to 720 hours). Breaches appear in the director pack and in `GET /api/exceptions`.

## What it takes to run

| Need | Detail |
|---|---|
| Service | One Rust service, PostgreSQL 18, optional Redis (production cache, optional shared sessions), a SvelteKit UI |
| Runbook | [operations pack](../../../operations/README.md): deployment, runbook, backup and restore, recovery objectives, upgrade and migration |
| Scheduled jobs | Nothing schedules itself. Retention purge, exception-breach notices and similar tasks must be put on a scheduler by your operators |
| Support | One maintainer, no SLA; budget for your own first-line support |

## Know the limits

- A plan of several thousand tasks is slow to schedule; the full answer is refused above 5,000 tasks unless asked for (`?full=true` or `?only=critical`). Figures and caveats are in [`BENCHMARKS.md`](../../../BENCHMARKS.md); they are an order of magnitude from one machine, not capacity.
- Screens exist for much of the workspace but **not** for change requests, issues, packs or delivery confidence yet; those are API-only today.

## Decisions that are yours

Who owns each exception, who issues the monthly pack (rhythm and reminders are `DP-12`, not built), and what review cadence the exception rule feeds.

## Read next

[Operations pack](../../../operations/README.md) · [Runbook](../../../operations/runbook.md) · [Recovery objectives](../../../operations/recovery-objectives.md)
