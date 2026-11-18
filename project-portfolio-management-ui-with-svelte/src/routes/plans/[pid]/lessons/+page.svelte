<!--
  A plan's lessons learned and what refers to a record (`/plans/[pid]/lessons`, NC-8, BR-4, NC-1 slice 12 part e).
  A lesson is recorded at a phase gate or at closure, as something that went well, went badly, or a recommendation,
  with a summary, an optional detail and optionally the plan's task, risk, milestone or issue it is about. A lesson is
  never changed; it is withdrawn after a question (Cancel sends nothing), and then no longer shown. "What refers to a
  record" lists the meetings and lessons that point at one of the plan's records. Rules in `src/lib/lesson-view.ts`;
  refusals are the catalogue's sentences, never the service's text.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { localeHref, t, tn } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { PpmClient, type Lesson, type LessonCategory, type RecordReferences } from "#lib/api/ppm.js";
  import RecordPicker from "#lib/components/RecordPicker.svelte";
  import { loadPlanRecords, recordTitle, recordWords, type PlanRecords } from "#lib/plan-records.js";
  import { OUTCOME_KEY, STATUS_MARK, linkOf } from "#lib/meeting-view.js";
  import {
    CATEGORIES,
    CATEGORY_MARK,
    DETAIL_MAX_BYTES,
    OCCASIONS,
    SUMMARY_MAX,
    isGate,
    lessonBody,
    lessonRefusalKey,
    type LessonAction,
  } from "#lib/lesson-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let lessons = $state<Lesson[] | null>(null);
  let loadError = $state<string | null>(null);
  let records = $state<PlanRecords>({});
  let recordsLoaded = $state(false);
  let refusal = $state("");
  let notice = $state("");
  let busy = $state(false);

  let occasion = $state("");
  let category = $state<LessonCategory | "">("");
  let summary = $state("");
  let detail = $state("");
  let link = $state("");
  let problems = $state<string[]>([]);

  let target = $state("");
  let refs = $state<RecordReferences | null>(null);
  let refsError = $state("");

  const occasionLabel = (o: string) => (isGate(o) ? enumLabel("pk_gate", o) : o === "closure" ? t("p.lessons.closure") : t("p.lessons.occasion_other"));
  const categoryWord = (c: LessonCategory) => (CATEGORY_MARK[c] ? t(CATEGORY_MARK[c].key) : t("p.lessons.category_other"));
  const fill = { max: SUMMARY_MAX, bytes: DETAIL_MAX_BYTES };

  async function load() {
    try {
      lessons = await ppm.lessons(pid);
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.lessons.could_not_load");
    }
  }

  onMount(async () => {
    await load();
    records = await loadPlanRecords(ppm, pid);
    recordsLoaded = true;
  });

  function refused(action: LessonAction, err: unknown, fallback: Parameters<typeof t>[0]) {
    const k = lessonRefusalKey(action, err);
    refusal = k ? t(k) : errorText(err, fallback);
    notice = "";
  }

  async function record(e: SubmitEvent) {
    e.preventDefault();
    refusal = "";
    notice = "";
    const r = lessonBody({ occasion, category, summary, detail, link });
    if ("problems" in r) {
      problems = r.problems.map((k) => t(k, fill));
      return;
    }
    problems = [];
    busy = true;
    try {
      await ppm.recordLesson(pid, r.body);
      notice = t("p.lessons.recorded", { summary: r.body.summary });
      occasion = "";
      category = "";
      summary = "";
      detail = "";
      link = "";
      await load();
    } catch (err) {
      refused("record", err, "p.lessons.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function withdraw(l: Lesson) {
    refusal = "";
    notice = "";
    if (!(await askConfirm(t("p.lessons.confirm_withdraw", { summary: l.summary })))) return;
    busy = true;
    try {
      await ppm.withdrawLesson(pid, l.pid);
      notice = t("p.lessons.withdrawn", { summary: l.summary });
    } catch (err) {
      refused("withdraw", err, "p.lessons.could_not_save");
    } finally {
      busy = false;
      await load();
    }
  }

  async function lookUp(e: SubmitEvent) {
    e.preventDefault();
    refsError = "";
    const chosen = linkOf(target);
    if (!chosen) {
      refs = null;
      refsError = t("p.lessons.problem_target");
      return;
    }
    try {
      refs = await ppm.references(pid, chosen.kind, chosen.pid);
    } catch (err) {
      refs = null;
      const k = lessonRefusalKey("references", err);
      refsError = k ? t(k) : errorText(err, "p.lessons.could_not_read_refs");
    }
  }

  /** The words for what a lesson is about, from the records read; one not listed is said to be not listed. */
  function aboutWords(l: Lesson): string | null {
    if (!l.link_kind || !l.link_pid) return null;
    const title = recordTitle(records, l.link_kind, l.link_pid);
    return title === null ? t("p.lessons.link_unlisted") : t("p.meetings.about", { record: recordWords(l.link_kind, title) });
  }
</script>

<svelte:head><title>{t("p.lessons.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a> · <a href={localeHref(`/plans/${pid}/meetings`)}>{t("p.meetings.link")}</a></p>
<h1>{t("p.lessons.title")}</h1>
<p class="small muted" data-testid="lessons-rules">{t("p.lessons.intro")}</p>
{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}
{#if refusal}<p class="banner error" role="alert" data-testid="lessons-refusal">{refusal}</p>{/if}
<p class="small" role="status" data-testid="page-notice">{notice}</p>

<section aria-labelledby="lesson-record-h">
  <h2 id="lesson-record-h">{t("p.lessons.record_heading")}</h2>
  <p class="small muted">{t("p.lessons.record_says", fill)}</p>
  <form class="stack" novalidate onsubmit={record}>
    <label>
      {t("p.lessons.f_occasion")}
      <select bind:value={occasion}>
        <option value="">{t("p.lessons.choose_occasion")}</option>
        {#each OCCASIONS as o (o)}<option value={o}>{occasionLabel(o)}</option>{/each}
      </select>
    </label>
    <fieldset class="choices">
      <legend>{t("p.lessons.f_category")}</legend>
      {#each CATEGORIES as c (c)}
        <label class="check"><input type="radio" name="lesson-category" value={c} bind:group={category} /><span aria-hidden="true">{CATEGORY_MARK[c].symbol}</span> {t(CATEGORY_MARK[c].key)}</label>
      {/each}
    </fieldset>
    <label>{t("p.lessons.f_summary", fill)}<input bind:value={summary} /></label>
    <label>{t("p.lessons.f_detail", fill)}<textarea bind:value={detail} rows="3"></textarea></label>
    <RecordPicker label={t("p.lessons.f_link")} bind:value={link} {records} loaded={recordsLoaded} none={t("p.meetings.no_record")} />
    {#if problems.length > 0}<ul class="banner error" role="alert">{#each problems as p (p)}<li>{p}</li>{/each}</ul>{/if}
    <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.lessons.record")}</button></div>
  </form>
</section>

<section aria-labelledby="lesson-list-h">
  <h2 id="lesson-list-h">{t("p.lessons.list_heading")}</h2>
  {#if lessons === null}
    {#if !loadError}<p class="muted">{t("p.lessons.loading")}</p>{/if}
  {:else if lessons.length === 0}
    <p class="muted">{t("p.lessons.none")}</p>
  {:else}
    <p class="small">{tn("p.lessons.count", lessons.length, { n: lessons.length })}</p>
    <ul class="lessons" aria-label={t("p.lessons.list_heading")}>
      {#each lessons as l (l.pid)}
        {@const about = aboutWords(l)}
        <li>
          <p>
            <strong><span aria-hidden="true">{CATEGORY_MARK[l.category]?.symbol ?? "·"}</span> {categoryWord(l.category)}</strong>
            · {occasionLabel(l.occasion)}
            · {l.recorded_by
              ? t("p.lessons.recorded_on_by", { date: formatDate(l.created_at), who: l.recorded_by })
              : t("p.lessons.recorded_on", { date: formatDate(l.created_at) })}
          </p>
          <p>{l.summary}</p>
          {#if l.detail}<p class="small detail">{l.detail}</p>{/if}
          {#if about}<p class="small muted">{about}</p>{/if}
          <button class="button small" type="button" disabled={busy} aria-label={t("p.lessons.withdraw", { summary: l.summary })} onclick={() => withdraw(l)}>{t("p.lessons.withdraw_short")}</button>
        </li>
      {/each}
    </ul>
  {/if}
</section>

<section aria-labelledby="lesson-refs-h">
  <h2 id="lesson-refs-h">{t("p.lessons.refs_heading")}</h2>
  <p class="small muted">{t("p.lessons.refs_says")}</p>
  <form class="row" novalidate onsubmit={lookUp}>
    <RecordPicker label={t("p.lessons.f_target")} bind:value={target} {records} loaded={recordsLoaded} none={t("p.lessons.choose_target")} />
    <button class="button" type="submit" disabled={!recordsLoaded}>{t("p.lessons.show_refs")}</button>
  </form>
  {#if refsError}<p class="banner error" role="alert">{refsError}</p>{/if}
  {#if refs}
    <div data-testid="refs">
      <p><strong>{recordWords(refs.target.kind, refs.target.title)}</strong>: {tn("p.lessons.refs_count", refs.count, { n: refs.count })}</p>
      {#if refs.meetings.length > 0}
        <h3>{t("p.lessons.refs_meetings")}</h3>
        <ul aria-label={t("p.lessons.refs_meetings")}>
          {#each refs.meetings as m, i (i)}
            <li>
              <a href={localeHref(`/plans/${pid}/meetings`)}>{m.meeting_title}</a>
              (<span aria-hidden="true">{STATUS_MARK[m.meeting_status]?.symbol ?? "·"}</span> {STATUS_MARK[m.meeting_status] ? t(STATUS_MARK[m.meeting_status].key) : ""}, {formatDate(m.scheduled_on)}):
              {m.part === "agenda" ? t("p.lessons.refs_agenda") : m.outcome_kind ? t(OUTCOME_KEY[m.outcome_kind]) : t("p.meetings.outcome_note")}: {m.text}
            </li>
          {/each}
        </ul>
      {/if}
      {#if refs.lessons.length > 0}
        <h3>{t("p.lessons.refs_lessons")}</h3>
        <ul aria-label={t("p.lessons.refs_lessons")}>
          {#each refs.lessons as l (l.lesson_pid)}
            <li><span aria-hidden="true">{CATEGORY_MARK[l.category]?.symbol ?? "·"}</span> {categoryWord(l.category)} · {occasionLabel(l.occasion)}: {l.summary}</li>
          {/each}
        </ul>
      {/if}
    </div>
  {/if}
</section>

<style>
  section {
    margin-block-start: 2rem;
  }
  form.stack {
    max-width: 40rem;
  }
  .choices {
    border: 0;
    padding: 0.4rem 0;
    margin: 0;
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }
  .check {
    display: inline-flex;
    gap: 0.35rem;
    align-items: center;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
  }
  .lessons {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 1rem;
  }
  .lessons p {
    margin-block: 0.2rem;
  }
  .detail {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }
</style>
