<!--
  One person's load against their capacity (PJ-3, PJ-4). The solid bar is what is
  **firm**; the hatched extension after it is what is only **proposed** (never
  added into the firm figure); the vertical line is the capacity in force (it
  moves left for a part-time spell or leave). Under it, a strip marks the days
  that are over capacity. State has a marker and a sentence, not just colour.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import type { CapacityView } from "#lib/api/ppm.js";
  import { loadModel, spanSegments } from "#lib/capacity-view.js";

  let {
    person,
    from,
    to,
  }: { person: CapacityView["people"][number]; from: string; to: string } = $props();

  const model = $derived(loadModel(person));
  const segments = $derived(spanSegments(from, to, person.over_capacity));
</script>

<div class="meter-wrap" data-testid="capacity-meter">
  <span class="mk" aria-hidden="true">{model.marker}</span>
  <span class="stack">
    <span class="meter" role="img" aria-label={model.label}>
      <span class="fill {model.state}" style="width:{model.committedPct}%"></span>
      {#if model.proposedPct > 0}
        <span class="proposed" style="left:{model.committedPct}%;width:{model.proposedPct}%"></span>
      {/if}
      <span class="cap" style="left:{model.capacityPct}%"></span>
    </span>
    {#if segments.length > 0}
      <span class="strip" role="img" aria-label={t("c.CapacityMeter.days_over_capacity")}>
        {#each segments as s (s.label)}
          <span class="seg" style="left:{s.startPct}%;width:{s.widthPct}%" title={s.label}></span>
        {/each}
      </span>
    {/if}
    <span class="small">{model.label}</span>
  </span>
</div>

<style>
  .meter-wrap {
    display: flex;
    gap: 0.5rem;
    align-items: flex-start;
  }
  .mk {
    font-weight: 700;
    min-width: 1.1em;
  }
  .stack {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    min-width: 12rem;
    max-width: 24rem;
  }
  .meter,
  .strip {
    position: relative;
    display: block;
    height: 0.8rem;
    background: var(--surface-2, #f0f0f0);
    border-radius: 4px;
  }
  .strip {
    height: 0.4rem;
  }
  .fill {
    display: block;
    height: 100%;
    background: #1d8a4e;
    border-radius: 4px 0 0 4px;
  }
  .fill.over {
    background: #a4262c;
  }
  .fill.proposed_over {
    background: #b45309;
  }
  .proposed {
    position: absolute;
    top: 0;
    bottom: 0;
    background: repeating-linear-gradient(135deg, #35547c 0 2px, transparent 2px 6px);
    border: 1px solid #35547c;
  }
  .cap {
    position: absolute;
    top: -2px;
    bottom: -2px;
    border-inline-start: 3px double currentColor;
  }
  .seg {
    position: absolute;
    top: 0;
    bottom: 0;
    background: #a4262c;
  }
</style>
