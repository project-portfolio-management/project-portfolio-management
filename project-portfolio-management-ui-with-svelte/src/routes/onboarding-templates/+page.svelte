<!--
  Checklist templates (`/onboarding-templates`; NC-1 slice 12 part n): the checklist each kind of onboarding case starts
  from, someone joining or leaving a plan and a plan taken in or closed out (`GET /api/onboarding/templates`), each item
  with when it is due from the effective date. Someone who may manage people changes one (`PUT …/{kind}`), items added,
  removed or reordered, with every problem said first with the item it is about, and nothing sent when nothing changed;
  or goes back to the built-in checklist after a question (`DELETE`). The built-in titles are the service's, in
  English, which the page says. A case already started keeps its items. Rules in `src/lib/onboarding-template-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatInteger } from "#lib/locale-format.js";
  import { ApiError } from "#lib/api/client.js";
  import { KIND_TEXT } from "#lib/onboarding-view.js";
  import { PpmClient, type OnboardingKind, type OnboardingTemplate } from "#lib/api/ppm.js";
  import {
    MAX_ITEMS,
    TITLE_MAX,
    offsetKey,
    rowsOf,
    sameItems,
    templateBody,
    type SeedRow,
    type TemplateProblem,
  } from "#lib/onboarding-template-view.js";

  const ppm = PpmClient.withFetch();

  let list = $state<OnboardingTemplate[] | null>(null);
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);
  let busy = $state(false);
  let editing = $state<OnboardingKind | null>(null);
  let rows = $state<SeedRow[]>([]);
  let problems = $state<TemplateProblem[]>([]);

  async function load() {
    try {
      list = await ppm.onboardingTemplates();
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.onboarding_templates.could_not_load");
    }
  }
  onMount(load);

  const refused = (err: unknown, key: "p.onboarding_templates.could_not_save" | "p.onboarding_templates.could_not_reset") =>
    err instanceof ApiError && err.status === 403 ? t("p.onboarding_templates.forbidden") : errorText(err, key);

  function startEdit(tpl: OnboardingTemplate) {
    editing = tpl.kind;
    rows = rowsOf(tpl.items);
    problems = [];
    error = notice = null;
  }
  function move(i: number, by: -1 | 1) {
    const j = i + by;
    if (j < 0 || j >= rows.length) return;
    const next = [...rows];
    [next[i], next[j]] = [next[j]!, next[i]!];
    rows = next;
  }

  async function save(e: SubmitEvent, tpl: OnboardingTemplate) {
    e.preventDefault();
    const r = templateBody(rows);
    if ("problems" in r) {
      problems = r.problems;
      return;
    }
    problems = [];
    if (sameItems(r.items, tpl.items)) {
      editing = null; // nothing changed: nothing sent
      return;
    }
    busy = true;
    try {
      await ppm.setOnboardingTemplate(tpl.kind, { items: r.items });
      notice = t("p.onboarding_templates.saved", { kind: KIND_TEXT[tpl.kind] });
      error = null;
      editing = null;
      await load();
    } catch (err) {
      error = refused(err, "p.onboarding_templates.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function reset(tpl: OnboardingTemplate) {
    if (!(await askConfirm(t("p.onboarding_templates.reset_ask", { kind: KIND_TEXT[tpl.kind] })))) return;
    busy = true;
    notice = error = null;
    try {
      await ppm.resetOnboardingTemplate(tpl.kind);
      notice = t("p.onboarding_templates.reset_done", { kind: KIND_TEXT[tpl.kind] });
      editing = null;
      await load();
    } catch (err) {
      error = refused(err, "p.onboarding_templates.could_not_reset");
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>{t("p.onboarding_templates.title_ppm")}</title></svelte:head>

<h1>{t("p.onboarding_templates.title")}</h1>
<p class="lede">{t("p.onboarding_templates.intro")}</p>
{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{#if error}<p class="banner error" role="alert" data-testid="ot-error">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="ot-notice">{notice}</p>{/if}

{#if !list && !loadError}
  <p class="muted">{t("p.onboarding_templates.loading")}</p>
{:else if list}
  {#each list as tpl (tpl.kind)}
    <section class="surface stack" aria-labelledby={`h-${tpl.kind}`} data-testid={`ot-${tpl.kind}`}>
      <h2 id={`h-${tpl.kind}`}>{KIND_TEXT[tpl.kind]}</h2>
      <p class="small muted">
        {tpl.custom ? t("p.onboarding_templates.is_custom") : t("p.onboarding_templates.is_builtin")}
      </p>
      {#if editing === tpl.kind}
        <form class="stack" onsubmit={(e) => save(e, tpl)} novalidate aria-labelledby={`h-${tpl.kind}`}>
          {#if problems.length > 0}
            <ul class="banner error" role="alert" data-testid="ot-problems">
              {#each problems as p, i (i)}<li>{tDynamic(p.key, { n: formatInteger(p.n ?? 0), max: formatInteger(MAX_ITEMS) })}</li>{/each}
            </ul>
          {/if}
          <ol class="rows">
            {#each rows as row, i (i)}
              <li>
                <label
                  >{t("p.onboarding_templates.item_title", { n: formatInteger(i + 1) })}
                  <input bind:value={row.title} maxlength={TITLE_MAX} /></label
                >
                <label
                  >{t("p.onboarding_templates.item_offset", { n: formatInteger(i + 1) })}
                  <input inputmode="numeric" bind:value={row.offset} /></label
                >
                <div class="row">
                  <button class="button small" type="button" disabled={i === 0} onclick={() => move(i, -1)}
                    >{t("p.onboarding_templates.up", { n: formatInteger(i + 1) })}</button
                  >
                  <button class="button small" type="button" disabled={i === rows.length - 1} onclick={() => move(i, 1)}
                    >{t("p.onboarding_templates.down", { n: formatInteger(i + 1) })}</button
                  >
                  <button
                    class="button small danger"
                    type="button"
                    onclick={() => (rows = rows.filter((_, k) => k !== i))}
                    >{t("p.onboarding_templates.remove", { n: formatInteger(i + 1) })}</button
                  >
                </div>
              </li>
            {/each}
          </ol>
          <div class="row">
            <button
              class="button small"
              type="button"
              disabled={rows.length >= MAX_ITEMS}
              onclick={() => (rows = [...rows, { title: "", offset: "0" }])}>{t("p.onboarding_templates.add")}</button
            >
          </div>
          <div class="row">
            <button class="button" type="submit" disabled={busy}>{t("p.onboarding_templates.save")}</button>
            <button class="button" type="button" onclick={() => (editing = null)}>{t("p.onboarding_templates.cancel")}</button>
          </div>
        </form>
      {:else}
        <ol class="items">
          {#each tpl.items as s, i (i)}
            {@const o = offsetKey(s.offset_days)}
            <li>{s.title} <span class="small muted">({tDynamic(o.key, { n: formatInteger(o.n) })})</span></li>
          {/each}
        </ol>
        <div class="row">
          <button class="button small" type="button" onclick={() => startEdit(tpl)} aria-describedby={`h-${tpl.kind}`}
            >{t("p.onboarding_templates.edit")}</button
          >
          {#if tpl.custom}
            <button
              class="button small danger"
              type="button"
              disabled={busy}
              onclick={() => reset(tpl)}
              aria-describedby={`h-${tpl.kind}`}>{t("p.onboarding_templates.reset")}</button
            >
          {/if}
        </div>
      {/if}
    </section>
  {/each}
{/if}

<style>
  h2 {
    font-size: 1.05rem;
  }
  label {
    display: block;
  }
  .rows {
    display: grid;
    gap: 0.75rem;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
  }
</style>
