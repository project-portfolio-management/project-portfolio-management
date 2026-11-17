<!--
  One director pack (`/director-packs/[pid]`, DP-11): every section of the issued pack as words and
  tables. A reading is a word and a symbol as well as a colour. The figures are read from the stored
  pack; fixed text comes from the catalogue, free text is shown as entered. Print it for a PDF in your own
  language, or download the service's English PDF. A draft is reviewed and issued here, and an issued pack reissued
  with a change note (DP-1, DP-12, NC-1 slice 11); which of these the reader may do is the service's to decide.
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, tDynamic, localeHref } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { i18n } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import { resource } from "#lib/resource.svelte.js";
  import { page } from "$app/state";
  import { PpmClient } from "#lib/api/ppm.js";
  import { buildPackView, type Section } from "#lib/pack-view.js";
  import { goto } from "$app/navigation";
  import { errorText } from "#lib/error-text.js";
  import { NOTE_MIN, noteProblem, packActions, refusalKey, type PackAction } from "#lib/pack-actions.js";

  const ppm = PpmClient.withFetch();
  const loaded = resource(() => ppm.directorPack(page.params.pid ?? ""), "p.director_pack.could_not_load_one");
  const pack = $derived(loaded.data);
  const error = $derived(loaded.error);

  onMount(() => void loaded.reload());

  const sections: Section[] = $derived(
    pack
      ? buildPackView(pack.body, {
          t: (k, v) => tDynamic(k, v),
          label: (g, v) => enumLabel(g as never, v),
          locale: i18n.locale,
        })
      : [],
  );
  const previous = $derived(
    pack ? ((pack.body.cover as { previous_issued_pack?: { period?: string; version?: number } } | undefined)?.previous_issued_pack ?? null) : null,
  );

  const actions = $derived(pack ? packActions(pack.pack) : null);
  let note = $state("");
  let busy = $state(false);
  let actionError = $state<string | null>(null);
  let notice = $state("");
  async function act(action: PackAction) {
    if (!pack) return;
    const pid = pack.pack.pid;
    actionError = null;
    notice = "";
    if (action === "reissue") {
      const problem = noteProblem(note);
      if (problem) {
        actionError = t(problem, { min: NOTE_MIN });
        return;
      }
    }
    busy = true;
    try {
      if (action === "review") {
        await ppm.reviewDirectorPack(pid);
        notice = t("p.director_pack.reviewed_done");
      } else if (action === "issue") {
        await ppm.issueDirectorPack(pid);
        notice = t("p.director_pack.issued_done");
      } else {
        const next = await ppm.reissueDirectorPack(pid, note.trim());
        note = "";
        await goto(localeHref(`/director-packs/${next.pid}`));
        return;
      }
      await loaded.reload();
    } catch (err) {
      const k = refusalKey(action, err);
      actionError = k ? t(k) : errorText(err, "p.director_pack.could_not_act");
    } finally {
      busy = false;
    }
  }

  type PackCell = { text: string; symbol?: string; tone?: string };
  const cellOf = (row: unknown, i: number): PackCell => ((row as { cells: PackCell[] }).cells[i] ?? { text: "" });
  /** A pack table: the first column names the row, and a cell with a tone carries its symbol and word as well. */
  const packColumns = (names: string[]): DataGridColumn[] =>
    names.map((name, i) => ({
      id: `c${i}`, header: name, rowHeader: i === 0, accessor: (row) => cellOf(row, i).text,
      ...(i === 0 ? {} : { cell: packCell, cellClass: (ctx: DataGridCellContext) => { const tone = cellOf(ctx.row, i).tone; return tone ? `cell-${tone}` : ""; } }),
    }));
</script>

{#snippet packCell({ row, column, formatted }: DataGridCellContext)}
  {@const c = cellOf(row, Number(column.id.slice(1)))}
  {#if c.symbol}<span aria-hidden="true" class="sym">{c.symbol}</span>{/if}{formatted}
{/snippet}


<svelte:head><title>{t("p.director_pack.director_packs_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref("/director-packs")}>{t("p.director_pack.all_packs")}</a></p>

{#if !pack}<h1>{t("p.director_pack.director_packs")}</h1>{/if}

{#if error}<p class="banner error" role="alert">{error}</p>{/if}

{#if pack}
  <h1>{t("p.director_pack.pack_heading", { period: pack.pack.period, audience: pack.pack.audience })}</h1>
  <p>
    {t("p.director_pack.version_line", {
      version: pack.pack.version,
      status: pack.pack.status === "issued" ? t("p.director_pack.status_issued") : t("p.director_pack.status_draft"),
      cutoff: formatDate(pack.pack.cut_off),
    })}
    {#if pack.pack.issued_at}· {t("p.director_pack.issued_on", { when: formatDate(pack.pack.issued_at) })}{/if}
  </p>
  {#if pack.pack.status !== "issued"}<p class="banner" role="status">{t("p.director_pack.draft_note")}</p>{/if}
  {#if pack.pack.change_note}<p>{t("p.director_pack.change_note", { note: pack.pack.change_note })}</p>{/if}
  {#if previous?.period}<p class="small muted">{t("p.director_pack.previous_pack", { period: previous.period, version: previous.version ?? 0 })}</p>{/if}
  {#if pack.pack.status === "draft"}
    <p class="small muted">
      {pack.pack.reviewed ? t("p.director_pack.is_reviewed") : t("p.director_pack.not_reviewed")}
      {#if pack.pack.origin === "scheduled"} · {t("p.director_pack.from_schedule")}{/if}
      {#if pack.pack.stale} · {t("p.director_pack.is_stale")}{/if}
    </p>
  {/if}
  {#if actions}
    <section class="noprint actions" aria-labelledby="pack-actions-h">
      <h2 id="pack-actions-h">{t("p.director_pack.actions_heading")}</h2>
      {#if actionError}<p class="banner error" role="alert">{actionError}</p>{/if}
      <p class="small" role="status">{notice}</p>
      {#if actions.review || actions.issue || actions.issueWaitsForReview}
        <div class="row">
          {#if actions.review}
            <button class="button" type="button" disabled={busy} onclick={() => act("review")}>{t("p.director_pack.review")}</button>
          {/if}
          <button class="button" type="button" disabled={busy || !actions.issue} onclick={() => act("issue")}>{t("p.director_pack.issue")}</button>
        </div>
        <p class="small muted">
          {actions.issueWaitsForReview ? t("p.director_pack.issue_waits") : t("p.director_pack.issue_says")}
        </p>
      {/if}
      {#if actions.reissue}
        <form class="stack" onsubmit={(e) => { e.preventDefault(); void act("reissue"); }}>
          <label>
            {t("p.director_pack.f_change_note")}
            <textarea bind:value={note} rows="3"></textarea>
          </label>
          <p class="small muted">{t("p.director_pack.reissue_says", { min: NOTE_MIN })}</p>
          <div class="row">
            <button class="button" type="submit" disabled={busy}>{t("p.director_pack.reissue")}</button>
          </div>
        </form>
      {/if}
    </section>
  {/if}
  <p class="small muted">{t("p.director_pack.basis")}</p>
  <p class="noprint">
    <a href={`/api/proxy/api/director-packs/${encodeURIComponent(pack.pack.pid)}/pdf`}>{t("p.director_pack.download_pdf")}</a>
    <span class="small muted"> — {t("p.director_pack.pdf_note")}</span>
  </p>

  {#each sections as s (s.id)}
    <section aria-labelledby={`pack-${s.id}`}>
      <h2 id={`pack-${s.id}`}>{s.title}</h2>
      {#each s.lines as line, i (i)}<p>{line}</p>{/each}
      {#each s.tables as tb, ti (ti)}
        <!-- A wide table scrolls on its own; the region is focusable so a keyboard can scroll it. -->
        <DataTable label={tb.caption} caption={tb.caption} captionVisible columns={packColumns(tb.columns)} rows={tb.rows.map((cells) => ({ cells }))} rowId={(_r, i) => String(i)} sortable={false} />
      {/each}
    </section>
  {/each}
{:else if !error}
  <p class="muted">{t("p.director_pack.loading")}</p>
{/if}

<style>
  :global(.sym) {
    display: inline-block;
    inline-size: 1.25em;
  }
  /* Colour is never the only signal: every reading also carries its word and its symbol. These tints are
     pale so the text keeps its contrast. */
  .actions {
    margin-block: 1rem;
  }
  @media print {
    .noprint { display: none; }
    section { break-inside: avoid; }
  }
</style>
