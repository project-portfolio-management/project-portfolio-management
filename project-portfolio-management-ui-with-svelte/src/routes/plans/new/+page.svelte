<!--
  Create-plan route (`/plans/new`).

  Renders an empty PlanForm and, on submit, POSTs it then navigates to the
  new record's detail page. `kind` is an optional label chosen in the form.
-->
<script lang="ts">
  import { goto } from "$app/navigation";
  import PlanForm from "#lib/components/PlanForm.svelte";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { Plan } from "#lib/api/types.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";

  const repo = PlanRepository.withFetch();
  const initial: Plan = { name: "" };

  async function handleSubmit(record: Plan) {
    const created = await repo.create(record);
    await goto(localeHref(`/plans/${created.pid}`));
  }
</script>

<svelte:head><title>{t("p.plans.new.main_x", { t: t("new.title") })}</title></svelte:head>

<h1>{t("new.title")}</h1>
<PlanForm {initial} submitLabel={t("new.create")} onsubmit={handleSubmit} />
