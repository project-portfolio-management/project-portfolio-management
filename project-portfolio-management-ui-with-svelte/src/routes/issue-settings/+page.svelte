<!--
  Organization-defined issue types, statuses and roles (`/issue-settings`, OC-1). Anyone signed in can read them; only an
  administrator can change them, and the service enforces that. A role's permissions only narrow what someone's sign-in already
  allows, on a plan that has role members. Statuses are the states of an issue workflow. Failures say the catalogue's sentence,
  never the service's own text.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import IssueWorkflows from "#lib/components/IssueWorkflows.svelte";
  import {
    PpmClient,
    type IssuePermission,
    type IssueType,
    type OrganizationRole,
    type WorkflowSummary,
  } from "#lib/api/ppm.js";
  import { typeLabel } from "#lib/issue-view.js";
  import {
    ISSUE_PERMISSIONS,
    labelsText,
    parseLabels,
  } from "#lib/issue-settings-view.js";

  const ppm = PpmClient.withFetch();

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
  let rolePermissions = $state<IssuePermission[]>([]);
  // What is being typed in each row, by key, until it is saved.
  let typeWords = $state<Record<string, string>>({});
  let roleWords = $state<Record<string, string>>({});
  let held = $state<Record<string, IssuePermission[]>>({});

  const issueWorkflows = $derived(
    workflows.filter((w) => w.applies_to === "issue" && w.plan_pid === null),
  );

  async function load() {
    try {
      [types, roles, workflows] = await Promise.all([
        ppm.issueTypes(true),
        ppm.organizationRoles(true),
        ppm.workflows(),
      ]);
      typeWords = Object.fromEntries(types.map((x) => [x.key, labelsText(x.labels)]));
      roleWords = Object.fromEntries(roles.map((r) => [r.key, labelsText(r.labels)]));
      held = Object.fromEntries(roles.map((r) => [r.key, r.permissions ?? []]));
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

  function refuse(text: string) {
    notice = "";
    error = text;
  }

  /** What was typed as translations, or `null` after saying what is wrong. */
  function typed(text: string | undefined) {
    const labels = parseLabels(text ?? "");
    if (labels === null) refuse(t("p.issue_settings.bad_translations"));
    return labels;
  }

  function togglePermission(list: IssuePermission[], p: IssuePermission, on: boolean) {
    return on ? [...list, p] : list.filter((x) => x !== p);
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
        permissions: rolePermissions,
        position: roles.length,
      });
      rolePermissions = [];
      roleKey = "";
      roleName = "";
      roleDescription = "";
    });
  }
  function saveType(x: IssueType) {
    const labels = typed(typeWords[x.key]);
    if (labels) void act(() => ppm.changeIssueType(x.key, { label: x.label, labels }));
  }
  function saveRole(r: OrganizationRole) {
    const labels = typed(roleWords[r.key]);
    if (labels)
      void act(() =>
        ppm.changeOrganizationRole(r.key, {
          label: r.label,
          description: r.description,
          labels,
          permissions: held[r.key] ?? [],
        }),
      );
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
        <th scope="col">{t("p.issue_settings.translations")}</th>
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
            <input
              bind:value={typeWords[x.key]}
              maxlength="1000"
              autocomplete="off"
              aria-label={`${t("p.issue_settings.translations")}: ${x.label}`}
            />
          </td>
          <td>
            <button
              class="button small"
              type="button"
              disabled={saving}
              aria-label={`${t("p.issue_settings.save")}: ${x.label}`}
              onclick={() => saveType(x)}
            >
              {t("p.issue_settings.save")}
            </button>
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
          <th scope="col">{t("p.issue_settings.permissions")}</th>
          <th scope="col">{t("p.issue_settings.state")}</th>
          <th scope="col">{t("p.issue_settings.translations")}</th>
          <th scope="col">{t("p.issue_settings.actions")}</th>
        </tr>
      </thead>
      <tbody>
        {#each roles as r (r.key)}
          <tr>
            <th scope="row">{r.label}</th>
            <td><code>{r.key}</code></td>
            <td>{r.description ?? ""}</td>
            <td>
              {#each ISSUE_PERMISSIONS as p (p)}
                <label class="check">
                  <input
                    type="checkbox"
                    checked={held[r.key]?.includes(p) ?? false}
                    aria-label={`${t(`p.issue_settings.perm_${p}`)}: ${r.label}`}
                    onchange={(e) =>
                      (held[r.key] = togglePermission(held[r.key] ?? [], p, e.currentTarget.checked))}
                  />
                  {t(`p.issue_settings.perm_${p}`)}
                </label>
              {/each}
            </td>
            <td>{r.retired ? t("p.issue_settings.retired") : t("p.issue_settings.in_use")}</td>
            <td>
              <input
                bind:value={roleWords[r.key]}
                maxlength="1000"
                autocomplete="off"
                aria-label={`${t("p.issue_settings.translations")}: ${r.label}`}
              />
            </td>
            <td>
              <button
                class="button small"
                type="button"
                disabled={saving}
                aria-label={`${t("p.issue_settings.save")}: ${r.label}`}
                onclick={() => saveRole(r)}
              >
                {t("p.issue_settings.save")}
              </button>
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
    <fieldset>
      <legend>{t("p.issue_settings.permissions")}</legend>
      {#each ISSUE_PERMISSIONS as p (p)}
        <label class="check">
          <input
            type="checkbox"
            checked={rolePermissions.includes(p)}
            onchange={(e) => (rolePermissions = togglePermission(rolePermissions, p, e.currentTarget.checked))}
          />
          {t(`p.issue_settings.perm_${p}`)}
        </label>
      {/each}
    </fieldset>
    <button class="button" type="submit" disabled={saving}>{t("p.issue_settings.add_role")}</button>
  </form>
</section>

<IssueWorkflows {types} workflows={issueWorkflows} {saving} {act} {refuse} />

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
  .settings-form > label {
    display: grid;
    gap: 0.2rem;
  }
  .check {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }
  .check input {
    inline-size: 1.5rem;
    block-size: 1.5rem;
    margin: 0;
  }
</style>
