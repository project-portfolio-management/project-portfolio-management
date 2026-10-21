<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { page } from "$app/state";
  import LinksPanel from "#lib/components/LinksPanel.svelte";

  const pid = $derived(page.params.pid ?? "");
</script>

<svelte:head><title>{t("p.links.links_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.links.back")}</a></p>
<h1>{t("p.links.links")}</h1>
<p class="small muted">{t("p.links.intro")}</p>
<LinksPanel {pid} />
