<!--
  Choosing one of the plan's records (NC-1 slice 12 part e): a list grouped by kind (tasks, risks, milestones, issues),
  with "none" first. The value is `kind:<pid>`, or "" for none. Disabled until the records have loaded.
-->
<script lang="ts">
  import { t, tDynamic } from "#lib/i18n.svelte.js";
  import { LINK_KINDS } from "#lib/meeting-view.js";
  import type { PlanRecords } from "#lib/plan-records.js";

  let {
    label,
    value = $bindable(""),
    records,
    loaded,
    none,
  }: {
    label: string;
    value?: string;
    records: PlanRecords;
    /** Whether the records have been read. */
    loaded: boolean;
    /** The words for choosing no record. */
    none: string;
  } = $props();
</script>

<label>
  {label}
  <select bind:value disabled={!loaded}>
    <option value="">{none}</option>
    {#each LINK_KINDS as k (k)}
      {#if (records[k] ?? []).length > 0}
        <optgroup label={tDynamic(`p.status.group_${k}`)}>
          {#each records[k] ?? [] as r (r.pid)}<option value={`${k}:${r.pid}`}>{r.title}</option>{/each}
        </optgroup>
      {/if}
    {/each}
  </select>
</label>
{#if !loaded}<span class="small muted">{t("p.meetings.records_loading")}</span>{/if}
