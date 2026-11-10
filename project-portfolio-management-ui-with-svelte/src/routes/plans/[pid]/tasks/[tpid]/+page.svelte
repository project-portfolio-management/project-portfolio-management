<!--
  One task (`/plans/[pid]/tasks/[tpid]`): what it is and where it stands; editing it (NC-1 slice 3: the title,
  description, assignee, points and due date; the service replaces every field, so the sprint, component and parent go
  back as they are, and a status change stays the board's move); its links to other tasks of the plan (LT-10:
  blocks, comes before, related, duplicate, copied from), added and removed here; and its links to outside systems,
  such as the Jira or Linear issue it is tracked as (the service stores a link's address and never fetches it). Each
  card on the board links here. Refusals say the catalogue's sentence, never the service's own text.
-->
<script lang="ts">
  import { resource } from "#lib/resource.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { t, tn, localeHref, type StringKey } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type Task, type TaskRelationKind, type TaskRelationList } from "#lib/api/ppm.js";
  import LinksPanel from "#lib/components/LinksPanel.svelte";
  import {
    RELATION_KINDS,
    formFromTask,
    relationProblem,
    relationWordsKey,
    taskBody,
    taskProblem,
    taskRefusalKey,
    type RelationProblem,
    type TaskForm,
    type TaskProblem,
    type TaskStep,
  } from "#lib/task-edit-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  const tpid = $derived(page.params.tpid ?? "");
  const loaded = resource<Task>(() => ppm.getTask(pid, tpid), "p.task.could_not_load");
  const task = $derived(loaded.data);
  const loadError = $derived(loaded.error);
  const relations = resource<TaskRelationList>(() => ppm.taskRelations(pid, tpid), "p.task.could_not_load_links");
  // The plan's other tasks, to link to.
  const planTasks = resource<Task[]>(async () => (await ppm.listTasks(pid)).tasks, "p.task.could_not_load_tasks");
  const others = $derived((planTasks.data ?? []).filter((o) => o.pid !== tpid));

  let form = $state<TaskForm>({ title: "", description: "", assignee: "", points: "", dueDate: "" });
  let linkKind = $state<TaskRelationKind>("blocks");
  let linkTarget = $state("");
  let notice = $state("");
  let actionError = $state<string | null>(null);
  let saving = $state(false);
  const error = $derived(actionError ?? loadError);

  const STATUS: Record<Task["status"], string> = {
    get todo() { return t("p.task.s_todo"); },
    get in_progress() { return t("p.task.s_in_progress"); },
    get in_review() { return t("p.task.s_in_review"); },
    get done() { return t("p.task.s_done"); },
    get blocked() { return t("p.task.s_blocked"); },
  };
  const statusWords = (s: string): string => (s in STATUS ? STATUS[s as Task["status"]] : s);

  const TASK_PROBLEM: Record<NonNullable<TaskProblem>, StringKey> = {
    title_required: "p.task.problem_title_required",
    title_too_long: "p.task.problem_title_too_long",
    assignee_not_reference: "p.task.problem_assignee",
    points_not_whole: "p.task.problem_points_whole",
    points_out_of_range: "p.task.problem_points_range",
    due_not_date: "p.task.problem_due",
  };
  const RELATION_PROBLEM: Record<NonNullable<RelationProblem>, StringKey> = {
    choose_task: "p.task.problem_choose_task",
    self: "p.task.problem_self",
    exists: "p.task.problem_exists",
    cycle: "p.task.problem_cycle",
    precedes_dates: "p.task.problem_precedes_dates",
  };

  async function loadTask() {
    await loaded.reload();
    if (loaded.data) form = formFromTask(loaded.data);
  }
  onMount(() => {
    void loadTask();
    void relations.reload();
    void planTasks.reload();
  });

  /** The sentence for a failed step: a refusal in the catalogue's words; what changed is read again. */
  async function refused(err: unknown, step: TaskStep) {
    const key = err instanceof ApiError ? taskRefusalKey(step, err.status) : null;
    actionError = key ? t(key) : errorText(err, "ppm.common.actionFailed");
    if (err instanceof ApiError && err.status === 404) {
      if (step === "edit") await loadTask();
      else await relations.reload();
    }
    if (err instanceof ApiError && err.status === 409) await relations.reload();
  }

  async function save(e: SubmitEvent) {
    e.preventDefault();
    if (!task) return;
    actionError = null;
    notice = "";
    const problem = taskProblem(form);
    if (problem) {
      actionError = t(TASK_PROBLEM[problem]);
      return;
    }
    saving = true;
    try {
      await ppm.updateTask(pid, task.pid, taskBody(form, task));
      notice = t("p.task.saved");
      await loadTask();
      // A changed title or due date shows in the plan's task list and in the links.
      void planTasks.reload();
      void relations.reload();
    } catch (err) {
      await refused(err, "edit");
    } finally {
      saving = false;
    }
  }

  async function addLink(e: SubmitEvent) {
    e.preventDefault();
    if (!task) return;
    actionError = null;
    notice = "";
    const target = others.find((o) => o.pid === linkTarget) ?? null;
    const problem = relationProblem(linkKind, task, target, relations.data?.relations ?? []);
    if (problem || !target) {
      actionError = t(RELATION_PROBLEM[problem ?? "choose_task"]);
      return;
    }
    saving = true;
    try {
      await ppm.addTaskRelation(pid, task.pid, { kind: linkKind, to_task_pid: target.pid });
      notice = t("p.task.link_added");
      linkTarget = "";
      await relations.reload();
    } catch (err) {
      await refused(err, "add_link");
    } finally {
      saving = false;
    }
  }

  async function removeLink(relationPid: string) {
    if (!task) return;
    actionError = null;
    notice = "";
    saving = true;
    try {
      await ppm.removeTaskRelation(pid, task.pid, relationPid);
      notice = t("p.task.link_removed");
      await relations.reload();
    } catch (err) {
      await refused(err, "remove_link");
    } finally {
      saving = false;
    }
  }
</script>

<svelte:head><title>{t("p.task.task_ppm")}</title></svelte:head>

<p class="small">
  <a href={localeHref(`/plans/${pid}/board`)}>{t("p.task.back_board")}</a> ·
  <a href={localeHref(`/plans/${pid}/wbs`)}>{t("p.task.back")}</a>
</p>
<h1>{task ? task.title : t("p.task.task")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>
{#if task}
  <dl>
    <dt>{t("p.task.status")}</dt><dd>{statusWords(task.status)}</dd>
    {#if task.due_date}<dt>{t("p.task.due")}</dt><dd>{formatDate(task.due_date)}</dd>{/if}
    {#if task.assignee_ref}<dt>{t("p.task.assignee")}</dt><dd>{task.assignee_ref}</dd>{/if}
    {#if task.description}<dt>{t("p.task.description")}</dt><dd>{task.description}</dd>{/if}
  </dl>

  <section aria-labelledby="task-edit-h">
    <h2 id="task-edit-h">{t("p.task.edit_heading")}</h2>
    <p class="small muted">{t("p.task.edit_note")}</p>
    <form class="task-form" onsubmit={save} aria-labelledby="task-edit-h">
      <label>{t("p.task.f_title")}
        <input bind:value={form.title} maxlength="1024" required />
      </label>
      <label>{t("p.task.description")}
        <textarea bind:value={form.description} rows="3" maxlength="10000"></textarea>
      </label>
      <label>{t("p.task.f_assignee")}
        <input bind:value={form.assignee} maxlength="120" autocomplete="off" />
      </label>
      <label>{t("p.task.f_points")}
        <input bind:value={form.points} inputmode="numeric" class="points" />
      </label>
      <label>{t("p.task.f_due")}
        <input type="date" bind:value={form.dueDate} />
      </label>
      <div class="row">
        <button class="button primary" type="submit" disabled={saving}>{t("p.task.save")}</button>
      </div>
    </form>
  </section>

  <section aria-labelledby="task-rel-h">
    <h2 id="task-rel-h">{t("p.task.relations_heading")}</h2>
    <p class="small muted">{t("p.task.relations_intro")}</p>
    {#if relations.error}<p class="banner" role="alert">{relations.error}</p>{/if}
    {#if relations.data}
      {#if relations.data.relations.length === 0}
        <p>{t("p.task.no_relations")}</p>
      {:else}
        <ul data-testid="task-relations">
          {#each relations.data.relations as r (r.pid)}
            {@const words = t(relationWordsKey(r))}
            <li>
              <strong>{words}</strong>
              <a href={localeHref(`/plans/${r.other_task.plan_pid}/tasks/${r.other_task.pid}`)}>{r.other_task.title}</a>
              <span class="small muted">{r.other_task.due_date
                  ? t("p.task.other_state_due", { status: statusWords(r.other_task.status), date: formatDate(r.other_task.due_date) })
                  : t("p.task.other_state", { status: statusWords(r.other_task.status) })}</span>
              <button
                class="button small"
                type="button"
                disabled={saving}
                aria-label={t("p.task.remove_link_named", { link: words, title: r.other_task.title })}
                onclick={() => void removeLink(r.pid)}
              >{t("p.task.remove_link")}</button>
            </li>
          {/each}
        </ul>
      {/if}
      {#if relations.data.hidden > 0}
        <p class="small muted">{tn("p.task.hidden_links", relations.data.hidden, { n: relations.data.hidden })}</p>
      {/if}
    {:else if !relations.error}
      <p class="muted">{t("p.task.loading")}</p>
    {/if}

    <form class="task-form" onsubmit={addLink} aria-labelledby="task-rel-add-h">
      <h3 id="task-rel-add-h">{t("p.task.add_link_heading")}</h3>
      {#if planTasks.error}<p class="banner" role="alert">{planTasks.error}</p>{/if}
      <label>{t("p.task.f_kind")}
        <select bind:value={linkKind}>
          {#each RELATION_KINDS as k (k)}<option value={k}>{t(relationWordsKey({ kind: k, direction: "outgoing" }))}</option>{/each}
        </select>
      </label>
      <label>{t("p.task.f_other")}
        <select bind:value={linkTarget}>
          <option value="">{t("p.task.choose_task")}</option>
          {#each others as o (o.pid)}<option value={o.pid}>{o.title}</option>{/each}
        </select>
      </label>
      <div class="row">
        <button class="button" type="submit" disabled={saving}>{t("p.task.add_link")}</button>
      </div>
    </form>
  </section>

  <p class="small muted">{t("p.task.links_intro")}</p>
  <LinksPanel {pid} taskPid={tpid} />
{:else if !loadError}
  <p class="muted">{t("p.task.loading")}</p>
{/if}

<style>
  .task-form {
    display: grid;
    gap: 0.5rem;
    max-inline-size: 32rem;
    margin-block: 1rem;
  }
  .task-form label {
    display: grid;
    gap: 0.2rem;
  }
  input.points {
    inline-size: 6rem;
  }
</style>
