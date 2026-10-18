// The words Lily's widgets need, in the page's language, from `@lilydesignsystem/svelte-locale-translations`.
//
// English and Welsh keep this app's own wording (`p.table.*` and friends, tested and in the Welsh review
// list). The other languages used to fall back to English for every widget label; they now get the package's
// pack. Every pack but English is a machine draft that no native speaker has read (`pack.reviewed` is false),
// so these labels are unreviewed.
import { labelsFor, type LabelHelper, type LabelPack } from "@lilydesignsystem/svelte-locale-translations";
import type { Locale } from "./locales";

/** Locales whose widget words this app writes itself (see `i18n-pages`). */
const OWN: ReadonlySet<Locale> = new Set(["en-001", "cy-001"]);

/** Our locale → the package's pack. A regional locale uses its language's pack. */
const PACK_OF: Record<Locale, string> = {
  "ar-001": "ar-001",
  "cy-001": "cy-001",
  "de-001": "de-001",
  "de-de": "de-001",
  "en-001": "en-001",
  "es-001": "es-001",
  "fr-001": "fr-001",
  "hi-001": "hi-001",
  "zh-001": "zh-001",
  "zh-cn": "zh-001",
};

// The JSON packs are typed with `direction: string`, not the `"ltr" | "rtl"` of `LabelPack`, so they are cast on use.
const PACKS: Record<string, () => Promise<{ default: unknown }>> = {
  "ar-001": () => import("@lilydesignsystem/svelte-locale-translations/ar-001"),
  "cy-001": () => import("@lilydesignsystem/svelte-locale-translations/cy-001"),
  "de-001": () => import("@lilydesignsystem/svelte-locale-translations/de-001"),
  "en-001": () => import("@lilydesignsystem/svelte-locale-translations/en-001"),
  "es-001": () => import("@lilydesignsystem/svelte-locale-translations/es-001"),
  "fr-001": () => import("@lilydesignsystem/svelte-locale-translations/fr-001"),
  "hi-001": () => import("@lilydesignsystem/svelte-locale-translations/hi-001"),
  "zh-001": () => import("@lilydesignsystem/svelte-locale-translations/zh-001"),
};

/** Whether this app supplies the locale's widget words itself. */
export const hasOwnWidgetWords = (locale: Locale): boolean => OWN.has(locale);

/** The pack's `labels` for a widget in this locale, or `{}` when the app supplies its own words (or the pack cannot load). */
export async function packLabels(helper: LabelHelper, locale: Locale): Promise<Record<string, unknown>> {
  if (hasOwnWidgetWords(locale)) return {};
  try {
    const load = PACKS[PACK_OF[locale]];
    if (!load) return {};
    return labelsFor(helper, (await load()).default as LabelPack);
  } catch {
    return {};
  }
}
