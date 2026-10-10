# Maintainers

This project has one maintainer.

| Name | Role | Contact |
|---|---|---|
| Joel Parker Henderson | Sole maintainer; accountable for every change in this repository, whatever tool produced it | joel@joelparkerhenderson.com |

## What the maintainer does

- Reviews and merges every change, and is the only person who does.
- Decides what a silence in the [specification](spec/index.md) means and records
  the decision there (see [GOVERNANCE.md](GOVERNANCE.md)).
- Cuts releases (a `vX.Y.Z` tag; see [`.github/workflows/release.yml`](.github/workflows/release.yml)).
- Receives security reports (see [SECURITY.md](SECURITY.md)).
- Owns the review of the [AI statement](AI_STATEMENT.md).

## What this means for you

With one maintainer, review depth is one person's, and a vacation or an illness
slows everything. Machine gates (formatting, `clippy -D warnings`, the tests,
`cargo deny`, the link checker, the accessibility audit) stand in for part of
the review capacity a larger team would have; they prove what they test, not
more. If this project matters to you and you could share the load, see
[GOVERNANCE.md](GOVERNANCE.md#becoming-a-maintainer).

Code ownership for review requests is in [CODEOWNERS](CODEOWNERS).
