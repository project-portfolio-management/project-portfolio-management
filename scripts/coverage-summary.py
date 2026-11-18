#!/usr/bin/env python3
"""Summarize a cargo-tarpaulin JSON report by crate, and name the least-covered files.

    python3 scripts/coverage-summary.py [REPORT] [--least N] [--min-lines M] [--crate NAME] [--markdown]

REPORT defaults to project-portfolio-management-service-with-rust/target/coverage/tarpaulin-report.json, which
`make coverage` (tarpaulin.toml) and CI's `coverage` job (tarpaulin-db.toml) write. The crates are told apart by
directory: `domain/src` (the domain crate), `datetime/src`, `migration/src`, and the service's own `src`.
`--least N` lists the N files with the lowest line coverage (of those with at least M coverable lines, default 20),
for every crate or for `--crate`. `--markdown` writes tables instead of plain text (CI appends them to the job
summary). It only reads the report; it never fails on a coverage figure (COVERAGE.md: no threshold yet).
"""

import argparse
import json
import sys
from pathlib import Path

SERVICE = "project-portfolio-management-service-with-rust"
CRATES = [
    ("domain", "domain/src/"),
    ("datetime", "datetime/src/"),
    ("migration", "migration/src/"),
    ("service", "src/"),
]


def relative(path):
    """The file's path relative to the service directory (tarpaulin writes absolute paths, as a list or a string)."""
    text = "/".join(path) if isinstance(path, list) else str(path)
    text = text.replace("//", "/")
    marker = SERVICE + "/"
    return text.split(marker, 1)[1] if marker in text else text


def crate_of(rel):
    for name, prefix in CRATES:
        if rel.startswith(prefix):
            return name
    return "other"


def pct(covered, coverable):
    return 100.0 * covered / coverable if coverable else 0.0


def main():
    default = Path(__file__).resolve().parent.parent / SERVICE / "target/coverage/tarpaulin-report.json"
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("report", nargs="?", default=str(default))
    ap.add_argument("--least", type=int, default=0)
    ap.add_argument("--min-lines", type=int, default=20)
    ap.add_argument("--crate", choices=[c for c, _ in CRATES] + ["other"])
    ap.add_argument("--markdown", action="store_true")
    args = ap.parse_args()

    try:
        report = json.loads(Path(args.report).read_text())
    except (OSError, ValueError) as e:
        print(f"coverage-summary: cannot read {args.report}: {e}", file=sys.stderr)
        return 2

    files = []
    for f in report.get("files", []):
        rel = relative(f["path"])
        files.append((rel, crate_of(rel), int(f.get("covered", 0)), int(f.get("coverable", 0))))

    totals = {}
    for _, crate, cov, able in files:
        c, a, n = totals.get(crate, (0, 0, 0))
        totals[crate] = (c + cov, a + able, n + 1)
    all_cov = sum(c for c, _, _ in totals.values())
    all_able = sum(a for _, a, _ in totals.values())

    order = [c for c, _ in CRATES] + ["other"]
    if args.markdown:
        print("| Crate | Files | Lines covered | Coverable | Coverage |")
        print("|---|---|---|---|---|")
        for crate in order:
            if crate in totals:
                c, a, n = totals[crate]
                print(f"| {crate} | {n} | {c} | {a} | {pct(c, a):.1f}% |")
        print(f"| **all** | {len(files)} | {all_cov} | {all_able} | **{pct(all_cov, all_able):.1f}%** |")
    else:
        for crate in order:
            if crate in totals:
                c, a, n = totals[crate]
                print(f"{crate:<10} {pct(c, a):6.2f}%  {c:>6} / {a:<6} lines in {n} files")
        print(f"{'all':<10} {pct(all_cov, all_able):6.2f}%  {all_cov:>6} / {all_able:<6} lines in {len(files)} files")

    if args.least:
        chosen = [f for f in files if f[3] >= args.min_lines and (args.crate is None or f[1] == args.crate)]
        chosen.sort(key=lambda f: (pct(f[2], f[3]), -f[3], f[0]))
        print()
        if args.markdown:
            print(f"Least covered ({args.least}, at least {args.min_lines} coverable lines):")
            print()
            print("| File | Covered | Coverable | Coverage |")
            print("|---|---|---|---|")
            for rel, _, c, a in chosen[: args.least]:
                print(f"| `{rel}` | {c} | {a} | {pct(c, a):.1f}% |")
        else:
            for rel, _, c, a in chosen[: args.least]:
                print(f"{pct(c, a):6.2f}%  {c:>5} / {a:<5}  {rel}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
