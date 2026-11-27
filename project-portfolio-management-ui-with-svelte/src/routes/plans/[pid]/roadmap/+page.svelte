<!--
  A product plan's public roadmap (`/plans/[pid]/roadmap`, RE-1, RE-3; NC-1 slice 12 part l): made for a plan of kind
  Product (`PUT /api/plans/{pid}/roadmap`), its title, address (slug), vision and service description changed (only
  what changed sent), how fresh it is and when it was last reviewed, marked reviewed today (`PUT …/reviewed`), and its
  items in page order by placement, then the unplaced: added, changed, moved one place up or down or to another
  placement, and removed after a question. Every limit is said before sending; a slug another roadmap has or had (409)
  has its own sentence. Publishing or taking it off the public reader goes as one change set (RE-2): previewed first
  (`POST …/publish/preview`: what would change, what conflicts), then published (`POST …/publish`), a conflict meanwhile
  (409) said in the page's words. An item's details (indented lines) and the organizations it is for are set with its
  other fields; the roadmap and its items are translated field by field (RE-6), its category chosen (RE-7). With gathering on,
  several edits are gathered in memory into one pending change set (RE-13), previewed and published together, kept
  through an ended session, and leaving with any asks first. Translating details is not on this page yet. Rules in
  `src/lib/product-roadmap-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { beforeNavigate } from "$app/navigation";
  import { isSessionEnded } from "#lib/unsaved.js";
  import { page } from "$app/state";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { i18n, localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatDate, formatInteger } from "#lib/locale-format.js";
  import { ApiError } from "#lib/api/client.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import { PpmClient, type ProductRoadmap } from "#lib/api/ppm.js";
  import { isLanguageCode, shownName, type RoadmapCategory } from "#lib/roadmap-category-view.js";
  import { shownOrgName } from "#lib/organization-view.js";
  import type { Organization } from "#lib/api/ppm.js";
  import {
    PHASE_MAX,
    PLACEMENTS,
    SLUG_MAX,
    TEXT_MAX,
    TITLE_MAX,
    fieldKey,
    groups,
    itemChange,
    emptyPending,
    gatherItem,
    gatherNew,
    gatherRoadmap,
    pendingCount,
    pendingLines,
    pendingSet as gatheredSet,
    withPending,
    type Pending,
    publishSet,
    valueWords,
    ITEM_TRANSLATABLE,
    ROADMAP_TRANSLATABLE,
    translationChange,
    fieldsOf,
    translationInput,
    translationSummary,
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
  let organizations = $state<Organization[]>([]);
  // RE-2, RE-13: edits gathered in this tab (in memory only), published as one change set; leaving with any asks first.
  let gathering = $state(false);
  let gathered = $state<Pending>(emptyPending());
  let sessionEnded = $state(false);
  const shown = $derived(roadmap ? withPending(roadmap, gathered) : null);
  beforeNavigate(({ cancel, type }) => {
    if (pendingCount(gathered) === 0) return;
    if (type === "leave") {
      cancel();
      return;
    }
    if (!confirm(t("form.leaveUnsaved"))) cancel();
  });
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
  let lang = $state("cy");
  /** Each record's translation inputs for `lang`: "roadmap", or an item's id, then field to text. */
  let trInputs = $state<Record<string, Record<string, string>>>({});
  let trProblems = $state<{ key: string; field: string }[]>([]);

  /** Fill the translation inputs from what is stored for `lang`. */
  function fillTranslations() {
    if (!roadmap) return;
    const of = (shown: Record<string, { value: unknown }> | undefined, fields: readonly string[]) =>
      Object.fromEntries(fields.map((f) => [f, translationInput(shown?.[f]?.value)]));
    trInputs = {
      roadmap: of(roadmap.translations?.[lang], ROADMAP_TRANSLATABLE),
      ...Object.fromEntries(roadmap.items.map((i) => [i.pid, of(i.translations?.[lang], ITEM_TRANSLATABLE)])),
    };
  }
  let preview = $state<{ changes: ChangeLine[]; conflicts: ConflictLine[]; problems: string[]; would_publish: boolean } | null>(null);

  async function load() {
    try {
      const [r, plan, cats, orgs] = await Promise.all([
        ppm.productRoadmap(pid),
        repo.get(pid),
        // The categories only fill the picker; without them the roadmap still reads.
        ppm.roadmapCategories().catch(() => [] as RoadmapCategory[]),
        // The organizations only fill the picker and name an item's; without them the roadmap still reads.
        ppm.organizations().then((r) => r.organizations, () => [] as Organization[]),
      ]);
      roadmap = r;
      categories = cats;
      organizations = orgs;
      fillTranslations();
      isProduct = plan.kind === "Product";
      form = roadmapFormOf(r ? withPending(r, gathered) : null);
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
    sessionEnded = isSessionEnded(err);
    if (sessionEnded) error = t("form.sessionEnded");
    else if (err instanceof ApiError && err.status === 409) error = t("p.product_roadmap.slug_taken");
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

  /** Keep the gathered edits and say how many are pending. */
  function gather(next: Pending) {
    gathered = next;
    error = null;
    notice = t("p.product_roadmap.pending_added", { n: formatInteger(pendingCount(next)) });
  }
  /** Drop every gathered edit, after a question. */
  async function discard() {
    if (!(await askConfirm(t("p.product_roadmap.pending_discard_ask", { n: formatInteger(pendingCount(gathered)) })))) return;
    gathered = emptyPending();
    preview = pendingSet = null;
    if (roadmap) form = roadmapFormOf(roadmap);
    notice = t("p.product_roadmap.pending_discarded");
  }

  async function saveRoadmap(e: SubmitEvent) {
    e.preventDefault();
    const r = roadmapChange(form, shown);
    if (r === null) {
      notice = t("p.product_roadmap.unchanged");
      return;
    }
    if ("problems" in r) {
      problems = r.problems;
      return;
    }
    if (gathering && roadmap) {
      const g = gatherRoadmap(gathered, roadmap, r.body);
      if ("problems" in g) return void (problems = g.problems);
      problems = [];
      return gather(g.pending);
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
    if (gathering && roadmap) {
      gather(gatherNew(gathered, r.body));
      draft = itemFormOf(null);
      return;
    }
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
    const read = roadmap?.items.find((x) => x.pid === i.pid);
    if (gathering && read) {
      gather(gatherItem(gathered, read, r.body));
      editing = null;
      return;
    }
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

  /** A category or organizations by name rather than id, for a change line. */
  function named(field: string | null | undefined, v: unknown): unknown {
    if (field === "category_pid" && typeof v === "string") {
      const c = categories.find((x) => x.pid === v);
      return c ? shownName(c, i18n.locale) : v;
    }
    if (field === "organization_pids" && Array.isArray(v)) return v.map((o) => orgName(String(o)));
    return v;
  }
  /** A change line in words. */
  function changeText(c: ChangeLine): string {
    if (c.subject === "new_item") return t("p.product_roadmap.new_item_line", { title: c.title ?? "" });
    const params = {
      field: tDynamic(fieldKey(c.field)),
      from: valueWords(c.field, named(c.field, c.from), words),
      to: valueWords(c.field, named(c.field, c.to), words),
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
  async function previewPublish(publish: boolean | null) {
    if (!roadmap) return;
    error = notice = null;
    const set = gatheredSet(gathered, publish === null ? undefined : publishSet(roadmap.published, publish).roadmap?.published);
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
      gathered = emptyPending();
      sessionEnded = false;
      await load();
    } catch (err) {
      if (err instanceof ApiError && err.status === 409) {
        const body = err.body as { conflicts?: ConflictLine[] } | undefined;
        error = t("p.product_roadmap.conflict_refused");
        preview = { changes: [], conflicts: body?.conflicts ?? [], problems: [], would_publish: false };
        await load();
      } else if (isSessionEnded(err)) {
        sessionEnded = true;
        error = t("form.sessionEnded");
      } else {
        error = errorText(err, "p.product_roadmap.could_not_publish");
      }
    } finally {
      busy = false;
    }
  }

  /** Save one record's translation into `lang`: only what changed; an emptied field removes its translation. */
  async function saveTranslation(e: SubmitEvent, key: string) {
    e.preventDefault();
    if (!roadmap) return;
    error = notice = null;
    if (!isLanguageCode(lang.trim().toLowerCase())) {
      trProblems = [];
      error = t("p.product_roadmap.tr_lang_bad");
      return;
    }
    const code = lang.trim().toLowerCase();
    const item = roadmap.items.find((i) => i.pid === key);
    const r = item
      ? translationChange(ITEM_TRANSLATABLE, fieldsOf(item), item.translations?.[code], trInputs[key] ?? {})
      : translationChange(ROADMAP_TRANSLATABLE, fieldsOf(roadmap), roadmap.translations?.[code], trInputs.roadmap ?? {});
    if (r === null) return void (notice = t("p.product_roadmap.unchanged"));
    if ("problems" in r) return void (trProblems = r.problems);
    trProblems = [];
    const body = r.body;
    await run(
      () => (item ? ppm.translateRoadmapItem(pid, item.pid, code, body) : ppm.translateRoadmap(pid, code, body)),
      t("p.product_roadmap.tr_saved", { lang: code }),
      "p.product_roadmap.could_not_save",
    );
  }

  /** An organization's name in the reader's language, a retired one marked; an unknown id as itself. */
  const orgName = (opid: string) => {
    const o = organizations.find((x) => x.pid === opid);
    if (!o) return opid;
    const n = shownOrgName(o, i18n.locale);
    return o.active ? n : t("p.product_roadmap.org_retired", { name: n });
  };
  /** The organizations an item's form offers: the active ones, and any retired one the item already has. */
  const orgChoices = (f: ItemForm) => organizations.filter((o) => o.active || f.organizations.includes(o.pid));

  const sourceOf = (record: Record<string, unknown>, f: string) => {
    const v = record[f];
    return Array.isArray(v) ? v.join(", ") : (v as string | null | undefined);
  };

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
  <label>{t("p.product_roadmap.f_details")} <textarea rows="4" bind:value={f.details}></textarea></label>
  {#if orgChoices(f).length > 0}
    <fieldset class="orgs">
      <legend>{t("p.product_roadmap.f_orgs")}</legend>
      {#each orgChoices(f) as o (o.pid)}
        <label class="inline"><input
            type="checkbox"
            checked={f.organizations.includes(o.pid)}
            onchange={(e) => (f.organizations = e.currentTarget.checked ? [...f.organizations, o.pid] : f.organizations.filter((x) => x !== o.pid))}
          /> {orgName(o.pid)}</label>
      {/each}
    </fieldset>
  {/if}
{/snippet}

<svelte:head><title>{t("p.product_roadmap.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.product_roadmap.title")}</h1>
<p class="lede">{t("p.product_roadmap.intro")}</p>
{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{#if error}
  <p class="banner error" role="alert" data-testid="roadmap-error">
    {error}{#if sessionEnded}
      <a href={localeHref("/signin")} target="_blank" rel="noopener">{t("form.signInNewTab")}</a>{/if}
  </p>
{/if}
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

    <section class="surface stack" aria-labelledby="gather-h" data-testid="pending">
      <h2 id="gather-h">{t("p.product_roadmap.gather_h")}</h2>
      <p class="small muted">{t("p.product_roadmap.gather_intro")}</p>
      <label class="inline"
        ><input type="checkbox" bind:checked={gathering} disabled={pendingCount(gathered) > 0} />
        {t("p.product_roadmap.gather_on")}</label
      >
      {#if pendingCount(gathered) > 0}
        <p data-testid="pending-count">{t("p.product_roadmap.pending_n", { n: formatInteger(pendingCount(gathered)) })}</p>
        <ul data-testid="pending-lines">{#each pendingLines(gathered) as c, n (n)}<li>{changeText(c)}</li>{/each}</ul>
        <div class="row">
          <button class="button" type="button" disabled={busy} onclick={() => previewPublish(null)}>{t("p.product_roadmap.pending_preview")}</button>
          <button class="button danger" type="button" disabled={busy} onclick={discard}>{t("p.product_roadmap.pending_discard")}</button>
        </div>
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
      {#each groups(shown?.items ?? []) as g (g.placement ?? "none")}
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
                {#if (i.details ?? []).length > 0}
                  <ul class="details" data-testid={`details-${i.pid}`}>
                    {#each i.details ?? [] as d, n (n)}<li style:margin-inline-start={`${d.level * 1.25}rem`}>{d.text}</li>{/each}
                  </ul>
                {/if}
                {#if (i.organization_pids ?? []).length > 0}
                  <p class="small">{t("p.product_roadmap.orgs_are", { orgs: (i.organization_pids ?? []).map(orgName).join(", ") })}</p>
                {/if}
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


    <section class="surface stack" aria-labelledby="tr-h" data-testid="translations">
      <h2 id="tr-h">{t("p.product_roadmap.tr_h")}</h2>
      <p class="small muted">{t("p.product_roadmap.tr_intro")}</p>
      {#each [translationSummary([roadmap.translations, ...roadmap.items.map((i) => i.translations)])] as summary (summary.length)}
        {#if summary.length > 0}
          <p class="small" data-testid="tr-summary">
            {summary.map((s) => (s.review > 0 ? t("p.product_roadmap.tr_lang_review", { lang: s.lang, n: formatInteger(s.review) }) : s.lang)).join(" · ")}
          </p>
        {/if}
      {/each}
      <label>{t("p.product_roadmap.tr_lang")} <input bind:value={lang} onchange={fillTranslations} maxlength="35" autocomplete="off" /></label>
      {#if trProblems.length > 0}
        <ul class="banner error" role="alert" data-testid="tr-problems">
          {#each trProblems as p, n (n)}<li>{tDynamic(p.key, { field: tDynamic(fieldKey(p.field)) })}</li>{/each}
        </ul>
      {/if}
      {#snippet trFields(record: Record<string, unknown>, key: string, fields: readonly string[], shown: Record<string, { needs_review: boolean }> | undefined)}
        {#each fields as f (f)}
          {#if sourceOf(record, f)}
            <label
              >{tDynamic(fieldKey(f))}
              <span class="small muted">{t("p.product_roadmap.tr_source", { text: sourceOf(record, f) ?? "" })}</span>
              {#if shown?.[f]?.needs_review}<span class="small"> · {t("p.product_roadmap.tr_needs_review")}</span>{/if}
              {#if f === "vision" || f === "service_description" || f === "description" || f === "expected_outcome"}
                <textarea rows="2" bind:value={trInputs[key]![f]}></textarea>
              {:else}
                <input bind:value={trInputs[key]![f]} />
              {/if}
            </label>
          {/if}
        {/each}
      {/snippet}
      {#if trInputs.roadmap}
        <form class="stack" onsubmit={(e) => saveTranslation(e, "roadmap")} novalidate aria-label={t("p.product_roadmap.tr_roadmap_h")}>
          <h3>{t("p.product_roadmap.tr_roadmap_h")}</h3>
          {@render trFields(fieldsOf(roadmap), "roadmap", ROADMAP_TRANSLATABLE, roadmap.translations?.[lang])}
          <div><button class="button" type="submit" disabled={busy}>{t("p.product_roadmap.tr_save")}</button></div>
        </form>
      {/if}
      {#each roadmap.items as i (i.pid)}
        {#if trInputs[i.pid]}
          <details>
            <summary>{t("p.product_roadmap.tr_item", { title: i.title })}</summary>
            <form class="stack" onsubmit={(e) => saveTranslation(e, i.pid)} novalidate aria-label={t("p.product_roadmap.tr_item", { title: i.title })}>
              {@render trFields(fieldsOf(i), i.pid, ITEM_TRANSLATABLE, i.translations?.[lang])}
              <div><button class="button" type="submit" disabled={busy}>{t("p.product_roadmap.tr_save")}</button></div>
            </form>
          </details>
        {/if}
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
  .details {
    list-style: disc;
    padding-inline-start: 1.25rem;
  }
  fieldset.orgs {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 1rem;
  }
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
