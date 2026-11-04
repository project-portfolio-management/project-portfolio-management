<script lang="ts">
  import Doc from "#lib/Doc.svelte";
  import { t } from "#lib/i18n.svelte.js";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<svelte:head><title>{data.title} · {t("brand")}</title></svelte:head>

<Doc html={data.html} path={data.path} />
