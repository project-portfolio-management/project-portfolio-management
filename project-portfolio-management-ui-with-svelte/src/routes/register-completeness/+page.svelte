<!--
  Register completeness (`/register-completeness`, EX-1; NC-1 slice 12 part d): how complete each programme's register
  is, every missing field named, and whether "no register, no money" is on. Linked from `/director-packs` and
  `/exceptions`. Read-only. The words are the catalogue's (`src/lib/register-completeness-view.ts`); the service's
  `note` is English and is not shown.
-->
<script lang="ts">
  import { t, tn, localeHref } from "#lib/i18n.svelte.js";
  import { formatInteger, formatPercentValue } from "#lib/locale-format.js";
  import { onMount } from "svelte";
  import { resource } from "#lib/resource.svelte.js";
  import { rowOf } from "#lib/row-of.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { PpmClient, type RegisterCompleteness, type RegisterProgramme } from "#lib/api/ppm.js";
  import { counts, fieldList, orderProgrammes, stateOf, switchKey } from "#lib/register-completeness-view.js";

  const ppm = PpmClient.withFetch();
  const loaded = resource<RegisterCompleteness>(() => ppm.registerCompleteness(), "p.register_completeness.could_not_load");
  const register = $derived(loaded.data);
  const rows = $derived(register ? orderProgrammes(register.programmes) : []);
  const tally = $derived(register ? counts(register) : null);
  onMount(() => void loaded.reload());

  const columns: DataGridColumn[] = $derived([
    { id: "name", header: t("p.register_completeness.col_programme"), rowHeader: true, cell: nameCell },
    { id: "complete", header: t("p.register_completeness.col_state"), format: (_v, r) => stateOf(rowOf<RegisterProgramme>(r)).text, cell: stateCell },
    { id: "score_pct", header: t("p.register_completeness.col_score"), align: "end", format: (v) => formatPercentValue(Number(v)) },
    {
      id: "present",
      header: t("p.register_completeness.col_on_record"),
      align: "end",
      format: (_v, r) => {
        const p = rowOf<RegisterProgramme>(r);
        return t("p.register_completeness.of", { present: formatInteger(p.present), applicable: formatInteger(p.applicable) });
      },
    },
    { id: "missing", header: t("p.register_completeness.col_missing"), sortable: false, format: (v) => fieldList(v as string[]) },
    { id: "not_applicable", header: t("p.register_completeness.col_not_asked"), sortable: false, format: (v) => fieldList(v as string[]) },
  ]);
</script>

{#snippet nameCell({ row }: DataGridCellContext)}
  {@const p = rowOf<RegisterProgramme>(row)}
  <a href={localeHref(`/plans/${p.plan_pid}`)}>{p.name}</a>
{/snippet}

{#snippet stateCell({ row }: DataGridCellContext)}
  {@const s = stateOf(rowOf<RegisterProgramme>(row))}
  <span aria-hidden="true">{s.symbol}</span> {s.text}
{/snippet}

<svelte:head><title>{t("p.register_completeness.title_ppm")}</title></svelte:head>

<h1>{t("p.register_completeness.title")}</h1>
<p class="lede">{t("p.register_completeness.intro")}</p>
<p class="small muted">{t("p.register_completeness.rules")}</p>
{#if loaded.error}<p class="banner" role="alert">{loaded.error}</p>{/if}
{#if !register && !loaded.error}<p class="muted">{t("p.register_completeness.loading")}</p>{/if}

{#if register && tally}
  <p data-testid="rc-switch"><strong>{t(switchKey(register.enforced))}</strong></p>
  {#if tally.total === 0}
    <p>{t("p.register_completeness.none")}</p>
  {:else}
    <p data-testid="rc-count">{tn("p.register_completeness.count", tally.total, { complete: formatInteger(tally.complete), n: formatInteger(tally.total) })}</p>
    <DataTable label={t("p.register_completeness.table")} {columns} {rows} rowId={(r) => String(r.plan_pid)} sortable={false} />
  {/if}
{/if}
