<!--
  A term or acronym with its plain-language gloss (LT-12): `<abbr title="…">`.
  An unknown term renders as plain text rather than a tooltip that says nothing.
-->
<script lang="ts">
  import { glossFor } from "#lib/glossary.js";

  let { t }: { t: string } = $props();
  const gloss = $derived(glossFor(t));
</script>

{#if gloss}<abbr title={gloss}>{t}</abbr>{:else}{t}{/if}
