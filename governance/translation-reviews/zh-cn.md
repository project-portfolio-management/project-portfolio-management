# Review record: `zh-cn`

Chinese (Simplified, China). One record per locale (WW-1.1); Welsh keeps its own record,
[`../welsh-translation-review.md`](../welsh-translation-review.md).

| Item | Value |
| --- | --- |
| Status | **draft: machine-written by Claude (the AI assistant), not reviewed** by a speaker or a professional translator |
| Page and component text | `project-portfolio-management-ui-with-svelte/src/lib/i18n-pages.zh.ts` |
| Navigation, forms and other interface text | the `zh-cn` block of `project-portfolio-management-ui-with-svelte/src/lib/i18n.svelte.ts` |
| Widget labels | the `@lilydesignsystem/svelte-locale-translations` pack for this language (WW-1.17; not reviewed either) |
| Working file | [`zh-cn.csv`](./zh-cn.csv) (`python3 scripts/export-translation-review.py zh-cn`) |
| Texts reviewed | 0 |
| Reviewer | none |
| Reviewed on | never |

## How a review is done

1. Run the export (above); each row is `key`, `english`, `translation` and three columns for the reviewer:
   `reviewed_by`, `reviewed_on`, `verdict` (`accepted`, `changed` or `rejected`).
2. Correct the `translation` column where needed and fill in the three review columns. Placeholders such as
   `{count}` or `{name}` must be kept exactly: `tests/unit/catalogue-coverage.test.ts` fails if one is lost.
3. A maintainer copies each changed value back to its catalogue (the page file for keys beginning `p.` and `c.`,
   the locale's block for the others), runs the tests, and re-runs the export, which keeps the review columns of
   unchanged rows.
4. When every row has a verdict, update the table above and add a row to the log below; only then change the
   "machine draft" notice at the top of the page file and the [language statement](../language-statement.md).
   Read short labels in place, in the running interface under `/zh-cn/…`, not only in the file.

## Known weak spots

- Shares the page catalogue of `zh-001` (`i18n-pages.zh.ts`); only the `zh-cn` block of `i18n.svelte.ts` is its own.

## Review log

No review yet.

| Date | Reviewer | Scope | Outcome |
| --- | --- | --- | --- |
