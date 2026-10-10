# Welsh translation review record

The Welsh text of the interface was **drafted, not professionally translated**. This file is
where a professional Welsh translator's review is recorded. Until the status below says
otherwise, treat the Welsh as a draft and do not describe it as professionally reviewed.

## Status

| | |
|---|---|
| **State** | **Draft: not yet reviewed by a professional translator** |
| Texts in scope | 2,691 (every interface text, English and Welsh) |
| Texts reviewed | 0 |
| Reviewer | ‹buyer or supplier to appoint› |
| Review completed on | — |
| Result | — |

## How the review is done

1. Generate the working file: `python3 scripts/export-welsh-review.py` writes
   [`welsh-translation-review.csv`](welsh-translation-review.csv) with one row per text:
   `key`, `english`, `welsh`, and three columns for the translator: `reviewed_by`,
   `reviewed_on`, `verdict` (`accepted`, `changed` or `rejected`).
2. The translator corrects the `welsh` column where needed and fills in the three review
   columns. Placeholders such as `{count}` or `{name}` must be kept exactly: the CI test
   fails if one is lost.
3. A maintainer copies each changed `welsh` value back to its catalogue
   (`src/lib/i18n-pages.cy.ts` for keys beginning `p.`, `c.`, `l.`; the `cy-001` block of
   `src/lib/i18n.svelte.ts` for the others) and runs the tests.
4. Re-running the export keeps the review columns for every text whose English and Welsh
   are unchanged, and clears them for a text that has changed since it was reviewed.
5. When every row has a verdict, update the table above: state, counts, reviewer's name and
   organization, date. Only then change the "draft" notice in `src/lib/i18n-pages.cy.ts` and
   in the [language statement](language-statement.md).

## Known points for the translator

- Compound sentences are built from several texts around a number or a name, so mutation
  (treiglad) and agreement after a count may need a rewording of the whole sentence. A
  plural is two texts (`…_one`, `…_other`); Welsh has more plural forms than that. Where a
  noun after a number needs its own form, say so in a comment and the key will be split.
- Business terms such as "float", "baseline", "earned value" and "critical path" have been
  given plain Welsh equivalents; replace them with the terms your organization already
  uses.
- Short labels (a button, a column heading) are translated without their page, so check them
  in place: run the app and open `/cy-001/…`.

## History

| Date | Event |
|---|---|
| 2026-10-10 | Welsh drafted for every text in the catalogue (HD-13). No professional review yet. |
