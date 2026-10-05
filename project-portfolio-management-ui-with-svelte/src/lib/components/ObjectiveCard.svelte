<!--
  One objective on the OKR page: its derived score, its key results with
  progress (or why there is none), check-ins, and the forms to edit, add and
  delete. A score of `null` reads "Not measured" — never 0%. Confidence is shown
  beside a key result and is not part of any score.
-->
<script lang="ts">
  import type { OkrObjective, PpmClient, KeyResultMetric, KeyResultDirection } from "#lib/api/ppm.js";
  import { percentText, progressValue, keyResultProgress, valueText, confidenceText } from "#lib/okr-view.js";

  let {
    item,
    ppm,
    onchange,
    onerror,
  }: {
    item: OkrObjective;
    ppm: PpmClient;
    onchange: () => void;
    onerror: (message: string) => void;
  } = $props();

  const o = $derived(item.objective);
  const score = $derived(progressValue(item.score_basis_points));

  async function run(action: () => Promise<unknown>) {
    try {
      await action();
      onchange();
    } catch (err) {
      onerror(err instanceof Error ? err.message : "That did not work");
    }
  }

  // Edit the objective.
  let title = $state("");
  let period = $state("");
  $effect(() => {
    title = o.title;
    period = o.period ?? "";
  });
  const saveObjective = (e: SubmitEvent) => {
    e.preventDefault();
    void run(() => ppm.updateObjective(o.pid, { title, description: o.description, period: period.trim() || null }));
  };
  const removeObjective = () => {
    if (confirm(`Delete the objective “${o.title}” and its key results?`)) void run(() => ppm.deleteObjective(o.pid));
  };

  // Add a key result.
  let krTitle = $state("");
  let metric = $state<KeyResultMetric>("number");
  let direction = $state<KeyResultDirection>("increase");
  let start = $state("0");
  let target = $state("");
  let tolerance = $state("");
  let unit = $state("");
  let currency = $state("");
  const addKeyResult = (e: SubmitEvent) => {
    e.preventDefault();
    void run(async () => {
      await ppm.createKeyResult(o.pid, {
        title: krTitle,
        metric,
        direction,
        start_value: Number(start),
        target_value: Number(target),
        tolerance: tolerance === "" ? null : Number(tolerance),
        unit: unit.trim() || null,
        currency: metric === "currency" ? currency.trim() || null : null,
      });
      krTitle = "";
      target = "";
      tolerance = "";
    });
  };

  // Check in and edit, per key result (kept in small keyed records).
  let checkValue = $state<Record<string, string>>({});
  let checkConfidence = $state<Record<string, string>>({});
  let checkNote = $state<Record<string, string>>({});
  const checkIn = (pid: string) => (e: SubmitEvent) => {
    e.preventDefault();
    void run(async () => {
      await ppm.checkIn(pid, {
        value: Number(checkValue[pid]),
        confidence: checkConfidence[pid] ? Number(checkConfidence[pid]) : null,
        note: checkNote[pid]?.trim() || null,
      });
      checkValue[pid] = "";
      checkConfidence[pid] = "";
      checkNote[pid] = "";
    });
  };
  let editTitle = $state<Record<string, string>>({});
  let editTarget = $state<Record<string, string>>({});
  const saveKeyResult = (pid: string, tol: number | null, krUnit: string | null) => (e: SubmitEvent) => {
    e.preventDefault();
    void run(() =>
      ppm.updateKeyResult(pid, {
        title: editTitle[pid],
        target_value: Number(editTarget[pid]),
        tolerance: tol,
        unit: krUnit,
      }),
    );
  };
</script>

<article class="surface objective" aria-labelledby={`o-${o.pid}`}>
  <header>
    <h3 id={`o-${o.pid}`}>{o.title}</h3>
    {#if o.period}<span class="chip">{o.period}</span>{/if}
  </header>

  <p class="small">
    <strong>{percentText(item.score_basis_points)}</strong>
    {#if score !== null}
      <progress max="100" value={score} aria-label={`${o.title}: ${percentText(item.score_basis_points)}`}></progress>
    {:else}
      <span class="muted"> — it has no measurable key result yet, so it has no score (not zero).</span>
    {/if}
  </p>
  {#if !item.single_currency}
    <p class="small muted">Key results in more than one currency are scored separately; currencies are never converted.</p>
  {/if}

  {#if item.key_results.length === 0}
    <p class="small muted">No key results yet.</p>
  {:else}
    <ul class="krs">
      {#each item.key_results as kr (kr.key_result.pid)}
        {@const k = kr.key_result}
        <li>
          <p>
            <strong>{k.title}</strong>
            <span class="small muted"> · {k.direction}</span>
          </p>
          <p class="small">
            {keyResultProgress(kr)}
            {#if kr.progress_basis_points !== null}
              <progress max="100" value={progressValue(kr.progress_basis_points) ?? 0} aria-label={`${k.title}: ${percentText(kr.progress_basis_points)}`}></progress>
            {/if}
          </p>
          <p class="small muted">
            {valueText(k.current_value, k.target_value, k.unit)} (from {k.start_value})
            {#if k.tolerance !== null} · tolerance ±{k.tolerance}{/if}
            {#if kr.latest_check_in}
              · last check-in {kr.latest_check_in.observed_at.slice(0, 10)}
              {#if confidenceText(kr.latest_check_in.confidence)}· {confidenceText(kr.latest_check_in.confidence)}{/if}
              {#if kr.latest_check_in.note}· “{kr.latest_check_in.note}”{/if}
            {/if}
          </p>
          <details>
            <summary>Check in</summary>
            <form onsubmit={checkIn(k.pid)} class="inline">
              <label>Value
                <input type="number" step="1" required bind:value={checkValue[k.pid]} />
              </label>
              <label>Confidence (0–100)
                <input type="number" min="0" max="100" step="1" bind:value={checkConfidence[k.pid]} />
              </label>
              <label>Note
                <input type="text" bind:value={checkNote[k.pid]} />
              </label>
              <button class="button small" type="submit">Record</button>
            </form>
          </details>
          <details>
            <summary>Edit or delete</summary>
            <form
              onsubmit={saveKeyResult(k.pid, k.tolerance, k.unit)}
              class="inline"
              onfocusin={() => {
                editTitle[k.pid] ??= k.title;
                editTarget[k.pid] ??= String(k.target_value);
              }}
            >
              <label>Title
                <input type="text" required bind:value={editTitle[k.pid]} placeholder={k.title} />
              </label>
              <label>Target
                <input type="number" step="1" required bind:value={editTarget[k.pid]} placeholder={String(k.target_value)} />
              </label>
              <button class="button small" type="submit">Save</button>
              <button class="button small" type="button" onclick={() => confirm(`Delete “${k.title}”?`) && void run(() => ppm.deleteKeyResult(k.pid))}>Delete</button>
            </form>
            <p class="small muted">The baseline, metric and direction cannot be changed: progress from a moving baseline is not progress.</p>
          </details>
        </li>
      {/each}
    </ul>
  {/if}

  <details>
    <summary>Add a key result</summary>
    <form onsubmit={addKeyResult} class="inline">
      <label>Title <input type="text" required bind:value={krTitle} /></label>
      <label>Metric
        <select bind:value={metric}>
          <option value="number">Number</option>
          <option value="percent">Percent</option>
          <option value="currency">Currency</option>
          <option value="boolean">Yes / no</option>
        </select>
      </label>
      <label>Direction
        <select bind:value={direction}>
          <option value="increase">Increase</option>
          <option value="decrease">Decrease</option>
          <option value="maintain">Maintain</option>
        </select>
      </label>
      <label>Start <input type="number" step="1" required bind:value={start} /></label>
      <label>Target <input type="number" step="1" required bind:value={target} /></label>
      {#if direction === "maintain"}
        <label>Tolerance <input type="number" min="0" step="1" required bind:value={tolerance} /></label>
      {/if}
      <label>Unit <input type="text" bind:value={unit} /></label>
      {#if metric === "currency"}
        <label>Currency (e.g. GBP) <input type="text" required maxlength="3" bind:value={currency} /></label>
      {/if}
      <button class="button small" type="submit">Add key result</button>
    </form>
  </details>

  <details>
    <summary>Edit or delete this objective</summary>
    <form onsubmit={saveObjective} class="inline">
      <label>Title <input type="text" required bind:value={title} /></label>
      <label>Period <input type="text" bind:value={period} placeholder="2026-Q4" /></label>
      <button class="button small" type="submit">Save</button>
      <button class="button small" type="button" onclick={removeObjective}>Delete</button>
    </form>
  </details>
</article>

<style>
  .objective header {
    display: flex;
    align-items: baseline;
    gap: 0.75rem;
  }
  .krs {
    list-style: none;
    padding: 0;
  }
  .krs li {
    border-top: 1px solid var(--mxi-color-border, #ddd);
    padding: 0.5rem 0;
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
  progress {
    vertical-align: middle;
    width: 8rem;
    margin-left: 0.5rem;
  }
</style>
