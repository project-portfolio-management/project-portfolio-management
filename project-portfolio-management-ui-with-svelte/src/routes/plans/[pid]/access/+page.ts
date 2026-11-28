// The plan access page's tab title, set server-free from the catalogue; it follows the
// `page.data.title` convention (see `../../../+layout.svelte`).

import type { PageLoad } from "./$types";
import { pageTitle } from "#lib/i18n.svelte.js";

export const load: PageLoad = ({ url }) => {
  return { title: pageTitle(url, "p.plan_access.title_ppm") };
};
