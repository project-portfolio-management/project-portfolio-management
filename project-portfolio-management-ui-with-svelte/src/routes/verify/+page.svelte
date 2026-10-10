<!--
  Verify page (BFF). The magic-link exchange happens entirely in the
  server `load`, which sets the httpOnly session cookie and redirects home
  on success — so this renders ONLY on a missing/invalid link. The browser
  never handles an access token.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
    import type { PageData } from "./$types";

    let { data }: { data: PageData } = $props();

    const message = $derived(
        data.error === "missingToken"
            ? t("p.verify.this_sign_in_link_is")
            : data.error === "serviceUnavailable"
              ? t("p.verify.we_could_not_reach_the")
              : t("p.verify.this_sign_in_link_is_2"),
    );
</script>

<svelte:head><title>{t("p.verify.sign_in_link")}</title></svelte:head>

<h1>{t("p.verify.sign_in_link")}</h1>
<p class="banner" role="alert">{message}</p>
<p><a class="button" href={localeHref("/signin")}>{t("p.verify.request_a_new_link")}</a></p>
