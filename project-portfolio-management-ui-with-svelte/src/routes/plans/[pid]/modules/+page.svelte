<!--
  A plan's tools (`/plans/[pid]/modules`, RM-7, NC-1 slice 8): which feature modules this plan shows. A module that is
  off is hidden from the plan page and the service answers its plan routes with 404; nothing is deleted. Three ways to
  change it, as the service offers: a chosen set, the personal preset, or back to the deployment's default. The page
  does not know whether the caller may change it; the service decides, and a refusal is said in the catalogue's words.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t } from "#lib/i18n.svelte.js";
  import { PpmClient, type PlanModules } from "#lib/api/ppm.js";
  import { MODULE_LABELS, modulesBody, toggled } from "#lib/module-settings.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let current = $state<PlanModules | null>(null);
  let chosen = $state<string[]>([]);
  let error = $state<string | null>(null);
  let notice = $state("");
  let saving = $state(false);

  const order = $derived(current?.available.map((m) => m.key) ?? []);
  const label = (key: string): string => {
    const k = MODULE_LABELS[key];
    return k ? t(k) : key;
  };
  const names = (keys: readonly string[]): string =>
    keys.length > 0 ? keys.map(label).join(", ") : t("p.plan_modules.no_tools");

  function show(m: PlanModules) {
    current = m;
    chosen = [...m.modules];
  }

  async function load() {
    error = null;
    try {
      show(await ppm.planModules(pid));
    } catch (err) {
      error = errorText(err, "p.plan_modules.could_not_load");
    }
  }
  onMount(load);

  async function send(body: Parameters<PpmClient["setPlanModules"]>[1] | null) {
    error = null;
    notice = "";
    if (body === null) {
      notice = t("p.plan_modules.unchanged");
      return;
    }
    saving = true;
    try {
      show(await ppm.setPlanModules(pid, body));
      notice = t("p.plan_modules.saved");
    } catch (err) {
      error = errorText(err, "p.plan_modules.could_not_save");
    } finally {
      saving = false;
    }
  }

  function save(e: SubmitEvent) {
    e.preventDefault();
    if (current) void send(modulesBody(chosen, current));
  }
</script>

<svelte:head><title>{t("p.plan_modules.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.plan_modules.title")}</h1>
<p class="small muted">{t("p.plan_modules.intro")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>

{#if current}
  <p>
    <strong>
      {current.source === "plan" ? t("p.plan_modules.source_plan") : t("p.plan_modules.source_default")}
    </strong>
  </p>
  <form class="stack" onsubmit={save} aria-labelledby="tools-legend">
    <fieldset>
      <legend id="tools-legend">{t("p.plan_modules.legend")}</legend>
      {#each current.available as m (m.key)}
        <label class="check">
          <input
            type="checkbox"
            checked={chosen.includes(m.key)}
            disabled={saving}
            onchange={(e) => (chosen = toggled(chosen, m.key, e.currentTarget.checked, order))}
          />
          {label(m.key)}
        </label>
      {/each}
    </fieldset>
    {#if chosen.length === 0}<p class="small" role="note">{t("p.plan_modules.none_chosen")}</p>{/if}
    <div class="row">
      <button class="button" type="submit" disabled={saving}>{t("p.plan_modules.save")}</button>
    </div>
  </form>

  <section aria-labelledby="presets-h">
    <h2 id="presets-h">{t("p.plan_modules.presets_heading")}</h2>
    <p class="small muted">{t("p.plan_modules.personal_is", { names: names(current.personal_preset) })}</p>
    <button class="button" type="button" disabled={saving} onclick={() => send({ preset: "personal" })}>
      {t("p.plan_modules.use_personal")}
    </button>
    <p class="small muted">{t("p.plan_modules.default_is", { names: names(current.default) })}</p>
    <button
      class="button"
      type="button"
      disabled={saving || current.source === "default"}
      onclick={() => send({ reset: true })}
    >
      {t("p.plan_modules.use_default")}
    </button>
  </section>
{/if}

<style>
  section {
    margin-block-start: 2rem;
  }
  fieldset {
    display: grid;
    gap: 0.5rem;
  }
  .check {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }
  .check input {
    inline-size: 1.5rem;
    block-size: 1.5rem;
    margin: 0;
  }
</style>
