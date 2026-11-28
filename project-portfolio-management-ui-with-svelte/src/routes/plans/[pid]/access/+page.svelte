<!--
  Who may see a plan (`/plans/[pid]/access`, RM-8, PL-7; NC-1 slice 12 part m). Private or not, and a private plan's
  members as user ids, one to a line (`GET`/`PUT /api/plans/{pid}/access`: the whole setting, sent only when it
  changed; the service adds the person saving as a member when making a plan private). Its guests: outside
  collaborators granted read or comment access, with an optional end, listed live first
  (`GET`/`POST /api/plans/{pid}/guests`), and revoked after a question (`POST …/guests/{grant}/revoke`). Every problem
  is said first. Rules in `src/lib/plan-access-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatDateTime } from "#lib/locale-format.js";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient } from "#lib/api/ppm.js";
  import {
    GUEST_ACCESS,
    accessChange,
    grantChange,
    grantState,
    membersText,
    sortedGrants,
    type GuestGrant,
    type PlanAccess,
  } from "#lib/plan-access-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let access = $state<PlanAccess | null>(null);
  let grants = $state<GuestGrant[] | null>(null);
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let problems = $state<string[]>([]);
  let notice = $state<string | null>(null);
  let busy = $state(false);
  let isPrivate = $state(false);
  let members = $state("");
  let guest = $state("");
  let guestAccess = $state<string>("read");
  let expires = $state("");

  async function load() {
    try {
      const [a, g] = await Promise.all([ppm.planAccess(pid), ppm.planGuests(pid)]);
      access = a;
      isPrivate = a.private;
      members = membersText(a.members);
      grants = g;
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.plan_access.could_not_load");
    }
  }
  onMount(load);

  async function run(work: () => Promise<unknown>, done: string) {
    busy = true;
    error = notice = null;
    try {
      await work();
      notice = done;
      await load();
      return true;
    } catch (err) {
      const detail = err instanceof ApiError ? JSON.stringify(err.body ?? "") : "";
      error =
        err instanceof ApiError && err.status === 422 && detail.includes("sub_not_addressable")
          ? t("p.plan_access.not_addressable")
          : errorText(err, "p.plan_access.could_not_save");
      return false;
    } finally {
      busy = false;
    }
  }

  async function saveAccess(e: SubmitEvent) {
    e.preventDefault();
    if (!access) return;
    const r = accessChange(isPrivate, members, access);
    if (r === null) {
      error = null;
      notice = t("p.plan_access.unchanged");
      return;
    }
    if ("problems" in r) return void (problems = r.problems);
    problems = [];
    const body = r.body;
    await run(() => ppm.setPlanAccess(pid, body), body.private ? t("p.plan_access.saved_private") : t("p.plan_access.saved_open"));
  }

  async function grant(e: SubmitEvent) {
    e.preventDefault();
    const r = grantChange(guest, guestAccess, expires, new Date());
    if ("problems" in r) return void (problems = r.problems);
    problems = [];
    const body = r.body;
    if (await run(() => ppm.grantGuest(pid, body), t("p.plan_access.granted", { guest: body.guest_sub }))) {
      guest = "";
      expires = "";
    }
  }

  async function revoke(g: GuestGrant) {
    if (!(await askConfirm(t("p.plan_access.revoke_ask", { guest: g.guest_sub })))) return;
    await run(() => ppm.revokeGuest(pid, g.pid), t("p.plan_access.revoked", { guest: g.guest_sub }));
  }
</script>

<svelte:head><title>{t("p.plan_access.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.plan_access.title")}</h1>
<p class="lede">{t("p.plan_access.intro")}</p>
{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{#if error}<p class="banner error" role="alert" data-testid="access-error">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="access-notice">{notice}</p>{/if}
{#if problems.length > 0}
  <ul class="banner error" role="alert" data-testid="access-problems">
    {#each problems as p (p)}<li>{tDynamic(p)}</li>{/each}
  </ul>
{/if}

{#if access}
  <form class="surface stack" onsubmit={saveAccess} aria-labelledby="who-h" novalidate>
    <h2 id="who-h">{t("p.plan_access.who_h")}</h2>
    <p data-testid="access-state">{access.private ? t("p.plan_access.is_private") : t("p.plan_access.is_open")}</p>
    <label class="inline"><input type="checkbox" bind:checked={isPrivate} /> {t("p.plan_access.f_private")}</label>
    <label>{t("p.plan_access.f_members")} <textarea rows="4" bind:value={members} autocomplete="off"></textarea></label>
    <p class="small muted">{t("p.plan_access.members_note")}</p>
    <div><button class="button primary" type="submit" disabled={busy}>{t("p.plan_access.save")}</button></div>
  </form>
{/if}

{#if grants}
  <section class="surface stack" aria-labelledby="guests-h">
    <h2 id="guests-h">{t("p.plan_access.guests_h")}</h2>
    <p class="small muted">{t("p.plan_access.guests_intro")}</p>
    {#if grants.length === 0}
      <p class="muted" data-testid="no-guests">{t("p.plan_access.no_guests")}</p>
    {:else}
      <ul class="grants" data-testid="grants">
        {#each sortedGrants(grants) as g (g.pid)}
          <li>
            <code id={`g-${g.pid}`}>{g.guest_sub}</code>
            · {tDynamic(`p.plan_access.access_${g.access}`)}
            · {tDynamic(`p.plan_access.state_${grantState(g)}`)}
            · {g.expires_at ? t("p.plan_access.until", { date: formatDateTime(g.expires_at) }) : t("p.plan_access.no_end")}
            {#if g.granted_by}<span class="small muted"> · {t("p.plan_access.granted_by", { who: g.granted_by })}</span>{/if}
            {#if g.live}
              <button class="button small danger" type="button" disabled={busy} onclick={() => revoke(g)} aria-describedby={`g-${g.pid}`}
                >{t("p.plan_access.revoke")}</button
              >
            {/if}
          </li>
        {/each}
      </ul>
    {/if}
    <form class="stack" onsubmit={grant} aria-label={t("p.plan_access.grant_h")} novalidate>
      <h3>{t("p.plan_access.grant_h")}</h3>
      <label>{t("p.plan_access.f_guest")} <input bind:value={guest} autocomplete="off" /></label>
      <label
        >{t("p.plan_access.f_access")}
        <select bind:value={guestAccess}>
          {#each GUEST_ACCESS as a (a)}<option value={a}>{tDynamic(`p.plan_access.access_${a}`)}</option>{/each}
        </select>
      </label>
      <label>{t("p.plan_access.f_expires")} <input type="datetime-local" bind:value={expires} /></label>
      <div><button class="button" type="submit" disabled={busy}>{t("p.plan_access.grant")}</button></div>
    </form>
  </section>
{/if}

<style>
  .grants {
    display: grid;
    gap: 0.5rem;
    padding-inline-start: 1.25rem;
  }
</style>
