<!--
  Buffer management (`/plans/[pid]/buffers`, CC-6). Freeze the chain and buffers as planned, then read each
  buffer against the live schedule: a fever chart per buffer (zone as a word, a shape and a colour) with a
  table that gives the same figures, an alert naming the chain of any buffer in the red, the recovery
  actions people agreed (recorded, closed once), and a history of snapshots. The service reports; a
  person decides.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type BufferManagement, type BufferReading } from "#lib/api/ppm.js";
  import FeverChart from "#lib/components/FeverChart.svelte";
  import { zoneLook } from "#lib/chain-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let data = $state<BufferManagement | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let confirmFreeze = $state(false);
  let form = $state({ buffer_key: "", action: "", owner_ref: "", due_date: "" });

  async function load() {
    try {
      data = await ppm.bufferManagement(pid);
      error = null;
      if (!form.buffer_key && data.readings?.[0]) form.buffer_key = data.readings[0].key;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.critical_chain.could_not_load_buffers");
    }
  }
  onMount(() => void load());

  async function run(fn: () => Promise<unknown>) {
    busy = true;
    try {
      await fn();
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.critical_chain.could_not_save");
    } finally {
      busy = false;
    }
  }

  const label = (r: BufferReading): string =>
    r.kind === "project" ? t("p.critical_chain.project_label") : t("p.critical_chain.feeding_label", { chain: r.chain.join(" → ") });
  const labelOf = $derived(new Map((data?.readings ?? []).map((r) => [r.key, label(r)])));
  const statusText = (s: string): string =>
    s === "done" ? t("p.critical_chain.status_done") : s === "dropped" ? t("p.critical_chain.status_dropped") : t("p.critical_chain.status_open");
  const pct = (n: number | undefined | null): string => (n === undefined || n === null ? "" : String(Math.round(n)));

  async function add(e: SubmitEvent) {
    e.preventDefault();
    await run(async () => {
      await ppm.addRecoveryAction(pid, {
        buffer_key: form.buffer_key,
        action: form.action.trim(),
        owner_ref: form.owner_ref.trim(),
        due_date: form.due_date || undefined,
      });
      form.action = "";
      form.due_date = "";
    });
  }
</script>

<svelte:head><title>{t("p.critical_chain.buffers_ppm")}</title></svelte:head>

<h1>{t("p.critical_chain.buffers_h1")}</h1>
<p class="lede">{t("p.critical_chain.buffers_lede2")}</p>
<p class="small">
  <a href={localeHref(`/plans/${pid}`)}>{t("p.critical_chain.link_plan")}</a> ·
  <a href={localeHref(`/plans/${pid}/critical-chain`)}>{t("p.critical_chain.link_chain")}</a>
</p>
<p class="banner" role="note">{t("p.critical_chain.advice")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !data && !error}<p class="muted">{t("p.critical_chain.loading")}</p>{/if}

{#if data?.state === "no_buffer_plan"}
  <p>{t("p.critical_chain.no_plan")}</p>
  <form onsubmit={(e) => { e.preventDefault(); void run(() => ppm.freezeBufferPlan(pid)); }}>
    <label class="check"><input type="checkbox" bind:checked={confirmFreeze} /> {t("p.critical_chain.freeze_confirm")}</label>
    <button class="button" type="submit" disabled={!confirmFreeze || busy}>{t("p.critical_chain.freeze_button")}</button>
  </form>
{/if}

{#if data?.state === "readings" && data.readings && data.lines}
  <p class="small muted">{t("p.critical_chain.frozen_on", { date: data.buffer_plan?.taken_on ?? "" })}</p>

  <section aria-labelledby="bm-alerts">
    <h2 id="bm-alerts">{t("p.critical_chain.alerts_h2")}</h2>
    {#if (data.alerts ?? []).length === 0}
      <p>{t("p.critical_chain.alerts_none")}</p>
    {:else}
      {#each data.alerts ?? [] as a (a.buffer_key)}
        <p role="alert">
          <span aria-hidden="true">▲</span>
          {#if a.joins}
            {t("p.critical_chain.alert_feeding", { consumed: pct(a.buffer_consumed_pct), chain: a.chain.join(" → "), joins: a.joins })}
          {:else}
            {t("p.critical_chain.alert_project", { consumed: pct(a.buffer_consumed_pct), chain: a.chain.join(" → ") })}
          {/if}
        </p>
      {/each}
    {/if}
    {#if (data.red_without_open_recovery_action ?? []).length > 0}
      <p>{t("p.critical_chain.needs_action", { buffers: (data.red_without_open_recovery_action ?? []).map((k) => labelOf.get(k) ?? k).join("; ") })}</p>
    {/if}
  </section>

  <section aria-labelledby="bm-charts">
    <h2 id="bm-charts">{t("p.critical_chain.charts_h2")}</h2>
    <div class="charts">
      {#each data.readings as r, i (r.key)}
        <FeverChart reading={r} lines={data.lines} label={label(r)} id={`fever-${i}`} />
      {/each}
    </div>
  </section>

  <section aria-labelledby="bm-table">
    <h2 id="bm-table">{t("p.critical_chain.table_h2")}</h2>
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div class="scroll" role="region" aria-label={t("p.critical_chain.table_h2")} tabindex="0">
      <table>
        <caption class="sr-only">{t("p.critical_chain.table_h2")}</caption>
        <thead>
          <tr>
            <th scope="col">{t("p.critical_chain.col_buffer_name")}</th>
            <th scope="col">{t("p.critical_chain.col_size")}</th>
            <th scope="col">{t("p.critical_chain.col_done")}</th>
            <th scope="col">{t("p.critical_chain.col_used")}</th>
            <th scope="col">{t("p.critical_chain.col_slip")}</th>
            <th scope="col">{t("p.critical_chain.col_zone")}</th>
          </tr>
        </thead>
        <tbody>
          {#each data.readings as r (r.key)}
            {@const look = zoneLook(r.point?.zone)}
            <tr>
              <th scope="row">{label(r)}</th>
              <td>{r.buffer_days ?? ""}</td>
              <td>{r.point ? pct(r.point.chain_completed_pct) : ""}</td>
              <td>{r.point ? pct(r.point.buffer_consumed_pct) : ""}</td>
              <td>{r.slip_days ?? ""}</td>
              <td class={`tone-${look.tone}`}>
                <span aria-hidden="true">{look.symbol}</span> {t(look.word as never)}
                {#if !r.point && r.no_point_reason}<span class="small"> — {r.no_point_reason}</span>{/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </section>

  <section aria-labelledby="bm-actions">
    <h2 id="bm-actions">{t("p.critical_chain.actions_h2")}</h2>
    {#if (data.recovery_actions ?? []).length === 0}
      <p>{t("p.critical_chain.actions_none")}</p>
    {:else}
      <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
      <div class="scroll" role="region" aria-label={t("p.critical_chain.actions_h2")} tabindex="0">
        <table>
          <caption class="sr-only">{t("p.critical_chain.actions_h2")}</caption>
          <thead>
            <tr>
              <th scope="col">{t("p.critical_chain.col_buffer_name")}</th>
              <th scope="col">{t("p.critical_chain.col_action")}</th>
              <th scope="col">{t("p.critical_chain.col_owner")}</th>
              <th scope="col">{t("p.critical_chain.col_due")}</th>
              <th scope="col">{t("p.critical_chain.col_status")}</th>
            </tr>
          </thead>
          <tbody>
            {#each data.recovery_actions ?? [] as a (a.pid)}
              <tr>
                <th scope="row">{labelOf.get(a.buffer_key) ?? a.buffer_key}</th>
                <td>{a.action}</td>
                <td>{a.owner_ref ?? (a.owner ? `${a.owner} ${t("p.critical_chain.hidden")}` : "")}</td>
                <td>{a.due_date ?? ""}</td>
                <td>
                  {statusText(a.status)}
                  {#if a.status === "open"}
                    <button class="button small" type="button" disabled={busy} aria-label={`${t("p.critical_chain.mark_done")}: ${a.action}`} onclick={() => void run(() => ppm.closeRecoveryAction(pid, a.pid, "done"))}>{t("p.critical_chain.mark_done")}</button>
                    <button class="button small" type="button" disabled={busy} aria-label={`${t("p.critical_chain.drop_action")}: ${a.action}`} onclick={() => void run(() => ppm.closeRecoveryAction(pid, a.pid, "dropped"))}>{t("p.critical_chain.drop_action")}</button>
                  {/if}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}

    <h3>{t("p.critical_chain.add_action_h3")}</h3>
    <form onsubmit={add}>
      <p>
        <label>{t("p.critical_chain.f_buffer")}
          <select bind:value={form.buffer_key}>
            {#each data.readings as r (r.key)}<option value={r.key}>{label(r)}</option>{/each}
          </select>
        </label>
      </p>
      <p><label>{t("p.critical_chain.f_action")} <input type="text" bind:value={form.action} required minlength="5" maxlength="2000" /></label></p>
      <p><label>{t("p.critical_chain.f_owner")} <input type="text" bind:value={form.owner_ref} required maxlength="200" /></label></p>
      <p><label>{t("p.critical_chain.f_due")} <input type="date" bind:value={form.due_date} /></label></p>
      <button class="button" type="submit" disabled={busy}>{t("p.critical_chain.add_button")}</button>
    </form>
  </section>

  <section aria-labelledby="bm-history">
    <h2 id="bm-history">{t("p.critical_chain.snap_h2")}</h2>
    <button class="button" type="button" disabled={busy} onclick={() => void run(() => ppm.takeBufferSnapshot(pid))}>{t("p.critical_chain.snap_button")}</button>
    {#if (data.snapshots ?? []).length === 0}
      <p>{t("p.critical_chain.snap_none")}</p>
    {:else}
      <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
      <div class="scroll" role="region" aria-label={t("p.critical_chain.snap_h2")} tabindex="0">
        <table>
          <caption class="sr-only">{t("p.critical_chain.snap_h2")}</caption>
          <thead>
            <tr>
              <th scope="col">{t("p.critical_chain.col_snap_date")}</th>
              {#each data.readings as r (r.key)}<th scope="col">{label(r)}</th>{/each}
            </tr>
          </thead>
          <tbody>
            {#each data.snapshots ?? [] as s (s.pid)}
              <tr>
                <th scope="row">{s.taken_on}</th>
                {#each data.readings as r (r.key)}
                  {@const look = zoneLook((s.zones[r.key] ?? null) as never)}
                  <td class={`tone-${look.tone}`}><span aria-hidden="true">{look.symbol}</span> {t(look.word as never)}</td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </section>
{/if}

<style>
  .charts { display: flex; flex-wrap: wrap; gap: 1rem; }
  .scroll { overflow-x: auto; margin-bottom: 1rem; }
  /* Pale tints so the text keeps its contrast; the word and the shape carry the meaning. */
  .tone-red { background: #fde8e8; }
  .tone-amber { background: #fff4d6; }
  .tone-green { background: #e3f4e3; }
  .tone-grey { background: #eeeeee; }
</style>
