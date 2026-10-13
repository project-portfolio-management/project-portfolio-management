<!--
  One director pack (`/director-packs/[pid]`, DP-11): every section of the issued pack as words and
  tables. A reading is a word and a symbol as well as a colour. The figures are read from the stored
  pack; fixed text comes from the catalogue, free text is shown as entered. Print it for a PDF in your own
  language, or download the service's English PDF.
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { i18n } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type DirectorPack } from "#lib/api/ppm.js";
  import { buildPackView, type Section } from "#lib/pack-view.js";

  const ppm = PpmClient.withFetch();
  let pack = $state<DirectorPack | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      pack = await ppm.directorPack(page.params.pid ?? "");
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.director_pack.could_not_load_one");
    }
  });

  const sections: Section[] = $derived(
    pack
      ? buildPackView(pack.body, {
          t: (k, v) => t(k as never, v),
          label: (g, v) => enumLabel(g as never, v),
          locale: i18n.locale,
        })
      : [],
  );
  const previous = $derived(
    pack ? ((pack.body.cover as { previous_issued_pack?: { period?: string; version?: number } } | undefined)?.previous_issued_pack ?? null) : null,
  );

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
      cutoff: pack.pack.cut_off.slice(0, 10),
    })}
    {#if pack.pack.issued_at}· {t("p.director_pack.issued_on", { when: pack.pack.issued_at.slice(0, 10) })}{/if}
  </p>
  {#if pack.pack.status !== "issued"}<p class="banner" role="status">{t("p.director_pack.draft_note")}</p>{/if}
  {#if pack.pack.change_note}<p>{t("p.director_pack.change_note", { note: pack.pack.change_note })}</p>{/if}
  {#if previous?.period}<p class="small muted">{t("p.director_pack.previous_pack", { period: previous.period, version: previous.version ?? 0 })}</p>{/if}
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
        <DataTable label={tb.caption} caption={tb.caption} columns={packColumns(tb.columns)} rows={tb.rows.map((cells) => ({ cells }))} rowId={(_r, i) => String(i)} sortable={false} />
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
  @media print {
    .noprint { display: none; }
    section { break-inside: avoid; }
  }
</style>
