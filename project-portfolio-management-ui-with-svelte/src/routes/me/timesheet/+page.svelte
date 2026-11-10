<!--
  My timesheet (`/me/timesheet`, NC-1 slice 6, LT-11): the signed-in person's own week by day and plan, in whole
  minutes with billable and non-billable always apart, and submitting it for approval; and, for an approver, someone
  else's week and the decision on it. Weeks are the service's ISO weeks (Monday to Sunday), and "this week" is the
  week of today in the browser's time zone. An approved week is final and locked against new time. The page offers
  each step by status only; who may take it is the service's to decide (a 403 is shown in words). Refusals say the
  catalogue's sentence, never the service's own text.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref, type StringKey } from "#lib/i18n.svelte.js";
  import { formatDate, formatDateRange } from "#lib/locale-format.js";
  import { onMount } from "svelte";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type TimesheetWeek } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import TimesheetGrid from "#lib/components/TimesheetGrid.svelte";
  import {
    STATUS_KEY,
    STATUS_SYMBOL,
    canDecide,
    canSubmit,
    decideBody,
    decisionProblem,
    parseWeek,
    personProblem,
    shiftWeek,
    statusOf,
    submitBody,
    thisWeek,
    timesheetRefusalKey,
    todayIn,
    type TimesheetStep,
  } from "#lib/timesheet-view.js";

  const ppm = PpmClient.withFetch();
  const plans = PlanRepository.withFetch();
  const tz = Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";
  const current = thisWeek(tz);

  let week = $state(current);
  let weekInput = $state(current);
  let mine = $state<TimesheetWeek | null>(null);
  let mineError = $state<string | null>(null);
  let other = $state<TimesheetWeek | null>(null);
  let otherError = $state<string | null>(null);
  let personInput = $state("");
  // The person whose week the approver section shows, once a reference has been accepted.
  let person = $state<string | null>(null);
  let decision = $state<"approved" | "rejected">("approved");
  let comment = $state("");
  let actionError = $state<string | null>(null);
  let notice = $state("");
  let saving = $state(false);
  let planNames = $state<Record<string, string>>({});

  /** The catalogue sentence for a failure of `step`, or the general rule with `fallback`. */
  function why(err: unknown, step: TimesheetStep, fallback: StringKey): string {
    const key = err instanceof ApiError ? timesheetRefusalKey(step, err.status) : null;
    return key ? t(key) : errorText(err, fallback);
  }

  /** Read the names of the plans a week shows that the page has not named yet. A plan it cannot read keeps its id. */
  async function namePlans(w: TimesheetWeek) {
    const missing = w.grid.rows.map((r) => r.plan_pid).filter((pid) => !(pid in planNames));
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

  async function loadMine() {
    try {
      mine = await ppm.timesheetWeek(week);
      mineError = null;
      void namePlans(mine);
    } catch (err) {
      mine = null;
      mineError = why(err, "read_own", "p.timesheet.could_not_load");
    }
  }
  async function loadOther() {
    if (!person) return;
    try {
      other = await ppm.timesheetWeek(week, person);
      otherError = null;
      void namePlans(other);
    } catch (err) {
      other = null;
      otherError = why(err, "read_other", "p.timesheet.could_not_load_other");
    }
  }
  onMount(() => {
    void loadMine();
  });

  function goTo(next: string | null) {
    if (!next) return;
    week = next;
    weekInput = next;
    actionError = null;
    notice = "";
    void loadMine();
    void loadOther();
  }
  function showTyped(e: SubmitEvent) {
    e.preventDefault();
    const parsed = parseWeek(weekInput);
    if (!parsed) {
      actionError = t("p.timesheet.problem_week");
      notice = "";
      return;
    }
    goTo(parsed);
  }

  async function submit() {
    actionError = null;
    notice = "";
    saving = true;
    try {
      await ppm.submitTimesheet(submitBody(week));
      notice = t("p.timesheet.submitted_notice", { week });
    } catch (err) {
      actionError = why(err, "submit", "p.timesheet.could_not_submit");
    } finally {
      saving = false;
      await loadMine();
    }
  }

  function showPerson(e: SubmitEvent) {
    e.preventDefault();
    actionError = null;
    notice = "";
    if (personProblem(personInput)) {
      actionError = t("p.timesheet.problem_person");
      return;
    }
    person = personInput.trim();
    other = null;
    comment = "";
    decision = "approved";
    void loadOther();
  }

  async function decide(e: SubmitEvent) {
    e.preventDefault();
    if (!person) return;
    actionError = null;
    notice = "";
    const problem = decisionProblem(decision, comment, person, mine?.person ?? null);
    if (problem) {
      actionError = t(
        problem === "own_week" ? "p.timesheet.problem_own_week" : "p.timesheet.problem_comment",
      );
      return;
    }
    saving = true;
    try {
      await ppm.decideTimesheet(decideBody(person, week, decision, comment));
      notice = t(decision === "approved" ? "p.timesheet.approved_notice" : "p.timesheet.rejected_notice", {
        person,
        week,
      });
      comment = "";
    } catch (err) {
      actionError = why(err, "decide", "p.timesheet.could_not_decide");
    } finally {
      saving = false;
      await loadOther();
    }
  }

  function range(w: TimesheetWeek): string {
    return formatDateRange(w.grid.days[0], w.grid.days[6]);
  }
</script>

<svelte:head><title>{t("p.timesheet.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref("/me")}>{t("p.timesheet.back")}</a></p>
<h1>{t("p.timesheet.heading")}</h1>
<p class="small muted">{t("p.timesheet.intro")}</p>

{#if actionError}<p class="banner" role="alert">{actionError}</p>{/if}
<p class="small" role="status">{notice}</p>

<nav class="weeks" aria-label={t("p.timesheet.week_nav")}>
  <button class="button" type="button" onclick={() => goTo(shiftWeek(week, -1))}>{t("p.timesheet.previous_week")}</button>
  <button class="button" type="button" disabled={week === current} onclick={() => goTo(current)}>{t("p.timesheet.this_week")}</button>
  <button class="button" type="button" onclick={() => goTo(shiftWeek(week, 1))}>{t("p.timesheet.next_week")}</button>
  <form class="week-form" onsubmit={showTyped} novalidate>
    <label>{t("p.timesheet.f_week")}
      <input bind:value={weekInput} autocomplete="off" spellcheck="false" maxlength="10" required aria-describedby="ts-week-hint" />
    </label>
    <button class="button" type="submit">{t("p.timesheet.show_week")}</button>
  </form>
</nav>
<p class="small muted" id="ts-week-hint">{t("p.timesheet.week_hint", { today: formatDate(todayIn(tz)), tz })}</p>

<section aria-labelledby="ts-mine-h" data-testid="mine">
  <h2 id="ts-mine-h">{t("p.timesheet.mine_heading", { week })}</h2>
  {#if mineError}<p class="banner" role="alert">{mineError}</p>{/if}
  {#if mine}
    {@const status = statusOf(mine.status)}
    <p class="small muted">{range(mine)}</p>
    <p data-testid="mine-status">
      <span aria-hidden="true">{STATUS_SYMBOL[status]}</span> {t(STATUS_KEY[status])}
      {#if status === "rejected" && mine.decision_comment}
        — {t("p.timesheet.rejected_because", { comment: mine.decision_comment })}
      {/if}
    </p>
    <TimesheetGrid week={mine} {planNames} caption={t("p.timesheet.mine_caption", { week })} />
    {#if canSubmit(status, mine.grid.total)}
      <button class="button primary" type="button" disabled={saving} onclick={() => void submit()}>
        {status === "rejected" ? t("p.timesheet.submit_again") : t("p.timesheet.submit")}
      </button>
    {:else if status === "open"}
      <p class="small muted">{t("p.timesheet.nothing_to_submit")}</p>
    {:else if status === "submitted"}
      <p class="small muted">{t("p.timesheet.waiting")}</p>
    {:else if status === "approved"}
      <p class="small muted">{t("p.timesheet.locked")}</p>
    {/if}
  {:else if !mineError}
    <p class="muted">{t("p.timesheet.loading")}</p>
  {/if}
</section>

<section aria-labelledby="ts-other-h" data-testid="other">
  <h2 id="ts-other-h">{t("p.timesheet.other_heading")}</h2>
  <p class="small muted">{t("p.timesheet.other_intro")}</p>
  <form class="person-form" onsubmit={showPerson} novalidate>
    <label>{t("p.timesheet.f_person")}
      <input bind:value={personInput} autocomplete="off" spellcheck="false" maxlength="200" required />
    </label>
    <button class="button" type="submit">{t("p.timesheet.show_person")}</button>
  </form>
  {#if otherError}<p class="banner" role="alert">{otherError}</p>{/if}
  {#if person && other}
    {@const status = statusOf(other.status)}
    <h3>{t("p.timesheet.other_week_heading", { person, week })}</h3>
    <p class="small muted">{range(other)}</p>
    <p data-testid="other-status">
      <span aria-hidden="true">{STATUS_SYMBOL[status]}</span> {t(STATUS_KEY[status])}
      {#if status !== "open" && status !== "submitted" && other.decision_comment}
        — {t("p.timesheet.decision_comment", { comment: other.decision_comment })}
      {/if}
    </p>
    <TimesheetGrid week={other} {planNames} caption={t("p.timesheet.other_caption", { person, week })} />
    {#if canDecide(status)}
      <form class="decide-form" onsubmit={decide} novalidate>
        <fieldset>
          <legend>{t("p.timesheet.decision_legend")}</legend>
          <label class="inline"><input type="radio" name="decision" value="approved" bind:group={decision} /> {t("p.timesheet.approve")}</label>
          <label class="inline"><input type="radio" name="decision" value="rejected" bind:group={decision} /> {t("p.timesheet.reject")}</label>
        </fieldset>
        <label>{decision === "rejected" ? t("p.timesheet.f_comment_required") : t("p.timesheet.f_comment")}
          <textarea bind:value={comment} rows="3" maxlength="2000"></textarea>
        </label>
        <p class="small muted">{t("p.timesheet.approve_final")}</p>
        <div class="row">
          <button class="button" type="submit" disabled={saving}>{t("p.timesheet.record_decision")}</button>
        </div>
      </form>
    {:else if status === "open"}
      <p class="small muted">{t("p.timesheet.other_not_submitted")}</p>
    {:else if status === "approved"}
      <p class="small muted">{t("p.timesheet.locked")}</p>
    {:else if status === "rejected"}
      <p class="small muted">{t("p.timesheet.other_rejected")}</p>
    {/if}
  {:else if person && !otherError}
    <p class="muted">{t("p.timesheet.loading")}</p>
  {/if}
</section>

<style>
  .weeks {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
    margin-block: 0.75rem 0.25rem;
  }
  .week-form,
  .person-form {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
  }
  .week-form label,
  .person-form label,
  .decide-form label {
    display: grid;
    gap: 0.2rem;
  }
  .person-form input {
    min-inline-size: min(26rem, 100%);
  }
  .decide-form {
    display: grid;
    gap: 0.5rem;
    max-inline-size: 32rem;
    margin-block: 1rem;
  }
  .decide-form label.inline {
    display: inline-flex;
    gap: 0.35rem;
    align-items: center;
    margin-inline-end: 1rem;
  }
  section {
    margin-block: 1.5rem;
  }
</style>
