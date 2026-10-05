<!--
  Display (`/display`): two choices that change how the app looks and reads —
  never what it can do.

  * Simple view puts the advanced destinations behind a "More" link. Nothing is
    removed: every page is still one step away and still works at its address.
    It is on by default for people whose role is the day-to-day worker, and a
    choice for everyone else; whatever you choose here stays chosen.
  * Dyslexia-friendly font uses OpenDyslexic with looser spacing.

  Choices are kept on this device and, when you are signed in, on your account.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { display } from "#lib/preferences.svelte.js";
  import { effectiveSimple } from "#lib/preferences.js";

  const data = $derived(page.data as { signedIn?: boolean; view?: string | null });
  const signedIn = $derived(data.signedIn === true);
  const simple = $derived(effectiveSimple(display.prefs, data.view));
  const chosen = $derived(typeof display.prefs.simple === "boolean");

  const STATUS: Record<string, string> = {
    idle: "",
    device: "Saved on this device.",
    account: "Saved to your account, so it follows you to other browsers.",
    "account-failed": "Could not save to your account just now; it is kept on this device.",
  };
</script>

<svelte:head><title>Display — PPM</title></svelte:head>

<h1>Display</h1>
<p class="small muted">How the app looks and reads. These never change what you can do.</p>

<section class="surface stack" aria-labelledby="simple-h">
  <h2 id="simple-h">Simple view</h2>
  <p>
    Keeps the everyday pages in front (My work, Plans, Board, Calendar, Tour) and puts the rest behind a
    “More” link. Every page is still there and still works.
  </p>
  <label class="check">
    <input type="checkbox" checked={simple} onchange={(e) => void display.set({ simple: e.currentTarget.checked }, signedIn)} />
    Use simple view
  </label>
  <p class="small muted">
    {#if chosen}
      You chose this.
      <button type="button" class="linklike" onclick={() => void display.set({ simple: undefined }, signedIn)}>
        Go back to the default for my role
      </button>
    {:else}
      This is the default for your role: {simple ? "on" : "off"}.
    {/if}
  </p>
</section>

<section class="surface stack" aria-labelledby="font-h">
  <h2 id="font-h">Dyslexia-friendly font</h2>
  <p>Uses the OpenDyslexic typeface, with wider spacing between letters, words and lines.</p>
  <label class="check">
    <input type="checkbox" checked={display.dyslexia} onchange={(e) => void display.set({ dyslexia: e.currentTarget.checked }, signedIn)} />
    Use the dyslexia-friendly font
  </label>
</section>

<p class="small" role="status" aria-live="polite">{STATUS[display.status]}</p>
{#if !signedIn}
  <p class="small muted">Sign in to keep these on your account as well as on this device.</p>
{/if}

<style>
  .stack {
    display: grid;
    gap: 0.6rem;
    margin: 1rem 0;
  }
  .check {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-weight: 600;
  }
  .linklike {
    background: none;
    border: 0;
    padding: 0;
    color: var(--mxi-color-primary);
    text-decoration: underline;
    cursor: pointer;
    font: inherit;
  }
</style>
