# Licence

This repository is available under **any one of** the licences below, at your
choice, or under custom licence terms by arrangement (contact the copyright
holder, Joel Parker Henderson, joel@joelparkerhenderson.com).

The SPDX expression, used in every `Cargo.toml` and `package.json`, is:

```
MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only OR OGL-UK-3.0 OR CC-BY-SA-4.0
```

| Licence | SPDX identifier | Text |
|---|---|---|
| [MIT](https://opensource.org/license/mit) | [`MIT`](https://spdx.org/licenses/MIT.html) | [`LICENSES/mit/index.md`](LICENSES/mit/index.md) |
| [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) | [`Apache-2.0`](https://spdx.org/licenses/Apache-2.0.html) | [`LICENSES/apache-2.0/index.md`](LICENSES/apache-2.0/index.md) |
| [BSD 3-Clause License](https://opensource.org/license/bsd-3-clause) | [`BSD-3-Clause`](https://spdx.org/licenses/BSD-3-Clause.html) | [`LICENSES/bsd-3-clause/index.md`](LICENSES/bsd-3-clause/index.md) |
| [GNU General Public License v2.0](https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html) | [`GPL-2.0-only`](https://spdx.org/licenses/GPL-2.0-only.html) | [`LICENSES/gpl-2.0-only/index.md`](LICENSES/gpl-2.0-only/index.md) |
| [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0-standalone.html) | [`GPL-3.0-only`](https://spdx.org/licenses/GPL-3.0-only.html) | [`LICENSES/gpl-3.0-only/index.md`](LICENSES/gpl-3.0-only/index.md) |
| [UK Open Government Licence v3.0](https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/) | [`OGL-UK-3.0`](https://spdx.org/licenses/OGL-UK-3.0.html) | [`LICENSES/ogl-uk-3.0/index.md`](LICENSES/ogl-uk-3.0/index.md) |
| [Creative Commons Attribution Share Alike 4.0 International](https://creativecommons.org/licenses/by-sa/4.0/) | [`CC-BY-SA-4.0`](https://spdx.org/licenses/CC-BY-SA-4.0.html) | [`LICENSES/cc-by-sa-4.0/index.md`](LICENSES/cc-by-sa-4.0/index.md) |

## Why a choice, and why a directory

The permissive licences (MIT, Apache-2.0, BSD-3-Clause) suit organizations that
embed or adapt the code in their own products; Apache-2.0 adds an explicit
patent grant. The GNU licences suit those who want copyleft. The UK Open
Government Licence v3.0 suits public-sector reuse in the UK, and CC BY-SA 4.0
suits reuse, commercial or not, of documentation and other content that is
shared alike (adaptations must carry the same licence; Creative Commons
recommends its licences for content rather than for software). Offering all seven lets each user take the one that fits their own constraints. Custom terms are
for cases none of them fits.

All licence texts live in the [`LICENSES/`](LICENSES) directory, one
`LICENSES/<lowercase-spdx-id>/index.md` per licence in the expression, and
nowhere else in the repository. This file is the only licence-related file
outside it. Forge licence detection usually looks for
a root `LICENSE` file, so it may not recognize this layout; this file and the
[README](README.md#licence) carry the statement instead.

`tests/licence_files.rs` in the API crate keeps the manifests, the expression,
`LICENSES/` and this file consistent.

## Services used only for development and testing

These are not part of this repository and are not distributed with it; they are pulled as container images to
run tests.

| Service | Where | Licence | Use |
|---|---|---|---|
| Redis 8 (`redis:8-alpine`) | `project-portfolio-management-service-with-rust/compose.test.yaml`; CI's `ui` job service container | Redis 8 is offered under a choice of AGPL-3.0-only, RSALv2 or SSPLv1 | development and tests only: the UI's session-store tests against a real Redis, and running the service against Redis by hand. The root `compose.yaml` (the example deployment) still pins Redis 7 (`redis:7-alpine`, BSD-3-Clause); a deployer who moves it to 8 takes on one of the three licences above |

## Full license texts

The full text of each license is in its own directory under `LICENSES/`:

- [MIT](LICENSES/mit/index.md)
- [Apache-2.0](LICENSES/apache-2.0/index.md)
- [BSD-3-Clause](LICENSES/bsd-3-clause/index.md)
- [GPL-2.0-only](LICENSES/gpl-2.0-only/index.md)
- [GPL-3.0-only](LICENSES/gpl-3.0-only/index.md)
- [OGL-UK-3.0](LICENSES/ogl-uk-3.0/index.md)
- [CC-BY-SA-4.0](LICENSES/cc-by-sa-4.0/index.md)
