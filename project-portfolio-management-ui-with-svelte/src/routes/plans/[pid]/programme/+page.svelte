<!--
  A plan's gates and programme status (`/plans/[pid]/programme`, DP-5, NC-1 slice 11): the date the plan expects its
  review at each gate, and, for a plan of kind Program, its status (active, paused or stopped), the decisions behind
  it, and what a paused or stopped programme still holds. A decision pauses, resumes or stops it with a reason, citing a
  gate review or an exception of the programme if it follows from one. Stopped is final and keeps the record; a pause
  flags allocations and deletes none. The service decides who may; a refusal is the catalogue's sentence. Every plan's
  phase, and a programme's start check and start (`PlanPhase.svelte`, EX-8).
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
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { i18n, tDynamic } from "#lib/i18n.svelte.js";
  import { buildPackView, type Section } from "#lib/pack-view.js";
  import {
    PLAN_GATES,
    PpmClient,
    type DeliveryException,
    type GateJourney,
    type PlannedGates,
    type ProgrammeStatusView,
    type ProjectBufferView,
    DELIVERY_MODELS,
    type DeliveryProfile,
    type Organization,
  } from "#lib/api/ppm.js";
  import ProgrammeThresholds from "#lib/components/ProgrammeThresholds.svelte";
  import PlanPhase from "#lib/components/PlanPhase.svelte";
  import {
    BUFFER_DAYS_MAX,
    CHANGE_LABELS,
    REASON_MIN,
    bufferBody,
    bufferRefusalKey,
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

  // The delivery model and the reporting method (EX-2, EX-14).
  let profile = $state<DeliveryProfile | null>(null);
  let orgs = $state<Organization[]>([]);
  let model = $state("");
  let owner = $state("");
  let method = $state<"evm" | "product">("evm");
  let profileError = $state<string | null>(null);
  let profileNotice = $state("");
  async function loadProfile() {
    try {
      profile = await ppm.deliveryProfile(pid);
      model = profile.delivery_model ?? "";
      owner = profile.delivery_owner?.organization_pid ?? "";
      method = profile.reporting_method;
    } catch {
      profile = null;
      profileError = t("p.programme.could_not_load_profile");
    }
  }
  async function saveModel(e: SubmitEvent) {
    e.preventDefault();
    profileError = null;
    profileNotice = "";
    if (profile && (profile.delivery_model ?? "") === model && (profile.delivery_owner?.organization_pid ?? "") === owner) {
      profileNotice = t("p.programme.profile_unchanged");
      return;
    }
    busy = true;
    try {
      // Both are sent: the service clears a field left out.
      await ppm.setDeliveryModel(pid, { delivery_model: model || null, delivery_owner_organization_pid: owner || null });
      profileNotice = t("p.programme.model_saved");
      await loadProfile();
    } catch (err) {
      profileError = errorText(err, "p.programme.could_not_save_profile");
    } finally {
      busy = false;
    }
  }
  async function saveMethod(e: SubmitEvent) {
    e.preventDefault();
    profileError = null;
    profileNotice = "";
    if (profile && profile.reporting_method === method) {
      profileNotice = t("p.programme.profile_unchanged");
      return;
    }
    busy = true;
    try {
      await ppm.setReportingMethod(pid, method);
      profileNotice = method === "product" ? t("p.programme.method_saved_product") : t("p.programme.method_saved_evm");
      await loadProfile();
    } catch (err) {
      profileError = errorText(err, "p.programme.could_not_save_profile");
    } finally {
      busy = false;
    }
  }

  let buffer = $state<ProjectBufferView | null>(null);
  let bufferHow = $state<"days" | "rule">("days");
  let bufferDays = $state("");
  let bufferNote = $state("");
  let bufferError = $state<string | null>(null);
  let bufferNotice = $state("");
  const fever: Section | null = $derived(
    buffer
      ? (buildPackView(
          { buffer_fever: { programmes: [buffer.now], lines: buffer.lines } },
          { t: (k, v) => tDynamic(k, v), label: (g, v) => enumLabel(g as never, v), locale: i18n.locale },
        ).find((x) => x.id === "fever") ?? null)
      : null,
  );
  type PackCell = { text: string; symbol?: string; tone?: string };
  const cellOf = (row: unknown, i: number): PackCell => (row as { cells: PackCell[] }).cells[i] ?? { text: "" };
  const packColumns = (names: string[]): DataGridColumn[] =>
    names.map((n, i) => ({ id: `c${i}`, header: n, rowHeader: i === 0, accessor: (row) => cellOf(row, i).text, ...(i === 0 ? {} : { cell: packCell }) }));

  async function loadBuffer() {
    try {
      buffer = await ppm.projectBuffer(pid);
    } catch {
      buffer = null; // the rest of the page still works; the buffer section then says it could not be read
      bufferError = t("p.programme.could_not_load_buffer");
    }
  }
  async function declareBuffer(e: SubmitEvent) {
    e.preventDefault();
    bufferError = null;
    bufferNotice = "";
    const r = bufferBody({ how: bufferHow, days: bufferDays, note: bufferNote });
    if ("problem" in r) {
      bufferError = t(r.problem, { max: BUFFER_DAYS_MAX });
      return;
    }
    busy = true;
    try {
      const done = await ppm.declareProjectBuffer(pid, r.body);
      bufferNotice = tn("p.programme.buffer_declared", done.buffer_days, { days: done.buffer_days });
      bufferDays = "";
      bufferNote = "";
      await loadBuffer();
    } catch (err) {
      const k = bufferRefusalKey(bufferHow, err);
      bufferError = k ? t(k) : errorText(err, "p.programme.could_not_declare_buffer");
    } finally {
      busy = false;
    }
  }

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
        void loadBuffer();
        void loadProfile();
        ppm.organizations().then((r) => (orgs = r.organizations.filter((o) => o.active))).catch(() => (orgs = []));
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

{#if isProgramme !== null}
  <PlanPhase {pid} {isProgramme} />
{/if}

{#snippet packCell({ row, column, formatted }: DataGridCellContext)}
  {@const c = cellOf(row, Number(column.id.slice(1)))}
  {#if c.symbol}<span aria-hidden="true" class="sym">{c.symbol}</span>{/if}{formatted}
{/snippet}

{#if isProgramme}
  <section aria-labelledby="profile-h">
    <h2 id="profile-h">{t("p.programme.profile_heading")}</h2>
    {#if profileError}<p class="banner error" role="alert">{profileError}</p>{/if}
    <p class="small" role="status">{profileNotice}</p>
    <form class="stack" novalidate onsubmit={saveModel}>
      <label>
        {t("p.programme.f_model")}
        <select bind:value={model} disabled={!profile}>
          <option value="">{t("p.director_pack.dm_not_stated")}</option>
          {#each DELIVERY_MODELS as m (m)}<option value={m}>{t(`p.director_pack.dm_${m}`)}</option>{/each}
        </select>
      </label>
      <label>
        {t("p.programme.f_owner")}
        <select bind:value={owner} disabled={!profile}>
          <option value="">{t("p.programme.no_owner")}</option>
          {#each orgs as o (o.pid)}<option value={o.pid}>{o.name}</option>{/each}
        </select>
      </label>
      <p class="small muted">{t("p.programme.model_says")}</p>
      <div class="row"><button class="button" type="submit" disabled={busy || !profile}>{t("p.programme.save_model")}</button></div>
    </form>
    <form class="stack" novalidate onsubmit={saveMethod}>
      <fieldset>
        <legend>{t("p.programme.f_method")}</legend>
        <label class="check"><input type="radio" name="method" value="evm" bind:group={method} disabled={!profile} /> {t("p.programme.method_evm")}</label>
        <label class="check"><input type="radio" name="method" value="product" bind:group={method} disabled={!profile} /> {t("p.programme.method_product")}</label>
      </fieldset>
      <div class="row"><button class="button" type="submit" disabled={busy || !profile}>{t("p.programme.save_method")}</button></div>
    </form>
  </section>

  <ProgrammeThresholds {pid} />

  <section aria-labelledby="buffer-h">
    <h2 id="buffer-h">{t("p.programme.buffer_heading")}</h2>
    <p class="small muted">{t("p.programme.buffer_says")}</p>
    {#if fever}
      {#each fever.lines as line, i (i)}<p>{line}</p>{/each}
      {#each fever.tables as tb, ti (ti)}
        <DataTable label={tb.caption} caption={tb.caption} captionVisible columns={packColumns(tb.columns)} rows={tb.rows.map((cells) => ({ cells }))} rowId={(_r, i) => String(i)} sortable={false} />
      {/each}
    {/if}
    {#if bufferError}<p class="banner error" role="alert">{bufferError}</p>{/if}
    <p class="small" role="status">{bufferNotice}</p>
    <form class="stack" novalidate onsubmit={declareBuffer}>
      <fieldset>
        <legend>{t("p.programme.f_buffer_how")}</legend>
        <label class="check"><input type="radio" name="buffer-how" value="days" bind:group={bufferHow} /> {t("p.programme.buffer_by_days")}</label>
        <label class="check"><input type="radio" name="buffer-how" value="rule" bind:group={bufferHow} /> {t("p.programme.buffer_by_rule")}</label>
      </fieldset>
      {#if bufferHow === "days"}
        <label>{t("p.programme.f_buffer_days", { max: BUFFER_DAYS_MAX })}<input type="text" inputmode="numeric" bind:value={bufferDays} size="6" /></label>
      {/if}
      <label>{t("p.programme.f_buffer_note")}<input type="text" bind:value={bufferNote} /></label>
      <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.programme.declare_buffer")}</button></div>
    </form>
    {#if buffer && buffer.history.length > 0}
      <h3>{t("p.programme.buffer_history")}</h3>
      <ol class="history small" reversed>
        {#each [...buffer.history].reverse() as h, i (i)}
          <li>
            {h.source === "rule" ? tn("p.programme.buffer_was_rule", h.buffer_days, { days: h.buffer_days }) : tn("p.programme.buffer_was_days", h.buffer_days, { days: h.buffer_days })}
            · {formatDate(h.declared_on)}{#if h.note} · {h.note}{/if}
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
  :global(.sym) {
    display: inline-block;
    inline-size: 1.25em;
  }
  .reason {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
</style>
