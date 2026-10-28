<!--
  A link to the sibling app that owns a `worker:` or `contact:` reference (cross-app bridges), or
  nothing when no bridge applies or none is configured. See `#lib/bridge.ts`.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { bridgeLink, type Bridges } from "#lib/bridge.js";
  import { t } from "#lib/i18n.svelte.js";

  let { ref, name }: { ref: string; name: string } = $props();
  const bridges = $derived(page.data.bridges as Bridges | undefined);
  const link = $derived(bridgeLink(ref, bridges));
</script>

{#if link}
  <a
    class="bridge"
    href={link.href}
    target="_blank"
    rel="noopener noreferrer"
    aria-label={link.kind === "workforce"
      ? t("p.bridge.open_workforce", { name })
      : t("p.bridge.open_contacts", { name })}>↗</a
  >
{/if}
