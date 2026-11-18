<!--
  A placeholder given a person or removed (`/resources`, NC-1 slice 12 part j, SC-7): `POST
  /api/plans/{pid}/allocations/{a_pid}/resolve` (a manager's call; the person's reference checked first; when it would
  put them over capacity the service says so and the reader is asked whether to go ahead, then `confirm: true`) and
  `DELETE /api/plans/{pid}/allocations/{a_pid}` (after a question). Rules in `src/lib/resource-actions-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { formatDate, formatPercentValue } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { PpmClient, type ResourcePlan } from "#lib/api/ppm.js";
  import { isOverCapacity, placeholderRefusalKey, resolveBody } from "#lib/resource-actions-view.js";

  type Placeholder = ResourcePlan["placeholders"][number];
  let {
    placeholders,
    planName,
    onchanged,
  }: { placeholders: Placeholder[]; planName: (pid: string) => string; onchanged: () => void } = $props();

  const ppm = PpmClient.withFetch();
  let chosen = $state("");
  let person = $state("");
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  const placeholder = $derived(placeholders.find((p) => p.allocation_pid === chosen) ?? null);
  const describe = (p: Placeholder) =>
    t("p.placeholder.option", {
      role: p.role ?? t("p.resources.unclassified"),
      plan: planName(p.plan_pid),
      percent: formatPercentValue(p.percent),
      from: p.start_date ? formatDate(p.start_date) : t("p.placeholder.open"),
      to: p.end_date ? formatDate(p.end_date) : t("p.placeholder.open"),
    });

  async function resolve(e: SubmitEvent) {
    e.preventDefault();
    notice = "";
    if (!placeholder) {
      problem = t("p.placeholder.problem_choose");
      return;
    }
    const r = resolveBody(person, false);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k)).join(" ");
      return;
    }
    problem = null;
    const target = placeholder;
    busy = true;
    try {
      try {
        await ppm.resolvePlaceholder(target.plan_pid, target.allocation_pid, r.body);
      } catch (err) {
        if (!isOverCapacity(err)) throw err;
        if (!(await askConfirm(t("p.placeholder.confirm_over", { person: r.body.person_ref })))) {
          notice = t("p.placeholder.not_given");
          return;
        }
        const again = resolveBody(person, true);
        if ("body" in again) await ppm.resolvePlaceholder(target.plan_pid, target.allocation_pid, again.body);
      }
      notice = t("p.placeholder.given", { person: r.body.person_ref });
      chosen = "";
      person = "";
      onchanged();
    } catch (err) {
      const key = placeholderRefusalKey("resolve", err);
      problem = key ? t(key) : errorText(err, "p.placeholder.could_not");
    } finally {
      busy = false;
    }
  }

  async function remove() {
    notice = "";
    if (!placeholder) {
      problem = t("p.placeholder.problem_choose");
      return;
    }
    problem = null;
    const target = placeholder;
    if (!(await askConfirm(t("p.placeholder.confirm_remove", { what: describe(target) })))) return;
    busy = true;
    try {
      await ppm.removeAllocation(target.plan_pid, target.allocation_pid);
      notice = t("p.placeholder.removed");
      chosen = "";
      onchanged();
    } catch (err) {
      const key = placeholderRefusalKey("remove", err);
      problem = key ? t(key) : errorText(err, "p.placeholder.could_not");
    } finally {
      busy = false;
    }
  }
</script>

<section aria-labelledby="ph-h" data-testid="placeholder-actions">
  <h3 id="ph-h">{t("p.placeholder.heading")}</h3>
  <p class="small muted">{t("p.placeholder.says")}</p>
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  <p class="small" role="status">{notice}</p>
  <form class="stack" novalidate onsubmit={resolve} aria-label={t("p.placeholder.form")}>
    <label>{t("p.placeholder.f_placeholder")}
      <select bind:value={chosen}>
        <option value="">{t("p.placeholder.choose")}</option>
        {#each placeholders as p (p.allocation_pid)}<option value={p.allocation_pid}>{describe(p)}</option>{/each}
      </select>
    </label>
    <label>{t("p.placeholder.f_person")}<input type="text" bind:value={person} disabled={!placeholder} /></label>
    <div class="row">
      <button class="button" type="submit" disabled={busy}>{t("p.placeholder.give")}</button>
      <button class="button" type="button" disabled={busy} onclick={() => void remove()}>{t("p.placeholder.remove")}</button>
    </div>
  </form>
</section>

<style>
  form { max-inline-size: 36rem; margin-block: 0.75rem; }
  label { display: grid; gap: 0.2rem; }
  .row { display: flex; gap: 0.75rem; flex-wrap: wrap; }
</style>
