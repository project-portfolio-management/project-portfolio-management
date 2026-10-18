<!--
  Planned value, earned value and actual cost for one currency, as three bars
  against the budget-at-completion line (SC-4). Each bar carries its amount as text
  — the chart is never the only place a number appears — and an unknown actual cost
  is drawn as nothing and called unknown, never zero.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { money, type EvBlock } from "#lib/api/ppm.js";
  import { evBars } from "#lib/ev-view.js";

  let { block }: { block: EvBlock } = $props();
  const bars = $derived(evBars(block));
</script>

<ul class="bars" data-testid="ev-bars">
  {#each bars as bar (bar.key)}
    <li>
      <span class="label">{bar.label}</span>
      <span class="track" aria-hidden="true">
        <span class="fill {bar.key}" style:width="{bar.widthPct}%"></span>
        <span class="bac" style:left="{bar.bacPct}%"></span>
      </span>
      <span class="amount">{bar.minor === null ? t("c.EvBars.unknown") : money(bar.minor, block.currency)}</span>
    </li>
  {/each}
  <li class="small muted">{t("c.EvBars.the_double_line_is_the", { money: money(block.bac_minor, block.currency) })}</li>
</ul>

<style>
  .bars {
    list-style: none;
    margin: 0.5rem 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    max-width: 44rem;
  }
  li:not(.small) {
    display: grid;
    grid-template-columns: 9rem 1fr 11rem;
    gap: 0.6rem;
    align-items: center;
  }
  .track {
    position: relative;
    display: block;
    height: 1rem;
    background: var(--surface-2, #f0f0f0);
    border-radius: 3px;
  }
  .fill {
    display: block;
    height: 100%;
    border-radius: 3px;
  }
  .fill.pv {
    background: repeating-linear-gradient(135deg, #35547c 0 2px, transparent 2px 6px);
    border: 1px solid #35547c;
  }
  .fill.ev {
    background: #1d8a4e;
  }
  .fill.ac {
    background: #b45309;
  }
  .bac {
    position: absolute;
    top: -2px;
    bottom: -2px;
    border-inline-start: 3px double currentColor;
  }
  .amount {
    font-variant-numeric: tabular-nums;
  }
  @media (max-width: 600px) {
    li:not(.small) {
      grid-template-columns: 1fr;
    }
  }
</style>
