import type { PageServerLoad } from "./$types";
import { briefSummary } from "#lib/server/content.js";
import { COMPARE_PRODUCTS } from "#lib/site.js";

export const load: PageServerLoad = () => ({
  title: "Compare",
  products: COMPARE_PRODUCTS.map((slug) => ({ slug, ...briefSummary(`locales/en-001/compare/${slug}/index.md`) })),
});
