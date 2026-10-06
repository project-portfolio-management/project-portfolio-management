// `GET /robots.txt`: points crawlers at the sitemap.
import type { RequestHandler } from "./$types";
import { buildRobots, cleanOrigin } from "#lib/sitemap.js";

export const GET: RequestHandler = ({ url }) =>
  new Response(
    buildRobots(cleanOrigin(process.env.PUBLIC_SITE_URL ?? "") ?? url.origin),
    {
      headers: {
        "content-type": "text/plain; charset=utf-8",
        "cache-control": "public, max-age=3600",
      },
    },
  );
