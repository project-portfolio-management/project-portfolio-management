<!--
  A plan's template changes (`/plans/[pid]/template`, NC-12; NC-1 slice 12 part k): what the template it was made from
  changed since the plan was last brought up to date (`GET /api/plans/{pid}/template-diff`), each change ticked to
  take, the rest left waiting; a change to a task the plan's own people edited or removed is applied only with a second
  tick to overwrite that edit. Applied after a question (`POST /api/plans/{pid}/template-sync`); what was applied,
  overwritten, refused and still waiting is said. A plan not made from a saved template is said so. Rules in
  `src/lib/template-sync-view.ts`; the service's English reasons are never shown.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t } from "#lib/i18n.svelte.js";
  import { formatInteger } from "#lib/locale-format.js";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type TemplateDiff, type TemplateSynced } from "#lib/api/ppm.js";
  import { differences, kindOf, refusalText, syncBody } from "#lib/template-sync-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let diff = $state<TemplateDiff | null>(null);
  let none = $state(false);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let result = $state<TemplateSynced | null>(null);
  // What was sent, so each refusal is said by its title and why, as the page knew it when it sent them.
  let sentChanges = $state<TemplateDiff["changes"]>([]);
  let sentOverwrite = $state<Set<string>>(new Set());
  let accepted = $state<Record<string, boolean>>({});
  let overwrite = $state<Record<string, boolean>>({});

  async function load() {
    try {
      diff = await ppm.templateDiff(pid);
      none = false;
      accepted = {};
      overwrite = {};
    } catch (err) {
      if (err instanceof ApiError && err.status === 404) {
        none = true;
        diff = null;
      } else error = errorText(err, "p.template_sync.could_not_load");
    }
  }
  onMount(load);

  const ids = (m: Record<string, boolean>) => new Set(Object.keys(m).filter((k) => m[k]));

  async function apply(e: SubmitEvent) {
    e.preventDefault();
    if (!diff) return;
    const body = syncBody(diff.changes, ids(accepted), ids(overwrite));
    if (!body) {
      error = t("p.template_sync.choose_first");
      return;
    }
    if (!(await askConfirm(t("p.template_sync.apply_ask")))) return;
    busy = true;
    error = null;
    sentChanges = diff.changes;
    sentOverwrite = new Set(body.overwrite_local ?? []);
    try {
      result = await ppm.templateSync(pid, body);
      // Read again: what was applied leaves the list, what was refused stays waiting.
      await load();
    } catch (err) {
      error =
        err instanceof ApiError && err.status === 422
          ? t("p.template_sync.apply_refused")
          : errorText(err, "p.template_sync.could_not_apply");
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>{t("p.template_sync.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.template_sync.title")}</h1>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}

{#if none}
  <p data-testid="not-from-template">{t("p.template_sync.not_from_template")}</p>
{:else if !diff && !error}
  <p class="muted">{t("p.template_sync.loading")}</p>
{:else if diff}
  <p>
    <a href={localeHref(`/plan-templates/${diff.template_pid}`)}>{t("p.template_sync.made_from", { name: diff.template_name })}</a>
  </p>
  <p class="small muted">{t("p.template_sync.versions", { synced: formatInteger(diff.synced_version), version: formatInteger(diff.template_version) })}</p>

  {#if result}
    <div class="banner" role="status" data-testid="sync-result">
      <p>{t("p.template_sync.applied", { n: formatInteger(result.applied.length) })}</p>
      {#if result.overwritten_local_edits.length > 0}<p>{t("p.template_sync.overwritten", { n: formatInteger(result.overwritten_local_edits.length) })}</p>{/if}
      {#each result.refused as r (r.id)}<p>{refusalText(r, sentChanges, sentOverwrite)}</p>{/each}
      <p>{t("p.template_sync.remaining", { n: formatInteger(result.remaining) })}</p>
    </div>
  {/if}

  {#if diff.changes.length === 0}
    <p data-testid="up-to-date">{t("p.template_sync.up_to_date")}</p>
  {:else}
    <p class="small muted">{t("p.template_sync.intro")}</p>
    <form class="stack" onsubmit={apply}>
      <ul class="changes stack">
        {#each diff.changes as c (c.id)}
          {@const k = kindOf(c)}
          <li class="surface">
            <h2 id={`change-${c.id}`}><span aria-hidden="true">{k.symbol}</span> {c.title} <span class="small muted">· {k.text}</span></h2>
            {#each differences(c) as line, i (i)}<p class="small">{line}</p>{/each}
            <label><input type="checkbox" bind:checked={accepted[c.id]} aria-describedby={`change-${c.id}`} /> {t("p.template_sync.take")}</label>
            {#if c.locally_edited}
              <p class="small"><strong>{t("p.template_sync.edited")}</strong></p>
              <label><input type="checkbox" bind:checked={overwrite[c.id]} disabled={!accepted[c.id]} aria-describedby={`change-${c.id}`} /> {t("p.template_sync.overwrite")}</label>
            {/if}
          </li>
        {/each}
      </ul>
      <button class="button primary" type="submit" disabled={busy}>{t("p.template_sync.apply")}</button>
    </form>
  {/if}
{/if}

<style>
  .changes {
    list-style: none;
    padding: 0;
  }
  h2 {
    font-size: 1.05rem;
  }
  label {
    display: block;
  }
</style>
