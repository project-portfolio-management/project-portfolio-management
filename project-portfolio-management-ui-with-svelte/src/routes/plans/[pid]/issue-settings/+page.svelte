<!--
  A plan's issue settings (`/plans/[pid]/issue-settings`, OC-3, OC-5, OC-1): who holds which role here, which issue types this
  plan offers, and its own workflows. Anyone who can see the plan can read this; only an administrator can change it, and the
  service enforces that. Failures say the catalogue's sentence, never the service's own text.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import IssueWorkflows from "#lib/components/IssueWorkflows.svelte";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t } from "#lib/i18n.svelte.js";
  import {
    PpmClient,
    type IssueType,
    type OrganizationRole,
    type PlanRoleMembers,
    type WorkflowSummary,
  } from "#lib/api/ppm.js";
  import { typeLabel } from "#lib/issue-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let roster = $state<PlanRoleMembers | null>(null);
  let roles = $state<OrganizationRole[]>([]);
  let types = $state<IssueType[]>([]);
  let offered = $state<string[]>([]);
  let workflows = $state<WorkflowSummary[]>([]);
  let error = $state<string | null>(null);
  let notice = $state("");
  let saving = $state(false);
  let person = $state("");
  let roleKey = $state("");

  const usableRoles = $derived(roles.filter((r) => !r.retired));
  const usableTypes = $derived(types.filter((x) => !x.retired));
  const planWorkflows = $derived(
    workflows.filter((w) => w.applies_to === "issue" && w.plan_pid === pid),
  );

  async function load() {
    try {
      const [members, allRoles, allTypes, chosen, allWorkflows] = await Promise.all([
        ppm.planRoleMembers(pid),
        ppm.organizationRoles(true),
        ppm.issueTypes(true),
        ppm.planIssueTypes(pid),
        ppm.workflows(),
      ]);
      roster = members;
      roles = allRoles;
      types = allTypes;
      offered = chosen.types;
      workflows = allWorkflows;
    } catch (err) {
      error = errorText(err, "p.issue_settings.could_not_load");
    }
  }
  onMount(load);

  /** Run a change, then show the plan as the service now has it. */
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

  function addMember(e: SubmitEvent) {
    e.preventDefault();
    void act(async () => {
      await ppm.addPlanRoleMember(pid, { person_ref: person.trim(), role_key: roleKey });
      person = "";
    });
  }

  function saveTypes(e: SubmitEvent) {
    e.preventDefault();
    void act(() => ppm.setPlanIssueTypes(pid, offered));
  }

  function toggle(key: string, on: boolean) {
    offered = on ? [...offered, key] : offered.filter((k) => k !== key);
  }

  function roleName(key: string): string {
    return roles.find((r) => r.key === key)?.label ?? key;
  }
</script>

<svelte:head><title>{t("p.plan_settings.title_ppm")}</title></svelte:head>

<p class="small">
  <a href={localeHref(`/plans/${pid}/issues`)}>{t("p.plan_settings.back")}</a> ·
  <a href={localeHref("/issue-settings")}>{t("p.plan_settings.all_settings")}</a>
</p>
<h1>{t("p.plan_settings.title")}</h1>
<p class="small muted">{t("p.plan_settings.intro")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>

<section aria-labelledby="members-h">
  <h2 id="members-h">{t("p.plan_settings.members_heading")}</h2>
  <p class="small muted">{t("p.plan_settings.members_intro")}</p>
  {#if roster}
    <p><strong>{roster.enforced ? t("p.plan_settings.enforced") : t("p.plan_settings.not_enforced")}</strong></p>
    {#if roster.members.length > 0}
      <table>
        <caption class="sr-only">{t("p.plan_settings.members_heading")}</caption>
        <thead>
          <tr>
            <th scope="col">{t("p.plan_settings.person")}</th>
            <th scope="col">{t("p.plan_settings.role")}</th>
            <th scope="col">{t("p.issue_settings.actions")}</th>
          </tr>
        </thead>
        <tbody>
          {#each roster.members as m (m.pid)}
            <tr>
              <th scope="row"><code>{m.person_ref}</code></th>
              <td>{roleName(m.role_key)}</td>
              <td>
                <button
                  class="button small"
                  type="button"
                  disabled={saving}
                  aria-label={`${t("p.issue_settings.remove")}: ${m.person_ref}, ${roleName(m.role_key)}`}
                  onclick={() => act(() => ppm.removePlanRoleMember(pid, m.pid))}
                >
                  {t("p.issue_settings.remove")}
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    {/if}
  {/if}
  <form class="settings-form" onsubmit={addMember} aria-labelledby="members-h">
    <label>{t("p.plan_settings.person")}
      <input bind:value={person} required maxlength="36" autocomplete="off" spellcheck="false" />
    </label>
    <label>{t("p.plan_settings.role")}
      <select bind:value={roleKey} required>
        <option value="">{t("p.plan_settings.choose_role")}</option>
        {#each usableRoles as r (r.key)}<option value={r.key}>{r.label}</option>{/each}
      </select>
    </label>
    <button class="button" type="submit" disabled={saving}>{t("p.plan_settings.add_member")}</button>
  </form>
</section>

<section aria-labelledby="plan-types-h">
  <h2 id="plan-types-h">{t("p.plan_settings.types_heading")}</h2>
  <p class="small muted">{t("p.plan_settings.types_intro")}</p>
  <form class="settings-form" onsubmit={saveTypes} aria-labelledby="plan-types-h">
    <fieldset>
      <legend>{t("p.plan_settings.types_heading")}</legend>
      {#each usableTypes as x (x.key)}
        <label class="check">
          <input
            type="checkbox"
            checked={offered.includes(x.key)}
            onchange={(e) => toggle(x.key, e.currentTarget.checked)}
          />
          {typeLabel(types, x.key)}
        </label>
      {/each}
    </fieldset>
    <button class="button" type="submit" disabled={saving}>{t("p.plan_settings.save_types")}</button>
  </form>
</section>

<IssueWorkflows
  plan={pid}
  {types}
  workflows={planWorkflows}
  {saving}
  {act}
  refuse={(text) => {
    notice = "";
    error = text;
  }}
/>

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
