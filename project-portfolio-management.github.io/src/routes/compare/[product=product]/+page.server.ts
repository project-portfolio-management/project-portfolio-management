import type { EntryGenerator, PageServerLoad } from "./$types";
import { COMPARE_PRODUCTS } from "#lib/site.js";
import { render } from "#lib/server/content.js";
import { href } from "#lib/href.js";

export const entries: EntryGenerator = () => COMPARE_PRODUCTS.map((product) => ({ product }));

export const load: PageServerLoad = ({ params }) => {
  const path = `locales/en-001/compare/${params.product}/index.md`;
  const { title, html } = render(path, href);
  return { title, html, path };
};
