// See https://svelte.dev/docs/kit/types#app
declare global {
  namespace App {
    interface PageData {
      /** The page title, also used in <title>. */
      title?: string;
      /** The locale of the page, when it has one: a brief locale, or (on the path to production) a page-only one. */
      locale?: import("#lib/site.js").PathLocale;
      /** True on a path-to-production page. */
      pathToProduction?: boolean;
    }
  }
}

export {};
