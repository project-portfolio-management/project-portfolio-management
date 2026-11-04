import type { EntryGenerator, RequestHandler } from "./$types";
import { allRoutes, sitemap } from "#lib/server/content.js";
import { BRIEF_LOCALES } from "#lib/site.js";

export const prerender = true;
export const entries: EntryGenerator = () => BRIEF_LOCALES.map((locale) => ({ locale }));
export const GET: RequestHandler = ({ params }) =>
  sitemap(allRoutes().filter((r) => r.startsWith(`/${params.locale}/`)));
