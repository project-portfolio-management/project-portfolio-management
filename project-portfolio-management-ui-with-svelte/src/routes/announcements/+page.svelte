<!--
  Announcements (`/announcements`): a feed of short messages to everyone —
  pinned first, then urgent, important, info, then newest. Severity is a word
  and a shape, never colour alone. Publishing needs a permission; someone
  without it sees the feed and a plain note, not a broken form. A publisher
  changes one in place (`PUT`, every problem said first, nothing sent when
  nothing changed; rules in `announcement-view.ts`).
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { isAbsent } from "#lib/absent.js";
  import { t, tDynamic } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type Announcement, type AnnouncementSeverity } from "#lib/api/ppm.js";
  import {
    SEVERITY_SYMBOL,
    SEVERITY_TEXT,
    ageText,
    announcementEdit,
    formOf,
    stateLabel,
    type AnnouncementForm,
  } from "#lib/announcement-view.js";

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
      error = errorText(err, "p.announcements.could_not_load_the_announcements");
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
      error = errorText(err, "p.announcements.could_not_publish");
    }
  }
  let editing = $state<string | null>(null);
  let form = $state<AnnouncementForm | null>(null);
  let problems = $state<string[]>([]);
  let notice = $state<string | null>(null);

  function startEdit(a: Announcement) {
    editing = a.pid;
    form = formOf(a);
    problems = [];
    notice = null;
  }

  async function save(e: SubmitEvent, a: Announcement) {
    e.preventDefault();
    if (!form) return;
    const r = announcementEdit(form, a);
    if (r === null) {
      editing = null; // nothing changed: nothing sent
      return;
    }
    if ("problems" in r) {
      problems = r.problems;
      return;
    }
    problems = [];
    try {
      await ppm.updateAnnouncement(a.pid, r.body);
      notice = t("p.announcements.saved", { title: r.body.title });
      editing = null;
      error = null;
      await refresh();
    } catch (err) {
      error = errorText(err, "p.announcements.could_not_save");
    }
  }

  async function retract(a: Announcement) {
    try {
      await ppm.retractAnnouncement(a.pid);
      await refresh();
    } catch (err) {
      error = errorText(err, "p.announcements.could_not_retract");
    }
  }
</script>

<svelte:head><title>{t("p.announcements.announcements_ppm")}</title></svelte:head>

<h1>{t("p.announcements.announcements")}</h1>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="announcement-notice">{notice}</p>{/if}

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
        {#if manage && editing === a.pid && form}
          <form class="stack" onsubmit={(e) => save(e, a)} novalidate aria-label={t("p.announcements.change_h", { title: a.title })}>
            {#if problems.length > 0}
              <ul class="banner error" role="alert" data-testid="announcement-problems">
                {#each problems as p (p)}<li>{tDynamic(p)}</li>{/each}
              </ul>
            {/if}
            <label>{t("p.announcements.title")} <input bind:value={form.title} maxlength="120" /></label>
            <label>{t("p.announcements.message")} <textarea bind:value={form.body} maxlength="4000" rows="3"></textarea></label>
            <label>
              {t("p.announcements.severity")}
              <select bind:value={form.severity}>
                <option value="info">{t("p.announcements.info")}</option>
                <option value="important">{t("p.announcements.important")}</option>
                <option value="urgent">{t("p.announcements.urgent")}</option>
              </select>
            </label>
            <label><input type="checkbox" bind:checked={form.pinned} /> {t("p.announcements.pin_to_the_top")}</label>
            <label>{t("p.announcements.publish_at")} <input type="datetime-local" bind:value={form.publish} /></label>
            <label>{t("p.announcements.expires_optional")} <input type="datetime-local" bind:value={form.expires} /></label>
            <div class="row">
              <button class="button" type="submit">{t("p.announcements.save")}</button>
              <button class="button" type="button" onclick={() => (editing = null)}>{t("p.announcements.cancel")}</button>
            </div>
          </form>
        {:else if manage}
          <div class="row">
            <button type="button" class="button" onclick={() => startEdit(a)} aria-label={t("p.announcements.change_h", { title: a.title })}>{t("p.announcements.change")}</button>
            <button type="button" class="button danger" onclick={() => retract(a)} aria-label={`Retract ${a.title}`}>{t("p.announcements.retract")}</button>
          </div>
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
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
</style>
