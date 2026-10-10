<!--
  One KPI: its derived status (a word and a shape, never colour alone), latest
  reading, trend, staleness, thresholds, and the forms to add a reading, view
  the history, edit and delete. "No data yet" is its own state: not red, not
  green.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
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
      onerror(err instanceof Error ? err.message : t("c.KpiCard.that_did_not_work"));
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
      onerror(err instanceof Error ? err.message : t("c.KpiCard.could_not_load_the_history"));
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
    if (confirm(t("c.KpiCard.delete_the_kpi_its_readings", { name: k.name }))) void run(() => ppm.deleteKpi(k.pid));
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
      {t("c.KpiCard.on", { valueText: valueText(item.latest.value, k.unit), observed_on: item.latest.observed_on })}
      {#if item.latest.note}· “{item.latest.note}”{/if}
    {:else}
      {t("c.KpiCard.no_reading_yet")}
    {/if}
    {#if trendText(item.trend)}· {trendText(item.trend)}{/if}
  </p>
  {#if staleText(item)}<p class="small stale">{staleText(item)}</p>{/if}
  <p class="small muted">{thresholdText(k)}{#if k.cadence} {t("c.KpiCard.read", { cadence: k.cadence })}{/if}{#if k.owner_ref} · {k.owner_ref}{/if}</p>

  <details>
    <summary>{t("c.KpiCard.add_a_reading")}</summary>
    <form onsubmit={addReading} class="inline">
      <label>{t("c.KpiCard.value")} <input type="number" step="1" required bind:value /></label>
      <label>{t("c.KpiCard.observed_on")} <input type="date" bind:value={observedOn} /></label>
      <label>{t("c.KpiCard.note")} <input type="text" bind:value={note} /></label>
      <button class="button small" type="submit">{t("c.KpiCard.record")}</button>
    </form>
  </details>
  <details ontoggle={loadHistory}>
    <summary>{t("c.KpiCard.history", { readings: item.readings })}</summary>
    {#if history === null}
      <p class="small muted">{t("c.KpiCard.loading")}</p>
    {:else if history.length === 0}
      <p class="small muted">{t("c.KpiCard.no_readings")}</p>
    {:else}
      <table class="small">
        <thead><tr><th scope="col">{t("c.KpiCard.observed")}</th><th scope="col">{t("c.KpiCard.value")}</th><th scope="col">{t("c.KpiCard.note")}</th></tr></thead>
        <tbody>
          {#each history as r (r.pid)}
            <tr><td>{r.observed_on}</td><td>{valueText(r.value, k.unit)}</td><td>{r.note ?? ""}</td></tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </details>
  <details>
    <summary>{t("c.KpiCard.edit_or_delete")}</summary>
    <form onsubmit={save} class="inline">
      <label>{t("c.KpiCard.name")} <input type="text" required bind:value={name} /></label>
      <label>{t("c.KpiCard.unit")} <input type="text" bind:value={unit} /></label>
      <label>{t("c.KpiCard.target")} <input type="number" step="1" required bind:value={target} /></label>
      <label>{t("c.KpiCard.red_line")} <input type="number" step="1" required bind:value={red} /></label>
      <label>{t("c.KpiCard.cadence")}
        <select bind:value={cadence}>
          <option value="">{t("c.KpiCard.none")}</option>
          <option value="daily">{t("c.KpiCard.daily")}</option>
          <option value="weekly">{t("c.KpiCard.weekly")}</option>
          <option value="monthly">{t("c.KpiCard.monthly")}</option>
          <option value="quarterly">{t("c.KpiCard.quarterly")}</option>
        </select>
      </label>
      <button class="button small" type="submit">{t("c.KpiCard.save")}</button>
      <button class="button small" type="button" onclick={remove}>{t("c.KpiCard.delete")}</button>
    </form>
    <p class="small muted">{t("c.KpiCard.which_way_is_good_cannot", { v: k.direction === "higher_better" ? t("c.KpiCard.direction_higher") : t("c.KpiCard.direction_lower") })}</p>
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
