// `page.data.title` convention (see `../+layout.svelte`).

import type { PageLoad } from "./$types";

export const load: PageLoad = () => {
  return { title: "Milestones — PPM" };
};
