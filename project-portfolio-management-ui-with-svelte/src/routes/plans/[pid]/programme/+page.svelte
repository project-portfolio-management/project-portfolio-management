<!--
  A plan's gates and programme status (`/plans/[pid]/programme`, DP-5, NC-1 slice 11): the date the plan expects its
  review at each gate, and, for a plan of kind Program, its status (active, paused or stopped), the decisions behind
  it, and what a paused or stopped programme still holds. A decision pauses, resumes or stops it with a reason, citing a
  gate review or an exception of the programme if it follows from one. Stopped is final and keeps the record; a pause
  flags allocations and deletes none. The service decides who may; a refusal is the catalogue's sentence.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { errorText } from "#lib/error-text.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { localeHref, t, tn } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { money } from "#lib/money.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import {
    PLAN_GATES,
    PpmClient,
    type DeliveryException,
    type GateJourney,
    type PlannedGates,
    type ProgrammeStatusView,
  } from "#lib/api/ppm.js";
  import {
    CHANGE_LABELS,
    REASON_MIN,
    changedGates,
    decisionProblems,
    programmeRefusalKey,
    statusChoices,
  } from "#lib/programme-view.js";

  const ppm = PpmClient.withFetch();
  const plans = PlanRepository.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let isProgramme = $state<boolean | null>(null);
  let gates = $state<PlannedGates | null>(null);
  let dates = $state<Record<string, string>>({});
  let status = $state<ProgrammeStatusView | null>(null);
  let reviews = $state<GateJourney["reviews"]>([]);
  let exceptions = $state<DeliveryException[]>([]);
  let loadError = $state<string | null>(null);

  let gateError = $state<string | null>(null);
  let gateNotice = $state("");
  let error = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  let next = $state("");
  let reason = $state("");
  let reviewPid = $state("");
  let exceptionPid = $state("");

  const choices = $derived(status ? statusChoices(status.status) : []);

  async function loadGates() {
    gates = await ppm.plannedGates(pid);
    dates = Object.fromEntries(PLAN_GATES.map((g) => [g, gates?.planned_gates.find((x) => x.gate === g)?.planned_on ?? ""]));
  }
  async function loadStatus() {
    status = await ppm.programmeStatus(pid);
  }
  async function load() {
    loadError = null;
    try {
      const plan = await plans.get(pid);
      isProgramme = plan.kind === "Program";
      await loadGates();
      if (isProgramme) {
        await loadStatus();
        // What a decision may cite: best effort, the form works without them.
        ppm.gateJourney(pid).then((j) => (reviews = j.reviews)).catch(() => (reviews = []));
        ppm
          .deliveryExceptions()
          .then((r) => (exceptions = r.exceptions.filter((x) => x.plan_pid === pid)))
          .catch(() => (exceptions = []));
      }
    } catch (err) {
      loadError = errorText(err, "p.programme.could_not_load");
    }
  }
  onMount(load);

  async function saveGates(e: SubmitEvent) {
    e.preventDefault();
    gateError = null;
    gateNotice = "";
    const changed = changedGates(dates, gates?.planned_gates ?? []);
    if (changed.length === 0) {
      gateNotice = t("p.programme.gates_unchanged");
      return;
    }
    busy = true;
    try {
      for (const [gate, date] of changed) await ppm.setPlannedGate(pid, gate, date);
      gateNotice = t("p.programme.gates_saved");
      await loadGates();
    } catch (err) {
      gateError = errorText(err, "p.programme.could_not_save_gates");
      await loadGates().catch(() => {});
    } finally {
      busy = false;
    }
  }

  async function decide(e: SubmitEvent) {
    e.preventDefault();
    error = null;
    notice = "";
    const problems = decisionProblems({ status: next, reason });
    if (problems.length > 0) {
      error = problems.map((k) => t(k, { min: REASON_MIN })).join(" ");
      return;
    }
    busy = true;
    try {
      await ppm.decideProgrammeStatus(pid, {
        status: next as "active" | "paused" | "stopped",
        reason: reason.trim(),
        ...(reviewPid ? { gate_review_pid: reviewPid } : {}),
        ...(exceptionPid ? { exception_pid: exceptionPid } : {}),
      });
      notice = t("p.programme.decided", { status: enumLabel("pk_status", next) });
      next = "";
      reason = "";
      reviewPid = "";
      exceptionPid = "";
      await loadStatus();
    } catch (err) {
      const k = programmeRefusalKey(err);
      error = k ? t(k) : errorText(err, "p.programme.could_not_decide");
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>{t("p.programme.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.programme.title")}</h1>
{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}

{#if gates}
  <section aria-labelledby="gates-h">
    <h2 id="gates-h">{t("p.programme.gates_heading")}</h2>
    <p class="small muted">{t("p.programme.gates_says")}</p>
    {#if gateError}<p class="banner error" role="alert">{gateError}</p>{/if}
    <p class="small" role="status">{gateNotice}</p>
    <form class="stack" novalidate onsubmit={saveGates}>
      <div class="gates">
        {#each PLAN_GATES as g (g)}
          <label>
            {enumLabel("pk_gate", g)}
            <input type="date" bind:value={dates[g]} />
          </label>
        {/each}
      </div>
      <div class="row">
        <button class="button" type="submit" disabled={busy}>{t("p.programme.save_gates")}</button>
      </div>
    </form>
  </section>
{/if}

{#if isProgramme === false}
  <p class="muted">{t("p.programme.not_a_programme")}</p>
{/if}

{#if status}
  <section aria-labelledby="status-h">
    <h2 id="status-h">{t("p.programme.status_heading")}</h2>
    <p class="status"><strong>{t("p.programme.status_is", { status: enumLabel("pk_status", status.status) })}</strong></p>
    <p class="small muted">{t("p.programme.status_says")}</p>
    {#if status.still_held}
      <h3>{t("p.programme.held_heading")}</h3>
      <ul class="small">
        <li>{tn("p.programme.held_allocations", status.still_held.allocations_flagged, { count: status.still_held.allocations_flagged })}</li>
        {#each status.still_held.money_by_currency as m (m.currency)}
          <li>{t("p.programme.held_money", { planned: money(m.planned_minor, m.currency), actual: money(m.actual_minor, m.currency) })}</li>
        {/each}
        <li>{tn("p.programme.held_benefits", status.still_held.benefits, { count: status.still_held.benefits })}</li>
      </ul>
    {/if}

    {#if choices.length === 0}
      <p>{t("p.programme.stopped_final")}</p>
    {:else}
      {#if error}<p class="banner error" role="alert">{error}</p>{/if}
      <p class="small" role="status">{notice}</p>
      <form class="stack" novalidate onsubmit={decide}>
        <fieldset>
          <legend>{t("p.programme.f_change")}</legend>
          {#each choices as c (c)}
            <label class="check"><input type="radio" name="next" value={c} bind:group={next} /> {t(CHANGE_LABELS[c])}</label>
          {/each}
        </fieldset>
        {#if next === "stopped"}<p class="small" role="note">{t("p.programme.stop_warns")}</p>{/if}
        <label>
          {t("p.programme.f_reason", { min: REASON_MIN })}
          <textarea bind:value={reason} rows="3"></textarea>
        </label>
        {#if reviews.length > 0}
          <label>
            {t("p.programme.f_review")}
            <select bind:value={reviewPid}>
              <option value="">{t("p.programme.cites_none")}</option>
              {#each reviews as r (r.pid)}<option value={r.pid}>{enumLabel("pk_gate", r.gate)} · {formatDate(r.decided_at)}</option>{/each}
            </select>
          </label>
        {/if}
        {#if exceptions.length > 0}
          <label>
            {t("p.programme.f_exception")}
            <select bind:value={exceptionPid}>
              <option value="">{t("p.programme.cites_none")}</option>
              {#each exceptions as x (x.pid)}<option value={x.pid}>{enumLabel("pk_conf", x.worst_value)} · {formatDate(x.opened_at)}</option>{/each}
            </select>
          </label>
        {/if}
        <div class="row">
          <button class="button primary" type="submit" disabled={busy}>{t("p.programme.decide")}</button>
        </div>
      </form>
    {/if}

    <h3>{t("p.programme.history_heading")}</h3>
    {#if status.history.length === 0}
      <p class="muted small">{t("p.programme.no_history")}</p>
    {:else}
      <ol class="history" reversed>
        {#each [...status.history].reverse() as h, i (i)}
          <li>
            <p><strong>{enumLabel("pk_status", h.status)}</strong> · {formatDate(h.decided_on)}</p>
            <p class="reason">{h.reason}</p>
            {#if h.gate_review_pid}<p class="small muted">{t("p.programme.cited_review")}</p>{/if}
            {#if h.exception_pid}<p class="small muted">{t("p.programme.cited_exception")}</p>{/if}
          </li>
        {/each}
      </ol>
    {/if}
  </section>
{/if}

<style>
  section {
    margin-block-start: 1.5rem;
  }
  .gates {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));
    gap: 0.75rem;
  }
  .gates label {
    display: grid;
    gap: 0.25rem;
  }
  .check {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }
  fieldset {
    display: grid;
    gap: 0.35rem;
  }
  .history {
    display: grid;
    gap: 0.75rem;
    padding-inline-start: 1.5rem;
  }
  .history p {
    margin-block: 0 0.25rem;
  }
  .reason {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
</style>
