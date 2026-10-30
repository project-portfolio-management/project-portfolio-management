import type { EntryGenerator, PageServerLoad } from "./$types";
import { BRIEF_LOCALES, ROLES, type BriefLocale } from "#lib/site.js";
import { render } from "#lib/server/content.js";
import { href } from "#lib/href.js";

export const entries: EntryGenerator = () =>
  BRIEF_LOCALES.flatMap((locale) => ROLES.map((role) => ({ locale, role })));

export const load: PageServerLoad = ({ params }) => {
  const locale = params.locale as BriefLocale;
  const path = `locales/${locale}/for/${params.role}/index.md`;
  const { title, html } = render(path, href);
  return { title, html, path, locale, role: params.role };
};
