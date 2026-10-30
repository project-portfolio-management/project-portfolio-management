<!--
  Portfolio balance (`/balance`, NC-9): a bubble chart of value, risk and cost for each portfolio, with
  the imbalances it contains stated in words, and the same figures as a table.

  Across is planned cost, up is target benefit value, the bubble's area is open risk exposure. Money is
  shown in one currency at a time and never converted. A flagged portfolio is drawn with a heavy dashed
  outline and a "!" mark and is named in the list; colour is never the only signal.

  Grouped by portfolio, not by category or strategic aim: portfolio is the only grouping that cost,
  benefit value and risk are all reported by. See the notes on the page.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { resource } from "#lib/resource.svelte.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { formatPercent } from "#lib/locale-format.js";
  import {
    PpmClient,
    money,
    type ExecutiveBenefits,
    type FinancialVariance,
    type RiskHeatmap,
  } from "#lib/api/ppm.js";
  import {
    AHEAD_OF_VALUE_GAP,
    BALANCE_BOX,
    CONCENTRATION_SHARE,
    buildBalance,
    currenciesOf,
    layoutBubbles,
    type BalanceFlag,
    type BalanceGroup,
  } from "#lib/balance-view.js";

  const ppm = PpmClient.withFetch();
  const varianceLoad = resource<FinancialVariance>(() => ppm.financialVariance(), "p.balance.could_not_load");
  const benefitsLoad = resource<ExecutiveBenefits>(() => ppm.executiveBenefits(), "p.balance.could_not_load");
  const heatLoad = resource<RiskHeatmap>(() => ppm.riskHeatmap(), "p.balance.could_not_load");
  const error = $derived(varianceLoad.error ?? benefitsLoad.error ?? heatLoad.error);
  const loaded = $derived(varianceLoad.data !== null && benefitsLoad.data !== null && heatLoad.data !== null);

  onMount(() => {
    void varianceLoad.reload();
    void benefitsLoad.reload();
    void heatLoad.reload();
  });

  let chosen = $state("");
  const currencies = $derived(currenciesOf(varianceLoad.data, benefitsLoad.data));
  const currency = $derived(currencies.includes(chosen) ? chosen : (currencies[0] ?? ""));
  const groups = $derived(buildBalance(varianceLoad.data, benefitsLoad.data, heatLoad.data, currency));
  const bubbles = $derived(layoutBubbles(groups));
  const box = BALANCE_BOX;

  const nameOf = (g: BalanceGroup) => g.name ?? t("p.balance.unassigned");
  const pct = (x: number) => formatPercent(x);
  const flagText = (g: BalanceGroup, f: BalanceFlag): string => {
    const values = { name: nameOf(g), percent: f.percent ?? 0, currency };
    switch (f.kind) {
      case "cost_concentration":
        return t("p.balance.flag_cost_concentration", values);
      case "risk_concentration":
        return t("p.balance.flag_risk_concentration", values);
      case "cost_ahead_of_value":
        return t("p.balance.flag_cost_ahead_of_value", values);
      case "risk_ahead_of_value":
        return t("p.balance.flag_risk_ahead_of_value", values);
      case "no_value":
        return t("p.balance.flag_no_value", values);
    }
  };
  const flagged = $derived(groups.filter((g) => g.flags.length > 0));

  const rows = $derived(
    groups.map((g) => ({
      key: g.key,
      name: nameOf(g),
      cost: g.cost,
      value: g.value,
      risk: g.risk,
      costShare: g.costShare,
      valueShare: g.valueShare,
      riskShare: g.riskShare,
      flags: g.flags.length === 0 ? t("p.balance.no_flag") : g.flags.map((f) => flagText(g, f)).join(" "),
    })),
  );
  const columns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.balance.portfolio"), rowHeader: true },
    { id: "cost", header: t("p.balance.cost", { currency }), align: "end", format: (v) => money(Number(v), currency) },
    { id: "value", header: t("p.balance.value", { currency }), align: "end", format: (v) => money(Number(v), currency) },
    { id: "risk", header: t("p.balance.risk"), align: "end" },
    { id: "costShare", header: t("p.balance.cost_share"), align: "end", format: (v) => pct(Number(v)) },
    { id: "valueShare", header: t("p.balance.value_share"), align: "end", format: (v) => pct(Number(v)) },
    { id: "riskShare", header: t("p.balance.risk_share"), align: "end", format: (v) => pct(Number(v)) },
    { id: "flags", header: t("p.balance.flags") },
  ]);

  const labelRight = (cx: number) => cx > box.width - 150;
</script>

<svelte:head><title>{t("p.balance.balance_ppm")}</title></svelte:head>

<h1>{t("p.balance.balance")}</h1>
<p class="small muted">{t("p.balance.intro")}</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if !loaded && !error}
  <p class="small muted">{t("p.balance.loading")}</p>
{/if}

{#if loaded}
  {#if groups.length === 0}
    <p class="small muted" data-testid="balance-empty">{t("p.balance.none")}</p>
  {:else}
    {#if currencies.length > 1}
      <label class="small">
        {t("p.balance.currency")}
        <select bind:value={chosen} data-testid="balance-currency">
          {#each currencies as c (c)}
            <option value={c}>{c}</option>
          {/each}
        </select>
      </label>
    {/if}

    <figure class="balance" data-testid="balance-chart">
      <figcaption id="balance-cap">{t("p.balance.caption", { currency })}</figcaption>
      <svg viewBox={`0 0 ${box.width} ${box.height}`} role="img" aria-labelledby="balance-cap">
        <line x1={box.left} y1={box.top} x2={box.left} y2={box.height - box.bottom} class="axisline" />
        <line x1={box.left} y1={box.height - box.bottom} x2={box.width - box.right} y2={box.height - box.bottom} class="axisline" />
        <text x={(box.left + box.width - box.right) / 2} y={box.height - 6} class="axis" text-anchor="middle">{t("p.balance.axis_cost", { currency })}</text>
        <text transform={`translate(12 ${(box.top + box.height - box.bottom) / 2}) rotate(-90)`} class="axis" text-anchor="middle">{t("p.balance.axis_value", { currency })}</text>
        <text x={box.left - 4} y={box.height - box.bottom + 12} class="tick" text-anchor="end">0</text>
        {#each groups as g, i (g.key)}
          {@const b = bubbles[i]}
          {#if b}
            <g data-testid="balance-bubble" data-flagged={g.flags.length > 0 ? "yes" : "no"}>
              <title>{nameOf(g)}: {money(g.cost, currency)}, {money(g.value, currency)}, {g.risk}</title>
              <circle cx={b.cx} cy={b.cy} r={b.r} class={g.flags.length > 0 ? "bubble flagged" : "bubble"} />
              {#if g.flags.length > 0}
                <text x={b.cx} y={b.cy + 4} class="bang" text-anchor="middle" aria-hidden="true">!</text>
              {/if}
              <text
                x={labelRight(b.cx) ? b.cx - b.r - 4 : b.cx + b.r + 4}
                y={b.cy - b.r - 3}
                class="blabel"
                text-anchor={labelRight(b.cx) ? "end" : "start"}>{nameOf(g)}</text>
            </g>
          {/if}
        {/each}
      </svg>
    </figure>

    <section aria-labelledby="balance-flags">
      <h2 id="balance-flags">{t("p.balance.imbalances")}</h2>
      {#if flagged.length === 0}
        <p class="small" data-testid="balance-no-flags">{t("p.balance.no_imbalance")}</p>
      {:else}
        <ul data-testid="balance-flags">
          {#each flagged as g (g.key)}
            {#each g.flags as f (f.kind)}
              <li><span aria-hidden="true">! </span>{flagText(g, f)}</li>
            {/each}
          {/each}
        </ul>
      {/if}
      <p class="small muted">
        {t("p.balance.rules", { concentration: Math.round(CONCENTRATION_SHARE * 100), gap: Math.round(AHEAD_OF_VALUE_GAP * 100) })}
      </p>
    </section>

    <section aria-labelledby="balance-table-h">
      <h2 id="balance-table-h">{t("p.balance.table")}</h2>
      <DataTable label={t("p.balance.table")} {columns} {rows} rowId={(r) => String(r.key)} emptyText={t("p.balance.none")} testid="balance-table" />
    </section>
  {/if}

  <p class="small muted">{t("p.balance.notes")}</p>
{/if}

<style>
  .balance { margin: 1rem 0; max-inline-size: 36rem; }
  figcaption { font-weight: 600; margin-block-end: 0.25rem; }
  svg { inline-size: 100%; block-size: auto; }
  .axisline { stroke: #555; stroke-width: 1; }
  .axis, .tick { font-size: 10px; fill: #333; }
  .bubble { fill: #8fb4d9; fill-opacity: 0.75; stroke: #1f4e79; stroke-width: 1.5; }
  .bubble.flagged { fill: #f3c9a0; stroke: #7a2e00; stroke-width: 3; stroke-dasharray: 5 3; }
  .bang { font-size: 14px; font-weight: 700; fill: #7a2e00; }
  .blabel { font-size: 10px; fill: #111; }
  ul { padding-inline-start: 1.2rem; }
</style>
