<!--
  One milestone: its status against the calendar (a word and a shape), whether
  it is achieved and how, the forecast from the critical-path analysis of the
  tasks it waits for, how far it has slipped, and the forms to complete,
  reopen, reschedule and link tasks. A milestone with nothing to forecast from
  says so; it is never called "on track" by default.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import type { MilestoneView, MilestoneMode, PpmClient, Task } from "#lib/api/ppm.js";
  import {
    STATE_TEXT,
    STATUS_SYMBOL,
    STATUS_TEXT,
    achievedText,
    completionNeedsReason,
    forecastText,
    openTasksText,
    slipText,
  } from "#lib/milestone-view.js";

  let {
    item,
    ppm,
    onchange,
    onerror,
  }: {
    item: MilestoneView;
    ppm: PpmClient;
    onchange: () => void;
    onerror: (message: string) => void;
  } = $props();

  async function run(action: () => Promise<unknown>) {
    try {
      await action();
      onchange();
    } catch (err) {
      onerror(errorText(err, "c.MilestoneCard.that_did_not_work"));
    }
  }

  const needsReason = $derived(completionNeedsReason(item));
  let completeReason = $state("");
  const complete = (e: SubmitEvent) => {
    e.preventDefault();
    void run(async () => {
      await ppm.completeMilestone(item.plan_pid, item.pid, completeReason.trim() || undefined);
      completeReason = "";
    });
  };

  let reopenReason = $state("");
  const reopen = (e: SubmitEvent) => {
    e.preventDefault();
    void run(async () => {
      await ppm.reopenMilestone(item.plan_pid, item.pid, reopenReason.trim());
      reopenReason = "";
    });
  };

  let newDue = $state("");
  let moveReason = $state("");
  const reschedule = (e: SubmitEvent) => {
    e.preventDefault();
    void run(async () => {
      await ppm.rescheduleMilestone(item.plan_pid, item.pid, newDue, moveReason.trim() || undefined);
      newDue = "";
      moveReason = "";
    });
  };

  // Linking tasks: load the plan's tasks when the section opens.
  let planTasks = $state<Task[] | null>(null);
  let chosen = $state<Record<string, boolean>>({});
  let mode = $state<MilestoneMode>("linked");
  async function openLinks(e: Event) {
    if (!(e.currentTarget as HTMLDetailsElement).open) return;
    try {
      planTasks = (await ppm.listTasks(item.plan_pid)).tasks;
      chosen = Object.fromEntries(item.links.map((l) => [l.task_pid, true]));
      mode = item.completion_mode;
    } catch (err) {
      onerror(errorText(err, "c.MilestoneCard.could_not_load_the_plan"));
    }
  }
  const saveLinks = (e: SubmitEvent) => {
    e.preventDefault();
    const picked = Object.entries(chosen).filter(([, on]) => on).map(([pid]) => pid);
    void run(() => ppm.setMilestoneLinks(item.plan_pid, item.pid, picked, picked.length === 0 ? "manual" : mode));
  };

  let history = $state<NonNullable<MilestoneView["history"]> | null>(null);
  async function openHistory(e: Event) {
    if (!(e.currentTarget as HTMLDetailsElement).open) return;
    try {
      history = (await ppm.milestone(item.pid)).history ?? [];
    } catch (err) {
      onerror(errorText(err, "c.MilestoneCard.could_not_load_the_history"));
    }
  }

  const historyColumns: DataGridColumn[] = $derived([
    { id: "from", header: t("c.MilestoneCard.from"), rowHeader: true },
    { id: "to", header: t("c.MilestoneCard.to") },
    { id: "reason", header: t("c.MilestoneCard.why"), format: (v) => (v ? String(v) : t("c.MilestoneCard.no_reason_given")) },
    { id: "actor", header: t("c.MilestoneCard.by"), format: (v) => (v ? String(v) : "") },
  ]);
</script>

<article class={`surface milestone status-${item.status}`} aria-labelledby={`m-${item.pid}`}>
  <header>
    <h3 id={`m-${item.pid}`}>{item.name}</h3>
    <p class="status">
      <span aria-hidden="true">{STATUS_SYMBOL[item.status]}</span>
      <strong>{STATUS_TEXT[item.status]}</strong>
    </p>
  </header>
  <p class="small">
    {t("c.MilestoneCard.due", { due: item.due, kind: item.kind })}
    {#if item.schedule.state}· <strong>{STATE_TEXT[item.schedule.state]}</strong>{/if}
    · <a href={localeHref(`/plans/${item.plan_pid}`)}>{t("c.MilestoneCard.view_plan")}</a>
  </p>
  {#if forecastText(item.schedule, item.due)}<p class="small">{forecastText(item.schedule, item.due)}</p>{/if}
  {#if achievedText(item)}<p class="small">{achievedText(item)}</p>{/if}
  {#if openTasksText(item)}<p class="small muted">{openTasksText(item)}</p>{/if}
  {#if slipText(item.slip)}<p class="small muted">{slipText(item.slip)}</p>{/if}

  {#if item.links.length > 0}
    <ul class="small links" aria-label={t("c.MilestoneCard.tasks_this_milestone_waits_for")}>
      {#each item.links as l (l.task_pid)}
        <li>
          {l.complete ? t("c.MilestoneCard.done") : t("c.MilestoneCard.open")}: {l.title}
          {#if !l.complete && l.early_finish}<span class="muted"> {t("c.MilestoneCard.forecast", { early_finish: l.early_finish })}</span>{/if}
        </li>
      {/each}
    </ul>
  {/if}

  {#if !item.achieved}
    <details>
      <summary>{t("c.MilestoneCard.complete")}</summary>
      <form class="inline" onsubmit={complete}>
        <label>
          {t("c.MilestoneCard.reason")}{needsReason ? t("c.MilestoneCard.reason_required_open") : t("c.MilestoneCard.reason_optional")}
          <input type="text" bind:value={completeReason} required={needsReason} />
        </label>
        <button class="button small" type="submit">{t("c.MilestoneCard.mark_done")}</button>
      </form>
    </details>
  {:else if item.achieved_by === "manual"}
    <details>
      <summary>{t("c.MilestoneCard.reopen")}</summary>
      <form class="inline" onsubmit={reopen}>
        <label>{t("c.MilestoneCard.reason_required")} <input type="text" required bind:value={reopenReason} /></label>
        <button class="button small" type="submit">{t("c.MilestoneCard.reopen")}</button>
      </form>
    </details>
  {/if}

  <details>
    <summary>{t("c.MilestoneCard.reschedule")}</summary>
    <form class="inline" onsubmit={reschedule}>
      <label>{t("c.MilestoneCard.new_due_date")} <input type="date" required bind:value={newDue} /></label>
      <label>{t("c.MilestoneCard.why")} <input type="text" bind:value={moveReason} /></label>
      <button class="button small" type="submit">{t("c.MilestoneCard.move")}</button>
    </form>
    <p class="small muted">{t("c.MilestoneCard.every_move_is_kept_in")}</p>
  </details>

  <details ontoggle={openLinks}>
    <summary>{t("c.MilestoneCard.tasks_it_waits_for")}</summary>
    {#if planTasks === null}
      <p class="small muted">{t("c.MilestoneCard.loading")}</p>
    {:else if planTasks.length === 0}
      <p class="small muted">{t("c.MilestoneCard.this_plan_has_no_tasks")}</p>
    {:else}
      <form onsubmit={saveLinks}>
        <fieldset>
          <legend class="small">{t("c.MilestoneCard.pick_the_tasks_that_must")}</legend>
          {#each planTasks as t (t.pid)}
            <label class="check"><input type="checkbox" bind:checked={chosen[t.pid]} /> {t.title}</label>
          {/each}
        </fieldset>
        <label class="inline-label">
          {t("c.MilestoneCard.completes")}
          <select bind:value={mode}>
            <option value="linked">{t("c.MilestoneCard.when_all_those_tasks_are")}</option>
            <option value="manual">{t("c.MilestoneCard.by_hand_only")}</option>
          </select>
        </label>
        <button class="button small" type="submit">{t("c.MilestoneCard.save")}</button>
      </form>
    {/if}
  </details>

  <details ontoggle={openHistory}>
    <summary>{t("c.MilestoneCard.slip_history", { reschedules: item.slip.reschedules })}</summary>
    {#if history === null}
      <p class="small muted">{t("c.MilestoneCard.loading")}</p>
    {:else if history.length === 0}
      <p class="small muted">{t("c.MilestoneCard.never_rescheduled")}</p>
    {:else}
      <DataTable label={t("c.MilestoneCard.from")} columns={historyColumns} rows={history} rowId={(r) => String(r.at) + String(r.to)} sortable={false} />
    {/if}
  </details>
</article>

<style>
  .milestone header {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 1rem;
  }
  .status {
    margin: 0;
    white-space: nowrap;
  }
  .status-late .status {
    color: var(--danger, #b00020);
  }
  .links {
    padding-left: 1.25rem;
  }
  .inline {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: end;
    margin: 0.5rem 0;
  }
  .inline label,
  .inline-label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
  .check {
    display: block;
    font-size: 0.9rem;
  }
</style>
