# Governance

This is a single-maintainer project. This file says how decisions are made, so
that nobody has to guess.

## Roles

- **Maintainer:** Joel Parker Henderson ([MAINTAINERS.md](MAINTAINERS.md)). Merges,
  releases, decides.
- **Contributors:** anyone who sends a change, a report, a translation or a
  review comment ([CONTRIBUTING.md](CONTRIBUTING.md)).

## How decisions are made

1. **The specification decides.** The [entity specification](spec/index.md) is
   the authority for behaviour; each subproject's own `spec/` covers its
   internals, and the entity spec wins on the cross-subproject contract.
2. **A change to the specification is a change here first.** Material changes
   (the domain model, the API surface, compliance scope) land in the same commit
   as the code and the test ([`spec/18-change-control.md`](spec/18-change-control.md)).
3. **A silence in the specification is adjudicated by the maintainer** and the
   ruling is written into the specification. A decision that exists only in a
   chat or a tool session is not a decision this project made.
4. **The backlog of record is [`tasks.md`](tasks.md)**; the reasoning behind it
   is in [`plan.md`](plan.md), including the open questions put to the owner.
5. **Disagreement:** open an issue with the evidence. The maintainer decides and
   records the reason. There is no vote.

## Releases

A release is a `vX.Y.Z` tag. The release workflow builds, tests and publishes;
the notes come from the changelog. The project is pre-1.0 and may change the
API between minor versions; changes are listed in the
[changelog](CHANGELOG.md).

## Licensing

The repository is offered under any one of MIT, Apache-2.0, BSD-3-Clause,
GPL-2.0-only or GPL-3.0-only, and custom terms by arrangement
([LICENSE.md](LICENSE.md)). A contribution is accepted on the same terms.

## Use of AI tools

See [AI_STATEMENT.md](AI_STATEMENT.md). The maintainer is accountable for every
merged change, however it was produced; AI does not merge, decide
specification questions or weaken a test to make it pass.

## The honest limitation

One maintainer means one reviewer. The project relies on machine gates to
stand in for the rest, and says so. If the maintainer is unavailable for a long
time, the repository stays available under its licences and anyone may fork it.

## Becoming a maintainer

Maintainers are added when someone has made sustained, good contributions and
the current maintainer invites them. Write to the address in
[MAINTAINERS.md](MAINTAINERS.md) if you would like to share the work.

## Code of conduct

Be courteous and specific, assume good faith, and criticise the work, not the
person. Harassment is not tolerated; report it to the maintainer.
