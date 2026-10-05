<!--
  Handover (`/plans/[pid]/handover`): when someone leaves (or changes role),
  preview what they hold on the plan that is still live — unfinished tasks, live
  bookings, roles, open risks — then hand it to someone else or release it.
  Nothing changes until you commit, and a commit is refused if what they hold
  changed since the preview. About people, so it needs the permission to manage
  people.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
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
      status = err instanceof ApiError && err.status === 403 ? "Handover is about people, so it needs the permission to manage people." : err instanceof Error ? err.message : "Could not preview";
    } finally {
      busy = false;
    }
  }
  async function commit() {
    if (!preview) return;
    busy = true;
    try {
      const r = await ppm.handoverCommit(pid, { from_person: preview.from_person, to_person: preview.to_person, reason: reason || undefined }, preview.handover_etag);
      status = `Done: ${r.audit_entries} item${r.audit_entries === 1 ? "" : "s"} handed over and recorded.`;
      preview = null;
      reason = "";
    } catch (err) {
      if (err instanceof ApiError && err.status === 409) {
        preview = null;
        status = "What this person holds changed since the preview, so nothing was changed. Preview again, then commit.";
      } else status = err instanceof Error ? err.message : "Could not commit";
    } finally {
      busy = false;
    }
  }
  function discard() {
    preview = null;
    status = "Discarded. Nothing was changed.";
  }
</script>

<svelte:head><title>Handover — PPM</title></svelte:head>

<h1>Handover</h1>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a> · <a href={localeHref(`/plans/${pid}/onboarding`)}>Onboarding and offboarding</a></p>

<form class="surface stack" onsubmit={run} aria-labelledby="hv-h">
  <h2 id="hv-h">Who is handing over?</h2>
  <label>From <input bind:value={from} oninput={reset} placeholder="person:…" /></label>
  <label>To <input bind:value={to} oninput={reset} placeholder="person:… (empty releases the work)" /></label>
  <button class="button primary" type="submit" disabled={busy}>Preview</button>
</form>
{#if status}<p role="status" class="small">{status}</p>{/if}

{#if preview}
  <section class="surface stack" aria-labelledby="pv-h" data-testid="preview">
    <h2 id="pv-h">Preview</h2>
    <p>{handoverText(preview.action, preview.holdings.length, preview.to_person)}</p>
    {#if preview.holdings.length}
      <table>
        <caption class="small muted">What {preview.from_person} holds that is still live</caption>
        <thead><tr><th scope="col">Kind</th><th scope="col">What</th></tr></thead>
        <tbody>
          {#each preview.holdings as h (h.pid)}
            <tr><td>{HOLDING_TEXT[h.kind] ?? h.kind}</td><td>{h.label}</td></tr>
          {/each}
        </tbody>
      </table>
      <p class="small muted">The new person's capacity is not checked here.</p>
      <label>Reason (recorded with each item) <input bind:value={reason} maxlength="500" /></label>
      <div class="row">
        <button class="button primary" type="button" onclick={commit} disabled={busy}>{preview.action === "reassign" ? "Hand over" : "Release"}</button>
        <button class="button" type="button" onclick={discard} disabled={busy}>Discard</button>
      </div>
    {/if}
  </section>
{/if}

<style>
  label {
    display: block;
  }
</style>
