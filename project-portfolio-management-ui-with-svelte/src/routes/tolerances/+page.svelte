<!--
  Tolerances (`/tolerances`): every plan you can read that has a tolerance, worst first, with the overall colour and
  each dimension's colour. A colour is always a word and a shape too, and "not measurable" is its own state: a limit
  that could not be measured is never shown as green.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type ToleranceBoard, type ToleranceStatus } from "#lib/api/ppm.js";
  import { DIMENSIONS, STATUS_SYMBOL, STATUS_TEXT, dimensionLabel } from "#lib/tolerance-view.js";

  const ppm = PpmClient.withFetch();
  let board = $state<ToleranceBoard | null>(null);
  let error = $state<string | null>(null);
  let status = $state("");

  async function refresh() {
    try {
      board = await ppm.toleranceBoard({ status: status || undefined });
      error = null;
    } catch (err) {
      error = errorText(err, "p.tolerances.could_not_load");
    }
  }
  onMount(refresh);

  const FILTER: ToleranceStatus[] = ["red", "amber", "unknown", "green"];
  const statusOf = (plan: NonNullable<typeof board>["plans"][number], d: string): ToleranceStatus =>
    plan.dimensions.find((x) => x.dimension === d)?.status ?? "not_set";
</script>

<svelte:head><title>{t("p.tolerances.tolerances_ppm")}</title></svelte:head>

<h1>{t("p.tolerances.tolerances")}</h1>
<p class="small muted">{t("p.tolerances.board_intro")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void refresh(); }}>
  <label>{t("p.tolerances.show")}
    <select bind:value={status} onchange={refresh}>
      <option value="">{t("p.tolerances.all_statuses")}</option>
      {#each FILTER as s (s)}<option value={s}>{STATUS_TEXT[s]}</option>{/each}
    </select>
  </label>
</form>

{#if board}
  {#if board.truncated}<p class="banner" role="note">{t("p.tolerances.truncated", { n: board.measured_at_most })}</p>{/if}
  {#if board.plans.length === 0}
    <p>{status ? t("p.tolerances.none_match") : t("p.tolerances.none")}</p>
  {:else}
    <table>
      <caption class="sr-only">{t("p.tolerances.tolerances")}</caption>
      <thead>
        <tr>
          <th scope="col">{t("p.tolerances.plan")}</th>
          <th scope="col">{t("p.tolerances.overall")}</th>
          {#each DIMENSIONS as d (d)}<th scope="col">{dimensionLabel(d)}</th>{/each}
        </tr>
      </thead>
      <tbody>
        {#each board.plans as plan (plan.plan_pid)}
          <tr>
            <th scope="row"><a href={localeHref(`/plans/${plan.plan_pid}/tolerances`)}>{plan.plan_name}</a></th>
            <td class={`rag rag-${plan.overall}`}><span aria-hidden="true">{STATUS_SYMBOL[plan.overall]}</span> {STATUS_TEXT[plan.overall]}</td>
            {#each DIMENSIONS as d (d)}
              {@const s = statusOf(plan, d)}
              <td class={`rag rag-${s}`}><span aria-hidden="true">{STATUS_SYMBOL[s]}</span> {STATUS_TEXT[s]}</td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
{:else if !error}
  <p class="muted">{t("p.tolerances.loading")}</p>
{/if}

<style>
  .filter { display: flex; gap: 1rem; align-items: end; margin: 1rem 0; }
  table { border-collapse: collapse; inline-size: 100%; }
  th, td { text-align: start; padding: 0.4rem 0.6rem; border-block-end: 1px solid currentColor; }
  .rag { white-space: nowrap; border-inline-start: 0.4rem solid transparent; }
  .rag-red { border-inline-start-color: #b00020; }
  .rag-amber { border-inline-start-color: #c77700; }
  .rag-green { border-inline-start-color: #1b7a3a; }
  .rag-unknown { border-inline-start-color: #5a5a5a; border-inline-start-style: dotted; }
  .rag-not_set { border-inline-start-color: #9a9a9a; border-inline-start-style: dashed; }
  .sr-only { position: absolute; inline-size: 1px; block-size: 1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap; }
</style>
