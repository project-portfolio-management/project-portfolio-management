<!--
  Announcements (`/announcements`): a feed of short messages to everyone —
  pinned first, then urgent, important, info, then newest. Severity is a word
  and a shape, never colour alone. Publishing needs a permission; someone
  without it sees the feed and a plain note, not a broken form.
-->
<script lang="ts">
  import { isAbsent } from "#lib/absent.js";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type Announcement, type AnnouncementSeverity } from "#lib/api/ppm.js";
  import { SEVERITY_SYMBOL, SEVERITY_TEXT, ageText, stateLabel } from "#lib/announcement-view.js";

  const ppm = PpmClient.withFetch();
  let items = $state<Announcement[] | null>(null);
  let error = $state<string | null>(null);
  let manage = $state(false);
  const now = new Date();

  async function refresh() {
    try {
      // A publisher also sees scheduled and expired ones; anyone else gets the live feed.
      try {
        items = (await ppm.announcements({ include: "all" })).announcements;
        manage = true;
      } catch (err) {
        // Only "you may not see those" falls back to the live feed; a fault is shown (CQ-19).
        if (!isAbsent(err)) throw err;
        items = (await ppm.announcements()).announcements;
        manage = false;
      }
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.announcements.could_not_load_the_announcements");
    }
  }
  onMount(refresh);

  let title = $state("");
  let body = $state("");
  let severity = $state<AnnouncementSeverity>("info");
  let pinned = $state(false);
  let expires = $state("");
  async function publish(e: SubmitEvent) {
    e.preventDefault();
    try {
      await ppm.publishAnnouncement({
        title,
        body,
        severity,
        pinned,
        expires_at: expires ? new Date(expires).toISOString() : null,
      });
      title = body = expires = "";
      severity = "info";
      pinned = false;
      await refresh();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.announcements.could_not_publish");
    }
  }
  async function retract(a: Announcement) {
    try {
      await ppm.retractAnnouncement(a.pid);
      await refresh();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.announcements.could_not_retract");
    }
  }
</script>

<svelte:head><title>{t("p.announcements.announcements_ppm")}</title></svelte:head>

<h1>{t("p.announcements.announcements")}</h1>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}

{#if items === null && !error}
  <p class="muted">{t("p.announcements.loading")}</p>
{:else if items && items.length === 0}
  <p class="muted" data-testid="no-announcements">{t("p.announcements.nothing_has_been_announced")}</p>
{:else if items}
  <ul class="feed stack" data-testid="announcements">
    {#each items as a (a.pid)}
      <li class="surface">
        <h2>
          <span aria-hidden="true">{SEVERITY_SYMBOL[a.severity]}</span>
          {a.title}
        </h2>
        <p class="small muted">
          {SEVERITY_TEXT[a.severity]}{a.pinned ? t("p.announcements.pinned") : ""}{stateLabel(a) ? ` · ${stateLabel(a)}` : ""} · {ageText(a.publish_at, now)}
        </p>
        <p>{a.body}</p>
        {#if manage}
          <button type="button" class="button danger" onclick={() => retract(a)} aria-label={`Retract ${a.title}`}>{t("p.announcements.retract")}</button>
        {/if}
      </li>
    {/each}
  </ul>
{/if}

{#if manage}
  <form class="surface stack" onsubmit={publish} aria-labelledby="publish-h">
    <h2 id="publish-h">{t("p.announcements.publish_an_announcement")}</h2>
    <label>{t("p.announcements.title")} <input bind:value={title} required maxlength="120" /></label>
    <label>{t("p.announcements.message")} <textarea bind:value={body} required maxlength="4000" rows="3"></textarea></label>
    <label>
      {t("p.announcements.severity")}
      <select bind:value={severity}>
        <option value="info">{t("p.announcements.info")}</option>
        <option value="important">{t("p.announcements.important")}</option>
        <option value="urgent">{t("p.announcements.urgent")}</option>
      </select>
    </label>
    <label><input type="checkbox" bind:checked={pinned} /> {t("p.announcements.pin_to_the_top")}</label>
    <label>{t("p.announcements.expires_optional")} <input type="datetime-local" bind:value={expires} /></label>
    <button class="button primary" type="submit">{t("p.announcements.publish")}</button>
  </form>
{:else if items}
  <p class="muted small">{t("p.announcements.publishing_needs_the_announcements_p")}</p>
{/if}

<style>
  .feed {
    list-style: none;
    padding: 0;
  }
  h2 {
    font-size: 1.1rem;
  }
  label {
    display: block;
  }
</style>
