#!/usr/bin/env python3
"""Render llms.txt from llms.json (the one source) and check both against the tree.

    scripts/generate-llms.py          rewrite llms.txt
    scripts/generate-llms.py --check  exit 1 if llms.txt is stale or a listed path does not exist

llms.json holds the project name, summary, a few facts and the sections of links. llms.txt follows the
llms.txt convention: an H1, a blockquote summary, then H2 sections of `- [title](path): description`.
"""
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent


def render(d: dict) -> str:
    out = [f"# {d['name']}", "", f"> {d['summary']}", ""]
    facts = d.get("facts")
    if facts:
        out += ["## Facts", ""]
        out.append(f"- Version: {facts['version']}")
        out.append("- Projects: " + ", ".join(f"`{v}` ({k})" for k, v in facts["layout"].items()))
        out += [f"- Stack: {s}" for s in facts["stack"]]
        out += [f"- Run: `{c}`" for c in facts["run"]]
        out += [f"- Check: `{c}`" for c in facts["check"]]
        out += [f"- Rule: {r}" for r in facts["rules"]]
        out.append("")
    for sec in d["sections"]:
        out += [f"## {sec['title']}", ""]
        out += [f"- [{l['title']}]({l['path']}): {l['description']}" for l in sec["links"]]
        out.append("")
    return "\n".join(out).rstrip("\n") + "\n"


def main() -> int:
    d = json.loads((ROOT / "llms.json").read_text(encoding="utf-8"))
    missing = [l["path"] for s in d["sections"] for l in s["links"] if not (ROOT / l["path"]).exists()]
    text = render(d)
    target = ROOT / "llms.txt"
    if "--check" in sys.argv:
        bad = False
        if missing:
            print("listed paths that do not exist:", *missing, sep="\n  ")
            bad = True
        if not target.exists() or target.read_text(encoding="utf-8") != text:
            print("llms.txt is stale: run scripts/generate-llms.py")
            bad = True
        return 1 if bad else 0
    if missing:
        print("listed paths that do not exist:", *missing, sep="\n  ")
        return 1
    target.write_text(text, encoding="utf-8")
    print(f"wrote llms.txt ({len(text)} bytes) from llms.json")
    return 0


if __name__ == "__main__":
    sys.exit(main())
