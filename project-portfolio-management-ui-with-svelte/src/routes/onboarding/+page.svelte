<!--
  Operator onboarding guide (`/onboarding`, T-28p): a role-by-role
  "first hour" walkthrough. Pure static content — no API call, so
  nothing here can fail to load.

  Deliberately makes no time-to-productivity claim: nobody has
  measured how long a first hour with this guide actually takes.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { ONBOARDING_ROLES } from "#lib/onboarding.js";
</script>

<svelte:head><title>{t("p.onboarding.onboarding_ppm")}</title></svelte:head>

<h1>{t("p.onboarding.operator_onboarding")}</h1>
<p class="small muted">
  {t("p.onboarding.a_role_by_role_first")}
</p>

{#each ONBOARDING_ROLES as role (role.id)}
  <section class="role">
    <h2>{role.title}</h2>
    <p class="small muted">{role.summary}</p>
    <ol>
      {#each role.steps as step, i (step.route + i)}
        <li>
          <a href={localeHref(step.route)}>{step.label}</a>
          <p class="small muted">{step.note}</p>
        </li>
      {/each}
    </ol>
  </section>
{/each}

<style>
  .role {
    margin-block: 1.5rem;
  }
  ol {
    padding-inline-start: 1.5rem;
  }
  li {
    margin-block-end: 0.75rem;
  }
</style>
