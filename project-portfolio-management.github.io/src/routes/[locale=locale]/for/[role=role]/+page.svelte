<script lang="ts">
  import Doc from "#lib/Doc.svelte";
  import { href } from "#lib/href.js";
  import { siteTag, t } from "#lib/i18n.svelte.js";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<svelte:head><title>{data.title} · {t("brand")}</title></svelte:head>

<p class="up" lang={siteTag()} dir="ltr"><a href={href(`/${data.locale}/for/`)}>{t("briefs.all")}</a></p>
<Doc html={data.html} path={data.path} />

<style>
  .up {
    margin-block: 0 1rem;
    font-size: 0.875rem;
  }
</style>
