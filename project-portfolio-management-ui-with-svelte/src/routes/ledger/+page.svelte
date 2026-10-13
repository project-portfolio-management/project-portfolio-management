<!--
  The funding ledger (`/ledger`, EX-12): each allocation and what it bought. Untraced money is shown with its
  amount and why.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type FundingAllocation, type AllocationTrace } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let allocations = $state<FundingAllocation[] | null>(null);
  let trace = $state<AllocationTrace | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      allocations = (await ppm.fundingAllocations()).allocations;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.ledger.could_not_load");
    }
  });

  async function open(a: FundingAllocation) {
    try {
      trace = await ppm.traceAllocation(a.pid);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.ledger.could_not_trace");
    }
  }
  /** Minor units (pence, cents) as a decimal amount. */
  const money = (minor: number) => `${minor < 0 ? "-" : ""}${Math.floor(Math.abs(minor) / 100)}.${String(Math.abs(minor) % 100).padStart(2, "0")}`;
  const label = (a: FundingAllocation) => `${a.funding_source} ${a.allocated_on}`;
</script>

<svelte:head><title>{t("p.ledger.title")}</title></svelte:head>

<h1>{t("p.ledger.h1")}</h1>
<p class="lede">{t("p.ledger.lede")}</p>
<p class="muted">{t("p.ledger.scope")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !allocations && !error}<p class="muted">{t("p.ledger.loading")}</p>{/if}

{#if allocations}
  {#if allocations.length === 0}
    <p>{t("p.ledger.none")}</p>
  {:else}
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div class="scroll" role="region" aria-label={t("p.ledger.h1")} tabindex="0">
      <table>
        <caption class="sr-only">{t("p.ledger.h1")}</caption>
        <thead>
          <tr>
            <th scope="col">{t("p.ledger.col_source")}</th>
            <th scope="col">{t("p.ledger.col_ref")}</th>
            <th scope="col">{t("p.ledger.col_on")}</th>
            <th scope="col">{t("p.ledger.col_amount")}</th>
            <th scope="col">{t("p.ledger.col_planned")}</th>
            <th scope="col">{t("p.ledger.col_untraced")}</th>
            <th scope="col">{t("p.ledger.col_lines")}</th>
            <th scope="col">{t("p.ledger.col_act")}</th>
          </tr>
        </thead>
        <tbody>
          {#each allocations as a (a.pid)}
            <tr>
              <th scope="row">{a.funding_source}</th>
              <td>{a.reference ?? ""}</td>
              <td>{a.allocated_on}</td>
              <td>{money(a.amount_minor)} {a.currency}</td>
              <td>{money(a.planned_on_lines_minor)}</td>
              <td class:untraced={a.untraced_minor !== 0}>{money(a.untraced_minor)}</td>
              <td>{a.lines}</td>
              <td><button class="button" type="button" aria-label={t("p.ledger.trace", { name: label(a) })} onclick={() => void open(a)}>{t("p.ledger.trace_short")}</button></td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
{/if}

{#if trace}
  <section aria-labelledby="lg-trace">
    <h2 id="lg-trace">{t("p.ledger.trace_h2", { name: label(trace.allocation) })}</h2>
    <p>{t("p.ledger.money", { allocated: money(trace.money.allocated_minor), planned: money(trace.money.planned_on_lines_minor), spent: money(trace.money.spent_minor), untraced: money(trace.money.untraced_minor), currency: trace.currency })}</p>
    {#if trace.money.untraced_reason}<p class="banner" role="note">{trace.money.untraced_reason}</p>{/if}
    <p>{t("p.ledger.gates", { gated: trace.gates.lines_with_a_gate, released: trace.gates.released })}</p>
    <p>{t("p.ledger.milestones_met", { met: trace.milestones.met, total: trace.milestones.total })}</p>
    <p>{t("p.ledger.benefits", { n: trace.benefits.benefits })}
      {#each trace.benefits.validated_by_currency as v (v.currency)}{t("p.ledger.validated", { currency: v.currency, amount: money(v.validated_minor) })}. {/each}
    </p>
    {#if trace.lines.length > 0}
      <h3>{t("p.ledger.lines_h3")}</h3>
      <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
      <div class="scroll" role="region" aria-label={t("p.ledger.lines_h3")} tabindex="0">
        <table>
          <caption class="sr-only">{t("p.ledger.lines_h3")}</caption>
          <thead>
            <tr>
              <th scope="col">{t("p.ledger.line_plan")}</th>
              <th scope="col">{t("p.ledger.line_desc")}</th>
              <th scope="col">{t("p.ledger.line_planned")}</th>
              <th scope="col">{t("p.ledger.line_actual")}</th>
              <th scope="col">{t("p.ledger.line_gate")}</th>
            </tr>
          </thead>
          <tbody>
            {#each trace.lines as l (l.line_pid)}
              <tr>
                <th scope="row">{l.plan ?? ""}</th>
                <td>{l.description}</td>
                <td>{money(l.planned_minor)}</td>
                <td>{money(l.actual_minor)}</td>
                <td>{l.gate ? `${l.gate}: ${l.released ? t("p.ledger.released") : t("p.ledger.held")}` : t("p.ledger.no_gate")}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </section>
{/if}

<style>
  .scroll { overflow-x: auto; margin-block: 1rem; }
  .untraced { background: #fff4d6; }
</style>
