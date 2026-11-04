<script lang="ts">
  import Home from "#lib/Home.svelte";
  import { langTag } from "#lib/site.js";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<svelte:head><title>{data.title}</title></svelte:head>

<Home home={data.home} lang={langTag(data.locale)} />
