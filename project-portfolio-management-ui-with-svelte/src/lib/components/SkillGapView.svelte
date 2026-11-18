<!--
  A plan's skill gaps (`/resources`, NC-1 slice 12 part j, SC-7), read on asking: `GET /api/plans/{pid}/skill-gap`,
  each skill a booking asks for against what its person holds, read from the workforce service: held ("✓"), missing
  ("▲") or not known ("?", when the person's skills could not be read; the service's reason is English and is not
  shown). A placeholder has no person and is left out. Rules in `src/lib/resource-actions-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatInteger } from "#lib/locale-format.js";
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import type { PlanRef } from "#lib/api/types.js";
  import { PpmClient, type SkillGap, type SkillGapFinding } from "#lib/api/ppm.js";
  import { SKILL_STATUS_KEY, SKILL_STATUS_SYMBOL, skillCounts } from "#lib/resource-actions-view.js";

  let { plans }: { plans: PlanRef[] | null } = $props();

  const ppm = PpmClient.withFetch();
  let plan = $state("");
  let data = $state<SkillGap | null>(null);
  let problem = $state<string | null>(null);
  let busy = $state(false);

  async function read(e: SubmitEvent) {
    e.preventDefault();
    if (!plan) {
      problem = t("p.skillgap.problem_plan");
      return;
    }
    problem = null;
    busy = true;
    try {
      data = await ppm.skillGap(plan);
    } catch (err) {
      data = null;
      problem = errorText(err, "p.skillgap.could_not");
    } finally {
      busy = false;
    }
  }

  const counts = $derived(data ? skillCounts(data.findings) : null);
  const columns: DataGridColumn[] = $derived([
    { id: "person_ref", header: t("p.skillgap.col_person"), rowHeader: true },
    { id: "tag", header: t("p.skillgap.col_skill") },
    { id: "status", header: t("p.skillgap.col_status"), accessor: (r) => { const s = rowOf<SkillGapFinding>(r).status; return `${SKILL_STATUS_SYMBOL[s]} ${t(SKILL_STATUS_KEY[s])}`; } },
  ]);
</script>

<section aria-labelledby="skillgap-h" data-testid="skill-gap">
  <h2 id="skillgap-h">{t("p.skillgap.heading")}</h2>
  <p class="small muted">{t("p.skillgap.says")}</p>
  <form class="row" novalidate onsubmit={read} aria-label={t("p.skillgap.form")}>
    <label>{t("p.skillgap.f_plan")}
      <select bind:value={plan} disabled={plans === null}>
        <option value="">{t("p.skillgap.choose_plan")}</option>
        {#each plans ?? [] as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
      </select>
    </label>
    <button class="button" type="submit" disabled={busy || plans === null}>{t("p.skillgap.read")}</button>
  </form>
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  {#if data && counts}
    {#if data.findings.length === 0}
      <p>{t("p.skillgap.none")}</p>
    {:else}
      <p>
        {tn("p.skillgap.count_covered", counts.covered, { n: formatInteger(counts.covered) })}
        {tn("p.skillgap.count_missing", counts.missing, { n: formatInteger(counts.missing) })}
        {tn("p.skillgap.count_unknown", counts.unknown, { n: formatInteger(counts.unknown) })}
      </p>
      <DataTable label={t("p.skillgap.table")} {columns} rows={data.findings} rowId={(r, i) => `${rowOf<SkillGapFinding>(r).allocation_pid}-${i}`} />
    {/if}
  {/if}
</section>

<style>
  section { margin-block: 1.5rem; }
  .row { display: flex; gap: 0.75rem; align-items: end; flex-wrap: wrap; }
  .row label { display: grid; gap: 0.2rem; }
</style>
