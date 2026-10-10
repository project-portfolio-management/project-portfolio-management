# AI Statement

Version 1.0.0, status is active.

Canonical location is `AI_STATEMENT.md` at the repository root.

Review at least at every major or minor release.

Abstract. This document discloses how artificial-intelligence tools are used to
develop this software. It states what the tools do and do not touch, who is
accountable, which controls bound the work and how each is enforced, the
licensing and data posture, the rules for contributors, the uses that are
prohibited, and the limitations that survive all of it. It is a self-declaration
by the maintainer, written for evaluators and regulated adopters performing
supplier due diligence, and it changes in the same pull request that changes the
practice it describes.

The key words shall, should and may are used as ISO/IEC Directives Part 2
defines them: requirement, recommendation, permission.

## 1. Scope

This document covers the use of AI tools in developing everything in this
repository.

It does not cover an AI system in the product, because there is none: the
software contains no model, calls no model and sends nothing to a model. AI is
used to build the software, in the sense that a compiler, a linter or an
autocomplete is used to build it, but at a much higher level of delegation.

## 2. Which frameworks apply here, and which do not

- The EU AI Act imposes no obligation on this project. The Act binds providers
  and deployers of AI systems (Articles 2 and 3(1)); this software is not one.
  This document is voluntary.
- This project does not classify itself as a medical device or as any regulated
  product. A downstream integrator who gives their product a regulated purpose
  makes that classification; this document exists partly so they can answer
  their own supplier questions.
- ISO/IEC 42001 and the NIST AI RMF are used as vocabulary, not claimed as
  conformity. No certification is claimed and no audit has occurred. The words
  "certified", "audited" and "validated" appear in this document only inside
  this sentence, to say they do not apply.

## 3. Terms

This document uses the W3C AI Content Disclosure vocabulary:

- none: entirely human-authored
- ai-assisted: human-authored; AI edited, refined or filled in boilerplate
- ai-generated: AI-generated with human prompting and review
- autonomous: AI-generated without meaningful human oversight
- agentic tool: a tool that plans and executes multi-step work under a human's
  direction, as opposed to inline completion.

## 4. Accountability

[MAINTAINERS.md](MAINTAINERS.md) names the accountable maintainer for every
change in this repository, whatever tool produced the bytes.

## 5. Where AI is used, and at what level

The tooling is agentic AI coding assistance (currently Claude Code, with
Anthropic's Claude models), operated in sessions the maintainer directs and
reviews. Levels below use the §3 vocabulary, per development activity. No
percentage appears anywhere in this document: no defensible method exists for
measuring one.

| Activity | Level | Notes |
|---|---|---|
| Application and tooling code | ai-generated | written in directed sessions; reviewed by the maintainer |
| Tests | ai-generated | held to the same authority as the code they test; a test is never weakened to make something pass (§11) |
| Documentation, the specification text and this statement | ai-generated | the maintainer's requests are recorded in the specification and `tasks.md`, which state what is built and what is not |
| Specification rulings, owner decisions, release decisions | none | decided by the maintainer and recorded in the specification (see [GOVERNANCE.md](GOVERNANCE.md)) |
| Translations | ai-generated | the Welsh interface text is an AI draft; a professional translator's review is recorded as **not yet done** in [`governance/welsh-translation-review.md`](governance/welsh-translation-review.md) |
| Review verdicts on others' contributions | none | not delegated |

## 6. Human oversight

The maintainer directs the work, reads the results, and decides what is kept.
In a session the tool may edit files, run the tests and, when the maintainer
tells it to, commit and push. Those are the maintainer's actions carried out by
a tool on instruction; the maintainer remains accountable for them.

Decisions with consequences (what a specification silence means, what ships in a
release, what is deliberately not built) are written into the specification, the
plan or the backlog, with the reason. A decision that exists only inside a tool
session is not a decision this project made.

## 7. Quality controls

These controls apply to AI-written and human-written changes alike, and they can
disagree with this document:

- **Specification and backlog.** The [specification](spec/index.md) is the
  authority; [`tasks.md`](tasks.md) says for each item what was built and, in
  words, what was **not**. `scripts/generate-status.sh --check` fails when the
  generated status block drifts from the code, and `scripts/check-links.py`
  fails on a dead relative link.
- **Tests.** Unit tests, database-backed request tests, container tests (a real
  Keycloak), a PostgreSQL restore drill and browser tests with an
  accessibility audit of every route run in CI. Tests that cannot run in a given
  environment are listed as not run in the change that adds them.
- **Static and supply-chain gates.** `cargo fmt --check`, `cargo clippy -D
  warnings`, `#![forbid(unsafe_code)]` in the Rust crates, `cargo deny`
  (advisories, licences, bans, sources), `svelte-check` and Dependabot.
- **Honest status.** Documents state deviations plainly: for example, the
  audit trail is not a hash chain, and a feature "built" in the backlog says
  which parts are not.

## 8. Licensing and provenance of AI output

The repository is offered under the licences in [LICENSE.md](LICENSE.md). The
position taken here follows the Apache Software Foundation's and LLVM's
published reasoning: an AI tool's output does not launder anyone's copyright,
the full provenance of generated text is generally not knowable, and prompting
alone is not treated as authorship. In practice: contributions of substantially
copied third-party material are refused however they were produced; generated
code is held to the same originality expectations as human code, under the same
review; and if identifiable third-party material is found in the tree, it is
removed or licensed properly, exactly as for a human-introduced copy. The tools
are used under terms that do not restrict the output's use in licensed software.

## 9. Data

The repository is intended to hold no real personal data, no customer data and
no secrets: test data is synthetic, fixtures use invented names, and the
[personal-data inventory](governance/personal-data-inventory.csv) lists every
database column that can hold personal data, with a test that keeps it true to
the schema. This is a property a reader can check against the tree, not a
promise about tool behaviour. What the tool vendor does with a prompt is
governed by the vendor's terms; this document makes no claim on the vendor's
behalf, because such claims go stale silently. The maintainer is responsible for
not putting real personal data or secrets into a session.

## 10. Rules for contributors

Contributors may use AI tools. A contribution with **ai-generated** content per
§3 shall say so in the pull-request description: which tool, and what it did.
The contributor remains responsible for their submission in full, under the
same [CONTRIBUTING.md](CONTRIBUTING.md) bar as any other work: understood,
explained on request, tested and honest.

**Commit trailers.** Commits made in an AI-assisted session carry a
`Co-Authored-By:` trailer naming the model. That is the practice in this
repository today, and it is why many commits carry one (about three in four at
the time of writing); the trailer is a courtesy marker, not a measure. Commits
made before the practice began, and commits made by hand, carry none.

## 11. Prohibited uses

In this project, AI shall not: merge anything on its own initiative; adjudicate,
score or answer reviews of contributions; sign anything; decide a
specification-facing question (silences are adjudicated by the maintainer and
recorded); or weaken a test, an expectation or a gate to make something pass.
The last is a standing hard rule for humans and tools alike.

## 12. Limitations and residual risks

This section exists because a disclosure without one is marketing.

- **The gates prove what they test, not correctness.** The tests show the
  behaviours they cover; coverage is broad, but it is a boundary.
- **Review depth is one person's.** The project has a single maintainer;
  machine gates stand in for the review capacity a larger team would have
  ([GOVERNANCE.md](GOVERNANCE.md) says this plainly). "The maintainer can
  explain every merged change" is the honest claim; "every line was
  independently re-derived" would not be.
- **Translations are drafts.** See §5.
- **Retroactivity.** This document describes the practice, not a per-commit
  audit trail, and no such trail is claimed.
- **Provenance uncertainty survives.** Whether any generated fragment echoes
  unlicensed training material is not fully knowable with current tools; §8
  states the handling, not a guarantee.
- **The legal ground is unsettled.** Copyright in AI output is an open question
  in most jurisdictions; this document records positions, and positions may
  have to change. §13 names the triggers.
- **This is a self-declaration.** No third party has audited it.

## 13. Review and change

This statement is reviewed at every major or minor release, and revised
off-cycle when any of these happens: the tooling changes materially, a tool
vendor's terms change in a way §8 or §9 relies on, a binding rule emerges (AI
Act guidance touching this use, a foundation policy this project follows, a
court decision on AI output and copyright), or a claim in this document stops
being true. The maintainer owns the review; the change lands like any other,
and the version and change log update in the same change.

## 14. Reporting

A suspected provenance, licensing or quality problem in this repository,
including a claim in this document that does not survive checking, is a report
this project wants. Open an issue and cite this file; for anything
security-sensitive, use the private route in [SECURITY.md](SECURITY.md).

## 15. References

**Normative for this project:** the licences ([LICENSE.md](LICENSE.md));
[GOVERNANCE.md](GOVERNANCE.md), [MAINTAINERS.md](MAINTAINERS.md),
[CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md); the
[specification](spec/index.md) and [`agents/`](agents/index.md) guidance.

**Informative** (the sources whose vocabulary and positions this document draws
on): the W3C AI Content Disclosure vocabulary; the ISO/IEC Directives Part 2
document conventions and verbal forms; the Apache Software Foundation's
generative-tooling guidance; the Linux Foundation's generative-AI policy; the
OpenSSF security guidance for AI code assistants; NIST AI RMF and ISO/IEC 42001
as vocabulary; EU AI Act Articles 2, 3 and 50. This statement follows the
structure of the AI statement published by the FerroEHR project, adapted to
this project's facts.

## Annex A. Change log

| Version | Date | Change |
|---|---|---|
| 1.0.0 | 2026-10-10 | First issue. |

## Annex B. Machine-readable summary

Levels per the W3C AI Content Disclosure vocabulary (§3); the prose above is
authoritative where the two could ever disagree.

```yaml
ai-statement:
  version: 1.0.0
  last-updated: 2026-10-10
  vocabulary: w3c-ai-content-disclosure
  disclosure-default: ai-generated
  tools:
    - name: Claude Code
      provider: Anthropic
  processes:
    design: ai-assisted
    implementation: ai-generated
    testing: ai-generated
    documentation: ai-generated
    translation: ai-generated
    review: none
    adjudication: none
    release-decisions: none
  ships-ai-system: false
  autonomous-use: none
```
