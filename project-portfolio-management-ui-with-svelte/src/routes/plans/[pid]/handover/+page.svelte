<!--
  Handover (`/plans/[pid]/handover`): when someone leaves (or changes role),
  preview what they hold on the plan that is still live — unfinished tasks, live
  bookings, roles, open risks — then hand it to someone else or release it.
  Nothing changes until you commit, and a commit is refused if what they hold
  changed since the preview. About people, so it needs the permission to manage
  people.
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import type { DataGridCellContext, DataGridColumn } from "@lilydesignsystem/svelte-data-grid";
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type HandoverPreview } from "#lib/api/ppm.js";
  import { HOLDING_TEXT, handoverBody, handoverText } from "#lib/onboarding-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let from = $state("");
  let to = $state("");
  let reason = $state("");
  let preview = $state<HandoverPreview | null>(null);
  let status = $state("");
  let busy = $state(false);

  function reset() {
    preview = null;
  }
  async function run(e: SubmitEvent) {
    e.preventDefault();
    const body = handoverBody(from, to);
    if ("error" in body) return void (status = body.error);
    busy = true;
    try {
      preview = await ppm.handoverPreview(pid, body);
      status = "";
    } catch (err) {
      preview = null;
      status = err instanceof ApiError && err.status === 403 ? t("p.plans.pid.handover.handover_is_about_people_so") : err instanceof Error ? err.message : t("p.plans.pid.handover.could_not_preview");
    } finally {
      busy = false;
    }
  }
  async function commit() {
    if (!preview) return;
    busy = true;
    try {
      const r = await ppm.handoverCommit(pid, { from_person: preview.from_person, to_person: preview.to_person, reason: reason || undefined }, preview.handover_etag);
      status = t(r.audit_entries === 1 ? "p.plans.pid.handover.done_one" : "p.plans.pid.handover.done_other", { count: r.audit_entries });
      preview = null;
      reason = "";
    } catch (err) {
      if (err instanceof ApiError && err.status === 409) {
        preview = null;
        status = t("p.plans.pid.handover.what_this_person_holds_changed");
      } else status = err instanceof Error ? err.message : t("p.plans.pid.handover.could_not_commit");
    } finally {
      busy = false;
    }
  }
  function discard() {
    preview = null;
    status = t("p.plans.pid.handover.discarded_nothing_was_changed");
  }

  const columns: DataGridColumn[] = $derived([
    { id: "kind", header: t("p.plans.pid.handover.kind"), format: (v) => HOLDING_TEXT[String(v)] ?? String(v) },
    { id: "label", header: t("p.plans.pid.handover.what") },
  ]);
</script>

<svelte:head><title>{t("p.plans.pid.handover.handover_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.handover.handover")}</h1>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.handover.plan")}</a> · <a href={localeHref(`/plans/${pid}/onboarding`)}>{t("p.plans.pid.handover.onboarding_and_offboarding")}</a></p>

<form class="surface stack" onsubmit={run} aria-labelledby="hv-h">
  <h2 id="hv-h">{t("p.plans.pid.handover.who_is_handing_over")}</h2>
  <label>{t("p.plans.pid.handover.from")} <input bind:value={from} oninput={reset} placeholder={t("p.plans.pid.handover.person")} /></label>
  <label>{t("p.plans.pid.handover.to")} <input bind:value={to} oninput={reset} placeholder={t("p.plans.pid.handover.person_empty_releases_the_work")} /></label>
  <button class="button" class:primary={!preview} type="submit" disabled={busy}>{t("p.plans.pid.handover.preview")}</button>
</form>
{#if status}<p role="status" class="small">{status}</p>{/if}

{#if preview}
  <section class="surface stack" aria-labelledby="pv-h" data-testid="preview">
    <h2 id="pv-h">{t("p.plans.pid.handover.preview")}</h2>
    <p>{handoverText(preview.action, preview.holdings.length, preview.to_person)}</p>
    {#if preview.holdings.length}
      <DataTable label={t("p.plans.pid.handover.what_holds_that_is_still", { from_person: preview.from_person })} {columns} rows={preview.holdings} rowId={(r) => String(r.pid)} />
      <p class="small muted">{t("p.plans.pid.handover.the_new_person_s_capacity")}</p>
      <label>{t("p.plans.pid.handover.reason_recorded_with_each_item")} <input bind:value={reason} maxlength="500" /></label>
      <div class="row">
        <button class="button primary" type="button" onclick={commit} disabled={busy}>{preview.action === "reassign" ? t("p.plans.pid.handover.hand_over") : t("p.plans.pid.handover.release")}</button>
        <button class="button" type="button" onclick={discard} disabled={busy}>{t("p.plans.pid.handover.discard")}</button>
      </div>
    {/if}
  </section>
{/if}

<style>
  label {
    display: block;
  }
</style>
