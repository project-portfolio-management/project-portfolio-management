<!--
  Import (`/plans/[pid]/import`): bring a Jira or Asana CSV export into this plan.
  The first step is always a preview that writes nothing and says what would be
  created, updated or left out; the same button then imports. Importing the same
  file again updates in place and never duplicates. Whatever cannot come across is
  listed with the reason, never dropped quietly.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { PpmClient, type TrackerImportReport } from "#lib/api/ppm.js";
  import { mspdiHint, SOURCE_ACCEPT, SOURCE_LABEL, actionLabel, summaryText, unmappedGroups, type ImportSource } from "#lib/import-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let source = $state<ImportSource>("jira");
  let csv = $state<string | null>(null);
  let fileName = $state("");
  let preview = $state<TrackerImportReport | null>(null);
  let done = $state<TrackerImportReport | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);

  async function choose(e: Event) {
    const f = (e.currentTarget as HTMLInputElement).files?.[0];
    preview = done = null;
    error = null;
    if (!f) return;
    fileName = f.name;
    csv = await f.text();
  }
  function reset() {
    csv = null;
    fileName = "";
    preview = done = null;
    error = null;
  }
  async function go(e: SubmitEvent) {
    e.preventDefault();
    if (csv === null) {
      error = t("p.plans.pid.import.choose_the_file_first");
      return;
    }
    busy = true;
    try {
      if (!preview) {
        preview = await ppm.importTracker(pid, source, csv, true);
      } else {
        done = await ppm.importTracker(pid, source, csv, false);
        preview = null;
      }
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.plans.pid.import.the_import_did_not_work");
    } finally {
      busy = false;
    }
  }
  const shown = $derived(done ?? preview);
</script>

<svelte:head><title>{t("p.plans.pid.import.import_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.import.plan")}</a></p>
<h1>{t("p.plans.pid.import.import")}</h1>
<p class="lede">{t("p.plans.pid.import.bring_tasks_in_from_jira")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}

<form class="surface stack" onsubmit={go} aria-labelledby="src-h">
  <h2 id="src-h">{t("p.plans.pid.import.where_from")}</h2>
  <label>
    {t("p.plans.pid.import.tracker")}
    <select bind:value={source} onchange={reset}>
      <option value="jira">{SOURCE_LABEL.jira}</option>
      <option value="asana">{SOURCE_LABEL.asana}</option>
      <option value="mspdi">{SOURCE_LABEL.mspdi}</option>
    </select>
  </label>
  <label>{t("p.plans.pid.import.file")} <input type="file" accept={SOURCE_ACCEPT[source]} onchange={choose} /></label>
  {#if source === "mspdi"}<p class="small muted">{mspdiHint()}</p>{/if}
  {#if fileName}<p class="small muted">{fileName}</p>{/if}
  <button class="button primary" type="submit" disabled={busy || csv === null}>{actionLabel(preview)}</button>
  {#if preview}<button type="button" class="button" onclick={reset}>{t("p.plans.pid.import.choose_another_file")}</button>{/if}
</form>

{#if shown}
  <section class="surface stack" aria-labelledby="report-h" data-testid="report">
    <h2 id="report-h">{shown.dry_run ? t("p.plans.pid.import.preview") : t("p.plans.pid.import.imported")}</h2>
    <p role="status">{summaryText(shown)}</p>
    <p class="small muted">{shown.note}</p>
    {#each unmappedGroups(shown.unmapped) as g (g.title)}
      <details>
        <summary>{g.title} ({g.reasons.length})</summary>
        <ul>{#each g.reasons as r}<li>{r}</li>{/each}</ul>
      </details>
    {/each}
  </section>
{/if}
