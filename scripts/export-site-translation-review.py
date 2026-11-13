#!/usr/bin/env python3
"""Write governance/translation-reviews/site-<language>.csv: the public site's words and pages in each translated language (for
German, Spanish and French, the path to production alone), for
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
LANGUAGES = {"ar": "ar-001", "cy": "cy-gb", "hi": "hi-in", "zh": "zh-cn", "de": "de-de", "es": "es-001", "fr": "fr-001"}


def peer(path):
    return (path / ".locale-peer-id").read_text(encoding="utf-8")


def stamped(path):
    digest = hashlib.sha256((path / "index.md").read_bytes()).hexdigest()[:12]
    return f"{(path / 'index.md').relative_to(root)} ({digest})"


def page_rows(locale):
    base = root / "locales" / "en-001"
    here = root / "locales" / locale
    english = {peer(p): p for p in (base / "topics").iterdir()}
    # German, Spanish and French have only the path to production: no home page or topics.
    rows = [("page:home", stamped(base), stamped(here))] if (here / "index.md").exists() else []
    for path in sorted((here / "topics").iterdir()) if (here / "topics").is_dir() else []:
        rows.append((f"page:{english[peer(path)].name}", stamped(english[peer(path)]), stamped(path)))
    # The path to production has the same slug in every locale, so it needs no peer id.
    if (here / "path-to-production").is_dir():
        rows.append(("page:path-to-production", stamped(base / "path-to-production"), stamped(here / "path-to-production")))
    return rows


def render(lang, locale, out):
    previous = {}
    if out.exists():
        for row in csv.DictReader(out.open(newline="", encoding="utf-8")):
            previous[row["key"]] = row
    # The site's own words exist only in its five languages; a page-only language reviews its page alone.
    words = [(k, v, messages[lang][k]) for k, v in messages["en"].items()] if lang in messages else []
    source = words + page_rows(locale)
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
