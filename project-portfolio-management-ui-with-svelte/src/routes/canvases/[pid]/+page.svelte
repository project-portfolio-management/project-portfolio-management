<!--
  One strategy canvas (`/canvases/[pid]`, LT-1, NC-1 slice 9): every box of its template in order, empty ones too
  (an empty box is itself the finding), each box's items, and the ways to change them: add an item to a box, edit its
  text or move it to another box, remove it, and turn it into an idea, an objective or (on a plan's canvas) a risk,
  once. Removing the canvas removes its items; what an item already became stays. Every problem the service would
  refuse is said before anything is sent; a refusal is the catalogue's sentence, never the service's text.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t } from "#lib/i18n.svelte.js";
  import { PpmClient, type CanvasItem, type CanvasPromoteBody, type CanvasView } from "#lib/api/ppm.js";
  import {
    BOX_LABELS,
    CANVAS_ITEMS_MAX,
    CANVAS_TEXT_MAX,
    TEMPLATE_LABELS,
    itemProblem,
    itemsByBox,
    labelFor,
    promoteBody,
    promoteTargets,
    promotedLabel,
  } from "#lib/canvas-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let view = $state<CanvasView | null>(null);
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  let drafts = $state<Record<string, string>>({});
  let editing = $state<string | null>(null);
  let editText = $state("");
  let editBox = $state("");
  let promoting = $state<string | null>(null);
  let target = $state<CanvasPromoteBody["target"]>("idea");
  let probability = $state("");
  let impact = $state("");

  const canvas = $derived(view?.canvas ?? null);
  const byBox = $derived(itemsByBox(view?.items ?? []));
  const full = $derived((view?.items.length ?? 0) >= CANVAS_ITEMS_MAX);
  const back = $derived(canvas?.plan_pid ? `/plans/${canvas.plan_pid}/canvases` : "/canvases");
  const boxName = (key: string): string =>
    labelFor(BOX_LABELS, key, view?.boxes.find((b) => b.key === key)?.title, t);
  const firstLine = (body: string): string => body.split("\n").find((l) => l.trim() !== "")?.trim() ?? body;
  const targetName = (k: CanvasPromoteBody["target"]): string =>
    k === "idea" ? t("p.canvas.target_idea") : k === "risk" ? t("p.canvas.target_risk") : t("p.canvas.target_objective");
  function becameHref(item: CanvasItem): string {
    if (item.promoted_to_kind === "idea") return localeHref("/ideas");
    if (item.promoted_to_kind === "objective") return localeHref("/objectives");
    return localeHref(`/plans/${canvas?.plan_pid ?? ""}/governance`);
  }

  async function load() {
    loadError = null;
    try {
      view = await ppm.canvas(pid);
    } catch (err) {
      loadError = errorText(err, "p.canvas.could_not_load");
    }
  }
  onMount(load);

  /** Run one change, say it was done, and show the canvas as the service now has it. */
  async function act(run: () => Promise<unknown>, done: string, failKey: Parameters<typeof errorText>[1]) {
    error = null;
    notice = "";
    busy = true;
    try {
      await run();
      notice = done;
      await load();
      return true;
    } catch (err) {
      error = errorText(err, failKey);
      return false;
    } finally {
      busy = false;
    }
  }

  async function add(e: SubmitEvent, box: string) {
    e.preventDefault();
    const text = drafts[box] ?? "";
    const problem = itemProblem(text);
    if (problem) {
      error = t(problem);
      notice = "";
      return;
    }
    if (await act(() => ppm.addCanvasItem(pid, { box, body: text.trim() }), t("p.canvas.added"), "p.canvas.could_not_add"))
      drafts[box] = "";
  }

  function startEdit(item: CanvasItem) {
    promoting = null;
    editing = item.pid;
    editText = item.body;
    editBox = item.box_key;
  }
  async function saveEdit(e: SubmitEvent, item: CanvasItem) {
    e.preventDefault();
    const problem = itemProblem(editText);
    if (problem) {
      error = t(problem);
      notice = "";
      return;
    }
    const body = editText.trim();
    const change = {
      ...(body !== item.body ? { body } : {}),
      ...(editBox !== item.box_key ? { box: editBox } : {}),
    };
    if (Object.keys(change).length === 0) {
      editing = null;
      notice = t("p.canvas.unchanged");
      return;
    }
    if (await act(() => ppm.updateCanvasItem(pid, item.pid, change), t("p.canvas.saved"), "p.canvas.could_not_save"))
      editing = null;
  }

  async function remove(item: CanvasItem) {
    if (!(await askConfirm(t("p.canvas.confirm_remove_item", { text: firstLine(item.body) })))) return;
    await act(() => ppm.deleteCanvasItem(pid, item.pid), t("p.canvas.removed"), "p.canvas.could_not_remove");
  }

  function startPromote(item: CanvasItem) {
    editing = null;
    promoting = item.pid;
    target = "idea";
    probability = "";
    impact = "";
  }
  async function promote(e: SubmitEvent, item: CanvasItem) {
    e.preventDefault();
    const made = promoteBody(target, probability, impact);
    if ("problem" in made) {
      error = t(made.problem);
      notice = "";
      return;
    }
    if (
      await act(
        () => ppm.promoteCanvasItem(pid, item.pid, made.body),
        t("p.canvas.promoted", { kind: targetName(target) }),
        "p.canvas.could_not_promote",
      )
    )
      promoting = null;
  }

  async function removeCanvas() {
    if (!canvas || !(await askConfirm(t("p.canvas.confirm_remove_canvas", { title: canvas.title })))) return;
    error = null;
    busy = true;
    try {
      await ppm.deleteCanvas(pid);
      await goto(localeHref(back));
    } catch (err) {
      error = errorText(err, "p.canvas.could_not_remove_canvas");
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>{t("p.canvas.one_title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(back)}>{t("p.canvas.back")}</a></p>
{#if !canvas}
  <h1>{t("p.canvas.one_title")}</h1>
  {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{:else}
  <h1>{canvas.title}</h1>
  <p class="small muted">
    {labelFor(TEMPLATE_LABELS, canvas.template, undefined, t)} ·
    {canvas.plan_pid ? t("p.canvas.on_plan") : t("p.canvas.free_standing")}
  </p>
  {#if canvas.description}<p class="description">{canvas.description}</p>{/if}
  {#if error}<p class="banner" role="alert">{error}</p>{/if}
  <p class="small" role="status">{notice}</p>
  {#if full}<p class="small" role="note">{t("p.canvas.full", { max: CANVAS_ITEMS_MAX })}</p>{/if}

  <div class="boxes">
    {#each view?.boxes ?? [] as b (b.key)}
      {@const items = byBox.get(b.key) ?? []}
      <section class="box" aria-labelledby={`box-${b.key}`}>
        <h2 id={`box-${b.key}`}>{boxName(b.key)} <span class="small muted">({items.length})</span></h2>
        {#if items.length === 0}<p class="small muted">{t("p.canvas.box_empty")}</p>{/if}
        <ul>
          {#each items as item (item.pid)}
            <li>
              {#if editing === item.pid}
                <form class="stack" onsubmit={(e) => saveEdit(e, item)}>
                  <label>
                    {t("p.canvas.f_text")}
                    <textarea bind:value={editText} rows="3" maxlength={CANVAS_TEXT_MAX}></textarea>
                  </label>
                  <label>
                    {t("p.canvas.f_box")}
                    <select bind:value={editBox}>
                      {#each view?.boxes ?? [] as other (other.key)}<option value={other.key}>{boxName(other.key)}</option>{/each}
                    </select>
                  </label>
                  <div class="row">
                    <button class="button small" type="submit" disabled={busy}>{t("p.canvas.save")}</button>
                    <button class="button small" type="button" onclick={() => (editing = null)}>{t("p.canvas.cancel")}</button>
                  </div>
                </form>
              {:else}
                <p class="body">{item.body}</p>
                {#if item.owner_ref}<p class="small muted">{t("p.canvas.owner", { owner: item.owner_ref })}</p>{/if}
                {@const became = promotedLabel(item.promoted_to_kind)}
                {#if became}<p class="small"><a href={becameHref(item)}>{t(became)}</a></p>{/if}
                <div class="row">
                  <button class="button small" type="button" disabled={busy} onclick={() => startEdit(item)} aria-label={`${t("p.canvas.edit")}: ${firstLine(item.body)}`}>{t("p.canvas.edit")}</button>
                  <button class="button small" type="button" disabled={busy} onclick={() => remove(item)} aria-label={`${t("p.canvas.remove")}: ${firstLine(item.body)}`}>{t("p.canvas.remove")}</button>
                  {#if !became}
                    <button class="button small" type="button" disabled={busy} onclick={() => startPromote(item)} aria-label={`${t("p.canvas.promote")}: ${firstLine(item.body)}`}>{t("p.canvas.promote")}</button>
                  {/if}
                </div>
                {#if promoting === item.pid}
                  <form class="stack promote" novalidate onsubmit={(e) => promote(e, item)}>
                    <fieldset>
                      <legend>{t("p.canvas.promote_legend")}</legend>
                      {#each promoteTargets(canvas.plan_pid) as k (k)}
                        <label class="check"><input type="radio" name={`target-${item.pid}`} value={k} bind:group={target} /> {targetName(k)}</label>
                      {/each}
                    </fieldset>
                    {#if !canvas.plan_pid}<p class="small muted">{t("p.canvas.no_risk_free_standing")}</p>{/if}
                    {#if target === "risk"}
                      <label>
                        {t("p.canvas.f_probability")}
                        <input type="number" min="1" max="5" step="1" bind:value={probability} />
                      </label>
                      <label>
                        {t("p.canvas.f_impact")}
                        <input type="number" min="1" max="5" step="1" bind:value={impact} />
                      </label>
                    {/if}
                    <p class="small muted">{t("p.canvas.promote_once")}</p>
                    <div class="row">
                      <button class="button small" type="submit" disabled={busy}>{t("p.canvas.promote_go")}</button>
                      <button class="button small" type="button" onclick={() => (promoting = null)}>{t("p.canvas.cancel")}</button>
                    </div>
                  </form>
                {/if}
              {/if}
            </li>
          {/each}
        </ul>
        {#if !full}
          <form class="stack" onsubmit={(e) => add(e, b.key)}>
            <label>
              {t("p.canvas.f_new_item", { box: boxName(b.key) })}
              <textarea bind:value={drafts[b.key]} rows="2" maxlength={CANVAS_TEXT_MAX}></textarea>
            </label>
            <div class="row">
              <button class="button small" type="submit" disabled={busy} aria-label={`${t("p.canvas.add")}: ${boxName(b.key)}`}>{t("p.canvas.add")}</button>
            </div>
          </form>
        {/if}
      </section>
    {/each}
  </div>

  <section class="danger" aria-labelledby="remove-canvas-h">
    <h2 id="remove-canvas-h">{t("p.canvas.remove_canvas_heading")}</h2>
    <p class="small muted">{t("p.canvas.remove_canvas_says")}</p>
    <button class="button" type="button" disabled={busy} onclick={removeCanvas}>{t("p.canvas.remove_canvas")}</button>
  </section>
{/if}

<style>
  .boxes {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(16rem, 1fr));
    gap: 1rem;
    margin-block-start: 1rem;
  }
  .box {
    border: 1px solid var(--border, currentColor);
    border-radius: 0.5rem;
    padding: 0.75rem;
    min-inline-size: 0;
  }
  .box h2 {
    font-size: 1.05rem;
    margin-block: 0 0.5rem;
  }
  .box ul {
    list-style: none;
    padding: 0;
    margin: 0 0 0.75rem;
    display: grid;
    gap: 0.75rem;
  }
  .body,
  .description {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    margin-block: 0 0.25rem;
  }
  .check {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }
  .promote fieldset {
    display: grid;
    gap: 0.35rem;
  }
  .danger {
    margin-block-start: 2rem;
  }
</style>
