<!--
  A plan's status updates on a cadence, with their evidence (`/plans/[pid]/status`, NC-4, BR-2, NC-1 slice 12 part b).
  Where the plan stands: the latest update (a person's call, never a computed score) and, against the plan's cadence,
  when the next is due or how many days it is overdue, read from the service. The cadence is set (1 to 92 days; setting
  it restarts the clock; nothing sent for the same cadence; no route removes one). An update is posted with its claims
  (`StatusUpdateForm.svelte`). The history is newest first, each claim's sources as the service resolves them for the
  reader: a record's title, a number's value (unknown is never zero) or "no longer available". How each number is
  counted is the catalogue's words by key; the service's English labels, definitions and notes are never shown. Rules
  in `src/lib/status-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { isAbsent } from "#lib/absent.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tDynamic, tn } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { PpmClient, type PlanStatus, type StatusCadence, type StatusMetrics, type StatusUpdate } from "#lib/api/ppm.js";
  import StatusUpdateForm from "#lib/components/StatusUpdateForm.svelte";
  import {
    CADENCE_MAX,
    CADENCE_MIN,
    CALL_MARK,
    METRIC_KEYS,
    cadenceBody,
    isMetricKey,
    sourceView,
    standingOf,
    statusRefusalKey,
    type RecordKind,
  } from "#lib/status-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let status = $state<PlanStatus | null>(null);
  let cadence = $state<StatusCadence | null>(null);
  let cadenceLoaded = $state(false);
  let updates = $state<StatusUpdate[] | null>(null);
  let metrics = $state<StatusMetrics | null>(null);
  let records = $state<Partial<Record<RecordKind, Array<{ pid: string; title: string }>>>>({});
  let loadError = $state<string | null>(null);

  let cadenceValue = $state<string | number>("");
  let cadenceError = $state<string | null>(null);
  let cadenceNotice = $state("");
  let busy = $state(false);

  const callWord = (c: StatusUpdate["status"]) => t(CALL_MARK[c].key);

  async function loadStatus() {
    status = await ppm.planStatus(pid);
  }
  async function loadCadence() {
    try {
      cadence = await ppm.statusCadence(pid);
    } catch (err) {
      // A 404 is a plan with no cadence yet.
      if (!isAbsent(err)) throw err;
      cadence = null;
    }
    cadenceValue = cadence?.cadence_days ?? "";
    cadenceLoaded = true;
  }
  async function loadUpdates() {
    updates = await ppm.statusUpdates(pid);
  }
  /** The plan's records a claim may cite; a kind that cannot be read is simply not offered. */
  async function loadRecords() {
    const out: Partial<Record<RecordKind, Array<{ pid: string; title: string }>>> = {};
    await Promise.all([
      ppm.listTasks(pid).then((r) => (out.task = r.tasks.map((x) => ({ pid: x.pid, title: x.title })))).catch(() => {}),
      ppm.listRisks(pid).then((r) => (out.risk = r.map((x) => ({ pid: x.pid, title: x.title })))).catch(() => {}),
      ppm.milestones(pid).then((r) => (out.milestone = r.map((x) => ({ pid: x.pid, title: x.name })))).catch(() => {}),
      ppm.planIssues(pid).then((r) => (out.issue = r.issues.map((x) => ({ pid: x.pid, title: x.title })))).catch(() => {}),
    ]);
    records = out;
  }

  onMount(async () => {
    try {
      await Promise.all([loadStatus(), loadCadence(), loadUpdates()]);
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.status.could_not_load");
    }
    ppm.statusMetrics().then((m) => (metrics = m)).catch(() => (metrics = null));
    void loadRecords();
  });

  async function reload() {
    try {
      await Promise.all([loadStatus(), loadUpdates()]);
    } catch (err) {
      loadError = errorText(err, "p.status.could_not_load");
    }
  }

  async function saveCadence(e: SubmitEvent) {
    e.preventDefault();
    cadenceError = null;
    cadenceNotice = "";
    const r = cadenceBody(cadence?.cadence_days ?? null, cadenceValue);
    if (r === null) {
      cadenceNotice = t("p.status.unchanged");
      return;
    }
    if ("problem" in r) {
      cadenceError = t(r.problem, { min: CADENCE_MIN, max: CADENCE_MAX });
      return;
    }
    busy = true;
    try {
      cadence = await ppm.setStatusCadence(pid, r.body.cadence_days);
      cadenceValue = cadence.cadence_days;
      cadenceNotice = tn("p.status.cadence_saved", cadence.cadence_days, { n: cadence.cadence_days });
      await loadStatus();
    } catch (err) {
      const k = statusRefusalKey("cadence", err);
      cadenceError = k ? t(k, { min: CADENCE_MIN, max: CADENCE_MAX }) : errorText(err, "p.status.could_not_save");
    } finally {
      busy = false;
    }
  }

  /** The metrics the service publishes that this page has words for, in the service's order. */
  const shownMetrics = $derived(metrics ? metrics.metrics.map((m) => m.key).filter(isMetricKey) : [...METRIC_KEYS]);
  const dueSoonDays = $derived(metrics?.due_soon_days ?? 7);
</script>

{#snippet standingLine(s: NonNullable<PlanStatus["standing"]>)}
  {@const st = standingOf(s)}
  {#if st.is === "overdue"}
    <span aria-hidden="true">✗</span> {tn("p.status.standing_overdue", st.days, { date: formatDate(st.date), n: st.days })}
  {:else if st.is === "due"}
    <span aria-hidden="true">!</span> {t("p.status.standing_due", { date: formatDate(st.date) })}
  {:else}
    <span aria-hidden="true">✓</span> {t("p.status.standing_next", { date: formatDate(st.date) })}
  {/if}
{/snippet}

{#snippet updateView(u: StatusUpdate)}
  <p>
    <strong><span aria-hidden="true">{CALL_MARK[u.status].symbol}</span> {callWord(u.status)}</strong> ·
    {u.posted_by
      ? t("p.status.posted_on_by", { date: formatDate(u.posted_on), who: u.posted_by })
      : t("p.status.posted_on", { date: formatDate(u.posted_on) })}
  </p>
  <p class="narrative">{u.narrative}</p>
  {#if u.claims.length > 0}
    <ul class="claims" aria-label={t("p.status.claims_of", { date: formatDate(u.posted_on) })}>
      {#each u.claims as c, i (i)}
        <li>
          <p>{c.text}</p>
          {#if c.sources.length > 0}
            <ul class="small">
              {#each c.sources as s, j (j)}
                {@const v = sourceView(s)}
                <li>
                  {#if v.as === "record"}
                    {tDynamic(`p.status.kind_${v.kind}`, { title: v.title })}
                  {:else if v.as === "metric"}
                    {v.value === null
                      ? t("p.status.source_metric_unknown", { metric: tDynamic(`p.status.metric_${v.key}`) })
                      : t("p.status.source_metric", { metric: tDynamic(`p.status.metric_${v.key}`), value: v.value })}
                  {:else}
                    <span aria-hidden="true">?</span> {t("p.status.source_unavailable")}
                  {/if}
                </li>
              {/each}
            </ul>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
{/snippet}

<svelte:head><title>{t("p.status.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.status.title")}</h1>
<p class="small muted">{t("p.status.intro")}</p>
{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}

<section aria-labelledby="status-now-h">
  <h2 id="status-now-h">{t("p.status.now_heading")}</h2>
  {#if status}
    <div data-testid="status-now">
      {#if status.latest}
        {@render updateView(status.latest)}
      {:else}
        <p class="muted">{t("p.status.no_update")}</p>
      {/if}
      {#if status.cadence_days !== null && status.standing}
        <p>{tn("p.status.cadence_is", status.cadence_days, { n: status.cadence_days })}</p>
        <p data-testid="standing">{@render standingLine(status.standing)}</p>
      {:else}
        <p data-testid="standing">{t("p.status.no_cadence")}</p>
      {/if}
      <p class="small muted">{t("p.status.as_of", { date: formatDate(status.as_of) })}</p>
    </div>
  {:else if !loadError}
    <p class="muted">{t("p.status.loading")}</p>
  {/if}
</section>

<section aria-labelledby="status-cadence-h">
  <h2 id="status-cadence-h">{t("p.status.cadence_heading")}</h2>
  <p class="small muted">{t("p.status.cadence_says", { min: CADENCE_MIN, max: CADENCE_MAX })}</p>
  {#if cadence}
    <p class="small">
      {cadence.set_by
        ? t("p.status.cadence_set_by", { date: formatDate(cadence.set_on), who: cadence.set_by })
        : t("p.status.cadence_set_on", { date: formatDate(cadence.set_on) })}
    </p>
  {/if}
  {#if cadenceError}<p class="banner error" role="alert">{cadenceError}</p>{/if}
  <p class="small" role="status">{cadenceNotice}</p>
  <form class="stack" novalidate onsubmit={saveCadence}>
    <label>
      {t("p.status.f_cadence", { min: CADENCE_MIN, max: CADENCE_MAX })}
      <input type="number" step="1" min={CADENCE_MIN} max={CADENCE_MAX} inputmode="numeric" bind:value={cadenceValue} disabled={!cadenceLoaded} />
    </label>
    <div class="row"><button class="button" type="submit" disabled={busy || !cadenceLoaded}>{t("p.status.save_cadence")}</button></div>
  </form>
</section>

<StatusUpdateForm {pid} {records} ready={status !== null} onPosted={reload} />

<section aria-labelledby="status-history-h">
  <h2 id="status-history-h">{t("p.status.history_heading")}</h2>
  <p class="small muted">{t("p.status.history_says")}</p>
  {#if updates}
    {#if updates.length === 0}
      <p class="muted">{t("p.status.history_none")}</p>
    {:else}
      <ol class="history" aria-label={t("p.status.history_heading")}>
        {#each updates as u (u.pid)}
          <li>{@render updateView(u)}</li>
        {/each}
      </ol>
    {/if}
  {/if}
</section>

<section aria-labelledby="status-metrics-h">
  <h2 id="status-metrics-h">{t("p.status.metrics_heading")}</h2>
  <p class="small muted">{t("p.status.metrics_says")}</p>
  <dl class="small">
    {#each shownMetrics as m (m)}
      <dt>{tDynamic(`p.status.metric_${m}`)}</dt>
      <dd>{tDynamic(`p.status.metric_def_${m}`, { days: dueSoonDays })}</dd>
    {/each}
  </dl>
</section>

<style>
  section {
    margin-block-start: 2rem;
  }
  form {
    max-width: 40rem;
  }
  .narrative {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
  .history {
    padding-inline-start: 1.25rem;
    display: grid;
    gap: 1rem;
  }
  .claims {
    padding-inline-start: 1.25rem;
  }
  .claims p {
    margin-block: 0.25rem;
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
  }
</style>
