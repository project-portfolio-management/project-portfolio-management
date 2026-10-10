// `page.data.title` convention (see `../+layout.svelte`): mirrors this
// route's own <svelte:head><title> so SharePicker gets the right title.
import type { PageServerLoad } from "./$types";
import { pageTitle } from "#lib/i18n.svelte.js";

export const load: PageServerLoad = ({ url }) => {
  return { title: pageTitle(url, "p.tour.tour_main_x_plans") };
};
