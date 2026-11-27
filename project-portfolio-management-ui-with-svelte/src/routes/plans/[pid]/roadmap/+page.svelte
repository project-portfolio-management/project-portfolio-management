<!--
  A product plan's public roadmap (`/plans/[pid]/roadmap`, RE-1, RE-3; NC-1 slice 12 part l): made for a plan of kind
  Product (`PUT /api/plans/{pid}/roadmap`), its title, address (slug), vision and service description changed (only
  what changed sent), how fresh it is and when it was last reviewed, marked reviewed today (`PUT …/reviewed`), and its
  items in page order by placement, then the unplaced: added, changed, moved one place up or down or to another
  placement, and removed after a question. Every limit is said before sending; a slug another roadmap has or had (409)
  has its own sentence. Publishing or taking it off the public reader goes as one change set (RE-2): previewed first
  (`POST …/publish/preview`: what would change, what conflicts), then published (`POST …/publish`), a conflict meanwhile
  (409) said in the page's words. Gathering several edits into one change set, translations, an item's details and
  organizations, the family and the category are not on this page yet. Rules in `src/lib/roadmap-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { i18n, localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatDate, formatInteger } from "#lib/locale-format.js";
  import { ApiError } from "#lib/api/client.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import { PpmClient, type ProductRoadmap } from "#lib/api/ppm.js";
  import { shownName, type RoadmapCategory } from "#lib/roadmap-category-view.js";
  import {
    PHASE_MAX,
    PLACEMENTS,
    SLUG_MAX,
    TEXT_MAX,
    TITLE_MAX,
    fieldKey,
    groups,
    itemChange,
    publishSet,
    valueWords,
    type ChangeLine,
    type ChangeSet,
    type ConflictLine,
    itemFormOf,
    roadmapChange,
    roadmapFormOf,
    step,
    type ItemForm,
    type Placement,
    type RoadmapForm,
    type RoadmapItem,
  } from "#lib/product-roadmap-view.js";

  const ppm = PpmClient.withFetch();
  const repo = PlanRepository.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let roadmap = $state<ProductRoadmap | null>(null);
  let categories = $state<RoadmapCategory[]>([]);
  let isProduct = $state<boolean | null>(null);
  let loaded = $state(false);
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let problems = $state<string[]>([]);
  let notice = $state<string | null>(null);
  let busy = $state(false);
  let form = $state<RoadmapForm>(roadmapFormOf(null));
  let draft = $state<ItemForm>(itemFormOf(null));
  let draftPlacement = $state<Placement | "">("now");
  let editing = $state<string | null>(null);
  let edit = $state<ItemForm>(itemFormOf(null));
  let moveTo = $state<Record<string, Placement | "">>({});
  let pendingSet = $state<ChangeSet | null>(null);
  let preview = $state<{ changes: ChangeLine[]; conflicts: ConflictLine[]; problems: string[]; would_publish: boolean } | null>(null);

  async function load() {
    try {
      const [r, plan, cats] = await Promise.all([
        ppm.productRoadmap(pid),
        repo.get(pid),
        // The categories only fill the picker; without them the roadmap still reads.
        ppm.roadmapCategories().catch(() => [] as RoadmapCategory[]),
      ]);
      roadmap = r;
      categories = cats;
      isProduct = plan.kind === "Product";
      form = roadmapFormOf(r);
      loaded = true;
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.product_roadmap.could_not_load");
    }
  }
  onMount(load);

  const placementLabel = (p: Placement | null) => tDynamic(`p.product_roadmap.placement.${p ?? "none"}`);

  /** A refusal in the page's words where it has its own sentence. */
  function refused(err: unknown, fallback: "p.product_roadmap.could_not_save" | "p.product_roadmap.could_not_change_item") {
    if (err instanceof ApiError && err.status === 409) error = t("p.product_roadmap.slug_taken");
    else if (err instanceof ApiError && err.status === 404) error = t("p.product_roadmap.gone");
    else error = errorText(err, fallback);
  }

  async function run(work: () => Promise<unknown>, done: string, fallback: "p.product_roadmap.could_not_save" | "p.product_roadmap.could_not_change_item") {
    busy = true;
    error = notice = null;
    try {
      await work();
      notice = done;
      await load();
      return true;
    } catch (err) {
      refused(err, fallback);
      return false;
    } finally {
      busy = false;
    }
  }

  async function saveRoadmap(e: SubmitEvent) {
    e.preventDefault();
    const r = roadmapChange(form, roadmap);
    if (r === null) {
      notice = t("p.product_roadmap.unchanged");
      return;
    }
    if ("problems" in r) {
      problems = r.problems;
      return;
    }
    problems = [];
    await run(
      () => ppm.saveProductRoadmap(pid, r.body),
      roadmap ? t("p.product_roadmap.saved") : t("p.product_roadmap.made"),
      "p.product_roadmap.could_not_save",
    );
  }

  async function addItem(e: SubmitEvent) {
    e.preventDefault();
    const r = itemChange(draft, null, draftPlacement === "" ? null : draftPlacement);
    if (!r) return;
    if ("problems" in r) {
      problems = r.problems;
      return;
    }
    problems = [];
    if (await run(() => ppm.addRoadmapItem(pid, r.body), t("p.product_roadmap.item_added", { title: r.body.title ?? "" }), "p.product_roadmap.could_not_change_item"))
      draft = itemFormOf(null);
  }

  async function saveItem(e: SubmitEvent, i: RoadmapItem) {
    e.preventDefault();
    const r = itemChange(edit, i);
    if (r === null) {
      editing = null;
      return;
    }
    if ("problems" in r) {
      problems = r.problems;
      return;
    }
    problems = [];
    if (await run(() => ppm.changeRoadmapItem(pid, i.pid, r.body), t("p.product_roadmap.item_saved", { title: r.body.title ?? i.title }), "p.product_roadmap.could_not_change_item"))
      editing = null;
  }

  async function move(i: RoadmapItem, to: { placement: Placement | null; index?: number }) {
    await run(
      () => ppm.moveRoadmapItem(pid, i.pid, to),
      t("p.product_roadmap.item_moved", { title: i.title, placement: placementLabel(to.placement) }),
      "p.product_roadmap.could_not_change_item",
    );
  }

  const words = $derived({
    yes: t("p.product_roadmap.yes"),
    no: t("p.product_roadmap.no"),
    none: t("p.product_roadmap.none"),
    placement: (p: string) => placementLabel(p as Placement),
  });
  const titleOf = (pid: string | undefined) => roadmap?.items.find((i) => i.pid === pid)?.title ?? pid ?? "";

  /** A change line in words. */
  function changeText(c: ChangeLine): string {
    if (c.subject === "new_item") return t("p.product_roadmap.new_item_line", { title: c.title ?? "" });
    const params = {
      field: tDynamic(fieldKey(c.field)),
      from: valueWords(c.field, c.from, words),
      to: valueWords(c.field, c.to, words),
    };
    return c.subject === "item"
      ? t("p.product_roadmap.item_change_line", { ...params, title: titleOf(c.item) })
      : t("p.product_roadmap.change_line", params);
  }
  /** A conflict in words. */
  function conflictText(c: ConflictLine): string {
    if (c.gone) return t("p.product_roadmap.conflict_gone", { title: titleOf(c.item) });
    return t("p.product_roadmap.conflict_line", {
      field: tDynamic(fieldKey(c.field)),
      current: valueWords(c.field, c.current, words),
      value: valueWords(c.field, c.value, words),
    });
  }

  /** Preview the change set that publishes the roadmap or takes it off the public reader. */
  async function previewPublish(publish: boolean) {
    if (!roadmap) return;
    error = notice = null;
    const set = publishSet(roadmap.published, publish);
    busy = true;
    try {
      preview = await ppm.previewRoadmapPublish(pid, set);
      pendingSet = set;
    } catch (err) {
      error = errorText(err, "p.product_roadmap.could_not_publish");
    } finally {
      busy = false;
    }
  }

  /** Publish the change set just previewed; a conflict meanwhile refuses it all and is shown. */
  async function publishNow() {
    if (!pendingSet) return;
    busy = true;
    error = notice = null;
    try {
      await ppm.publishRoadmap(pid, pendingSet);
      notice = t("p.product_roadmap.published_done");
      preview = pendingSet = null;
      await load();
    } catch (err) {
      if (err instanceof ApiError && err.status === 409) {
        const body = err.body as { conflicts?: ConflictLine[] } | undefined;
        error = t("p.product_roadmap.conflict_refused");
        preview = { changes: [], conflicts: body?.conflicts ?? [], problems: [], would_publish: false };
        await load();
      } else {
        error = errorText(err, "p.product_roadmap.could_not_publish");
      }
    } finally {
      busy = false;
    }
  }

  async function remove(i: RoadmapItem) {
    if (!(await askConfirm(t("p.product_roadmap.remove_ask", { title: i.title, id: i.public_id })))) return;
    await run(() => ppm.removeRoadmapItem(pid, i.pid), t("p.product_roadmap.item_removed", { title: i.title }), "p.product_roadmap.could_not_change_item");
  }
</script>

{#snippet itemFields(f: ItemForm)}
  <label>{t("p.product_roadmap.f_title")} <input bind:value={f.title} maxlength={TITLE_MAX} /></label>
  <label>{t("p.product_roadmap.f_description")} <textarea rows="2" maxlength={TEXT_MAX} bind:value={f.description}></textarea></label>
  <label>{t("p.product_roadmap.f_outcome")} <textarea rows="2" maxlength={TEXT_MAX} bind:value={f.expected_outcome}></textarea></label>
  <label>{t("p.product_roadmap.f_phase")} <input bind:value={f.phase} maxlength={PHASE_MAX} /></label>
  <label>{t("p.product_roadmap.f_tags")} <input bind:value={f.tags} /></label>
{/snippet}

<svelte:head><title>{t("p.product_roadmap.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.product_roadmap.title")}</h1>
<p class="lede">{t("p.product_roadmap.intro")}</p>
{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{#if error}<p class="banner error" role="alert" data-testid="roadmap-error">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="roadmap-notice">{notice}</p>{/if}
{#if problems.length > 0}
  <ul class="banner error" role="alert" data-testid="roadmap-problems">
    {#each problems as p (p)}<li>{tDynamic(p)}</li>{/each}
  </ul>
{/if}

{#if !loaded && !loadError}
  <p class="muted">{t("p.product_roadmap.loading")}</p>
{:else if loaded && !roadmap && !isProduct}
  <p data-testid="roadmap-not-product">{t("p.product_roadmap.not_product")}</p>
{:else if loaded}
  {#if roadmap}
    <section class="surface stack" aria-labelledby="state-h">
      <h2 id="state-h">{t("p.product_roadmap.state_h")}</h2>
      <p data-testid="roadmap-state">
        {roadmap.published ? t("p.product_roadmap.published") : t("p.product_roadmap.not_published")} ·
        {tDynamic(`p.product_roadmap.fresh.${roadmap.freshness.state}`)}{#if roadmap.reviewed_on} · {t("p.product_roadmap.reviewed_on", { date: formatDate(roadmap.reviewed_on) })}{/if}
      </p>
      <p class="small muted">{t("p.product_roadmap.stale_rule", { days: formatInteger(roadmap.freshness.stale_after_days) })}</p>
      <div class="row">
        <button class="button" type="button" disabled={busy} onclick={() => run(() => ppm.markProductRoadmapReviewed(pid), t("p.product_roadmap.reviewed_done"), "p.product_roadmap.could_not_save")}>{t("p.product_roadmap.mark_reviewed")}</button>
        <button class="button" type="button" disabled={busy} onclick={() => previewPublish(!roadmap!.published)}>{roadmap.published ? t("p.product_roadmap.unpublish") : t("p.product_roadmap.publish")}</button>
      </div>
      {#if preview}
        <section class="surface stack" aria-labelledby="preview-h" data-testid="publish-preview">
          <h3 id="preview-h">{t("p.product_roadmap.preview_h")}</h3>
          {#if preview.changes.length > 0}
            <ul data-testid="preview-changes">{#each preview.changes as c, n (n)}<li>{changeText(c)}</li>{/each}</ul>
          {:else if preview.conflicts.length === 0}
            <p>{t("p.product_roadmap.preview_nothing")}</p>
          {/if}
          {#if preview.conflicts.length > 0}
            <p>{t("p.product_roadmap.preview_conflicts")}</p>
            <ul data-testid="preview-conflicts">{#each preview.conflicts as c, n (n)}<li>{conflictText(c)}</li>{/each}</ul>
          {/if}
          {#if preview.problems.length > 0}
            <p>{t("p.product_roadmap.preview_problems")}</p>
            <ul>{#each preview.problems as p, n (n)}<li>{p}</li>{/each}</ul>
          {/if}
          <div class="row">
            {#if preview.would_publish && preview.changes.length > 0}
              <button class="button" type="button" disabled={busy} onclick={publishNow}>{t("p.product_roadmap.publish_now")}</button>
            {/if}
            <button class="button" type="button" onclick={() => (preview = pendingSet = null)}>{t("p.product_roadmap.cancel")}</button>
          </div>
        </section>
      {/if}
    </section>
  {/if}

  <form class="surface stack" onsubmit={saveRoadmap} aria-labelledby="own-h" novalidate>
    <h2 id="own-h">{roadmap ? t("p.product_roadmap.own_h") : t("p.product_roadmap.make_h")}</h2>
    <label>{t("p.product_roadmap.f_title")} <input bind:value={form.title} maxlength={TITLE_MAX} /></label>
    <label>{roadmap ? t("p.product_roadmap.f_slug") : t("p.product_roadmap.f_slug_new")} <input bind:value={form.slug} maxlength={SLUG_MAX} autocomplete="off" /></label>
    <label>{t("p.product_roadmap.f_vision")} <textarea rows="2" maxlength={TEXT_MAX} bind:value={form.vision}></textarea></label>
    <label>{t("p.product_roadmap.f_service")} <textarea rows="2" maxlength={TEXT_MAX} bind:value={form.service_description}></textarea></label>
    <label
      >{t("p.product_roadmap.f_category")}
      <select bind:value={form.category}>
        <option value="">{t("p.product_roadmap.no_category")}</option>
        {#each categories.filter((c) => c.active || c.pid === roadmap?.category_pid) as c (c.pid)}
          <option value={c.pid}>{shownName(c, i18n.locale)}{c.active ? "" : ` (${t("p.roadmap_categories.is_retired")})`}</option>
        {/each}
      </select></label
    >
    <p class="small muted"><a href={localeHref("/roadmap-categories")}>{t("p.product_roadmap.manage_categories")}</a></p>
    <button class="button" class:primary={!roadmap} type="submit" disabled={busy}>{roadmap ? t("p.product_roadmap.save") : t("p.product_roadmap.make")}</button>
  </form>

  {#if roadmap}
    <section class="stack" aria-labelledby="items-h">
      <h2 id="items-h">{t("p.product_roadmap.items_h")}</h2>
      {#if roadmap.items.length === 0}<p class="muted" data-testid="roadmap-no-items">{t("p.product_roadmap.no_items")}</p>{/if}
      {#each groups(roadmap.items) as g (g.placement ?? "none")}
        <section class="surface stack" aria-labelledby={`pl-${g.placement ?? "none"}`} data-testid={`placement-${g.placement ?? "none"}`}>
          <h3 id={`pl-${g.placement ?? "none"}`}>{placementLabel(g.placement)}</h3>
          <ol class="items">
            {#each g.items as i (i.pid)}
              <li>
                <p><strong id={`it-${i.pid}`}>{i.title}</strong> <span class="small muted">{i.public_id}</span></p>
                {#if i.phase}<p class="small">{t("p.product_roadmap.phase_is", { phase: i.phase })}</p>{/if}
                {#if i.tags.length > 0}<p class="small muted">{t("p.product_roadmap.tags_are", { tags: i.tags.join(", ") })}</p>{/if}
                {#if i.description}<p>{i.description}</p>{/if}
                {#if i.expected_outcome}<p class="small">{t("p.product_roadmap.outcome_is", { outcome: i.expected_outcome })}</p>{/if}
                {#if editing === i.pid}
                  <form class="stack" onsubmit={(e) => saveItem(e, i)} novalidate aria-label={t("p.product_roadmap.change_item", { title: i.title })}>
                    {@render itemFields(edit)}
                    <div class="row">
                      <button class="button" type="submit" disabled={busy}>{t("p.product_roadmap.save_item")}</button>
                      <button class="button" type="button" onclick={() => (editing = null)}>{t("p.product_roadmap.cancel")}</button>
                    </div>
                  </form>
                {:else}
                  <div class="row">
                    <button class="button small" type="button" onclick={() => ((editing = i.pid), (edit = itemFormOf(i)), (problems = []))} aria-describedby={`it-${i.pid}`}>{t("p.product_roadmap.change")}</button>
                    <button class="button small" type="button" disabled={busy || !step(roadmap.items, i, -1)} onclick={() => move(i, step(roadmap!.items, i, -1)!)} aria-describedby={`it-${i.pid}`}>{t("p.product_roadmap.up")}</button>
                    <button class="button small" type="button" disabled={busy || !step(roadmap.items, i, 1)} onclick={() => move(i, step(roadmap!.items, i, 1)!)} aria-describedby={`it-${i.pid}`}>{t("p.product_roadmap.down")}</button>
                    <label class="inline"
                      >{t("p.product_roadmap.move_to")}
                      <select bind:value={moveTo[i.pid]} aria-describedby={`it-${i.pid}`}>
                        <option value="">{placementLabel(null)}</option>
                        {#each PLACEMENTS as p (p)}<option value={p}>{placementLabel(p)}</option>{/each}
                      </select></label
                    >
                    <button
                      class="button small"
                      type="button"
                      disabled={busy || (moveTo[i.pid] ?? i.placement ?? "") === (i.placement ?? "")}
                      onclick={() => move(i, { placement: (moveTo[i.pid] ?? "") === "" ? null : (moveTo[i.pid] as Placement) })}
                      aria-describedby={`it-${i.pid}`}>{t("p.product_roadmap.move")}</button
                    >
                    <button class="button small danger" type="button" disabled={busy} onclick={() => remove(i)} aria-describedby={`it-${i.pid}`}>{t("p.product_roadmap.remove")}</button>
                  </div>
                {/if}
              </li>
            {/each}
          </ol>
        </section>
      {/each}
    </section>

    <form class="surface stack" onsubmit={addItem} aria-labelledby="add-h" novalidate>
      <h2 id="add-h">{t("p.product_roadmap.add_h")}</h2>
      <label
        >{t("p.product_roadmap.f_placement")}
        <select bind:value={draftPlacement}>
          {#each PLACEMENTS as p (p)}<option value={p}>{placementLabel(p)}</option>{/each}
          <option value="">{placementLabel(null)}</option>
        </select></label
      >
      {@render itemFields(draft)}
      <button class="button primary" type="submit" disabled={busy}>{t("p.product_roadmap.add")}</button>
    </form>
  {/if}
{/if}

<style>
  .items {
    padding-inline-start: 1.25rem;
    display: grid;
    gap: 1rem;
  }
  h2 {
    font-size: 1.1rem;
  }
  h3 {
    font-size: 1rem;
  }
  label {
    display: block;
  }
  label.inline {
    display: inline-block;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
  }
</style>
