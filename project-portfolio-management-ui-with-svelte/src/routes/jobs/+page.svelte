<!--
  Bulk imports and exports (`/jobs`, T-8; NC-1 slice 12 part k): the caller's jobs newest first (an administrator's:
  everyone's), each state as a word and a symbol, the counts and whether they add up, the error report and an
  export's file as links; a queued or running job cancelled after a question; an export or an import started.
  Rules in `src/lib/jobs-view.ts`. The service's English (`note`, `failure_reason`) is never shown. The review
  queue for held rows has no screen yet, and the page says so.
-->
<script lang="ts">
  import { localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatDateTime, formatPercentValue } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { API_BASE_URL } from "#lib/config.js";
  import { onMount } from "svelte";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type BulkJob, type BulkJobKind, type BulkJobStatus } from "#lib/api/ppm.js";
  import { STATUSES, cancellable, decisionRefusalKey, exportBody, formatOfName, heldName, matchText, orderJobs, refusalKey, rowsText, stateOf, type ExportForm } from "#lib/jobs-view.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import type { ReviewQueueItem } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let jobs = $state<BulkJob[] | null>(null);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);
  let kind = $state<"" | BulkJobKind>("");
  let status = $state<"" | BulkJobStatus>("");
  let busy = $state(false);

  const statusOf = (err: unknown) => (err instanceof ApiError ? err.status : undefined);
  const say = (action: "export" | "import" | "cancel" | "load", err: unknown) =>
    statusOf(err) !== undefined && statusOf(err)! < 500 && refusalKey(action, statusOf(err)) !== `p.jobs.could_not_${action}`
      ? tDynamic(refusalKey(action, statusOf(err)))
      : errorText(err, `p.jobs.could_not_${action}` as Parameters<typeof errorText>[1]);

  async function load() {
    try {
      jobs = orderJobs(await ppm.bulkJobs({ kind: kind || undefined, status: status || undefined }));
      error = null;
    } catch (err) {
      error = say("load", err);
    }
  }
  onMount(() => {
    void load();
    void loadHeld();
  });

  // Rows held for review: all, or one import's.
  let held = $state<ReviewQueueItem[] | null>(null);
  let heldError = $state<string | null>(null);
  let heldJob = $state<string | null>(null);
  async function loadHeld() {
    try {
      held = await ppm.reviewQueue(heldJob ?? undefined);
      heldError = null;
    } catch (err) {
      heldError = errorText(err, "p.jobs.could_not_load_review");
    }
  }
  function showHeld(job: string | null) {
    heldJob = job;
    void loadHeld();
  }
  async function decide(i: ReviewQueueItem, create: boolean) {
    const name = heldName(i);
    if (!(await askConfirm(t(create ? "p.jobs.review_create_ask" : "p.jobs.review_discard_ask", { name })))) return;
    try {
      if (create) await ppm.confirmQueueItem(i.pid);
      else await ppm.rejectQueueItem(i.pid);
      notice = t(create ? "p.jobs.review_created" : "p.jobs.review_discarded", { name });
      heldError = null;
      await Promise.all([loadHeld(), load()]);
    } catch (err) {
      const status = statusOf(err);
      const message = status !== undefined && status < 500 && decisionRefusalKey(status) !== "p.jobs.could_not_decide" ? tDynamic(decisionRefusalKey(status)) : errorText(err, "p.jobs.could_not_decide");
      // Read the rows again (the row may have been decided elsewhere), then say why this decision was refused.
      await loadHeld();
      heldError = message;
    }
  }

  async function cancel(j: BulkJob) {
    if (!(await askConfirm(t("p.jobs.cancel_ask")))) return;
    try {
      const r = await ppm.cancelBulkJob(j.job_id);
      notice = r.changed ? null : t("p.jobs.nothing_to_cancel");
      error = null;
      await load();
    } catch (err) {
      error = say("cancel", err);
    }
  }

  let form = $state<ExportForm>({ format: "jsonl", full: false, kind: "", name: "" });
  async function startExport(e: SubmitEvent) {
    e.preventDefault();
    busy = true;
    try {
      await ppm.startExport(exportBody(form));
      notice = t("p.jobs.queued_ok");
      error = null;
      await load();
    } catch (err) {
      error = say("export", err);
    } finally {
      busy = false;
    }
  }

  let file = $state<File | null>(null);
  let importFormat = $state<"jsonl" | "csv">("jsonl");
  let onKeyless = $state<"review" | "create">("review");
  function choose(e: Event) {
    const f = (e.currentTarget as HTMLInputElement).files?.[0] ?? null;
    file = f;
    if (f) importFormat = formatOfName(f.name);
  }
  async function startImport(e: SubmitEvent) {
    e.preventDefault();
    if (!file) {
      error = t("p.jobs.no_file");
      return;
    }
    busy = true;
    try {
      const r = await ppm.startImport(file, importFormat, onKeyless);
      notice = r.replayed ? t("p.jobs.replayed") : t("p.jobs.queued_ok");
      error = null;
      await load();
    } catch (err) {
      error = say("import", err);
    } finally {
      busy = false;
    }
  }
  const kindWord = (k: string) => (k === "import" ? t("p.jobs.kind_import") : t("p.jobs.kind_export"));
  const formatWord = (f: string) => (f === "csv" ? t("p.reports.csv") : t("p.jobs.fmt_jsonl"));
</script>

<svelte:head><title>{t("p.jobs.title_ppm")}</title></svelte:head>

<h1>{t("p.jobs.title")}</h1>
<p class="lede">{t("p.jobs.intro")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="jobs-notice">{notice}</p>{/if}

<form class="row" onsubmit={(e) => { e.preventDefault(); void load(); }} aria-label={t("p.jobs.title")}>
  <label>
    {t("p.jobs.filter_kind")}
    <select bind:value={kind} onchange={load}>
      <option value="">{t("p.jobs.any")}</option>
      <option value="import">{t("p.jobs.kind_import")}</option>
      <option value="export">{t("p.jobs.kind_export")}</option>
    </select>
  </label>
  <label>
    {t("p.jobs.filter_status")}
    <select bind:value={status} onchange={load}>
      <option value="">{t("p.jobs.any")}</option>
      {#each STATUSES as s (s)}<option value={s}>{stateOf(s).text}</option>{/each}
    </select>
  </label>
  <button class="button small" type="submit">{t("p.jobs.refresh")}</button>
</form>

{#if jobs === null && !error}
  <p class="muted">{t("p.jobs.loading")}</p>
{:else if jobs && jobs.length === 0}
  <p data-testid="jobs-none">{t("p.jobs.none")}</p>
{:else if jobs}
  <ul class="stack jobs" data-testid="jobs">
    {#each jobs as j (j.job_id)}
      {@const s = stateOf(j.status)}
      <li class="surface">
        <h2>{kindWord(j.kind)} · {formatWord(j.format)}</h2>
        <p data-testid="job-state"><span aria-hidden="true">{s.symbol}</span> {s.text}{j.status === "running" ? ` · ${t("p.jobs.progress", { percent: formatPercentValue(j.progress) })}` : ""}</p>
        <p class="small muted">
          {j.started_at ? t("p.jobs.started", { at: formatDateTime(j.started_at) }) : t("p.jobs.not_started")}{j.finished_at ? ` · ${t("p.jobs.finished", { at: formatDateTime(j.finished_at) })}` : ""}
        </p>
        <p>{rowsText(j)}</p>
        {#if !j.reconciles}<p class="small"><strong>{t("p.jobs.not_reconciled")}</strong></p>{/if}
        {#if j.counts.queued_for_review > 0}<p class="small">{t("p.jobs.held")}</p>{/if}
        {#if j.cancel_requested && j.status === "running"}<p class="small">{t("p.jobs.stopping")}</p>{/if}
        <p class="row">
          {#if j.errors_url}<a href={`${API_BASE_URL}${j.errors_url}?format=csv`}>{t("p.jobs.errors")}</a>{/if}
          {#if j.download_url}<a href={`${API_BASE_URL}${j.download_url}`}>{t("p.jobs.download")}</a>{/if}
          {#if j.counts.queued_for_review > 0}
            <button type="button" class="button small" onclick={() => showHeld(j.job_id)}>{t("p.jobs.review_show")}</button>
          {/if}
          {#if cancellable(j)}
            <button type="button" class="button small danger" onclick={() => cancel(j)} aria-label={t("p.jobs.cancel_named", { kind: kindWord(j.kind) })}>{t("p.jobs.cancel")}</button>
          {/if}
        </p>
      </li>
    {/each}
  </ul>
{/if}

<section class="surface stack" aria-labelledby="held-h" data-testid="held">
  <h2 id="held-h">{t("p.jobs.review_h")}</h2>
  <p class="small muted">{t("p.jobs.review_intro")}</p>
  {#if heldError}<p class="banner error" role="alert">{heldError}</p>{/if}
  {#if heldJob}
    <p class="small">{t("p.jobs.review_one_job")} <button type="button" class="button small" onclick={() => showHeld(null)}>{t("p.jobs.review_all")}</button></p>
  {/if}
  {#if held && held.length === 0}
    <p data-testid="held-none">{t("p.jobs.review_none")}</p>
  {:else if held}
    <ul class="stack jobs">
      {#each held as i (i.pid)}
        <li class="surface">
          <h3 id={`held-${i.pid}`}>{heldName(i)}{i.candidate.kind ? ` · ${enumLabel("kind", String(i.candidate.kind))}` : ""}</h3>
          <p class="small muted">{t("p.jobs.review_line", { line: i.source_line })}</p>
          {#if i.matches.length > 0}
            <p>{t("p.jobs.review_matches")}</p>
            <ul>
              {#each i.matches as m (m.pid)}
                <li><a href={localeHref(`/plans/${m.pid}`)}>{m.name}</a>: {matchText(m)}</li>
              {/each}
            </ul>
          {/if}
          <p class="row">
            <button type="button" class="button small" onclick={() => decide(i, true)} aria-describedby={`held-${i.pid}`}>{t("p.jobs.review_create")}</button>
            <button type="button" class="button small danger" onclick={() => decide(i, false)} aria-describedby={`held-${i.pid}`}>{t("p.jobs.review_discard")}</button>
          </p>
        </li>
      {/each}
    </ul>
    {#if held.length >= 500}<p class="small">{t("p.jobs.review_more")}</p>{/if}
  {/if}
</section>

<form class="surface stack" onsubmit={startExport} aria-labelledby="export-h">
  <h2 id="export-h">{t("p.jobs.export_h")}</h2>
  <p class="small muted">{t("p.jobs.export_rules")}</p>
  <label>
    {t("p.jobs.format")}
    <select bind:value={form.format}>
      <option value="jsonl">{t("p.jobs.fmt_jsonl")}</option>
      <option value="csv">{t("p.reports.csv")}</option>
    </select>
  </label>
  <label><input type="checkbox" bind:checked={form.full} /> {t("p.jobs.full")}</label>
  <label>{t("p.jobs.kind_filter")} <input bind:value={form.kind} maxlength="60" /></label>
  <label>{t("p.jobs.name_filter")} <input bind:value={form.name} maxlength="200" /></label>
  <button class="button primary" type="submit" disabled={busy}>{t("p.jobs.start_export")}</button>
</form>

<form class="surface stack" onsubmit={startImport} aria-labelledby="import-h">
  <h2 id="import-h">{t("p.jobs.import_h")}</h2>
  <p class="small muted">{t("p.jobs.import_rules")}</p>
  <label>{t("p.jobs.file")} <input type="file" accept=".jsonl,.ndjson,.json,.csv,text/csv,application/x-ndjson" onchange={choose} /></label>
  <label>
    {t("p.jobs.format")}
    <select bind:value={importFormat}>
      <option value="jsonl">{t("p.jobs.fmt_jsonl")}</option>
      <option value="csv">{t("p.reports.csv")}</option>
    </select>
  </label>
  <fieldset>
    <legend>{t("p.jobs.keyless")}</legend>
    <label><input type="radio" name="keyless" value="review" bind:group={onKeyless} /> {t("p.jobs.keyless_review")}</label>
    <label><input type="radio" name="keyless" value="create" bind:group={onKeyless} /> {t("p.jobs.keyless_create")}</label>
  </fieldset>
  <button class="button" type="submit" disabled={busy}>{t("p.jobs.start_import")}</button>
</form>

<style>
  .jobs {
    list-style: none;
    padding: 0;
  }
  h2 {
    font-size: 1.1rem;
  }
  h3 {
    font-size: 1rem;
  }
  label {
    display: block;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: end;
  }
</style>
