# Product evaluation (proposed 2026-11-05)

> **Status: proposed. Nothing here is built.** Tasks PE-1 to PE-16 in [`tasks.md`](../tasks.md); decisions PE-D1 to PE-D6 are the
> owner's. The research and the standing against other tools are in [`plan.md`](../plan.md) §85 to §87. The sister file is
> [service-management.md](service-management.md): evaluation ends where a product goes into service, and service management
> starts there.

## What it is for

An organization needs one place for the products it is considering, evaluating and using: searchable, with its own fields, with
impact assessments and issue logging, and with every record rolling up to the product. AI tools are the pressing case, but
nothing here is specific to AI. A payroll system or a survey tool is evaluated the same way, and AI adds questions, not a
second process.

The owner's example is a build in HaloITSM: an asset type, three ticket types, an approval process and rules. This spec takes
that shape and fits it to what this service already has.

## What exists today

Checked against the code on 2026-11-05. About half of the example is here already, under other names.

| The example asks for | Here today | Standing |
| --- | --- | --- |
| A record per tool | A plan of `kind` `Product`, in the one recursive collection | Have the record, not the lifecycle |
| Lifecycle stage | Plan phases and `phase_transitions`, gates in `gate_reviews` and `planned_gates` (with `conditions`) | Have the machinery, not the stages |
| Many custom fields | `custom_field_defs`: text, date, bool, integer, list, multi_list, per kind, searchable at `/api/custom-fields/search` | Have; no groups, no number with decimals, plan and task only |
| Global search | `/api/plans/search`, and `/api/attachments/search` for what files say | Have |
| Vendor | `organizations` of kind `supplier`: a name, a kind and an active flag | A stub |
| Issue log per tool | The issues register, with custom types, workflows and field rules (OC-1 to OC-6) | Have |
| Risk | The risk register | Have |
| Expert review | `reviews`: one score from 0 to 100 and a recommendation per reviewer, with consensus | Have, but one number per person |
| Prioritization | `scoring_inputs` (WSJF and RICE inputs) | A different question: which work first, not which product |
| Evidence | `controlled_documents` with versions and approval, `attachments` | Have |
| Approval | `approval_requests`: one named approver, a final decision. `process_executions` can chain steps | One step; a chain needs a process |
| Rules on a date or an event | `automations`, `scheduled_actions` | Have, with few triggers |
| Do not re-request a rejected tool | The matcher, already used for duplicate plans and proposals | Have, and no comparator read here showed this |
| Audit | `audit_logs` on every change | Have |

**Missing altogether:** weighted scoring by criterion, side-by-side comparison, scored questionnaires, a mapping to frameworks,
vendor due diligence, contract and renewal facts, and a catalogue that tells staff what is approved.

## The design

### A product is a plan

A product is a plan of kind `Product` with a one-to-one `product_profile`, as a programme has `programme_profiles`. This is the
main decision (PE-D1). The reasons:

- **Issues, risks, approvals, documents, custom fields, comments, watchers, workflows and audit already hang off a plan.** A new
  table would have to rebuild or generalize each one.
- **Privacy comes free.** A route under `/api/plans/{pid}` is behind the plan guard. The audit of 2026-11-05 (OC-13 to OC-25) found
  gaps on routes outside it, and every new route outside it is a new way to get that wrong.
- **The matcher works on plans,** so a request for a tool that was rejected last year is found by what it does, under a new name.
- **The project's position is one record type.** Portfolios, programmes, projects and products already share it.

The cost is the risk to weigh: **a product is not funded work.** A tool someone merely proposed would appear in every portfolio
total, capacity view and director pack that counts plans. PE-1 therefore starts with a census of those views, and the default is
that a product plan counts in none of them until the owner says otherwise. If the census shows that cannot be held cheaply, the
alternative is a `products` table with a plan created only when a product enters evaluation.

### The lifecycle

`product_profile.stage` is a workflow, using the workflow engine rather than a fixed list, so an organization can rename or add
stages. The built-in vocabulary:

| Stage | Category | Meaning |
| --- | --- | --- |
| `proposed` | intake | Someone asked for it |
| `considering` | intake | Triaged, in the backlog |
| `evaluating` | active | A trial or assessment is running |
| `approved_restricted` | in_use | Approved for named teams or data classes |
| `approved` | in_use | Approved for general use |
| `suspended` | held | Paused because of an issue or a review |
| `rejected` | closed | Assessed and declined. Kept, so it is not requested again unseen |
| `retired` | closed | No longer used |

A move into an `in_use` stage is **gated**: it needs an evaluation decision and, where the template says so, a completed assessment
and its approvals. As with task gates (OC-12), there must be no way round: a stage cannot be set at creation, by import, or by
moving the plan.

A status change is a record with a reason, and `rejected` and `suspended` need one.

### Vendors

`organizations` of kind `supplier` gain a `vendor_profile`: website, country of establishment, sub-processors link, trust page,
contact, and any number of `product_profile` rows pointing to them. A vendor's due-diligence assessment is reusable: a second
product from the same vendor shows the first and its date, and does not repeat it unless it has expired.

### Evaluations: weighted, independent, comparable

An **evaluation** belongs to a product plan, or to a shortlist of several when products are being compared for one need. It holds:

- **Purpose and success criteria,** written before any score, and frozen when scoring opens.
- **A criteria set:** criteria with weights, chosen from a template or written for this evaluation. Weights total 100 and are
  **frozen when scoring opens**, so a weight cannot be moved after the scores are seen.
- **Evaluators,** each scoring every criterion on a stated scale, with evidence. Scores stay **hidden from other evaluators until
  each has submitted**, so that nobody anchors on a colleague. An evaluator can leave a criterion unscored, with a reason.
- **A trial,** if there is one: pilot group, dates, and the measured result beside each success criterion.
- **A decision:** adopt, adopt restricted, extend, or reject, with the reason, the decider and the date.

**The result shows its working, as the rest of this service does.** An unscored criterion stays `not scored`; it is never counted as
zero, and the total says how much of the weight was scored. Evaluators' disagreement is shown, as range and spread, not averaged
away. A **sensitivity check** states whether the ranking changes when each weight moves by 20 percent up or down. A close result reads
as close: "A leads B, but the order reverses if Cost weighs 10 points more" is more use to a board than two decimals. Cost is
compared per currency and never converted, as elsewhere.

None of the open-source comparators read for this spec showed these as a built-in record; a commercial tool such as
[ProcessUnity](https://www.processunity.com/vendor-risk-management/sourcing-rfx/) scores at question level. Weighted criteria and [sensitivity on weights](https://www.dst.defence.gov.au/publication/decision-maker-user-guide)
are established method in decision analysis. Hiding scores until all are in is a judgement, not a cited finding: the search for evidence on it found none.

### Assessments: scored questionnaires, versioned

An **assessment template** is a versioned set of sections and questions. A question has a type (yes or no, one of a list, score, text),
guidance, a required-evidence flag, and optional **references** to clauses of a framework. A template can declare **stops** (an answer
that ends the assessment) and **rules** that derive a tier or add a required approver.

An **assessment** is one template version answered for one product, by named assessors. Its controls table lists each risk, its control,
its owner and its state, and a control can raise a risk in the register. Answers and the template version are kept together, so a record
reads the same after the template changes.

Assessments are **bundled as data**, editable by an organization and marked as starting points:

| Template | Draws on |
| --- | --- |
| AI impact assessment | [ISO/IEC 42005:2025](https://adr-association.eu/adr-awareness-centre/isoiec-420052025-ai-system-impact-assessment), [NIST AI RMF](https://www.nist.gov/itl/ai-risk-management-framework) (Govern, Map, Measure, Manage), the EU AI Act's risk categories |
| Fundamental rights impact assessment | The six elements of [Article 27](https://ai-act-service-desk.ec.europa.eu/en/ai-act/article-27) |
| Data protection impact assessment | This project's own [`governance/dpia.md`](../governance/dpia.md) as a model |
| Security due diligence | SSO, MFA, audit logging, certifications, sub-processors |
| Accessibility and language | The project's own accessibility and locale rules |
| Algorithmic transparency record | The UK [ATRS](https://www.gov.uk/government/publications/algorithmic-transparency-recording-standard-mandatory-scope-and-exemptions-policy) tiers, exported for the organization to submit |

**The service never says a product is compliant.** It records what was asked, who answered, what evidence was offered and who decided.
A tier it derives is labelled as derived from the answers, with the rule shown. Legal classification is a person's decision, recorded
as one. See "Not legal advice" below.

### Approvals: steps, parallel and conditional

Approvals gain **routes**: ordered steps, each with one or more approvers who act in parallel, a rule (all, any, or a majority), and an
optional condition on the assessment (such as "tier is high" or "personal data is yes"). A rejection returns the record with a reason
required. A step can resolve its approver from the product (its business owner), and the existing rule stands that only the named
approver may decide. Routes build on `approval_requests` and `process_executions`, not beside them, and they are open to any subject
that can be approved, so a change request can use them too.

### The decision and what follows it

An approval records the **permitted data classes, the conditions, and a review date**, on the product. The default review is 12 months,
or 6 for a high tier. Scheduled actions then raise:

- a review assessment 30 days before the date;
- a renewal evaluation 90 days before the contract's renewal;
- an **out-of-cycle assessment** when an agreed number of issues are raised on the product in a period.

A condition that nobody is checking is decoration, so each condition can be tied to a control with a cadence, using the existing
`controls`, and a missed reading raises an issue.

### Contract facts, without becoming a finance system

The profile holds licensing model, annual cost per currency, seats licensed and seats used, contract dates, notice period, and a link
to the contract in a document. These are **descriptive**, in keeping with §1.3: no ledger and no invoices. Cost can be a budget line on
the product's plan like any other.

### The register and the catalogue

- **Register:** saved views over product plans, for each stage and for "review overdue", "high tier" and "renewal in 90 days", with
  CSV import and export through the bulk path.
- **Catalogue:** a page any signed-in person can read, of approved products with their permitted data classes, conditions and how to
  get access, and a "request a product" form. Publishing what is approved is the cheapest answer to unapproved tools. It answers the
  question staff actually have: "may I paste this into that?"
- **Duplicate check on request:** the form runs the matcher and shows what already exists, including what was rejected and why. The
  reader sees only what they may see (OC-14).

### Issues against a product

Issues on a product plan are the service-management issues of [service-management.md](service-management.md). Evaluation adds two
things: a failed control or a high-severity issue can move the product to `suspended`, by rule, and the product page shows the whole
record: evaluations, assessments, approvals, issues and changes, on one timeline.

### AI additions

AI systems get extra profile fields rather than a separate model: capability (generation, classification, agentic action and so on),
provider and model version, whether the organization is deployer or provider, whether the vendor trains on its data, and the human
oversight arrangement.

[Article 26](https://ai-act-service-desk.ec.europa.eu/en/ai-act/article-26) gives a checklist that a deployer of a high-risk system
must evidence: human oversight assigned to trained people, monitoring, logs kept at least six months where the deployer controls them,
telling workers and affected people, and reporting risks and serious incidents. PE-12 makes these an assessment with evidence, not a
statement that they are met.

A change of model version is an **event on the product**: it can raise a review, because a model that changes under an approved use
is the commonest way an approval stops meaning what it said.

## Security and privacy

- Every route is under `/api/plans/{pid}`, or is listed in `plan-guard-census.txt` with how it checks. The catalogue route must return
  only `approved` stages, and a private product is left out.
- Evaluator scores are hidden until all have submitted, including from an administrator reading the API, except by break-glass, which
  is audited. A test must prove this, not a comment.
- A vendor's documents can be confidential: they use handling marks.
- A hidden plan's evaluation does not appear in another plan's shortlist comparison (OC-13 to OC-25 are the precedent).
- A prompt or output sample in an issue may hold personal data. The field warns, the retention rules apply, and it is covered by the
  data-subject routes (PPL). A prompt sample is not stored by default.

## Not legal advice

Regulation moves. On 2026-11-05 the sources read said the Digital Omnibus (Regulation (EU) 2026/1744) moved the AI Act's Annex III
duties [to 2 December 2027](https://www.orrick.com/en/Insights/2026/07/EU-AI-Act-Update-Digital-Omnibus-Finalizes-8-Compliance-Changes), and to
2 August 2030 for systems used by public authorities. Those dates come from a law firm's summary; the regulation itself was not read,
and whether the Omnibus changed Article 27's timing was not confirmed. The templates carry a `reviewed_on` date and a note of the text
they follow, and a dated template that has not been reviewed in 12 months says so. Bundled templates are starting points for a
competent person, not advice.

## Tasks

See [`tasks.md`](../tasks.md), "Product evaluation (PE-n)".

## Decisions (proposed, the owner's)

- **PE-D1. A product is a plan,** with a profile. **Confirmed by the owner 2026-11-05,** subject to the PE-1 census. First reading of that census: plans
  are loaded directly in 101 places across 60 files, so there is no one loader to change, but `HiddenScope::of` is called in 75 places across 34 files and is what
  the aggregates pass through. The likely hook is a separate scope for totals that also leaves out product plans, leaving access unchanged, with the
  `tests/private_aggregates.rs` comparison run with a product plan seeded to catch what does not pass through it. That is read from the code, not yet tried.
- **PE-D2. A product plan counts in no portfolio total** unless a person says it should.
- **PE-D3. Scores are hidden until every evaluator has submitted.** Recommended. It costs a little convenience.
- **PE-D4. AI is fields and templates, not a separate module.** Recommended.
- **PE-D5. Bundled templates ship as data,** in the licence of the repository, drafted by machine and marked unreviewed until a competent
  person reviews them, as translations are. **Confirmed by the owner 2026-11-05.** No reviewer is named yet. A template that claims to follow a specific law
  needs a qualified review before it says so.
- **PE-D6. Vendors do not log in** in the first version. Recommended. A vendor questionnaire is sent and entered by staff, or uploaded.

## Not taken

- **A vendor portal, RFP publication and supplier responses.** A procurement system's job, with its own legal duties.
- **Contract lifecycle management** beyond dates and a linked document.
- **A live security rating of vendors.** A bought feed, and a number nobody here can show the working for.
- **Discovering unapproved tools** from network or SSO logs. It needs access this service should not ask for. An import of a list from
  such a tool would be welcome.
- **Running AI evaluations**: benchmarks, red-teaming or prompt tests. Tools such as [Langfuse](https://github.com/langfuse/langfuse) do that;
  a result can be attached as evidence.
- **Saying a product is compliant,** in any wording, on any screen.
- **A single overall score with no working.** The number is shown with its coverage, spread and sensitivity, or not at all.
