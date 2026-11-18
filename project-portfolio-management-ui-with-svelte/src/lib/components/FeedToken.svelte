<!--
  The calendar feed (`/me`, NC-1 slice 12 part h, LT-7): a private address a calendar app subscribes to, and a second
  one for a feed reader. Making one (`POST /api/me/feed-token`) replaces any earlier one, so it is asked first; the
  addresses are shown once, as the service returns them (through this site's proxy), in read-only boxes, and are
  never logged or kept by the page. Stopping them all (`POST /api/me/feed-token/revoke`) is asked first too; Cancel
  sends nothing. The service's `note` is English and is not shown. Rules in `src/lib/my-work-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, type StringKey } from "#lib/i18n.svelte.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { API_BASE_URL } from "#lib/config.js";
  import { PpmClient } from "#lib/api/ppm.js";
  import { feedAddress, meRefusalKey, type MeAction } from "#lib/my-work-view.js";

  const ppm = PpmClient.withFetch();
  let shown = $state<{ calendar: string; atom: string | null } | null>(null);
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  function why(action: MeAction, err: unknown, fallback: StringKey): string {
    const key = meRefusalKey(action, err);
    return key ? t(key) : errorText(err, fallback);
  }

  async function issue() {
    notice = "";
    problem = null;
    if (!(await askConfirm(t("p.feed.confirm_issue")))) return;
    busy = true;
    shown = null;
    try {
      const r = await ppm.issueFeedToken();
      const calendar = feedAddress(API_BASE_URL, r.calendar_url);
      if (calendar) {
        shown = { calendar, atom: feedAddress(API_BASE_URL, r.atom_url) };
        notice = t("p.feed.issued");
      } else {
        problem = t("p.feed.no_address");
      }
    } catch (err) {
      problem = why("issue_feed", err, "p.feed.could_not_issue");
    } finally {
      busy = false;
    }
  }

  async function revoke() {
    notice = "";
    problem = null;
    if (!(await askConfirm(t("p.feed.confirm_revoke")))) return;
    busy = true;
    try {
      await ppm.revokeFeedToken();
      shown = null;
      notice = t("p.feed.revoked");
    } catch (err) {
      problem = why("revoke_feed", err, "p.feed.could_not_revoke");
    } finally {
      busy = false;
    }
  }

  function selectAll(e: FocusEvent) {
    (e.currentTarget as HTMLInputElement).select();
  }
</script>

<section class="feed" aria-labelledby="feed-h" data-testid="feed">
  <h2 id="feed-h">{t("p.feed.heading")}</h2>
  <p class="small muted">{t("p.feed.says")}</p>
  <p class="small muted">{t("p.feed.rules")}</p>
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  <p class="small" role="status">{notice}</p>
  {#if shown}
    <div class="addresses">
      <label>{t("p.feed.calendar_address")}
        <input type="text" readonly value={shown.calendar} onfocus={selectAll} spellcheck="false" autocomplete="off" />
      </label>
      {#if shown.atom}
        <label>{t("p.feed.atom_address")}
          <input type="text" readonly value={shown.atom} onfocus={selectAll} spellcheck="false" autocomplete="off" />
        </label>
      {/if}
    </div>
  {/if}
  <div class="row">
    <button class="button" type="button" disabled={busy} onclick={() => void issue()}>{t("p.feed.issue")}</button>
    <button class="button" type="button" disabled={busy} onclick={() => void revoke()}>{t("p.feed.revoke")}</button>
  </div>
</section>

<style>
  .feed {
    margin-block: 1.5rem;
  }
  .addresses {
    display: grid;
    gap: 0.5rem;
    max-inline-size: 48rem;
    margin-block: 0.5rem;
  }
  .addresses label {
    display: grid;
    gap: 0.2rem;
  }
  .addresses input {
    inline-size: 100%;
    font-family: monospace;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }
</style>
