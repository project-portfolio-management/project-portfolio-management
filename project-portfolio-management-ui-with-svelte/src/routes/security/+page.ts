// `page.data.title` convention (see `../+layout.svelte`): mirrors this
// route's own <svelte:head><title> so SharePicker gets the right title
// without reading the DOM.

import type { PageLoad } from "./$types";
import { pageTitle } from "#lib/i18n.svelte.js";

export const load: PageLoad = ({ url }) => {
  return { title: pageTitle(url, "p.security.security_ppm") };
};
