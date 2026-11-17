<!--
  Plan templates (`/plan-templates`, LT-2, NC-1 slice 9): the saved templates the caller may see, then the built-in
  starters, each with its size and whether starting from it needs a start date. A template is saved from a plan's page
  ("Save as a template"); starting plans from one is on the template's own page.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { localeHref, t, tn } from "#lib/i18n.svelte.js";
  import { resource } from "#lib/resource.svelte.js";
  import { PpmClient, type PlanTemplateSummary } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  const loaded = resource<PlanTemplateSummary[]>(() => ppm.planTemplates(), "p.plan_templates.could_not_load_list");
  onMount(() => void loaded.reload());
  const groups = $derived([
    {
      id: "saved",
      rows: (loaded.data ?? []).filter((x) => !x.starter),
      heading: t("p.plan_templates.saved_heading"),
      none: t("p.plan_templates.none_saved"),
    },
    {
      id: "starters",
      rows: (loaded.data ?? []).filter((x) => x.starter),
      heading: t("p.plan_templates.starters_heading"),
      none: t("p.plan_templates.none_starters"),
    },
  ]);
</script>

<svelte:head><title>{t("p.plan_templates.title_ppm")}</title></svelte:head>

<h1>{t("p.plan_templates.title")}</h1>
<p class="small muted">{t("p.plan_templates.intro")}</p>
{#if loaded.error}<p class="banner" role="alert">{loaded.error}</p>{/if}

{#if loaded.data}
  {#each groups as group (group.id)}
    <section aria-labelledby={`${group.id}-h`}>
      <h2 id={`${group.id}-h`}>{group.heading}</h2>
      {#if group.rows.length === 0}
        <p class="muted">{group.none}</p>
      {:else}
        <ul class="templates">
          {#each group.rows as tpl (tpl.pid)}
            <li>
              <a href={localeHref(`/plan-templates/${tpl.pid}`)}>{tpl.name}</a>
              <span class="small muted">
                {tn("p.plan_templates.plans_count", tpl.plans, { count: tpl.plans })}{tpl.needs_start_date
                  ? ` · ${t("p.plan_templates.needs_start")}`
                  : ""}
              </span>
              {#if tpl.description}<p class="small">{tpl.description}</p>{/if}
            </li>
          {/each}
        </ul>
      {/if}
    </section>
  {/each}
{/if}

<style>
  section {
    margin-block-start: 1.5rem;
  }
  .templates {
    display: grid;
    gap: 0.75rem;
    padding-inline-start: 1.25rem;
  }
  .templates p {
    margin-block: 0.25rem 0;
  }
</style>
