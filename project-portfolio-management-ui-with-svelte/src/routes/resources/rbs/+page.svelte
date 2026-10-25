<!--
  Resource breakdown (`/resources/rbs`): resources grouped by role (with the
  people in each, if you may see them) and by skill (demand only: the service does
  not know supply or people for skills, and says so). Each grouping is a table.
-->
<script lang="ts">
  import { errorReason, errorText } from "#lib/error-text.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type RoleTasks } from "#lib/api/ppm.js";
  import { buildRbs, canDrill, roleText, type Rbs } from "#lib/rbs.js";

  const ppm = PpmClient.withFetch();
  let rbs = $state<Rbs | null>(null);
  let error = $state<string | null>(null);
  /** The open tasks of a role's people, loaded when the reader asks. */
  let open = $state<Record<string, RoleTasks | "loading" | { error: string }>>({});

  async function toggle(role: string) {
    if (open[role]) {
      const { [role]: _gone, ...rest } = open;
      open = rest;
      return;
    }
    open = { ...open, [role]: "loading" };
    try {
      open = { ...open, [role]: await ppm.roleTasks(role) };
    } catch (err) {
      open = { ...open, [role]: { error: errorReason(err) } };
    }
  }

  onMount(async () => {
    try {
      const [byRole, bySkill] = await Promise.all([
        ppm.resourcePlan({ group_by: "role" }),
        ppm.resourcePlan({ group_by: "skill" }),
      ]);
      rbs = buildRbs(byRole, bySkill);
    } catch (err) {
      error = errorText(err, "p.resources.rbs.could_not_load_resources");
    }
  });

  const roleColumns: DataGridColumn[] = $derived([
    { id: "role", header: t("p.resources.rbs.role"), rowHeader: true },
    { id: "headcount", header: t("p.resources.rbs.people"), align: "end" },
    { id: "demand_fte", header: t("p.resources.rbs.demand_fte"), align: "end" },
    { id: "supply_fte", header: t("p.resources.rbs.supply_fte"), align: "end", format: (v) => (v ?? t("p.resources.rbs.not_known")) as string },
  ]);
  const skillColumns: DataGridColumn[] = $derived([
    { id: "skill", header: t("p.resources.rbs.skill"), rowHeader: true },
    { id: "demand_fte", header: t("p.resources.rbs.demand_fte"), align: "end" },
  ]);
</script>

<svelte:head><title>{t("p.resources.rbs.resource_breakdown_ppm")}</title></svelte:head>

<h1>{t("p.resources.rbs.resource_breakdown")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if rbs}
  <section aria-labelledby="roles-h">
    <h2 id="roles-h">{t("p.resources.rbs.by_role")}</h2>
    {#if rbs.masked}<p class="small muted">{t("p.resources.rbs.you_can_see_headcounts_only")}</p>{/if}
    {#if rbs.roles.length === 0}
      <p>{t("p.resources.rbs.no_roles_in_this_window")}</p>
    {:else}
      <ul>
        {#each rbs.roles as r (r.role)}
          <li>
            {roleText(r, rbs.masked)}
            {#if r.people.length > 0}<ul>{#each r.people as p (p.ref)}<li>{p.ref}</li>{/each}</ul>{/if}
            {#if canDrill(r, rbs.masked)}
              {@const shown = open[r.role]}
              <button type="button" aria-expanded={shown !== undefined} onclick={() => toggle(r.role)}>{shown ? t("p.resources.rbs.hide_tasks", { role: r.role }) : t("p.resources.rbs.show_tasks", { role: r.role })}</button>
              {#if shown && shown !== "loading"}
                <div role="region" aria-label={t("p.resources.rbs.show_tasks", { role: r.role })}>
                  {#if "error" in shown}
                    <p class="banner" role="alert">{t("p.resources.rbs.could_not_load_tasks", { reason: shown.error })}</p>
                  {:else}
                    <ul>
                      {#each shown.people as person (person.person_ref)}
                        <li>
                          {person.person_ref}:
                          {#if person.tasks.length === 0}<span class="muted">{t("p.resources.rbs.no_open_tasks")}</span>
                          {:else}
                            <ul>
                              {#each person.tasks as task (task.pid)}
                                <li><a href={localeHref(`/plans/${task.plan_pid}/tasks/${task.pid}`)}>{task.title}</a> <span class="small muted">({task.plan_name})</span></li>
                              {/each}
                            </ul>
                          {/if}
                        </li>
                      {/each}
                    </ul>
                    {#if shown.omitted_unreadable > 0}<p class="small muted">{t("p.resources.rbs.tasks_in_plans_you_cannot_read", { count: shown.omitted_unreadable })}</p>{/if}
                    {#if shown.truncated}<p class="small muted">{t("p.resources.rbs.only_the_first_500_tasks")}</p>{/if}
                  {/if}
                </div>
              {/if}
            {/if}
          </li>
        {/each}
      </ul>
      <DataTable label={t("p.resources.rbs.roles_with_people_demand_and")} columns={roleColumns} rows={rbs.roles} rowId={(r) => String(r.role)} />
    {/if}
  </section>
  <section aria-labelledby="skills-h">
    <h2 id="skills-h">{t("p.resources.rbs.by_skill")}</h2>
    <p class="small muted">{rbs.skillNote}</p>
    {#if rbs.skills.length === 0}
      <p>{t("p.resources.rbs.no_skill_demand_in_this")}</p>
    {:else}
      <DataTable label={t("p.resources.rbs.demand_by_skill")} columns={skillColumns} rows={rbs.skills} rowId={(r) => String(r.skill)} />
    {/if}
  </section>
{/if}
