<!--
  ConfirmHost — the Lily AlertDialog that answers `askConfirm()`.
  Mounted once in the root layout. Headless, so focus and Escape are done here: Cancel takes focus
  when the question opens (the safe default for a destructive question) and Escape answers "no".
-->
<script lang="ts">
  import { AlertDialog } from "@lilydesignsystem/svelte-headless";
  import { answerConfirm, confirmState } from "#lib/confirm.svelte.js";
  import { t } from "#lib/i18n.svelte.js";

  const open = $derived(confirmState.pending !== null);
  let cancel = $state<HTMLButtonElement | null>(null);
  $effect(() => {
    if (open) cancel?.focus();
  });
  const onkeydown = (e: KeyboardEvent) => {
    if (open && e.key === "Escape") answerConfirm(false);
  };
</script>

<svelte:window {onkeydown} />

<AlertDialog
  class="confirm-dialog"
  open={open}
  title={t("c.ConfirmHost.title")}
  description={confirmState.pending?.message}
>
  <div class="confirm-actions">
    <button class="button" type="button" bind:this={cancel} onclick={() => answerConfirm(false)}>{t("c.ConfirmHost.cancel")}</button>
    <button class="button danger" type="button" onclick={() => answerConfirm(true)}>{t("c.ConfirmHost.ok")}</button>
  </div>
</AlertDialog>
