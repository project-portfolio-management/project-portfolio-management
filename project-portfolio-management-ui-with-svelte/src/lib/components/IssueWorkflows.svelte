<!--
  The issue workflows of one scope (OC-1, OC-4, OC-6): the deployment's defaults with no `plan`, or one plan's own. Lists them,
  registers one, changes one in place and withdraws one. The parent runs each change, so it can report and reload as it
  does for the rest of the page. The service decides who may do this.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import {
    PpmClient,
    type IssueStatusCategory,
    type IssueType,
    type WorkflowSummary,
  } from "#lib/api/ppm.js";
  import { typeLabel } from "#lib/issue-view.js";
  import {
    RULE_FIELDS,
    draftFromWorkflow,
    emptyWorkflowDraft,
    workflowBody,
    workflowChange,
    workflowProblem,
    type RuleField,
    type WorkflowDraft,
  } from "#lib/issue-settings-view.js";

  interface Props {
    /** The plan these belong to; absent is deployment-wide. */
    plan?: string | null;
    types: IssueType[];
    workflows: WorkflowSummary[];
    saving: boolean;
    /** Runs a change and reloads the page's data. */
    act: (change: () => Promise<unknown>) => Promise<void>;
    /** Says a problem found before anything was sent. */
    refuse: (text: string) => void;
  }
  let { plan = null, types, workflows, saving, act, refuse }: Props = $props();

  const ppm = PpmClient.withFetch();
  const CATEGORIES: IssueStatusCategory[] = ["todo", "active", "waiting", "done"];
  const headingId = $derived(plan ? "plan-workflows-h" : "workflows-h");

  let draft = $state<WorkflowDraft>(emptyWorkflowDraft());
  /** The workflow being changed; `null` while registering a new one. */
  let editing = $state<WorkflowSummary | null>(null);

  const usableTypes = $derived(types.filter((x) => !x.retired));
  const stateKeys = $derived(draft.states.map((s) => s.key.trim()).filter((k) => k !== ""));

  function reset() {
    editing = null;
    draft = emptyWorkflowDraft();
  }

  async function startEdit(w: WorkflowSummary) {
    await act(async () => {
      draft = draftFromWorkflow(await ppm.workflowDetail(w.pid));
      editing = w;
    });
  }

  function submit(e: SubmitEvent) {
    e.preventDefault();
    const problem = workflowProblem(draft);
    if (problem) {
      refuse(
        problem === "bad_translations"
          ? t("p.issue_settings.bad_translations")
          : t("p.issue_settings.could_not_save"),
      );
      return;
    }
    void act(async () => {
      if (editing) await ppm.changeIssueWorkflow(editing.pid, workflowChange(draft));
      else await ppm.addIssueWorkflow(workflowBody(draft, plan));
      reset();
    });
  }

  function fieldName(field: RuleField): string {
    return t(`p.issue_settings.field_${field}`);
  }
</script>

<section aria-labelledby={headingId}>
  <h2 id={headingId}>{t("p.issue_settings.workflows_heading")}</h2>
  <p class="small muted">{t(plan ? "p.plan_settings.workflows_intro" : "p.issue_settings.workflows_intro")}</p>
  {#if workflows.length === 0}
    <p>{t("p.issue_settings.none_workflows")}</p>
  {:else}
    <table>
      <caption class="sr-only">{t("p.issue_settings.workflows_heading")}</caption>
      <thead>
        <tr>
          <th scope="col">{t("p.issue_settings.wf_name")}</th>
          <th scope="col">{t("p.issue_settings.wf_type")}</th>
          <th scope="col">{t("p.issue_settings.actions")}</th>
        </tr>
      </thead>
      <tbody>
        {#each workflows as w (w.pid)}
          <tr>
            <th scope="row">{w.name}</th>
            <td>
              {w.issue_type ? typeLabel(types, w.issue_type) : t("p.issue_settings.wf_all_types")}
              {#if w.plan_pid && !plan}({t("p.issue_settings.wf_plan")}){/if}
            </td>
            <td>
              <button
                class="button small"
                type="button"
                disabled={saving}
                aria-label={`${t("p.issue_settings.edit")}: ${w.name}`}
                onclick={() => startEdit(w)}
              >
                {t("p.issue_settings.edit")}
              </button>
              <button
                class="button small"
                type="button"
                disabled={saving}
                aria-label={`${t("p.issue_settings.withdraw")}: ${w.name}`}
                onclick={() => act(() => ppm.withdrawWorkflow(w.pid))}
              >
                {t("p.issue_settings.withdraw")}
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}

  <form class="workflow-form" onsubmit={submit} aria-labelledby={headingId}>
    {#if editing}<p role="status"><strong>{t("p.issue_settings.editing", { name: editing.name })}</strong></p>{/if}
    <label>{t("p.issue_settings.wf_name")}
      <input bind:value={draft.name} required maxlength="200" autocomplete="off" />
    </label>
    <label>{t("p.issue_settings.wf_type")}
      <select bind:value={draft.issueType} disabled={editing !== null}>
        <option value="">{t("p.issue_settings.wf_all_types")}</option>
        {#each usableTypes as x (x.key)}<option value={x.key}>{typeLabel(types, x.key)}</option>{/each}
      </select>
    </label>

    <fieldset>
      <legend>{t("p.issue_settings.statuses")}</legend>
      {#each draft.states as s, i (s.id)}
        <div class="draft-state">
          <div class="draft-row">
            <label>{t("p.issue_settings.key")}
              <input bind:value={s.key} required maxlength="32" pattern="[a-z0-9_]+" autocomplete="off" />
            </label>
            <label>{t("p.issue_settings.label")}
              <input bind:value={s.label} required maxlength="100" autocomplete="off" />
            </label>
            <label>{t("p.issue_settings.category")}
              <select bind:value={s.category}>
                {#each CATEGORIES as c (c)}<option value={c}>{t(`p.issue_settings.cat_${c}`)}</option>{/each}
              </select>
            </label>
            <label class="check">
              <input type="radio" name={`initial-${headingId}`} checked={draft.initial === s.id} onchange={() => (draft.initial = s.id)} />
              {t("p.issue_settings.initial")}
            </label>
            <label class="check">
              <input type="checkbox" bind:checked={s.terminal} />
              {t("p.issue_settings.terminal")}
            </label>
            {#if draft.states.length > 1}
              <button
                class="button small"
                type="button"
                aria-label={`${t("p.issue_settings.remove")}: ${s.label || i + 1}`}
                onclick={() => (draft.states = draft.states.filter((x) => x.id !== s.id))}
              >
                {t("p.issue_settings.remove")}
              </button>
            {/if}
          </div>
          <label class="wide">{t("p.issue_settings.translations")}
            <input bind:value={s.labels} maxlength="1000" autocomplete="off" />
          </label>
          {#each s.rules as r, j (r.id)}
            <div class="draft-row">
              <label>{t("p.issue_settings.rule_field")}
                <select bind:value={r.field}>
                  {#each RULE_FIELDS as f (f)}<option value={f}>{fieldName(f)}</option>{/each}
                </select>
              </label>
              <label>{t("p.issue_settings.rule_kind")}
                <select bind:value={r.rule}>
                  <option value="required">{t("p.issue_settings.rule_required")}</option>
                  <option value="read_only">{t("p.issue_settings.rule_read_only")}</option>
                </select>
              </label>
              <label>{t("p.issue_settings.rule_roles")}
                <input bind:value={r.roles} maxlength="300" autocomplete="off" />
              </label>
              <button
                class="button small"
                type="button"
                aria-label={`${t("p.issue_settings.remove")}: ${t("p.issue_settings.rule_kind")} ${j + 1}, ${s.label || i + 1}`}
                onclick={() => (s.rules = s.rules.filter((x) => x.id !== r.id))}
              >
                {t("p.issue_settings.remove")}
              </button>
            </div>
          {/each}
          <button
            class="button small"
            type="button"
            aria-label={`${t("p.issue_settings.add_rule")}: ${s.label || i + 1}`}
            onclick={() => {
              s.rules.push({ id: draft.nextId, field: "owner_ref", rule: "required", roles: "" });
              draft.nextId += 1;
            }}
          >
            {t("p.issue_settings.add_rule")}
          </button>
        </div>
      {/each}
      <button
        class="button small"
        type="button"
        onclick={() => {
          draft.states.push({
            id: draft.nextId,
            key: "",
            label: "",
            labels: "",
            category: "active",
            terminal: false,
            rules: [],
          });
          draft.nextId += 1;
        }}
      >
        {t("p.issue_settings.add_status")}
      </button>
    </fieldset>

    <fieldset>
      <legend>{t("p.issue_settings.moves")}</legend>
      <p class="small muted">{t("p.issue_settings.moves_intro")}</p>
      {#each draft.moves as m, i (m.id)}
        <div class="draft-row">
          <label>{t("p.issue_settings.from")}
            <select bind:value={m.from} required>
              {#each stateKeys as k (k)}<option value={k}>{k}</option>{/each}
            </select>
          </label>
          <label>{t("p.issue_settings.to")}
            <select bind:value={m.to} required>
              {#each stateKeys as k (k)}<option value={k}>{k}</option>{/each}
            </select>
          </label>
          <label>{t("p.issue_settings.move_roles")}
            <input bind:value={m.roles} maxlength="300" autocomplete="off" />
          </label>
          <button
            class="button small"
            type="button"
            aria-label={`${t("p.issue_settings.remove")}: ${i + 1}`}
            onclick={() => (draft.moves = draft.moves.filter((x) => x.id !== m.id))}
          >
            {t("p.issue_settings.remove")}
          </button>
        </div>
      {/each}
      <button
        class="button small"
        type="button"
        onclick={() => {
          draft.moves.push({ id: draft.nextId, from: stateKeys[0] ?? "", to: stateKeys[1] ?? "", roles: "" });
          draft.nextId += 1;
        }}
      >
        {t("p.issue_settings.add_move")}
      </button>
    </fieldset>

    <div>
      <button class="button primary" type="submit" disabled={saving}>
        {editing ? t("p.issue_settings.save_workflow") : t("p.issue_settings.register")}
      </button>
      {#if editing}
        <button class="button" type="button" onclick={reset}>{t("p.issue_settings.cancel")}</button>
      {/if}
    </div>
  </form>
</section>

<style>
  section {
    margin-block-start: 2rem;
  }
  .draft-row label,
  .wide {
    display: grid;
    gap: 0.2rem;
  }
  .workflow-form {
    display: grid;
    gap: 0.75rem;
    max-inline-size: 56rem;
    margin-block: 1rem;
  }
  .workflow-form > label {
    display: grid;
    gap: 0.2rem;
    max-inline-size: 28rem;
  }
  .draft-row {
    display: flex;
    flex-wrap: wrap;
    align-items: end;
    gap: 0.75rem;
    margin-block-end: 0.75rem;
  }
  .draft-row .check {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    min-block-size: 2.5rem;
  }
  .draft-state {
    display: grid;
    gap: 0.5rem;
    justify-items: start;
    padding-block-end: 0.75rem;
    margin-block-end: 0.75rem;
    border-block-end: 1px solid var(--border, currentColor);
  }
  .wide,
  .wide input {
    inline-size: 100%;
  }
</style>
