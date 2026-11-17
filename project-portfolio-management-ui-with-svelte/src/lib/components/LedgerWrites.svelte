<!--
  The funding ledger's writes (`/ledger`, EX-12, NC-1 slice 11): record money a funding source allocated
  (append-only); draw a plan's budget lines on allocations of their own source and currency; map finance cost codes to
  budget lines; and reconcile a finance export against the register, which writes nothing. Every check the service
  makes is said first; a refusal is the catalogue's sentence, never the service's text.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tn } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { money } from "#lib/money.js";
  import { todayIn } from "#lib/timesheet-view.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import {
    PpmClient,
    type BudgetBoard,
    type FundingAllocation,
    type FundingSource,
    type ReconcileReport,
  } from "#lib/api/ppm.js";
  import {
    COST_CODE_MAX,
    REFERENCE_MAX,
    allocationBody,
    allocationChoices,
    costCodeProblem,
    exportProblem,
    ledgerRefusalKey,
  } from "#lib/ledger-actions.js";

  let { allocations, onChanged }: { allocations: FundingAllocation[]; onChanged: () => Promise<void> } = $props();

  const ppm = PpmClient.withFetch();
  const plansRepo = PlanRepository.withFetch();
  const tz = Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";

  let sources = $state<FundingSource[]>([]);
  let plans = $state<PlanRef[]>([]);
  let codes = $state<Array<{ cost_code: string; budget_line_pid: string }>>([]);
  let busy = $state(false);

  // Recording an allocation.
  let source = $state("");
  let amount = $state("");
  let currency = $state("GBP");
  let allocatedOn = $state("");
  let reference = $state("");
  let allocProblems = $state<string[]>([]);
  let allocError = $state<string | null>(null);
  let allocNotice = $state("");

  // A plan's lines and the allocations they draw on.
  let planPid = $state("");
  let lines = $state<BudgetBoard["lines"] | null>(null);
  let lineError = $state<string | null>(null);
  let lineNotice = $state("");

  // Cost codes.
  let code = $state("");
  let codeLine = $state("");
  let codeError = $state<string | null>(null);
  let codeNotice = $state("");

  // Reconciling a finance export.
  let csv = $state("");
  let report = $state<ReconcileReport | null>(null);
  let recError = $state<string | null>(null);

  const sourceName = (pid: string | null | undefined) => sources.find((s) => s.pid === pid)?.name ?? "";
  const allocLabel = (a: FundingAllocation) =>
    `${a.funding_source} · ${money(a.amount_minor, a.currency)} · ${formatDate(a.allocated_on)}${a.reference ? ` · ${a.reference}` : ""}`;
  const lineName = (pid: string) => lines?.find((l) => l.pid === pid)?.description ?? t("p.ledger.a_line");

  async function loadCodes() {
    try {
      codes = (await ppm.costCodes()).cost_codes;
    } catch {
      codes = [];
    }
  }
  onMount(() => {
    ppm.fundingSources().then((r) => (sources = r.funding_sources)).catch(() => (sources = []));
    plansRepo.list().then((p) => (plans = p)).catch(() => (plans = []));
    void loadCodes();
  });

  async function record(e: SubmitEvent) {
    e.preventDefault();
    allocError = null;
    allocNotice = "";
    const r = allocationBody({ source, amount, currency, allocatedOn, reference }, todayIn(tz));
    if ("problems" in r) {
      allocProblems = r.problems.map((k) => t(k));
      return;
    }
    allocProblems = [];
    busy = true;
    try {
      await ppm.recordAllocation(r.body);
      allocNotice = t("p.ledger.recorded");
      amount = "";
      reference = "";
      await onChanged();
    } catch (err) {
      allocError = errorText(err, "p.ledger.could_not_record");
    } finally {
      busy = false;
    }
  }

  async function loadLines(fresh = true) {
    if (fresh) {
      lineError = null;
      lineNotice = "";
      lines = null;
      codeLine = "";
    }
    if (!planPid) return;
    try {
      lines = (await ppm.budget(planPid)).lines;
    } catch (err) {
      lineError = errorText(err, "p.ledger.could_not_load_lines");
    }
  }
  async function draw(linePid: string, allocationPid: string, box: HTMLSelectElement, was: string) {
    lineError = null;
    lineNotice = "";
    busy = true;
    try {
      await ppm.setLineAllocation(planPid, linePid, allocationPid || null);
      lineNotice = allocationPid ? t("p.ledger.line_drawn") : t("p.ledger.line_detached");
      await loadLines(false);
      await onChanged();
    } catch (err) {
      const k = ledgerRefusalKey("line", err);
      lineError = k ? t(k) : errorText(err, "p.ledger.could_not_save");
      box.value = was;
    } finally {
      busy = false;
    }
  }

  async function mapCode(e: SubmitEvent) {
    e.preventDefault();
    codeError = null;
    codeNotice = "";
    const problem = costCodeProblem(code, codeLine);
    if (problem) {
      codeError = t(problem, { max: COST_CODE_MAX });
      return;
    }
    busy = true;
    try {
      await ppm.mapCostCode(code.trim(), codeLine);
      codeNotice = t("p.ledger.code_mapped");
      code = "";
      await loadCodes();
    } catch (err) {
      codeError = errorText(err, "p.ledger.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function readFile(e: Event) {
    const file = (e.currentTarget as HTMLInputElement).files?.[0];
    if (file) csv = await file.text();
  }
  async function reconcile(e: SubmitEvent) {
    e.preventDefault();
    recError = null;
    report = null;
    const problem = exportProblem(csv);
    if (problem) {
      recError = t(problem);
      return;
    }
    busy = true;
    try {
      report = await ppm.reconcileLedger(csv);
    } catch (err) {
      const k = ledgerRefusalKey("reconcile", err);
      recError = k ? t(k) : errorText(err, "p.ledger.could_not_reconcile");
    } finally {
      busy = false;
    }
  }
</script>

<section aria-labelledby="lg-record">
  <h2 id="lg-record">{t("p.ledger.record_heading")}</h2>
  <p class="small muted">{t("p.ledger.record_says")}</p>
  {#if allocError}<p class="banner error" role="alert">{allocError}</p>{/if}
  {#if allocProblems.length > 0}
    <ul class="banner error" role="alert">{#each allocProblems as p (p)}<li>{p}</li>{/each}</ul>
  {/if}
  <p class="small" role="status">{allocNotice}</p>
  <form class="grid" novalidate onsubmit={record}>
    <label>
      {t("p.ledger.f_source")}
      <select bind:value={source}>
        <option value="">{t("p.ledger.choose_source")}</option>
        {#each sources.filter((s) => s.active) as s (s.pid)}<option value={s.pid}>{s.name}</option>{/each}
      </select>
    </label>
    <label>{t("p.ledger.f_amount")}<input type="text" inputmode="decimal" bind:value={amount} /></label>
    <label>{t("p.ledger.f_currency")}<input type="text" bind:value={currency} maxlength="3" size="4" /></label>
    <label>{t("p.ledger.f_on")}<input type="date" bind:value={allocatedOn} /></label>
    <label>{t("p.ledger.f_reference")}<input type="text" bind:value={reference} maxlength={REFERENCE_MAX} /></label>
    <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.ledger.record")}</button></div>
  </form>
  {#if sources.length === 0}<p class="small"><a href={localeHref("/funding")}>{t("p.ledger.to_funding")}</a></p>{/if}
</section>

<section aria-labelledby="lg-lines">
  <h2 id="lg-lines">{t("p.ledger.lines_heading")}</h2>
  <p class="small muted">{t("p.ledger.lines_says")}</p>
  <label>
    {t("p.ledger.f_plan")}
    <select bind:value={planPid} onchange={() => loadLines()}>
      <option value="">{t("p.ledger.choose_plan")}</option>
      {#each plans as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
    </select>
  </label>
  {#if lineError}<p class="banner error" role="alert">{lineError}</p>{/if}
  <p class="small" role="status">{lineNotice}</p>
  {#if lines}
    {#if lines.length === 0}
      <p class="muted">{t("p.ledger.no_lines")}</p>
    {:else}
      <ul class="lines">
        {#each lines as l (l.pid)}
          {@const current = l.funding_allocation_pid ?? ""}
          {@const choices = allocationChoices(l, allocations)}
          <li>
            <span>{l.description} · {money(l.planned_minor, l.currency)} · {l.funding_source_pid ? sourceName(l.funding_source_pid) : t("p.ledger.no_source")}</span>
            {#if !l.funding_source_pid}
              <span class="small muted">{t("p.ledger.needs_source")}</span>
            {:else}
              <select
                aria-label={`${t("p.ledger.f_allocation")}: ${l.description}`}
                value={current}
                disabled={busy}
                onchange={(e) => draw(l.pid, e.currentTarget.value, e.currentTarget, current)}
              >
                <option value="">{t("p.ledger.no_allocation")}</option>
                {#each choices as a (a.pid)}<option value={a.pid}>{allocLabel(a)}</option>{/each}
              </select>
              {#if choices.length === 0}<span class="small muted">{t("p.ledger.no_matching_allocation")}</span>{/if}
            {/if}
          </li>
        {/each}
      </ul>
    {/if}
  {/if}
</section>

<section aria-labelledby="lg-codes">
  <h2 id="lg-codes">{t("p.ledger.codes_heading")}</h2>
  <p class="small muted">{t("p.ledger.codes_says")}</p>
  {#if codes.length === 0}
    <p class="muted small">{t("p.ledger.no_codes")}</p>
  {:else}
    <ul class="codes small">
      {#each codes as c (c.cost_code)}<li><code>{c.cost_code}</code> → {lineName(c.budget_line_pid)}</li>{/each}
    </ul>
  {/if}
  {#if codeError}<p class="banner error" role="alert">{codeError}</p>{/if}
  <p class="small" role="status">{codeNotice}</p>
  <form class="grid" novalidate onsubmit={mapCode}>
    <label>{t("p.ledger.f_code")}<input type="text" bind:value={code} maxlength={COST_CODE_MAX} /></label>
    <label>
      {t("p.ledger.f_code_line")}
      <select bind:value={codeLine} disabled={!lines || lines.length === 0}>
        <option value="">{lines ? t("p.ledger.choose_line") : t("p.ledger.choose_plan_first")}</option>
        {#each lines ?? [] as l (l.pid)}<option value={l.pid}>{l.description}</option>{/each}
      </select>
    </label>
    <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.ledger.map_code")}</button></div>
  </form>
</section>

<section aria-labelledby="lg-reconcile">
  <h2 id="lg-reconcile">{t("p.ledger.reconcile_heading")}</h2>
  <p class="small muted">{t("p.ledger.reconcile_says")}</p>
  {#if recError}<p class="banner error" role="alert">{recError}</p>{/if}
  <form class="stack" novalidate onsubmit={reconcile}>
    <label>{t("p.ledger.f_file")}<input type="file" accept=".csv,text/csv" onchange={readFile} /></label>
    <label>{t("p.ledger.f_csv")}<textarea bind:value={csv} rows="5"></textarea></label>
    <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.ledger.reconcile")}</button></div>
  </form>
  {#if report}
    <div role="status">
      <p><strong>{report.report.reconciled ? t("p.ledger.reconciled") : t("p.ledger.not_reconciled")}</strong></p>
      <p class="small">{tn("p.ledger.rows_read", report.rows, { n: report.rows })}</p>
    </div>
    {#if report.report.matched.length > 0}
      <h3>{t("p.ledger.matched_heading")}</h3>
      <ul class="small">
        {#each report.report.matched as m (m.cost_code)}
          <li>
            <code>{m.cost_code}</code>:
            {t("p.ledger.matched_line", {
              ledger: money(m.ledger_minor, m.currency),
              register: money(m.register_minor, m.currency),
              difference: money(m.difference_minor, m.currency),
            })}
          </li>
        {/each}
      </ul>
    {/if}
    {#if report.report.unmatched.length > 0}
      <h3>{t("p.ledger.unmatched_heading")}</h3>
      <ul class="small">
        {#each report.report.unmatched as u (`${u.cost_code}-${u.currency}`)}<li><code>{u.cost_code}</code>: {money(u.amount_minor, u.currency)}</li>{/each}
      </ul>
    {/if}
    {#if report.report.currency_mismatches.length > 0}
      <h3>{t("p.ledger.mismatch_heading")}</h3>
      <ul class="small">
        {#each report.report.currency_mismatches as x, i (i)}
          <li><code>{x.cost_code}</code>: {t("p.ledger.mismatch_line", { row: x.row_currency, line: x.line_currency, amount: money(x.amount_minor, x.row_currency) })}</li>
        {/each}
      </ul>
    {/if}
  {/if}
</section>

<style>
  section {
    margin-block-start: 2rem;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));
    gap: 0.75rem;
    align-items: end;
  }
  .grid label {
    display: grid;
    gap: 0.25rem;
  }
  .lines,
  .codes {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 0.5rem;
  }
  .lines li {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 0.75rem;
    align-items: center;
  }
</style>
