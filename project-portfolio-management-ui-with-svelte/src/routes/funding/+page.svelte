<!--
  Funding (`/funding`, DP-6, NC-1 slice 11): the owner-supplied list of funding sources (add one, retire or restore
  one; a retired source stays on the lines that have it), a plan's budget lines each with its source (or none), and
  programme money by funding source for a financial year, drawn as the director pack draws it (capital and revenue
  apart unless combined, currencies never summed, lines with no source counted and listed). The service decides who may
  change them; a refusal is the catalogue's sentence.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { errorText } from "#lib/error-text.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { i18n, t, tDynamic } from "#lib/i18n.svelte.js";
  import { money } from "#lib/money.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import { PpmClient, type BudgetBoard, type FundingSource } from "#lib/api/ppm.js";
  import { buildPackView, type Section } from "#lib/pack-view.js";
  import { SOURCE_NAME_MAX, fundingRefusalKey, fyProblem, lineChoices, sourceNameProblem } from "#lib/funding-view.js";

  const ppm = PpmClient.withFetch();
  const plansRepo = PlanRepository.withFetch();

  let sources = $state<FundingSource[] | null>(null);
  let plans = $state<PlanRef[]>([]);
  let loadError = $state<string | null>(null);
  let name = $state("");
  let error = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  let planPid = $state("");
  let lines = $state<BudgetBoard["lines"] | null>(null);
  let linesError = $state<string | null>(null);
  let lineNotice = $state("");

  let fy = $state("");
  let combine = $state(false);
  let moneyBody = $state<Record<string, unknown> | null>(null);
  let moneyError = $state<string | null>(null);

  const section: Section | null = $derived(
    moneyBody
      ? (buildPackView(
          { funding: moneyBody },
          { t: (k, v) => tDynamic(k, v), label: (g, v) => enumLabel(g as never, v), locale: i18n.locale },
        ).find((s) => s.id === "funding") ?? null)
      : null,
  );
  type PackCell = { text: string; symbol?: string; tone?: string };
  const cellOf = (row: unknown, i: number): PackCell => (row as { cells: PackCell[] }).cells[i] ?? { text: "" };
  const packColumns = (names: string[]): DataGridColumn[] =>
    names.map((n, i) => ({ id: `c${i}`, header: n, rowHeader: i === 0, accessor: (row) => cellOf(row, i).text, ...(i === 0 ? {} : { cell: packCell }) }));

  async function loadSources() {
    sources = (await ppm.fundingSources()).funding_sources;
  }
  async function loadMoney() {
    moneyError = null;
    const problem = fyProblem(fy);
    if (problem) {
      moneyError = t(problem);
      return;
    }
    try {
      moneyBody = await ppm.portfolioMoney(fy.trim() ? Number(fy) : undefined, combine);
    } catch (err) {
      moneyError = errorText(err, "p.funding.could_not_load_money");
    }
  }
  onMount(async () => {
    try {
      await loadSources();
    } catch (err) {
      loadError = errorText(err, "p.funding.could_not_load");
    }
    plansRepo.list().then((p) => (plans = p)).catch(() => (plans = []));
    void loadMoney();
  });

  async function add(e: SubmitEvent) {
    e.preventDefault();
    error = null;
    notice = "";
    const problem = sourceNameProblem(name, sources ?? []);
    if (problem) {
      error = t(problem);
      return;
    }
    busy = true;
    try {
      await ppm.createFundingSource(name.trim());
      notice = t("p.funding.added");
      name = "";
      await loadSources();
    } catch (err) {
      const k = fundingRefusalKey(err);
      error = k ? t(k) : errorText(err, "p.funding.could_not_save");
    } finally {
      busy = false;
    }
  }
  async function setActive(s: FundingSource, active: boolean) {
    error = null;
    notice = "";
    busy = true;
    try {
      await ppm.setFundingSourceActive(s.pid, active);
      notice = active ? t("p.funding.restored") : t("p.funding.retired");
      await loadSources();
    } catch (err) {
      error = errorText(err, "p.funding.could_not_save");
    } finally {
      busy = false;
    }
  }

  /** Choosing another plan starts afresh; reloading after a change keeps what was said about it. */
  async function loadLines(fresh = true) {
    if (fresh) {
      linesError = null;
      lineNotice = "";
      lines = null;
    }
    if (!planPid) return;
    try {
      lines = (await ppm.budget(planPid)).lines;
    } catch (err) {
      linesError = errorText(err, "p.funding.could_not_load_lines");
    }
  }
  async function setLine(linePid: string, sourcePid: string, box: HTMLSelectElement, was: string) {
    linesError = null;
    lineNotice = "";
    busy = true;
    try {
      await ppm.setLineFundingSource(planPid, linePid, sourcePid || null);
      lineNotice = sourcePid ? t("p.funding.line_set") : t("p.funding.line_cleared");
      await loadLines(false);
      void loadMoney();
    } catch (err) {
      const k = fundingRefusalKey(err);
      linesError = k ? t(k) : errorText(err, "p.funding.could_not_save");
      box.value = was; // nothing changed: show what the line still has
    } finally {
      busy = false;
    }
  }
</script>

{#snippet packCell({ row, column, formatted }: DataGridCellContext)}
  {@const c = cellOf(row, Number(column.id.slice(1)))}
  {#if c.symbol}<span aria-hidden="true" class="sym">{c.symbol}</span>{/if}{formatted}
{/snippet}

<svelte:head><title>{t("p.funding.title_ppm")}</title></svelte:head>

<h1>{t("p.funding.title")}</h1>
<p class="small muted">{t("p.funding.intro")}</p>
{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}

{#if sources}
  <section aria-labelledby="sources-h">
    <h2 id="sources-h">{t("p.funding.sources_heading")}</h2>
    {#if error}<p class="banner error" role="alert">{error}</p>{/if}
    <p class="small" role="status">{notice}</p>
    {#if sources.length === 0}
      <p class="muted">{t("p.funding.no_sources")}</p>
    {:else}
      <ul class="sources">
        {#each sources as s (s.pid)}
          <li>
            <span>{s.name}</span>
            <span class="small muted">{s.active ? t("p.funding.is_active") : t("p.funding.is_retired")}</span>
            <button class="button small" type="button" disabled={busy} onclick={() => setActive(s, !s.active)} aria-label={`${s.active ? t("p.funding.retire") : t("p.funding.restore")}: ${s.name}`}>
              {s.active ? t("p.funding.retire") : t("p.funding.restore")}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
    <p class="small muted">{t("p.funding.retire_says")}</p>
    <form class="row add" novalidate onsubmit={add}>
      <label>
        {t("p.funding.f_name")}
        <input type="text" bind:value={name} maxlength={SOURCE_NAME_MAX} />
      </label>
      <button class="button primary" type="submit" disabled={busy}>{t("p.funding.add")}</button>
    </form>
  </section>

  <section aria-labelledby="lines-h">
    <h2 id="lines-h">{t("p.funding.lines_heading")}</h2>
    <p class="small muted">{t("p.funding.lines_says")}</p>
    <label>
      {t("p.funding.f_plan")}
      <select bind:value={planPid} onchange={() => loadLines()}>
        <option value="">{t("p.funding.choose_plan")}</option>
        {#each plans as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
      </select>
    </label>
    {#if linesError}<p class="banner error" role="alert">{linesError}</p>{/if}
    <p class="small" role="status">{lineNotice}</p>
    {#if lines}
      {#if lines.length === 0}
        <p class="muted">{t("p.funding.no_lines")}</p>
      {:else}
        <ul class="lines">
          {#each lines as l (l.pid)}
            {@const current = l.funding_source_pid ?? ""}
            <li>
              <span>{l.description} · {enumLabel("pk_class", l.category)} · {money(l.planned_minor, l.currency)}</span>
              <select
                aria-label={`${t("p.funding.f_source")}: ${l.description}`}
                value={current}
                disabled={busy}
                onchange={(e) => setLine(l.pid, e.currentTarget.value, e.currentTarget, current)}
              >
                <option value="">{t("p.funding.no_source")}</option>
                {#each lineChoices(sources, current) as s (s.pid)}
                  <option value={s.pid}>{s.active ? s.name : t("p.funding.retired_option", { name: s.name })}</option>
                {/each}
              </select>
            </li>
          {/each}
        </ul>
      {/if}
    {/if}
  </section>
{/if}

<section aria-labelledby="money-h">
  <h2 id="money-h">{t("p.funding.money_heading")}</h2>
  <form class="row add" novalidate onsubmit={(e) => { e.preventDefault(); void loadMoney(); }}>
    <label>
      {t("p.funding.f_fy")}
      <input type="text" inputmode="numeric" bind:value={fy} size="6" />
    </label>
    <label class="check">
      <input type="checkbox" bind:checked={combine} />
      {t("p.funding.f_combine")}
    </label>
    <button class="button" type="submit">{t("p.funding.show")}</button>
  </form>
  {#if moneyError}<p class="banner error" role="alert">{moneyError}</p>{/if}
  {#if section}
    {#each section.lines as line, i (i)}<p>{line}</p>{/each}
    {#each section.tables as tb, ti (ti)}
      <DataTable label={tb.caption} caption={tb.caption} captionVisible columns={packColumns(tb.columns)} rows={tb.rows.map((cells) => ({ cells }))} rowId={(_r, i) => String(i)} sortable={false} />
    {/each}
  {/if}
</section>

<style>
  section {
    margin-block-start: 1.5rem;
  }
  .sources,
  .lines {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 0.5rem;
  }
  .sources li,
  .lines li {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 0.75rem;
    align-items: center;
  }
  .add {
    gap: 0.75rem;
    align-items: end;
    margin-block: 0.75rem;
  }
  .check {
    display: inline-flex;
    gap: 0.35rem;
    align-items: center;
  }
  :global(.sym) {
    display: inline-block;
    inline-size: 1.25em;
  }
</style>
