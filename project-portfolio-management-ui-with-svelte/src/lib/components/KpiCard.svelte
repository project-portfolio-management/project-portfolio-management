<!--
  One KPI: its derived status (a word and a shape, never colour alone), latest
  reading, trend, staleness, thresholds, and the forms to add a reading, view
  the history, edit and delete. "No data yet" is its own state: not red, not
  green.
-->
<script lang="ts">
  import type { KpiView, KpiCadence, PpmClient, KpiReading } from "#lib/api/ppm.js";
  import { STATUS_SYMBOL, STATUS_TEXT, staleText, thresholdText, trendText, valueText } from "#lib/kpi-view.js";

  let {
    item,
    ppm,
    onchange,
    onerror,
  }: {
    item: KpiView;
    ppm: PpmClient;
    onchange: () => void;
    onerror: (message: string) => void;
  } = $props();

  const k = $derived(item.kpi);

  async function run(action: () => Promise<unknown>) {
    try {
      await action();
      onchange();
    } catch (err) {
      onerror(err instanceof Error ? err.message : "That did not work");
    }
  }

  let value = $state("");
  let observedOn = $state("");
  let note = $state("");
  const addReading = (e: SubmitEvent) => {
    e.preventDefault();
    void run(async () => {
      await ppm.addKpiReading(k.pid, {
        value: Number(value),
        observed_on: observedOn || undefined,
        note: note.trim() || null,
      });
      value = "";
      observedOn = "";
      note = "";
    });
  };

  let history = $state<KpiReading[] | null>(null);
  async function loadHistory(e: Event) {
    if (!(e.currentTarget as HTMLDetailsElement).open) return;
    try {
      history = (await ppm.kpi(k.pid)).history ?? [];
    } catch (err) {
      onerror(err instanceof Error ? err.message : "Could not load the history");
    }
  }

  let name = $state("");
  let target = $state("");
  let red = $state("");
  let cadence = $state<KpiCadence | "">("");
  let unit = $state("");
  $effect(() => {
    name = k.name;
    target = String(k.target_value);
    red = String(k.red_value);
    cadence = k.cadence ?? "";
    unit = k.unit ?? "";
  });
  const save = (e: SubmitEvent) => {
    e.preventDefault();
    void run(() =>
      ppm.updateKpi(k.pid, {
        name,
        description: k.description,
        unit: unit.trim() || null,
        target_value: Number(target),
        red_value: Number(red),
        cadence: cadence || null,
        owner_ref: k.owner_ref,
        objective_pid: k.objective_pid,
      }),
    );
  };
  const remove = () => {
    if (confirm(`Delete the KPI “${k.name}”? Its readings are kept.`)) void run(() => ppm.deleteKpi(k.pid));
  };
</script>

<article class={`surface kpi status-${item.status}`} aria-labelledby={`k-${k.pid}`}>
  <header>
    <h3 id={`k-${k.pid}`}>{k.name}</h3>
    <p class="status">
      <span aria-hidden="true">{STATUS_SYMBOL[item.status]}</span>
      <strong>{STATUS_TEXT[item.status]}</strong>
    </p>
  </header>
  <p class="small">
    {#if item.latest}
      {valueText(item.latest.value, k.unit)} on {item.latest.observed_on}
      {#if item.latest.note}· “{item.latest.note}”{/if}
    {:else}
      No reading yet.
    {/if}
    {#if trendText(item.trend)}· {trendText(item.trend)}{/if}
  </p>
  {#if staleText(item)}<p class="small stale">{staleText(item)}</p>{/if}
  <p class="small muted">{thresholdText(k)}{#if k.cadence} · read {k.cadence}{/if}{#if k.owner_ref} · {k.owner_ref}{/if}</p>

  <details>
    <summary>Add a reading</summary>
    <form onsubmit={addReading} class="inline">
      <label>Value <input type="number" step="1" required bind:value /></label>
      <label>Observed on <input type="date" bind:value={observedOn} /></label>
      <label>Note <input type="text" bind:value={note} /></label>
      <button class="button small" type="submit">Record</button>
    </form>
  </details>
  <details ontoggle={loadHistory}>
    <summary>History ({item.readings})</summary>
    {#if history === null}
      <p class="small muted">Loading…</p>
    {:else if history.length === 0}
      <p class="small muted">No readings.</p>
    {:else}
      <table class="small">
        <thead><tr><th scope="col">Observed</th><th scope="col">Value</th><th scope="col">Note</th></tr></thead>
        <tbody>
          {#each history as r (r.pid)}
            <tr><td>{r.observed_on}</td><td>{valueText(r.value, k.unit)}</td><td>{r.note ?? ""}</td></tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </details>
  <details>
    <summary>Edit or delete</summary>
    <form onsubmit={save} class="inline">
      <label>Name <input type="text" required bind:value={name} /></label>
      <label>Unit <input type="text" bind:value={unit} /></label>
      <label>Target <input type="number" step="1" required bind:value={target} /></label>
      <label>Red line <input type="number" step="1" required bind:value={red} /></label>
      <label>Cadence
        <select bind:value={cadence}>
          <option value="">None</option>
          <option value="daily">Daily</option>
          <option value="weekly">Weekly</option>
          <option value="monthly">Monthly</option>
          <option value="quarterly">Quarterly</option>
        </select>
      </label>
      <button class="button small" type="submit">Save</button>
      <button class="button small" type="button" onclick={remove}>Delete</button>
    </form>
    <p class="small muted">Which way is good ({k.direction === "higher_better" ? "higher" : "lower"}) cannot be changed: it would re-colour every past reading.</p>
  </details>
</article>

<style>
  .kpi header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 1rem;
  }
  .status {
    margin: 0;
    white-space: nowrap;
  }
  .status-red .status {
    color: var(--danger, #b00020);
  }
  .stale {
    font-style: italic;
  }
  .inline {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: end;
    margin: 0.5rem 0;
  }
  .inline label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
</style>
