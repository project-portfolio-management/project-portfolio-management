<!--
  A plan's phase and, for a programme, its start (`/plans/[pid]/programme`, FR-30, EX-8, NC-1 slice 11 part m). The
  phase moves one step forward, or back with a reason; the time in each phase is shown. A programme's start (its move
  into executing) meets the start check: the limit on programmes in active delivery and the constraint pool. The check
  is shown first, with this programme's firm demand by skill pool and month; a refused start joins the start queue,
  and an override with a reason of 10 or more characters starts it anyway. The refusal's sentence is built from the
  start check, read again after the refusal: the service's own text is never shown. Rules in `src/lib/phase-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tDynamic, tn } from "#lib/i18n.svelte.js";
  import { formatDecimal, formatMonth } from "#lib/locale-format.js";
  import { PpmClient, type PhaseHistory, type PhaseToken, type StartCheck } from "#lib/api/ppm.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import {
    OVERRIDE_MIN,
    demandGrid,
    isBack,
    isStartRefused,
    meetsStartCheck,
    phaseChoices,
    phaseMoveBody,
    phaseRefusalKey,
    startRefusalText,
    wholeDays,
  } from "#lib/phase-view.js";

  let { pid, isProgramme }: { pid: string; isProgramme: boolean } = $props();
  const ppm = PpmClient.withFetch();

  let history = $state<PhaseHistory | null>(null);
  let check = $state<StartCheck | null>(null);
  let loadError = $state<string | null>(null);
  let checkError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let notice = $state("");
  let refused = $state(false);
  let busy = $state(false);
  let to = $state<PhaseToken | "">("");
  let reason = $state("");
  let override = $state("");

  const current = $derived(history?.current_phase ?? null);
  const choices = $derived(phaseChoices(current));
  const starting = $derived(meetsStartCheck(isProgramme, to));
  const showOverride = $derived(starting && (refused || check === null || check.would_be_refused));
  const phaseName = (p: PhaseToken) => tDynamic(`p.phase.name_${p}`);
  const durationLine = (d: PhaseHistory["durations"][number]) =>
    [
      tn("p.phase.days_in", wholeDays(d.ms), { phase: phaseName(d.phase), days: wholeDays(d.ms) }),
      tn("p.phase.visits", d.visits, { visits: d.visits }),
      ...(d.current ? [t("p.phase.current")] : []),
    ].join(" · ");
  const why = (r: StartCheck["refusals"][number]) => startRefusalText(r, (k, v) => t(k, v), (m) => formatMonth(m, "long"));

  const grid = $derived(check ? demandGrid(check.demand_by_pool) : { months: [], rows: [] });
  const demandColumns: DataGridColumn[] = $derived([
    { id: "pool", header: t("p.phase.col_pool"), rowHeader: true },
    ...grid.months.map((m, i) => ({
      id: `m${i}`,
      header: formatMonth(m),
      align: "end" as const,
      accessor: (row: Record<string, unknown>) => (row.cells as (number | null)[])[i] ?? null,
      format: (v: unknown) => (v === null ? "—" : formatDecimal(Number(v), 2)),
    })),
  ]);

  async function loadCheck() {
    if (!isProgramme) return;
    try {
      check = await ppm.startCheck(pid);
      checkError = null;
    } catch (err) {
      check = null;
      checkError = errorText(err, "p.phase.could_not_check");
    }
  }

  async function load() {
    try {
      history = await ppm.phaseHistory(pid);
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.phase.could_not_load");
    }
    await loadCheck();
  }
  onMount(() => void load());

  async function move(e: SubmitEvent) {
    e.preventDefault();
    error = null;
    notice = "";
    const r = phaseMoveBody(current, { to, reason, override }, isProgramme);
    if ("problem" in r) {
      error = t(r.problem, { min: OVERRIDE_MIN });
      return;
    }
    busy = true;
    try {
      const done = await ppm.setPhase(pid, r.body);
      notice = r.body.start_override_reason
        ? t("p.phase.started_override", { phase: phaseName(done.phase) })
        : t("p.phase.moved", { phase: phaseName(done.phase) });
      to = "";
      reason = "";
      override = "";
      refused = false;
      await load();
    } catch (err) {
      if (isStartRefused(err)) {
        refused = true;
        // The 422 says why only in the service's English: read the check again for the reasons and the place.
        await loadCheck();
        error =
          check?.queue_position != null
            ? t("p.phase.start_refused_place", { n: check.queue_position })
            : t("p.phase.start_refused");
      } else {
        const k = phaseRefusalKey(err);
        error = k ? t(k) : errorText(err, "p.phase.could_not_move");
      }
    } finally {
      busy = false;
    }
  }
</script>

<section aria-labelledby="phase-h">
  <h2 id="phase-h">{isProgramme ? t("p.phase.heading_programme") : t("p.phase.heading")}</h2>
  <p class="small muted">{t("p.phase.says")}</p>
  {#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}
  {#if history}
    <p data-testid="phase-now"><strong>{current ? t("p.phase.now", { phase: phaseName(current) }) : t("p.phase.none")}</strong></p>
    {#if history.transitions.length > 0}
      <ul class="small" aria-label={t("p.phase.time_heading")}>
        {#each history.durations.filter((d) => d.visits > 0) as d (d.phase)}
          <li>{durationLine(d)}</li>
        {/each}
      </ul>
    {/if}
  {/if}

  {#if isProgramme}
    <h3 id="start-check-h">{t("p.phase.check_heading")}</h3>
    <p class="small muted">{t("p.phase.check_says", { min: OVERRIDE_MIN })}</p>
    {#if checkError}<p class="banner error" role="alert">{checkError}</p>{/if}
    {#if check}
      <ul aria-labelledby="start-check-h" data-testid="start-check">
        <li>
          {#if check.wip.limit !== null}
            {t("p.phase.places", { in_delivery: check.wip.in_delivery, limit: check.wip.limit })}
          {:else if check.wip.limit_setting_unusable}
            {t("p.phase.limit_unusable")}
          {:else}
            {t("p.phase.no_limit")}
          {/if}
        </li>
        <li>{check.constraint_pool ? t("p.phase.pool_is", { pool: check.constraint_pool }) : t("p.phase.no_pool")}</li>
        <li>
          {#if check.would_be_refused}
            <span aria-hidden="true">✗</span> {t("p.phase.would_refuse")}
            <ul>{#each check.refusals as r (r.reason)}<li>{why(r)}</li>{/each}</ul>
          {:else}
            <span aria-hidden="true">✓</span> {t("p.phase.would_start")}
          {/if}
        </li>
        {#if check.queue_position !== null}
          <li>{t("p.phase.in_queue", { n: check.queue_position })} <a href={localeHref("/start-queue")}>{t("p.phase.see_queue")}</a></li>
        {/if}
      </ul>
      <h4>{t("p.phase.demand_heading")}</h4>
      {#if grid.rows.length === 0}
        <p class="small muted">{t("p.phase.no_demand")}</p>
      {:else}
        <DataTable label={t("p.phase.demand_heading")} caption={t("p.phase.demand_caption")} columns={demandColumns} rows={grid.rows} rowId={(r) => String(r.pool)} sortable={false} />
      {/if}
    {/if}
    <div class="row">
      <button class="button" type="button" disabled={busy} onclick={() => void loadCheck()}>{t("p.phase.check_again")}</button>
    </div>
  {/if}

  {#if error}<p class="banner error" role="alert">{error}</p>{/if}
  <p class="small" role="status">{notice}</p>
  <form class="stack" novalidate onsubmit={move} aria-labelledby="phase-move-h">
    <h3 id="phase-move-h">{t("p.phase.move_heading")}</h3>
    <fieldset>
      <legend>{t("p.phase.f_to")}</legend>
      {#if choices.forward}
        <label class="check"><input type="radio" name="phase-to" value={choices.forward} bind:group={to} disabled={!history} /> {t("p.phase.forward_to", { phase: phaseName(choices.forward) })}</label>
      {/if}
      {#each choices.back as b (b)}
        <label class="check"><input type="radio" name="phase-to" value={b} bind:group={to} disabled={!history} /> {t("p.phase.back_to", { phase: phaseName(b) })}</label>
      {/each}
    </fieldset>
    {#if to !== "" && isBack(current, to)}<p class="small" role="note">{t("p.phase.back_needs_reason")}</p>{/if}
    <label>{t("p.phase.f_reason")}<textarea bind:value={reason} rows="2" disabled={!history}></textarea></label>
    {#if starting}
      <p class="small" role="note">{check?.would_be_refused || refused ? t("p.phase.start_would_queue") : t("p.phase.start_may_queue")}</p>
    {/if}
    {#if showOverride}
      <label>{t("p.phase.f_override", { min: OVERRIDE_MIN })}<textarea bind:value={override} rows="2"></textarea></label>
    {/if}
    {#if refused && check && check.refusals.length > 0}
      <ul class="small" aria-label={t("p.phase.refused_why")}>{#each check.refusals as r (r.reason)}<li>{why(r)}</li>{/each}</ul>
    {/if}
    <div class="row"><button class="button" type="submit" disabled={busy || !history}>{t("p.phase.move")}</button></div>
  </form>
</section>

<style>
  section {
    margin-block-start: 2rem;
  }
  form {
    max-width: 40rem;
    margin-block-start: 1rem;
  }
  fieldset {
    display: grid;
    gap: 0.35rem;
  }
  .check {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }
  h3,
  h4 {
    font-size: 1rem;
    margin-block: 1rem 0.25rem;
  }
</style>
