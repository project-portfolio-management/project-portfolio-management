# Service management (proposed 2026-11-05)

> **Status: proposed. SM-1 is built; nothing else is.** Tasks SM-1 to SM-17 in [`tasks.md`](../tasks.md);
> decisions SM-D1 to SM-D6 are the owner's. The research is in [`plan.md`](../plan.md) §85 to §87. The sister file is
> [product-evaluation.md](product-evaluation.md).

## What it is for

A project ends and the thing it delivered goes on running. Issues, requests and security reports keep arriving, and today they fall
outside anything this service measures. This spec covers what happens after go-live: incidents, requests, problems, service levels and
security reporting, tied to the product and to the plan that built it.

**This is not a help desk.** Open-source service desks are mature: [GLPI](https://glpi-project.org/features/) has assets, incidents, problems,
changes and SLAs, [iTop](https://github.com/Combodo/iTop) has a full CMDB and ITIL processes, and [Zammad](https://zammad.com/en/product/features)
has the channels, including email, chat, SMS, WhatsApp and phone. Competing with them on channels, agent speed or infrastructure
discovery would be a poor use of one maintainer. What none of them has is the line this service can draw: **from a service's incidents back
to the plan, the business case and the benefits that justified it.** The aim is a small, honest service record that serves that line,
that an organization with no service desk can use, and that one with a service desk can feed.

## What exists today

Checked against the code on 2026-11-05.

| Needed | Here today | Standing |
| --- | --- | --- |
| Ticket types | `issue_types`, with custom types, per plan (OC-1, OC-5) | Have the mechanism; no incident, request or problem seeded |
| Workflow per type | Issue workflows, role-restricted moves, field rules | Have |
| Priority | `issues.severity`, `issues.priority` | Have the fields; no impact and urgency matrix |
| Open, waiting, done | `status_category` on every issue | Have, and it is what a paused clock needs |
| Escalation | `escalations`, with levels and `respond_by` | Have |
| Change | `change_requests`, `issues.change_request_pid` | Have |
| Links between issues | None: `source_risk_pid`, `source_task_pid`, `change_request_pid` only | Missing |
| When things happened | `raised_on` and `resolved_on` are **dates**. Age is in calendar days | **Missing, and it blocks all of the rest** |
| Service level clocks | None. `delivery_exceptions` and the breach tasks are about plan tolerances | Missing |
| Business hours | `work_calendars` (weekdays and holidays), used for scheduling | Have, not yet applied here |
| Requester | `raised_by` is a token subject; nobody outside can raise one | Missing |
| Satisfaction | `satisfaction_responses` per plan | Have, not per ticket |
| Knowledge | `lessons`, `controlled_documents` | Partial |
| Confidential issues | Access is **per plan**. Everyone who can read the plan reads every issue | **Missing, and it blocks security reporting** |
| DevOps | `devops_events`: deploys, incidents, recoveries, for DORA-style metrics | Have; a parallel notion of incident |
| This project's own reports | `SECURITY.md`, email, with stated targets, and `security.txt` | Have |

Two gaps stand out, and both are small to state. **An issue has no times.** And **an issue cannot be confidential within its plan.**

## The design

### A service is a product in use

No new "service" record and no CMDB. A service is a product plan in an `in_use` stage (PE-1). Its `parent_ref` and dependencies say what it
relies on, and `plan_links` say where it lives in an outside system, by `system` and `external_id`. This is a deliberate floor: enough to
hang records on, and no more. An organization with a real CMDB keeps it and links to it.

### Tickets are issues

No ticket table. **Incident, request, problem and security report are issue types,** seeded and retired like any other, with their
workflows as data. Every existing screen, filter, role and field rule then applies, and an organization can rename them. `devops_events`
of kind `incident` keep their role as a measurement feed. SM-7 links the two, so an outage is not recorded twice with two answers.

### Times on an issue (SM-2)

An issue gains `reported_at`, `acknowledged_at`, `first_response_at`, `resolved_at` and `closed_at`, and for an incident `occurred_at` and
`detected_at`. The existing dates stay and are derived. Times are stored in UTC and shown in the reader's zone, as elsewhere. A person may
correct `occurred_at`, with the correction audited, because the real start is often only known later. Existing issues keep their dates and
show no time: **nothing is back-filled by guessing,** and an SLA report says how many issues it left out for that reason.

### Service levels

A **service level** belongs to a product or to the deployment, and has targets per type and priority: time to respond and time to resolve,
as [iTop's TTO and TTR](https://ithelp.ithome.com.tw/articles/10363670) are (a tutorial, not iTop's own documentation, which was not reached). It names a work calendar. The clock:

- runs in the calendar's working time, or in calendar hours for a clock that the law sets;
- **pauses in a `waiting` status category**, each pause recorded with who and why;
- restarts on reopening, with the reopening counted;
- is **derived from the transition log**, not from a counter that can drift, so a report can be rebuilt and checked.

A breach is computed by a pure rule, the same one for a screen, a report and a job. Before and at a breach, it can notify, raise an
`escalation` and fire an automation. A report states the met share per target **with its count**, and states what it left out, such as tickets
with no times, in the manner of "not assessed" elsewhere. A paused ticket is not hidden in the figures: total paused time is shown beside them.

### Priority

Impact and urgency each come from a short list and set the priority through a matrix an organization can edit. A person can override it,
with a reason recorded.

### Requests and the requester

- **A request form** per product: a template of the issue type, its fields and its service level. The set of forms is the service catalogue.
- **A requester** is a person reference, not always a user of the service. `requester_ref` and a contact are added, with retention, erasure and
  the `governance/personal-data-inventory.csv` entry that a new personal-data column needs.
- **The requester's view:** "my requests", with the public history of each. This builds on guest grants, narrowed to the requester's own
  issues. Guests have confined access today, and the confinement must stay as tight.
- **Internal notes.** Comments gain `visibility`, because a requester must not read staff notes. This needs care: every comment read path
  must honour it, and it gets its own census test.

Email is left out of the core: see SM-D2.

### Incidents and problems

- **Links between issues:** `caused_by`, `duplicates`, `parent_of` and `relates_to`, like task relations and with the same guard (a hidden issue
  gives the answer of one that does not exist, OC-21).
- **A major incident:** a parent with children, an incident lead, and a timeline made from the audit and the comments.
- **A problem** is the cause behind incidents, with its workaround and root cause. A **known error** is a problem with a workaround published
  to staff.
- **A review after an incident** writes to `lessons`, with its actions as tasks or control actions. It is required by rule for a high severity,
  and blameless by wording. The existing lesson search then surfaces it to the next proposal that resembles it, which is the line back to the
  plan that no service desk has.

### What portfolio leaders see

This is the reason to have any of this here. For each product and portfolio, per period:

- incidents by severity, time to resolve, and service levels met;
- **service health beside the plan's benefits and business case,** so a project reported as a success with a failing service reads as what it is;
- **the cost of running it:** time entries on tickets, beside the budget;
- a section in the director pack, built like the others from what is live, not private and not marked;
- the DORA-style metrics, now with real incident times.

## Security reporting

"Security reporting" means three different things, and they must not blur.

### 1. Reporting a vulnerability to this project (SM-1)

**Built 2026-11-05.** `SECURITY.md` is joined by `/.well-known/security.txt`, per [RFC 9116](https://www.rfc-editor.org/rfc/rfc9116.html). A test fails when
`Expires` is under 60 days away, because an expired file is worse than none. **The file is unsigned:** the RFC recommends a signature, and this project has no
published key.

### 2. Reports of a vulnerability in something the organization runs (SM-8 to SM-13)

A **security report** is an issue type, with:

- **Confidentiality.** An issue gains a `restricted` flag and a named access list, within its plan. It is the first access rule below the plan, so it
  must be enforced where issues are read: every list, search, count, export, audit feed, notification, webhook and director pack. The audit of
  2026-11-05 shows what happens otherwise: gaps come from the paths nobody listed. SM-8 is therefore built before any security feature, with a census
  of every path that reads an issue. A restricted issue is **left out of counts** for those not on the list. A plan's count can then differ by reader, as it
  does for private plans.
- **The reporter is protected.** A reporter's identity can be withheld from those handling the fix.
- **Severity and exploitation.** CVSS vector, CVE, CWE, and whether it is known to be exploited ([KEV](https://www.cisa.gov/known-exploited-vulnerabilities-catalog)), as recorded facts with their source. An
  [SSVC](https://www.cisa.gov/ssvc)-style decision (Track, Track*, Attend, Act) records what the organization decided, which is more use than a score alone.
- **Clocks by severity:** remediation targets by severity, as [DefectDojo's findings SLAs](https://docs.defectdojo.com/help/glossary/) are, on the same engine as service levels.
- **Risk acceptance with an expiry,** as DefectDojo's is: an accepted finding stays visible, leaves the clock, carries a decider and evidence, and returns when
  it expires. It links to the risk register. This is the honest form of "we are not fixing that".
- **Disclosure state:** received, triaged, confirmed, fixed, disclosed, with a target date. [ISO/IEC 29147 and 30111](https://secureframe.com/frameworks-glossary/iso-iec-29147) describe this
  ground. Those standards were not read here, only summaries.
- **Machine-readable output:** an advisory exported as [CSAF](https://github.com/oasis-tcs/csaf), with a VEX status (affected, not affected and so on), for those who publish.

### 3. Clocks set by law or contract (SM-10)

A **notification clock** is a service level with hard calendar hours, started from a person's recorded awareness, which is what these duties turn on. Each
is a named policy, shipped as data, with a `reviewed_on` date, for example:

| Policy | Stages | Source read |
| --- | --- | --- |
| EU CRA, actively exploited vulnerability | early warning 24 hours, notification 72 hours, final report 14 days after a fix is available | [Article 14](https://www.cyberresilienceact.eu/the-cyber-resilience-act/) |
| EU CRA, severe incident | 24 hours, 72 hours, final report one month after the notification | Article 14 |
| EU AI Act, deployer | "without undue delay" to the provider or authority; a serious incident "immediately" | [Article 26](https://ai-act-service-desk.ec.europa.eu/en/ai-act/article-26). No hours given: it is shown as a duty with no countdown, never an invented figure |

A GDPR breach assessment was named in the owner's example at 72 hours. The regulation itself was not read for this spec, so SM-10 reads it before shipping the
policy. The service **shows the clock and records who sent what, when**. It does not decide whether a duty applies, and it does not send anything to a regulator.
The CRA's [single reporting platform](https://complexdiscovery.com/cyber-resilience-act-reporting-starts-sept-11-on-an-unfinished-platform/) is out of scope.

### Security findings: integrate, do not rebuild

[DefectDojo](https://github.com/DefectDojo/django-DefectDojo) (findings and deduplication) and [Dependency-Track](https://github.com/DependencyTrack/dependency-track) (components against an SBOM) do
this deeply and are open source. This service should **receive their summaries**, not parse scanner output: SM-13 takes a signed webhook per product with counts by severity and
overdue counts, and links back, on the model of `/api/devops/events`. A portfolio view then answers the question leaders have: which of our live services carry overdue critical findings?

### This project's own standing under the CRA

Article 14 has [applied since 11 September 2026](https://www.cyberresilienceact.eu/the-cyber-resilience-act/) to manufacturers. Article 71(2) names no exception for Article 24, so duties on
open-source software stewards appear to follow the general date, 11 December 2027, and secondary sources disagree. A steward is a legal person, and this project has one maintainer and sells nothing. **Whether either role fits is a
legal question this spec does not answer** (SM-D5). A body that deploys this software commercially may be a manufacturer of what it ships.

## Security and privacy

- All routes sit under the plan guard or in `plan-guard-census.txt`. The requester's view is the most exposed new surface, and gets its own threat review before SM-6 ships.
- A requester's contact details are personal data: inventory, retention, erasure and pseudonymization apply from the first commit.
- An SLA report that crosses plans goes through `HiddenScope`. A clock's job runs without a caller, as other scheduled jobs do, and its notifications respect restricted issues.
- A webhook carrying security summaries is authenticated and rate limited, and it stores counts, not vulnerability detail, unless the owner decides otherwise.
- A notification about a restricted issue says only that something needs attention, because notifications store their words (OC-23).

## Tasks

See [`tasks.md`](../tasks.md), "Service management (SM-n)".

## Decisions (proposed, the owner's)

- **SM-D1. This is a service record, not a help desk.** **Confirmed by the owner 2026-11-05.**
- **SM-D2. No inbound email in the core.** Recommended. An adapter or a webhook can create issues, and Zammad or GLPI can front the channels. Parsing mail is a large security surface for one maintainer.
- **SM-D3. Tickets are issues.** Recommended.
- **SM-D4. Restricted issues are built first,** before any security feature. Recommended, and not one to reorder: a security report in a plan-wide list is worse than none.
- **SM-D5. Take legal advice on the project's own CRA role** before stating one anywhere. Recommended.
- **SM-D6. No CMDB.** Recommended. Products, dependencies and links are the floor.

## Not taken

- **Mail, chat, phone and social channels,** and a shared agent inbox.
- **An infrastructure CMDB or discovery.** GLPI and iTop do this, and the link is by `plan_links`.
- **On-call rotas and paging.** A webhook can call what the organization already uses.
- **A public status page.**
- **Vulnerability scanning or parsing scanner reports.**
- **Sending anything to a regulator,** or deciding that a duty applies.
- **Automatic replies or ticket summaries by AI.** Not ruled out for ever, but not before a person can see what was done and why.
- **Billing tickets to customers,** as a professional-services tool does. §1.3 stands: this is not a finance system.
