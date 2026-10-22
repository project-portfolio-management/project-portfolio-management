<!--
  Board area (`/board`): the period board pack (window decisions,
  benefits realized, milestones, tranche releases + as-of-now health),
  the investment-decisions view, and the stored trend series with an
  explicit "take snapshot" action. All numbers are server-derived;
  trends only ever show captured snapshots. English-first, like the
  other PPM views.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import {
    PpmClient,
    money,
    type BoardPack,
    type Investment,
    type TrendSeries,
  } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let pack = $state<BoardPack | null>(null);
  let investments = $state<Investment[] | null>(null);
  let trends = $state<TrendSeries | null>(null);
  let error = $state<string | null>(null);

  async function load() {
    try {
      pack = await ppm.boardPack();
      investments = (await ppm.boardInvestments()).investments;
      trends = await ppm.boardTrends();
    } catch (err) {
      error = errorText(err, "ppm.common.loadFailed");
    }
  }
  onMount(load);

  async function snapshot() {
    error = null;
    try {
      await ppm.takeSnapshot();
      trends = await ppm.boardTrends();
    } catch (err) {
      error = errorText(err, "ppm.common.actionFailed");
    }
  }

  const label = (entry: Investment) =>
    entry.name ?? entry.title ?? entry.description ?? entry.kind;
  const amount = (entry: Investment) => {
    const minor =
      entry.planned_minor ?? entry.requested_minor ?? entry.budget_cap_minor;
    return minor !== null && minor !== undefined && entry.currency
      ? money(minor, entry.currency)
      : "—";
  };

  type Dec = BoardPack["decisions"][number];
  type Trend = TrendSeries["series"][number];
  const decisionColumns: DataGridColumn[] = $derived([
    { id: "at", header: t("p.board.when"), rowHeader: true },
    { id: "kind", header: t("p.board.kind") },
    { id: "decision", header: t("p.board.decision") },
    { id: "subject", header: t("p.board.subject"), accessor: (r) => { const s = (r as unknown as Dec).subject; return s.name ?? s.pid; } },
  ]);
  const investmentColumns: DataGridColumn[] = $derived([
    { id: "at", header: t("p.board.when"), rowHeader: true },
    { id: "kind", header: t("p.board.kind"), accessor: (r) => { const e = r as unknown as Investment; return `${e.kind}${e.gate ? ` · ${e.gate}` : ""}`; } },
    { id: "what", header: t("p.board.what"), accessor: (r) => label(r as unknown as Investment) },
    { id: "amount", header: t("p.board.amount"), align: "end", sortable: false, accessor: (r) => amount(r as unknown as Investment) },
  ]);
  const trendColumns: DataGridColumn[] = $derived([
    { id: "taken_at", header: t("p.board.taken"), rowHeader: true },
    { id: "plans", header: t("p.board.work_items"), align: "end", accessor: (r) => String((r as unknown as Trend).body["plans"] ?? "—") },
    { id: "portfolios", header: t("p.board.portfolios"), align: "end", accessor: (r) => String((r as unknown as Trend).body["portfolios"] ?? "—") },
    { id: "open_exposure", header: t("p.board.open_exposure"), align: "end", accessor: (r) => String((r as unknown as Trend).body["open_exposure"] ?? "—") },
  ]);
</script>

<svelte:head><title>{t("p.board.ppm", { t: t("ppm.nav.board") })}</title></svelte:head>

<h1>{t("ppm.nav.board")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if pack}
  <h2>{t("p.board.board_pack")}</h2>
  <p class="muted">{t("p.board.window", { from: pack.window.from, to: pack.window.to })}</p>
  <section class="tiles" data-testid="board-tiles">
    <div class="tile"><strong>{pack.health_now.portfolios["red"] ?? 0}</strong><span>{t("p.board.red_portfolios")}</span></div>
    <div class="tile"><strong>{pack.health_now.portfolios["amber"] ?? 0}</strong><span>{t("p.board.amber")}</span></div>
    <div class="tile"><strong>{pack.health_now.portfolios["green"] ?? 0}</strong><span>{t("p.board.green")}</span></div>
    <div class="tile"><strong>{pack.milestones_completed}</strong><span>{t("p.board.milestones_completed")}</span></div>
    <div class="tile"><strong>{pack.tranches_released.count}</strong><span>{t("p.board.tranches_released")}</span></div>
    <div class="tile"><strong>{pack.benefits_realized.events}</strong><span>{t("p.board.benefit_realizations")}</span></div>
  </section>
  <p class="muted">{pack.health_now.note}</p>

  {#if Object.keys(pack.benefits_realized.per_currency_minor).length > 0}
    <p data-testid="board-realized">
      {t("p.board.realized_in_window")}
      {#each Object.entries(pack.benefits_realized.per_currency_minor) as [currency, minor] (currency)}
        <strong>{money(minor, currency)}</strong>&nbsp;
      {/each}
      {#if pack.benefits_realized.unattributed_events > 0}
        <span class="muted">{t("p.board.unattributed_events", { unattributed_events: pack.benefits_realized.unattributed_events })}</span>
      {/if}
    </p>
  {/if}

  <h3>{t("p.board.decisions_in_window")}</h3>
  <DataTable label={t("p.board.decision")} columns={decisionColumns} rows={pack.decisions} rowId={(r) => String(r.kind) + String(r.at) + String((r as unknown as Dec).subject.pid)} emptyText={t("p.board.no_decisions_in_the_window")} testid="board-decisions" pageSize={50} searchable />
{/if}

{#if investments}
  <h2>{t("p.board.investment_decisions")}</h2>
  <DataTable label={t("p.board.what")} columns={investmentColumns} rows={investments} rowId={(r) => String(r.kind) + String(r.at) + label(r as unknown as Investment)} emptyText={t("p.board.no_investment_decisions_recorded")} testid="board-investments" />
{/if}

{#if trends}
  <h2>{t("p.board.trends")}</h2>
  <p class="muted">{trends.note}</p>
  <p><button onclick={() => void snapshot()}>{t("p.board.take_snapshot_now")}</button></p>
  <DataTable label={t("p.board.taken")} columns={trendColumns} rows={trends.series} rowId={(r) => String(r.taken_at)} emptyText={t("p.board.no_snapshots_captured_yet")} testid="board-trends" />
{/if}
