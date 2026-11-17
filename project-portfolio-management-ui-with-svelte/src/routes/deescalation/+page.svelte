<!--
  Scorecards and de-escalation (`/deescalation`, EX-10, NC-1 slice 11): the de-escalation tracker
  (`DeescalationTracker.svelte`) over `GET /api/deescalation` and its three writes, and the scorecards
  (`Scorecards.svelte`) over `/api/scorecards` and the starter template. A change to a scorecard reads the tracker
  again, as the tracker may read that scorecard. The rules are in `src/lib/deescalation-view.ts`.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import DeescalationTracker from "#lib/components/DeescalationTracker.svelte";
  import Scorecards from "#lib/components/Scorecards.svelte";

  let tracker = $state<ReturnType<typeof DeescalationTracker> | null>(null);
</script>

<svelte:head><title>{t("p.deescalation.title")}</title></svelte:head>

<h1>{t("p.deescalation.h1")}</h1>
<p class="lede">{t("p.deescalation.lede")}</p>

<DeescalationTracker bind:this={tracker} />
<Scorecards onChanged={async () => { await tracker?.reload(); }} />
