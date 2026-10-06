<!--
  Resource breakdown (`/resources/rbs`): resources grouped by role (with the
  people in each, if you may see them) and by skill (demand only: the service does
  not know supply or people for skills, and says so). Each grouping is a table.
-->
<script lang="ts">
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
      error = err instanceof Error ? err.message : "Could not load resources";
    }
  });
</script>

<svelte:head><title>Resource breakdown — PPM</title></svelte:head>

<h1>Resource breakdown</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if rbs}
  <section aria-labelledby="roles-h">
    <h2 id="roles-h">By role</h2>
    {#if rbs.masked}<p class="small muted">You can see headcounts only, not names.</p>{/if}
    {#if rbs.roles.length === 0}
      <p>No roles in this window.</p>
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
        <caption class="sr-only">Roles with people, demand and supply</caption>
        <thead><tr><th scope="col">Role</th><th scope="col">People</th><th scope="col">Demand FTE</th><th scope="col">Supply FTE</th></tr></thead>
        <tbody>
          {#each rbs.roles as r (r.role)}
            <tr><th scope="row">{r.role}</th><td>{r.headcount}</td><td>{r.demand_fte}</td><td>{r.supply_fte ?? "not known"}</td></tr>
          {/each}
        </tbody>
      </table>
    {/if}
  </section>
  <section aria-labelledby="skills-h">
    <h2 id="skills-h">By skill</h2>
    <p class="small muted">{rbs.skillNote}</p>
    {#if rbs.skills.length === 0}
      <p>No skill demand in this window.</p>
    {:else}
      <table>
        <caption class="sr-only">Demand by skill</caption>
        <thead><tr><th scope="col">Skill</th><th scope="col">Demand FTE</th></tr></thead>
        <tbody>{#each rbs.skills as s (s.skill)}<tr><th scope="row">{s.skill}</th><td>{s.demand_fte}</td></tr>{/each}</tbody>
      </table>
    {/if}
  </section>
{/if}
