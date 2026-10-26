<!--
  PaletteTrigger (BR-14) — the menu entry that opens the command palette. The dialog itself is `CommandPalette`, mounted
  once in the root layout (it cannot sit inside the collapsed menu). Ctrl or Cmd with K opens it from anywhere.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { requestPalette } from "#lib/palette.svelte.js";
</script>

<button type="button" class="palette-trigger" aria-haspopup="dialog" aria-keyshortcuts="Control+K Meta+K" onclick={requestPalette}>
  {t("c.CommandPalette.open")}
</button>

<style>
  /* In the menu it reads as one of its entries. */
  .palette-trigger {
    background: none;
    border: 0;
    padding: 0;
    color: inherit;
    font: inherit;
    cursor: pointer;
    text-decoration: underline;
  }
</style>
