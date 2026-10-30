<!--
  Skill pools (`/skill-pools`, FB-6): every pool with the workforce service's pool it is linked to, and a control
  to set or remove that link (`PUT /api/skill-pools/{pid}/peer-link`, FB-1). The id is checked here before it is
  sent (a lower-case hyphenated UUID, as the workforce service writes it); a pool already linked elsewhere (409)
  and an id the service refuses (422) are said plainly; removing a link is confirmed first. The page says that the
  link is set on both sides by a person who can see both, and is never matched by name.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type SkillPool } from "#lib/api/ppm.js";
  import { ApiError } from "#lib/api/client.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { isPeerPoolRef, linkRefusal } from "#lib/capacity-heatmap-view.js";
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";

  const ppm = PpmClient.withFetch();
  let pools = $state<SkillPool[] | null>(null);
  let loadError = $state<string | null>(null);
  let editing = $state<SkillPool | null>(null);
  let ref = $state("");
  let touched = $state(false);
  let busy = $state(false);
  let failure = $state<string | null>(null);
  let done = $state<string | null>(null);

  async function load() {
    try {
      pools = (await ppm.skillPools()).skill_pools;
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.skill_pools.could_not_load");
    }
  }
  onMount(() => void load());

  const refOk = $derived(isPeerPoolRef(ref));
  const showInvalid = $derived(touched && ref.trim() !== "" && !refOk);

  function edit(p: SkillPool) {
    editing = p;
    ref = p.peer_pool_ref ?? "";
    touched = false;
    failure = null;
    done = null;
  }

  function refusal(err: unknown): string {
    if (err instanceof ApiError) {
      const kind = linkRefusal(err.status);
      if (kind === "taken") return t("p.skill_pools.taken");
      if (kind === "invalid") return t("p.skill_pools.invalid");
      if (kind === "missing") return t("p.skill_pools.missing");
    }
    return errorText(err, "p.skill_pools.could_not_save");
  }

  async function change(p: SkillPool, next: string | null) {
    busy = true;
    failure = null;
    done = null;
    try {
      await ppm.setSkillPoolPeerLink(p.pid, next);
      done = next === null ? t("p.skill_pools.cleared", { name: p.name }) : t("p.skill_pools.saved", { name: p.name, ref: next });
      editing = null;
      await load();
    } catch (err) {
      failure = refusal(err);
      if (err instanceof ApiError && err.status === 404) {
        editing = null;
        await load();
      }
    } finally {
      busy = false;
    }
  }

  function save() {
    touched = true;
    if (!editing || !refOk) return;
    void change(editing, ref.trim());
  }

  async function clear() {
    if (!editing) return;
    const p = editing;
    if (!(await askConfirm(t("p.skill_pools.confirm_clear", { name: p.name })))) return;
    await change(p, null);
  }

  const columns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.skill_pools.col_name"), rowHeader: true },
    { id: "roles", header: t("p.skill_pools.col_roles"), accessor: (r) => rowOf<SkillPool>(r).roles.join(", ") },
    { id: "skills", header: t("p.skill_pools.col_skills"), accessor: (r) => rowOf<SkillPool>(r).skills.join(", ") },
    { id: "active", header: t("p.skill_pools.col_state"), format: (v) => (v ? t("p.skill_pools.active") : t("p.skill_pools.retired")) },
    { id: "peer_pool_ref", header: t("p.skill_pools.col_link"), accessor: (r) => rowOf<SkillPool>(r).peer_pool_ref ?? "", cell: linkCell },
    { id: "pid", header: t("p.skill_pools.col_act"), sortable: false, cell: actCell },
  ]);
</script>

{#snippet linkCell({ formatted }: DataGridCellContext)}
  {#if formatted}<code>{formatted}</code>{:else}<span class="muted">{t("p.skill_pools.not_linked")}</span>{/if}
{/snippet}

{#snippet actCell({ row }: DataGridCellContext)}
  <button class="button" type="button" disabled={busy}
    aria-label={t("p.skill_pools.change", { name: String(row.name) })}
    onclick={() => edit(rowOf<SkillPool>(row))}
  >{t("p.skill_pools.change_short")}</button>
{/snippet}

<svelte:head><title>{t("p.skill_pools.title")}</title></svelte:head>

<h1>{t("p.skill_pools.h1")}</h1>
<p class="lede">{t("p.skill_pools.lede")}</p>
<p class="banner" role="note" data-testid="link-rule">{t("p.skill_pools.rule")}</p>
<p><a href={localeHref("/capacity-heatmap")}>{t("p.skill_pools.see_heatmap")}</a></p>

{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}
{#if failure}<p class="banner error" role="alert" data-testid="link-failure">{failure}</p>{/if}
{#if done}<p class="banner success" role="status" data-testid="link-done">{done}</p>{/if}
{#if !pools && !loadError}<p class="muted">{t("p.skill_pools.loading")}</p>{/if}

{#if editing}
  <section aria-labelledby="link-edit-h" class="surface stack">
    <h2 id="link-edit-h">{t("p.skill_pools.edit_h2", { name: editing.name })}</h2>
    <form onsubmit={(e) => { e.preventDefault(); save(); }}>
      <label for="peer-ref">{t("p.skill_pools.ref_label")}</label>
      <input id="peer-ref" bind:value={ref} onblur={() => (touched = true)} autocomplete="off" spellcheck="false"
        maxlength="64" aria-describedby={showInvalid ? "peer-ref-hint peer-ref-error" : "peer-ref-hint"}
        aria-invalid={showInvalid} />
      <p id="peer-ref-hint" class="small muted">{t("p.skill_pools.ref_hint")}</p>
      {#if showInvalid}<p id="peer-ref-error" class="small error-text">{t("p.skill_pools.ref_invalid")}</p>{/if}
      <div class="row">
        <button class="button primary" type="submit" disabled={busy}>{t("p.skill_pools.save")}</button>
        {#if editing.peer_pool_ref}
          <button class="button" type="button" disabled={busy} onclick={() => void clear()}>{t("p.skill_pools.clear")}</button>
        {/if}
        <button class="button" type="button" disabled={busy} onclick={() => (editing = null)}>{t("p.skill_pools.cancel")}</button>
      </div>
    </form>
  </section>
{/if}

{#if pools}
  {#if pools.length === 0}
    <p>{t("p.skill_pools.none")}</p>
  {:else}
    <DataTable label={t("p.skill_pools.h1")} {columns} rows={pools} rowId={(r) => String(r.pid)} searchable />
  {/if}
{/if}

<style>
  form { display: grid; gap: 0.4rem; max-width: 40rem; }
  input { font-family: var(--mxi-font-mono, monospace); }
  .error-text { color: var(--mxi-color-danger); font-weight: 600; }
</style>
