import { defineParams } from "@sveltejs/kit/params";
import { COMPARE_PRODUCTS, DOC_PAGES, HOME_LOCALES, TOPIC_KEYS, isBriefLocale, isRole, topicSlug } from "#lib/site.ts";

// Route parameters: [slug=doc] is one of the documents (not the home page), [locale=locale] a role-brief locale and
// [role=role] one of the roles,
// [locale=home] a locale with a home page, [topic=topic] a topic's slug in any locale and [product=product] a compared product.
export const params = defineParams({
  doc: (p: string) => (DOC_PAGES.some((d) => d.slug !== "" && d.slug === p) ? p : undefined),
  locale: (p: string) => (isBriefLocale(p) ? p : undefined),
  role: (p: string) => (isRole(p) ? p : undefined),
  home: (p: string) => ((HOME_LOCALES as readonly string[]).includes(p) ? p : undefined),
  topic: (p: string) => (HOME_LOCALES.some((l) => TOPIC_KEYS.some((k) => topicSlug(l, k) === p)) ? p : undefined),
  product: (p: string) => (COMPARE_PRODUCTS.includes(p) ? p : undefined),
});
