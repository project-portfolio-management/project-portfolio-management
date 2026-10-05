<!--
  Announcements (`/announcements`): a feed of short messages to everyone —
  pinned first, then urgent, important, info, then newest. Severity is a word
  and a shape, never colour alone. Publishing needs a permission; someone
  without it sees the feed and a plain note, not a broken form.
-->
<script lang="ts">
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
      } catch {
        items = (await ppm.announcements()).announcements;
        manage = false;
      }
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the announcements";
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
      error = err instanceof Error ? err.message : "Could not publish";
    }
  }
  async function retract(a: Announcement) {
    try {
      await ppm.retractAnnouncement(a.pid);
      await refresh();
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not retract";
    }
  }
</script>

<svelte:head><title>Announcements — PPM</title></svelte:head>

<h1>Announcements</h1>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}

{#if items === null && !error}
  <p class="muted">Loading…</p>
{:else if items && items.length === 0}
  <p class="muted" data-testid="no-announcements">Nothing has been announced.</p>
{:else if items}
  <ul class="feed stack" data-testid="announcements">
    {#each items as a (a.pid)}
      <li class="surface">
        <h2>
          <span aria-hidden="true">{SEVERITY_SYMBOL[a.severity]}</span>
          {a.title}
        </h2>
        <p class="small muted">
          {SEVERITY_TEXT[a.severity]}{a.pinned ? " · Pinned" : ""}{stateLabel(a) ? ` · ${stateLabel(a)}` : ""} · {ageText(a.publish_at, now)}
        </p>
        <p>{a.body}</p>
        {#if manage}
          <button type="button" class="button danger" onclick={() => retract(a)} aria-label={`Retract ${a.title}`}>Retract</button>
        {/if}
      </li>
    {/each}
  </ul>
{/if}

{#if manage}
  <form class="surface stack" onsubmit={publish} aria-labelledby="publish-h">
    <h2 id="publish-h">Publish an announcement</h2>
    <label>Title <input bind:value={title} required maxlength="120" /></label>
    <label>Message <textarea bind:value={body} required maxlength="4000" rows="3"></textarea></label>
    <label>
      Severity
      <select bind:value={severity}>
        <option value="info">Info</option>
        <option value="important">Important</option>
        <option value="urgent">Urgent</option>
      </select>
    </label>
    <label><input type="checkbox" bind:checked={pinned} /> Pin to the top</label>
    <label>Expires (optional) <input type="datetime-local" bind:value={expires} /></label>
    <button class="button primary" type="submit">Publish</button>
  </form>
{:else if items}
  <p class="muted small">Publishing needs the announcements permission.</p>
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
