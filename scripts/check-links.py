#!/usr/bin/env python3
"""Offline relative-link checker for Markdown (stdlib only).

Resolves every relative inline link `[text](path#fragment)` in tracked-style
Markdown files and reports targets that do not exist, and `#fragment`
anchors that do not match a heading in the target Markdown file.
Equivalent in spirit to `lychee --offline --config lychee.toml .`.
Usage: scripts/check-links.py [root]   (exit 1 if any link is broken)
"""
import os, re, sys, urllib.parse

ROOT = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else ".")
SKIP = {"target", "node_modules", ".svelte-kit", "build", "dist", ".git", ".next", "coverage", "playwright-report", "test-results"}
LINK = re.compile(r"(?<!\!)\[[^\]]*\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)|!\[[^\]]*\]\(([^)\s]+)\)")
FENCE = re.compile(r"^\s*(```|~~~)")

def slug(h):
    h = re.sub(r"`|\*|_(?=\W|$)", "", h.strip().lower())
    h = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", h)
    h = re.sub(r"[^\w\- ]", "", h, flags=re.UNICODE)
    return h.replace(" ", "-")

def anchors(path, cache={}):
    if path not in cache:
        seen, out, fence = {}, set(), False
        for line in open(path, encoding="utf-8", errors="replace"):
            if FENCE.match(line):
                fence = not fence
            if fence:
                continue
            m = re.match(r"^#{1,6}\s+(.*?)\s*#*\s*$", line)
            if m:
                s = slug(m.group(1)); n = seen.get(s, 0); seen[s] = n + 1
                out.add(s if n == 0 else f"{s}-{n}")
            for a in re.findall(r'<a\s+(?:name|id)="([^"]+)"', line):
                out.add(a)
        cache[path] = out
    return cache[path]

bad = 0
for d, dirs, files in os.walk(ROOT):
    dirs[:] = [x for x in dirs if x not in SKIP]
    for f in files:
        if not f.endswith(".md"):
            continue
        p = os.path.join(d, f); fence = False
        for no, line in enumerate(open(p, encoding="utf-8", errors="replace"), 1):
            if FENCE.match(line):
                fence = not fence
            if fence:
                continue
            line = re.sub(r"`[^`]*`", "", line)
            for m in LINK.finditer(line):
                t = urllib.parse.unquote(m.group(1) or m.group(2)).strip("<>")
                if re.match(r"^[a-zA-Z][a-zA-Z0-9+.-]*:", t) or t.startswith("//"):
                    continue
                path, _, frag = t.partition("#")
                tgt = p if not path else os.path.normpath(os.path.join(d if not path.startswith("/") else ROOT, path))
                if not os.path.exists(tgt):
                    print(f"{os.path.relpath(p, ROOT)}:{no}: missing {t}"); bad += 1
                elif frag and tgt.endswith(".md") and frag not in anchors(tgt) and not re.match(r"^L\d+", frag):
                    print(f"{os.path.relpath(p, ROOT)}:{no}: missing anchor {t}"); bad += 1
print(f"{bad} broken link(s)")
sys.exit(1 if bad else 0)
