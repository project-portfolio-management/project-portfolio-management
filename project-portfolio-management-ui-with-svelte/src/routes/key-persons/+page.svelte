<!--
  Key people (`/key-persons`, EX-11): single points of failure, their successors and how ready they are, and
  the contingent workers. Resource managers only (the API answers 404 to anyone else).
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { ApiError } from "#lib/api/client.js";
  import { t, tDynamic } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type KeyPersons } from "#lib/api/ppm.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";

  const ppm = PpmClient.withFetch();
  let data = $state<KeyPersons | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let kind = $state("person");
  let subject = $state("");
  let successor = $state("");
  let readiness = $state("none");
  let note = $state("");
  let contingentRef = $state("");

  async function load() {
    try {
      data = await ppm.keyPersons();
      error = null;
    } catch (err) {
      error = err instanceof ApiError && err.status === 404
        ? t("p.key_persons.not_allowed")
        : errorText(err, "p.key_persons.could_not_load");
    }
  }
  onMount(() => void load());

  async function run(job: () => Promise<unknown>) {
    busy = true;
    try {
      await job();
      await load();
    } catch (err) {
      error = errorText(err, "p.key_persons.could_not_save");
    } finally {
      busy = false;
    }
  }
  async function record() {
    await run(() => ppm.recordKeyPerson({
      subject_kind: kind, subject: subject.trim(), readiness,
      ...(successor.trim() ? { successor_ref: successor.trim() } : {}),
      ...(note.trim() ? { note: note.trim() } : {}),
    }));
    if (!error) { subject = ""; successor = ""; note = ""; }
  }
  async function mark() {
    await run(() => ppm.setContingent(contingentRef.trim(), true));
    if (!error) contingentRef = "";
  }
  const ready = (r: string) => tDynamic(`p.key_persons.ready_${r}`);
  const kindWord = (k: string) => tDynamic(`p.key_persons.kind_${k}`);
  const columns: DataGridColumn[] = $derived([
    { id: "subject", header: t("p.key_persons.col_subject"), rowHeader: true, cell: subjectCell },
    { id: "successor_ref", header: t("p.key_persons.col_successor"), format: (v) => (v ? String(v) : t("p.key_persons.no_successor")) },
    { id: "readiness", header: t("p.key_persons.col_ready"), format: (v) => ready(String(v)), cell: readyCell, cellClass: (c) => ({ ready: "cell-green", partly: "cell-amber", none: "cell-red" })[String(c.value)] ?? "" },
    { id: "note", header: t("p.key_persons.col_note"), format: (v) => (v ? String(v) : "") },
    { id: "pid", header: t("p.key_persons.col_act"), sortable: false, cell: actCell },
  ]);
</script>

{#snippet subjectCell({ row }: DataGridCellContext)}{row.subject} <span class="muted">({kindWord(String(row.subject_kind))})</span>{/snippet}

{#snippet readyCell({ value, formatted }: DataGridCellContext)}
  <span aria-hidden="true">{value === "ready" ? "●" : value === "partly" ? "◐" : "✖"}</span> {formatted}
{/snippet}

{#snippet actCell({ row }: DataGridCellContext)}
  <button class="button" type="button" disabled={busy} aria-label={t("p.key_persons.remove", { name: String(row.subject) })} onclick={() => void run(() => ppm.removeKeyPerson(String(row.pid)))}>{t("p.key_persons.remove_short")}</button>
{/snippet}

<svelte:head><title>{t("p.key_persons.title")}</title></svelte:head>

<h1>{t("p.key_persons.h1")}</h1>
<p class="lede">{t("p.key_persons.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !data && !error}<p class="muted">{t("p.key_persons.loading")}</p>{/if}

{#if data}
  <section aria-labelledby="kp-add">
    <h2 id="kp-add">{t("p.key_persons.add_h2")}</h2>
    <form onsubmit={(e) => { e.preventDefault(); if (subject.trim()) void record(); }}>
      <label>{t("p.key_persons.subject_kind")}
        <select bind:value={kind}>{#each ["person", "role"] as k (k)}<option value={k}>{kindWord(k)}</option>{/each}</select>
      </label>
      <label>{t("p.key_persons.subject")} <input bind:value={subject} required maxlength="200" /></label>
      <label>{t("p.key_persons.successor")} <input bind:value={successor} maxlength="200" /></label>
      <label>{t("p.key_persons.readiness")}
        <select bind:value={readiness}>{#each ["ready", "partly", "none"] as r (r)}<option value={r}>{ready(r)}</option>{/each}</select>
      </label>
      <label>{t("p.key_persons.note")} <input bind:value={note} maxlength="500" /></label>
      <button class="button" type="submit" disabled={busy || !subject.trim()}>{t("p.key_persons.add")}</button>
    </form>
  </section>

  {#if data.entries.length === 0}
    <p>{t("p.key_persons.none")}</p>
  {:else}
    <DataTable label={t("p.key_persons.h1")} {columns} rows={data.entries} rowId={(r) => String(r.pid)} />
  {/if}

  <section aria-labelledby="kp-cont">
    <h2 id="kp-cont">{t("p.key_persons.contingent_h2")}</h2>
    <p class="muted">{t("p.key_persons.contingent_note")}</p>
    {#if data.contingent.length === 0}
      <p>{t("p.key_persons.contingent_none")}</p>
    {:else}
      <ul>
        {#each data.contingent as c (c)}
          <li>{c}
            <button class="button" type="button" disabled={busy} aria-label={t("p.key_persons.contingent_unmark", { name: c })} onclick={() => void run(() => ppm.setContingent(c, false))}>{t("p.key_persons.unmark")}</button>
          </li>
        {/each}
      </ul>
    {/if}
    <form onsubmit={(e) => { e.preventDefault(); if (contingentRef.trim()) void mark(); }}>
      <label>{t("p.key_persons.contingent_person")} <input bind:value={contingentRef} required maxlength="200" /></label>
      <button class="button" type="submit" disabled={busy || !contingentRef.trim()}>{t("p.key_persons.mark")}</button>
    </form>
  </section>
{/if}

<style>
  form { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: end; margin-block: 0.75rem; }
</style>
