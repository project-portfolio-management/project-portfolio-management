<!--
  Edit-plan route (`/plans/[pid]/edit`).

  Loads the existing plan, seeds a PlanForm with it, and PUTs the edited
  record on submit before returning to the detail page.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import PlanForm from "#lib/components/PlanForm.svelte";
  import { PlanRepository } from "#lib/api/plans.js";
  import type { Plan } from "#lib/api/types.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";

  // Reactive, so the form follows the URL to another plan (CQ-18).
  const pid = $derived(page.params.pid ?? "");
  const repo = PlanRepository.withFetch();

  let record = $state<Plan | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  let latest = 0;
  async function load(id: string) {
    const mine = ++latest;
    record = null;
    loading = true;
    error = null;
    try {
      const found = await repo.get(id);
      if (mine === latest) record = found;
    } catch (err) {
      if (mine === latest) error = errorText(err, "edit.notFound");
    } finally {
      if (mine === latest) loading = false;
    }
  }
  $effect(() => {
    void load(pid);
  });

  async function handleSubmit(updated: Plan) {
    await repo.update(pid, updated);
    await goto(localeHref(`/plans/${pid}`));
  }
</script>

<svelte:head><title>{t("p.plans.pid.edit.main_x", { t: t("edit.title") })}</title></svelte:head>

<h1>{t("edit.title")}</h1>

{#if loading}
  <p>{t("edit.loading")}</p>
{:else if error}
  <p class="banner" role="alert">{error}</p>
{:else if record}
  {#key pid}<PlanForm initial={record} submitLabel={t("edit.saveChanges")} onsubmit={handleSubmit} />{/key}
{/if}
