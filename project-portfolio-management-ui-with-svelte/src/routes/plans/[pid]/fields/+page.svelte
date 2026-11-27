<!--
  A plan's custom field values (`/plans/[pid]/fields`, RM-1; NC-1 slice 12 part n): the fields that apply to this
  plan's kind and their values, in `CustomFieldValues.svelte`, which a task's page shares.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { localeHref, t } from "#lib/i18n.svelte.js";
  import CustomFieldValues from "#lib/components/CustomFieldValues.svelte";

  const pid = $derived(page.params.pid ?? "");
</script>

<svelte:head><title>{t("p.custom_fields.values_title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.custom_fields.values_title")}</h1>
<p class="lede">{t("p.custom_fields.values_intro")}</p>
{#key pid}<CustomFieldValues appliesTo="plan" subjectPid={pid} />{/key}
