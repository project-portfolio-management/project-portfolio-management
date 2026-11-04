import type { RequestHandler } from "./$types";
import { allRoutes, sitemap } from "#lib/server/content.js";

export const prerender = true;
export const GET: RequestHandler = () => sitemap(allRoutes());
