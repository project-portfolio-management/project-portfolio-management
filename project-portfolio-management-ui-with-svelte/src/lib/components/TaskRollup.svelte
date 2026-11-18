<!--
  A task's sub-tasks rolled up (`/plans/[pid]/tasks/[tpid]`, NC-1 slice 12 part i, RM-5): `GET
  /api/plans/{pid}/tasks/{t_pid}/rollup`, derived on read and never stored: how many sub-tasks and how many done, the
  latest due date among them, the points summed, progress weighted by the estimated ones only (the unestimated named,
  never counted as zero) and the time recorded across the family. An absent figure says why in words. Rules in
  `src/lib/plan-readings-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatBasisPoints, formatDate, formatInteger } from "#lib/locale-format.js";
  import { durationText } from "#lib/timesheet-view.js";
  import { PpmClient, type TaskRollup } from "#lib/api/ppm.js";
  import { derivedKey, readingRefusalKey } from "#lib/plan-readings-view.js";

  let { pid, taskPid, titles }: { pid: string; taskPid: string; titles: ReadonlyMap<string, string> } = $props();

  const ppm = PpmClient.withFetch();
  let data = $state<TaskRollup | null>(null);
  let loadError = $state<string | null>(null);

  onMount(async () => {
    try {
      data = await ppm.taskRollup(pid, taskPid);
    } catch (err) {
      const key = readingRefusalKey("task_rollup", err);
      loadError = key ? t(key) : errorText(err, "p.taskrollup.could_not_load");
    }
  });

  const r = $derived(data?.rollup ?? null);
</script>

<section aria-labelledby="rollup-h" data-testid="task-rollup">
  <h2 id="rollup-h">{t("p.taskrollup.heading")}</h2>
  <p class="small muted">{t("p.taskrollup.says")}</p>
  {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
  {#if r}
    {#if r.descendants === 0}
      <p>{t("p.taskrollup.no_children")}</p>
    {:else}
      {@const due = derivedKey(r.due)}
      {@const est = derivedKey(r.estimate_points)}
      {@const prog = derivedKey(r.progress_bp)}
      <dl>
        <dt>{t("p.taskrollup.subtasks")}</dt>
        <dd>{tn("p.taskrollup.done_of", r.descendants, { n: formatInteger(r.descendants), done: formatInteger(r.done) })}</dd>
        <dt>{t("p.taskrollup.due")}</dt>
        <dd>{r.due.value ? formatDate(r.due.value) : due ? t(due) : ""}</dd>
        <dt>{t("p.taskrollup.points")}</dt>
        <dd>{r.estimate_points.value !== null ? formatInteger(r.estimate_points.value) : est ? t(est) : ""}</dd>
        <dt>{t("p.taskrollup.progress")}</dt>
        <dd>{r.progress_bp.value !== null ? formatBasisPoints(r.progress_bp.value, 0) : prog ? t(prog) : ""}</dd>
        <dt>{t("p.taskrollup.spent")}</dt>
        <dd>{durationText(r.spent_minutes)}</dd>
      </dl>
      {#if r.excluded_unestimated.length > 0}
        <p class="small">{tn("p.taskrollup.unestimated", r.excluded_unestimated.length, { n: r.excluded_unestimated.length })}</p>
        <ul class="small">
          {#each r.excluded_unestimated as p (p)}<li>{titles.get(p) ?? t("p.taskrollup.unnamed", { ref: p.slice(0, 8) })}</li>{/each}
        </ul>
      {/if}
    {/if}
  {/if}
</section>
