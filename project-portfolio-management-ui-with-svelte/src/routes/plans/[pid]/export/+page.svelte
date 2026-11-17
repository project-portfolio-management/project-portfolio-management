<!--
  Export to Microsoft Project (`/plans/[pid]/export`, PJ-10, NC-1): the plan's tasks as an MSPDI XML file, the format
  Microsoft Project and other scheduling tools read. The file is the service's own download (`Content-Disposition:
  attachment`), so following the link saves it and the page never draws it. No `download` attribute: the
  service's header names the file, as for attachments. The page says what the file carries and what it does
  not, as the service writes into the file itself, and the limit on its size.
-->
<script lang="ts">
  import { page } from "$app/state";
  import { API_BASE_URL } from "#lib/config.js";
  import { localeHref, t } from "#lib/i18n.svelte.js";

  const pid = $derived(page.params.pid ?? "");
</script>

<svelte:head><title>{t("p.plan_export.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.plan_export.title")}</h1>
<p>{t("p.plan_export.intro")}</p>

<section aria-labelledby="carried-h">
  <h2 id="carried-h">{t("p.plan_export.carried_heading")}</h2>
  <p>{t("p.plan_export.carried")}</p>
  <h2>{t("p.plan_export.not_carried_heading")}</h2>
  <p>{t("p.plan_export.not_carried")}</p>
  <p class="small muted">{t("p.plan_export.limit")}</p>
  <p class="small muted">{t("p.plan_export.not_checked")}</p>
</section>

<p>
  <a class="button primary" href={`${API_BASE_URL}/api/plans/${pid}/export/mspdi`}>
    {t("p.plan_export.download")}
  </a>
</p>
<p class="small"><a href={localeHref(`/plans/${pid}/import`)}>{t("p.plan_export.to_import")}</a></p>

<style>
  section {
    margin-block: 1rem 1.5rem;
  }
</style>
