<!--
  Many tasks at once on the board (`/plans/[pid]/board`, NC-1 slice 12 part j, RM-9): the chosen tasks' assignee, due
  date and start date each left, set or cleared (`POST /api/tasks/bulk`); the chosen tasks copied or moved to a plan
  (`POST /api/tasks/copy`, `/move`; the plans from `GET /api/plans`, read on opening, the list disabled until read; a
  move asked first); the chosen tasks deleted after a question (`DELETE /api/plans/{pid}/tasks/{t_pid}`, one call
  each). Every rule said first and checked first, every problem at once; nothing sent for no change; the answer in
  the page's words (the tasks refused named, the service's reasons not shown). Rules in `src/lib/board-bulk-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatInteger } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import { PpmClient, type BulkResult, type Task } from "#lib/api/ppm.js";
  import {
    BULK_MAX,
    START_MAX,
    START_MIN,
    bulkEditBody,
    bulkOutcome,
    bulkRefusalKey,
    deleteGone,
    deleteProblem,
    emptyBulkEditForm,
    transferBody,
    type BulkEditForm,
    type TransferMode,
  } from "#lib/board-bulk-view.js";

  let { pid, tasks, onchanged }: { pid: string; tasks: Task[]; onchanged: () => void } = $props();

  const FIELDS = [
    ["assignee", "p.bulk.f_assignee", "text"],
    ["due", "p.bulk.f_due", "date"],
    ["start", "p.bulk.f_start", "date"],
  ] as const;

  const ppm = PpmClient.withFetch();
  const planRepo = PlanRepository.withFetch();
  let chosen = $state<string[]>([]);
  let form = $state<BulkEditForm>(emptyBulkEditForm());
  let plans = $state<PlanRef[] | null>(null);
  let toPlan = $state("");
  let keepAssignee = $state(false);
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  // Only tasks still on the board stay chosen.
  const live = $derived(chosen.filter((c) => tasks.some((x) => x.pid === c)));
  const titleOf = (p: string) => tasks.find((x) => x.pid === p)?.title;
  const planName = (p: string) => plans?.find((x) => x.pid === p)?.name ?? p.slice(0, 8);

  async function opened(e: Event) {
    if (!(e.currentTarget as HTMLDetailsElement).open || plans !== null) return;
    try {
      plans = await planRepo.list();
    } catch (err) {
      problem = errorText(err, "p.bulk.could_not_load_plans");
    }
  }

  function toggle(p: string, on: boolean) {
    chosen = on ? [...new Set([...chosen, p])] : chosen.filter((c) => c !== p);
  }

  function say(mode: "edit" | TransferMode, result: BulkResult) {
    const o = bulkOutcome(mode, result, titleOf);
    const parts = [tn("p.bulk.done", o.requested, { done: formatInteger(o.done), requested: formatInteger(o.requested) })];
    if (o.mapped > 0) parts.push(tn("p.bulk.mapped", o.mapped, { n: formatInteger(o.mapped) }));
    notice = parts.join(" ");
    problem = o.refused.length > 0 ? t("p.bulk.refused_titles", { titles: o.refused.join(", ") }) : null;
  }

  async function run(mode: "edit" | TransferMode, send: () => Promise<BulkResult>) {
    busy = true;
    try {
      const result = await send();
      say(mode, result);
      chosen = [];
      onchanged();
    } catch (err) {
      const key = bulkRefusalKey(mode, err);
      problem = key ? t(key) : errorText(err, "p.bulk.could_not");
    } finally {
      busy = false;
    }
  }

  async function edit(e: SubmitEvent) {
    e.preventDefault();
    notice = "";
    const r = bulkEditBody(live, form, tasks);
    if (r === null) {
      problem = null;
      notice = t("p.bulk.unchanged");
      return;
    }
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k, { max: formatInteger(BULK_MAX), min: START_MIN, latest: START_MAX })).join(" ");
      return;
    }
    problem = null;
    await run("edit", () => ppm.bulkEditTasks(r.body));
    form = emptyBulkEditForm();
  }

  async function transfer(mode: TransferMode) {
    notice = "";
    const r = transferBody(mode, live, toPlan, pid, keepAssignee);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k, { max: formatInteger(BULK_MAX) })).join(" ");
      return;
    }
    problem = null;
    const n = r.body.ids.length;
    if (mode === "move" && !(await askConfirm(tn("p.bulk.confirm_move", n, { n: formatInteger(n), plan: planName(toPlan) })))) return;
    await run(mode, () => (mode === "copy" ? ppm.copyTasks(r.body) : ppm.moveTasks(r.body)));
  }

  async function remove() {
    notice = "";
    const p = deleteProblem(live);
    if (p) {
      problem = t(p);
      return;
    }
    problem = null;
    const ids = [...live];
    if (!(await askConfirm(tn("p.bulk.confirm_delete", ids.length, { n: formatInteger(ids.length) })))) return;
    busy = true;
    let deleted = 0;
    let gone = 0;
    const failed: string[] = [];
    for (const id of ids) {
      try {
        await ppm.deleteTask(pid, id);
        deleted += 1;
      } catch (err) {
        if (deleteGone(err)) gone += 1;
        else failed.push(titleOf(id) ?? id.slice(0, 8));
      }
    }
    busy = false;
    const parts = [tn("p.bulk.deleted", deleted, { n: formatInteger(deleted) })];
    if (gone > 0) parts.push(tn("p.bulk.deleted_gone", gone, { n: formatInteger(gone) }));
    notice = parts.join(" ");
    problem = failed.length > 0 ? t("p.bulk.delete_failed", { titles: failed.join(", ") }) : null;
    chosen = [];
    onchanged();
  }
</script>

<details class="surface" data-testid="board-bulk" ontoggle={opened}>
  <summary>{t("p.bulk.heading")}</summary>
  <p class="small muted">{t("p.bulk.says", { max: formatInteger(BULK_MAX), min: START_MIN, latest: START_MAX })}</p>

  <fieldset>
    <legend>{tn("p.bulk.chosen", live.length, { n: formatInteger(live.length) })}</legend>
    <div class="row">
      <button class="button small" type="button" onclick={() => (chosen = tasks.map((x) => x.pid))}>{t("p.bulk.choose_all")}</button>
      <button class="button small" type="button" onclick={() => (chosen = [])}>{t("p.bulk.choose_none")}</button>
    </div>
    <ul class="picks">
      {#each tasks as task (task.pid)}
        <li>
          <label class="check">
            <input type="checkbox" checked={live.includes(task.pid)} onchange={(e) => toggle(task.pid, (e.currentTarget as HTMLInputElement).checked)} />
            {task.title}
          </label>
        </li>
      {:else}
        <li class="muted">{t("p.bulk.no_tasks")}</li>
      {/each}
    </ul>
  </fieldset>

  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  <p class="small" role="status">{notice}</p>

  <form class="stack" novalidate onsubmit={edit} aria-label={t("p.bulk.edit_form")}>
    {#each FIELDS as [field, key, type] (field)}
      <div class="row">
        <label>{t(key)}
          <select bind:value={form[field].mode}>
            <option value="keep">{t("p.bulk.mode_keep")}</option>
            <option value="set">{t("p.bulk.mode_set")}</option>
            <option value="clear">{t("p.bulk.mode_clear")}</option>
          </select>
        </label>
        <label>{t("p.bulk.f_value", { field: t(key) })}
          <input {type} bind:value={form[field].value} disabled={form[field].mode !== "set"} />
        </label>
      </div>
    {/each}
    <div><button class="button" type="submit" disabled={busy}>{t("p.bulk.apply")}</button></div>
  </form>

  <form class="stack" novalidate onsubmit={(e) => e.preventDefault()} aria-label={t("p.bulk.transfer_form")}>
    <label>{t("p.bulk.f_plan")}
      <select bind:value={toPlan} disabled={plans === null}>
        <option value="">{t("p.bulk.choose_plan")}</option>
        {#each plans ?? [] as p (p.pid)}<option value={p.pid}>{p.pid === pid ? t("p.bulk.this_plan", { name: p.name }) : p.name}</option>{/each}
      </select>
    </label>
    <label class="check"><input type="checkbox" bind:checked={keepAssignee} /> {t("p.bulk.keep_assignee")}</label>
    <div class="row">
      <button class="button" type="button" disabled={busy || plans === null} onclick={() => void transfer("copy")}>{t("p.bulk.copy")}</button>
      <button class="button" type="button" disabled={busy || plans === null} onclick={() => void transfer("move")}>{t("p.bulk.move")}</button>
    </div>
  </form>

  <div class="row">
    <button class="button" type="button" disabled={busy} onclick={() => void remove()}>{t("p.bulk.delete")}</button>
  </div>
</details>

<style>
  details { margin-block: 1rem; }
  .row { display: flex; gap: 0.75rem; align-items: end; flex-wrap: wrap; margin-block: 0.5rem; }
  .row label, .stack > label { display: grid; gap: 0.2rem; }
  .check { display: flex; gap: 0.5rem; align-items: center; }
  .picks { list-style: none; padding: 0; max-block-size: 16rem; overflow-y: auto; }
  .picks li { padding-block: 0.15rem; }
  form { margin-block: 1rem; }
  fieldset { border: 1px solid var(--mxi-color-border, #ccc); padding: 0.5rem 0.75rem; }
</style>
