<!--
  Capacity commitments (`/capacity-commitments`, FB-6 completed, FB-5): the form for `PUT /api/capacity-commitments`
  (skill pool, programme, optional partner organization, supply or demand, month, committed and optional delivered
  full-time equivalents in the reader's own number format, note), reached from the capacity heatmap and from a
  programme's plan page (`?plan=<pid>` chooses it). After a save the page says, in plain words, what happened to
  the demand sent to the workforce service (`peer_demand_send`): started, taken, a withdrawal at zero, already
  there, refused, not reached, or why nothing was sent. Recent sends come from `GET /api/capacity-commitments`.
  The words are in `src/lib/capacity-commitments-view.ts`; this page only draws them.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type SkillPool } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import {
    readForm,
    recentSends,
    sendOutcome,
    type CapacityCommitment,
    type Problem,
  } from "#lib/capacity-commitments-view.js";
  import { monthLabel } from "#lib/capacity-heatmap-view.js";

  const ppm = PpmClient.withFetch();
  const repo = PlanRepository.withFetch();

  let pools = $state<SkillPool[]>([]);
  let plans = $state<PlanRef[]>([]);
  let commitments = $state<CapacityCommitment[] | null>(null);
  let loadError = $state<string | null>(null);

  const thisMonth = new Date().toISOString().slice(0, 7);
  let pool = $state("");
  let plan = $state(page.url.searchParams.get("plan") ?? "");
  let organization = $state("");
  let kind = $state("demand");
  let month = $state(thisMonth);
  let committed = $state("");
  let delivered = $state("");
  let note = $state("");

  let problems = $state<Problem[]>([]);
  let busy = $state(false);
  let failure = $state<string | null>(null);
  let saved = $state<CapacityCommitment | null>(null);

  const has = (p: Problem) => problems.includes(p);
  const activePools = $derived(pools.filter((p) => p.active));
  const poolName = (pid: string) => pools.find((p) => p.pid === pid)?.name ?? pid;
  const planName = (pid: string) => plans.find((p) => p.pid === pid)?.name ?? pid;

  async function loadSends() {
    try {
      commitments = (await ppm.capacityCommitments()).commitments;
    } catch (err) {
      loadError = errorText(err, "p.commitments.could_not_load");
    }
  }

  async function load() {
    try {
      const [ps, pl] = await Promise.all([ppm.skillPools(), repo.list()]);
      pools = ps.skill_pools;
      const chosen = plan;
      if (chosen && !pl.some((p) => p.pid === chosen)) {
        try {
          const one = await repo.get(chosen);
          pl.unshift({ pid: chosen, name: one.name });
        } catch {
          plan = "";
        }
      }
      plans = pl;
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.commitments.could_not_load");
    }
    await loadSends();
  }
  onMount(() => void load());

  function refusal(err: unknown): string {
    if (err instanceof ApiError && err.status === 404) return t("p.commitments.not_found");
    if (err instanceof ApiError && err.status === 422) return t("p.commitments.refused");
    return errorText(err, "p.commitments.could_not_save");
  }

  async function save() {
    failure = null;
    saved = null;
    const r = readForm({ pool, plan, organization, kind, month, committed, delivered, note });
    problems = r.problems;
    if (!r.body) return;
    busy = true;
    try {
      saved = await ppm.setCapacityCommitment(r.body);
      await loadSends();
    } catch (err) {
      failure = refusal(err);
    } finally {
      busy = false;
    }
  }

  const sends = $derived(commitments ? recentSends(commitments, poolName, planName) : []);
  const columns: DataGridColumn[] = $derived([
    { id: "pool", header: t("p.commitments.c_pool"), rowHeader: true },
    { id: "programme", header: t("p.commitments.c_programme") },
    { id: "month", header: t("p.commitments.c_month") },
    { id: "sent", header: t("p.commitments.c_sent") },
    { id: "outcome", header: t("p.commitments.c_outcome") },
    { id: "at", header: t("p.commitments.c_at") },
  ]);
  const describedBy = (id: string, bad: boolean) => (bad ? `${id}-hint ${id}-error` : `${id}-hint`);
</script>

<svelte:head><title>{t("p.commitments.title")}</title></svelte:head>

<h1>{t("p.commitments.h1")}</h1>
<p class="lede">{t("p.commitments.lede")}</p>
<p class="banner" role="note">{t("p.commitments.bridge_note")}</p>
<p><a href={localeHref("/capacity-heatmap")}>{t("p.commitments.see_heatmap")}</a></p>

{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}

<section aria-labelledby="commit-h" class="surface stack">
  <h2 id="commit-h">{t("p.commitments.form_h2")}</h2>
  <form novalidate onsubmit={(e) => { e.preventDefault(); void save(); }}>
    <label for="c-pool">{t("p.commitments.pool")}</label>
    <select id="c-pool" bind:value={pool} aria-invalid={has("pool")} aria-describedby={describedBy("c-pool", has("pool"))}>
      <option value="">{t("p.commitments.choose")}</option>
      {#each activePools as p (p.pid)}
        <option value={p.pid}>{p.peer_pool_ref ? t("p.commitments.pool_linked", { name: p.name }) : p.name}</option>
      {/each}
    </select>
    <p id="c-pool-hint" class="small muted">{t("p.commitments.pool_hint")}</p>
    {#if has("pool")}<p id="c-pool-error" class="small error-text">{t("p.commitments.pool_missing")}</p>{/if}

    <label for="c-plan">{t("p.commitments.plan")}</label>
    <select id="c-plan" bind:value={plan} aria-invalid={has("plan")} aria-describedby={describedBy("c-plan", has("plan"))}>
      <option value="">{t("p.commitments.choose")}</option>
      {#each plans as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
    </select>
    <p id="c-plan-hint" class="small muted">{t("p.commitments.plan_hint")}</p>
    {#if has("plan")}<p id="c-plan-error" class="small error-text">{t("p.commitments.plan_missing")}</p>{/if}

    <fieldset>
      <legend>{t("p.commitments.kind")}</legend>
      <label class="choice"><input type="radio" name="kind" value="demand" bind:group={kind} /> {t("p.commitments.kind_demand")}</label>
      <label class="choice"><input type="radio" name="kind" value="supply" bind:group={kind} /> {t("p.commitments.kind_supply")}</label>
    </fieldset>

    <label for="c-org">{t("p.commitments.organization")}</label>
    <input id="c-org" bind:value={organization} maxlength="200" autocomplete="organization" aria-describedby="c-org-hint" />
    <p id="c-org-hint" class="small muted">{t("p.commitments.organization_hint")}</p>

    <label for="c-month">{t("p.commitments.month")}</label>
    <input id="c-month" type="month" bind:value={month} aria-invalid={has("month")} aria-describedby={describedBy("c-month", has("month"))} />
    <p id="c-month-hint" class="small muted">{t("p.commitments.month_hint")}</p>
    {#if has("month")}<p id="c-month-error" class="small error-text">{t("p.commitments.month_invalid")}</p>{/if}

    <label for="c-committed">{t("p.commitments.committed")}</label>
    <input id="c-committed" inputmode="decimal" autocomplete="off" bind:value={committed} aria-invalid={has("committed")}
      aria-describedby={describedBy("c-committed", has("committed"))} />
    <p id="c-committed-hint" class="small muted">{t("p.commitments.committed_hint")}</p>
    {#if has("committed")}<p id="c-committed-error" class="small error-text">{t("p.commitments.fte_invalid")}</p>{/if}

    <label for="c-delivered">{t("p.commitments.delivered")}</label>
    <input id="c-delivered" inputmode="decimal" autocomplete="off" bind:value={delivered} aria-invalid={has("delivered")}
      aria-describedby={describedBy("c-delivered", has("delivered"))} />
    <p id="c-delivered-hint" class="small muted">{t("p.commitments.delivered_hint")}</p>
    {#if has("delivered")}<p id="c-delivered-error" class="small error-text">{t("p.commitments.fte_invalid")}</p>{/if}

    <label for="c-note">{t("p.commitments.note")}</label>
    <input id="c-note" bind:value={note} maxlength="500" autocomplete="off" />

    <div class="row">
      <button class="button primary" type="submit" disabled={busy}>{t("p.commitments.save")}</button>
    </div>
  </form>

  {#if failure}<p class="banner error" role="alert" data-testid="commit-failure">{failure}</p>{/if}
  {#if saved}
    <div class="stack" role="status" data-testid="commit-done">
      <p class="banner">{t("p.commitments.saved", { pool: poolName(saved.pool_pid), plan: planName(saved.plan_pid), month: monthLabel(saved.month) })}</p>
      {#if saved.kind === "demand"}
        <p class="banner" data-testid="send-outcome">{sendOutcome(saved.peer_demand_send)}</p>
      {/if}
    </div>
  {/if}
</section>

<section aria-labelledby="sends-h">
  <h2 id="sends-h">{t("p.commitments.sends_h2")}</h2>
  <p class="small muted">{t("p.commitments.sends_note")}</p>
  <div class="row"><button class="button" type="button" onclick={() => void loadSends()}>{t("p.commitments.refresh")}</button></div>
  {#if commitments === null && !loadError}
    <p class="muted">{t("p.commitments.loading")}</p>
  {:else if sends.length === 0}
    <p>{t("p.commitments.no_sends")}</p>
  {:else}
    <DataTable label={t("p.commitments.sends_h2")} {columns} rows={sends} rowId={(r) => String(r.key)} sortable={false} />
  {/if}
</section>

<style>
  form { display: grid; gap: 0.4rem; max-width: 40rem; }
  fieldset { display: grid; gap: 0.3rem; border: 1px solid var(--mxi-color-border, #ddd); padding: 0.5rem 0.75rem; }
  .choice { display: flex; gap: 0.4rem; align-items: center; min-height: 1.5rem; }
  .choice input { width: 1.5rem; height: 1.5rem; margin: 0; }
  .error-text { color: var(--mxi-color-danger); font-weight: 600; }
</style>
