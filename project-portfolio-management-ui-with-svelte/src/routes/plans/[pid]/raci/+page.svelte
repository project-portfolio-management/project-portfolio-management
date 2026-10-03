<!--
  RACI matrix (`/plans/[pid]/raci`): who is Responsible, Accountable, Consulted
  and Informed on each task. Toggle a letter and the row is saved; a task has one
  Accountable person (giving it to someone moves it, and the page says so); a row
  with no Accountable or no Responsible is named, not hidden. Add a person with
  their `person:` / `worker:` reference; leave a person with no letters and they
  drop out of the matrix. Export to a spreadsheet from the link.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { API_BASE_URL } from "#lib/config.js";
  import { PpmClient, type RaciMatrix } from "#lib/api/ppm.js";
  import RaciMatrixTable from "#lib/components/RaciMatrix.svelte";
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
      error = err instanceof Error ? err.message : "Could not load the RACI matrix";
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
      error = err instanceof Error ? err.message : "Could not save";
    }
  }

  function addPerson(event: SubmitEvent) {
    event.preventDefault();
    const ref = newPerson.trim();
    if (!validPersonRef(ref)) {
      error = "Use a person: or worker: reference with a UUID.";
      return;
    }
    error = null;
    if (!people.includes(ref)) extra = [...extra, ref];
    newPerson = "";
  }
</script>

<svelte:head><title>RACI — PPM</title></svelte:head>

<p class="small"><a href={`/plans/${pid}`}>← Plan</a></p>
<h1>RACI matrix</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if notice}<p class="banner" role="status">{notice}</p>{/if}

<ul class="legend small" aria-label="Legend">
  {#each LETTERS as l (l)}<li><strong>{l}</strong> {LETTER_WORD[l]}: {LETTER_MEANING[l]}</li>{/each}
</ul>

{#if matrix}
  <p>
    {matrix.summary.complete} of {matrix.summary.tasks} tasks are complete (one accountable, at least one responsible).
    {#if matrix.summary.missing_accountable > 0}{matrix.summary.missing_accountable} have nobody accountable. {/if}
    {#if matrix.summary.missing_responsible > 0}{matrix.summary.missing_responsible} have nobody responsible.{/if}
    <a href={`${API_BASE_URL}/api/plans/${pid}/raci?format=csv`} download="raci.csv">Export to a spreadsheet</a>
  </p>

  <form onsubmit={addPerson} class="row">
    <label>Add a person (person:… or worker:…) <input type="text" bind:value={newPerson} size="46" /></label>
    <button type="submit">Add column</button>
  </form>

  {#if matrix.rows.length === 0}
    <p class="muted">This plan has no tasks yet.</p>
  {:else if people.length === 0}
    <p class="muted">Add a person to start assigning letters.</p>
  {:else}
    <RaciMatrixTable {matrix} {people} onchange={(t, c, m) => void save(t, c, m)} />
  {/if}

  {#if matrix.workload.length > 0}
    <h2>Load by person</h2>
    <table>
      <caption class="sr-only">How many tasks each person holds each letter on</caption>
      <thead>
        <tr><th scope="col">Person</th>{#each LETTERS as l (l)}<th scope="col">{LETTER_WORD[l]}</th>{/each}</tr>
      </thead>
      <tbody>
        {#each matrix.workload as w (w.person_ref)}
          <tr>
            <th scope="row" title={w.person_ref}>{shortRef(w.person_ref)}</th>
            <td>{w.responsible}</td><td>{w.accountable}</td><td>{w.consulted}</td><td>{w.informed}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
{:else if !error}
  <p>Loading…</p>
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
  table {
    border-collapse: collapse;
  }
  th,
  td {
    border: 1px solid var(--border, #ccc);
    padding: 0.25rem 0.5rem;
    text-align: start;
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }
</style>
