<!--
  Email deliveries (`/email-deliveries`): one row for each recipient of each email the service tried to send.
  Administrators only (the API answers 404 to anyone else).
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type EmailDeliveries } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let data = $state<EmailDeliveries | null>(null);
  let error = $state<string | null>(null);
  let status = $state("");

  async function load() {
    try {
      data = await ppm.emailDeliveries(status || undefined);
      error = null;
    } catch (err) {
      error = err instanceof Error && /404|not found/i.test(err.message)
        ? t("p.email_deliveries.not_allowed")
        : err instanceof Error ? err.message : t("p.email_deliveries.could_not_load");
    }
  }
  onMount(() => void load());

  const word = (s: string) => t(`p.email_deliveries.status_${s}` as never);
</script>

<svelte:head><title>{t("p.email_deliveries.title")}</title></svelte:head>

<h1>{t("p.email_deliveries.h1")}</h1>
<p class="lede">{t("p.email_deliveries.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !data && !error}<p class="muted">{t("p.email_deliveries.loading")}</p>{/if}

{#if data}
  <p>{t("p.email_deliveries.counts", data.counts)}</p>
  <label>{t("p.email_deliveries.filter")}
    <select bind:value={status} onchange={() => void load()}>
      <option value="">{t("p.email_deliveries.all")}</option>
      {#each ["queued", "retry", "sent", "failed"] as s (s)}<option value={s}>{word(s)}</option>{/each}
    </select>
  </label>
  {#if data.deliveries.length === 0}
    <p>{t("p.email_deliveries.none")}</p>
  {:else}
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div class="scroll" role="region" aria-label={t("p.email_deliveries.h1")} tabindex="0">
      <table>
        <caption class="sr-only">{t("p.email_deliveries.h1")}</caption>
        <thead>
          <tr>
            <th scope="col">{t("p.email_deliveries.col_recipient")}</th>
            <th scope="col">{t("p.email_deliveries.col_subject")}</th>
            <th scope="col">{t("p.email_deliveries.col_status")}</th>
            <th scope="col">{t("p.email_deliveries.col_attempts")}</th>
            <th scope="col">{t("p.email_deliveries.col_error")}</th>
            <th scope="col">{t("p.email_deliveries.col_next")}</th>
            <th scope="col">{t("p.email_deliveries.col_sent")}</th>
          </tr>
        </thead>
        <tbody>
          {#each data.deliveries as d (d.pid)}
            <tr>
              <th scope="row">{d.recipient}</th>
              <td>{d.subject}</td>
              <td class={`st-${d.status}`}><span aria-hidden="true">{d.status === "sent" ? "●" : d.status === "failed" ? "✖" : "◆"}</span> {word(d.status)}</td>
              <td>{d.attempts}</td>
              <td>{d.last_error ?? ""}</td>
              <td>{d.next_attempt_at?.slice(0, 16).replace("T", " ") ?? ""}</td>
              <td>{d.sent_at?.slice(0, 16).replace("T", " ") ?? ""}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
{/if}

<style>
  .scroll { overflow-x: auto; margin-block: 1rem; }
  .st-failed { background: #fde8e8; }
  .st-retry, .st-queued { background: #fff4d6; }
  .st-sent { background: #e3f4e3; }
</style>
