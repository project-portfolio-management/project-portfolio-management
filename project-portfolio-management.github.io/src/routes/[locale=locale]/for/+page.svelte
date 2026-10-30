<script lang="ts">
  import { href } from "#lib/href.js";
  import { langTag } from "#lib/site.js";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();

  // The language's own name for itself, from the build machine's Intl data (not a translation written here).
  const endonym = $derived.by(() => {
    try {
      return new Intl.DisplayNames([langTag(data.locale)], { type: "language" }).of(langTag(data.locale)) ?? data.locale;
    } catch {
      return data.locale;
    }
  });
</script>

<svelte:head><title>Role briefs ({data.locale}) · Project Portfolio Management</title></svelte:head>

<h1 lang="en" dir="ltr">Role briefs</h1>
<p class="lede"><span lang="en" dir="ltr">Locale:</span> {endonym} (<code>{data.locale}</code>)</p>
<p class="lede" lang="en" dir="ltr">
  One page for each senior role: what the software is, what it gives you, what it does not, and the decisions that are
  yours. Use the language button in the header for the same brief in another locale.
</p>
<ul class="briefs">
  {#each data.briefs as b (b.role)}
    <li>
      <a href={href(`/${data.locale}/for/${b.role}/`)}>{b.title}</a>
      {#if b.summary}<p>{b.summary}</p>{/if}
    </li>
  {/each}
</ul>

<style>
  .lede {
    color: var(--mxi-color-muted);
    max-width: 60rem;
  }
  .briefs {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 1rem;
  }
  .briefs li {
    border: 1px solid var(--mxi-color-border);
    border-radius: var(--mxi-radius);
    padding: 0.75rem 1rem;
    background: var(--mxi-color-surface);
  }
  .briefs a {
    font-weight: 600;
    text-decoration: underline;
  }
  .briefs p {
    margin: 0.25rem 0 0;
  }
</style>
