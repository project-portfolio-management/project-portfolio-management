<!--
  Workflow automation + set and forget (`/automations`): the rules that
  fire as work crosses the Kanban board, the honest log of what each
  firing actually did, and the deadline queue that runs itself.

  Three things this page deliberately shows rather than hides: a rule's
  on/off state (disabling keeps the rule and its history), every run
  including the skipped and failed ones, and whether a sweep hit its
  per-run cap.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import {
    CapabilityClient,
    type ActionKind,
    type Automation,
    type AutomationRun,
    type ScheduledAction,
    type TriggerKind,
  } from "#lib/api/capabilities.js";

  const api = CapabilityClient.withFetch();
  let automations = $state<Automation[]>([]);
  let runs = $state<AutomationRun[]>([]);
  let scheduled = $state<ScheduledAction[]>([]);
  // Rows matching overall, from `X-Total-Count` — not `<list>.length`,
  // which is only this page. Shown so an operator can tell a short list
  // from a first page of a long one (agents/share/restful.md).
  let automationsTotal = $state(0);
  let runsTotal = $state(0);
  let scheduledTotal = $state(0);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);

  const TASK_STATUSES = ["todo", "in_progress", "in_review", "done", "blocked"];

  // Rule form.
  let name = $state("");
  let planPid = $state("");
  let triggerKind = $state<TriggerKind>("task_moved");
  let fromStatus = $state("");
  let toStatus = $state("");
  let actionKind = $state<ActionKind>("assign");
  let assigneeRef = $state("");
  let label = $state("");
  let recipientRef = $state("");
  let message = $state("");
  let scheduledKind = $state<"notify" | "expire_review">("notify");
  let inDays = $state(7);
  let taskStatus = $state("in_review");

  function actionValue(): Record<string, unknown> {
    switch (actionKind) {
      case "assign":
        return { assignee_ref: assigneeRef.trim() };
      case "add_label":
        return { label: label.trim() };
      case "notify":
        return {
          recipient_ref: recipientRef.trim(),
          message: message.trim() || undefined,
        };
      case "schedule_action":
        return {
          action_kind: scheduledKind,
          in_days: Number(inDays),
          recipient_ref: recipientRef.trim() || undefined,
          message: message.trim() || undefined,
        };
      case "set_task_status":
        return { status: taskStatus };
    }
  }

  async function refresh() {
    try {
      const [automationsPage, runsPage, scheduledPage] = await Promise.all([
        api.listAutomationsPage(),
        api.runsPage(),
        api.listScheduledPage({ status: "pending" }),
      ]);
      automations = automationsPage.items;
      automationsTotal = automationsPage.total;
      runs = runsPage.items;
      runsTotal = runsPage.total;
      scheduled = scheduledPage.items;
      scheduledTotal = scheduledPage.total;
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.automations.could_not_load_automations");
    }
  }

  async function create(event: SubmitEvent) {
    event.preventDefault();
    try {
      await api.createAutomation({
        plan_pid: planPid.trim() || undefined,
        name: name.trim(),
        trigger_kind: triggerKind,
        from_status: triggerKind === "task_moved" && fromStatus ? fromStatus : undefined,
        to_status: triggerKind === "task_moved" && toStatus ? toStatus : undefined,
        action_kind: actionKind,
        action_value: actionValue(),
      });
      notice = t("p.automations.rule_saved_it_fires_from");
      error = null;
      name = "";
      await refresh();
    } catch (err) {
      notice = null;
      error = err instanceof Error ? err.message : t("p.automations.could_not_save_the_rule");
    }
  }

  async function act(run: () => Promise<unknown>) {
    try {
      await run();
      error = null;
      await refresh();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.automations.that_action_was_refused");
    }
  }

  async function sweep() {
    try {
      const result = await api.sweep();
      notice = t("p.automations.swept_fired_already_claimed", { fired: result.fired, skipped_already_claimed: result.skipped_already_claimed, v: result.capped ? t("p.automations.capped_at_run_again_for", { cap: result.cap }) : "" });
      error = null;
      await refresh();
    } catch (err) {
      notice = null;
      error = err instanceof Error ? err.message : t("p.automations.the_sweep_failed");
    }
  }

  onMount(refresh);
</script>

<svelte:head><title>{t("p.automations.automations_ppm")}</title></svelte:head>

<h1>{t("p.automations.workflow_automation")}</h1>
<p class="small muted">
  {t("p.automations.configure_a_rule_once_as")}
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if notice}<p class="small">{notice}</p>{/if}

<form class="grid" onsubmit={create}>
  <label class="small">{t("p.automations.name")} <input bind:value={name} required /></label>
  <label class="small">
    {t("p.automations.plan_pid")}
    <input bind:value={planPid} placeholder={t("p.automations.blank_every_plan")} />
  </label>
  <label class="small">
    {t("p.automations.trigger")}
    <select bind:value={triggerKind}>
      <option value="task_moved">{t("p.automations.task_moved")}</option>
      <option value="review_submitted">{t("p.automations.review_submitted")}</option>
      <option value="plan_stage_changed">{t("p.automations.plan_stage_changed")}</option>
    </select>
  </label>
  {#if triggerKind === "task_moved"}
    <label class="small">
      {t("p.automations.from")}
      <select bind:value={fromStatus}>
        <option value="">{t("p.automations.any")}</option>
        {#each TASK_STATUSES as status (status)}<option value={status}>{enumLabel("task_status", status)}</option>{/each}
      </select>
    </label>
    <label class="small">
      {t("p.automations.to")}
      <select bind:value={toStatus}>
        <option value="">{t("p.automations.any")}</option>
        {#each TASK_STATUSES as status (status)}<option value={status}>{enumLabel("task_status", status)}</option>{/each}
      </select>
    </label>
  {/if}
  <label class="small">
    {t("p.automations.action")}
    <select bind:value={actionKind}>
      <option value="assign">{t("p.automations.assign")}</option>
      <option value="add_label">{t("p.automations.add_label")}</option>
      <option value="notify">{t("p.automations.notify")}</option>
      <option value="schedule_action">{t("p.automations.schedule_action")}</option>
      <option value="set_task_status">{t("p.automations.set_task_status")}</option>
    </select>
  </label>
  {#if actionKind === "assign"}
    <label class="small">
      {t("p.automations.assignee")}
      <input bind:value={assigneeRef} placeholder={t("p.automations.person_uuid")} required />
    </label>
  {:else if actionKind === "add_label"}
    <label class="small">{t("p.automations.label")} <input bind:value={label} required /></label>
  {:else if actionKind === "notify"}
    <label class="small">
      {t("p.automations.recipient")}
      <input bind:value={recipientRef} placeholder={t("p.automations.person_uuid")} required />
    </label>
    <label class="small">{t("p.automations.message")} <input bind:value={message} /></label>
  {:else if actionKind === "schedule_action"}
    <label class="small">
      {t("p.automations.then")}
      <select bind:value={scheduledKind}>
        <option value="notify">{t("p.automations.notify")}</option>
        <option value="expire_review">{t("p.automations.expire_review")}</option>
      </select>
    </label>
    <label class="small">
      {t("p.automations.in_calendar_days")} <input type="number" min="1" max="365" bind:value={inDays} />
    </label>
    <label class="small">
      {t("p.automations.recipient")}
      <input bind:value={recipientRef} placeholder={t("p.automations.person_uuid")} />
    </label>
  {:else if actionKind === "set_task_status"}
    <label class="small">
      {t("p.automations.status")}
      <select bind:value={taskStatus}>
        {#each TASK_STATUSES as status (status)}<option value={status}>{enumLabel("task_status", status)}</option>{/each}
      </select>
    </label>
  {/if}
  <button class="button small primary" type="submit">{t("p.automations.save_rule")}</button>
</form>

<h2>{t("p.automations.rules")}</h2>
{#if automations.length > 0}
  <p class="count">
    {automations.length === automationsTotal
      ? `${automationsTotal}`
      : `${automations.length} / ${automationsTotal}`}
  </p>
{/if}
<table>
  <thead>
    <tr><th>{t("p.automations.name_2")}</th><th>{t("p.automations.scope")}</th><th>{t("p.automations.trigger_2")}</th><th>{t("p.automations.action_2")}</th><th>{t("p.automations.state")}</th><th></th></tr>
  </thead>
  <tbody>
    {#each automations as rule (rule.pid)}
      <tr>
        <td>{rule.name}</td>
        <td class="small muted">{rule.plan_pid ? `${rule.plan_pid.slice(0, 8)}…` : t("p.automations.every_plan")}</td>
        <td class="small">
          {rule.trigger_kind}
          {#if rule.from_status || rule.to_status}
            ({rule.from_status ?? t("p.automations.any")} → {rule.to_status ?? t("p.automations.any")})
          {/if}
        </td>
        <td class="small">{rule.action_kind}</td>
        <td>{rule.enabled ? t("p.automations.on") : t("p.automations.off")}</td>
        <td class="actions">
          <button
            class="button small"
            type="button"
            onclick={() => act(() => api.setAutomationEnabled(rule.pid, !rule.enabled))}
          >
            {rule.enabled ? t("p.automations.disable") : t("p.automations.enable")}
          </button>
          <button
            class="button small"
            type="button"
            onclick={() => act(() => api.deleteAutomation(rule.pid))}
          >
            {t("p.automations.delete")}
          </button>
        </td>
      </tr>
    {/each}
  </tbody>
</table>
{#if automations.length === 0}<p class="small muted">{t("p.automations.no_rules_configured")}</p>{/if}

<h2>{t("p.automations.set_and_forget")}</h2>
<div class="row">
  <button class="button small" type="button" onclick={sweep}>{t("p.automations.sweep_due_actions")}</button>
  <span class="small muted">
    {t("p.automations.claim_based_so_a_deadline")}
  </span>
</div>
{#if scheduled.length > 0}
  <p class="count">
    {scheduled.length === scheduledTotal
      ? `${scheduledTotal}`
      : `${scheduled.length} / ${scheduledTotal}`}
  </p>
{/if}
<table>
  <thead><tr><th>{t("p.automations.subject")}</th><th>{t("p.automations.action_2")}</th><th>{t("p.automations.due")}</th><th>{t("p.automations.source")}</th><th></th></tr></thead>
  <tbody>
    {#each scheduled as action (action.pid)}
      <tr>
        <td class="small">{action.subject_kind} {action.subject_pid.slice(0, 8)}…</td>
        <td>{action.action_kind}</td>
        <td class="small">{action.due_at}</td>
        <td class="small muted">{action.source_automation_pid ? t("p.automations.automation") : t("p.automations.operator")}</td>
        <td>
          <button
            class="button small"
            type="button"
            onclick={() => act(() => api.cancelScheduled(action.pid))}
          >
            {t("p.automations.cancel")}
          </button>
        </td>
      </tr>
    {/each}
  </tbody>
</table>
{#if scheduled.length === 0}<p class="small muted">{t("p.automations.nothing_pending")}</p>{/if}

<h2>{t("p.automations.runs")}</h2>
{#if runs.length > 0}
  <p class="count">
    {runs.length === runsTotal ? `${runsTotal}` : `${runs.length} / ${runsTotal}`}
  </p>
{/if}
<table>
  <thead><tr><th>{t("p.automations.when")}</th><th>{t("p.automations.subject")}</th><th>{t("p.automations.outcome")}</th><th>{t("p.automations.detail")}</th></tr></thead>
  <tbody>
    {#each runs as run (run.pid)}
      <tr>
        <td class="small">{run.created_at}</td>
        <td class="small">{run.subject_kind} {run.subject_pid.slice(0, 8)}…</td>
        <td class:failed={run.outcome === "failed"}>{run.outcome}</td>
        <td class="small muted">{JSON.stringify(run.detail)}</td>
      </tr>
    {/each}
  </tbody>
</table>
{#if runs.length === 0}<p class="small muted">{t("p.automations.nothing_has_fired_yet")}</p>{/if}

<style>
  .grid {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    align-items: flex-end;
    margin: 0.8rem 0;
  }
  .row { display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center; margin: 0.8rem 0; }
  .actions { display: flex; gap: 0.25rem; flex-wrap: wrap; }
  .failed { color: #a4262c; font-weight: 700; }

  /* The row count: just the total when the page holds everything, and
     "shown / total" when it does not, so a first page of a long list
     cannot be mistaken for a short list (same convention as the
     organization service's front-end /organizations list route). */
  .count {
    margin: 0 0 0.3rem;
    opacity: 0.75;
    font-variant-numeric: tabular-nums;
    font-size: 0.85rem;
  }
</style>
