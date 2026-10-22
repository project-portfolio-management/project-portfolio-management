<!--
  Milestone sign-off (`/milestone-signoffs`, EX-4): milestones that are met only when named partner
  organizations have signed them off. Delivered and met are different things, and the gap is shown.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type MilestoneSignoffs, type Organization } from "#lib/api/ppm.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";

  const ppm = PpmClient.withFetch();
  let data = $state<MilestoneSignoffs | null>(null);
  let orgs = $state<Organization[]>([]);
  let pick = $state<Record<string, string>>({});
  let reason = $state<Record<string, string>>({});
  let error = $state<string | null>(null);
  let busy = $state(false);

  async function load() {
    try {
      data = await ppm.milestoneSignoffs();
      orgs = (await ppm.organizations()).organizations.filter((o) => o.active);
      error = null;
    } catch (err) {
      error = errorText(err, "p.signoffs.could_not_load");
    }
  }
  onMount(() => void load());

  async function act(m: MilestoneSignoffs["milestones"][number], action: "sign" | "withdraw") {
    const org = pick[m.milestone_pid] ?? orgs[0]?.pid;
    if (!org) return;
    busy = true;
    try {
      await ppm.actOnSignoff(m.plan_pid, m.milestone_pid, org, action, reason[m.milestone_pid]?.trim() || undefined);
      await load();
    } catch (err) {
      error = errorText(err, "p.signoffs.could_not_save");
    } finally {
      busy = false;
    }
  }
  const SYM: Record<string, string> = { met: "●", delivered_not_met: "▲", not_delivered: "–", not_required: "–" };
  type Row = MilestoneSignoffs["milestones"][number];
  const stateWord = (s: string) => t(`p.signoffs.state_${s === "not_required" ? "met" : s}` as never);
  const columns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.signoffs.col_milestone"), rowHeader: true },
    { id: "due", header: t("p.signoffs.col_due"), format: (v) => (v ? String(v).slice(0, 10) : "") },
    { id: "state", header: t("p.signoffs.col_state"), accessor: (r) => (r as unknown as Row).signoff.state, format: (v) => stateWord(String(v)), cell: stateCell, cellClass: (c) => (c.value === "delivered_not_met" ? "cell-red" : c.value === "met" ? "cell-green" : "") },
    { id: "delivered_on", header: t("p.signoffs.col_delivered"), accessor: (r) => (r as unknown as Row).signoff.delivered_on ?? "" },
    { id: "met_on", header: t("p.signoffs.col_met"), accessor: (r) => (r as unknown as Row).signoff.met_on ?? "" },
    { id: "outstanding", header: t("p.signoffs.col_outstanding"), accessor: (r) => (r as unknown as Row).signoff.outstanding.join(", ") },
    { id: "milestone_pid", header: t("p.signoffs.col_act"), sortable: false, cell: actCell },
  ]);
  const share = $derived(
    data?.totals.met_share_of_delivered_bp == null ? t("p.signoffs.no_share") : `${Math.round(data.totals.met_share_of_delivered_bp / 100)}%`,
  );
</script>

{#snippet stateCell({ value, formatted }: DataGridCellContext)}<span aria-hidden="true">{SYM[String(value)] ?? "–"}</span> {formatted}{/snippet}

{#snippet actCell({ row }: DataGridCellContext)}
  {@const m = row as unknown as Row}
  <div class="act">
    <label>{t("p.signoffs.organization")}
      <select bind:value={pick[m.milestone_pid]}>{#each orgs as o (o.pid)}<option value={o.pid}>{o.name}</option>{/each}</select>
    </label>
    <label>{t("p.signoffs.reason")} <input bind:value={reason[m.milestone_pid]} maxlength="500" /></label>
    <button class="button" type="button" disabled={busy || orgs.length === 0} aria-label={t("p.signoffs.sign_for", { name: m.name })} onclick={() => void act(m, "sign")}>{t("p.signoffs.sign")}</button>
    <button class="button" type="button" disabled={busy || orgs.length === 0} aria-label={t("p.signoffs.withdraw_for", { name: m.name })} onclick={() => void act(m, "withdraw")}>{t("p.signoffs.withdraw")}</button>
  </div>
{/snippet}

<svelte:head><title>{t("p.signoffs.title")}</title></svelte:head>

<h1>{t("p.signoffs.h1")}</h1>
<p class="lede">{t("p.signoffs.lede")}</p>
<p class="muted">{t("p.signoffs.not_built")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !data && !error}<p class="muted">{t("p.signoffs.loading")}</p>{/if}

{#if data}
  <p>{t("p.signoffs.totals", { n: data.totals.requiring_signoff, delivered: data.totals.delivered, met: data.totals.met, gap: data.totals.delivered_not_met, share })}</p>
  {#if data.milestones.length === 0}
    <p>{t("p.signoffs.none")}</p>
  {:else}
    <DataTable label={t("p.signoffs.h1")} {columns} rows={data.milestones} rowId={(r) => String(r.milestone_pid)} />
  {/if}
{/if}

<style>
  .act { display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: end; }
</style>
