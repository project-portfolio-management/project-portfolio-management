<!--
  The links of a plan, or of one of its task (`taskPid`): where its real documents and work live, including its Jira,
  Trello, Asana, Linear or Monday board or issue. The service stores the address and never fetches it. A link opens
  in a new tab.
-->
<script lang="ts">
  import { errorReason, errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type PlanLink } from "#lib/api/ppm.js";
  import { LINK_KINDS, LINK_SYSTEMS, kindLabel, linkBody, linkProblems, systemText, systemLabel, type LinkForm } from "#lib/framework-view.js";

  let { pid, taskPid }: { pid: string; taskPid?: string } = $props();
  const ppm = PpmClient.withFetch();
  let links = $state<PlanLink[] | null>(null);
  let error = $state<string | null>(null);
  let notice = $state("");
  let problems = $state<string[]>([]);
  let form = $state<LinkForm>({ kind: "docs", url: "", label: "", system: "", external_id: "" });

  async function load() {
    try {
      links = await ppm.planLinks(pid, taskPid);
      error = null;
    } catch (err) {
      error = errorText(err, "p.links.could_not_load");
    }
  }
  onMount(load);

  const message = (err: unknown) => t("p.links.could_not", { reason: errorReason(err) });

  async function add(e: SubmitEvent) {
    e.preventDefault();
    problems = linkProblems(form);
    if (problems.length > 0) return;
    try {
      await ppm.addPlanLink(pid, linkBody(form), taskPid);
      notice = t("p.links.added", { label: form.label.trim() });
      form = { kind: "docs", url: "", label: "", system: "", external_id: "" };
      await load();
    } catch (err) {
      notice = message(err);
    }
  }

  async function remove(l: PlanLink) {
    try {
      await ppm.removePlanLink(pid, l.pid, taskPid);
      notice = t("p.links.removed", { label: l.label });
      await load();
    } catch (err) {
      notice = message(err);
    }
  }
</script>

{#if error}<p class="banner" role="alert">{error}</p>{/if}
<p class="small" role="status">{notice}</p>

<section aria-labelledby="list-h">
  <h2 id="list-h">{t("p.links.links")}</h2>
  {#if links === null}
    <p class="muted">{t("p.links.loading")}</p>
  {:else if links.length === 0}
    <p>{t("p.links.none")}</p>
  {:else}
    <ul>
      {#each links as l (l.pid)}
        <li>
          <a href={l.url} target="_blank" rel="noopener noreferrer">{l.label}</a> <span class="small muted">({t("p.links.opens_new")})</span>
          <span class="small muted"> · {kindLabel(l.kind)}{systemText(l) ? ` · ${systemText(l)}` : ""}</span>
          <button type="button" onclick={() => remove(l)}>{t("p.links.remove", { label: l.label })}</button>
        </li>
      {/each}
    </ul>
  {/if}
</section>

<section aria-labelledby="add-h">
  <h2 id="add-h">{t("p.links.add_heading")}</h2>
  <form onsubmit={add} aria-labelledby="add-h">
    <label>{t("p.links.f_label")} <input bind:value={form.label} maxlength="200" /></label>
    <label>{t("p.links.f_url")} <input type="url" bind:value={form.url} maxlength="2048" /></label>
    <label>{t("p.links.f_kind")}
      <select bind:value={form.kind}>{#each LINK_KINDS as k (k)}<option value={k}>{kindLabel(k)}</option>{/each}</select>
    </label>
    <label>{t("p.links.f_system")}
      <select bind:value={form.system}>
        <option value=""></option>
        {#each LINK_SYSTEMS as s (s)}<option value={s}>{systemLabel(s)}</option>{/each}
      </select>
    </label>
    <label>{t("p.links.f_id")} <input bind:value={form.external_id} maxlength="100" /></label>
    {#if problems.length > 0}<ul class="banner" role="alert">{#each problems as m (m)}<li>{m}</li>{/each}</ul>{/if}
    <button type="submit">{t("p.links.add")}</button>
  </form>
</section>
