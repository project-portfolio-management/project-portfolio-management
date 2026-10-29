// `page.data.title` convention (see `../../../+layout.svelte`).

import type { PageLoad } from "./$types";
import { pageTitle } from "#lib/i18n.svelte.js";

export const load: PageLoad = ({ url }) => {
  return { title: pageTitle(url, "p.plan_issues.title_ppm") };
};
