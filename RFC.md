# Request for comments

This project is young and has one maintainer. It wants to learn where it is
wrong, unclear or missing something, before more is built on top. This page says
what kind of feedback helps. Answer in an issue; a short, specific answer is
better than a long general one.

## What the project is trying to be

A portfolio and project-management system that is **secure by default**,
**explains itself** (every figure says how it was worked out, and a missing input
gives a stated reason, never a guess), works for **both managers and individual
workers**, and is **usable in more than one language**.

## What we would like to learn

1. **Who is it for, really?** Is the split (project managers and the individual
   workers whose tasks they plan) the right one? What would make you give it to
   your team, or stop you?
2. **The model.** One recursive collection of plans with an optional `kind`
   label (Portfolio, Project, Program, ...) and sub-resources (tasks, risks,
   issues, budget lines). Does that fit how your organization nests work, or is
   a fixed hierarchy something you need? ([`spec/05-domain-model.md`](spec/05-domain-model.md))
3. **Duplicate detection.** Explainable matching with a default threshold of 0.85
   to catch the same initiative chartered twice. Is that threshold sensible for
   your data, and is the explanation useful? ([`spec/16-open-questions.md`](spec/16-open-questions.md), OQ-7)
4. **Scheduling.** Critical path, float, levelling, what-if and Monte Carlo risk,
   with every duration in calendar or business days. Which scheduling features do
   you rely on that are missing, and which present ones would you never use?
5. **Honest reporting.** Does "no forecast, because ..." beat a number that might
   be wrong? Where would you rather have the number?
6. **Governance and change control.** Issues, change requests, gate reviews,
   controlled documents, approvals. What does your organization require that is
   not here? (The change-request design is in [`plan.md`](plan.md) §57, with four
   questions for the owner.)
7. **Information governance.** The [governance pack](governance/README.md) is a
   starting point for a data protection impact assessment, retention schedule and
   accessibility statement. Is it close to what your information governance team
   would accept? What is missing?
8. **Languages.** The Welsh interface is an unreviewed draft. If you read Welsh,
   what is wrong? For other languages, which matter most?
9. **Operations.** Is the deployment and recovery material (RPO and RTO
   statements, a tested restore) what an operator needs?
10. **What should it stop doing?** Features you think are bloat.

## What kinds of feedback help most

- **Specific:** a page, an endpoint, a sentence.
- **Grounded:** what you were trying to do, and what happened.
- **Constructive:** what you would do instead.
- **Honest about context:** your size, sector and constraints, so the answer can
  be weighed.

Feedback that is less useful: general praise or criticism with no example, and
requests to copy another product's whole feature list.

## How to respond

Open an issue titled `RFC: <question number or topic>`. If you would rather not
be public, email the maintainer ([MAINTAINERS.md](MAINTAINERS.md)). Answers that
change the plan are recorded in [`plan.md`](plan.md) or the
[specification](spec/index.md) with the reason, as decisions are in
[GOVERNANCE.md](GOVERNANCE.md).
