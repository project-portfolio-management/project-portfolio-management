# Project management standards and methods

The product is built for any country (`../02-scope.md` §2.0), so no one method is its model. [PRINCE2](../prince2/index.md)
has the longest chapter only because it was mapped first; it is **one example**, of UK government origin, beside the
international standard and the other widely used bodies of knowledge mapped here (WW-1.22).

**Read this first.** Written on 2026-10-31 from general knowledge of each publication. **None of the texts was read for
this chapter**: topics are named as each publication is commonly summarised, without clause or section numbers, and
nothing here is checked against a current edition. It is a map for finding where a feature sits, not a statement of
conformity.

## 1. ISO 21502:2020 (project, programme and portfolio context; project management guidance)

The international guidance standard, and the most neutral reference. Its management practices, against this service:

| ISO 21502 practice (topic) | Where this service stands |
|---|---|
| Planning | plans, tasks, dependencies, calendars, the critical path and critical chain |
| Benefits management | `benefits`, business-case targets, benefit validation |
| Scope management | the plan tree, work breakdown, change requests |
| Resource management | allocations, capacity, leveling, the workforce bridge |
| Schedule management | baselines, variance, schedule health, what-if |
| Cost management | budget lines, cost rates, earned value, forecasts |
| Risk management | risks, joint risk, schedule and cost risk |
| Issue management | the issues register |
| Change control | change requests with a named approver |
| Quality management | **weak**: no products with quality criteria (as the PRINCE2 map also finds) |
| Stakeholder engagement and communication | announcements, comments, the director pack, notifications |
| Organization and governance | access, roles through ABAC attributes, gate reviews, escalation levels |
| Reporting | status updates, the director pack, exports |
| Information and documentation | controlled documents and versions |
| Procurement | **not built** |
| Lessons learned | lessons and retrospectives |
| Closing | the closing phase and handover |

ISO 21503 (programme management) and ISO 21504 (portfolio management) are the matching guidance for the programme and
portfolio levels; they are not mapped yet.

## 2. PMI: the PMBOK Guide (seventh edition) and the Standard for Project Management

Principles-based; organised around **performance domains**: stakeholders, team, development approach and life cycle,
planning, project work, delivery, measurement and uncertainty. The service's nearest equivalents: stakeholders and team
(people references, RACI, capacity); development approach (custom workflows, flow metrics, plan kinds); planning and
project work (plans, tasks, schedules); delivery (benefits, milestones, sign-offs); measurement (earned value, flow,
delivery confidence, KPIs); uncertainty (risks, schedule and cost risk). PMI's *Standard for Portfolio Management* and
*Standard for Program Management* are not mapped yet.

## 3. IPMA: the Individual Competence Baseline (ICB4)

A competence model rather than a process: **perspective** (strategy, governance, compliance, power and interest,
culture), **people** (leadership, teamwork, conflict, negotiation and the like) and **practice** (design, requirements,
scope, time, organisation, quality, finance, resources, procurement, plan and control, risk and opportunity, stakeholders,
change and transformation, select and balance). The practice elements match the ISO 21502 rows above; the people
elements are outside what software records. IPMA's organisational baseline (OCB) is not mapped.

## 4. The terms (checked in this repository)

Some words in the specification and the interface are common to several methods, some are one method's:

| Term here | What it means here | Where else it is used | One method's? |
|---|---|---|---|
| **Tolerance** (`plan_tolerances`) | the permitted deviation on a dimension before a plan must be escalated | PRINCE2's term; ISO 21502 and PMI speak of thresholds and control limits | PRINCE2 vocabulary, used neutrally |
| **Escalation** (`escalations`, levels) | raising a matter to the next level of authority | every method, and ordinary English | no |
| **Gate stage** (`g0_concept` … `g5_benefits`) | an ordered decision point on whether to continue | stage-gate reviews in general; the UK's Gateway reviews; PMI phase gates | the name is generic; the six steps are this product's own |
| **Management stage** | PRINCE2's planning and control unit between decision points | PRINCE2 only | yes (see the PRINCE2 chapter's vocabulary trap) |
| **Exception** (`delivery_exceptions`) | a breach of a tolerance that needs a decision | PRINCE2's "manage by exception"; others call it a variance or a threshold breach | PRINCE2 vocabulary, used neutrally |
| **Phase** (`Initiating` … `Closing`) | the plan's life-cycle phase | the PMI process groups' names | PMI-style names, used neutrally |

The interface keeps these words; a deployment that uses another method's words can rename them in its own catalogue
copy (no setting does it). Not done: no per-deployment vocabulary setting.
