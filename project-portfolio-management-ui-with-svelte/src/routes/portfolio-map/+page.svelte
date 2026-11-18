<!--
  The portfolio map and strategic aims (`/portfolio-map`, DP-4, NC-1 slice 11): the live map of the programmes the
  reader can read, each once under its primary strategic aim, drawn as the director pack draws it (`buildPackView`'s
  map section: confidence a word and a symbol, beside the derived RAG, never blended). Below, which objectives are
  strategic aims, and each programme's one primary aim. The service decides who may change them; a refusal is the
  catalogue's sentence. Last (NC-1 slice 12 part f), the pack's programme readings read now, each drawn by the pack's
  renderer: the buffer fever chart, earned value over the threshold (its actual cost chosen), validated benefits, and
  issues and change requests over a window of days (`live-pack-view.ts`).
-->
<script lang="ts">
  import { onMount } from "svelte";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { errorText } from "#lib/error-text.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { i18n, localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { PpmClient, type Objective, type PortfolioMap } from "#lib/api/ppm.js";
  import { buildPackView, type Section } from "#lib/pack-view.js";
  import { aimsOf, mapPeriodProblem, primaryProblem, programmesOf } from "#lib/aims-view.js";
  import type { AcSource } from "#lib/api/ppm.js";
  import {
    AC_KEY,
    AC_SOURCES,
    WINDOW_MAX,
    WINDOW_MIN,
    acParam,
    issuesUnavailable,
    liveSections,
    windowParam,
    windowProblem,
    type LiveId,
  } from "#lib/live-pack-view.js";

  const ppm = PpmClient.withFetch();

  // The pack's programme readings, read now (NC-1 slice 12 part f).
  type Body = Record<string, unknown>;
  type Reading = "fever" | "evm" | "benefits" | "issues";
  let live = $state<Record<Reading, Body | null>>({ fever: null, evm: null, benefits: null, issues: null });
  let liveErrors = $state<Partial<Record<Reading, string>>>({});
  let liveProblem = $state<string | null>(null);
  let ac = $state<AcSource>("budget_lines");
  let windowDays = $state<number | null>(null);
  const sections = $derived(
    liveSections(live, { t: (k, v) => tDynamic(k, v), label: (g, v) => enumLabel(g as never, v), locale: i18n.locale }),
  );
  const READINGS: Array<{ id: Reading; shows: LiveId[]; fails: "p.live_pack.could_not_read_fever" | "p.live_pack.could_not_read_evm" | "p.live_pack.could_not_read_benefits" | "p.live_pack.could_not_read_issues" }> = [
    { id: "fever", shows: ["fever"], fails: "p.live_pack.could_not_read_fever" },
    { id: "evm", shows: ["evm"], fails: "p.live_pack.could_not_read_evm" },
    { id: "benefits", shows: ["benefits"], fails: "p.live_pack.could_not_read_benefits" },
    { id: "issues", shows: ["issues", "changes"], fails: "p.live_pack.could_not_read_issues" },
  ];
  async function readOne(id: Reading, run: () => Promise<Body>, fails: (typeof READINGS)[number]["fails"]) {
    try {
      live[id] = await run();
      liveErrors = { ...liveErrors, [id]: undefined };
    } catch (err) {
      live[id] = null;
      liveErrors = { ...liveErrors, [id]: errorText(err, fails) };
    }
  }
  function loadLive(e?: SubmitEvent) {
    e?.preventDefault();
    const problem = windowProblem(windowDays);
    liveProblem = problem ? t(problem, { min: WINDOW_MIN, max: WINDOW_MAX }) : null;
    if (problem) return;
    const runs: Record<Reading, () => Promise<Body>> = {
      fever: () => ppm.bufferFever(),
      evm: () => ppm.portfolioEarnedValue(acParam(ac)),
      benefits: () => ppm.portfolioBenefits(),
      issues: () => ppm.issuesAndChanges(windowParam(windowDays)),
    };
    for (const r of READINGS) void readOne(r.id, runs[r.id], r.fails);
  }

  let map = $state<PortfolioMap | null>(null);
  let objectives = $state<Objective[] | null>(null);
  let loadError = $state<string | null>(null);
  let period = $state("");
  let error = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);
  let programme = $state("");
  let aim = $state("");

  const aims = $derived(map ? aimsOf(map) : []);
  const aimSet = $derived(new Set(aims.map((a) => a.pid)));
  const programmes = $derived(map ? programmesOf(map) : []);
  const section: Section | null = $derived(
    map
      ? (buildPackView(
          { portfolio_map: map },
          { t: (k, v) => tDynamic(k, v), label: (g, v) => enumLabel(g as never, v), locale: i18n.locale },
        ).find((s) => s.id === "map") ?? null)
      : null,
  );

  type PackCell = { text: string; symbol?: string; tone?: string };
  const cellOf = (row: unknown, i: number): PackCell => (row as { cells: PackCell[] }).cells[i] ?? { text: "" };
  const packColumns = (names: string[]): DataGridColumn[] =>
    names.map((name, i) => ({
      id: `c${i}`,
      header: name,
      rowHeader: i === 0,
      accessor: (row) => cellOf(row, i).text,
      ...(i === 0 ? {} : { cell: packCell }),
    }));

  async function loadMap() {
    loadError = null;
    const problem = mapPeriodProblem(period);
    if (problem) {
      loadError = t(problem);
      return;
    }
    try {
      map = await ppm.portfolioMap(period || undefined);
    } catch (err) {
      loadError = errorText(err, "p.aims.could_not_load");
    }
  }
  async function loadObjectives() {
    try {
      objectives = await ppm.listObjectives();
    } catch {
      objectives = null; // the aims list is then not shown; the map still is
    }
  }
  onMount(() => {
    void loadMap();
    void loadObjectives();
    loadLive();
  });

  async function change(run: () => Promise<unknown>, done: string) {
    error = null;
    notice = "";
    busy = true;
    try {
      await run();
      notice = done;
      await loadMap();
    } catch (err) {
      error = errorText(err, "p.aims.could_not_save");
    } finally {
      busy = false;
    }
  }
  function toggleAim(o: Objective, on: boolean, box: HTMLInputElement) {
    void change(() => ppm.markStrategicAim(o.pid, on), on ? t("p.aims.marked") : t("p.aims.unmarked")).then(() => {
      // A refusal leaves the box as the reader clicked it: put it back to what the map says.
      box.checked = aimSet.has(o.pid);
    });
  }
  function setPrimary(e: SubmitEvent) {
    e.preventDefault();
    const problem = primaryProblem(
      programmes.find((p) => p.pid === programme),
      aim,
    );
    if (problem) {
      error = t(problem);
      notice = "";
      return;
    }
    void change(() => ppm.setPrimaryAim(programme, aim), t("p.aims.primary_set"));
  }
</script>

{#snippet packCell({ row, column, formatted }: DataGridCellContext)}
  {@const c = cellOf(row, Number(column.id.slice(1)))}
  {#if c.symbol}<span aria-hidden="true" class="sym">{c.symbol}</span>{/if}{formatted}
{/snippet}

<svelte:head><title>{t("p.aims.title_ppm")}</title></svelte:head>

<h1>{t("p.aims.title")}</h1>
<p class="small muted">{t("p.aims.intro")}</p>

<form class="row period" novalidate onsubmit={(e) => { e.preventDefault(); void loadMap(); }}>
  <label>
    {t("p.aims.f_period")}
    <input type="month" bind:value={period} />
  </label>
  <button class="button" type="submit">{t("p.aims.show")}</button>
</form>

{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}

{#if map}
  <section aria-labelledby="map-h">
    <h2 id="map-h">{t("p.aims.map_heading", { period: map.period })}</h2>
    {#if map.programmes_total === 0}
      <p class="muted">{t("p.aims.no_programmes")}</p>
    {:else if section}
      {#each section.lines as line, i (i)}<p>{line}</p>{/each}
      {#each section.tables as tb, ti (ti)}
        <DataTable label={tb.caption} caption={tb.caption} captionVisible columns={packColumns(tb.columns)} rows={tb.rows.map((cells) => ({ cells }))} rowId={(_r, i) => String(i)} sortable={false} />
      {/each}
    {/if}
  </section>

  {#if error}<p class="banner error" role="alert">{error}</p>{/if}
  <p class="small" role="status">{notice}</p>

  {#if objectives}
    <section aria-labelledby="aims-h">
      <h2 id="aims-h">{t("p.aims.aims_heading")}</h2>
      <p class="small muted">{t("p.aims.aims_says")}</p>
      {#if objectives.length === 0}
        <p class="muted">{t("p.aims.no_objectives")} <a href={localeHref("/objectives")}>{t("p.aims.to_objectives")}</a></p>
      {:else}
        <fieldset class="aims">
          <legend>{t("p.aims.aims_legend")}</legend>
          {#each objectives as o (o.pid)}
            <label class="check">
              <input type="checkbox" checked={aimSet.has(o.pid)} disabled={busy} onchange={(e) => toggleAim(o, e.currentTarget.checked, e.currentTarget)} />
              {o.title}
            </label>
          {/each}
        </fieldset>
      {/if}
    </section>
  {/if}

  <section aria-labelledby="primary-h">
    <h2 id="primary-h">{t("p.aims.primary_heading")}</h2>
    <p class="small muted">{t("p.aims.primary_says")}</p>
    {#if programmes.length === 0 || aims.length === 0}
      <p class="muted">{programmes.length === 0 ? t("p.aims.no_programmes") : t("p.aims.no_aims")}</p>
    {:else}
      <form class="stack" novalidate onsubmit={setPrimary}>
        <label>
          {t("p.aims.f_programme")}
          <select bind:value={programme}>
            <option value="">{t("p.aims.choose_programme")}</option>
            {#each programmes as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
          </select>
        </label>
        <label>
          {t("p.aims.f_aim")}
          <select bind:value={aim}>
            <option value="">{t("p.aims.choose_aim")}</option>
            {#each aims as a (a.pid)}<option value={a.pid}>{a.title}</option>{/each}
          </select>
        </label>
        <div class="row">
          <button class="button primary" type="submit" disabled={busy}>{t("p.aims.set_primary")}</button>
        </div>
      </form>
    {/if}
  </section>
{/if}

<section aria-labelledby="live-h" data-testid="live-readings">
  <h2 id="live-h">{t("p.live_pack.heading")}</h2>
  <p class="small muted">{t("p.live_pack.says")}</p>
  <form class="row period" novalidate onsubmit={loadLive}>
    <label>
      {t("p.live_pack.f_ac")}
      <select bind:value={ac}>
        {#each AC_SOURCES as a (a)}<option value={a}>{t(AC_KEY[a])}</option>{/each}
      </select>
    </label>
    <label>
      {t("p.live_pack.f_window", { min: WINDOW_MIN, max: WINDOW_MAX })}
      <input type="number" min={WINDOW_MIN} max={WINDOW_MAX} step="1" bind:value={windowDays} />
    </label>
    <button class="button" type="submit">{t("p.live_pack.read_again")}</button>
  </form>
  {#if liveProblem}<p class="banner error" role="alert">{liveProblem}</p>{/if}
  {#each READINGS as r (r.id)}
    {#if liveErrors[r.id]}<p class="banner error" role="alert">{liveErrors[r.id]}</p>{/if}
    {#if r.id === "issues" && issuesUnavailable(live.issues)}
      <h3>{t("p.live_pack.issues_heading")}</h3>
      <p class="muted" data-testid="live-issues-none">{t("p.live_pack.issues_none")}</p>
    {/if}
    {#each r.shows as id (id)}
      {@const s = sections[id]}
      {#if s}
        <section aria-labelledby={`live-${id}`} data-testid={`live-${id}`}>
          <h3 id={`live-${id}`}>{s.title}</h3>
          {#each s.lines as line, i (i)}<p class="small">{line}</p>{/each}
          {#each s.tables as tb, ti (ti)}
            <DataTable label={tb.caption} caption={tb.caption} captionVisible columns={packColumns(tb.columns)} rows={tb.rows.map((cells) => ({ cells }))} rowId={(_r, i) => String(i)} sortable={false} />
          {/each}
          {#if s.lines.length === 0 && s.tables.length === 0}<p class="muted">{t("p.live_pack.nothing")}</p>{/if}
        </section>
      {/if}
    {/each}
  {/each}
</section>

<style>
  section {
    margin-block-start: 1.5rem;
  }
  .period {
    gap: 0.75rem;
    align-items: end;
  }
  .aims {
    display: grid;
    gap: 0.4rem;
  }
  .check {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }
  .check input {
    inline-size: 1.5rem;
    block-size: 1.5rem;
    margin: 0;
  }
  :global(.sym) {
    display: inline-block;
    inline-size: 1.25em;
  }
</style>
