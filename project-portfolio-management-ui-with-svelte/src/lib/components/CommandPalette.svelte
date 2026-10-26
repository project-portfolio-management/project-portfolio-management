<!--
  CommandPalette (BR-14) — Ctrl or Cmd with K opens a modal palette: type a plan name or a command, move with the
  arrow keys, Enter goes there, Escape closes. Lily's Dialog is the modal (focus trap, Escape, focus returned to the
  opener) and Lily's Combobox is the input and listbox (roles and labels); the arrow keys and the active row are
  done here because the primitives are headless. The ranking is in `#lib/palette-view.ts`.
  Mounted once in the root layout, which also shows its trigger button in the top bar.
-->
<script lang="ts">
  import { Combobox, Dialog } from "@lilydesignsystem/svelte-headless";
  import { goto } from "$app/navigation";
  import { PlanRepository } from "#lib/api/plans.js";
  import { localeHref, t } from "#lib/i18n.svelte.js";
  import {
    RECENT_KEY,
    move,
    opensPalette,
    parseRecent,
    planItem,
    pushRecent,
    rows,
    type PaletteItem,
  } from "#lib/palette-view.js";

  let open = $state(false);
  let query = $state("");
  let found = $state<PaletteItem[]>([]);
  let failed = $state(false);
  let recent = $state<PaletteItem[]>([]);
  let root = $state<HTMLElement | null>(null);

  const commands = $derived<PaletteItem[]>([
    { id: "cmd:new-plan", kind: "command", label: t("c.CommandPalette.cmd_new_plan"), href: "/plans/new" },
    { id: "cmd:my-work", kind: "command", label: t("c.CommandPalette.cmd_my_work"), href: "/me" },
    { id: "cmd:escalations", kind: "command", label: t("c.CommandPalette.cmd_escalations"), href: "/escalations" },
    { id: "cmd:dashboard", kind: "command", label: t("c.CommandPalette.cmd_dashboard"), href: "/dashboard" },
  ]);
  const shown = $derived(rows(query, commands, found, recent));
  // The first row is active as soon as there is a query, so Enter goes to the best match; the arrow keys move it
  // (a writable derived value: it starts again whenever the rows change).
  let active = $derived(shown.length === 0 || query.trim() === "" ? -1 : 0);
  const KIND = {
    command: "c.CommandPalette.kind_command",
    plan: "c.CommandPalette.kind_plan",
    recent: "c.CommandPalette.kind_recent",
  } as const;

  function load() {
    try {
      recent = parseRecent(localStorage.getItem(RECENT_KEY));
    } catch {
      recent = [];
    }
  }
  function save() {
    try {
      localStorage.setItem(RECENT_KEY, JSON.stringify(recent));
    } catch {
      /* storage is a convenience: the palette works without it */
    }
  }

  function show() {
    query = "";
    found = [];
    failed = false;
    load();
    open = true;
  }

  // Plan search: after a short pause, and only the latest answer counts.
  let ticket = 0;
  $effect(() => {
    const q = query.trim();
    const mine = ++ticket;
    if (!open || q.length < 2) {
      found = [];
      failed = false;
      return;
    }
    const timer = setTimeout(async () => {
      try {
        const hits = await PlanRepository.withFetch().search(q);
        if (mine === ticket) {
          found = hits.slice(0, 20).map(planItem);
          failed = false;
        }
      } catch {
        if (mine === ticket) {
          found = [];
          failed = true;
        }
      }
    }, 200);
    return () => clearTimeout(timer);
  });


  // The input is inside Lily's Combobox, so the active row is announced by setting aria-activedescendant on it.
  $effect(() => {
    const input = root?.querySelector<HTMLInputElement>('input[role="combobox"]');
    if (!input) return;
    const id = active >= 0 ? `palette-row-${active}` : null;
    if (id) input.setAttribute("aria-activedescendant", id);
    else input.removeAttribute("aria-activedescendant");
  });
  $effect(() => {
    if (open) root?.querySelector<HTMLInputElement>('input[role="combobox"]')?.focus();
  });
  $effect(() => {
    if (active >= 0) document.getElementById(`palette-row-${active}`)?.scrollIntoView?.({ block: "nearest" });
  });

  function choose(item: PaletteItem) {
    recent = pushRecent(recent, item);
    save();
    open = false;
    void goto(localeHref(item.href));
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "ArrowDown" || e.key === "ArrowUp") {
      e.preventDefault();
      active = move(active, e.key, shown.length);
    } else if (e.key === "Enter") {
      const item = shown[active];
      if (item) {
        e.preventDefault();
        choose(item);
      }
    }
  }

  function onwindowkeydown(e: KeyboardEvent) {
    if (opensPalette(e)) {
      e.preventDefault();
      if (open) open = false;
      else show();
    }
  }
</script>

<svelte:window onkeydown={onwindowkeydown} />

<button
  type="button"
  class="session-button palette-trigger"
  aria-haspopup="dialog"
  aria-keyshortcuts="Control+K Meta+K"
  aria-label={t("c.CommandPalette.open")}
  onclick={show}
>{t("c.CommandPalette.button")}</button>

<Dialog class="palette-dialog" bind:open label={t("c.CommandPalette.title")}>
  <div bind:this={root} class="palette">
    <Combobox
      class="palette-combobox"
      label={t("c.CommandPalette.label")}
      bind:value={query}
      open={true}
      placeholder={t("c.CommandPalette.placeholder")}
      {onkeydown}
    >
      {#each shown as item, i (item.id)}
        <div
          id={`palette-row-${i}`}
          role="option"
          tabindex="-1"
          class="palette-row"
          class:active={i === active}
          aria-selected={i === active}
          onclick={() => choose(item)}
          onkeydown={() => {}}
        >
          <span class="palette-label">{item.label}</span>
          <span class="palette-kind">{t(KIND[item.kind])}</span>
        </div>
      {/each}
    </Combobox>
    <p class="palette-status" role="status">
      {#if failed}
        {t("c.CommandPalette.failed")}
      {:else if shown.length === 0}
        {t("c.CommandPalette.none")}
      {:else}
        {t(shown.length === 1 ? "c.CommandPalette.count_one" : "c.CommandPalette.count_other", { n: shown.length })}
      {/if}
    </p>
  </div>
</Dialog>

<style>
  :global(dialog.palette-dialog) {
    position: fixed;
    inset: 0;
    margin: 10vh auto auto;
    inline-size: min(36rem, calc(100vw - 2rem));
    block-size: fit-content;
    max-block-size: 80vh;
    padding: 0.75rem;
    border: 2px solid currentColor;
    border-radius: 0.5rem;
    background: var(--mxi-color-surface, Canvas);
    color: var(--mxi-color-fg, CanvasText);
    z-index: 1000;
  }
  .palette :global(input[role="combobox"]) {
    inline-size: 100%;
    box-sizing: border-box;
    padding: 0.5rem;
    font: inherit;
  }
  .palette :global([role="listbox"]) {
    margin-block-start: 0.5rem;
    max-block-size: 50vh;
    overflow-y: auto;
  }
  .palette-row {
    display: flex;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.4rem 0.5rem;
    cursor: pointer;
    border-inline-start: 4px solid transparent;
  }
  /* The active row is marked by a bar and bold text, not by colour alone. */
  .palette-row.active {
    border-inline-start-color: currentColor;
    font-weight: 700;
    background: var(--mxi-color-hover, color-mix(in srgb, currentColor 10%, transparent));
  }
  .palette-kind {
    opacity: 0.8;
    font-size: 0.85em;
  }
  .palette-status {
    margin: 0.5rem 0 0;
    font-size: 0.85em;
  }
</style>
