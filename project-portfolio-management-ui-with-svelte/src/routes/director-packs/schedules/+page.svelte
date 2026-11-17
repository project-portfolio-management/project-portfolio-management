<!--
  Director pack schedules (`/director-packs/schedules`, DP-12, NC-1 slice 11): each audience's pack drafted on a day of
  each month, never issued; a person reviews and issues it. Editors add a schedule, move its day, switch it off and on,
  and run what is due now. The service decides who may; a refusal says which right is needed. The last run's result is
  shown in the reader's language when it is one the page knows ("drafted" or "skipped"), never as the service's text.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { PpmClient, type Organization, type PackSchedule } from "#lib/api/ppm.js";
  import { PACK_AUDIENCES, dayProblem, lastResult, rhythmRefusalKey } from "#lib/pack-actions.js";

  const ppm = PpmClient.withFetch();

  let schedules = $state<PackSchedule[] | null>(null);
  let orgs = $state<Organization[]>([]);
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);
  let audience = $state("");
  let day = $state<number | null>(1);
  let days = $state<Record<string, number | null>>({});

  const audienceName = (a: string): string => {
    const builtIn = PACK_AUDIENCES.find((x) => x.key === a);
    if (builtIn) return t(builtIn.label);
    const org = orgs.find((o) => o.audience === a);
    return org ? t("p.director_pack.aud_org", { name: org.name }) : a;
  };
  const scheduled = $derived(new Set((schedules ?? []).map((s) => s.audience)));

  async function load() {
    loadError = null;
    try {
      schedules = (await ppm.packSchedules()).schedules;
      days = Object.fromEntries(schedules.map((s) => [s.pid, s.day_of_month]));
    } catch (err) {
      loadError = rhythmRefusalKey("schedule", err) ? t("p.director_pack.refused_edit") : errorText(err, "p.director_pack.could_not_load_schedules");
    }
  }
  async function loadOrgs() {
    try {
      orgs = (await ppm.organizations()).organizations.filter((o) => o.active);
    } catch {
      orgs = [];
    }
  }
  onMount(() => {
    void load();
    void loadOrgs();
  });

  async function run(change: () => Promise<unknown>, done: string) {
    error = null;
    notice = "";
    busy = true;
    try {
      await change();
      notice = done;
      await load();
    } catch (err) {
      const k = rhythmRefusalKey("schedule", err);
      error = k ? t(k) : errorText(err, "p.director_pack.could_not_change_schedule");
    } finally {
      busy = false;
    }
  }

  function add(e: SubmitEvent) {
    e.preventDefault();
    error = null;
    notice = "";
    if (!audience) {
      error = t("p.director_pack.problem_audience");
      return;
    }
    const problem = dayProblem(String(day ?? ""));
    if (problem) {
      error = t(problem);
      return;
    }
    void run(() => ppm.createPackSchedule({ audience, day_of_month: Number(day) }), t("p.director_pack.schedule_added"));
  }
  function moveDay(s: PackSchedule) {
    const d = String(days[s.pid] ?? "");
    const problem = dayProblem(d);
    if (problem) {
      error = t(problem);
      notice = "";
      return;
    }
    if (Number(d) === s.day_of_month) {
      error = null;
      notice = t("p.director_pack.schedule_unchanged");
      return;
    }
    void run(() => ppm.updatePackSchedule(s.pid, { day_of_month: Number(d) }), t("p.director_pack.schedule_saved"));
  }
  async function runSweep() {
    error = null;
    notice = "";
    busy = true;
    try {
      const r = await ppm.sweepPackSchedules();
      notice = t("p.director_pack.sweep_result", { drafted: r.drafted.length, skipped: r.skipped });
      await load();
    } catch (err) {
      const k = rhythmRefusalKey("schedule", err);
      error = k ? t(k) : errorText(err, "p.director_pack.could_not_change_schedule");
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>{t("p.director_pack.schedules_title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref("/director-packs")}>{t("p.director_pack.all_packs")}</a></p>
<h1>{t("p.director_pack.schedules_title")}</h1>
<p class="small muted">{t("p.director_pack.schedules_intro")}</p>
{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}

{#if schedules}
  {#if error}<p class="banner error" role="alert">{error}</p>{/if}
  <p class="small" role="status">{notice}</p>
  {#if schedules.length === 0}
    <p class="muted">{t("p.director_pack.no_schedules")}</p>
  {:else}
    <ul class="schedules">
      {#each schedules as s (s.pid)}
        {@const last = lastResult(s.last_result)}
        <li>
          <h2>{audienceName(s.audience)}</h2>
          <p>
            {s.enabled
              ? t("p.director_pack.schedule_on", { day: s.day_of_month, next: formatDate(s.next_run_date) })
              : t("p.director_pack.schedule_off", { day: s.day_of_month })}
          </p>
          {#if s.last_run_on}
            <p class="small muted">
              {last
                ? t(last.key, { period: last.period, when: formatDate(s.last_run_on) })
                : t("p.director_pack.last_run", { when: formatDate(s.last_run_on) })}
            </p>
          {/if}
          <div class="row">
            <label>
              {t("p.director_pack.f_day")}
              <input type="number" min="1" max="28" step="1" bind:value={days[s.pid]} />
            </label>
            <button class="button small" type="button" disabled={busy} onclick={() => moveDay(s)} aria-label={`${t("p.director_pack.move_day")}: ${audienceName(s.audience)}`}>{t("p.director_pack.move_day")}</button>
            <button
              class="button small"
              type="button"
              disabled={busy}
              onclick={() => run(() => ppm.updatePackSchedule(s.pid, { enabled: !s.enabled }), s.enabled ? t("p.director_pack.schedule_switched_off") : t("p.director_pack.schedule_switched_on"))}
              aria-label={`${s.enabled ? t("p.director_pack.switch_off") : t("p.director_pack.switch_on")}: ${audienceName(s.audience)}`}
            >
              {s.enabled ? t("p.director_pack.switch_off") : t("p.director_pack.switch_on")}
            </button>
          </div>
        </li>
      {/each}
    </ul>
  {/if}

  <section aria-labelledby="sweep-h">
    <h2 id="sweep-h">{t("p.director_pack.sweep_heading")}</h2>
    <p class="small muted">{t("p.director_pack.sweep_says")}</p>
    <button class="button" type="button" disabled={busy} onclick={runSweep}>{t("p.director_pack.sweep")}</button>
  </section>

  <section aria-labelledby="add-schedule-h">
    <h2 id="add-schedule-h">{t("p.director_pack.add_schedule_heading")}</h2>
    <!-- novalidate: the page says a day out of range in the reader's language, not the browser's bubble. -->
    <form class="stack" novalidate onsubmit={add}>
      <label>
        {t("p.director_pack.f_audience")}
        <select bind:value={audience}>
          <option value="">{t("p.director_pack.choose_audience")}</option>
          {#each PACK_AUDIENCES.filter((a) => !scheduled.has(a.key)) as a (a.key)}<option value={a.key}>{t(a.label)}</option>{/each}
          {#each orgs.filter((o) => !scheduled.has(o.audience)) as o (o.pid)}<option value={o.audience}>{t("p.director_pack.aud_org", { name: o.name })}</option>{/each}
        </select>
      </label>
      <label>
        {t("p.director_pack.f_day")}
        <input type="number" min="1" max="28" step="1" bind:value={day} />
      </label>
      <p class="small muted">{t("p.director_pack.day_says")}</p>
      <div class="row">
        <button class="button primary" type="submit" disabled={busy}>{t("p.director_pack.add_schedule")}</button>
      </div>
    </form>
  </section>
{/if}

<style>
  section {
    margin-block-start: 1.5rem;
  }
  .schedules {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 1rem;
  }
  .schedules h2 {
    font-size: 1.05rem;
    margin-block: 0 0.25rem;
  }
  .schedules p {
    margin-block: 0 0.25rem;
  }
  .row label {
    display: inline-flex;
    gap: 0.4rem;
    align-items: center;
  }
  .row input {
    inline-size: 5rem;
  }
</style>
