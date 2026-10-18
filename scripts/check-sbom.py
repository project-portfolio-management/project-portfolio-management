#!/usr/bin/env python3
"""Check a CycloneDX SBOM against the lock file it was made from.

    scripts/check-sbom.py sbom.cdx.json Cargo.lock
    scripts/check-sbom.py sbom.cdx.json pnpm-lock.yaml

Fails (exit 1) when the SBOM is empty or lists a package, at a version, that
the lock file does not contain. It reports how many lock-file packages the SBOM
leaves out: development-only, build-host-only and other-platform packages are
not in a shipped SBOM, so a lock file always has more packages than the SBOM and
the left-out count is information, not a failure. A package missing from the
SBOM that SHOULD be there is not caught by this check.
"""
import json
import re
import sys


def lock_packages(path: str) -> set[tuple[str, str]]:
    text = open(path, encoding="utf-8").read()
    if path.endswith("Cargo.lock"):
        return {(m.group(1), m.group(2)) for m in re.finditer(r'\[\[package\]\]\nname = "([^"]+)"\nversion = "([^"]+)"', text)}
    # pnpm-lock.yaml: keys like `name@version:` or `'@scope/name@version':` (optionally with a peer suffix)
    found = set()
    for m in re.finditer(r"^\s{2}'?((?:@[^/\s'@]+/)?[^@\s'(]+)@([^\s':(]+)[^\n]*:\s*$", text, re.M):
        found.add((m.group(1), m.group(2)))
    return found


def sbom_components(path: str) -> set[tuple[str, str]]:
    doc = json.load(open(path, encoding="utf-8"))
    out = set()
    for c in doc.get("components", []):
        name, version = c.get("name"), c.get("version")
        if c.get("group"):
            name = f'{c["group"]}/{name}' if not str(name).startswith(c["group"]) else name
        if name and version:
            out.add((name, version))
    return out


def main() -> int:
    if len(sys.argv) != 3:
        print(__doc__)
        return 2
    sbom, lock = sys.argv[1], sys.argv[2]
    have, want = sbom_components(sbom), lock_packages(lock)
    if not have:
        print(f"FAIL: {sbom} lists no components")
        return 1
    if not want:
        print(f"FAIL: found no packages in {lock}")
        return 1
    unknown = sorted(have - want)
    print(f"{sbom}: {len(have)} components, {len(want)} packages in {lock}, {len(want - have)} in the lock file only")
    if unknown:
        print(f"FAIL: {len(unknown)} SBOM components are not in {lock}, e.g. {unknown[:5]}")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
