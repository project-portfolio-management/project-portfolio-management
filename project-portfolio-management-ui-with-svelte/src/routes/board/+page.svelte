<!--
  Board area (`/board`): the period board pack (window decisions,
  benefits realized, milestones, tranche releases + as-of-now health),
  the investment-decisions view, and the stored trend series with an
  explicit "take snapshot" action. All numbers are server-derived;
  trends only ever show captured snapshots. English-first, like the
  other PPM views.
-->
<script lang="ts">
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
      error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  }
  onMount(load);

  async function snapshot() {
    error = null;
    try {
      await ppm.takeSnapshot();
      trends = await ppm.boardTrends();
    } catch (err) {
      error = err instanceof Error ? err.message : t("ppm.common.actionFailed");
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
  <table data-testid="board-decisions">
    <thead><tr><th>{t("p.board.when")}</th><th>{t("p.board.kind")}</th><th>{t("p.board.decision")}</th><th>{t("p.board.subject")}</th></tr></thead>
    <tbody>
      {#each pack.decisions as entry (entry.kind + entry.at + entry.subject.pid)}
        <tr>
          <td>{entry.at}</td>
          <td>{entry.kind}</td>
          <td>{entry.decision}</td>
          <td>{entry.subject.name ?? entry.subject.pid}</td>
        </tr>
      {:else}
        <tr><td colspan="4" class="muted">{t("p.board.no_decisions_in_the_window")}</td></tr>
      {/each}
    </tbody>
  </table>
{/if}

{#if investments}
  <h2>{t("p.board.investment_decisions")}</h2>
  <table data-testid="board-investments">
    <thead><tr><th>{t("p.board.when")}</th><th>{t("p.board.kind")}</th><th>{t("p.board.what")}</th><th>{t("p.board.amount")}</th></tr></thead>
    <tbody>
      {#each investments as entry (entry.kind + entry.at + label(entry))}
        <tr>
          <td>{entry.at}</td>
          <td>{entry.kind}{#if entry.gate}&nbsp;· {entry.gate}{/if}</td>
          <td>{label(entry)}</td>
          <td>{amount(entry)}</td>
        </tr>
      {:else}
        <tr><td colspan="4" class="muted">{t("p.board.no_investment_decisions_recorded")}</td></tr>
      {/each}
    </tbody>
  </table>
{/if}

{#if trends}
  <h2>{t("p.board.trends")}</h2>
  <p class="muted">{trends.note}</p>
  <p><button onclick={() => void snapshot()}>{t("p.board.take_snapshot_now")}</button></p>
  <table data-testid="board-trends">
    <thead><tr><th>{t("p.board.taken")}</th><th>{t("p.board.work_items")}</th><th>{t("p.board.portfolios")}</th><th>{t("p.board.open_exposure")}</th></tr></thead>
    <tbody>
      {#each trends.series as row (row.taken_at)}
        <tr>
          <td>{row.taken_at}</td>
          <td>{String(row.body["plans"] ?? "—")}</td>
          <td>{String(row.body["portfolios"] ?? "—")}</td>
          <td>{String(row.body["open_exposure"] ?? "—")}</td>
        </tr>
      {:else}
        <tr><td colspan="4" class="muted">{t("p.board.no_snapshots_captured_yet")}</td></tr>
      {/each}
    </tbody>
  </table>
{/if}
