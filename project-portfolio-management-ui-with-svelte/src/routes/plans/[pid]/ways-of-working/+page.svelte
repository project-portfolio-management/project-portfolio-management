<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { page } from "$app/state";
  import FrameworkPanel from "#lib/components/FrameworkPanel.svelte";
</script>

<svelte:head><title>{t("p.frameworks.wow_ppm")}</title></svelte:head>
<FrameworkPanel family="ways-of-working" pid={page.params.pid ?? ""} heading={t("p.frameworks.wow")} />
