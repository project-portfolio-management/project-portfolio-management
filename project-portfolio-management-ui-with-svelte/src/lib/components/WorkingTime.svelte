<!--
  The working day and time away (`/plans/[pid]/calendar`, NC-1 slice 12 part i): a working day and week for this plan
  or as the organization's default (`POST /api/working-time`), and a person's leave or other time away from project
  work (`POST /api/non-working`). Each is a new record that cannot be changed or removed here, so each is asked first
  (Cancel sends nothing); every rule is said and checked first, every problem at once; a blank note is not sent. Rules
  in `src/lib/working-time-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, tn, type StringKey } from "#lib/i18n.svelte.js";
  import { formatDateRange } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { durationText } from "#lib/timesheet-view.js";
  import { PpmClient } from "#lib/api/ppm.js";
  import {
    NON_WORKING_KEY,
    NON_WORKING_KINDS,
    emptyNonWorkingForm,
    emptyWorkingTimeForm,
    nonWorkingBody,
    workingTimeBody,
    workingTimeRefusalKey,
    type NonWorkingForm,
    type WorkingTimeForm,
  } from "#lib/working-time-view.js";

  let { pid }: { pid: string } = $props();

  const ppm = PpmClient.withFetch();
  let wform = $state<WorkingTimeForm>(emptyWorkingTimeForm());
  let nform = $state<NonWorkingForm>(emptyNonWorkingForm());
  let wProblem = $state<string | null>(null);
  let nProblem = $state<string | null>(null);
  let wNotice = $state("");
  let nNotice = $state("");
  let busy = $state(false);

  function why(action: "working_time" | "non_working", err: unknown, fallback: StringKey): string {
    const key = workingTimeRefusalKey(action, err);
    return key ? t(key) : errorText(err, fallback);
  }

  async function saveWorking(e: SubmitEvent) {
    e.preventDefault();
    wNotice = "";
    const r = workingTimeBody(wform, pid);
    if ("problems" in r) {
      wProblem = r.problems.map((k) => t(k)).join(" ");
      return;
    }
    wProblem = null;
    const words = { day: durationText(r.body.minutes_per_day), days: r.body.working_days_per_week };
    if (!(await askConfirm(tn(r.body.scope_ref ? "p.workingtime.confirm_plan" : "p.workingtime.confirm_default", words.days, words)))) return;
    busy = true;
    try {
      await ppm.setWorkingTime(r.body);
      wNotice = tn("p.workingtime.saved", words.days, words);
    } catch (err) {
      wProblem = why("working_time", err, "p.workingtime.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function saveAway(e: SubmitEvent) {
    e.preventDefault();
    nNotice = "";
    const r = nonWorkingBody(nform);
    if ("problems" in r) {
      nProblem = r.problems.map((k) => t(k)).join(" ");
      return;
    }
    nProblem = null;
    const words = { person: r.body.person_ref, range: formatDateRange(r.body.starts_on, r.body.ends_on), kind: t(NON_WORKING_KEY[r.body.kind]) };
    if (!(await askConfirm(t("p.workingtime.confirm_away", words)))) return;
    busy = true;
    try {
      await ppm.recordNonWorking(r.body);
      nNotice = t("p.workingtime.recorded", words);
      nform = emptyNonWorkingForm();
    } catch (err) {
      nProblem = why("non_working", err, "p.workingtime.could_not_save");
    } finally {
      busy = false;
    }
  }
</script>

<section aria-labelledby="wt-h" data-testid="working-time">
  <h2 id="wt-h">{t("p.workingtime.heading")}</h2>
  <p class="small muted">{t("p.workingtime.says")}</p>
  {#if wProblem}<p class="banner error" role="alert">{wProblem}</p>{/if}
  <p class="small" role="status">{wNotice}</p>
  <form class="stack" novalidate onsubmit={saveWorking} aria-label={t("p.workingtime.form")}>
    <fieldset>
      <legend>{t("p.workingtime.f_scope")}</legend>
      <label class="inline"><input type="radio" name="wt-scope" value="plan" bind:group={wform.scope} /> {t("p.workingtime.scope_plan")}</label>
      <label class="inline"><input type="radio" name="wt-scope" value="default" bind:group={wform.scope} /> {t("p.workingtime.scope_default")}</label>
    </fieldset>
    <div class="row">
      <label>{t("p.workingtime.f_hours")}<input type="number" min="0" max="24" step="1" bind:value={wform.hours} /></label>
      <label>{t("p.workingtime.f_minutes")}<input type="number" min="0" max="59" step="1" bind:value={wform.minutes} /></label>
      <label>{t("p.workingtime.f_days")}<input type="number" min="1" max="7" step="1" bind:value={wform.days} /></label>
    </div>
    <div><button class="button" type="submit" disabled={busy}>{t("p.workingtime.save")}</button></div>
  </form>

  <h3 id="away-h">{t("p.workingtime.away_heading")}</h3>
  <p class="small muted">{t("p.workingtime.away_says")}</p>
  {#if nProblem}<p class="banner error" role="alert">{nProblem}</p>{/if}
  <p class="small" role="status">{nNotice}</p>
  <form class="stack" novalidate onsubmit={saveAway} aria-labelledby="away-h">
    <label>{t("p.workingtime.f_person")}<input bind:value={nform.person} autocomplete="off" maxlength="120" /></label>
    <div class="row">
      <label>{t("p.workingtime.f_starts")}<input type="date" bind:value={nform.startsOn} /></label>
      <label>{t("p.workingtime.f_ends")}<input type="date" bind:value={nform.endsOn} /></label>
    </div>
    <label>{t("p.workingtime.f_kind")}
      <select bind:value={nform.kind}>
        {#each NON_WORKING_KINDS as k (k)}<option value={k}>{t(NON_WORKING_KEY[k])}</option>{/each}
      </select>
    </label>
    <label>{t("p.workingtime.f_note")}<input bind:value={nform.note} maxlength="1000" /></label>
    <div><button class="button" type="submit" disabled={busy}>{t("p.workingtime.record")}</button></div>
  </form>
</section>

<style>
  form { max-inline-size: 32rem; margin-block: 1rem; }
  label { display: grid; gap: 0.2rem; }
  label.inline { display: inline-flex; gap: 0.3rem; margin-inline-end: 1rem; }
  .row { display: flex; flex-wrap: wrap; gap: 0.5rem 1rem; }
  fieldset { border: 1px solid var(--border, #ccc); padding: 0.5rem 0.75rem; }
</style>
