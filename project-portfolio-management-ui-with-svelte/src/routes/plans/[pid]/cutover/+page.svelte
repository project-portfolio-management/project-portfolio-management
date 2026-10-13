<!--
  Cutover go/no-go (`/plans/[pid]/cutover`, EX-6): stop criteria published and frozen before the decision,
  evidence against each, co-signatures by role, and the decision. A no-go or a delay for safety is a decision,
  not slippage.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type CutoverDecision } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let decisions = $state<CutoverDecision[] | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let title = $state("");
  let criteriaText = $state("");
  let evidence = $state<Record<string, string>>({});
  let role = $state<Record<string, string>>({});
  let outcome = $state<Record<string, string>>({});
  let reason = $state<Record<string, string>>({});

  async function load() {
    try {
      decisions = (await ppm.cutoverDecisions(pid)).decisions;
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.cutover.could_not_load");
    }
  }
  onMount(() => void load());

  async function run(job: () => Promise<unknown>) {
    busy = true;
    try {
      await job();
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.cutover.could_not_save");
    } finally {
      busy = false;
    }
  }
  const draft = async () => {
    await run(() => ppm.createCutoverDecision(pid, title.trim(), criteriaText.split("\n").map((l) => l.trim()).filter(Boolean)));
    if (!error) { title = ""; criteriaText = ""; }
  };
  const status = (s: string) => t(`p.cutover.status_${s}` as never);
  const outcomeWord = (o: string) => t(`p.cutover.outcome_${o}` as never);
  const list = (xs: string[]) => (xs.length ? xs.join(", ") : t("p.cutover.cosign_none"));
</script>

<svelte:head><title>{t("p.cutover.title")}</title></svelte:head>

<h1>{t("p.cutover.h1")}</h1>
<p class="lede">{t("p.cutover.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !decisions && !error}<p class="muted">{t("p.cutover.loading")}</p>{/if}

{#if decisions}
  <section aria-labelledby="co-new">
    <h2 id="co-new">{t("p.cutover.new_h2")}</h2>
    <form onsubmit={(e) => { e.preventDefault(); if (title.trim() && criteriaText.trim()) void draft(); }}>
      <label>{t("p.cutover.new_title")} <input bind:value={title} required maxlength="200" /></label>
      <label>{t("p.cutover.new_criteria")} <textarea bind:value={criteriaText} rows="4" required></textarea></label>
      <button class="button" type="submit" disabled={busy || !title.trim() || !criteriaText.trim()}>{t("p.cutover.draft")}</button>
    </form>
  </section>

  {#if decisions.length === 0}
    <p>{t("p.cutover.none")}</p>
  {/if}

  {#each decisions as d (d.pid)}
    <section class="decision" aria-labelledby={`co-${d.pid}`}>
      <h2 id={`co-${d.pid}`}>{d.title} <small>({t("p.cutover.version", { n: d.version })}, {status(d.status)})</small></h2>
      {#if d.outcome && d.decided_on}
        <p class="banner" role="note">{t("p.cutover.decided_line", { on: d.decided_on.slice(0, 10), outcome: outcomeWord(d.outcome), why: d.decision_reason ?? "" })}</p>
      {/if}

      <h3>{t("p.cutover.criteria_h3")}</h3>
      <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
      <div class="scroll" role="region" aria-label={`${d.title}: ${t("p.cutover.criteria_h3")}`} tabindex="0">
        <table>
          <caption class="sr-only">{d.title}: {t("p.cutover.criteria_h3")}</caption>
          <thead>
            <tr>
              <th scope="col">{t("p.cutover.col_n")}</th>
              <th scope="col">{t("p.cutover.col_criterion")}</th>
              <th scope="col">{t("p.cutover.col_met")}</th>
              <th scope="col">{t("p.cutover.col_evidence")}</th>
              {#if d.status === "open"}<th scope="col">{t("p.cutover.col_mark")}</th>{/if}
            </tr>
          </thead>
          <tbody>
            {#each d.criteria as c (c.n)}
              <tr>
                <td>{c.n}</td>
                <th scope="row">{c.criterion}</th>
                <td class={c.met ? "ok" : "no"}><span aria-hidden="true">{c.met ? "●" : "✖"}</span> {c.met ? t("p.cutover.met") : t("p.cutover.not_met")}</td>
                <td>{c.evidence ?? ""}</td>
                {#if d.status === "open"}
                  <td>
                    <label>{t("p.cutover.evidence")} <input bind:value={evidence[`${d.pid}-${c.n}`]} maxlength="500" /></label>
                    <button class="button" type="button" disabled={busy} aria-label={t("p.cutover.mark_met", { name: `${c.n}` })} onclick={() => void run(() => ppm.markCutoverCriterion(pid, d.pid, c.n, true, (evidence[`${d.pid}-${c.n}`] ?? "").trim()))}>{t("p.cutover.mark_met_short")}</button>
                    <button class="button" type="button" disabled={busy} aria-label={t("p.cutover.mark_unmet", { name: `${c.n}` })} onclick={() => void run(() => ppm.markCutoverCriterion(pid, d.pid, c.n, false, (evidence[`${d.pid}-${c.n}`] ?? "").trim()))}>{t("p.cutover.mark_unmet_short")}</button>
                  </td>
                {/if}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <h3>{t("p.cutover.cosign_h3")}</h3>
      <p>{t("p.cutover.cosign_signed", { roles: list(d.cosignatures.signed.map((s) => s.role)) })} {t("p.cutover.cosign_missing", { roles: list(d.cosignatures.missing) })}</p>
      {#if d.status === "open"}
        <form onsubmit={(e) => { e.preventDefault(); const r = (role[d.pid] ?? "").trim(); if (r) void run(() => ppm.cosignCutover(pid, d.pid, r)); }}>
          <label>{t("p.cutover.cosign_role")} <input bind:value={role[d.pid]} maxlength="100" /></label>
          <button class="button" type="submit" disabled={busy}>{t("p.cutover.cosign")}</button>
        </form>
      {/if}

      {#if d.go_would_be_refused_because.length > 0}
        <h3>{t("p.cutover.refused_h3")}</h3>
        <ul>{#each d.go_would_be_refused_because as why (why)}<li>{why}</li>{/each}</ul>
      {/if}

      <div class="steps">
        {#if d.status === "draft"}
          <button class="button" type="button" disabled={busy} onclick={() => void run(() => ppm.cutoverStep(pid, d.pid, "publish"))}>{t("p.cutover.publish")}</button>
        {:else if d.status === "published"}
          <button class="button" type="button" disabled={busy} onclick={() => void run(() => ppm.cutoverStep(pid, d.pid, "open"))}>{t("p.cutover.open")}</button>
        {/if}
        {#if d.status === "published" || d.status === "open" || d.status === "decided"}
          <button class="button" type="button" disabled={busy} onclick={() => void run(() => ppm.cutoverStep(pid, d.pid, "new-version"))}>{t("p.cutover.new_version")}</button>
        {/if}
      </div>

      {#if d.status === "open"}
        <h3>{t("p.cutover.decide_h3")}</h3>
        <form onsubmit={(e) => { e.preventDefault(); const o = outcome[d.pid] ?? "go"; void run(() => ppm.decideCutover(pid, d.pid, o, (reason[d.pid] ?? "").trim())); }}>
          <label>{t("p.cutover.outcome")}
            <select bind:value={outcome[d.pid]}>{#each ["go", "no_go", "delay_for_safety"] as o (o)}<option value={o}>{outcomeWord(o)}</option>{/each}</select>
          </label>
          <label>{t("p.cutover.reason")} <input bind:value={reason[d.pid]} required maxlength="1000" /></label>
          <button class="button" type="submit" disabled={busy}>{t("p.cutover.decide")}</button>
        </form>
      {/if}
    </section>
  {/each}
{/if}

<style>
  .scroll { overflow-x: auto; margin-block: 1rem; }
  form { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: end; margin-block: 0.5rem; }
  textarea { min-inline-size: 20rem; }
  .decision { border-block-start: 1px solid #ccc; margin-block-start: 1.5rem; padding-block-start: 0.5rem; }
  .steps { display: flex; gap: 0.5rem; flex-wrap: wrap; margin-block: 0.75rem; }
  .ok { background: #e3f4e3; }
  .no { background: #fde8e8; }
</style>
