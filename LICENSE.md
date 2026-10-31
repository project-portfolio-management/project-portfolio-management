# Licence

This repository is available under **any one of** the licences below, at your
choice, or under custom licence terms by arrangement (contact the copyright
holder, Joel Parker Henderson, joel@joelparkerhenderson.com).

The SPDX expression, used in every `Cargo.toml` and `package.json`, is:

```
MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only
```

| Licence | SPDX identifier | Text |
|---|---|---|
| [MIT](https://opensource.org/license/mit) | [`MIT`](https://spdx.org/licenses/MIT.html) | [`LICENSE/MIT.txt`](LICENSE/MIT.txt) |
| [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) | [`Apache-2.0`](https://spdx.org/licenses/Apache-2.0.html) | [`LICENSE/Apache-2.0.txt`](LICENSE/Apache-2.0.txt) |
| [BSD 3-Clause License](https://opensource.org/license/bsd-3-clause) | [`BSD-3-Clause`](https://spdx.org/licenses/BSD-3-Clause.html) | [`LICENSE/BSD-3-Clause.txt`](LICENSE/BSD-3-Clause.txt) |
| [GNU General Public License v2.0](https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html) | [`GPL-2.0-only`](https://spdx.org/licenses/GPL-2.0-only.html) | [`LICENSE/GPL-2.0-only.txt`](LICENSE/GPL-2.0-only.txt) |
| [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0-standalone.html) | [`GPL-3.0-only`](https://spdx.org/licenses/GPL-3.0-only.html) | [`LICENSE/GPL-3.0-only.txt`](LICENSE/GPL-3.0-only.txt) |

## Why a choice, and why a directory

The permissive licences (MIT, Apache-2.0, BSD-3-Clause) suit organizations that
embed or adapt the code in their own products; Apache-2.0 adds an explicit
patent grant. The GNU licences suit those who want copyleft. Offering all five
lets each user take the one that fits their own constraints. Custom terms are
for cases none of them fits.

All licence texts live in the [`LICENSE/`](LICENSE) directory, one file per
licence in the expression, and nowhere else in the repository. This file is the
only licence-related file outside it. Forge licence detection usually looks for
a root `LICENSE` file, so it may not recognize this layout; this file and the
[README](README.md#licence) carry the statement instead.

`tests/licence_files.rs` in the API crate keeps the manifests, the expression,
`LICENSE/` and this file consistent.

## Services used only for development and testing

These are not part of this repository and are not distributed with it; they are pulled as container images to
run tests.

| Service | Where | Licence | Use |
|---|---|---|---|
| Redis 8 (`redis:8-alpine`) | `project-portfolio-management-service-with-rust/compose.test.yaml`; CI's `ui` job service container | Redis 8 is offered under a choice of AGPL-3.0-only, RSALv2 or SSPLv1 | development and tests only: the UI's session-store tests against a real Redis, and running the service against Redis by hand. The root `compose.yaml` (the example deployment) still pins Redis 7 (`redis:7-alpine`, BSD-3-Clause); a deployer who moves it to 8 takes on one of the three licences above |
