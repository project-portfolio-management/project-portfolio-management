import type { PageServerLoad } from "./$types";
import { render } from "#lib/server/content.js";
import { href } from "#lib/href.js";

export const load: PageServerLoad = () => {
  const { title, html } = render("index.md", href);
  return { title, html, path: "index.md" };
};
