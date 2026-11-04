import type { PageServerLoad } from "./$types";
import { home } from "#lib/server/content.js";
import { href } from "#lib/href.js";
import { DEFAULT_LOCALE } from "#lib/site.js";

// The canonical home page: the default locale's, with no locale in the address.
export const load: PageServerLoad = () => {
  const content = home(DEFAULT_LOCALE, href);
  return { title: content.headline, home: content };
};
