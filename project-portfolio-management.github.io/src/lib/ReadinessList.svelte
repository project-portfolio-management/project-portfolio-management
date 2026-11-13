<!--
  One run of readiness items on a path-to-production page: each item's title, the project's estimate, a Lily slider
  that starts at the estimate (the reader may move it; nothing is saved or sent) and the item's text. The page keys this
  on its path, so a reader's values never carry over to another locale's page.
-->
<script lang="ts">
  import { Slider } from "@lilydesignsystem/svelte-headless";
  import { siteDir, siteTag, t } from "#lib/i18n.svelte.js";

  type Item = { title: string; percent: number; html: string };
  let { items, offset = 0 }: { items: Item[]; offset?: number } = $props();

  // svelte-ignore state_referenced_locally
  let values = $state(items.map((i) => i.percent));
</script>

<ul class="readiness">
  {#each items as item, j (j)}
    {@const id = `ptp-${offset + j}`}
    <li>
      <p class="readiness-head">
        <strong>{item.title}</strong>
        <span class="estimate" lang={siteTag()} dir={siteDir()}>{t("ptp.estimate")}: {item.percent}%</span>
      </p>
      <div class="readiness-slider">
        <Slider
          {id}
          label={`${item.title}: ${t("ptp.estimate")}`}
          bind:value={values[j]}
          min={0}
          max={100}
          step={5}
          aria-valuetext={`${values[j]}%`}
        />
        <output for={id} aria-hidden="true">{values[j]}%</output>
      </div>
      <!-- Rendered at build time from the repository's own Markdown, raw HTML escaped ($lib/server/content.ts). -->
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      <p>{@html item.html}</p>
    </li>
  {/each}
</ul>

<style>
  .readiness {
    list-style: none;
    padding-inline-start: 0;
  }
  .readiness > li {
    margin-block: 0 1.25rem;
    padding-block-end: 1rem;
    border-block-end: 1px solid var(--mxi-color-border);
  }
  .readiness-head {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem 0.75rem;
    align-items: baseline;
    margin-block: 0 0.25rem;
  }
  .estimate {
    color: var(--mxi-color-muted);
    font-size: 0.875rem;
  }
  .readiness-slider {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }
  .readiness-slider :global(input[type="range"]) {
    flex: 1 1 auto;
    min-inline-size: 0;
    accent-color: var(--mxi-color-link);
  }
  .readiness-slider output {
    min-inline-size: 3.5em;
    text-align: end;
    font-variant-numeric: tabular-nums;
  }
  .readiness li > p:last-child {
    margin-block: 0.25rem 0;
  }
</style>
