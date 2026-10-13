<!--
  Milestone sign-off (`/milestone-signoffs`, EX-4): milestones that are met only when named partner
  organizations have signed them off. Delivered and met are different things, and the gap is shown.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type MilestoneSignoffs, type Organization } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let data = $state<MilestoneSignoffs | null>(null);
  let orgs = $state<Organization[]>([]);
  let pick = $state<Record<string, string>>({});
  let reason = $state<Record<string, string>>({});
  let error = $state<string | null>(null);
  let busy = $state(false);

  async function load() {
    try {
      data = await ppm.milestoneSignoffs();
      orgs = (await ppm.organizations()).organizations.filter((o) => o.active);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.signoffs.could_not_load");
    }
  }
  onMount(() => void load());

  async function act(m: MilestoneSignoffs["milestones"][number], action: "sign" | "withdraw") {
    const org = pick[m.milestone_pid] ?? orgs[0]?.pid;
    if (!org) return;
    busy = true;
    try {
      await ppm.actOnSignoff(m.plan_pid, m.milestone_pid, org, action, reason[m.milestone_pid]?.trim() || undefined);
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.signoffs.could_not_save");
    } finally {
      busy = false;
    }
  }
  const SYM: Record<string, string> = { met: "●", delivered_not_met: "▲", not_delivered: "–", not_required: "–" };
  const share = $derived(
    data?.totals.met_share_of_delivered_bp == null ? t("p.signoffs.no_share") : `${Math.round(data.totals.met_share_of_delivered_bp / 100)}%`,
  );
</script>

<svelte:head><title>{t("p.signoffs.title")}</title></svelte:head>

<h1>{t("p.signoffs.h1")}</h1>
<p class="lede">{t("p.signoffs.lede")}</p>
<p class="muted">{t("p.signoffs.not_built")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !data && !error}<p class="muted">{t("p.signoffs.loading")}</p>{/if}

{#if data}
  <p>{t("p.signoffs.totals", { n: data.totals.requiring_signoff, delivered: data.totals.delivered, met: data.totals.met, gap: data.totals.delivered_not_met, share })}</p>
  {#if data.milestones.length === 0}
    <p>{t("p.signoffs.none")}</p>
  {:else}
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div class="scroll" role="region" aria-label={t("p.signoffs.h1")} tabindex="0">
      <table>
        <caption class="sr-only">{t("p.signoffs.h1")}</caption>
        <thead>
          <tr>
            <th scope="col">{t("p.signoffs.col_milestone")}</th>
            <th scope="col">{t("p.signoffs.col_due")}</th>
            <th scope="col">{t("p.signoffs.col_state")}</th>
            <th scope="col">{t("p.signoffs.col_delivered")}</th>
            <th scope="col">{t("p.signoffs.col_met")}</th>
            <th scope="col">{t("p.signoffs.col_outstanding")}</th>
            <th scope="col">{t("p.signoffs.col_act")}</th>
          </tr>
        </thead>
        <tbody>
          {#each data.milestones as m (m.milestone_pid)}
            <tr>
              <th scope="row">{m.name}</th>
              <td>{m.due?.slice(0, 10) ?? ""}</td>
              <td class={`so-${m.signoff.state}`}><span aria-hidden="true">{SYM[m.signoff.state] ?? "–"}</span> {t(`p.signoffs.state_${m.signoff.state === "not_required" ? "met" : m.signoff.state}` as never)}</td>
              <td>{m.signoff.delivered_on ?? ""}</td>
              <td>{m.signoff.met_on ?? ""}</td>
              <td>{m.signoff.outstanding.join(", ")}</td>
              <td class="act">
                <label>{t("p.signoffs.organization")}
                  <select bind:value={pick[m.milestone_pid]}>{#each orgs as o (o.pid)}<option value={o.pid}>{o.name}</option>{/each}</select>
                </label>
                <label>{t("p.signoffs.reason")} <input bind:value={reason[m.milestone_pid]} maxlength="500" /></label>
                <button class="button" type="button" disabled={busy || orgs.length === 0} aria-label={t("p.signoffs.sign_for", { name: m.name })} onclick={() => void act(m, "sign")}>{t("p.signoffs.sign")}</button>
                <button class="button" type="button" disabled={busy || orgs.length === 0} aria-label={t("p.signoffs.withdraw_for", { name: m.name })} onclick={() => void act(m, "withdraw")}>{t("p.signoffs.withdraw")}</button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
{/if}

<style>
  .scroll { overflow-x: auto; margin-block: 1rem; }
  .act { display: flex; flex-wrap: wrap; gap: 0.5rem; align-items: end; }
  .so-delivered_not_met { background: #fde8e8; }
  .so-met { background: #e3f4e3; }
</style>
