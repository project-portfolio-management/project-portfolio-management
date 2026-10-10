import type { PageLoad } from "./$types";
import { pageTitle } from "#lib/i18n.svelte.js";

export const load: PageLoad = ({ url }) => {
  return { title: pageTitle(url, "p.plans.pid.work.work_and_assignments_ppm") };
};
