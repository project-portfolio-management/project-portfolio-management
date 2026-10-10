#!/usr/bin/env python3
"""Write governance/welsh-translation-review.csv: every English text next to its Welsh, for the
professional translator (HD-13). Columns: key, english, welsh, reviewed_by, reviewed_on, verdict.

Reads the two page catalogues (`i18n-pages.ts`, `i18n-pages.cy.ts`) and the `en-001` / `cy-001`
blocks of `i18n.svelte.ts`. Re-running keeps any review columns already filled in for a key whose
English and Welsh are unchanged.
"""
import csv, json, re, sys
from pathlib import Path

ui = Path(__file__).resolve().parent.parent / "project-portfolio-management-ui-with-svelte" / "src" / "lib"
out = Path(__file__).resolve().parent.parent / "governance" / "welsh-translation-review.csv"
PAIR = re.compile(r'^\s*"([^"]+)":\s*("(?:[^"\\]|\\.)*")\s*,?\s*$', re.M)

def pairs(text):
    return {m.group(1): json.loads(m.group(2)) for m in PAIR.finditer(text)}

def block(source, locale):
    start = source.index(f'  "{locale}": {{')
    end = source.index("\n  },", start)
    return pairs(source[start:end])

main = (ui / "i18n.svelte.ts").read_text()
en = {**block(main, "en-001"), **pairs((ui / "i18n-pages.ts").read_text())}
cy = {**block(main, "cy-001"), **pairs((ui / "i18n-pages.cy.ts").read_text())}

previous = {}
if out.exists():
    for row in csv.DictReader(out.open(newline="", encoding="utf-8")):
        previous[row["key"]] = row

rows = []
for key in en:
    old = previous.get(key)
    same = old and old["english"] == en[key] and old["welsh"] == cy.get(key, "")
    rows.append({
        "key": key, "english": en[key], "welsh": cy.get(key, ""),
        "reviewed_by": old["reviewed_by"] if same else "",
        "reviewed_on": old["reviewed_on"] if same else "",
        "verdict": old["verdict"] if same else "",
    })
with out.open("w", newline="", encoding="utf-8") as f:
    w = csv.DictWriter(f, fieldnames=["key", "english", "welsh", "reviewed_by", "reviewed_on", "verdict"])
    w.writeheader(); w.writerows(rows)
missing = [k for k in en if k not in cy]
reviewed = sum(1 for r in rows if r["verdict"])
print(f"{len(rows)} texts, {reviewed} reviewed, {len(missing)} without Welsh")
sys.exit(1 if missing else 0)
