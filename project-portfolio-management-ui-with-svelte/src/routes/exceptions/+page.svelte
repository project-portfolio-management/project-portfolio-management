<!--
  Delivery exceptions (`/exceptions`, DP-3, NC-1 slice 11): across the plans the reader can read, each exception that
  delivery confidence at amber/red or red opened, newest first. An open one is to be completed within the deployment's
  window (its cause, the decision needed, an owner and a decide-by date) and is closed only by a recorded decision;
  improving confidence does not close it. A breach (not completed in time) stays on the record. Every value is a word
  and a symbol, never a colour alone. A refusal is the catalogue's sentence.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { localeHref, t, tn } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import { PpmClient, type DeliveryException } from "#lib/api/ppm.js";
  import { CONFIDENCE_MARK } from "#lib/confidence-view.js";
  import {
    DECISION_MIN,
    MISSING_LABELS,
    completeBody,
    completeProblems,
    decisionProblem,
    exceptionRefusalKey,
    formOf,
    type CompleteForm,
  } from "#lib/exception-view.js";

  const ppm = PpmClient.withFetch();
  const plans = PlanRepository.withFetch();

  let list = $state<DeliveryException[] | null>(null);
  let windowHours = $state(48);
  let names = $state<Record<string, string>>({});
  let loadError = $state<string | null>(null);
  let status = $state<"" | "open" | "closed">("open");
  let breachedOnly = $state(false);

  let editing = $state<string | null>(null);
  let form = $state<CompleteForm>({ cause: "", decisionNeeded: "", ownerRef: "", ownerRole: "", decideBy: "" });
  let deciding = $state<string | null>(null);
  let decision = $state("");
  let error = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  const word = (v: string) => enumLabel("pk_conf", v);
  const mark = (v: string) => CONFIDENCE_MARK[v as keyof typeof CONFIDENCE_MARK] ?? CONFIDENCE_MARK.not_assessed;
  const planName = (pid: string) => names[pid] ?? t("p.exceptions.a_plan");

  async function load() {
    loadError = null;
    try {
      const r = await ppm.deliveryExceptions({
        ...(status ? { status } : {}),
        ...(breachedOnly ? { breached: true } : {}),
      });
      list = r.exceptions;
      windowHours = r.window_hours;
    } catch (err) {
      loadError = errorText(err, "p.exceptions.could_not_load");
    }
  }
  async function loadNames() {
    try {
      names = Object.fromEntries((await plans.list()).map((p) => [p.pid, p.name]));
    } catch {
      names = {}; // each exception then names "a plan" and still links to it
    }
  }
  onMount(() => {
    void load();
    void loadNames();
  });

  function startEdit(e: DeliveryException) {
    deciding = null;
    editing = e.pid;
    form = formOf(e);
  }
  function startDecide(e: DeliveryException) {
    editing = null;
    deciding = e.pid;
    decision = "";
  }

  async function act(run: () => Promise<unknown>, done: string, action: "complete" | "decide") {
    error = null;
    notice = "";
    busy = true;
    try {
      await run();
      notice = done;
      editing = null;
      deciding = null;
      await load();
    } catch (err) {
      const k = exceptionRefusalKey(action, err);
      error = k ? t(k) : errorText(err, "p.exceptions.could_not_save");
    } finally {
      busy = false;
    }
  }
  function save(ev: SubmitEvent, e: DeliveryException) {
    ev.preventDefault();
    const problems = completeProblems(form);
    if (problems.length > 0) {
      error = t(problems[0]!, { min: 3 });
      notice = "";
      return;
    }
    const body = completeBody(form, e);
    if (!body) {
      error = null;
      notice = t("p.exceptions.unchanged");
      editing = null;
      return;
    }
    void act(() => ppm.completeException(e.pid, body), t("p.exceptions.saved"), "complete");
  }
  function decide(ev: SubmitEvent, e: DeliveryException) {
    ev.preventDefault();
    const problem = decisionProblem(decision);
    if (problem) {
      error = t(problem, { min: DECISION_MIN });
      notice = "";
      return;
    }
    void act(() => ppm.decideException(e.pid, decision.trim()), t("p.exceptions.closed_done"), "decide");
  }
</script>

<svelte:head><title>{t("p.exceptions.title_ppm")}</title></svelte:head>

<h1>{t("p.exceptions.title")}</h1>
<p class="small muted">{tn("p.exceptions.intro", windowHours, { hours: windowHours })}</p>

<div class="row filters">
  <label>
    {t("p.exceptions.f_status")}
    <select bind:value={status} onchange={load}>
      <option value="open">{t("p.exceptions.status_open")}</option>
      <option value="closed">{t("p.exceptions.status_closed")}</option>
      <option value="">{t("p.exceptions.status_all")}</option>
    </select>
  </label>
  <label class="check">
    <input type="checkbox" bind:checked={breachedOnly} onchange={load} />
    {t("p.exceptions.f_breached")}
  </label>
</div>

{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>

{#if list}
  {#if list.length === 0}
    <p class="muted">{t("p.exceptions.none")}</p>
  {:else}
    <ul class="exceptions">
      {#each list as e (e.pid)}
        <li>
          <article aria-labelledby={`exc-${e.pid}`}>
            <h2 id={`exc-${e.pid}`}><a href={localeHref(`/plans/${e.plan_pid}/confidence`)}>{planName(e.plan_pid)}</a></h2>
            <p>
              <span aria-hidden="true">{mark(e.worst_value).symbol}</span>
              {t("p.exceptions.worst", { value: word(e.worst_value) })}
              {#if e.worsened_count > 0} · {tn("p.exceptions.worsened", e.worsened_count, { count: e.worsened_count })}{/if}
            </p>
            <p class="small">
              {t("p.exceptions.opened", { when: formatDate(e.opened_at) })} ·
              {e.status === "open" ? t("p.exceptions.is_open") : t("p.exceptions.is_closed", { when: formatDate(e.closed_at ?? "") })}
            </p>
            {#if e.breached}
              <p class="small"><strong>{tn("p.exceptions.breached", e.window_hours, { hours: e.window_hours })}</strong></p>
            {/if}
            {#if e.status === "open"}
              {#if e.complete}
                <p class="small">{t("p.exceptions.complete")}</p>
              {:else}
                <p class="small">{t("p.exceptions.missing", { parts: e.missing.map((m) => t(MISSING_LABELS[m])).join(", ") })}</p>
              {/if}
            {/if}
            <dl class="small">
              {#if e.cause}<dt>{t("p.exceptions.part_cause")}</dt><dd>{e.cause}</dd>{/if}
              {#if e.decision_needed}<dt>{t("p.exceptions.part_decision_needed")}</dt><dd>{e.decision_needed}</dd>{/if}
              {#if e.owner_ref || e.owner_role}<dt>{t("p.exceptions.part_owner")}</dt><dd>{[e.owner_role, e.owner_ref].filter(Boolean).join(" · ")}</dd>{/if}
              {#if e.decide_by}<dt>{t("p.exceptions.part_decide_by")}</dt><dd>{formatDate(e.decide_by)}</dd>{/if}
              {#if e.decision}<dt>{t("p.exceptions.part_decision")}</dt><dd>{e.decision}</dd>{/if}
            </dl>

            {#if e.status === "open"}
              <div class="row">
                <button class="button small" type="button" disabled={busy} onclick={() => startEdit(e)} aria-label={`${t("p.exceptions.complete_it")}: ${planName(e.plan_pid)}`}>{t("p.exceptions.complete_it")}</button>
                <button class="button small" type="button" disabled={busy} onclick={() => startDecide(e)} aria-label={`${t("p.exceptions.decide_it")}: ${planName(e.plan_pid)}`}>{t("p.exceptions.decide_it")}</button>
              </div>
              {#if editing === e.pid}
                <form class="stack" novalidate onsubmit={(ev) => save(ev, e)}>
                  <label>{t("p.exceptions.part_cause")}<textarea bind:value={form.cause} rows="2"></textarea></label>
                  <label>{t("p.exceptions.part_decision_needed")}<textarea bind:value={form.decisionNeeded} rows="2"></textarea></label>
                  <label>{t("p.exceptions.f_owner_ref")}<input type="text" bind:value={form.ownerRef} placeholder={t("p.plans.pid.handover.person")} /></label>
                  <label>{t("p.exceptions.f_owner_role")}<input type="text" bind:value={form.ownerRole} /></label>
                  <label>{t("p.exceptions.part_decide_by")}<input type="date" bind:value={form.decideBy} /></label>
                  <p class="small muted">{t("p.exceptions.complete_says")}</p>
                  <div class="row">
                    <button class="button small" type="submit" disabled={busy}>{t("p.exceptions.save")}</button>
                    <button class="button small" type="button" onclick={() => (editing = null)}>{t("p.exceptions.cancel")}</button>
                  </div>
                </form>
              {/if}
              {#if deciding === e.pid}
                <form class="stack" novalidate onsubmit={(ev) => decide(ev, e)}>
                  <label>{t("p.exceptions.f_decision", { min: DECISION_MIN })}<textarea bind:value={decision} rows="3"></textarea></label>
                  <p class="small muted">{t("p.exceptions.decide_says")}</p>
                  <div class="row">
                    <button class="button small" type="submit" disabled={busy}>{t("p.exceptions.decide")}</button>
                    <button class="button small" type="button" onclick={() => (deciding = null)}>{t("p.exceptions.cancel")}</button>
                  </div>
                </form>
              {/if}
            {/if}
          </article>
        </li>
      {/each}
    </ul>
  {/if}
{/if}

<style>
  .filters {
    gap: 1rem;
    align-items: center;
    margin-block: 1rem;
  }
  .check {
    display: inline-flex;
    gap: 0.35rem;
    align-items: center;
  }
  .exceptions {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 1.25rem;
  }
  .exceptions h2 {
    font-size: 1.1rem;
    margin-block: 0 0.25rem;
  }
  .exceptions p {
    margin-block: 0 0.25rem;
  }
  dl {
    display: grid;
    grid-template-columns: max-content 1fr;
    gap: 0.25rem 0.75rem;
  }
  dt {
    font-weight: 600;
  }
  dd {
    margin: 0;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
</style>
