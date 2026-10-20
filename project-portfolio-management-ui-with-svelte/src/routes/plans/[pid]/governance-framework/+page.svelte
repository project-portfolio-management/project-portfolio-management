<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { page } from "$app/state";
  import FrameworkPanel from "#lib/components/FrameworkPanel.svelte";
</script>

<svelte:head><title>{t("p.frameworks.gov_ppm")}</title></svelte:head>
<FrameworkPanel family="governance-frameworks" pid={page.params.pid ?? ""} heading={t("p.frameworks.gov")} />
