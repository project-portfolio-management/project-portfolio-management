import { error } from "@sveltejs/kit";
import type { EntryGenerator, PageServerLoad } from "./$types";
import { HOME_LOCALES, TOPIC_KEYS, topicKeyOf, topicSlug, type BriefLocale } from "#lib/site.js";
import { render } from "#lib/server/content.js";
import { href } from "#lib/href.js";

export const entries: EntryGenerator = () =>
  HOME_LOCALES.flatMap((locale) => TOPIC_KEYS.map((key) => ({ locale, topic: topicSlug(locale, key) })));

export const load: PageServerLoad = ({ params }) => {
  // The matcher takes a slug from any locale: it must be this locale's.
  const locale = params.locale as BriefLocale;
  const topic = topicKeyOf(locale, params.topic);
  if (!topic) error(404, "Not found");
  const path = `locales/${locale}/topics/${params.topic}/index.md`;
  const { title, html } = render(path, href);
  return { title, html, path, locale, topic };
};
