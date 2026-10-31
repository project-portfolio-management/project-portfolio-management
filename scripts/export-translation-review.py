#!/usr/bin/env python3
"""Write governance/translation-reviews/<locale>.csv: every English text next to its translation, for
the speaker or translator who reviews that locale (WW-1.1). Columns: key, english, translation,
reviewed_by, reviewed_on, verdict. Welsh keeps its own script and file
(`scripts/export-welsh-review.py`, `governance/welsh-translation-review.csv`).

Reads the English and the locale's page catalogue (`i18n-pages.ts`, `i18n-pages.<lang>.ts`) and the
`en-001` and `<locale>` blocks of `i18n.svelte.ts`. Re-running keeps the review columns already filled
in for a key whose English and translation are unchanged, and clears them for one that changed.

    python3 scripts/export-translation-review.py           # every locale below
    python3 scripts/export-translation-review.py de-001    # one
    python3 scripts/export-translation-review.py --check   # fail if a file is out of date
"""
import csv, io, json, re, sys
from pathlib import Path

root = Path(__file__).resolve().parent.parent
ui = root / "project-portfolio-management-ui-with-svelte" / "src" / "lib"
outdir = root / "governance" / "translation-reviews"
# A value is double-quoted, or single-quoted where Prettier chose that because the text holds a double quote.
PAIR = re.compile(r'^\s*"([^"]+)":\s*("(?:[^"\\]|\\.)*"|\'(?:[^\'\\]|\\.)*\')\s*,?\s*$', re.M)
# Locale -> its page catalogue file. A regional locale shares its language's page catalogue.
PAGES = {
    "ar-001": "ar", "de-001": "de", "de-de": "de", "es-001": "es",
    "fr-001": "fr", "hi-001": "hi", "zh-001": "zh", "zh-cn": "zh",
}
FIELDS = ["key", "english", "translation", "reviewed_by", "reviewed_on", "verdict"]


def literal(token):
    if token.startswith("'"):
        inner = token[1:-1].replace("\\'", "'").replace('"', '\\"')
        return json.loads(f'"{inner}"')
    return json.loads(token)


def pairs(text):
    return {m.group(1): literal(m.group(2)) for m in PAIR.finditer(text)}


def block(source, locale):
    start = source.index(f'  "{locale}": {{')
    end = source.index("\n  },", start)
    return pairs(source[start:end])


def render(locale, main, en):
    tr = {**block(main, locale), **pairs((ui / f"i18n-pages.{PAGES[locale]}.ts").read_text())}
    out = outdir / f"{locale}.csv"
    previous = {}
    if out.exists():
        for row in csv.DictReader(out.open(newline="", encoding="utf-8")):
            previous[row["key"]] = row
    rows = []
    for key in en:
        old = previous.get(key)
        same = old and old["english"] == en[key] and old["translation"] == tr.get(key, "")
        rows.append({
            "key": key, "english": en[key], "translation": tr.get(key, ""),
            "reviewed_by": old["reviewed_by"] if same else "",
            "reviewed_on": old["reviewed_on"] if same else "",
            "verdict": old["verdict"] if same else "",
        })
    buf = io.StringIO(newline="")
    w = csv.DictWriter(buf, fieldnames=FIELDS, lineterminator="\n")
    w.writeheader(); w.writerows(rows)
    missing = [k for k in en if k not in tr]
    return out, buf.getvalue(), rows, missing


def main():
    args = [a for a in sys.argv[1:] if a != "--check"]
    check = "--check" in sys.argv[1:]
    locales = args or list(PAGES)
    source = (ui / "i18n.svelte.ts").read_text()
    en = {**block(source, "en-001"), **pairs((ui / "i18n-pages.ts").read_text())}
    outdir.mkdir(exist_ok=True)
    status = 0
    for locale in locales:
        out, text, rows, missing = render(locale, source, en)
        reviewed = sum(1 for r in rows if r["verdict"])
        if check:
            if not out.exists() or out.read_text(encoding="utf-8") != text:
                print(f"{out.relative_to(root)} is out of date: run python3 scripts/export-translation-review.py")
                status = 1
        else:
            out.write_text(text, encoding="utf-8")
        print(f"{locale}: {len(rows)} texts, {reviewed} reviewed, {len(missing)} without a translation")
        if missing:
            status = 1
    sys.exit(status)


main()
