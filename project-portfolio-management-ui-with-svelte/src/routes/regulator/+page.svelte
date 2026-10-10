<!--
  Regulator area (`/regulator`): the deliberately coarse extract —
  portfolio-level aggregates only (no person references, no item-level
  budgets; the server states this and may withhold names under an ABAC
  `mask` obligation). English-first, like the other PPM views.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, money, type RegulatorExtract } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let extract = $state<RegulatorExtract | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      extract = await ppm.regulatorExtract();
    } catch (err) {
      error = err instanceof Error ? err.message : t("ppm.common.loadFailed");
    }
  });
</script>

<svelte:head><title>{t("p.regulator.ppm", { t: t("ppm.nav.regulator") })}</title></svelte:head>

<h1>{t("ppm.nav.regulator")}</h1>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if extract}
  <p class="muted">{extract.note}</p>
  {#if extract.masked}
    <p class="banner" data-testid="regulator-masked">
      {t("p.regulator.names_withheld_abac_mask_obligation")}
    </p>
  {/if}
  {#each extract.portfolios as portfolio (portfolio.pid)}
    <section data-testid="regulator-portfolio">
      <h2>{portfolio.name ?? portfolio.pid}</h2>
      <p class="muted">{t("p.regulator.stage", { v: portfolio.stage ?? t("p.common.pre_gate") })}</p>
      <p>
        {t("p.regulator.members")}
        {#each Object.entries(portfolio.members) as [kind, count] (kind)}
          {count} {kind}&nbsp;
        {/each}
        {t("p.regulator.gate_decisions")}
        {#each Object.entries(portfolio.gate_decisions) as [decision, count] (decision)}
          {count} {decision}&nbsp;
        {:else}
          <span class="muted">{t("p.regulator.none")}</span>
        {/each}
      </p>
      <table>
        <thead><tr><th>{t("p.regulator.currency")}</th><th>{t("p.regulator.planned")}</th><th>{t("p.regulator.actual")}</th></tr></thead>
        <tbody>
          {#each portfolio.spend as row (row.currency)}
            <tr>
              <td>{row.currency}</td>
              <td>{money(row.planned_minor, row.currency)}</td>
              <td>{money(row.actual_minor, row.currency)}</td>
            </tr>
          {:else}
            <tr><td colspan="3" class="muted">{t("p.regulator.no_budget_lines")}</td></tr>
          {/each}
        </tbody>
      </table>
      {#if portfolio.benefits.length > 0}
        <p>
          {t("p.regulator.benefits")}
          {#each portfolio.benefits as row (row.currency)}
            {t("p.regulator.target_realized", { money: money(row.target_minor, row.currency), money2: money(row.realized_minor, row.currency) })}
          {/each}
        </p>
      {/if}
    </section>
  {:else}
    <p class="muted">{t("p.regulator.no_portfolios_recorded")}</p>
  {/each}
{/if}
