<!--
  People's primary roles (`/resources`, NC-1 slice 12 part n): `GET /api/resource-roles` (everyone's for a resource
  manager or an administrator, else just the reader's own) and `PUT /api/resource-roles` to set or clear one (a blank
  role clears it). A role decides which row of the resource plan counts a person's declared capacity as supply, so the
  plan is read again after a change. The person and the role are checked before anything is sent; a refusal without
  the right (403) is the catalogue's sentence. Rules in `src/lib/component-view.ts` (`leadOk` for the reference).
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type ResourceRole } from "#lib/api/ppm.js";
  import { leadOk } from "#lib/component-view.js";

  let { onchanged }: { onchanged?: () => void | Promise<void> } = $props();
  const ppm = PpmClient.withFetch();
  const ROLE_MAX = 60;

  let roles = $state<ResourceRole[] | null>(null);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);
  let person = $state("");
  let role = $state("");
  let busy = $state(false);

  async function load() {
    try {
      roles = await ppm.resourceRoles();
    } catch (err) {
      error = errorText(err, "p.resource_roles.could_not_load");
    }
  }
  onMount(load);

  async function set(e: SubmitEvent) {
    e.preventDefault();
    notice = null;
    const p = person.trim();
    const r = role.trim();
    if (!leadOk(p)) {
      error = t("p.resource_roles.person_bad");
      return;
    }
    if ([...r].length > ROLE_MAX) {
      error = t("p.resource_roles.role_too_long");
      return;
    }
    busy = true;
    try {
      await ppm.setResourceRole({ person_ref: p, role: r || null });
      notice = r ? t("p.resource_roles.set_ok", { person: p, role: r }) : t("p.resource_roles.cleared_ok", { person: p });
      error = null;
      person = role = "";
      await load();
      await onchanged?.();
    } catch (err) {
      error =
        err instanceof ApiError && err.status === 403
          ? t("p.resource_roles.forbidden")
          : errorText(err, "p.resource_roles.could_not_set");
    } finally {
      busy = false;
    }
  }
</script>

<section aria-labelledby="roles-h" data-testid="resource-roles">
  <h2 id="roles-h">{t("p.resource_roles.heading")}</h2>
  <p class="small muted">{t("p.resource_roles.says")}</p>
  {#if error}<p class="banner error" role="alert">{error}</p>{/if}
  {#if notice}<p class="banner" role="status">{notice}</p>{/if}
  {#if roles && roles.length === 0}
    <p class="small">{t("p.resource_roles.none")}</p>
  {:else if roles}
    <ul>
      {#each roles as r (r.person_ref)}<li><code>{r.person_ref}</code>: {r.role}</li>{/each}
    </ul>
  {/if}
  <form class="row" onsubmit={set} aria-label={t("p.resource_roles.form")}>
    <label>{t("p.resource_roles.person")} <input bind:value={person} /></label>
    <label>{t("p.resource_roles.role")} <input bind:value={role} maxlength={ROLE_MAX} /></label>
    <button class="button" type="submit" disabled={busy}>{t("p.resource_roles.set")}</button>
  </form>
</section>

<style>
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    align-items: end;
  }
  label {
    display: grid;
    gap: 0.2rem;
    font-size: 0.85rem;
  }
</style>
