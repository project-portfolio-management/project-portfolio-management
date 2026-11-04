<!--
  Organization-defined issue types, statuses and roles (`/issue-settings`, OC-1). Anyone signed in can read them; only an
  administrator can change them, and the service enforces that. A role here grants nothing: what a person may do comes from
  their sign-in. Statuses are the states of an issue workflow, registered whole and withdrawn to change. Failures say the
  catalogue's sentence, never the service's own text.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import {
    PpmClient,
    type IssueStatusCategory,
    type IssueType,
    type OrganizationRole,
    type WorkflowSummary,
  } from "#lib/api/ppm.js";
  import { typeLabel } from "#lib/issue-view.js";
  import {
    emptyWorkflowDraft,
    workflowBody,
    workflowProblem,
    type WorkflowDraft,
  } from "#lib/issue-settings-view.js";

  const ppm = PpmClient.withFetch();
  const CATEGORIES: IssueStatusCategory[] = ["todo", "active", "waiting", "done"];

  let types = $state<IssueType[]>([]);
  let roles = $state<OrganizationRole[]>([]);
  let workflows = $state<WorkflowSummary[]>([]);
  let error = $state<string | null>(null);
  let notice = $state("");
  let saving = $state(false);

  let typeKey = $state("");
  let typeName = $state("");
  let roleKey = $state("");
  let roleName = $state("");
  let roleDescription = $state("");
  let draft = $state<WorkflowDraft>(emptyWorkflowDraft());

  const issueWorkflows = $derived(workflows.filter((w) => w.applies_to === "issue"));
  const usableTypes = $derived(types.filter((x) => !x.retired));
  const stateKeys = $derived(draft.states.map((s) => s.key.trim()).filter((k) => k !== ""));

  async function load() {
    try {
      [types, roles, workflows] = await Promise.all([
        ppm.issueTypes(true),
        ppm.organizationRoles(true),
        ppm.workflows(),
      ]);
    } catch (err) {
      error = errorText(err, "p.issue_settings.could_not_load");
    }
  }
  onMount(load);

  /** Run a change, then show the catalogue as the service now has it. */
  async function act(change: () => Promise<unknown>) {
    error = null;
    notice = "";
    saving = true;
    try {
      await change();
      notice = t("p.issue_settings.saved");
      await load();
    } catch (err) {
      error = errorText(err, "p.issue_settings.could_not_save");
    } finally {
      saving = false;
    }
  }

  function addType(e: SubmitEvent) {
    e.preventDefault();
    void act(async () => {
      await ppm.addIssueType({ key: typeKey.trim(), label: typeName.trim(), position: types.length });
      typeKey = "";
      typeName = "";
    });
  }
  function addRole(e: SubmitEvent) {
    e.preventDefault();
    void act(async () => {
      await ppm.addOrganizationRole({
        key: roleKey.trim(),
        label: roleName.trim(),
        description: roleDescription.trim() || null,
        position: roles.length,
      });
      roleKey = "";
      roleName = "";
      roleDescription = "";
    });
  }
  function register(e: SubmitEvent) {
    e.preventDefault();
    if (workflowProblem(draft)) {
      error = t("p.issue_settings.could_not_save");
      return;
    }
    void act(async () => {
      await ppm.addIssueWorkflow(workflowBody(draft));
      draft = emptyWorkflowDraft();
    });
  }
</script>

<svelte:head><title>{t("p.issue_settings.title_ppm")}</title></svelte:head>

<h1>{t("p.issue_settings.title")}</h1>
<p class="small muted">{t("p.issue_settings.intro")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>

<section aria-labelledby="types-h">
  <h2 id="types-h">{t("p.issue_settings.types_heading")}</h2>
  <p class="small muted">{t("p.issue_settings.types_intro")}</p>
  <table>
    <caption class="sr-only">{t("p.issue_settings.types_heading")}</caption>
    <thead>
      <tr>
        <th scope="col">{t("p.issue_settings.label")}</th>
        <th scope="col">{t("p.issue_settings.key")}</th>
        <th scope="col">{t("p.issue_settings.state")}</th>
        <th scope="col">{t("p.issue_settings.actions")}</th>
      </tr>
    </thead>
    <tbody>
      {#each types as x (x.key)}
        <tr>
          <th scope="row">{typeLabel(types, x.key)}</th>
          <td><code>{x.key}</code></td>
          <td>{x.retired ? t("p.issue_settings.retired") : t("p.issue_settings.in_use")}</td>
          <td>
            <button
              class="button small"
              type="button"
              disabled={saving}
              aria-label={`${x.retired ? t("p.issue_settings.restore") : t("p.issue_settings.retire")}: ${x.label}`}
              onclick={() =>
                act(() =>
                  ppm.changeIssueType(x.key, { label: x.label, position: x.position, retired: !x.retired }),
                )}
            >
              {x.retired ? t("p.issue_settings.restore") : t("p.issue_settings.retire")}
            </button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
  <form class="settings-form" onsubmit={addType} aria-labelledby="types-h">
    <label>{t("p.issue_settings.key")}
      <input bind:value={typeKey} required maxlength="40" pattern="[a-z][a-z0-9_]*" autocomplete="off" />
    </label>
    <label>{t("p.issue_settings.label")}
      <input bind:value={typeName} required maxlength="100" autocomplete="off" />
    </label>
    <button class="button" type="submit" disabled={saving}>{t("p.issue_settings.add_type")}</button>
  </form>
</section>

<section aria-labelledby="roles-h">
  <h2 id="roles-h">{t("p.issue_settings.roles_heading")}</h2>
  <p class="small muted">{t("p.issue_settings.roles_intro")}</p>
  {#if roles.length > 0}
    <table>
      <caption class="sr-only">{t("p.issue_settings.roles_heading")}</caption>
      <thead>
        <tr>
          <th scope="col">{t("p.issue_settings.label")}</th>
          <th scope="col">{t("p.issue_settings.key")}</th>
          <th scope="col">{t("p.issue_settings.description")}</th>
          <th scope="col">{t("p.issue_settings.state")}</th>
          <th scope="col">{t("p.issue_settings.actions")}</th>
        </tr>
      </thead>
      <tbody>
        {#each roles as r (r.key)}
          <tr>
            <th scope="row">{r.label}</th>
            <td><code>{r.key}</code></td>
            <td>{r.description ?? ""}</td>
            <td>{r.retired ? t("p.issue_settings.retired") : t("p.issue_settings.in_use")}</td>
            <td>
              <button
                class="button small"
                type="button"
                disabled={saving}
                aria-label={`${r.retired ? t("p.issue_settings.restore") : t("p.issue_settings.retire")}: ${r.label}`}
                onclick={() =>
                  act(() =>
                    ppm.changeOrganizationRole(r.key, {
                      label: r.label,
                      description: r.description,
                      position: r.position,
                      retired: !r.retired,
                    }),
                  )}
              >
                {r.retired ? t("p.issue_settings.restore") : t("p.issue_settings.retire")}
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
  <form class="settings-form" onsubmit={addRole} aria-labelledby="roles-h">
    <label>{t("p.issue_settings.key")}
      <input bind:value={roleKey} required maxlength="64" pattern="[a-z][a-z0-9_\-]*" autocomplete="off" />
    </label>
    <label>{t("p.issue_settings.label")}
      <input bind:value={roleName} required maxlength="100" autocomplete="off" />
    </label>
    <label>{t("p.issue_settings.description")}
      <input bind:value={roleDescription} maxlength="500" autocomplete="off" />
    </label>
    <button class="button" type="submit" disabled={saving}>{t("p.issue_settings.add_role")}</button>
  </form>
</section>

<section aria-labelledby="workflows-h">
  <h2 id="workflows-h">{t("p.issue_settings.workflows_heading")}</h2>
  <p class="small muted">{t("p.issue_settings.workflows_intro")}</p>
  {#if issueWorkflows.length === 0}
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
        {#each issueWorkflows as w (w.pid)}
          <tr>
            <th scope="row">{w.name}</th>
            <td>
              {w.issue_type ? typeLabel(types, w.issue_type) : t("p.issue_settings.wf_all_types")}
              {#if w.plan_pid}({t("p.issue_settings.wf_plan")}){/if}
            </td>
            <td>
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

  <form class="workflow-form" onsubmit={register} aria-labelledby="workflows-h">
    <label>{t("p.issue_settings.wf_name")}
      <input bind:value={draft.name} required maxlength="200" autocomplete="off" />
    </label>
    <label>{t("p.issue_settings.wf_type")}
      <select bind:value={draft.issueType}>
        <option value="">{t("p.issue_settings.wf_all_types")}</option>
        {#each usableTypes as x (x.key)}<option value={x.key}>{typeLabel(types, x.key)}</option>{/each}
      </select>
    </label>

    <fieldset>
      <legend>{t("p.issue_settings.statuses")}</legend>
      {#each draft.states as s, i (s.id)}
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
            <input type="radio" name="initial" checked={draft.initial === s.id} onchange={() => (draft.initial = s.id)} />
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
      {/each}
      <button
        class="button small"
        type="button"
        onclick={() => {
          draft.states.push({ id: draft.nextId, key: "", label: "", category: "active", terminal: false });
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
      <button class="button primary" type="submit" disabled={saving}>{t("p.issue_settings.register")}</button>
    </div>
  </form>
</section>

<style>
  section {
    margin-block-start: 2rem;
  }
  .settings-form {
    display: flex;
    flex-wrap: wrap;
    align-items: end;
    gap: 0.75rem;
    margin-block: 1rem;
  }
  .settings-form label,
  .draft-row label {
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
</style>
