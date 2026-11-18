<!--
  How many people use what the plan delivered (`/plans/[pid]/benefits`, NC-1 slice 12 part j): `POST
  /api/plans/{pid}/adoption`, the active users against the users it is for over a window, with what counted as
  active. Checked first, every problem at once; the adoption above is read again after. Rules in
  `src/lib/okr-reads-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { PpmClient } from "#lib/api/ppm.js";
  import { adoptionBody, adoptionRefusalKey, emptyAdoptionForm, type AdoptionForm } from "#lib/okr-reads-view.js";

  let { pid, onsaved }: { pid: string; onsaved: () => void } = $props();

  const ppm = PpmClient.withFetch();
  let form = $state<AdoptionForm>(emptyAdoptionForm());
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  async function record(e: SubmitEvent) {
    e.preventDefault();
    notice = "";
    const r = adoptionBody(form);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k)).join(" ");
      return;
    }
    problem = null;
    busy = true;
    try {
      await ppm.recordAdoption(pid, r.body);
      notice = t("p.adoption.saved");
      form = { ...emptyAdoptionForm(), windowDays: form.windowDays, definition: form.definition };
      onsaved();
    } catch (err) {
      const key = adoptionRefusalKey(err);
      problem = key ? t(key) : errorText(err, "p.adoption.could_not");
    } finally {
      busy = false;
    }
  }
</script>

<h2 id="adoption-h">{t("p.adoption.heading")}</h2>
<p class="small muted">{t("p.adoption.says")}</p>
{#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
<p class="small" role="status">{notice}</p>
<form class="grid" novalidate onsubmit={record} aria-label={t("p.adoption.form")} data-testid="adoption-form">
  <label>{t("p.adoption.f_active")}<input type="number" min="0" step="1" bind:value={form.active} /></label>
  <label>{t("p.adoption.f_target")}<input type="number" min="1" step="1" bind:value={form.target} /></label>
  <label>{t("p.adoption.f_window")}<input type="number" min="1" step="1" bind:value={form.windowDays} /></label>
  <label>{t("p.adoption.f_definition")}<input type="text" bind:value={form.definition} /></label>
  <div><button class="button" type="submit" disabled={busy}>{t("p.adoption.record")}</button></div>
</form>

<style>
  .grid { display: grid; gap: 0.5rem; max-inline-size: 32rem; margin-block: 0.75rem; }
  label { display: grid; gap: 0.2rem; }
</style>
