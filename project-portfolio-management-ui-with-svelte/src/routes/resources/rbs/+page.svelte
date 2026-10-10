<!--
  Resource breakdown (`/resources/rbs`): resources grouped by role (with the
  people in each, if you may see them) and by skill (demand only: the service does
  not know supply or people for skills, and says so). Each grouping is a table.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient } from "#lib/api/ppm.js";
  import { buildRbs, roleText, type Rbs } from "#lib/rbs.js";

  const ppm = PpmClient.withFetch();
  let rbs = $state<Rbs | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      const [byRole, bySkill] = await Promise.all([
        ppm.resourcePlan({ group_by: "role" }),
        ppm.resourcePlan({ group_by: "skill" }),
      ]);
      rbs = buildRbs(byRole, bySkill);
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.resources.rbs.could_not_load_resources");
    }
  });
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
          </li>
        {/each}
      </ul>
      <table>
        <caption class="sr-only">{t("p.resources.rbs.roles_with_people_demand_and")}</caption>
        <thead><tr><th scope="col">{t("p.resources.rbs.role")}</th><th scope="col">{t("p.resources.rbs.people")}</th><th scope="col">{t("p.resources.rbs.demand_fte")}</th><th scope="col">{t("p.resources.rbs.supply_fte")}</th></tr></thead>
        <tbody>
          {#each rbs.roles as r (r.role)}
            <tr><th scope="row">{r.role}</th><td>{r.headcount}</td><td>{r.demand_fte}</td><td>{r.supply_fte ?? t("p.resources.rbs.not_known")}</td></tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </section>
  <section aria-labelledby="skills-h">
    <h2 id="skills-h">{t("p.resources.rbs.by_skill")}</h2>
    <p class="small muted">{rbs.skillNote}</p>
    {#if rbs.skills.length === 0}
      <p>{t("p.resources.rbs.no_skill_demand_in_this")}</p>
    {:else}
      <table>
        <caption class="sr-only">{t("p.resources.rbs.demand_by_skill")}</caption>
        <thead><tr><th scope="col">{t("p.resources.rbs.skill")}</th><th scope="col">{t("p.resources.rbs.demand_fte")}</th></tr></thead>
        <tbody>{#each rbs.skills as s (s.skill)}<tr><th scope="row">{s.skill}</th><td>{s.demand_fte}</td></tr>{/each}</tbody>
      </table>
    {/if}
  </section>
{/if}
