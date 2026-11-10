<!--
  My notifications (`/me/notifications`, NC-1 slice 4, PL-5, LT-6, RM-11): the signed-in person's own inbox (with
  their daily digests applied in their own time zone, and a line marked read), what they are told about (each kind
  on or off per channel, and a daily digest for the inbox), and the destinations copies go to (add, remove, and the
  recent deliveries). Everything here is the caller's own; the service keys it by the token. A destination is a
  secret: the service only ever returns it masked, and the page clears the box once it is saved. A delivery's
  `last_error` is the service's text and can quote the destination, so it is never shown. Refusals say the
  catalogue's sentence, never the service's own text.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, tn, localeHref, type StringKey } from "#lib/i18n.svelte.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { formatDateTime } from "#lib/locale-format.js";
  import { onMount } from "svelte";
  import { ApiError } from "#lib/api/client.js";
  import { CapabilityClient } from "#lib/api/capabilities.js";
  import {
    PpmClient,
    type MyInbox,
    type NotificationChannel,
    type NotificationPrefs,
    type NotificationSink,
    type SinkDelivery,
  } from "#lib/api/ppm.js";
  import {
    CHANNELS,
    CHAT_NAMES,
    SINK_KINDS,
    choiceFor,
    digestBody,
    emptySinkForm,
    notificationRefusalKey,
    pidsToMark,
    sinkBody,
    sinkProblem,
    storedChannels,
    subjectPath,
    toggleBody,
    type NotificationStep,
    type SinkProblem,
  } from "#lib/notifications-view.js";

  const ppm = PpmClient.withFetch();
  const caps = CapabilityClient.withFetch();
  const tz = Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";

  let inbox = $state<MyInbox | null>(null);
  let prefs = $state<NotificationPrefs | null>(null);
  let sinks = $state<NotificationSink[] | null>(null);
  let unreadOnly = $state(false);
  // Bumped when a change to a choice is refused, so a box the person ticked is redrawn from what the service holds
  // (the values read again are the same, so nothing else would redraw it). Only then: a redraw moves the focus.
  let prefsVersion = $state(0);
  let inboxError = $state<string | null>(null);
  let prefsError = $state<string | null>(null);
  let sinksError = $state<string | null>(null);
  let actionError = $state<string | null>(null);
  let notice = $state("");
  let saving = $state(false);

  let form = $state(emptySinkForm());
  // The destination whose deliveries are open, and what was read for it.
  let logFor = $state<NotificationSink | null>(null);
  let deliveries = $state<SinkDelivery[] | null>(null);

  const PROBLEM_KEY: Record<NonNullable<SinkProblem>, StringKey> = {
    too_many: "p.notifications.problem_too_many",
    destination_required: "p.notifications.problem_destination_required",
    email_invalid: "p.notifications.problem_email_invalid",
    url_invalid: "p.notifications.problem_url_invalid",
    url_not_https: "p.notifications.problem_url_not_https",
    url_credentials: "p.notifications.problem_url_credentials",
    url_private: "p.notifications.problem_url_private",
    url_too_long: "p.notifications.problem_url_too_long",
    chat_id_required: "p.notifications.problem_chat_id_required",
  };

  /** The catalogue sentence for a failure of `step`, or the general rule with `fallback`. */
  function why(err: unknown, step: NotificationStep, fallback: StringKey): string {
    const key = err instanceof ApiError ? notificationRefusalKey(step, err.status) : null;
    return key ? t(key) : errorText(err, fallback);
  }

  async function loadInbox() {
    try {
      inbox = await ppm.myInbox(tz, unreadOnly);
      inboxError = null;
    } catch (err) {
      inboxError = why(err, "inbox", "p.notifications.could_not_load_inbox");
    }
  }
  async function loadPrefs() {
    try {
      prefs = await ppm.myNotificationPrefs();
      prefsError = null;
    } catch (err) {
      prefsError = why(err, "prefs", "p.notifications.could_not_load_prefs");
    }
  }
  async function loadSinks() {
    try {
      sinks = await ppm.myNotificationSinks();
      sinksError = null;
    } catch (err) {
      sinksError = why(err, "sinks", "p.notifications.could_not_load_sinks");
    }
  }
  onMount(() => {
    void loadInbox();
    void loadPrefs();
    void loadSinks();
  });

  /** Run one step, then read again what it changed; a refusal also reads again, as the page may be out of date. */
  async function take(
    step: NotificationStep,
    call: () => Promise<unknown>,
    done: string,
    reload: () => Promise<void>,
    fallback: StringKey,
  ): Promise<boolean> {
    actionError = null;
    notice = "";
    saving = true;
    try {
      await call();
      notice = done;
      await reload();
      return true;
    } catch (err) {
      actionError = why(err, step, fallback);
      await reload();
      if (step === "set_pref" || step === "reset_pref") prefsVersion += 1;
      return false;
    } finally {
      saving = false;
    }
  }

  function kindLabel(kind: string): string {
    return enumLabel("notify_kind", kind);
  }
  function sinkKindLabel(kind: string): string {
    const name = CHAT_NAMES[kind];
    return name ? t("p.notifications.chat_named", { name }) : enumLabel("sink_kind", kind);
  }

  function markRead(pids: string[]) {
    void take(
      "mark_read",
      async () => {
        for (const p of pids) await caps.markRead(p);
      },
      t("p.notifications.marked_read"),
      loadInbox,
      "p.notifications.could_not_mark",
    );
  }

  function toggle(kind: string, channel: NotificationChannel, enabled: boolean) {
    const current = prefs;
    if (!current) return;
    void take(
      "set_pref",
      () => ppm.setNotificationPref(toggleBody(current, kind, channel, enabled)),
      t("p.notifications.pref_saved"),
      loadPrefs,
      "p.notifications.could_not_save_pref",
    );
  }
  function setDaily(kind: string, daily: boolean) {
    const current = prefs;
    if (!current) return;
    void take(
      "set_pref",
      () => ppm.setNotificationPref(digestBody(current, kind, daily)),
      t("p.notifications.pref_saved"),
      async () => {
        await loadPrefs();
        await loadInbox();
      },
      "p.notifications.could_not_save_pref",
    );
  }
  function resetKind(kind: string) {
    const current = prefs;
    if (!current) return;
    const channels = storedChannels(current, kind);
    void take(
      "reset_pref",
      async () => {
        for (const channel of channels) await ppm.resetNotificationPref({ kind, channel });
      },
      t("p.notifications.pref_reset_done"),
      async () => {
        await loadPrefs();
        await loadInbox();
      },
      "p.notifications.could_not_save_pref",
    );
  }

  async function addSink(e: SubmitEvent) {
    e.preventDefault();
    actionError = null;
    notice = "";
    const problem = sinkProblem(form, sinks?.length ?? 0);
    if (problem) {
      actionError = t(PROBLEM_KEY[problem]);
      return;
    }
    saving = true;
    try {
      const added = await ppm.addNotificationSink(sinkBody(form));
      // The destination is a secret: it leaves the page once the service has it.
      form = emptySinkForm();
      notice = t("p.notifications.sink_added", { destination: added.destination });
      await loadSinks();
    } catch (err) {
      actionError = why(err, "add_sink", "p.notifications.could_not_save_sink");
    } finally {
      saving = false;
    }
  }

  function removeSink(sink: NotificationSink) {
    void take(
      "remove_sink",
      () => ppm.removeNotificationSink(sink.pid),
      t("p.notifications.sink_removed"),
      async () => {
        if (logFor?.pid === sink.pid) {
          logFor = null;
          deliveries = null;
        }
        await loadSinks();
      },
      "p.notifications.could_not_remove_sink",
    );
  }

  async function openLog(sink: NotificationSink) {
    actionError = null;
    logFor = sink;
    deliveries = null;
    try {
      deliveries = await ppm.sinkDeliveries(sink.pid);
    } catch (err) {
      actionError = why(err, "deliveries", "p.notifications.could_not_load_deliveries");
      logFor = null;
      if (err instanceof ApiError && err.status === 404) await loadSinks();
    }
  }
</script>

<svelte:head><title>{t("p.notifications.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref("/me")}>{t("p.notifications.back")}</a></p>
<h1>{t("p.notifications.heading")}</h1>
<p class="small muted">{t("p.notifications.intro")}</p>

{#if actionError}<p class="banner" role="alert">{actionError}</p>{/if}
<p class="small" role="status">{notice}</p>

<section aria-labelledby="n-inbox-h" data-testid="inbox">
  <h2 id="n-inbox-h">{t("p.notifications.inbox_heading")}</h2>
  <label class="inline">
    <input type="checkbox" bind:checked={unreadOnly} onchange={() => void loadInbox()} />
    {t("p.notifications.unread_only")}
  </label>
  {#if inboxError}<p class="banner" role="alert">{inboxError}</p>{/if}
  {#if inbox}
    <p class="small muted">
      {tn("p.notifications.unread_count", inbox.unread_count, { n: inbox.unread_count })}
      {t("p.notifications.inbox_tz", { tz: inbox.tz })}
    </p>
    {#if inbox.items.length === 0}
      <p>{unreadOnly ? t("p.notifications.inbox_unread_empty") : t("p.notifications.inbox_empty")}</p>
    {:else}
      <ul class="items">
        {#each inbox.items as item (item.pids.join(","))}
          {@const path = subjectPath(item)}
          <li class:unread={item.unread}>
            <span class="state">
              {#if item.unread}<span aria-hidden="true">● </span>{t("p.notifications.status_unread")}{:else}<span aria-hidden="true">○ </span>{t("p.notifications.status_read")}{/if}
            </span>
            <span class="kind">{kindLabel(item.kind)}</span>
            <span class="message">{item.message}</span>
            <span class="small muted">
              {formatDateTime(item.latest_at)}
              {#if item.digest}· {tn("p.notifications.digest_count", item.count, { n: item.count })}{/if}
            </span>
            <span class="row-actions">
              {#if path}<a href={localeHref(path)}>{t("p.notifications.open_subject")}</a>{/if}
              {#if item.unread}
                <button class="button" type="button" disabled={saving}
                  aria-label={t("p.notifications.mark_read_named", { message: item.message })}
                  onclick={() => markRead(pidsToMark(item))}
                >{t("p.notifications.mark_read")}</button>
              {/if}
            </span>
          </li>
        {/each}
      </ul>
    {/if}
  {:else if !inboxError}
    <p class="muted">{t("p.notifications.loading")}</p>
  {/if}
</section>

<section aria-labelledby="n-prefs-h" data-testid="prefs">
  <h2 id="n-prefs-h">{t("p.notifications.prefs_heading")}</h2>
  <p class="small muted">{t("p.notifications.prefs_intro")}</p>
  {#if prefsError}<p class="banner" role="alert">{prefsError}</p>{/if}
  {#if prefs}
    {@const current = prefs}
    {#key prefsVersion}
    {#each current.known_kinds as kind (kind)}
      {@const stored = storedChannels(current, kind)}
      <fieldset class="pref">
        <legend>{kindLabel(kind)}</legend>
        {#each CHANNELS.filter((c) => current.channels.includes(c)) as channel (channel)}
          {@const choice = choiceFor(current, kind, channel)}
          <label class="inline">
            <input type="checkbox" checked={choice.enabled} disabled={saving}
              onchange={(e) => toggle(kind, channel, e.currentTarget.checked)} />
            {enumLabel("notify_channel", channel)}
          </label>
        {/each}
        {#if current.digests.includes("daily")}
          {@const inApp = choiceFor(current, kind, "in_app")}
          <label class="inline">
            <input type="checkbox" checked={inApp.digest === "daily"} disabled={saving || !inApp.enabled}
              onchange={(e) => setDaily(kind, e.currentTarget.checked)} />
            {t("p.notifications.pref_daily")}
          </label>
        {/if}
        {#if stored.length > 0}
          <button class="button" type="button" disabled={saving}
            aria-label={t("p.notifications.reset_named", { kind: kindLabel(kind) })}
            onclick={() => resetKind(kind)}
          >{t("p.notifications.reset")}</button>
        {/if}
      </fieldset>
    {/each}
    {/key}
  {:else if !prefsError}
    <p class="muted">{t("p.notifications.loading")}</p>
  {/if}
</section>

<section aria-labelledby="n-sinks-h" data-testid="sinks">
  <h2 id="n-sinks-h">{t("p.notifications.sinks_heading")}</h2>
  <p class="small muted">{t("p.notifications.sinks_intro")}</p>
  {#if sinksError}<p class="banner" role="alert">{sinksError}</p>{/if}
  {#if sinks}
    {#if sinks.length === 0}
      <p>{t("p.notifications.no_sinks")}</p>
    {:else}
      <ul class="items">
        {#each sinks as sink (sink.pid)}
          <li>
            <span class="kind">{sinkKindLabel(sink.kind)}</span>
            <code>{sink.destination}</code>
            <span class="small">
              {#if sink.enabled}{t("p.notifications.sink_sending")}{:else}<span aria-hidden="true">⏸ </span>{t("p.notifications.sink_paused")}{/if}
            </span>
            <span class="row-actions">
              <button class="button" type="button" disabled={saving}
                aria-label={t("p.notifications.deliveries_named", { destination: sink.destination })}
                onclick={() => void openLog(sink)}
              >{t("p.notifications.show_deliveries")}</button>
              <button class="button" type="button" disabled={saving}
                aria-label={t("p.notifications.remove_sink_named", { destination: sink.destination })}
                onclick={() => removeSink(sink)}
              >{t("p.notifications.remove_sink")}</button>
            </span>
          </li>
        {/each}
      </ul>
    {/if}
  {:else if !sinksError}
    <p class="muted">{t("p.notifications.loading")}</p>
  {/if}

  {#if logFor}
    <section aria-labelledby="n-log-h" data-testid="deliveries">
      <h3 id="n-log-h">{t("p.notifications.deliveries_heading", { destination: logFor.destination })}</h3>
      <p class="small muted">{t("p.notifications.deliveries_note")}</p>
      {#if deliveries === null}
        <p class="muted">{t("p.notifications.loading")}</p>
      {:else if deliveries.length === 0}
        <p>{t("p.notifications.no_deliveries")}</p>
      {:else}
        <ul class="items">
          {#each deliveries as d, i (`${d.notification_pid}-${i}`)}
            <li>
              <span class="kind">{enumLabel("delivery_status", d.status)}</span>
              <span class="small">{tn("p.notifications.attempts", d.attempts, { n: d.attempts })}</span>
              {#if d.delivered_at}
                <span class="small muted">{t("p.notifications.delivered_at", { date: formatDateTime(d.delivered_at) })}</span>
              {:else if d.next_attempt_at}
                <span class="small muted">{t("p.notifications.next_attempt", { date: formatDateTime(d.next_attempt_at) })}</span>
              {/if}
            </li>
          {/each}
        </ul>
      {/if}
      <button class="button" type="button" onclick={() => { logFor = null; deliveries = null; }}>{t("p.notifications.close_deliveries")}</button>
    </section>
  {/if}

  <form class="sink-form" onsubmit={addSink} aria-labelledby="n-add-h" novalidate>
    <h3 id="n-add-h">{t("p.notifications.add_heading")}</h3>
    <label>{t("p.notifications.f_kind")}
      <select bind:value={form.kind}>
        {#each SINK_KINDS as k (k)}<option value={k}>{sinkKindLabel(k)}</option>{/each}
      </select>
    </label>
    <label>{form.kind === "email" ? t("p.notifications.f_destination_email") : t("p.notifications.f_destination_url")}
      <input bind:value={form.destination} type={form.kind === "email" ? "email" : "url"}
        autocomplete="off" spellcheck="false" maxlength="2048" required />
    </label>
    {#if form.kind === "telegram"}
      <label>{t("p.notifications.f_chat_id")}
        <input bind:value={form.chatId} autocomplete="off" spellcheck="false" maxlength="64" required />
      </label>
    {/if}
    <div class="row">
      <button class="button primary" type="submit" disabled={saving}>{t("p.notifications.add_sink")}</button>
    </div>
  </form>
</section>

<style>
  .items {
    list-style: none;
    padding: 0;
  }
  .items li {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem 0.75rem;
    align-items: baseline;
    padding-block: 0.4rem;
    border-block-end: 1px solid var(--border, #ddd);
  }
  .items li.unread .message {
    font-weight: 600;
  }
  .kind {
    font-weight: 600;
  }
  .row-actions {
    display: inline-flex;
    flex-wrap: wrap;
    gap: 0.25rem;
    margin-inline-start: auto;
  }
  .inline {
    display: inline-flex;
    gap: 0.35rem;
    align-items: center;
    margin-inline-end: 1rem;
  }
  .pref {
    margin-block: 0.5rem;
  }
  .sink-form {
    display: grid;
    gap: 0.5rem;
    max-inline-size: 32rem;
    margin-block: 1rem;
  }
  .sink-form label {
    display: grid;
    gap: 0.2rem;
  }
</style>
