// `GET /sitemap.xml`, generated from the route files at build time. The origin is
// `PUBLIC_SITE_URL` when set (so a proxy's internal host never leaks), else the
// request's own.
import type { RequestHandler } from "./$types";
import { buildSitemap, cleanOrigin, pagePaths } from "#lib/sitemap.js";

const FILES = Object.keys(import.meta.glob("/src/routes/**/+page.svelte"));

export const prerender = false;

export const GET: RequestHandler = ({ url }) => {
  const origin = cleanOrigin(process.env.PUBLIC_SITE_URL ?? "") ?? url.origin;
  return new Response(buildSitemap(origin, pagePaths(FILES)), {
    headers: {
      "content-type": "application/xml; charset=utf-8",
      "cache-control": "public, max-age=3600",
    },
  });
};
