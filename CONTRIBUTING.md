# Contributing

Thank you. You can help with your time, your code, your translations or your
money, and you can help by telling the project what it gets wrong.

## Ways to help

- **Use it and tell us what broke or confused you.** Open an issue with what you
  did, what you expected and what happened. The most useful reports name a page
  or an endpoint.
- **Answer the questions in [`RFC.md`](RFC.md).** Constructive feedback on those
  is as valuable as code.
- **Review the Welsh (or another language).** The Welsh interface text is an AI
  draft awaiting a professional translator; see
  [`governance/welsh-translation-review.md`](governance/welsh-translation-review.md).
  Corrections to a text are welcome even without code.
- **Send a change.** Start with an issue for anything larger than a fix, so the
  work is not wasted.
- **Donate.** No donation channel is set up at the moment. If you or your
  organization want to fund the work, write to the maintainer
  ([MAINTAINERS.md](MAINTAINERS.md)); sponsored features and support
  arrangements are possible, as are custom licence terms ([LICENSE.md](LICENSE.md)).

## Before you send a change

1. **Read the specification** for the area ([`spec/index.md`](spec/index.md)) and
   the item in [`tasks.md`](tasks.md). The specification decides behaviour.
2. **Change the three together.** A material change lands with its specification
   text, its code and its test in one change ([`spec/18-change-control.md`](spec/18-change-control.md)).
3. **Run the checks** for what you touched:
   - Rust: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
     `cargo test` (and `cargo test -- --ignored` against PostgreSQL for the service).
   - Front end: `pnpm run check`, `npx vitest run`, `npx playwright test`.
   - Documents: `python3 scripts/check-links.py` and `scripts/generate-status.sh --check`.
4. **Never weaken a test to make it pass.** If a test is wrong, say why in the
   change.
5. **Say what you did not run.** If you could not run something, say so rather
   than implying it passed.

## House rules

- **Spelling:** Oxford English Dictionary spelling, `-ize` endings ("organization", not "organisation") (NFR-10).
- **Durations in days** are always "calendar days" or "business days", never bare "days" (NFR-11).
- **Words on screen** belong in the locale catalogue, not in a page. A test fails on literal English in a template (NFR-19).
- **Say what is not built.** A backlog item records what it does not do.
- **Keep real personal data out** of code, fixtures, issues and prompts. Use invented names.
- **Licence:** by contributing you agree your contribution is offered on the same terms as the repository ([LICENSE.md](LICENSE.md)).

## Using AI tools

You may. Say so in the pull-request description (which tool, what it did) and be
able to explain the change. See [AI_STATEMENT.md](AI_STATEMENT.md).

## Review

There is one maintainer ([GOVERNANCE.md](GOVERNANCE.md)), so review can take a
while. A small, focused change with its test is merged faster than a large one.

## Conduct

See the code of conduct in [GOVERNANCE.md](GOVERNANCE.md#code-of-conduct).

## Security

Do not report vulnerabilities in public issues; use [SECURITY.md](SECURITY.md).
