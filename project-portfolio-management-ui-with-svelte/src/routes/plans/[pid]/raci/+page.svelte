<!--
  RACI matrix (`/plans/[pid]/raci`): who is Responsible, Accountable, Consulted
  and Informed on each task. Toggle a letter and the row is saved; a task has one
  Accountable person (giving it to someone moves it, and the page says so); a row
  with no Accountable or no Responsible is named, not hidden. Add a person with
  their `person:` / `worker:` reference; leave a person with no letters and they
  drop out of the matrix. Export to a spreadsheet from the link.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { API_BASE_URL } from "#lib/config.js";
  import { PpmClient, type RaciMatrix } from "#lib/api/ppm.js";
  import RaciMatrixTable from "#lib/components/RaciMatrix.svelte";
  import BridgeLink from "#lib/components/BridgeLink.svelte";
  import { LETTERS, LETTER_MEANING, LETTER_WORD, shortRef, toAssignments, validPersonRef } from "#lib/raci-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let matrix = $state<RaciMatrix | null>(null);
  let extra = $state<string[]>([]);
  let newPerson = $state("");
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);

  async function load() {
    try {
      matrix = await ppm.planRaci(pid);
      // A person added as an empty column stays until they hold a letter or the page reloads.
      extra = extra.filter((p) => !matrix?.people.includes(p));
      error = null;
    } catch (err) {
      error = errorText(err, "p.plans.pid.raci.could_not_load_who_does");
    }
  }
  onMount(() => void load());

  const people = $derived([...(matrix?.people ?? []), ...extra]);

  async function save(taskPid: string, cells: Record<string, string>, moved: string | null) {
    error = null;
    notice = moved;
    try {
      await ppm.setRaciRow(pid, taskPid, toAssignments(cells));
      await load();
    } catch (err) {
      error = errorText(err, "p.plans.pid.raci.could_not_save");
    }
  }

  function addPerson(event: SubmitEvent) {
    event.preventDefault();
    const ref = newPerson.trim();
    if (!validPersonRef(ref)) {
      error = t("p.plans.pid.raci.use_a_person_or_worker");
      return;
    }
    error = null;
    if (!people.includes(ref)) extra = [...extra, ref];
    newPerson = "";
  }

  const workloadColumns: DataGridColumn[] = $derived([
    { id: "person", header: t("p.plans.pid.raci.person"), rowHeader: true, accessor: (r) => shortRef(String(r.person_ref)), cell: personCell },
    { id: "responsible", header: LETTER_WORD.R, align: "end" },
    { id: "accountable", header: LETTER_WORD.A, align: "end" },
    { id: "consulted", header: LETTER_WORD.C, align: "end" },
    { id: "informed", header: LETTER_WORD.I, align: "end" },
  ]);
</script>

{#snippet personCell({ row, formatted }: DataGridCellContext)}{formatted} <BridgeLink ref={String(row.person_ref)} name={formatted} />{/snippet}

<svelte:head><title>{t("p.plans.pid.raci.raci_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.raci.plan")}</a></p>
<h1>{t("p.plans.pid.raci.who_does_what_raci_matrix")}</h1>
<p class="lede">{t("p.plans.pid.raci.for_each_task_who_does")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if notice}<p class="banner" role="status">{notice}</p>{/if}

<ul class="legend small" aria-label={t("p.plans.pid.raci.legend")}>
  {#each LETTERS as l (l)}<li><strong>{l}</strong> {LETTER_WORD[l]}: {LETTER_MEANING[l]}</li>{/each}
</ul>

{#if matrix}
  <p>
    {t("p.plans.pid.raci.of_tasks_are_complete_one", { complete: matrix.summary.complete, tasks: matrix.summary.tasks })}
    {#if matrix.summary.missing_accountable > 0}{t("p.plans.pid.raci.have_nobody_accountable", { missing_accountable: matrix.summary.missing_accountable })} {/if}
    {#if matrix.summary.missing_responsible > 0}{t("p.plans.pid.raci.have_nobody_responsible", { missing_responsible: matrix.summary.missing_responsible })}{/if}
    <a href={`${API_BASE_URL}/api/plans/${pid}/raci?format=csv`} download="raci.csv">{t("p.plans.pid.raci.export_to_a_spreadsheet")}</a>
  </p>

  <form onsubmit={addPerson} class="row">
    <label>{t("p.plans.pid.raci.add_a_person_person_or")} <input type="text" bind:value={newPerson} size="46" /></label>
    <button class="button primary" type="submit">{t("p.plans.pid.raci.add_column")}</button>
  </form>

  {#if matrix.rows.length === 0}
    <p class="muted">{t("p.plans.pid.raci.this_plan_has_no_tasks")}</p>
  {:else if people.length === 0}
    <p class="muted">{t("p.plans.pid.raci.add_a_person_to_start")}</p>
  {:else}
    <RaciMatrixTable {matrix} {people} onchange={(t, c, m) => void save(t, c, m)} />
  {/if}

  {#if matrix.workload.length > 0}
    <h2>{t("p.plans.pid.raci.load_by_person")}</h2>
    <DataTable label={t("p.plans.pid.raci.how_many_tasks_each_person")} columns={workloadColumns} rows={matrix.workload} rowId={(r) => String(r.person_ref)} />
  {/if}
{:else if !error}
  <p>{t("p.plans.pid.raci.loading")}</p>
{/if}

<style>
  .legend {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem 1.2rem;
    list-style: none;
    margin: 0.5rem 0;
    padding: 0;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 1rem;
    align-items: end;
    margin-block: 0.75rem;
  }
</style>
