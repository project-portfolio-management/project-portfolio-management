<!--
  A task's links to tasks of other plans (`/plans/[pid]/tasks/[tpid]`, NC-1 slice 12 part i, PJ-13, LT-10): the links
  touching it (`GET /api/dependencies?item=`), each named from the plan's critical path (`GET /api/plans/{pid}/cpm`,
  which carries the linked plans' tasks; best effort, so an end it does not carry is "a task not named here"), with
  the links to tasks the reader cannot read counted, never named; a link added to a task of another plan the reader
  can read (`GET /api/plans`, then that plan's `GET …/tasks`; `POST /api/dependencies`, the default kind and lag not
  sent) and one removed after a question (`DELETE /api/dependencies/{pid}`). Every rule is said and checked first.
  Rules in `src/lib/cross-plan-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tn, localeHref, type StringKey } from "#lib/i18n.svelte.js";
  import { formatInteger } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { PpmClient, type CpmNode, type Task } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { PlanRef } from "#lib/api/types.js";
  import {
    LAG_MAX,
    LINK_KINDS,
    LINK_KIND_KEY,
    crossLinkBody,
    crossLinkRefusalKey,
    emptyCrossLinkForm,
    otherEnd,
    splitRows,
    type CrossLinkForm,
    type FullDependency,
  } from "#lib/cross-plan-view.js";

  let { pid, taskPid, taskTitle }: { pid: string; taskPid: string; taskTitle: string } = $props();

  const ppm = PpmClient.withFetch();
  const planRepo = PlanRepository.withFetch();
  let links = $state<FullDependency[] | null>(null);
  let restricted = $state(0);
  let names = $state<Map<string, CpmNode>>(new Map());
  let plans = $state<PlanRef[] | null>(null);
  let otherTasks = $state<Task[] | null>(null);
  let otherFor = $state("");
  let form = $state<CrossLinkForm>(emptyCrossLinkForm());
  let loadError = $state<string | null>(null);
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);
  // Names of tasks chosen here, kept so a link just made is named.
  let chosenNames = $state<Record<string, { title: string; plan: string }>>({});

  async function loadLinks() {
    try {
      const r = splitRows(await ppm.dependencies(taskPid));
      links = r.links;
      restricted = r.restricted;
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.crossplan.could_not_load");
    }
  }
  async function loadNames() {
    try {
      const cpm = await ppm.planCpm(pid);
      names = new Map(cpm.nodes.map((n) => [n.pid, n]));
    } catch {
      names = new Map();
    }
  }
  async function loadPlans() {
    try {
      plans = (await planRepo.list()).filter((p) => p.pid !== pid);
    } catch (err) {
      loadError = errorText(err, "p.crossplan.could_not_load_plans");
    }
  }
  onMount(() => {
    void loadLinks();
    void loadNames();
    void loadPlans();
  });

  async function choosePlan(planPid: string) {
    otherTasks = null;
    otherFor = planPid;
    form.otherPid = "";
    if (!planPid) return;
    try {
      const ts = (await ppm.listTasks(planPid)).tasks.filter((x) => !x.is_hammock);
      if (otherFor === planPid) otherTasks = ts;
    } catch (err) {
      loadError = errorText(err, "p.crossplan.could_not_load_tasks");
    }
  }

  function endText(p: string): string {
    const n = names.get(p);
    if (n?.external) return t("p.crossplan.end_external", { title: n.title ?? t("p.crossplan.untitled"), plan: n.plan_name ?? "" });
    if (n) return t("p.crossplan.end_own", { title: n.title ?? t("p.crossplan.untitled") });
    const c = chosenNames[p];
    if (c) return t("p.crossplan.end_external", { title: c.title, plan: c.plan });
    return t("p.crossplan.end_unnamed", { ref: p.slice(0, 8) });
  }

  function why(action: "add" | "remove", err: unknown, fallback: StringKey): string {
    const key = crossLinkRefusalKey(action, err);
    return key ? t(key) : errorText(err, fallback);
  }

  async function add(e: SubmitEvent) {
    e.preventDefault();
    notice = "";
    const r = crossLinkBody(form, taskPid, links ?? []);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k, { max: formatInteger(LAG_MAX) })).join(" ");
      return;
    }
    problem = null;
    const other = otherTasks?.find((x) => x.pid === form.otherPid);
    const planName = plans?.find((p) => p.pid === form.planPid)?.name ?? "";
    if (other) chosenNames = { ...chosenNames, [other.pid]: { title: other.title, plan: planName } };
    busy = true;
    try {
      await ppm.addDependency(r.body);
      notice = t("p.crossplan.added", { title: other?.title ?? "" });
      form = { ...emptyCrossLinkForm(), planPid: form.planPid };
      await loadLinks();
    } catch (err) {
      problem = why("add", err, "p.crossplan.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function remove(l: FullDependency) {
    notice = "";
    const end = otherEnd(l, taskPid);
    const words = { task: taskTitle, other: endText(end.pid) };
    if (!(await askConfirm(t(end.thisFirst ? "p.crossplan.confirm_remove_before" : "p.crossplan.confirm_remove_after", words)))) return;
    busy = true;
    problem = null;
    try {
      await ppm.removeDependency(l.pid);
      notice = t("p.crossplan.removed");
    } catch (err) {
      problem = why("remove", err, "p.crossplan.could_not_save");
    } finally {
      busy = false;
      await loadLinks();
    }
  }
</script>

<section aria-labelledby="xlink-h" data-testid="cross-plan-links">
  <h2 id="xlink-h">{t("p.crossplan.task_heading")}</h2>
  <p class="small muted">{t("p.crossplan.task_says", { max: formatInteger(LAG_MAX) })}</p>
  {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  <p class="small" role="status">{notice}</p>
  {#if links}
    {#if links.length === 0}
      <p>{t("p.crossplan.no_links")}</p>
    {:else}
      <ul data-testid="cross-links">
        {#each links as l (l.pid)}
          {@const end = otherEnd(l, taskPid)}
          <li>
            {t(end.thisFirst ? "p.crossplan.this_before" : "p.crossplan.this_after", { other: endText(end.pid) })}
            <span class="small muted">· {t(LINK_KIND_KEY[l.kind])}{l.lag_days !== 0 ? ` · ${t("p.crossplan.lag", { days: l.lag_days })}` : ""}</span>
            <button class="button small" type="button" disabled={busy} onclick={() => void remove(l)} aria-label={t("p.crossplan.remove_named", { other: endText(end.pid) })}>{t("p.crossplan.remove")}</button>
          </li>
        {/each}
      </ul>
    {/if}
    {#if restricted > 0}<p class="small muted">{tn("p.crossplan.restricted", restricted, { n: restricted })}</p>{/if}
  {/if}
  <p class="small"><a href={localeHref(`/plans/${pid}/critical-path`)}>{t("p.crossplan.see_network")}</a></p>

  <form class="stack" novalidate onsubmit={add} aria-label={t("p.crossplan.form")}>
    <fieldset>
      <legend>{t("p.crossplan.f_order")}</legend>
      <label class="inline"><input type="radio" name="xlink-order" value={true} bind:group={form.thisFirst} /> {t("p.crossplan.order_before")}</label>
      <label class="inline"><input type="radio" name="xlink-order" value={false} bind:group={form.thisFirst} /> {t("p.crossplan.order_after")}</label>
    </fieldset>
    <label>{t("p.crossplan.f_plan")}
      <select bind:value={form.planPid} disabled={!plans} onchange={() => void choosePlan(form.planPid)}>
        <option value="">{t("p.crossplan.choose_plan")}</option>
        {#each plans ?? [] as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
      </select>
    </label>
    <label>{t("p.crossplan.f_task")}
      <select bind:value={form.otherPid} disabled={!otherTasks}>
        <option value="">{t("p.crossplan.choose_task")}</option>
        {#each otherTasks ?? [] as x (x.pid)}<option value={x.pid}>{x.title}</option>{/each}
      </select>
    </label>
    <label>{t("p.crossplan.f_kind")}
      <select bind:value={form.kind}>
        {#each LINK_KINDS as k (k)}<option value={k}>{t(LINK_KIND_KEY[k])}</option>{/each}
      </select>
    </label>
    <label>{t("p.crossplan.f_lag")}<input type="number" step="1" min={-LAG_MAX} max={LAG_MAX} bind:value={form.lag} /></label>
    <div><button class="button" type="submit" disabled={busy || !links}>{t("p.crossplan.add")}</button></div>
  </form>
</section>

<style>
  form { max-inline-size: 32rem; margin-block: 1rem; }
  label { display: grid; gap: 0.2rem; }
  label.inline { display: inline-flex; gap: 0.3rem; margin-inline-end: 1rem; }
  fieldset { border: 1px solid var(--border, #ccc); padding: 0.5rem 0.75rem; }
</style>
