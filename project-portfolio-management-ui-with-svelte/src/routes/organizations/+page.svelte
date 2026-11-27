<!--
  Partner organizations (`/organizations`, EX-3): who delivers, sponsors, supplies or partners on the
  portfolio. Add one, retire one or restore it; change its names (RE-7): its name, short name and code, whether it is
  the one national organization, and its names in other languages, only what changed sent.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { i18n, t, tDynamic } from "#lib/i18n.svelte.js";
  import { ApiError } from "#lib/api/client.js";
  import { orgChange, orgFormOf, shownOrgName, type OrgForm } from "#lib/organization-view.js";
  import { onMount } from "svelte";
  import { PpmClient, type Organization } from "#lib/api/ppm.js";
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";

  const KINDS = ["delivery", "partner", "sponsor", "supplier"] as const;
  const ppm = PpmClient.withFetch();
  let orgs = $state<Organization[] | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let name = $state("");
  let kind = $state<string>("partner");
  let editing = $state<Organization | null>(null);
  let form = $state<OrgForm | null>(null);
  let problems = $state<string[]>([]);
  let notice = $state<string | null>(null);

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
  const open = (pid: string) => {
    const o = orgs?.find((x) => x.pid === pid) ?? null;
    editing = o;
    form = o ? orgFormOf(o) : null;
    problems = [];
    notice = null;
  };
  async function saveNames(e: SubmitEvent) {
    e.preventDefault();
    if (!editing || !form || !orgs) return;
    const r = orgChange(form, editing, orgs);
    if (r === null) return void (notice = t("p.organizations.unchanged"));
    if ("problems" in r) return void (problems = r.problems);
    problems = [];
    const o = editing;
    busy = true;
    error = notice = null;
    try {
      await ppm.changeOrganization(o.pid, r.body);
      notice = t("p.organizations.saved", { name: r.body.name ?? o.name });
      editing = form = null;
      await load();
    } catch (err) {
      error =
        err instanceof ApiError && err.status === 409
          ? t("p.organizations.taken_409")
          : errorText(err, "p.organizations.could_not_save");
    } finally {
      busy = false;
    }
  }
  const kindWord = (k: string) => tDynamic(`p.organizations.kind_${k}`);
  const columns: DataGridColumn[] = $derived([
    {
      id: "name",
      header: t("p.organizations.col_name"),
      rowHeader: true,
      format: (_v, row) => {
        const o = rowOf<Organization>(row);
        const n = shownOrgName(o, i18n.locale);
        return o.national ? t("p.organizations.name_national", { name: n }) : n;
      },
    },
    { id: "short_name", header: t("p.organizations.col_short"), format: (v) => (v ? String(v) : "") },
    { id: "code", header: t("p.organizations.col_code"), format: (v) => (v ? String(v) : "") },
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
  <button class="button" type="button" disabled={busy}
    aria-label={t("p.organizations.names_for", { name: String(row.name) })}
    onclick={() => open(String(row.pid))}
  >{t("p.organizations.names_btn")}</button>
{/snippet}

<svelte:head><title>{t("p.organizations.title")}</title></svelte:head>

<h1>{t("p.organizations.h1")}</h1>
<p class="lede">{t("p.organizations.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="org-notice">{notice}</p>{/if}
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

{#if editing && form}
  <section aria-labelledby="org-names">
    <h2 id="org-names">{t("p.organizations.edit_h", { name: editing.name })}</h2>
    {#if problems.length > 0}
      <ul class="banner error" role="alert" data-testid="org-problems">
        {#each problems as p (p)}<li>{tDynamic(p)}</li>{/each}
      </ul>
    {/if}
    <form class="names" onsubmit={saveNames} novalidate aria-label={t("p.organizations.edit_h", { name: editing.name })}>
      <label>{t("p.organizations.f_name")} <input bind:value={form.name} /></label>
      <label>{t("p.organizations.f_short")} <input bind:value={form.short_name} /></label>
      <label>{t("p.organizations.f_code")} <input bind:value={form.code} autocomplete="off" /></label>
      <label class="check"><input type="checkbox" bind:checked={form.national} /> {t("p.organizations.f_national")}</label>
      <label>{t("p.organizations.f_translations")} <textarea rows="4" bind:value={form.translations}></textarea></label>
      <div class="actions">
        <button class="button primary" type="submit" disabled={busy}>{t("p.organizations.save")}</button>
        <button class="button" type="button" onclick={() => (editing = form = null)}>{t("p.organizations.cancel")}</button>
      </div>
    </form>
  </section>
{/if}

<style>
  form { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: end; }
  form.names { flex-direction: column; align-items: stretch; max-width: 40rem; }
  form.names label { display: flex; flex-direction: column; gap: 0.25rem; }
  form.names label.check { flex-direction: row; align-items: center; }
  .actions { display: flex; gap: 0.5rem; }
</style>
