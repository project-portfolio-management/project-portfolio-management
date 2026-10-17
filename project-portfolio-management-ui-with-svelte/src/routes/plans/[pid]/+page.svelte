<!--
  Plan detail route (`/plans/[pid]`).

  Loads one plan, renders its fields, and offers edit / delete /
  check-duplicates (kind does not gate matching).
-->
<script lang="ts">
  import { display } from "#lib/preferences.svelte.js";
  import { effectiveSimple, splitPlanLinks } from "#lib/preferences.js";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { ScoredRef, Plan } from "#lib/api/types.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { PpmClient } from "#lib/api/ppm.js";
  import { visiblePlanLinks } from "#lib/modules.js";

  const pid = page.params.pid ?? "";
  const repo = PlanRepository.withFetch();

  let record = $state<Plan | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);
  let duplicates = $state<ScoredRef[] | null>(null);
  let checking = $state(false);
  // RM-7: the modules this plan exposes; null = unknown ⇒ show every link.
  let modules = $state<string[] | null>(null);
  const links = $derived(visiblePlanLinks(modules));
  // Simple view keeps governance, board and schedule in front and the rest behind
  // "More tools" — prominence, not capability: the same links, one step away.
  const simple = $derived(effectiveSimple(display.prefs, page.data?.view));
  const split = $derived(splitPlanLinks(links, simple));
  const front = $derived(split.front);
  const more = $derived(split.more);
  const LINK_LABELS: Record<string, string> = {
    get flow() { return t("p.plans.pid.flow.flow"); },
    get "critical-path"() { return t("p.plans.pid.what_if.critical_path"); },
    get "earned-value"() { return t("p.plans.pid.earned_value.earned_value"); },
    raci: "RACI",
    get calendar() { return t("c.CpmTable.calendar"); },
    get "schedule-health"() { return t("p.plans.pid.schedule_health.schedule_health"); },
    get leveling() { return t("p.plans.pid.leveling"); },
    get "critical-chain"() { return t("p.critical_chain.link_chain"); },
    get buffers() { return t("p.critical_chain.link_buffers"); },
    get "deployment-moves"() { return t("p.moves.link"); },
    get cutover() { return t("p.cutover.link"); },
    get "schedule-risk"() { return t("p.plans.pid.schedule_risk.schedule_risk"); },
    get "cost-risk"() { return t("p.plans.pid.joint_risk.cost_risk"); },
    get "joint-risk"() { return t("p.plans.pid.cost_and_schedule_risk"); },
    get work() { return t("p.plans.pid.work.work_and_assignments"); },
    get "what-if"() { return t("p.plans.pid.what_if.what_if"); },
    get onboarding() { return t("p.plans.pid.onboarding.onboarding_and_offboarding"); },
    get handover() { return t("p.plans.pid.handover.handover"); },
    get wbs() { return t("p.plans.pid.wbs.work_breakdown"); },
    get import() { return t("p.plans.pid.import.import"); },
    get costs() { return t("p.plans.pid.work.costs"); },
  };
  const linkLabel = (k: string): string =>
    k === "governance" ? t("ppm.common.governance")
    : k === "board" ? t("ppm.common.board")
    : k === "schedule" ? t("ppm.common.schedule")
    : (LINK_LABELS[k] ?? k);

  onMount(async () => {
    try {
      record = await repo.get(pid);
      // Best effort: a failure leaves `modules` null and every link shown.
      PpmClient.withFetch()
        .planModules(pid)
        .then((m) => (modules = m.modules))
        .catch(() => {});
    } catch (err) {
      error = err instanceof Error ? err.message : t("detail.notFound");
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    await repo.remove(pid);
    await goto(localeHref(`/plans`));
  }

  async function handleCheckDuplicates() {
    if (!record) return;
    checking = true;
    try {
      const hits = await repo.checkDuplicates(record);
      duplicates = hits.filter((h: ScoredRef) => h.pid !== pid);
    } catch (err) {
      error = err instanceof Error ? err.message : t("detail.checkFailed");
    } finally {
      checking = false;
    }
  }
</script>

<svelte:head><title>{t("p.plans.pid.page_title", { name: record?.name ?? t("p.plans.pid.plan_fallback") })}</title></svelte:head>

{#if loading}
  <p>{t("detail.loading")}</p>
{:else if error}
  <p class="banner" role="alert">{error}</p>
{:else if record}
  <h1>{record.name}</h1>
  <p class="small tools">
    {#each front as k (k)}
      <a class="button small" href={localeHref(`/plans/${pid}/${k}`)}>{linkLabel(k)}</a>
    {/each}
    {#if more.length > 0}
      <details class="more-tools">
        <summary>{t("p.plans.pid.more_tools")}</summary>
        <span class="tools">
          {#each more as k (k)}
            <a class="button small" href={localeHref(`/plans/${pid}/${k}`)}>{linkLabel(k)}</a>
          {/each}
        </span>
      </details>
    {/if}
  </p>
  <div class="surface stack">
    {#if record.kind}<div><strong>{t("detail.caseType")}</strong> {record.kind}</div>{/if}
    {#if record.status}
      <div>
        <strong>{t("detail.status")}</strong>
        {typeof record.status === "string" ? record.status : record.status.Custom}
      </div>
    {/if}
    {#if record.code}<div><strong>{t("detail.caseNumber")}</strong> <code>{record.code}</code></div>{/if}
    {#if record.owner_org_name}<div><strong>{t("detail.agency")}</strong> {record.owner_org_name}</div>{/if}
    {#if record.parent_ref}<div><strong>{t("detail.opened")}</strong> <code>{record.parent_ref}</code></div>{/if}
    {#if record.keywords && record.keywords.length > 0}
      <div><strong>{t("detail.keywords")}</strong> {record.keywords.join(", ")}</div>
    {/if}
    {#if record.tags && record.tags.length > 0}
      <div><strong>{t("detail.subjects")}</strong> {record.tags.join(", ")}</div>
    {/if}
    {#if record.identifiers && record.identifiers.length > 0}
      <div>
        <strong>{t("detail.identifiers")}</strong>
        <ul>
          {#each record.identifiers as id, i (i)}
            <li>
              {typeof id.scheme === "string" ? id.scheme : t("p.plans.pid.custom_scheme", { name: id.scheme.Custom })}:
              <code>{id.value}</code>
            </li>
          {/each}
        </ul>
      </div>
    {/if}
    <div><strong>{t("detail.id")}</strong> <code>{pid}</code></div>
  </div>

  <div class="row" style="margin-top:1rem">
    <a class="button" href={localeHref(`/plans/${pid}/edit`)}>{t("detail.edit")}</a>
    <button class="button" onclick={handleCheckDuplicates} disabled={checking}>
      {checking ? t("detail.checking") : t("detail.checkDuplicates")}
    </button>
    <button onclick={handleDelete}>{t("detail.delete")}</button>
  </div>

  {#if duplicates}
    <h2>{t("detail.potentialDuplicates")}</h2>
    {#if duplicates.length === 0}
      <p>{t("detail.noneAboveThreshold")}</p>
    {:else}
      <ul class="stack">
        {#each duplicates as dup (dup.pid)}
          <li class="surface row">
            <a href={localeHref(`/plans/${dup.pid}`)}>{dup.name}</a>
            <span>{dup.score.toFixed(3)} · {dup.confidence}</span>
          </li>
        {/each}
      </ul>
    {/if}
  {/if}
{/if}

<style>
  /* The tool links are a wrapping row of boxes. As plain inline links, a long label broken across two lines made one
     box whose right padding could stick out past the phone viewport (a rare 28px horizontal scroll). */
  .tools {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    align-items: center;
  }
  .tools > details {
    flex: 1 1 100%;
  }
  .tools :global(a.button) {
    max-width: 100%;
  }
  .more-tools .tools {
    margin-block-start: 0.4rem;
  }
</style>
