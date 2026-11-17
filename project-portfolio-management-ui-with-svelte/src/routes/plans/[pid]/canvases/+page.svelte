<!--
  A plan's strategy canvases (`/plans/[pid]/canvases`, LT-1, NC-1 slice 9): the canvases on this plan, and a new one
  on it. An item on a plan's canvas can become a risk on this plan's register; a free-standing canvas cannot.
-->
<script lang="ts">
  import { page } from "$app/state";
  import CanvasList from "#lib/components/CanvasList.svelte";
  import { localeHref, t } from "#lib/i18n.svelte.js";

  const pid = $derived(page.params.pid ?? "");
</script>

<svelte:head><title>{t("p.canvas.plan_title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.canvas.plan_title")}</h1>
<p class="small muted">{t("p.canvas.plan_intro")}</p>
{#key pid}<CanvasList planPid={pid} />{/key}
