<!--
  The start queue (`/start-queue`, EX-8): programmes refused a start, in the order they asked, and why
  (the work-in-progress limit, or a constraint pool short of supply). The order never changes.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type StartQueue, type StartRefusal } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let queue = $state<StartQueue | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);

  async function load() {
    try {
      queue = await ppm.startQueue();
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.start_queue.could_not_load");
    }
  }
  onMount(() => void load());

  async function withdraw(planPid: string) {
    busy = true;
    try {
      await ppm.withdrawFromStartQueue(planPid);
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.start_queue.could_not_withdraw");
    } finally {
      busy = false;
    }
  }

  function why(r: StartRefusal): string {
    return r.reason === "limit_reached"
      ? t("p.start_queue.limit", { limit: r.limit, in_delivery: r.in_delivery })
      : t("p.start_queue.pool", { pool: r.pool, months: r.months.join(", ") });
  }
</script>

<svelte:head><title>{t("p.start_queue.title")}</title></svelte:head>

<h1>{t("p.start_queue.h1")}</h1>
<p class="lede">{t("p.start_queue.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !queue && !error}<p class="muted">{t("p.start_queue.loading")}</p>{/if}

{#if queue}
  {#if queue.queue.length === 0}
    <p>{t("p.start_queue.none")}</p>
  {:else}
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div class="scroll" role="region" aria-label={t("p.start_queue.h1")} tabindex="0">
      <table>
        <caption class="sr-only">{t("p.start_queue.h1")}</caption>
        <thead>
          <tr>
            <th scope="col">{t("p.start_queue.col_pos")}</th>
            <th scope="col">{t("p.start_queue.col_prog")}</th>
            <th scope="col">{t("p.start_queue.col_asked")}</th>
            <th scope="col">{t("p.start_queue.col_why")}</th>
            <th scope="col">{t("p.start_queue.col_act")}</th>
          </tr>
        </thead>
        <tbody>
          {#each queue.queue as q (q.plan_pid)}
            <tr>
              <td>{q.position}</td>
              <th scope="row">{q.name ?? q.plan_pid}</th>
              <td>{q.queued_on.slice(0, 10)}</td>
              <td>
                <ul>{#each q.refusals as r (r.reason)}<li>{why(r)}</li>{/each}</ul>
              </td>
              <td>
                <button class="button" type="button" disabled={busy} aria-label={t("p.start_queue.withdraw", { name: q.name ?? q.plan_pid })} onclick={() => void withdraw(q.plan_pid)}>{t("p.start_queue.withdraw_short")}</button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
{/if}

<style>
  .scroll { overflow-x: auto; margin-bottom: 1rem; }
  ul { margin: 0; padding-inline-start: 1.1rem; }
</style>
