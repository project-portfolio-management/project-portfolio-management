#!/usr/bin/env python3
"""Write governance/translation-reviews/site-<language>.csv: the public site's words and pages in Welsh and Mandarin, for
the speaker who reviews them (GP-4). Columns: key, english, translation, reviewed_by, reviewed_on, verdict.

Reads project-portfolio-management.github.io/src/lib/messages.json, then adds one row per page: its English and
translated paths, with a hash of each, so a page that changes loses its review. Re-running keeps the review columns for a
row whose English and translation are unchanged.

    python3 scripts/export-site-translation-review.py           # write
    python3 scripts/export-site-translation-review.py --check   # fail if a file is out of date
"""
import csv, hashlib, io, json, sys
from pathlib import Path

root = Path(__file__).resolve().parent.parent
outdir = root / "governance" / "translation-reviews"
messages = json.loads((root / "project-portfolio-management.github.io/src/lib/messages.json").read_text(encoding="utf-8"))
FIELDS = ["key", "english", "translation", "reviewed_by", "reviewed_on", "verdict"]
# The locale reviewed for each language: its -001 copy is byte-identical, and the site's tests check that.
LANGUAGES = {"cy": "cy-gb", "zh": "zh-cn"}


def peer(path):
    return (path / ".locale-peer-id").read_text(encoding="utf-8")


def stamped(path):
    digest = hashlib.sha256((path / "index.md").read_bytes()).hexdigest()[:12]
    return f"{(path / 'index.md').relative_to(root)} ({digest})"


def page_rows(locale):
    base = root / "locales" / "en-001"
    here = root / "locales" / locale
    english = {peer(p): p for p in (base / "topics").iterdir()}
    rows = [("page:home", stamped(base), stamped(here))]
    for path in sorted((here / "topics").iterdir()):
        rows.append((f"page:{english[peer(path)].name}", stamped(english[peer(path)]), stamped(path)))
    return rows


def render(lang, locale, out):
    previous = {}
    if out.exists():
        for row in csv.DictReader(out.open(newline="", encoding="utf-8")):
            previous[row["key"]] = row
    source = [(k, v, messages[lang][k]) for k, v in messages["en"].items()] + page_rows(locale)
    buffer = io.StringIO()
    writer = csv.DictWriter(buffer, fieldnames=FIELDS, lineterminator="\n")
    writer.writeheader()
    for key, english, translation in source:
        old = previous.get(key)
        same = old and old["english"] == english and old["translation"] == translation
        writer.writerow({
            "key": key, "english": english, "translation": translation,
            "reviewed_by": old["reviewed_by"] if same else "",
            "reviewed_on": old["reviewed_on"] if same else "",
            "verdict": old["verdict"] if same else "",
        })
    return buffer.getvalue()


check = "--check" in sys.argv
stale = []
for lang, locale in LANGUAGES.items():
    out = outdir / f"site-{lang}.csv"
    text = render(lang, locale, out)
    if check:
        if not out.exists() or out.read_text(encoding="utf-8") != text:
            stale.append(out.name)
    else:
        out.write_text(text, encoding="utf-8")
if stale:
    print(f"stale: {', '.join(stale)}: run python3 scripts/export-site-translation-review.py")
    sys.exit(1)
print("site translation review files are current" if check else "wrote the site translation review files")
