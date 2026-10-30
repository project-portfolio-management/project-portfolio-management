import type { EntryGenerator, PageServerLoad } from "./$types";
import { BRIEF_LOCALES, ROLES, type BriefLocale } from "#lib/site.js";
import { briefSummary } from "#lib/server/content.js";

export const entries: EntryGenerator = () => BRIEF_LOCALES.map((locale) => ({ locale }));

export const load: PageServerLoad = ({ params }) => {
  const locale = params.locale as BriefLocale;
  const briefs = ROLES.map((role) => ({ role, ...briefSummary(`locales/${locale}/for/${role}/index.md`) }));
  return { title: "Role briefs", locale, briefs };
};
