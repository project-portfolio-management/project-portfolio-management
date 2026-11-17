<!--
  Director packs (`/director-packs`, DP-11): the packs you may read, newest first. Each opens as an
  accessible page; the figures are read from the issued pack, never recalculated. Below, an editor drafts the first
  version of a period's pack for an audience (DP-1, NC-1 slice 11); the service decides who may.
-->
<script lang="ts">
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { onMount } from "svelte";
  import { resource } from "#lib/resource.svelte.js";
  import { goto } from "$app/navigation";
  import { errorText } from "#lib/error-text.js";
  import { PpmClient, type DirectorPackMeta, type Organization } from "#lib/api/ppm.js";
  import { PACK_AUDIENCES, draftProblems, refusalKey } from "#lib/pack-actions.js";

  const ppm = PpmClient.withFetch();
  const loaded = resource(async () => (await ppm.directorPacks()).packs, "p.director_pack.could_not_load");
  const packs = $derived(loaded.data);
  const error = $derived(loaded.error);

  // Organizations are audiences too (`org:<pid>`); without them the form offers the built-in audiences only.
  let orgs = $state<Organization[]>([]);
  async function loadOrgs() {
    try {
      orgs = (await ppm.organizations()).organizations.filter((o) => o.active);
    } catch {
      orgs = [];
    }
  }
  onMount(() => {
    void loaded.reload();
    void loadOrgs();
  });

  let period = $state("");
  let audience = $state("");
  let cutOff = $state("");
  let problems = $state<string[]>([]);
  let draftError = $state<string | null>(null);
  let drafting = $state(false);
  async function draft(e: SubmitEvent) {
    e.preventDefault();
    draftError = null;
    problems = draftProblems({ period, audience, cutOff }, new Date()).map((k) => t(k));
    if (problems.length > 0) return;
    drafting = true;
    try {
      const made = await ppm.draftDirectorPack({
        period,
        audience,
        ...(cutOff ? { cut_off: new Date(cutOff).toISOString() } : {}),
      });
      await goto(localeHref(`/director-packs/${made.pid}`));
    } catch (err) {
      const k = refusalKey("draft", err);
      draftError = k ? t(k) : errorText(err, "p.director_pack.could_not_draft");
    } finally {
      drafting = false;
    }
  }

  const columns: DataGridColumn[] = $derived([
    { id: "period", header: t("p.director_pack.col_period"), rowHeader: true, cell: periodCell },
    { id: "audience", header: t("p.director_pack.col_audience") },
    { id: "version", header: t("p.director_pack.col_version"), align: "end" },
    { id: "status", header: t("p.director_pack.col_status"), format: (v) => (v === "issued" ? t("p.director_pack.status_issued") : t("p.director_pack.status_draft")) },
    { id: "issued_at", header: t("p.director_pack.col_issued"), format: (v) => (v ? formatDate(String(v)) : "–") },
  ]);
</script>

{#snippet periodCell({ row, formatted }: DataGridCellContext)}
  {@const p = rowOf<DirectorPackMeta>(row)}
  <a href={localeHref(`/director-packs/${p.pid}`)} aria-label={t("p.director_pack.open_pack", { period: p.period, audience: p.audience, version: p.version })}>{formatted}</a>
{/snippet}


<svelte:head><title>{t("p.director_pack.director_packs_ppm")}</title></svelte:head>

<h1>{t("p.director_pack.director_packs")}</h1>
<p class="small muted">{t("p.director_pack.intro")}</p>

{#if error}<p class="banner error" role="alert">{error}</p>{/if}

{#if packs}
  {#if packs.length === 0}
    <p>{t("p.director_pack.none")}</p>
  {:else}
    <DataTable label={t("p.director_pack.col_period")} columns={columns} rows={packs} rowId={(r) => String(r.pid)} />
  {/if}
{:else if !error}
  <p class="muted">{t("p.director_pack.loading")}</p>
{/if}

<section class="draft" aria-labelledby="draft-h">
  <h2 id="draft-h">{t("p.director_pack.draft_heading")}</h2>
  <p class="small muted">{t("p.director_pack.draft_says")}</p>
  {#if draftError}<p class="banner error" role="alert">{draftError}</p>{/if}
  {#if problems.length > 0}
    <ul class="banner error" role="alert">{#each problems as p (p)}<li>{p}</li>{/each}</ul>
  {/if}
  <form class="stack" onsubmit={draft}>
    <label>
      {t("p.director_pack.f_period")}
      <input type="month" bind:value={period} />
    </label>
    <label>
      {t("p.director_pack.f_audience")}
      <select bind:value={audience}>
        <option value="">{t("p.director_pack.choose_audience")}</option>
        {#each PACK_AUDIENCES as a (a.key)}<option value={a.key}>{t(a.label)}</option>{/each}
        {#each orgs as o (o.pid)}<option value={o.audience}>{t("p.director_pack.aud_org", { name: o.name })}</option>{/each}
      </select>
    </label>
    <label>
      {t("p.director_pack.f_cut_off")}
      <input type="datetime-local" bind:value={cutOff} />
    </label>
    <p class="small muted">{t("p.director_pack.cut_off_says")}</p>
    <div class="row">
      <button class="button primary" type="submit" disabled={drafting}>{t("p.director_pack.draft")}</button>
    </div>
  </form>
</section>

<style>
  .draft {
    margin-block-start: 2rem;
  }
</style>
