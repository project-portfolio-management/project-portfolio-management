<!--
  Controlled documents (`/documents`): policies and procedures whose versions are
  reviewed, approved and brought into force. The list says which version is in
  force today; creating a document starts its first draft.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { t, localeHref } from "#lib/i18n.svelte.js";
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
      error = err instanceof Error ? err.message : t("p.documents.could_not_load_the_documents");
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
      error = err instanceof Error ? err.message : t("p.documents.could_not_create_the_document");
    } finally {
      saving = false;
    }
  }
</script>

<svelte:head><title>{t("p.documents.documents_ppm")}</title></svelte:head>

<h1>{t("p.documents.documents")}</h1>
<p class="lede">{t("p.documents.policies_and_procedures_a_new")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}

{#if docs === null && !error}
  <p class="muted">{t("p.documents.loading")}</p>
{:else if docs && docs.length === 0}
  <p class="muted" data-testid="no-documents">{t("p.documents.no_documents_yet")}</p>
{:else if docs}
  <table data-testid="documents">
    <caption class="sr-only">{t("p.documents.controlled_documents")}</caption>
    <thead><tr><th scope="col">{t("p.documents.document")}</th><th scope="col">{t("p.documents.in_force")}</th><th scope="col">{t("p.documents.versions")}</th></tr></thead>
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
  <h2 id="new-h">{t("p.documents.new_document")}</h2>
  <label>{t("p.documents.title")} <input bind:value={title} required maxlength="200" /></label>
  <label>{t("p.documents.first_draft")} <textarea bind:value={body} rows="5" maxlength="200000"></textarea></label>
  <button class="button primary" type="submit" disabled={saving}>{t("p.documents.create_document")}</button>
</form>
