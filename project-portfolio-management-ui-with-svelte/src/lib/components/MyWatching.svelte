<!--
  What I follow (`/me`, NC-1 slice 12 part h, RM-3): the things the signed-in person follows, newest first
  (`GET /api/me/watching`). The service gives a kind and a reference only, so a plan is named by `GET /api/plans/{pid}`
  and linked; anything else is its kind in words and the start of its reference. Each can be stopped after a question
  (`POST /api/watchers/unwatch`; Cancel sends nothing). Rules in `src/lib/my-work-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tn, localeHref } from "#lib/i18n.svelte.js";
  import { formatInteger } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { PpmClient, type MyWatching } from "#lib/api/ppm.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import { WATCHING_MAX, isWatchKind, meRefusalKey, shortRef, watchKindKey } from "#lib/my-work-view.js";

  const ppm = PpmClient.withFetch();
  const plans = PlanRepository.withFetch();
  let watching = $state<MyWatching | null>(null);
  let planNames = $state<Record<string, string>>({});
  let loadError = $state<string | null>(null);
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  async function namePlans(w: MyWatching) {
    const missing = w.items.filter((i) => i.subject_kind === "plan" && !(i.subject_pid in planNames)).map((i) => i.subject_pid);
    const found = await Promise.all(
      missing.map(async (pid) => {
        try {
          return [pid, (await plans.get(pid)).name] as const;
        } catch {
          return null;
        }
      }),
    );
    const named = found.filter((f): f is readonly [string, string] => f !== null);
    if (named.length > 0) planNames = { ...planNames, ...Object.fromEntries(named) };
  }

  async function load() {
    try {
      watching = await ppm.myWatching();
      loadError = null;
      void namePlans(watching);
    } catch (err) {
      watching = null;
      loadError = errorText(err, "p.watching.could_not_load");
    }
  }
  onMount(load);

  /** The thing in words: a named plan, or its kind and the start of its reference. */
  function what(kind: string, pid: string): string {
    if (kind === "plan" && planNames[pid]) return t("p.watching.named_plan", { name: planNames[pid] });
    return t("p.watching.unnamed", { kind: t(watchKindKey(kind)), ref: shortRef(pid) });
  }

  async function stop(kind: string, pid: string) {
    if (!isWatchKind(kind)) return;
    notice = "";
    problem = null;
    const thing = what(kind, pid);
    if (!(await askConfirm(t("p.watching.confirm_stop", { what: thing })))) return;
    busy = true;
    try {
      await ppm.unwatch({ subject_kind: kind, subject_pid: pid });
      notice = t("p.watching.stopped", { what: thing });
    } catch (err) {
      const key = meRefusalKey("unwatch", err);
      problem = key ? t(key) : errorText(err, "p.watching.could_not_stop");
    } finally {
      busy = false;
      await load();
    }
  }
</script>

<section class="watching" aria-labelledby="watching-h" data-testid="watching">
  <h2 id="watching-h">{t("p.watching.heading")}</h2>
  <p class="small muted">{t("p.watching.says")}</p>
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  <p class="small" role="status">{notice}</p>
  {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
  {#if watching}
    {#if watching.items.length === 0}
      <p>{t("p.watching.none")}</p>
    {:else}
      <p class="small">
        {tn("p.watching.count", watching.count, { n: formatInteger(watching.count) })}
        {#if watching.count >= WATCHING_MAX}{t("p.watching.capped", { max: formatInteger(WATCHING_MAX) })}{/if}
      </p>
      <ul class="followed">
        {#each watching.items as item (`${item.subject_kind}:${item.subject_pid}`)}
          <li>
            {#if item.subject_kind === "plan"}
              <a href={localeHref(`/plans/${item.subject_pid}`)}>{what(item.subject_kind, item.subject_pid)}</a>
            {:else}
              <span>{what(item.subject_kind, item.subject_pid)}</span>
            {/if}
            {#if isWatchKind(item.subject_kind)}
              <button class="button small" type="button" disabled={busy} onclick={() => void stop(item.subject_kind, item.subject_pid)}
                aria-label={t("p.watching.stop_named", { what: what(item.subject_kind, item.subject_pid) })}>{t("p.watching.stop")}</button>
            {/if}
          </li>
        {/each}
      </ul>
    {/if}
  {:else if !loadError}
    <p class="muted">{t("p.watching.loading")}</p>
  {/if}
</section>

<style>
  .watching {
    margin-block: 1.5rem;
  }
  .followed {
    list-style: none;
    padding: 0;
  }
  .followed li {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: baseline;
    padding-block: 0.3rem;
  }
</style>
