<!--
  Strategy canvases (`/canvases`, LT-1, NC-1 slice 9): every canvas the caller may read, of every plan and none, and a
  new free-standing one. A canvas on a plan is started from that plan's canvases page.
-->
<script lang="ts">
  import CanvasList from "#lib/components/CanvasList.svelte";
  import { t } from "#lib/i18n.svelte.js";
</script>

<svelte:head><title>{t("p.canvas.title_ppm")}</title></svelte:head>

<h1>{t("p.canvas.title")}</h1>
<p class="small muted">{t("p.canvas.intro")}</p>
<CanvasList />
