# Translation review records

One record per supported locale other than English (WW-1.1). Every translation in the interface was written by
Claude (the AI assistant) and **none has been reviewed** by a speaker or a professional translator.

| Locale | Language | Record | Working file | Reviewed |
| --- | --- | --- | --- | --- |
| `ar-001` | Arabic | [ar-001.md](ar-001.md) | [ar-001.csv](ar-001.csv) | no |
| `cy-001` | Welsh | [welsh-translation-review.md](../welsh-translation-review.md) | [welsh-translation-review.csv](../welsh-translation-review.csv) | no |
| `de-001` | German | [de-001.md](de-001.md) | [de-001.csv](de-001.csv) | no |
| `de-de` | German (Germany) | [de-de.md](de-de.md) | [de-de.csv](de-de.csv) | no |
| `es-001` | Spanish | [es-001.md](es-001.md) | [es-001.csv](es-001.csv) | no |
| `fr-001` | French | [fr-001.md](fr-001.md) | [fr-001.csv](fr-001.csv) | no |
| `hi-001` | Hindi | [hi-001.md](hi-001.md) | [hi-001.csv](hi-001.csv) | no |
| `zh-001` | Chinese (Simplified) | [zh-001.md](zh-001.md) | [zh-001.csv](zh-001.csv) | no |
| `zh-cn` | Chinese (Simplified, China) | [zh-cn.md](zh-cn.md) | [zh-cn.csv](zh-cn.csv) | no |

**Widget labels (WW-1.17, a stated limit).** For every locale but English and Welsh, the words inside Lily's widgets
come from a pack of `@lilydesignsystem/svelte-locale-translations` 0.4.0 that ships marked `"reviewed": false`; each
record above names its pack and says it is not reviewed. They are used as they are rather than falling back to English.

`project-portfolio-management-ui-with-svelte/tests/unit/review-records.test.ts` fails when a supported locale has no
record, or when a record claims a review its log does not show. `python3 scripts/export-translation-review.py --check`
fails when a working file is out of date with the catalogues.
