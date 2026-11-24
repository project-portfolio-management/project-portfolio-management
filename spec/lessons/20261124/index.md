# Lessons from an independent review of a large organization: ideas for this repository

Date: 2026-11-24. **Status: proposed, nothing built.** This file is an assessment and advice. It changes no behaviour. The owner
chose to take the ideas into the plan on 2026-11-24, so the ids are now taken: plan [§95](../../../plan.md), the specification
[outcome-follow-through.md](../../outcome-follow-through.md) (`OF-1` to `OF-16`, equal to the idea numbers below, and `OF-D1` to
`OF-D4` for the four questions at the end), tasks in [tasks.md](../../../tasks.md) (section "Outcome follow-through"), and
OQ-18 to OQ-21 in [open questions](../../16-open-questions.md). Where this file and that specification differ, the specification
wins.

## Where the ideas come from, and what is left out

The source is a published independent review of how a large organization sets direction, takes decisions, leads its people,
and follows through. **The source is anonymous here on purpose.** This file names no organization, person, place, sector,
programme, or document. It uses no quotation and no figure from the source, and it describes patterns in general terms. Keep
it that way in any plan section, task, test, demo data, or commit message that comes from it.

The patterns are common to many large organizations. The value is in using them as test questions: *if an organization with
these problems used this service, would the service show the problem, or help to hide it?*

**Evidence base.** The whole review was read. What this repository has was checked against `spec/`, `plan.md`, `tasks.md`,
the service code, and the migrations, by reading. No test was run and no service was started. "None" below means a search did
not find it, not that its absence is proved. The UI code was not searched.

## Advice, in one paragraph

The review's central pattern is an organization that had plenty of plans, programmes, meetings, registers, and reports, and
could not turn them into decisions, finished work, and lasting results. A portfolio service can make that worse by recording
activity very well. This service is already strong on money, gates, baselines, independent confidence, escalation, and stop
decisions. **Build first what shows whether activity became a result**: a recommendations register (1), an effect check after
completion (2), risk score history (3), and how long decisions wait (4). Then add **focus and ownership** (5, 6, 7). These
seven fit the existing doctrine: derived figures with a stated rule, unknown never shown as zero, no composite score, and no
measure of a person.

## Summary

| # | Idea | Pattern it answers | What exists here | Priority | Size |
| --- | --- | --- | --- | --- | --- |
| 1 | Recommendations register | The same findings return review after review | None (the `recommendation` lesson category is a label) | First | L |
| 2 | Effect check after completion | Actions close, the problem stays | Validated benefits, adoption; no scheduled re-check | First | M |
| 3 | Risk score history and real mitigations | Many mitigations, scores that do not move | The score is overwritten; mitigation is one text field | First | M |
| 4 | Decision waiting time | Decisions move up, loop, and stall | Escalations and approval logs; no waiting figure | First | M |
| 5 | Focus limit on priorities and deliverables | Too many priorities to deliver any | Limit on programmes in delivery only (EX-8) | Second | M |
| 6 | Named owner with stated authority | Accountable without authority; one person owns most of it | RACI per task, free-text decision rights; milestones have no owner | Second | M |
| 7 | Line of sight, both ways | Enabling plans with no parent strategy | Unaligned plans listed; not objectives without work | Second | M |
| 8 | Repeated slippage across the portfolio | Dates move again and again | Per-milestone slip history | Second | S |
| 9 | A stop that finishes | A paused programme keeps everything waiting | `stopped` is terminal, with a reason | Second | S to M |
| 10 | Phase entry criteria | Everything attempted at once, out of order | Gate order; frozen criteria for cutovers only (EX-6) | Third | M |
| 11 | What we heard, what we decided | People are consulted and never answered | Comments, satisfaction by role | Third | M |
| 12 | Forums, papers, and follow-through | Long papers, no conclusion, no action | Meetings per plan; forums as text | Third | M |
| 13 | The team's view beside the lead's | Bad news softens as it rises | Independent against self confidence | Third | M |
| 14 | Quality and fairness beside money | Money outweighs quality | Value, risk, and cost balance | Third | S |
| 15 | Good practice that spreads | Success stays in one place | Append-only lessons, matched by title | Third | S |
| 16 | Change load on a unit | Change on top of change | People impact passed through from the workforce service | Later | M |

Sizes are estimates: S is days, M is one to two weeks, and L is more, with screens and translations counted.

## Build first

### 1. A recommendations register

**Pattern.** Successive reviews, audits, and inspections reached similar findings. Each produced a plan. Nobody could show
which recommendations were implemented, whether they worked, or that a theme was coming back.

**Here.** Nothing holds a recommendation from outside a plan. `control_actions` (owner, due, outcome, conversion to a task or
an issue) and EX-10 (exit criteria with evidence each quarter) are close in shape.

**Add.** A register above plans:

- a **source** (a label and a date the deployment chooses, with an optional link), and its recommendations with their text;
- a **response**: accepted, accepted in part, or not accepted, with a reason. Not accepted is an honest answer;
- one accountable owner, a due date, and links to the plans, tasks, or controls that carry it out;
- **dependencies** between recommendations, drawn on the existing dependency rules, with a finding when one is started before
  the one it relies on. Work may overlap, so the finding never refuses the write;
- states that do not let activity pass for success: `open`, `in_progress`, `done` (the owner says so), `verified` (someone who
  is not the owner, with evidence), `sustained` (idea 2), and `relapsed`;
- a **theme** from a short vocabulary the deployment sets, and a recurrence view: themes that appear in more than one source,
  and themes that came back after a recommendation was verified.

**Accept when:** the owner cannot verify their own recommendation; a verified one with no evidence is refused; the recurrence
view names the sources and dates behind each line; and counts by state list `unknown` separately.

### 2. An effect check after completion

**Pattern.** Work was reported complete and the underlying measure did not change, or improved and fell back.

**Here.** DP-9 separates claimed from validated benefits, with a validator outside the programme. Nothing schedules a second
look after closure, and no task, control action, or milestone points at the measure it should move.

**Add.** An optional **effect check** on a recommendation, control action, benefit, or closed plan: the measure, the starting
value, the value expected, and the dates to look again. The result is `held`, `partly`, `not_held`, or `not_measured`,
recorded by someone other than the owner. A `sustained` state needs a set number of checks in a row. The workforce service
already uses a rule of this shape (consecutive periods, unknown counts as not met), so the two can agree without sharing code.

A portfolio view then puts two numbers side by side: **actions completed** and **effects held**, with the not-measured count
in between. It gives no percentage of success.

**Accept when:** a missed check shows as `not_measured` and never as held; closing a plan does not close its checks; and the
view lists the items behind both numbers.

### 3. Risk score history, and mitigations that can be counted

**Pattern.** The register was full and the mitigations were many, yet the scores hardly moved. Recording a risk was mistaken
for controlling it.

**Here.** `update_risk` (`src/controllers/governance.rs`) overwrites probability and impact and audits `risk_updated` with no
values, so the earlier score is lost. Mitigation is one text field. `escalate_risk` sets `materialized`, and an escalation
(FR-54) can give `risk` as its reason but does not hold which risk.

**Add, in this order:**

1. an append-only `risk_scores` history (score, date, actor, reason), written by every create and update (S);
2. an optional `risk_pid` on an escalation, and the wording of the risk **as it stood** at each level, so that softening on the
   way up can be seen (S);
3. mitigations as rows with an owner, a due date, and a state, and a target score (M);
4. a **flat risk** finding: several mitigations done and no change in score over a stated period. It is a question for the
   owner, not a verdict (S).

Inherent and residual scores are not advised at first. History and target answer the question at a lower cost.

**Accept when:** no path changes a score without a history row; a risk with no history before the migration says so; and the
flat-risk rule and its period are shown with the result.

### 4. How long decisions wait, and where

**Pattern.** Approvals passed through many layers. Business cases were returned for change again and again. Nobody could say
who would decide, or when, and a delay worked as an unrecorded refusal.

**Here.** Multi-step approvals (PL-2) keep an append-only log, escalations have `respond_by`, and exceptions have `decide_by`.
No figure is computed from them. A rejected proposal has no way back to draft (`proposal_transition`), so rework happens
outside the record.

**Add, cheapest first:**

1. a derived read over the PL-2 log: time waiting at each **step**, steps per approval, and what is waiting now and on which
   step (S). The query needs no new table;
2. a `decide_by` date and a named deciding role or forum on proposals and gate reviews, with an overdue list (M);
3. a `returned` outcome with a reason, and a count of returns (M);
4. a register of standing **decision limits** per kind and value, turning the free-text `decision_rights` of FR-52 into
   something a finding can check: "decided two levels above its limit" (M).

**Guardrail.** Figures are for steps, roles, and forums, never for people. A step that only one person can hold is shown by its
name and current items, with no history or ranking. The service refuses person metrics
([time-based analysis §12.4](../../time-based-analysis.md)), and this must not become one by another route.

**Accept when:** each duration states calendar or business days; a step still open counts as waiting and not as zero; and a
person-level breakdown does not exist at any role.

## Then: focus and ownership

### 5. A limit on priorities and deliverables

**Pattern.** An annual plan held far more priorities and deliverables than could be delivered, so people could not tell what
mattered most.

**Here.** EX-8 limits programmes in active delivery for the deployment. Objectives, aims, and deliverables have no limit.

**Add.** Optional limits on open objectives per period and on open milestones per plan, with the same shape as EX-8: a
refusal or a recorded override, never silence. Beside it, a **rank without ties** for objectives, kept by a person, and a
question on each new one: *what does it replace, or what gives way?* (EX-13 asks this of external requests already.)

**Accept when:** unset means no limit; the overrides are listed with their reasons; and the limit never hides an item.

### 6. A named owner, with the authority to act

**Pattern.** Leaders answered for results without the power to make the decisions involved. Most deliverables were assigned
to one senior role. Some owner roles were vacant or held temporarily.

**Here.** RACI gives one Accountable per task. Milestones have no owner column. Decision rights are text.

**Add.** One accountable owner on a milestone; a count of open milestones and recommendations per owner **role**; an owner
status of `confirmed`, `interim`, or `vacant`; and the owner's own declaration of whether they hold the authority needed
(`yes`, `in part`, `no`, `not asked`), with what is missing. A `no` on an item in delivery is a finding for whoever set the
work, not for the owner.

**Guardrail.** The load count appears to the owner and to the people who assign work. It is not a ranking. Vacancy and interim
cover come from the workforce service as a state, over the existing bridge, and never as a name.

### 7. Line of sight, both ways

**Pattern.** Workforce, digital, property, and finance plans were written with no agreed parent strategy, so they could not
line up with each other.

**Here.** `/api/executive/alignment` lists plans with no objective, with their spend. The reverse is not reported: no list of
objectives that no work serves, no link from a benefit to an objective, and no parent objective.

**Add.** The reverse list first (S, a query over `objective_links`). Then a benefit-to-objective link, and an `enables` link
from one plan to another with an agreed basis: the version of the parent it was written against. When the parent changes,
or was never approved, the enabling plan shows `parent not approved` or `parent changed since`. One read then traces objective,
plan, milestone, benefit, and effect check, with every gap shown as a gap.

### 8. Repeated slippage across the portfolio

**Here.** `milestone_slips` and `slip_summary` hold the moves per milestone, with reasons. The portfolio view is missing.

**Add.** A read listing milestones moved later more than once, with the original date, net days, and reasons, and re-baselines
per plan. A second move later raises a question with three answers: *re-plan, reduce, or stop*. Cutovers delayed for safety
are excluded, since a delay for safety must never look like failure. EX-6 states the same concern.

### 9. A stop that finishes

**Pattern.** A contested programme stayed half alive for a long time. Jobs stayed unfilled and decisions waited on it.

**Here.** DP-5 makes `stopped` terminal, with a reason and a report of what was kept. Nothing is announced or released.

**Add.** A **stop checklist** on that decision: who is told and by when; plans, gates, and dependencies that were waiting on
it, each given an answer (`released`, `re-pointed`, `closed`); thanks to the people who worked on it; and the lessons recorded.
Holds that belong to the workforce service, such as a recruitment freeze, are sent as a reference for that service to act on.
A **long pause** finding covers the opposite case: paused past a stated period, with the list of what is still waiting.

**Accept when:** a stop is never automatic; nothing is deleted; and an unanswered dependant keeps the checklist open.

## Later

10. **Phase entry criteria.** Extend EX-6's frozen, co-signed criteria from cutovers to entry into a phase, and schedule a
    follow-up review at the transition. The result is a readiness **finding**. The specification says phase never gates an
    operational write, and that stays.
11. **What we heard, what we decided.** A consultation record on a plan: the question, who was asked (by group), themes
    raised, and for each theme an answer (`changed`, `not changed, because`, `still open`). Closing it with an unanswered theme
    is refused. Counts by group follow the small-group rules.
12. **Forums, papers, and follow-through.** Tie a meeting to a forum. Ask a paper for five short parts: the decision wanted,
    the options, the exposure, confidence in the evidence, and what happens if nothing is decided. Report items with no
    conclusion, actions still open from earlier meetings, and the same item considered in several forums.
13. **The team's view beside the lead's.** A periodic, anonymous confidence answer from the people doing the work, as a
    distribution with a minimum group size, shown beside the lead's and the independent view. It is never blended with them.
    It suits EX-9, and it uses the satisfaction precedent of storing a role and never an identity (FR-36).
14. **Quality and fairness beside money.** A proposal states its expected effect on quality, safety, and equality, or says
    `not assessed`. Both columns appear wherever the money does. The PE-8 assessment templates could carry the long form.
15. **Good practice that spreads.** Let a plan record that it adopted a lesson (`adopted`, `adapted`, `not suitable, because`).
    The register can then show which lessons travelled.
16. **Change load on a unit.** Show initiatives that reach the same unit in the same months. The workforce service owns this
    measure; here it is a read over FB-4 and stays hidden when that bridge is off.

## Not advised

- **A health, maturity, or trust score.** The service refuses composite scores, and a maturity model (plan.md §42). The review
  is a good argument for that: frameworks existed and described something that did not happen.
- **Automatic stopping or escalation.** A person decides. The service shows what is waiting.
- **Confidential concerns about people.** This service has no protected case handling, and access to a plan is too wide for
  it. Point to the organization's own route. A restricted record (SM-8) is the least that would be needed first.
- **Person-level decision times or completion rates.** They would teach people to decide nothing that is hard.
- **An AI summary of risks or papers.** It is refused already, and softening bad news is the fault in question.
- **More required fields everywhere.** Extra approval steps were part of the problem. New fields are optional, or findings,
  unless this file says otherwise.

## Sequencing

1. Small reads over data already held: 8, 4 (step 1), and the reverse list of 7.
2. Risk score history (3, steps 1 and 2). **Each month without it loses history that cannot be rebuilt.**
3. The recommendations register with effect checks (1 and 2), designed together, since both need the same "not the owner" rule.
4. Ownership and focus (5 and 6), then the stop checklist (9).
5. The rest when a deployment asks for it.

## Questions for the owner

1. Is the recommendations register part of this service, or a generic link to one kept elsewhere? The advice is here, because
   the work that answers a recommendation is here.
2. Who may verify a recommendation: an independent role as in confidence, or any named person who is not the owner?
3. Are decision limits (4, step 4) wanted, knowing they must be kept up to date to be worth anything?
4. Is an anonymous team view (13) wanted at all? Small teams may see nothing, because the minimum group size hides them.

## Limits of this advice

One review, of one organization, was read. It relied on testimony and documents and says so. Its recommendations were new
when it was read, so nothing shows that they work. No user of this service was asked. Every size is an estimate.
