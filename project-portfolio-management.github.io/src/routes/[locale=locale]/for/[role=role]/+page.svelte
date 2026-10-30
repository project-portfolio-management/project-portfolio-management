<script lang="ts">
  import Doc from "#lib/Doc.svelte";
  import { href } from "#lib/href.js";
  import { BRIEF_LOCALES, langTag } from "#lib/site.js";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<svelte:head>
  <title>{data.title} · Project Portfolio Management</title>
  {#each BRIEF_LOCALES as l (l)}
    <link rel="alternate" hreflang={langTag(l)} href={href(`/${l}/for/${data.role}/`)} />
  {/each}
</svelte:head>

<p class="up" lang="en" dir="ltr"><a href={href(`/${data.locale}/for/`)}>All role briefs</a></p>
<Doc html={data.html} path={data.path} />

<style>
  .up {
    margin-block: 0 1rem;
    font-size: 0.875rem;
  }
</style>
