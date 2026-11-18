<!--
  Posting a status update (`/plans/[pid]/status`, NC-4, BR-2, NC-1 slice 12 part b): the call (on track, at risk, off
  track), a narrative, and optional claims, each a sentence citing sources: the published numbers and the plan's tasks,
  risks, milestones and issues. Every rule the service checks is said first and checked before sending; an update is
  never changed or removed, so each one posted is a new one and restarts the cadence's clock. A refusal is the
  catalogue's sentence. Rules in `src/lib/status-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, tDynamic } from "#lib/i18n.svelte.js";
  import { PpmClient, type StatusCall } from "#lib/api/ppm.js";
  import {
    CALL_MARK,
    CLAIMS_MAX,
    CLAIM_MAX,
    METRIC_KEYS,
    NARRATIVE_MAX,
    RECORD_KINDS,
    SOURCES_MAX,
    STATUS_CALLS,
    statusRefusalKey,
    updateBody,
    type ClaimForm,
    type RecordKind,
  } from "#lib/status-view.js";

  let {
    pid,
    records,
    ready,
    onPosted,
  }: {
    pid: string;
    /** The plan's records a claim may cite, by kind (a kind that could not be read is absent). */
    records: Partial<Record<RecordKind, Array<{ pid: string; title: string }>>>;
    /** Whether the page has loaded what the form needs. */
    ready: boolean;
    onPosted: () => Promise<void>;
  } = $props();

  const ppm = PpmClient.withFetch();
  let call = $state<StatusCall | "">("");
  let narrative = $state("");
  let claims = $state<Array<ClaimForm & { pick: string; id: number }>>([]);
  let nextId = 0;
  let error = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  const groupWord = (k: RecordKind) => tDynamic(`p.status.group_${k}`);
  const metricWord = (k: string) => tDynamic(`p.status.metric_${k}`);
  /** The words for a chosen source: a number's name, or a record's kind and title. */
  function sourceWords(source: string): string {
    const [kind, rest] = source.split(":") as [string, string];
    if (kind === "metric") return metricWord(rest);
    const r = records[kind as RecordKind]?.find((x) => x.pid === rest);
    return tDynamic(`p.status.kind_${kind}`, { title: r?.title ?? "" });
  }
  const recordSources = (c: ClaimForm) => c.sources.filter((s) => !s.startsWith("metric:"));

  function addClaim() {
    if (claims.length >= CLAIMS_MAX) return;
    claims.push({ text: "", sources: [], pick: "", id: nextId++ });
  }
  function addSource(i: number) {
    const c = claims[i];
    if (!c || !c.pick || c.sources.includes(c.pick)) return;
    c.sources.push(c.pick);
    c.pick = "";
  }
  function removeSource(i: number, source: string) {
    const c = claims[i];
    if (c) c.sources = c.sources.filter((s) => s !== source);
  }

  async function post(e: SubmitEvent) {
    e.preventDefault();
    error = null;
    notice = "";
    const r = updateBody({ status: call, narrative, claims: claims.map((c) => ({ text: c.text, sources: c.sources })) });
    if ("problems" in r) {
      error = r.problems
        .map((p) =>
          p.claim === 0
            ? t(p.key, { max: NARRATIVE_MAX, claims: CLAIMS_MAX, sources: SOURCES_MAX, chars: CLAIM_MAX })
            : t("p.status.problem_in_claim", {
                n: p.claim,
                problem: t(p.key, { max: NARRATIVE_MAX, claims: CLAIMS_MAX, sources: SOURCES_MAX, chars: CLAIM_MAX }),
              }),
        )
        .join(" ");
      return;
    }
    busy = true;
    try {
      const done = await ppm.postStatusUpdate(pid, r.body);
      notice = t("p.status.posted", { call: t(CALL_MARK[done.status].key) });
      call = "";
      narrative = "";
      claims = [];
      await onPosted();
    } catch (err) {
      const k = statusRefusalKey("update", err);
      error = k ? t(k) : errorText(err, "p.status.could_not_post");
    } finally {
      busy = false;
    }
  }
</script>

<section aria-labelledby="status-post-h">
  <h2 id="status-post-h">{t("p.status.post_heading")}</h2>
  <p class="small muted">{t("p.status.post_says", { max: NARRATIVE_MAX })}</p>
  <p class="small">{t("p.status.claims_rule", { claims: CLAIMS_MAX, chars: CLAIM_MAX, sources: SOURCES_MAX })}</p>
  {#if error}<p class="banner error" role="alert">{error}</p>{/if}
  <p class="small" role="status">{notice}</p>
  <form class="stack" novalidate onsubmit={post}>
    <fieldset class="calls" disabled={!ready}>
      <legend>{t("p.status.f_call")}</legend>
      {#each STATUS_CALLS as c (c)}
        <label class="check"><input type="radio" name="status-call" value={c} bind:group={call} /><span aria-hidden="true">{CALL_MARK[c].symbol}</span> {t(CALL_MARK[c].key)}</label>
      {/each}
    </fieldset>
    <label>{t("p.status.f_narrative", { max: NARRATIVE_MAX })}<textarea bind:value={narrative} rows="4" disabled={!ready}></textarea></label>

    {#each claims as c, i (c.id)}
      <fieldset class="claim">
        <legend>{t("p.status.claim_legend", { n: i + 1 })}</legend>
        <label>{t("p.status.f_claim_text", { chars: CLAIM_MAX })}<textarea bind:value={c.text} rows="2"></textarea></label>
        <fieldset class="metrics">
          <legend>{t("p.status.f_metrics")}</legend>
          {#each METRIC_KEYS as m (m)}
            <label class="check"><input type="checkbox" value={`metric:${m}`} bind:group={c.sources} />{metricWord(m)}</label>
          {/each}
        </fieldset>
        <div class="row">
          <label>
            {t("p.status.f_record", { n: i + 1 })}
            <select bind:value={c.pick}>
              <option value="">{t("p.status.choose_record")}</option>
              {#each RECORD_KINDS as k (k)}
                {#if (records[k] ?? []).length > 0}
                  <optgroup label={groupWord(k)}>
                    {#each records[k] ?? [] as r (r.pid)}<option value={`${k}:${r.pid}`}>{r.title}</option>{/each}
                  </optgroup>
                {/if}
              {/each}
            </select>
          </label>
          <button class="button small" type="button" disabled={!c.pick} onclick={() => addSource(i)}>{t("p.status.add_record", { n: i + 1 })}</button>
        </div>
        {#if recordSources(c).length > 0}
          <ul class="small" aria-label={t("p.status.claim_records", { n: i + 1 })}>
            {#each recordSources(c) as s (s)}
              <li>
                {sourceWords(s)}
                <button class="button small" type="button" onclick={() => removeSource(i, s)} aria-label={t("p.status.remove_source", { source: sourceWords(s), n: i + 1 })}>{t("p.status.remove")}</button>
              </li>
            {/each}
          </ul>
        {/if}
        <div class="row"><button class="button small" type="button" onclick={() => claims.splice(i, 1)}>{t("p.status.remove_claim", { n: i + 1 })}</button></div>
      </fieldset>
    {/each}
    <div class="row">
      <button class="button" type="button" disabled={!ready || claims.length >= CLAIMS_MAX} onclick={addClaim}>{t("p.status.add_claim")}</button>
      <button class="button" type="submit" disabled={busy || !ready}>{t("p.status.post")}</button>
    </div>
  </form>
</section>

<style>
  section {
    margin-block-start: 2rem;
  }
  form {
    max-width: 44rem;
  }
  fieldset {
    display: grid;
    gap: 0.35rem;
  }
  .calls,
  .metrics {
    border: 0;
    /* Room around the radios and checkboxes, so each is a large enough target (axe target-size). */
    padding: 0.4rem 0;
    margin: 0;
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }
  .claim {
    margin-block-start: 0.75rem;
  }
  .check {
    display: inline-flex;
    gap: 0.35rem;
    align-items: center;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: end;
  }
</style>
