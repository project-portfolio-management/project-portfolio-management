<!--
  Triage and thresholds of a programme (`/plans/[pid]/programme`, EX-7, NC-1 slice 11). A triage is a dated outcome
  (finish, fix, pause or stop) with a reason; it is a view and changes nothing. A threshold (red in N issued packs in a
  row, the forecast finish past a date, the P80 cost above a cap) opens a delivery exception when it is crossed;
  nothing is paused, reset or stopped automatically. Every rule the service checks is said first.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tDynamic, tn } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { minorToInput, money } from "#lib/money.js";
  import { PpmClient } from "#lib/api/ppm.js";
  import {
    REASON_MIN,
    RED_PACKS_MAX,
    RED_PACKS_MIN,
    THRESHOLD_KINDS,
    TRIAGE_OUTCOMES,
    sameThreshold,
    thresholdBody,
    triageProblem,
    type Threshold,
    type ThresholdForm,
    type ThresholdKind,
    type Triage,
    type TriageOutcome,
  } from "#lib/threshold-view.js";

  let { pid }: { pid: string } = $props();
  const ppm = PpmClient.withFetch();

  let busy = $state(false);
  let triage = $state<Triage | null>(null);
  let history = $state<Triage[]>([]);
  let outcome = $state<TriageOutcome>("fix");
  let reason = $state("");
  let triageError = $state<string | null>(null);
  let triageNotice = $state("");

  let thresholds = $state<Threshold[]>([]);
  const empty = (): ThresholdForm => ({ count: "", date: "", amount: "", currency: "" });
  let forms = $state<Record<ThresholdKind, ThresholdForm>>({
    confidence_red_packs: empty(),
    forecast_past: empty(),
    p80_cap: empty(),
  });
  let thresholdError = $state<string | null>(null);
  let thresholdNotice = $state("");
  let loadError = $state<string | null>(null);
  // The forms are filled from the service once; until then they are disabled, so a late answer cannot overwrite
  // what the reader has typed.
  let loaded = $state(false);

  const current = (kind: ThresholdKind) => thresholds.find((x) => x.kind === kind);

  async function load() {
    try {
      const [tr, th] = await Promise.all([ppm.triage(pid), ppm.thresholds(pid)]);
      triage = tr.triage;
      history = tr.history;
      thresholds = th.thresholds;
      for (const kind of THRESHOLD_KINDS) {
        const x = current(kind);
        forms[kind] = x
          ? {
              count: x.count === null ? "" : String(x.count),
              date: x.date ?? "",
              amount: x.cap_minor === null || !x.currency ? "" : minorToInput(x.cap_minor, x.currency),
              currency: x.currency ?? "",
            }
          : empty();
      }
      loadError = null;
      loaded = true;
    } catch (err) {
      loadError = errorText(err, "p.thresholds.could_not_load");
    }
  }
  onMount(() => void load());

  async function saveTriage(e: SubmitEvent) {
    e.preventDefault();
    triageError = null;
    triageNotice = "";
    const problem = triageProblem(outcome, reason);
    if (problem) {
      triageError = t(problem, { min: REASON_MIN });
      return;
    }
    busy = true;
    try {
      await ppm.setTriage(pid, outcome, reason.trim());
      triageNotice = t("p.thresholds.triage_saved");
      reason = "";
      await load();
    } catch (err) {
      triageError = errorText(err, "p.thresholds.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function saveThreshold(e: SubmitEvent, kind: ThresholdKind) {
    e.preventDefault();
    thresholdError = null;
    thresholdNotice = "";
    const r = thresholdBody(kind, forms[kind]);
    if ("problem" in r) {
      thresholdError = t(r.problem, { min: RED_PACKS_MIN, max: RED_PACKS_MAX });
      return;
    }
    if (sameThreshold(current(kind), r.body)) {
      thresholdNotice = t("p.thresholds.unchanged");
      return;
    }
    busy = true;
    try {
      await ppm.setThreshold(pid, r.body);
      thresholdNotice = t("p.thresholds.threshold_saved", { kind: tDynamic(`p.thresholds.kind_${kind}`) });
      await load();
    } catch (err) {
      thresholdError = errorText(err, "p.thresholds.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function removeThreshold(kind: ThresholdKind) {
    thresholdError = null;
    thresholdNotice = "";
    busy = true;
    try {
      await ppm.setThreshold(pid, { kind, remove: true });
      thresholdNotice = t("p.thresholds.threshold_removed", { kind: tDynamic(`p.thresholds.kind_${kind}`) });
      await load();
    } catch (err) {
      thresholdError = errorText(err, "p.thresholds.could_not_save");
    } finally {
      busy = false;
    }
  }

  function describe(x: Threshold): string {
    if (x.kind === "confidence_red_packs" && x.count !== null) return tn("p.thresholds.now_red_packs", x.count, { n: x.count });
    if (x.kind === "forecast_past" && x.date) return t("p.thresholds.now_forecast", { date: formatDate(x.date) });
    if (x.kind === "p80_cap" && x.cap_minor !== null && x.currency) return t("p.thresholds.now_cap", { amount: money(x.cap_minor, x.currency) });
    return t("p.thresholds.not_set");
  }
</script>

<section aria-labelledby="triage-h">
  <h2 id="triage-h">{t("p.thresholds.triage_heading")}</h2>
  <p class="small muted">{t("p.thresholds.triage_says")}</p>
  {#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}
  {#if triage}
    <p>{t("p.thresholds.triage_latest", { date: formatDate(triage.triaged_on), outcome: tDynamic(`p.thresholds.outcome_${triage.outcome}`), reason: triage.reason })}</p>
  {:else}
    <p class="muted">{t("p.thresholds.triage_none")}</p>
  {/if}
  {#if triageError}<p class="banner error" role="alert">{triageError}</p>{/if}
  <p class="small" role="status">{triageNotice}</p>
  <form class="stack" novalidate onsubmit={saveTriage}>
    <label>
      {t("p.thresholds.f_outcome")}
      <select bind:value={outcome}>
        {#each TRIAGE_OUTCOMES as o (o)}<option value={o}>{tDynamic(`p.thresholds.outcome_${o}`)}</option>{/each}
      </select>
    </label>
    <label>{t("p.thresholds.f_reason", { min: REASON_MIN })}<textarea bind:value={reason} rows="2"></textarea></label>
    <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.thresholds.save_triage")}</button></div>
  </form>
  {#if history.length > 1}
    <h3>{t("p.thresholds.history")}</h3>
    <ul class="small">
      {#each history as h (h.pid)}
        <li>{t("p.thresholds.history_item", { date: formatDate(h.triaged_on), outcome: tDynamic(`p.thresholds.outcome_${h.outcome}`), reason: h.reason })}</li>
      {/each}
    </ul>
  {/if}
</section>

<section aria-labelledby="thresholds-h">
  <h2 id="thresholds-h">{t("p.thresholds.thresholds_heading")}</h2>
  <p class="small muted">{t("p.thresholds.thresholds_says")}</p>
  {#if thresholdError}<p class="banner error" role="alert">{thresholdError}</p>{/if}
  <p class="small" role="status">{thresholdNotice}</p>
  {#each THRESHOLD_KINDS as kind (kind)}
    {@const x = current(kind)}
    <form class="stack threshold" novalidate onsubmit={(e) => saveThreshold(e, kind)} aria-labelledby={`th-${kind}`}>
      <h3 id={`th-${kind}`}>{tDynamic(`p.thresholds.kind_${kind}`)}</h3>
      <p class="small">{x ? describe(x) : t("p.thresholds.not_set")}</p>
      {#if kind === "confidence_red_packs"}
        <label>{t("p.thresholds.f_count", { min: RED_PACKS_MIN, max: RED_PACKS_MAX })}<input type="number" inputmode="numeric" bind:value={forms[kind].count} disabled={!loaded} /></label>
      {:else if kind === "forecast_past"}
        <label>{t("p.thresholds.f_date")}<input type="date" bind:value={forms[kind].date} disabled={!loaded} /></label>
      {:else}
        <label>{t("p.thresholds.f_amount")}<input type="text" inputmode="decimal" bind:value={forms[kind].amount} disabled={!loaded} /></label>
        <label>{t("p.thresholds.f_currency")}<input type="text" maxlength="3" bind:value={forms[kind].currency} disabled={!loaded} /></label>
      {/if}
      <div class="row">
        <button class="button" type="submit" disabled={busy || !loaded}>{t("p.thresholds.save_threshold")}</button>
        {#if x}<button class="button" type="button" disabled={busy} onclick={() => removeThreshold(kind)}>{t("p.thresholds.remove_threshold")}</button>{/if}
      </div>
    </form>
  {/each}
</section>

<style>
  section {
    margin-block-start: 2rem;
  }
  form {
    max-width: 40rem;
  }
  .threshold {
    margin-block-start: 1rem;
  }
  h3 {
    font-size: 1rem;
    margin-block: 0;
  }
</style>
