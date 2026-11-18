<!--
  A sprint's ceremonies and its commitment (`/plans/[pid]/board`, NC-1 slice 12 part j, FR-29): `GET
  /api/sprints/{pid}/ceremonies` (every kind counted, zero said) and `GET /api/sprints/{pid}/commitment` (the tasks
  committed beside those in it now, the ones added and removed named from the board); a ceremony recorded (`POST
  …/ceremonies`, checked first: a second planning or review, a facilitator that is not a reference); the sprint's
  tasks committed once after a question (`POST …/commit`; not offered for a sprint with no tasks). The form is
  disabled until the sprint is read. Rules in `src/lib/sprint-ceremony-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, tn } from "#lib/i18n.svelte.js";
  import { formatDateTime, formatInteger } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { PpmClient, type SprintCeremonies, type SprintCommitment, type Task } from "#lib/api/ppm.js";
  import {
    CEREMONY_KINDS,
    CEREMONY_KEY,
    ceremonyBody,
    ceremonyKey,
    commitProblem,
    emptyCeremonyForm,
    heldCounts,
    scopeChange,
    sprintRefusalKey,
    type CeremonyForm,
  } from "#lib/sprint-ceremony-view.js";

  let { sprintPid, sprintName, tasks }: { sprintPid: string; sprintName: string; tasks: Task[] } = $props();

  const ppm = PpmClient.withFetch();
  let ceremonies = $state<SprintCeremonies | null>(null);
  let commitment = $state<SprintCommitment | null>(null);
  let form = $state<CeremonyForm>(emptyCeremonyForm());
  let loadError = $state<string | null>(null);
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  async function load(sprint: string) {
    try {
      const [c, m] = await Promise.all([ppm.sprintCeremonies(sprint), ppm.sprintCommitment(sprint)]);
      if (sprint !== sprintPid) return;
      ceremonies = c;
      commitment = m;
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.ceremony.could_not_load");
    }
  }
  // A different sprint starts over.
  let loadedFor = "";
  $effect(() => {
    if (sprintPid !== loadedFor) {
      loadedFor = sprintPid;
      ceremonies = null;
      commitment = null;
      problem = null;
      notice = "";
      form = emptyCeremonyForm();
      void load(sprintPid);
    }
  });

  const titleOf = (p: string) => tasks.find((x) => x.pid === p)?.title;
  const change = $derived(commitment ? scopeChange(commitment, titleOf) : null);

  async function hold(e: SubmitEvent) {
    e.preventDefault();
    notice = "";
    const r = ceremonyBody(form, ceremonies);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k, { kind: form.kind ? t(ceremonyKey(form.kind)) : "" })).join(" ");
      return;
    }
    problem = null;
    busy = true;
    try {
      await ppm.holdCeremony(sprintPid, r.body);
      notice = t("p.ceremony.held_saved", { kind: t(ceremonyKey(r.body.kind)) });
      form = emptyCeremonyForm();
      await load(sprintPid);
    } catch (err) {
      const key = sprintRefusalKey("ceremony", err);
      problem = key ? t(key) : errorText(err, "p.ceremony.could_not_save");
      await load(sprintPid);
    } finally {
      busy = false;
    }
  }

  async function commit() {
    notice = "";
    const p = commitProblem(commitment);
    if (p) {
      problem = t(p);
      return;
    }
    problem = null;
    const n = commitment?.current ?? 0;
    if (!(await askConfirm(tn("p.commitment.confirm", n, { n: formatInteger(n), name: sprintName })))) return;
    busy = true;
    try {
      const r = await ppm.commitSprint(sprintPid);
      notice = tn("p.commitment.saved", r.committed, { n: formatInteger(r.committed) });
      await load(sprintPid);
    } catch (err) {
      const key = sprintRefusalKey("commit", err);
      problem = key ? t(key) : errorText(err, "p.commitment.could_not");
      await load(sprintPid);
    } finally {
      busy = false;
    }
  }
</script>

<section aria-labelledby="ceremony-h" data-testid="sprint-ceremonies">
  <h2 id="ceremony-h">{t("p.ceremony.heading", { name: sprintName })}</h2>
  <p class="small muted">{t("p.ceremony.says")}</p>
  {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}

  {#if ceremonies}
    <ul class="counts" aria-label={t("p.ceremony.counts")}>
      {#each heldCounts(ceremonies) as h (h.kind)}
        <li>
          {#if h.count === 0}<span aria-hidden="true">○ </span>{t("p.ceremony.none_held", { kind: t(CEREMONY_KEY[h.kind]) })}
          {:else}{tn("p.ceremony.held_count", h.count, { kind: t(CEREMONY_KEY[h.kind]), n: formatInteger(h.count) })}{/if}
        </li>
      {/each}
    </ul>
    {#if ceremonies.ceremonies.length > 0}
      <ul class="held">
        {#each ceremonies.ceremonies as c (c.pid)}
          <li>
            <strong>{t(ceremonyKey(c.kind))}</strong>, {formatDateTime(c.held_at)}{#if c.facilitator_ref}{` · ${t("p.ceremony.facilitated_by", { ref: c.facilitator_ref })}`}{/if}
            {#if c.summary}<br /><span class="small">{c.summary}</span>{/if}
          </li>
        {/each}
      </ul>
    {/if}
  {/if}

  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  <p class="small" role="status">{notice}</p>

  <form class="stack" novalidate onsubmit={hold} aria-label={t("p.ceremony.form")}>
    <label>{t("p.ceremony.f_kind")}
      <select bind:value={form.kind} disabled={!ceremonies}>
        <option value="">{t("p.ceremony.choose_kind")}</option>
        {#each CEREMONY_KINDS as k (k)}<option value={k}>{t(CEREMONY_KEY[k])}</option>{/each}
      </select>
    </label>
    <label>{t("p.ceremony.f_facilitator")}<input type="text" bind:value={form.facilitator} disabled={!ceremonies} /></label>
    <label>{t("p.ceremony.f_summary")}<textarea rows="2" bind:value={form.summary} disabled={!ceremonies}></textarea></label>
    <div><button class="button" type="submit" disabled={!ceremonies || busy}>{t("p.ceremony.record")}</button></div>
  </form>

  <h3>{t("p.commitment.heading")}</h3>
  <p class="small muted">{t("p.commitment.says")}</p>
  {#if commitment}
    {#if commitment.was_committed}
      <p data-testid="commitment">{t("p.commitment.state", { committed: formatInteger(commitment.committed), current: formatInteger(commitment.current) })}</p>
      {#if change && change.added.length > 0}<p><span aria-hidden="true">▲ </span>{tn("p.commitment.added", change.added.length, { n: formatInteger(change.added.length), titles: change.added.join(", ") })}</p>{/if}
      {#if change && change.removed.length > 0}<p><span aria-hidden="true">▼ </span>{tn("p.commitment.removed", change.removed.length, { n: formatInteger(change.removed.length), titles: change.removed.join(", ") })}</p>{/if}
      {#if change && change.added.length === 0 && change.removed.length === 0}<p>{t("p.commitment.unchanged")}</p>{/if}
    {:else}
      <p data-testid="commitment">{tn("p.commitment.not_yet", commitment.current, { n: formatInteger(commitment.current) })}</p>
      <button class="button" type="button" disabled={busy} onclick={() => void commit()}>{t("p.commitment.commit")}</button>
    {/if}
  {/if}
</section>

<style>
  section { margin-block: 1.5rem; }
  form { max-inline-size: 32rem; margin-block: 1rem; }
  label { display: grid; gap: 0.2rem; }
  .counts { display: flex; flex-wrap: wrap; gap: 0.5rem 1.25rem; list-style: none; padding: 0; }
  .held { padding-inline-start: 1.25rem; }
</style>
