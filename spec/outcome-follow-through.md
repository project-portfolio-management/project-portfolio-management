# Outcome follow-through: from activity to results (proposed 2026-11-24)

> **Status: proposed, nothing built.** Sixteen requirements `OF-1` to `OF-16`, in three tiers, and four decisions for the
> owner, `OF-D1` to `OF-D4`. Tasks are in [`tasks.md`](../tasks.md) (section "Outcome follow-through"), the assessment is in
> [plan.md §95](../plan.md), and the source reading is [lessons/20261124/index.md](lessons/20261124/index.md). The ids match
> the idea numbers in that file, so `OF-3` is its idea 3. Where this file and a built behaviour disagree, the built behaviour
> and its own spec win until the owner changes it.

## What it is for

A portfolio service can record activity very well and still help an organization avoid asking whether the activity became a
result. The pattern is common: plans, programmes, registers and reports exist in quantity, the same findings return review
after review, mitigations are many and risk scores do not move, decisions wait in a queue nobody measures, and actions close
while the problem stays. This spec adds the **reads and records that show whether activity became a result**, and a few that
help a person decide what to stop. It does not add a score, a ranking, or an automatic decision.

**The source is anonymous on purpose.** This file, and every plan section, task, test, demo record, and commit message that
comes from it, names no organization, person, place, sector, programme or document, and uses no quotation or figure from the
source. Describe the pattern in general terms. A new field's example values are invented.

## Principles

These already hold across the service ([time-based analysis](time-based-analysis.md) §12, the exceptions and escalation
work). They are repeated here because each requirement below depends on them.

| Id | Principle |
| --- | --- |
| OF-P1 | **A derived figure states its rule.** Each duration says calendar or business days; each finding shows the rule and the period it used, beside the result. |
| OF-P2 | **Unknown is never zero.** A missing check, an open step, or a risk with no history is shown as `unknown`, `not_measured` or `still waiting`, and is counted on its own line. |
| OF-P3 | **No composite score.** No health, maturity, trust or success percentage. Counts sit side by side, with the items behind each. |
| OF-P4 | **No person metric.** Figures are for a step, a role, a forum, a plan or a portfolio, never for a person. A step only one person can hold is shown by its name and its current items, with no history and no ranking ([§12.4](time-based-analysis.md)). |
| OF-P5 | **Optional, or a finding.** A new field is optional, and a new rule is a finding that asks a question. It refuses a write only where a requirement says so. Extra required fields and extra approval steps were part of the problem being answered. |
| OF-P6 | **A person decides.** Nothing stops, escalates, closes or verifies by itself. The service shows what is waiting. |
| OF-P7 | **Independence is checked, not assumed.** Where a requirement says "not the owner", the service compares the actor with the owner and refuses the write otherwise. |

## First tier: show whether activity became a result

### OF-1 A recommendations register

A register above plans for recommendations that come from outside a plan (a review, an audit, an inspection, a self-assessment).

- A **source** (a label and a date the deployment chooses, and an optional link) and its recommendations with their text.
- A **response**: `accepted`, `accepted_in_part` or `not_accepted`, with a reason. Not accepted is an honest answer and is kept.
- One accountable **owner** (a role or an opaque person reference), a due date, and links to the plans, tasks or control actions that carry it out.
- **Dependencies** between recommendations, on the existing dependency rules. Starting one before the one it relies on is a finding. It never refuses the write, because work may overlap.
- **States** that do not let activity pass for success: `open`, `in_progress`, `done` (the owner says so), `verified` (someone who is not the owner, with evidence), `sustained` (OF-2) and `relapsed`.
- A **theme** from a short vocabulary the deployment sets, and a **recurrence view**: themes that appear in more than one source, and themes that came back after a recommendation was verified.

**Acceptance.** The owner cannot verify their own recommendation (OF-P7). A recommendation set to `verified` with no evidence is refused. The recurrence view names the sources and dates behind each line. Counts by state list `unknown` separately (OF-P2).
**Not covered.** Importing a source document; reading a recommendation's wording for a theme (a person assigns it); any score for a source.

### OF-2 An effect check after completion

An optional **effect check** on a recommendation, a control action, a benefit or a closed plan: the measure, the starting value, the value expected, and the dates to look again. The result is `held`, `partly`, `not_held` or `not_measured`, recorded by someone other than the owner. The state `sustained` needs a stated number of consecutive checks that held, with an unknown or missed check counting as not met (the rule the workforce service already uses for consecutive periods; the two agree without sharing code).

A portfolio view puts **actions completed** and **effects held** side by side with the not-measured count between them, and gives no percentage of success.

**Acceptance.** A missed check shows as `not_measured`, never as held. Closing a plan does not close its checks. The view lists the items behind both numbers.
**Not covered.** Reading the measure from another system (the value is entered); a forecast of whether an effect will hold.

### OF-3 Risk score history, and mitigations that can be counted

Four steps, in this order:

1. An append-only `risk_scores` history (score, date, actor, reason), written by **every** create and update of a risk. This is the time-sensitive one: today an update overwrites probability and impact and audits `risk_updated` with no values, so every month without it loses history that cannot be rebuilt.
2. An optional `risk_pid` on an escalation, and the wording of the risk **as it stood** at each level, so that softening on the way up can be seen.
3. Mitigations as rows with an owner, a due date, a state and a target score.
4. A **flat risk** finding: several mitigations done and no change in score over a stated period. It asks the owner a question and is not a verdict.

Inherent and residual scores are not advised at first; history and target answer the question at lower cost.

**Acceptance.** No path changes a score without writing a history row. A risk with no history before the migration says so (OF-P2). The flat-risk rule and its period are shown with its result (OF-P1).
**Not covered.** Rescoring old risks; a risk-appetite threshold; any aggregate of scores.

**Built (25 November 2026), OF-3 step 1 only.** Table `risk_scores` (probability, impact, score, `recorded_at`, `actor`, `reason`, `backfilled`), append-only, written by a database trigger on `risks`, so the two create paths (risk and canvas) and every update that changes probability or impact each leave a row; an edit that leaves the score alone leaves none. `PUT /api/plans/{pid}/risks/{risk_pid}` takes an optional `reason` (at most 500 characters). Risks that existed at the migration got one row marked `backfilled`: their first *known* score, not their history. `GET /api/plans/{pid}/risks/{risk_pid}/scores` reads it, oldest first. `POST …/risks` and the canvas promotion to a risk (`POST /api/canvases/{pid}/items/{item_pid}/promote`) take the same optional `reason`. The risk form on a plan's governance page has a reason field (sent only with a changed score) and lists the history. Not built: steps 2 to 4, and any chart of the history. Test: `tests/requests/governance.rs`.

### OF-4 How long decisions wait, and where

Cheapest first:

1. A **derived read** over the approval log (PL-2): time waiting at each **step**, steps per approval, and what is waiting now and on which step. It needs no new table.
2. A `decide_by` date and a named deciding role or forum on proposals and gate reviews, with an overdue list.
3. A `returned` outcome with a reason, and a count of returns. Today a rejected proposal cannot go back to draft (`proposal_transition`), so rework happens outside the record.
4. A register of standing **decision limits** per kind and value, turning the free-text `decision_rights` of FR-52 into something a finding can check ("decided two levels above its limit"). It is wanted only if it will be kept up to date (OF-D3).

**Acceptance.** Each duration states calendar or business days (OF-P1). A step still open counts as waiting, not zero (OF-P2). No person-level breakdown exists at any role (OF-P4).
**Not covered.** A target duration; ranking steps; any prediction of a decision date.

## Second tier: focus and ownership

### OF-5 A limit on priorities and deliverables

Optional limits on open objectives per period and on open milestones per plan, with the shape of EX-8 (a refusal or a recorded override, never silence). Beside it, a **rank without ties** for objectives, kept by a person, and a question on each new objective: *what does it replace, or what gives way?* (EX-13 asks this of external requests already.)
**Acceptance.** Unset means no limit. Overrides are listed with their reasons. The limit never hides an item.

### OF-6 A named owner, with the authority to act

One accountable owner on a milestone; a count of open milestones and recommendations per owner **role**; an owner status of `confirmed`, `interim` or `vacant`; and the owner's own declaration of whether they hold the authority needed (`yes`, `in_part`, `no`, `not_asked`), with what is missing. A `no` on an item in delivery is a finding for whoever set the work, not for the owner.
**Guardrail.** The load count is shown to the owner and to those who assign work, never as a ranking (OF-P4). Vacancy and interim cover arrive from the workforce service as a state over the existing bridge, never as a name.

### OF-7 Line of sight, both ways

The reverse list first (objectives that no work serves, a query over `objective_links`). Then a benefit-to-objective link, and an `enables` link from one plan to another recording the version of the parent it was written against. When the parent changes, or was never approved, the enabling plan shows `parent not approved` or `parent changed since`. One read traces objective, plan, milestone, benefit and effect check, with every gap shown as a gap.

### OF-8 Repeated slippage across the portfolio

A read listing milestones moved later more than once, with the original date, net days, reasons, and re-baselines per plan, from `milestone_slips` and `slip_summary`. A second move later raises a question with three answers: *re-plan, reduce, or stop*. Cutovers delayed for safety are excluded, because a delay for safety must never look like failure (EX-6).

### OF-9 A stop that finishes

A **stop checklist** on the DP-5 stop decision: who is told and by when; plans, gates and dependencies that were waiting on it, each given an answer (`released`, `re_pointed`, `closed`); thanks to the people who worked on it; and the lessons recorded. A hold that belongs to the workforce service, such as a recruitment freeze, is sent as a reference for that service to act on. A **long pause** finding covers the opposite case: paused past a stated period, with the list of what is still waiting.
**Acceptance.** A stop is never automatic (OF-P6). Nothing is deleted. An unanswered dependant keeps the checklist open.

## Later: when a deployment asks

| Id | Requirement |
| --- | --- |
| OF-10 | **Phase entry criteria.** Extend EX-6's frozen, co-signed criteria from cutovers to entry into a phase, and schedule a follow-up review at the transition. The result is a readiness **finding**; phase never gates an operational write, and that stays. |
| OF-11 | **What we heard, what we decided.** A consultation record on a plan: the question, who was asked (by group), the themes raised, and for each theme an answer (`changed`, `not_changed_because`, `still_open`). Closing it with an unanswered theme is refused. Counts by group follow the small-group rules (FR-36). |
| OF-12 | **Forums, papers and follow-through.** Tie a meeting to a forum. Ask a paper for five short parts: the decision wanted, the options, the exposure, confidence in the evidence, and what happens if nothing is decided. Report items with no conclusion, actions still open from earlier meetings, and the same item considered in several forums. |
| OF-13 | **The team's view beside the lead's.** A periodic, anonymous confidence answer from the people doing the work, as a distribution with a minimum group size, shown beside the lead's and the independent view, never blended with them (EX-9; FR-36's rule of storing a role, never an identity). See OF-D4. |
| OF-14 | **Quality and fairness beside money.** A proposal states its expected effect on quality, safety and equality, or says `not_assessed`. Both columns appear wherever the money does. The PE-8 assessment templates can carry the long form. |
| OF-15 | **Good practice that spreads.** A plan records that it adopted a lesson (`adopted`, `adapted`, `not_suitable_because`), so the register shows which lessons travelled. |
| OF-16 | **Change load on a unit.** Initiatives that reach the same unit in the same months. The workforce service owns the measure; here it is a read over FB-4 and stays hidden when that bridge is off. |

## Decisions for the owner

| Id | Question | Lean |
| --- | --- | --- |
| OF-D1 | Is the recommendations register part of this service, or a generic link to one kept elsewhere? | Here, because the work that answers a recommendation is here. |
| OF-D2 | Who may verify a recommendation: an independent role (as in confidence), or any named person who is not the owner? | An independent role where one is configured, otherwise any other person; both refuse the owner (OF-P7). |
| OF-D3 | Are decision limits (OF-4 step 4) wanted, knowing they must be kept current to be worth anything? | Not until a deployment will maintain them. |
| OF-D4 | Is an anonymous team view (OF-13) wanted at all, knowing small teams may see nothing because the minimum group size hides them? | Ask a deployment first; it is the most sensitive of the sixteen. |

## Not advised

- **A health, maturity or trust score** (OF-P3; [plan.md §42](../plan.md)). Frameworks existed in the source organization and described something that did not happen.
- **Automatic stopping or escalation** (OF-P6).
- **Confidential concerns about people.** The service has no protected case handling, and access to a plan is too wide for it. Point to the organization's own route; a restricted record (SM-8) is the least that would be needed first.
- **Person-level decision times or completion rates** (OF-P4). They teach people to decide nothing that is hard.
- **An AI summary of risks or papers.** It is refused already, and softening bad news is the fault in question.
- **More required fields everywhere** (OF-P5).

## Sequencing

1. Small reads over data already held: OF-8, OF-4 step 1, and the reverse list of OF-7.
2. OF-3 steps 1 and 2 (the history cannot be rebuilt later).
3. OF-1 and OF-2 designed together, since both need the same "not the owner" rule (OF-P7).
4. OF-5 and OF-6, then OF-9.
5. OF-10 to OF-16 when a deployment asks.

## Limits of this specification

It rests on one reading of one review, of one organization, which relied on testimony and documents. The recommendations it
reviewed were new when it was read, so nothing shows they work. No user of this service was asked. The sizes in `tasks.md`
are estimates. The existing-feature statements were checked by reading the code and migrations; no test was run for them.
