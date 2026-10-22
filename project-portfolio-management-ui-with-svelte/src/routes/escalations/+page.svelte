<!--
  Escalations (`/escalations`): every escalation on the plans you can read, newest first, with a filter for the ones
  still awaiting an answer and for the overdue ones. Each state is a word and a shape; overdue is said in words.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type EscalationRegister } from "#lib/api/ppm.js";
  import { STATUS_SYMBOL, STATUS_TEXT, ageText, automaticText, levelText, reasonLabel } from "#lib/escalation-view.js";

  const ppm = PpmClient.withFetch();
  let register = $state<EscalationRegister | null>(null);
  let error = $state<string | null>(null);
  let filter = $state("live");

  async function refresh() {
    try {
      register = await ppm.escalationRegister({
        status: filter === "overdue" || filter === "all" ? undefined : filter,
        overdue: filter === "overdue",
      });
      error = null;
    } catch (err) {
      error = errorText(err, "p.escalations.could_not_load");
    }
  }
  onMount(refresh);
</script>

<svelte:head><title>{t("p.escalations.escalations_ppm")}</title></svelte:head>

<h1>{t("p.escalations.escalations")}</h1>
<p class="small muted">{t("p.escalations.register_intro")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

<form class="filter" onsubmit={(e) => { e.preventDefault(); void refresh(); }}>
  <label>{t("p.escalations.filter")}
    <select bind:value={filter} onchange={refresh}>
      <option value="live">{t("p.escalations.live")}</option>
      <option value="overdue">{t("l.escalation.overdue")}</option>
      <option value="all">{t("p.escalations.all")}</option>
      <option value="resolved">{STATUS_TEXT.resolved}</option>
      <option value="withdrawn">{STATUS_TEXT.withdrawn}</option>
    </select>
  </label>
</form>

{#if register}
  {#if register.overdue > 0}<p class="small" role="status">{t("p.escalations.overdue_count", { n: register.overdue })}</p>{/if}
  {#if register.escalations.length === 0}
    <p>{filter === "all" ? t("p.escalations.none") : t("p.escalations.none_match")}</p>
  {:else}
    <table>
      <caption class="sr-only">{t("p.escalations.escalations")}</caption>
      <thead>
        <tr>
          <th scope="col">{t("p.escalations.f_title")}</th>
          <th scope="col">{t("p.escalations.plan")}</th>
          <th scope="col">{t("p.escalations.status")}</th>
          <th scope="col">{t("p.escalations.level_col")}</th>
          <th scope="col">{t("p.escalations.age")}</th>
        </tr>
      </thead>
      <tbody>
        {#each register.escalations as e (e.pid)}
          <tr>
            <th scope="row"><a href={localeHref(`/plans/${e.plan_pid}/escalations`)}>{e.title}</a> <span class="small muted">({reasonLabel(e.reason_kind)}{automaticText(e) ? `, ${automaticText(e)}` : ""})</span></th>
            <td>{e.plan_name ?? ""}</td>
            <td class={`esc esc-${e.status}`}><span aria-hidden="true">{STATUS_SYMBOL[e.status]}</span> {STATUS_TEXT[e.status]}</td>
            <td>{levelText(e)}</td>
            <td>{ageText(e)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
{:else if !error}
  <p class="muted">{t("p.escalations.loading")}</p>
{/if}

<style>
  .filter { display: flex; gap: 1rem; align-items: end; margin: 1rem 0; }
  table { border-collapse: collapse; inline-size: 100%; }
  th, td { text-align: start; padding: 0.4rem 0.6rem; border-block-end: 1px solid currentColor; }
  .esc { border-inline-start: 0.4rem solid #5a5a5a; }
  .esc-open { border-inline-start-color: #b00020; }
  .esc-acknowledged { border-inline-start-color: #c77700; }
  .esc-resolved { border-inline-start-color: #1b7a3a; }
  .esc-withdrawn { border-inline-start-style: dashed; }
  .sr-only { position: absolute; inline-size: 1px; block-size: 1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap; }
</style>
