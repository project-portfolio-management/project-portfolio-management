<!--
  Partner organizations (`/organizations`, EX-3): who delivers, sponsors, supplies or partners on the
  portfolio. Add one, retire one or restore it.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, tDynamic } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type Organization } from "#lib/api/ppm.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";

  const KINDS = ["delivery", "partner", "sponsor", "supplier"] as const;
  const ppm = PpmClient.withFetch();
  let orgs = $state<Organization[] | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let name = $state("");
  let kind = $state<string>("partner");

  async function load() {
    try {
      orgs = (await ppm.organizations()).organizations;
      error = null;
    } catch (err) {
      error = errorText(err, "p.organizations.could_not_load");
    }
  }
  onMount(() => void load());

  async function run(job: () => Promise<unknown>) {
    busy = true;
    try {
      await job();
      await load();
    } catch (err) {
      error = errorText(err, "p.organizations.could_not_save");
    } finally {
      busy = false;
    }
  }
  const add = async () => {
    await run(() => ppm.addOrganization(name.trim(), kind));
    if (!error) name = "";
  };
  const kindWord = (k: string) => tDynamic(`p.organizations.kind_${k}`);
  const columns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.organizations.col_name"), rowHeader: true },
    { id: "kind", header: t("p.organizations.col_kind"), format: (v) => kindWord(String(v)) },
    { id: "active", header: t("p.organizations.col_state"), format: (v) => (v ? t("p.organizations.active") : t("p.organizations.retired")) },
    { id: "audience", header: t("p.organizations.col_audience"), cell: audienceCell },
    { id: "pid", header: t("p.organizations.col_act"), sortable: false, cell: actCell },
  ]);
</script>

{#snippet audienceCell({ formatted }: DataGridCellContext)}<code>{formatted}</code>{/snippet}

{#snippet actCell({ row }: DataGridCellContext)}
  <button class="button" type="button" disabled={busy}
    aria-label={t(row.active ? "p.organizations.retire" : "p.organizations.restore", { name: String(row.name) })}
    onclick={() => void run(() => ppm.changeOrganization(String(row.pid), { active: !row.active }))}
  >{row.active ? t("p.organizations.retire_short") : t("p.organizations.restore_short")}</button>
{/snippet}

<svelte:head><title>{t("p.organizations.title")}</title></svelte:head>

<h1>{t("p.organizations.h1")}</h1>
<p class="lede">{t("p.organizations.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !orgs && !error}<p class="muted">{t("p.organizations.loading")}</p>{/if}

<section aria-labelledby="org-add">
  <h2 id="org-add">{t("p.organizations.add_h2")}</h2>
  <form onsubmit={(e) => { e.preventDefault(); if (name.trim()) void add(); }}>
    <label>{t("p.organizations.name")} <input bind:value={name} required maxlength="200" /></label>
    <label>{t("p.organizations.kind")}
      <select bind:value={kind}>{#each KINDS as k (k)}<option value={k}>{kindWord(k)}</option>{/each}</select>
    </label>
    <button class="button" type="submit" disabled={busy || !name.trim()}>{t("p.organizations.add")}</button>
  </form>
</section>

{#if orgs}
  {#if orgs.length === 0}
    <p>{t("p.organizations.none")}</p>
  {:else}
    <DataTable label={t("p.organizations.h1")} {columns} rows={orgs} rowId={(r) => String(r.pid)} searchable />
  {/if}
{/if}

<style>
  form { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: end; }
</style>
