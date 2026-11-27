<!--
  Many MS Project files at once (`/mspdi-import`, NC-13; NC-1 slice 12 part k): up to ten MSPDI XML files, each into a
  plan the reader chooses or into a new plan (named, else the project's name in the file), new plans under an optional
  parent. The first step is always a preview (`POST /api/import/mspdi-batch?dry_run=true`) that writes nothing; the same
  button then imports after a question that says it is not one transaction. Each project's outcome is a word, with the
  single-file importer's report drawn as `/plans/[pid]/import` draws it; the resources of all the files are listed once,
  never matched to people; links between the files are counted. A failed project's English error is not shown. Not
  offered: the custom-field map, the resource map and rates (said on the page). Linked from `/jobs`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatInteger } from "#lib/locale-format.js";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, mspdiReport, type MspdiBatchAnswer, type MspdiBatchProject } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import { summaryText, unmappedGroups } from "#lib/import-view.js";

  const ppm = PpmClient.withFetch();
  const plansRepo = PlanRepository.withFetch();
  const MAX = 10;
  const OUTCOMES = ["would_import", "imported", "failed", "not_attempted"];

  type Row = { file: string; xml: string; target: "new" | "existing"; name: string; plan: string };
  let rows = $state<Row[]>([]);
  let plans = $state<PlanRef[]>([]);
  let parent = $state("");
  let answer = $state<MspdiBatchAnswer | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);

  onMount(async () => {
    try {
      plans = await plansRepo.list();
    } catch {
      plans = []; // no existing plan to choose: every file can still go into a new plan
    }
  });

  async function choose(e: Event) {
    const files = [...((e.currentTarget as HTMLInputElement).files ?? [])];
    answer = null;
    error = null;
    if (files.length > MAX) {
      rows = [];
      error = t("p.mspdi_batch.too_many");
      return;
    }
    const read: Row[] = [];
    for (const f of files) {
      try {
        read.push({ file: f.name, xml: await f.text(), target: "new", name: "", plan: "" });
      } catch {
        error = t("p.mspdi_batch.read_failed", { file: f.name });
        rows = [];
        return;
      }
    }
    rows = read;
  }

  /** The body, or the first problem to say; nothing is sent while a problem stands. */
  function body(): { projects: MspdiBatchProject[]; parent_pid?: string } | { problem: string } {
    if (rows.length === 0) return { problem: t("p.mspdi_batch.none_chosen") };
    const projects: MspdiBatchProject[] = [];
    for (const r of rows) {
      if (r.target === "existing") {
        if (!r.plan) return { problem: t("p.mspdi_batch.choose_plan_first", { file: r.file }) };
        projects.push({ plan_pid: r.plan, xml: r.xml });
      } else {
        projects.push(r.name.trim() ? { name: r.name.trim(), xml: r.xml } : { xml: r.xml });
      }
    }
    return parent ? { projects, parent_pid: parent } : { projects };
  }

  async function go(e: SubmitEvent) {
    e.preventDefault();
    const b = body();
    if ("problem" in b) {
      error = b.problem;
      return;
    }
    const real = answer?.dry_run === true;
    if (real && !(await askConfirm(t("p.mspdi_batch.import_ask")))) return;
    busy = true;
    error = null;
    try {
      answer = await ppm.importMspdiBatch(b, !real);
    } catch (err) {
      error =
        err instanceof ApiError && err.status === 422 ? t("p.mspdi_batch.refused") : errorText(err, "p.mspdi_batch.could_not");
    } finally {
      busy = false;
    }
  }
  function reset() {
    rows = [];
    answer = null;
    error = null;
  }
  const outcome = (o: string) =>
    OUTCOMES.includes(o) ? tDynamic(`p.mspdi_batch.outcome_${o}`) : t("p.mspdi_batch.outcome_unknown");
  const unresolved = (a: MspdiBatchAnswer) => a.cross_project_links.unresolved?.length ?? 0;
</script>

<svelte:head><title>{t("p.mspdi_batch.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref("/jobs")}>{t("p.jobs.title")}</a></p>
<h1>{t("p.mspdi_batch.title")}</h1>
<p class="lede">{t("p.mspdi_batch.intro")}</p>
<p class="small muted">{t("p.mspdi_batch.not_offered")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}

<form class="surface stack" onsubmit={go} aria-label={t("p.mspdi_batch.title")}>
  <label>{t("p.mspdi_batch.files")} <input type="file" multiple accept=".xml,application/xml,text/xml" onchange={choose} /></label>
  {#each rows as r, i (r.file + i)}
    <fieldset>
      <legend>{r.file}</legend>
      <label>
        {t("p.mspdi_batch.target")}
        <select bind:value={r.target}>
          <option value="new">{t("p.mspdi_batch.target_new")}</option>
          <option value="existing">{t("p.mspdi_batch.target_existing")}</option>
        </select>
      </label>
      {#if r.target === "new"}
        <label>{t("p.mspdi_batch.new_name")} <input bind:value={r.name} maxlength="200" /></label>
      {:else}
        <label>
          {t("p.mspdi_batch.existing_plan")}
          <select bind:value={r.plan}>
            <option value="">{t("p.mspdi_batch.choose_plan")}</option>
            {#each plans as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
          </select>
        </label>
      {/if}
    </fieldset>
  {/each}
  {#if rows.some((r) => r.target === "new")}
    <label>
      {t("p.mspdi_batch.parent")}
      <select bind:value={parent}>
        <option value="">{t("p.mspdi_batch.no_parent")}</option>
        {#each plans as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
      </select>
    </label>
  {/if}
  <div class="row">
    <button class="button primary" type="submit" disabled={busy}>{answer?.dry_run ? t("p.mspdi_batch.import") : t("p.mspdi_batch.preview")}</button>
    {#if answer}<button class="button" type="button" onclick={reset}>{t("p.mspdi_batch.start_over")}</button>{/if}
  </div>
</form>

{#if answer}
  <section class="stack" aria-labelledby="result-h" data-testid="batch-result">
    <h2 id="result-h">{answer.dry_run ? t("p.mspdi_batch.result_preview_h") : t("p.mspdi_batch.result_done_h")}</h2>
    {#each answer.projects as p, i (i)}
      <article class="surface">
        <h3>{p.project} · {outcome(p.outcome)}</h3>
        {#if p.plan_created}<p class="small">{t("p.mspdi_batch.plan_created")}</p>
        {:else if p.plan_would_be_created}<p class="small">{t("p.mspdi_batch.new_plan_note")}</p>{/if}
        {#if p.plan_pid && !answer.dry_run}<p class="small"><a href={localeHref(`/plans/${p.plan_pid}`)}>{p.project}</a></p>{/if}
        {#if p.outcome === "failed"}<p><strong>{t("p.mspdi_batch.failed_note")}</strong></p>{/if}
        {#if p.report}
          {@const r = mspdiReport(p.report)}
          <p>{summaryText(r)}</p>
          {#each unmappedGroups(r.unmapped) as g (g.title)}
            <details>
              <summary>{g.title} ({g.reasons.length})</summary>
              <ul>{#each g.reasons as reason, j (j)}<li>{reason}</li>{/each}</ul>
            </details>
          {/each}
        {/if}
      </article>
    {/each}

    <h3>{t("p.mspdi_batch.links_h")}</h3>
    {#if answer.dry_run}
      <p>{t("p.mspdi_batch.links_would", { n: formatInteger(answer.cross_project_links.would_create ?? 0) })}</p>
    {:else if answer.failed_at !== null}
      <p>{t("p.mspdi_batch.links_after_failure")}</p>
    {:else}
      <p>{t("p.mspdi_batch.links_made", { n: formatInteger(answer.cross_project_links.created ?? 0), m: formatInteger(answer.cross_project_links.already_present ?? 0) })}</p>
    {/if}
    {#if unresolved(answer) > 0}<p class="small">{t("p.mspdi_batch.links_unresolved", { n: formatInteger(unresolved(answer)) })}</p>{/if}

    {#if answer.resource_pool.length > 0}
      <h3>{t("p.mspdi_batch.pool_h")}</h3>
      <p class="small muted">{t("p.mspdi_batch.pool_note")}</p>
      <ul>
        {#each answer.resource_pool as res, k (k)}<li>{t("p.mspdi_batch.pool_row", { name: res.name, assignments: formatInteger(res.assignments) })}</li>{/each}
      </ul>
    {/if}
  </section>
{/if}

<style>
  fieldset {
    border: 1px solid var(--mxi-color-border, currentColor);
    padding: 0.5rem 0.75rem;
  }
  label {
    display: block;
  }
  h3 {
    font-size: 1rem;
  }
</style>
