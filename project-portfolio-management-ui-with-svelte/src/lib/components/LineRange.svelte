<!--
  A budget line's low and high final cost, for the cost-risk simulation. The
  planned amount is the likely cost; with no range the simulation uses its
  fallback band and says so. Low may not be above the planned amount, nor high
  below it.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { minorToInput } from "#lib/money.js";
  import { money, type PpmClient } from "#lib/api/ppm.js";
  import { lineEstimateProblems, rangeMinor, rangeText } from "#lib/simulation-inputs.js";

  let {
    line,
    pid,
    ppm,
    onchange,
    onerror,
  }: {
    line: { pid: string; currency: string; planned_minor: number; low_minor?: number | null; high_minor?: number | null };
    pid: string;
    ppm: PpmClient;
    onchange: () => void;
    onerror: (message: string) => void;
  } = $props();

  let low = $state("");
  let high = $state("");
  let problems = $state<string[]>([]);
  // Follows what is stored; opening the section never resets what is typed.
  $effect(() => {
    low = line.low_minor === null || line.low_minor === undefined ? "" : minorToInput(line.low_minor, line.currency);
    high = line.high_minor === null || line.high_minor === undefined ? "" : minorToInput(line.high_minor, line.currency);
    problems = [];
  });
  async function save(e: SubmitEvent) {
    e.preventDefault();
    problems = lineEstimateProblems(low, high, line.planned_minor, line.currency);
    if (problems.length > 0) return;
    try {
      await ppm.setLineEstimate(pid, line.pid, { low_minor: rangeMinor(low, line.currency), high_minor: rangeMinor(high, line.currency) });
      onchange();
    } catch (err) {
      onerror(errorText(err, "c.LineRange.could_not_save"));
    }
  }
</script>

<details class="line-range">
  <summary class="small">{rangeText(line.low_minor ?? null, line.high_minor ?? null, (m) => money(m, line.currency))}</summary>
  <form class="inline" onsubmit={save}>
    <label>{t("c.LineRange.low")} <input type="text" inputmode="decimal" bind:value={low} /></label>
    <label>{t("c.LineRange.high")} <input type="text" inputmode="decimal" bind:value={high} /></label>
    <button class="button small" type="submit">{t("c.LineRange.save_range")}</button>
  </form>
  {#each problems as p (p)}<p class="banner" role="alert">{p}</p>{/each}
  <p class="small muted">{t("c.LineRange.planned_is_the_likely_cost", { money: money(line.planned_minor, line.currency) })}</p>
</details>

<style>
  .inline {
    display: flex;
    flex-wrap: wrap;
    gap: 0.6rem;
    align-items: end;
    margin: 0.4rem 0;
  }
  .inline label {
    display: grid;
    gap: 0.15rem;
    font-size: 0.8rem;
  }
</style>
