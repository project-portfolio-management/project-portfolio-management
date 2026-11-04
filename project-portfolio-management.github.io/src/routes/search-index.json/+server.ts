import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";
import { searchIndex } from "#lib/server/content.js";

// A static file in the build: what the search page reads in the browser.
export const prerender = true;
export const GET: RequestHandler = () => json(searchIndex());
