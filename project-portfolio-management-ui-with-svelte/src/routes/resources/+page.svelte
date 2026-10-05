<!--
  Resources (`/resources`): demand against supply by role (or skill) and period,
  in FTE. The heatmap is a table: each cell says "Short 0.5 FTE", "Spare 1 FTE" or
  "Balanced" in words with a symbol, and only the shading's strength follows the
  size of the gap — colour is never the only signal. Skill rows are demand only
  (supply is not known for skills) and overlap. A caller who may not manage people
  sees roles and totals only. The whole of it is also available as a long table.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { PpmClient, type ResourcePlan } from "#lib/api/ppm.js";
  import { gapCell, maxAbsGap, periodLabel } from "#lib/scheduling-view.js";

  const ppm = PpmClient.withFetch();
  let plan = $state<ResourcePlan | null>(null);
  let error = $state<string | null>(null);
  let from = $state("");
  let to = $state("");
  let period = $state<"week" | "month">("month");
  let groupBy = $state<"role" | "skill">("role");

  async function load() {
    try {
      plan = await ppm.resourcePlan({ from: from || undefined, to: to || undefined, period, group_by: groupBy });
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the resource plan";
    }
  }
  onMount(() => void load());

  const keys = $derived(
    [...new Set((plan?.periods ?? []).flatMap((p) => p.rows.map((r) => r.key)))].sort(),
  );
  const maxAbs = $derived(maxAbsGap((plan?.periods ?? []).flatMap((p) => p.rows)));
</script>

<svelte:head><title>Resources — PPM</title></svelte:head>

<h1>Resources</h1>
<p class="small muted">
  What the plans ask for against what the people in each role can give, in full-time equivalents.
  Proposed bookings are kept apart and never counted.
</p>

{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void load(); }}>
  <label>From <input type="date" bind:value={from} /></label>
  <label>To <input type="date" bind:value={to} /></label>
  <label>Per
    <select bind:value={period}>
      <option value="month">Month</option>
      <option value="week">Week</option>
    </select>
  </label>
  <label>By
    <select bind:value={groupBy}>
      <option value="role">Role</option>
      <option value="skill">Skill (demand only)</option>
    </select>
  </label>
  <button class="button small" type="submit">Show</button>
</form>

{#if plan}
  <p class="small muted">
    {periodLabel(plan.from, plan.to)} · per {plan.period} · by {plan.group_by} · scope: {plan.scope.kind}
    {#if plan.masked} · <strong>roles and totals only</strong>{/if}
  </p>
  {#if plan.overlapping}
    <p class="banner" role="note">A multi-skill booking counts fully toward each of its skills, so skill rows overlap and do not add up to the total.</p>
  {/if}

  {#if keys.length === 0}
    <p>No demand or supply in this window.</p>
  {:else}
    <table class="heat">
      <caption>Gap per {plan.period}, in FTE (supply minus demand)</caption>
      <thead>
        <tr>
          <th scope="col">{plan.group_by === "role" ? "Role" : "Skill"}</th>
          {#each plan.periods as p (p.from)}<th scope="col">{periodLabel(p.from, p.to)}</th>{/each}
        </tr>
      </thead>
      <tbody>
        {#each keys as key (key)}
          <tr>
            <th scope="row">{key}</th>
            {#each plan.periods as p (p.from)}
              {@const cell = gapCell(p.rows.find((r) => r.key === key), maxAbs)}
              <td class={`cell ${cell.kind}`} style={`--level:${cell.level}`}>
                <span aria-hidden="true">{cell.symbol}</span> {cell.text}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
    <p class="small muted">▼ short · ▲ spare · ● balanced · ? supply not known. Shading only shows how large the gap is.</p>

    <details>
      <summary>Every figure as a table</summary>
      <table>
        <caption class="sr-only">Demand, supply and gap by period and {plan.group_by}</caption>
        <thead>
          <tr>
            <th scope="col">Period</th><th scope="col">{plan.group_by === "role" ? "Role" : "Skill"}</th>
            <th scope="col">Demand FTE</th><th scope="col">Of which unassigned</th><th scope="col">Proposed FTE</th>
            <th scope="col">Supply FTE</th><th scope="col">Gap FTE</th><th scope="col">People</th>
          </tr>
        </thead>
        <tbody>
          {#each plan.periods as p (p.from)}
            {#each p.rows as r (r.key)}
              <tr>
                <th scope="row">{periodLabel(p.from, p.to)}</th>
                <td>{r.key}</td>
                <td>{r.demand_fte}</td>
                <td>{r.placeholder_fte}</td>
                <td>{r.proposed_fte}</td>
                <td>{r.supply_fte ?? "not known"}</td>
                <td>{r.gap_fte ?? "not known"}</td>
                <td>{plan.group_by === "role" ? r.headcount : "—"}</td>
              </tr>
            {/each}
          {/each}
        </tbody>
      </table>
    </details>
  {/if}

  {#if plan.placeholders.length > 0}
    <h2>Roles asked for with no person yet</h2>
    <ul>
      {#each plan.placeholders as ph (ph.allocation_pid)}
        <li>{ph.role ?? "unclassified"}: {ph.percent}% {ph.start_date ? `from ${ph.start_date}` : ""} {ph.end_date ? `to ${ph.end_date}` : ""} ({ph.booking})</li>
      {/each}
    </ul>
  {/if}
  <p class="small muted">{plan.note}</p>
{:else if !error}
  <p class="muted">Loading…</p>
{/if}

<style>
  table {
    border-collapse: collapse;
    width: 100%;
  }
  th,
  td {
    text-align: left;
    padding: 0.4rem 0.5rem;
    border-bottom: 1px solid var(--mxi-color-border, #ddd);
  }
  .cell {
    white-space: nowrap;
  }
  .cell.short {
    background: color-mix(in srgb, var(--danger, #b00020) calc(var(--level) * 35%), transparent);
    font-weight: 600;
  }
  .cell.spare {
    background: color-mix(in srgb, var(--info, #1f6feb) calc(var(--level) * 30%), transparent);
  }
  .filter {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: end;
    margin: 0.75rem 0;
  }
  .filter label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
</style>
