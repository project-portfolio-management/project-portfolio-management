<!--
  Save a plan as a template (`/plans/[pid]/save-as-template`, LT-2, NC-1 slice 9): the plan and the plans under it that
  the caller can read, with their tasks, components, milestones and workflows, are kept as a snapshot. No person is
  kept, and every date becomes a number of days after the start. Editing the plan later does not change the template.
  The answer says what was kept and how many plans were left out because the caller cannot read them.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tn } from "#lib/i18n.svelte.js";
  import { PpmClient, type SavedTemplate } from "#lib/api/ppm.js";
  import { TEMPLATE_NAME_MAX, nameProblem } from "#lib/template-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let name = $state("");
  let description = $state("");
  let error = $state<string | null>(null);
  let busy = $state(false);
  let saved = $state<SavedTemplate | null>(null);

  async function save(e: SubmitEvent) {
    e.preventDefault();
    error = null;
    saved = null;
    const problem = nameProblem(name);
    if (problem) {
      error = t(problem);
      return;
    }
    busy = true;
    try {
      const d = description.trim();
      saved = await ppm.saveAsTemplate(pid, { name: name.trim(), ...(d ? { description: d } : {}) });
    } catch (err) {
      error =
        err instanceof ApiError && err.status === 409
          ? t("p.plan_templates.name_taken")
          : errorText(err, "p.plan_templates.could_not_save");
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>{t("p.plan_templates.save_title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.plan_templates.save_title")}</h1>
<p class="small muted">{t("p.plan_templates.save_says")}</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}
{#if saved}
  <div class="banner success" role="status">
    <p>
      {t("p.plan_templates.saved", {
        plans: saved.plans,
        tasks: saved.tasks,
        milestones: saved.milestones,
      })}
    </p>
    {#if saved.omitted_unreadable > 0}
      <p>{tn("p.plan_templates.omitted", saved.omitted_unreadable, { count: saved.omitted_unreadable })}</p>
    {/if}
    <p><a href={localeHref(`/plan-templates/${saved.pid}`)}>{t("p.plan_templates.open_template")}</a></p>
  </div>
{/if}
<form class="stack" onsubmit={save}>
  <label>
    {t("p.plan_templates.f_name")}
    <input type="text" bind:value={name} maxlength={TEMPLATE_NAME_MAX} />
  </label>
  <label>
    {t("p.plan_templates.f_description")}
    <textarea bind:value={description} rows="3"></textarea>
  </label>
  <div class="row">
    <button class="button primary" type="submit" disabled={busy}>{t("p.plan_templates.save")}</button>
  </div>
</form>
