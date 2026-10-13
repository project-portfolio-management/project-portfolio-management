<!--
  Deployment moves (`/plans/[pid]/deployment-moves`, EX-5): requests to move a task's start, the knock-on on
  other tasks and its labour cost, and the approver's decision. Nothing moves until it is approved.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { PpmClient, type DeploymentMoves, type Task } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let data = $state<DeploymentMoves | null>(null);
  let tasks = $state<Task[]>([]);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let form = $state({ task_pid: "", new_start: "", reason: "", approver_ref: "" });
  let why = $state<Record<string, string>>({});

  async function load() {
    try {
      data = await ppm.deploymentMoves(pid);
      tasks = (await ppm.listTasks(pid)).tasks.filter((x) => x.status !== "done");
      if (!form.task_pid && tasks[0]) form.task_pid = tasks[0].pid;
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.moves.could_not_load");
    }
  }
  onMount(() => void load());

  async function run(job: () => Promise<unknown>) {
    busy = true;
    try {
      await job();
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.moves.could_not_save");
    } finally {
      busy = false;
    }
  }
  const request = () =>
    run(() => ppm.requestDeploymentMove(pid, {
      task_pid: form.task_pid, new_start: form.new_start, reason: form.reason.trim(),
      ...(form.approver_ref.trim() ? { approver_ref: form.approver_ref.trim() } : {}),
    }));
  const decide = (mp: string, d: "approve" | "refuse") => run(() => ppm.decideDeploymentMove(pid, mp, d, (why[mp] ?? "").trim()));
  const money = (minor: number) => `${minor < 0 ? "-" : minor > 0 ? "+" : ""}${Math.floor(Math.abs(minor) / 100)}.${String(Math.abs(minor) % 100).padStart(2, "0")}`;
  const cost = (c: { currency: string; delta_minor: number }[] | undefined) =>
    c && c.length ? c.map((x) => `${money(x.delta_minor)} ${x.currency}`).join(", ") : t("p.moves.no_cost");
  const title = (taskPid: string) => tasks.find((x) => x.pid === taskPid)?.title ?? taskPid;
</script>

<svelte:head><title>{t("p.moves.title")}</title></svelte:head>

<h1>{t("p.moves.h1")}</h1>
<p class="lede">{t("p.moves.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !data && !error}<p class="muted">{t("p.moves.loading")}</p>{/if}

{#if data}
  <section aria-labelledby="mv-req">
    <h2 id="mv-req">{t("p.moves.request_h2")}</h2>
    <form onsubmit={(e) => { e.preventDefault(); if (form.task_pid && form.new_start && form.reason.trim()) void request(); }}>
      <label>{t("p.moves.task")}
        <select bind:value={form.task_pid}>{#each tasks as x (x.pid)}<option value={x.pid}>{x.title}</option>{/each}</select>
      </label>
      <label>{t("p.moves.new_start")} <input type="date" bind:value={form.new_start} required /></label>
      <label>{t("p.moves.reason")} <input bind:value={form.reason} required maxlength="500" /></label>
      <label>{t("p.moves.approver")} <input bind:value={form.approver_ref} maxlength="200" /></label>
      <button class="button" type="submit" disabled={busy || !form.task_pid || !form.new_start || !form.reason.trim()}>{t("p.moves.request")}</button>
    </form>
  </section>

  {#if data.moves.length === 0}
    <p>{t("p.moves.none")}</p>
  {:else}
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div class="scroll" role="region" aria-label={t("p.moves.h1")} tabindex="0">
      <table>
        <caption class="sr-only">{t("p.moves.h1")}</caption>
        <thead>
          <tr>
            <th scope="col">{t("p.moves.col_task")}</th>
            <th scope="col">{t("p.moves.col_dates")}</th>
            <th scope="col">{t("p.moves.col_reason")}</th>
            <th scope="col">{t("p.moves.col_by")}</th>
            <th scope="col">{t("p.moves.col_approver")}</th>
            <th scope="col">{t("p.moves.col_status")}</th>
            <th scope="col">{t("p.moves.col_knock")}</th>
            <th scope="col">{t("p.moves.col_act")}</th>
          </tr>
        </thead>
        <tbody>
          {#each data.moves as m (m.pid)}
            <tr>
              <th scope="row">{title(m.task_pid)}</th>
              <td>{m.from_start} → {m.to_start}</td>
              <td>{m.reason}</td>
              <td>{m.requested_by ?? ""}</td>
              <td>{m.approver_ref}</td>
              <td class={`mv-${m.status}`}>
                <span aria-hidden="true">{m.status === "approved" ? "●" : m.status === "refused" ? "✖" : "◆"}</span> {t(`p.moves.status_${m.status}` as never)}
                {#if m.decided_on}<br /><small>{t("p.moves.decided", { by: m.decided_by ?? "", on: m.decided_on.slice(0, 10), why: m.decision_reason ?? "" })}</small>{/if}
              </td>
              <td>{t("p.moves.knock", { n: m.knock_on.length, cost: cost(m.knock_on_cost.labour) })}</td>
              <td>
                {#if m.status === "requested"}
                  <label>{t("p.moves.decision_reason")} <input bind:value={why[m.pid]} maxlength="500" /></label>
                  <button class="button" type="button" disabled={busy} aria-label={t("p.moves.approve_for", { name: title(m.task_pid) })} onclick={() => void decide(m.pid, "approve")}>{t("p.moves.approve")}</button>
                  <button class="button" type="button" disabled={busy} aria-label={t("p.moves.refuse_for", { name: title(m.task_pid) })} onclick={() => void decide(m.pid, "refuse")}>{t("p.moves.refuse")}</button>
                {/if}
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    {#if data.cost_by_requester.length > 0}
      <h2>{t("p.moves.by_requester_h2")}</h2>
      <ul>
        {#each data.cost_by_requester as r (r.requested_by)}
          <li>{t("p.moves.requester_row", { who: r.requested_by, n: r.requests, cost: cost(r.knock_on_labour) })}</li>
        {/each}
      </ul>
    {/if}
  {/if}
{/if}

<style>
  .scroll { overflow-x: auto; margin-block: 1rem; }
  form { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: end; }
  .mv-requested { background: #fff4d6; }
  .mv-approved { background: #e3f4e3; }
  .mv-refused { background: #fde8e8; }
</style>
