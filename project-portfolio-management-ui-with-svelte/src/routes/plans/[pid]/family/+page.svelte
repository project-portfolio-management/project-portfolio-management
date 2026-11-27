<!--
  A product family (`/plans/[pid]/family`, RE-5, RE-7; NC-1 slice 12 part l): a plan that holds product plans, its name
  the plan's, its products counted (those the reader may read). Its colour, any `#rrggbb`, is shown with how it reads
  under white and under black text (WCAG 2 ratios; AA is 4.5:1 for body text), never refused for its contrast; its name
  in other languages is written one `code: text` to a line. A change sends only what changed
  (`PUT /api/plans/{pid}/roadmap-family`), every problem said first; removing the colour removes the translations too,
  after a question (`colour: null`). Rules in `src/lib/family-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatDecimal, formatInteger } from "#lib/locale-format.js";
  import { PpmClient } from "#lib/api/ppm.js";
  import { colourOf, familyChange, type RoadmapFamilyView } from "#lib/family-view.js";
  import { translationsText } from "#lib/roadmap-category-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let fam = $state<RoadmapFamilyView | null>(null);
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let problems = $state<string[]>([]);
  let notice = $state<string | null>(null);
  let busy = $state(false);
  let colour = $state("");
  let translations = $state("");

  function fill(f: RoadmapFamilyView) {
    fam = f;
    colour = f.colour ?? "";
    translations = translationsText(f.translations ?? {});
  }

  onMount(async () => {
    try {
      fill(await ppm.roadmapFamily(pid));
    } catch (err) {
      loadError = errorText(err, "p.roadmap_family.could_not_load");
    }
  });

  /** The colour picker's value: the typed colour when it is one; else none, and the picker shows its own default. */
  const pickerValue = $derived(colourOf(colour) ?? undefined);

  async function save(e: SubmitEvent) {
    e.preventDefault();
    if (!fam) return;
    notice = error = null;
    const r = familyChange(colour, translations, fam);
    if (r === null) return void (notice = t("p.roadmap_family.unchanged"));
    if ("problems" in r) return void (problems = r.problems);
    problems = [];
    busy = true;
    try {
      fill(await ppm.setRoadmapFamily(pid, r.body));
      notice = t("p.roadmap_family.saved");
    } catch (err) {
      error = errorText(err, "p.roadmap_family.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function removeColour() {
    if (!fam?.colour || !(await askConfirm(t("p.roadmap_family.remove_ask")))) return;
    busy = true;
    notice = error = null;
    try {
      fill(await ppm.setRoadmapFamily(pid, { colour: null }));
      notice = t("p.roadmap_family.removed");
    } catch (err) {
      error = errorText(err, "p.roadmap_family.could_not_save");
    } finally {
      busy = false;
    }
  }

  const ratio = (n: number) => formatDecimal(n, 2);
</script>

<svelte:head><title>{t("p.roadmap_family.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.roadmap_family.title")}</h1>
<p class="lede">{t("p.roadmap_family.intro")}</p>
{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{#if error}<p class="banner error" role="alert" data-testid="family-error">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="family-notice">{notice}</p>{/if}
{#if problems.length > 0}
  <ul class="banner error" role="alert" data-testid="family-problems">{#each problems as p (p)}<li>{tDynamic(p)}</li>{/each}</ul>
{/if}

{#if !fam && !loadError}
  <p class="muted">{t("p.roadmap_family.loading")}</p>
{:else if fam}
  <section class="surface stack" aria-labelledby="fam-h">
    <h2 id="fam-h">{fam.name}</h2>
    <p data-testid="family-products">{t("p.roadmap_family.products", { n: formatInteger(fam.products) })}</p>
    {#if fam.colour && fam.contrast}
      <p class="swatch-row">
        <span class="swatch" style:background-color={fam.colour} aria-hidden="true"></span>
        <span data-testid="family-colour">{t("p.roadmap_family.colour_is", { colour: fam.colour })}</span>
      </p>
      <ul class="small" data-testid="family-contrast">
        <li>
          {t("p.roadmap_family.with_white", { ratio: ratio(fam.contrast.with_white_text) })} —
          {fam.contrast.white_text_meets_aa ? t("p.roadmap_family.meets_aa") : t("p.roadmap_family.below_aa")}
        </li>
        <li>
          {t("p.roadmap_family.with_black", { ratio: ratio(fam.contrast.with_black_text) })} —
          {fam.contrast.black_text_meets_aa ? t("p.roadmap_family.meets_aa") : t("p.roadmap_family.below_aa")}
        </li>
        <li>{fam.contrast.better_text === "white" ? t("p.roadmap_family.better_white") : t("p.roadmap_family.better_black")}</li>
      </ul>
    {:else}
      <p class="muted" data-testid="family-no-colour">{t("p.roadmap_family.no_colour")}</p>
    {/if}
  </section>

  <form class="surface stack" onsubmit={save} aria-labelledby="set-h" novalidate>
    <h2 id="set-h">{t("p.roadmap_family.set_h")}</h2>
    <label>{t("p.roadmap_family.f_colour")} <input bind:value={colour} autocomplete="off" /></label>
    <label
      >{t("p.roadmap_family.f_picker")}
      <input type="color" value={pickerValue} oninput={(e) => (colour = e.currentTarget.value)} /></label
    >
    <label>{t("p.roadmap_family.f_translations")} <textarea rows="3" bind:value={translations}></textarea></label>
    <div class="row">
      <button class="button primary" type="submit" disabled={busy}>{t("p.roadmap_family.save")}</button>
      {#if fam.colour}
        <button class="button danger" type="button" disabled={busy} onclick={removeColour}>{t("p.roadmap_family.remove")}</button>
      {/if}
    </div>
  </form>
{/if}

<style>
  h2 {
    font-size: 1.1rem;
  }
  label {
    display: block;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
  .swatch-row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
  .swatch {
    display: inline-block;
    inline-size: 2rem;
    block-size: 2rem;
    border: 1px solid var(--mxi-color-muted);
    border-radius: 0.25rem;
  }
</style>
