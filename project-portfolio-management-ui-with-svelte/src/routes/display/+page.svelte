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
  import { t } from "#lib/i18n.svelte.js";
  import { page } from "$app/state";
  import { display } from "#lib/preferences.svelte.js";
  import { effectiveSimple } from "#lib/preferences.js";
  import { buildNavItems } from "#lib/nav-items.js";

  const data = $derived(page.data as { signedIn?: boolean; view?: string | null });
  const signedIn = $derived(data.signedIn === true);
  const simple = $derived(effectiveSimple(display.prefs, data.view));
  const chosen = $derived(typeof display.prefs.simple === "boolean");
  // BR-17: every section of the top bar except the way home, ticked unless this person hid it.
  const sections = $derived(buildNavItems().filter((i) => i.href !== "/"));
  const hidden = $derived(new Set(display.prefs.hidden ?? []));

  const STATUS: Record<string, string> = {
    idle: "",
    get device() { return t("p.display.saved_on_this_device"); },
    get account() { return t("p.display.saved_to_your_account_so"); },
    get "account-failed"() { return t("p.display.could_not_save_to_your"); },
  };
</script>

<svelte:head><title>{t("p.display.display_ppm")}</title></svelte:head>

<h1>{t("p.display.display")}</h1>
<p class="small muted">{t("p.display.how_the_app_looks_and")}</p>

<section class="surface stack" aria-labelledby="simple-h">
  <h2 id="simple-h">{t("p.display.simple_view")}</h2>
  <p>
    {t("p.display.keeps_the_everyday_pages_in")}
  </p>
  <label class="check">
    <input type="checkbox" checked={simple} onchange={(e) => void display.set({ simple: e.currentTarget.checked }, signedIn)} />
    {t("p.display.use_simple_view")}
  </label>
  <p class="small muted">
    {#if chosen}
      {t("p.display.you_chose_this")}
      <button type="button" class="linklike" onclick={() => void display.set({ simple: undefined }, signedIn)}>
        {t("p.display.go_back_to_the_default")}
      </button>
    {:else}
      {t("p.display.this_is_the_default_for", { v: simple ? t("p.common.on") : t("p.common.off") })}
    {/if}
  </p>
</section>

<section class="surface stack" aria-labelledby="font-h">
  <h2 id="font-h">{t("p.display.dyslexia_friendly_font")}</h2>
  <p>{t("p.display.uses_the_opendyslexic_typeface_with")}</p>
  <label class="check">
    <input type="checkbox" checked={display.dyslexia} onchange={(e) => void display.set({ dyslexia: e.currentTarget.checked }, signedIn)} />
    {t("p.display.use_the_dyslexia_friendly_font")}
  </label>
</section>

<section class="surface stack" aria-labelledby="sections-h">
  <h2 id="sections-h">{t("p.display.sections_title")}</h2>
  <p>{t("p.display.sections_help")}</p>
  <fieldset class="sections">
    <legend>{t("p.display.sections_legend")}</legend>
    {#each sections as item (item.href)}
      <label class="check">
        <input
          type="checkbox"
          checked={!hidden.has(item.href)}
          onchange={(e) => void display.setSectionHidden(item.href, !e.currentTarget.checked, signedIn)}
        />
        {item.label}
      </label>
    {/each}
  </fieldset>
  <p class="small muted">
    {#if hidden.size === 0}
      {t("p.display.sections_none_hidden")}
    {:else}
      {hidden.size === 1 ? t("p.display.sections_hidden_count_one", { n: 1 }) : t("p.display.sections_hidden_count_other", { n: hidden.size })}
      <button type="button" class="linklike" onclick={() => void display.showAllSections(signedIn)}>
        {t("p.display.sections_show_all")}
      </button>
    {/if}
  </p>
</section>

<p class="small" role="status" aria-live="polite">{STATUS[display.status]}</p>
{#if !signedIn}
  <p class="small muted">{t("p.display.sign_in_to_keep_these")}</p>
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
  .sections {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(14rem, 1fr));
    gap: 0.4rem 1rem;
    border: 0;
    padding: 0;
  }
  .sections .check {
    font-weight: 400;
  }
  .linklike {
    background: none;
    border: 0;
    padding: 0;
    color: var(--mxi-color-link);
    text-decoration: underline;
    cursor: pointer;
    font: inherit;
  }
</style>
