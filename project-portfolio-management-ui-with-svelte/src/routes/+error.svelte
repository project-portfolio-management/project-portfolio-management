<!--
  The page shown when a route fails to load or throws (a 404, a 500). SvelteKit's built-in page is
  English only, so this one speaks the reader's language: the status, a sentence for it, and a way
  back (HD-13).
-->
<script lang="ts">
  import { page } from "$app/state";
  import { t, localeHref } from "#lib/i18n.svelte.js";

  const status = $derived(page.status);
  const sentence = $derived(
    status === 404 ? t("p.error.not_found") : status === 401 || status === 403 ? t("p.error.not_allowed") : t("p.error.went_wrong"),
  );
</script>

<svelte:head><title>{t("p.error.title", { status })}</title></svelte:head>

<div class="stack">
  <h1>{status}</h1>
  <p role="alert">{sentence}</p>
  <p><a href={localeHref("/")}>{t("p.error.back")}</a></p>
</div>
