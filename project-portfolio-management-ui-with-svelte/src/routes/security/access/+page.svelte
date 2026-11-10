<!--
  Access review and emergency access (`/security/access`, NC-1 slice 7, RG-9, RG-10), linked from `/security`.

  The access review lists the grants that stand today in the access ledger, oldest first, for recertification
  (administrators only; each read is audited, so it is read when asked, never on opening the page), with the service's
  own spreadsheet, which has a column for the reviewer's decision.

  Emergency access ("break-glass") opens a private plan to the caller for a limited time. It is a security control,
  so the page says before the form that every use is recorded and audited and that the person named is told, asks for
  the reason and the length of time the service requires, checks them no more loosely than the service does, asks the
  person to say they understand, and does not make it the page's primary action. Who may use it is the service's to
  decide (a 403 is shown in words). Times are shown in the browser's time zone, which is named. Refusals say the
  catalogue's sentence, never the service's own text.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref, type StringKey } from "#lib/i18n.svelte.js";
  import { formatDate, formatDateTime, formatInteger } from "#lib/locale-format.js";
  import { onMount } from "svelte";
  import { ApiError } from "#lib/api/client.js";
  import {
    PpmClient,
    type AccessReview,
    type AccessReviewGrant,
    type AccessReviewQuery,
    type BreakGlassGrant,
  } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import {
    DEFAULT_MINUTES,
    DEFAULT_REVIEW_DAYS,
    GRANT_SYMBOL,
    MAX_MINUTES,
    MAX_REASON,
    MIN_MINUTES,
    MIN_REASON,
    accessRefusalKey,
    grantState,
    openBody,
    openProblems,
    reasonLength,
    reviewProblem,
    reviewQuery,
    type AccessStep,
    type GrantState,
    type OpenProblem,
  } from "#lib/access-view.js";

  const ppm = PpmClient.withFetch();
  const plans = PlanRepository.withFetch();
  const tz = Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";

  // The access review.
  let reviewPlan = $state("");
  let reviewDays = $state(String(DEFAULT_REVIEW_DAYS));
  let review = $state<AccessReview | null>(null);
  let reviewAsked = $state<AccessReviewQuery>({});
  let reviewError = $state<string | null>(null);
  let reviewBusy = $state(false);

  // Emergency access.
  let plan = $state("");
  let reason = $state("");
  let minutes = $state(String(DEFAULT_MINUTES));
  let notify = $state("");
  let acknowledged = $state(false);
  let problems = $state<OpenProblem[]>([]);
  let openError = $state<string | null>(null);
  let opened = $state<{ plan: string; until: string; told: string } | null>(null);
  let grants = $state<BreakGlassGrant[] | null>(null);
  let listError = $state<string | null>(null);
  let revokeError = $state<string | null>(null);
  let revokeNotice = $state("");
  let busy = $state(false);
  let planNames = $state<Record<string, string>>({});

  /** The catalogue sentence for a failure of `step`, or the general rule with `fallback`. */
  function why(err: unknown, step: AccessStep, fallback: StringKey): string {
    const key = err instanceof ApiError ? accessRefusalKey(step, err.status) : null;
    return key ? t(key) : errorText(err, fallback);
  }

  /** Name the plans not named yet. A plan the caller cannot read (a grant that has ended, say) keeps its id. */
  async function namePlans(pids: string[]) {
    const missing = [...new Set(pids)].filter((pid) => !(pid in planNames));
    const found = await Promise.all(
      missing.map(async (pid) => {
        try {
          return [pid, (await plans.get(pid)).name] as const;
        } catch {
          return null;
        }
      }),
    );
    const named = found.filter((f): f is readonly [string, string] => f !== null);
    if (named.length > 0) planNames = { ...planNames, ...Object.fromEntries(named) };
  }
  const planText = (pid: string) => planNames[pid] ?? t("p.access.plan_unnamed", { id: pid.slice(0, 8) });

  async function showReview(e: SubmitEvent) {
    e.preventDefault();
    reviewError = null;
    const problem = reviewProblem(reviewPlan, reviewDays);
    if (problem) {
      reviewError = t(problem === "plan_id" ? "p.access.problem_review_plan" : "p.access.problem_review_days");
      return;
    }
    const q = reviewQuery(reviewPlan, reviewDays);
    reviewBusy = true;
    try {
      review = await ppm.accessReview(q);
      reviewAsked = q;
      void namePlans(review.grants.map((g) => g.plan_pid));
    } catch (err) {
      review = null;
      reviewError = why(err, "review", "p.access.could_not_load_review");
    } finally {
      reviewBusy = false;
    }
  }

  async function loadGrants() {
    try {
      const list = await ppm.breakGlassGrants();
      grants = list.grants;
      listError = null;
      void namePlans(list.grants.map((g) => g.plan_pid));
    } catch (err) {
      grants = null;
      listError = why(err, "list", "p.access.could_not_load_grants");
    }
  }
  onMount(() => void loadGrants());

  /** The service's limits, for the sentences that name them. */
  const limits = {
    reason_min: formatInteger(MIN_REASON),
    reason_max: formatInteger(MAX_REASON),
    minutes_min: formatInteger(MIN_MINUTES),
    minutes_max: formatInteger(MAX_MINUTES),
  };
  const PROBLEM_KEY: Record<OpenProblem, StringKey> = {
    plan_id: "p.access.problem_plan",
    reason_short: "p.access.problem_reason_short",
    reason_long: "p.access.problem_reason_long",
    minutes: "p.access.problem_minutes",
    notify_ref: "p.access.problem_notify",
    acknowledged: "p.access.problem_acknowledged",
  };

  async function openPlan(e: SubmitEvent) {
    e.preventDefault();
    openError = null;
    opened = null;
    const fields = { plan, reason, minutes, notify, acknowledged };
    problems = openProblems(fields);
    if (problems.length > 0) return;
    busy = true;
    try {
      const body = openBody(fields);
      const answer = await ppm.openBreakGlass(body);
      opened = { plan: answer.plan_pid, until: answer.expires_at, told: body.notify_ref };
      reason = "";
      acknowledged = false;
    } catch (err) {
      openError = why(err, "open", "p.access.could_not_open");
    } finally {
      busy = false;
      await loadGrants();
    }
  }

  async function revoke(g: BreakGlassGrant) {
    revokeError = null;
    revokeNotice = "";
    busy = true;
    try {
      await ppm.revokeBreakGlass(g.pid);
      revokeNotice = t("p.access.revoked_notice", { plan: planText(g.plan_pid) });
      if (opened?.plan === g.plan_pid) opened = null;
    } catch (err) {
      revokeError = why(err, "revoke", "p.access.could_not_revoke");
    } finally {
      busy = false;
      await loadGrants();
    }
  }

  const STATE_KEY: Record<GrantState, StringKey> = {
    live: "p.access.state_live",
    revoked: "p.access.state_revoked",
    expired: "p.access.state_expired",
  };

  type ReviewRow = AccessReviewGrant;
  const reviewColumns: DataGridColumn[] = $derived([
    { id: "person_ref", header: t("p.access.col_person"), rowHeader: true },
    { id: "access", header: t("p.access.col_access") },
    { id: "plan_pid", header: t("p.access.col_plan"), accessor: (r) => planText(rowOf<ReviewRow>(r).plan_pid), cell: reviewPlanCell },
    { id: "granted_on", header: t("p.access.col_granted_on") },
    { id: "granted_by", header: t("p.access.col_granted_by"), format: (v) => (v ? String(v) : "—") },
    { id: "age_calendar_days", header: t("p.access.col_age"), align: "end", format: (v) => formatInteger(Number(v)) },
    { id: "review_due", header: t("p.access.col_review"), format: (v) => (v ? t("p.access.due") : t("p.access.not_due")), cell: dueCell },
    { id: "note", header: t("p.access.col_note"), format: (v) => (v ? String(v) : "") },
  ]);

  type GrantRow = BreakGlassGrant;
  const grantColumns: DataGridColumn[] = $derived([
    { id: "state", header: t("p.access.col_state"), accessor: (r) => grantState(rowOf<GrantRow>(r)), format: (v) => t(STATE_KEY[v as GrantState]), cell: stateCell },
    { id: "plan_pid", header: t("p.access.col_plan"), rowHeader: true, accessor: (r) => planText(rowOf<GrantRow>(r).plan_pid), cell: grantPlanCell },
    { id: "sub", header: t("p.access.col_holder") },
    { id: "opened_at", header: t("p.access.col_opened"), format: (v) => formatDateTime(String(v)) },
    { id: "expires_at", header: t("p.access.col_ends"), accessor: (r) => endsText(rowOf<GrantRow>(r)) },
    { id: "reason", header: t("p.access.col_reason") },
    { id: "notify_ref", header: t("p.access.col_told") },
    { id: "pid", header: t("p.access.col_act"), sortable: false, cell: actCell },
  ]);
  function endsText(g: GrantRow): string {
    if (g.revoked_at) return t("p.access.ended_by", { when: formatDateTime(g.revoked_at), who: g.revoked_by ?? "—" });
    return formatDateTime(g.expires_at);
  }
</script>

{#snippet reviewPlanCell({ row, formatted }: DataGridCellContext)}
  <a href={localeHref(`/plans/${rowOf<ReviewRow>(row).plan_pid}`)}>{formatted}</a>
{/snippet}
{#snippet dueCell({ value, formatted }: DataGridCellContext)}<span aria-hidden="true">{value ? "⚑" : "–"}</span> {formatted}{/snippet}
{#snippet stateCell({ value, formatted }: DataGridCellContext)}<span aria-hidden="true">{GRANT_SYMBOL[value as GrantState]}</span> {formatted}{/snippet}
{#snippet grantPlanCell({ row, formatted }: DataGridCellContext)}
  {@const g = rowOf<GrantRow>(row)}
  {#if g.live}<a href={localeHref(`/plans/${g.plan_pid}`)}>{formatted}</a>{:else}{formatted}{/if}
{/snippet}
{#snippet actCell({ row }: DataGridCellContext)}
  {@const g = rowOf<GrantRow>(row)}
  {#if g.live}
    <button class="button" type="button" disabled={busy} aria-label={t("p.access.revoke_for", { plan: planText(g.plan_pid) })} onclick={() => void revoke(g)}>{t("p.access.revoke")}</button>
  {/if}
{/snippet}

<svelte:head><title>{t("p.access.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref("/security")}>{t("p.access.back")}</a></p>
<h1>{t("p.access.heading")}</h1>
<p class="small muted">{t("p.access.intro")}</p>

<section aria-labelledby="ar-h" data-testid="review">
  <h2 id="ar-h">{t("p.access.review_heading")}</h2>
  <p class="small muted">{t("p.access.review_intro")}</p>
  <form class="filters" onsubmit={showReview} novalidate>
    <label>{t("p.access.f_review_plan")}
      <input bind:value={reviewPlan} autocomplete="off" spellcheck="false" maxlength="36" />
    </label>
    <label>{t("p.access.f_review_days")}
      <input bind:value={reviewDays} inputmode="numeric" autocomplete="off" maxlength="5" />
    </label>
    <button class="button" type="submit" disabled={reviewBusy}>{t("p.access.show_review")}</button>
  </form>
  {#if reviewError}<p class="banner" role="alert">{reviewError}</p>{/if}
  {#if review}
    <p data-testid="review-totals">
      {t("p.access.review_totals", {
        as_of: formatDate(review.as_of),
        standing: formatInteger(review.standing),
        due: formatInteger(review.review_due),
        days: formatInteger(review.older_than_days),
      })}
    </p>
    <p class="small muted">{t("p.access.review_note")}</p>
    <p><a href={ppm.accessReviewCsvUrl(reviewAsked)} download>{t("p.access.download_review")}</a></p>
    <DataTable
      label={t("p.access.review_caption")}
      columns={reviewColumns}
      rows={review.grants}
      rowId={(r) => `${rowOf<ReviewRow>(r).plan_pid}|${rowOf<ReviewRow>(r).person_ref}|${rowOf<ReviewRow>(r).access}`}
      emptyText={t("p.access.review_empty")}
      exportable={false}
      searchable
      testid="review-table"
    />
  {/if}
</section>

<section aria-labelledby="bg-h" data-testid="break-glass">
  <h2 id="bg-h">{t("p.access.bg_heading")}</h2>
  <div class="audited" role="note" aria-labelledby="bg-audited-h">
    <p id="bg-audited-h"><strong>{t("p.access.bg_audited_heading")}</strong></p>
    <ul>
      <li>{t("p.access.bg_audited_open")}</li>
      <li>{t("p.access.bg_audited_told")}</li>
      <li>{t("p.access.bg_audited_each")}</li>
      <li>{t("p.access.bg_audited_visibility")}</li>
      <li>{t("p.access.bg_audited_ends")}</li>
    </ul>
  </div>
  <p class="small muted">{t("p.access.bg_intro")}</p>

  {#if problems.length > 0}
    <div class="banner" role="alert" data-testid="bg-problems">
      <p>{t("p.access.problems_heading")}</p>
      <ul>{#each problems as p (p)}<li>{t(PROBLEM_KEY[p], limits)}</li>{/each}</ul>
    </div>
  {/if}
  {#if openError}<p class="banner" role="alert">{openError}</p>{/if}
  <div role="status" data-testid="bg-opened">
    {#if opened}
      <p>{t("p.access.opened_notice", { until: formatDateTime(opened.until), tz, person: opened.told })}</p>
      <p><a href={localeHref(`/plans/${opened.plan}`)}>{t("p.access.go_to_plan")}</a></p>
    {/if}
  </div>

  <form class="open-form" onsubmit={openPlan} novalidate>
    <label>{t("p.access.f_plan")}
      <input bind:value={plan} autocomplete="off" spellcheck="false" maxlength="36" required />
    </label>
    <label>{t("p.access.f_reason", { min: formatInteger(MIN_REASON) })}
      <textarea bind:value={reason} rows="4" maxlength={MAX_REASON} required aria-describedby="bg-reason-count"></textarea>
    </label>
    <p class="small muted" id="bg-reason-count">{t("p.access.reason_count", { n: formatInteger(reasonLength(reason)), min: formatInteger(MIN_REASON), max: formatInteger(MAX_REASON) })}</p>
    <label>{t("p.access.f_minutes", { min: formatInteger(MIN_MINUTES), max: formatInteger(MAX_MINUTES) })}
      <input bind:value={minutes} inputmode="numeric" autocomplete="off" maxlength="3" required />
    </label>
    <label>{t("p.access.f_notify")}
      <input bind:value={notify} autocomplete="off" spellcheck="false" maxlength="200" required />
    </label>
    <label class="inline"><input type="checkbox" bind:checked={acknowledged} /> {t("p.access.f_acknowledge")}</label>
    <div class="row">
      <button class="button" type="submit" disabled={busy}>{t("p.access.open")}</button>
    </div>
  </form>

  <h3>{t("p.access.grants_heading")}</h3>
  <p class="small muted">{t("p.access.grants_intro", { tz })}</p>
  {#if listError}<p class="banner" role="alert">{listError}</p>{/if}
  {#if revokeError}<p class="banner" role="alert">{revokeError}</p>{/if}
  <p class="small" role="status">{revokeNotice}</p>
  {#if grants}
    <DataTable
      label={t("p.access.grants_caption")}
      columns={grantColumns}
      rows={grants}
      rowId={(r) => rowOf<GrantRow>(r).pid}
      emptyText={t("p.access.grants_empty")}
      sortable={false}
      exportable={false}
      testid="grants-table"
    />
  {:else if !listError}
    <p class="muted">{t("p.access.loading")}</p>
  {/if}
</section>

<style>
  section {
    margin-block: 1.5rem;
  }
  .filters {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
  }
  .filters label,
  .open-form label {
    display: grid;
    gap: 0.2rem;
  }
  .filters input {
    min-inline-size: min(22rem, 100%);
  }
  .open-form {
    display: grid;
    gap: 0.5rem;
    max-inline-size: 36rem;
    margin-block: 1rem;
  }
  .open-form label.inline {
    display: flex;
    gap: 0.5rem;
    align-items: start;
  }
  .audited {
    border: 2px solid var(--mxi-color-border);
    border-inline-start-width: 0.5rem;
    padding: 0.5rem 1rem;
    margin-block: 0.75rem;
    max-inline-size: 48rem;
  }
  .audited ul {
    margin: 0;
    padding-inline-start: 1.25rem;
  }
</style>
