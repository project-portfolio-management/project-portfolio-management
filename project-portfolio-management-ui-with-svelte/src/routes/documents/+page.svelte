<!--
  Controlled documents (`/documents`): policies and procedures whose versions are
  reviewed, approved and brought into force. The list says which version is in
  force today; creating a document starts its first draft.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { localeHref } from "#lib/i18n.svelte.js";
  import { PpmClient, type ControlledDocument } from "#lib/api/ppm.js";
  import { forceText } from "#lib/document-view.js";

  const ppm = PpmClient.withFetch();
  let docs = $state<ControlledDocument[] | null>(null);
  let error = $state<string | null>(null);
  let title = $state("");
  let body = $state("");
  let saving = $state(false);

  onMount(async () => {
    try {
      docs = await ppm.documents();
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load the documents";
    }
  });

  async function create(e: SubmitEvent) {
    e.preventDefault();
    saving = true;
    try {
      const d = await ppm.createDocument({ title, body });
      docs = [...(docs ?? []), d];
      title = body = "";
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not create the document";
    } finally {
      saving = false;
    }
  }
</script>

<svelte:head><title>Documents — PPM</title></svelte:head>

<h1>Documents</h1>
<p class="lede">Policies and procedures. A new version is reviewed and approved before it comes into force, and a version in force never changes.</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}

{#if docs === null && !error}
  <p class="muted">Loading…</p>
{:else if docs && docs.length === 0}
  <p class="muted" data-testid="no-documents">No documents yet.</p>
{:else if docs}
  <table data-testid="documents">
    <caption class="sr-only">Controlled documents</caption>
    <thead><tr><th scope="col">Document</th><th scope="col">In force</th><th scope="col">Versions</th></tr></thead>
    <tbody>
      {#each docs as d (d.pid)}
        <tr>
          <th scope="row"><a href={localeHref(`/documents/${d.pid}`)}>{d.title}</a></th>
          <td>{forceText(d)}</td>
          <td>{d.versions.length}</td>
        </tr>
      {/each}
    </tbody>
  </table>
{/if}

<form class="surface stack" onsubmit={create} aria-labelledby="new-h">
  <h2 id="new-h">New document</h2>
  <label>Title <input bind:value={title} required maxlength="200" /></label>
  <label>First draft <textarea bind:value={body} rows="5" maxlength="200000"></textarea></label>
  <button class="button primary" type="submit" disabled={saving}>Create document</button>
</form>
