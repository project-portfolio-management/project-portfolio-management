<!--
  One controlled document (`/documents/[pid]`). Each version says where it is: a
  draft can be edited and sent for review; one in review waits for its approver;
  an approved one is brought into force today (the version it replaces is marked
  replaced the same day); a version in force or replaced is final. Exactly one
  next step is the primary button. "What was in force on" answers for any date.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { localeHref } from "#lib/i18n.svelte.js";
  import { PpmClient, type ControlledDocument, type DocumentInForce, type DocumentVersion } from "#lib/api/ppm.js";
  import { ACTION_LABEL, STATUS_SYMBOL, STEP_HELP, STEP_TITLE, approverRef, inFlight, isEditable, nextAction, versionLine } from "#lib/document-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let doc = $state<ControlledDocument | null>(null);
  let error = $state<string | null>(null);
  let notice = $state<string | null>(null);
  let selected = $state<DocumentVersion | null>(null);
  let text = $state("");
  let summary = $state("");
  let approver = $state("");
  let asOf = $state("");
  let inForce = $state<DocumentInForce | null>(null);

  async function load() {
    try {
      doc = await ppm.document(pid);
      const v = inFlight(doc) ?? doc.versions.find((x) => x.status === "effective") ?? doc.versions.at(-1) ?? null;
      if (v) await select(v);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the document";
    }
  }
  async function select(v: DocumentVersion) {
    selected = await ppm.documentVersion(pid, v.version);
    text = selected.body ?? "";
    summary = selected.change_summary ?? "";
  }
  onMount(load);

  async function run(work: () => Promise<unknown>, done: string) {
    try {
      await work();
      notice = done;
      error = null;
      await load();
    } catch (err) {
      notice = null;
      error = err instanceof Error ? err.message : "That did not work";
    }
  }
  const next = $derived(doc ? nextAction(doc) : "none");
  // Review, approval and coming into force act on the version in flight, whichever one is on screen.
  const flight = $derived(doc ? inFlight(doc) : null);
  // The one primary button does whatever the state calls for.
  function act(e: SubmitEvent) {
    e.preventDefault();
    if (next === "send_for_review") return send();
    if (next === "decide") return decide("approved");
    if (next === "make_effective") return effective();
    if (next === "start_next_draft") return nextDraft();
  }

  const save = () => selected && run(() => ppm.editDocumentVersion(pid, selected!.version, { body: text, change_summary: summary }), "Saved.");
  function send() {
    const ref = approverRef(approver);
    if (!ref || !flight) {
      error = "Enter the approver's id (a person id, not yourself).";
      return;
    }
    return run(() => ppm.submitDocumentVersion(pid, flight!.version, { approver_ref: ref }), "Sent for review.");
  }
  const decide = (decision: "approved" | "rejected") =>
    flight?.approval_pid && run(() => ppm.decideApproval(flight!.approval_pid!, { decision, comment: decision === "rejected" ? "Needs changes" : undefined }), decision === "approved" ? "Approved." : "Sent back to draft.");
  const effective = () => flight && run(() => ppm.makeDocumentVersionEffective(pid, flight!.version), "In force from today.");
  const nextDraft = () => run(() => ppm.newDocumentVersion(pid), "Next draft started.");
  async function lookup(e: SubmitEvent) {
    e.preventDefault();
    try {
      inForce = await ppm.documentInForce(pid, asOf || undefined);
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not look that up";
    }
  }
</script>

<svelte:head><title>{doc?.title ?? "Document"} — PPM</title></svelte:head>

<p class="small"><a href={localeHref("/documents")}>← Documents</a></p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if notice}<p class="banner" role="status">{notice}</p>{/if}

<h1>{doc?.title ?? "Document"}</h1>
{#if doc}

  <section aria-labelledby="versions-h">
    <h2 id="versions-h">Versions</h2>
    <ul class="stack" data-testid="versions">
      {#each doc.versions as v (v.pid)}
        <li>
          <button type="button" class="link" aria-current={selected?.pid === v.pid} onclick={() => select(v)}>
            <span aria-hidden="true">{STATUS_SYMBOL[v.status]}</span>
            {versionLine(v, doc.versions)}
          </button>
          {#if v.change_summary}<span class="small muted"> — {v.change_summary}</span>{/if}
        </li>
      {/each}
    </ul>
  </section>

  {#if selected}
    <section class="surface stack" aria-labelledby="ver-h">
      <h2 id="ver-h">Version {selected.version}</h2>
      {#if isEditable(selected)}
        <label>What changed <input bind:value={summary} maxlength="500" /></label>
        <label>Text <textarea bind:value={text} rows="10" maxlength="200000"></textarea></label>
        <button type="button" class="button" onclick={save}>Save draft</button>
      {:else}
        <p class="small muted">This version's text is final. A change goes in the next draft.</p>
        <pre class="body">{selected.body}</pre>
      {/if}
    </section>
  {/if}

  {#if next !== "none"}
    <form class="surface stack" onsubmit={act} aria-labelledby="next-h">
      <h2 id="next-h">{STEP_TITLE[next]}</h2>
      <p class="small muted">{STEP_HELP[next]}</p>
      {#if next === "send_for_review"}
        <label>Approver's id <input bind:value={approver} placeholder="person id" /></label>
      {/if}
      <button class="button primary" type="submit">{ACTION_LABEL[next]}</button>
      {#if next === "decide"}
        <button type="button" class="button" onclick={() => decide("rejected")}>Send back</button>
      {/if}
    </form>
  {/if}

  <form class="surface stack" onsubmit={lookup} aria-labelledby="when-h">
    <h2 id="when-h">What was in force on a date</h2>
    <label>Date <input type="date" bind:value={asOf} /></label>
    <button class="button" type="submit">Look up</button>
    {#if inForce}
      <p data-testid="in-force">
        {#if inForce.in_force}Version {inForce.in_force.version} was in force on {inForce.on}.{:else}Nothing was in force on {inForce.on}.{/if}
      </p>
    {/if}
  </form>
{/if}

<style>
  .body { white-space: pre-wrap; }
  .link { background: none; border: 0; padding: 0; color: inherit; text-decoration: underline; cursor: pointer; font: inherit; }
  .link[aria-current="true"] { font-weight: 700; }
</style>
