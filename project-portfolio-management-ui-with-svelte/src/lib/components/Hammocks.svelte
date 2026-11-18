<!--
  A plan's hammocks (`/plans/[pid]/critical-path`, NC-1 slice 12 part i, PJ-7): each hammock with the span its links
  give it (`GET /api/plans/{pid}/hammocks`), in the reader's locale, or why it has none; a task made a hammock (`POST
  /api/plans/{pid}/tasks/{t_pid}/hammock`, after a question, as its start date is cleared) or made a task again (after
  a question, as every link goes); a link added (`POST …/hammocks/{h_pid}/links`) or removed after a question (`DELETE
  …/links/{member}`). The plan's tasks come from `GET /api/plans/{pid}/tasks` (the lists disabled until read). Every
  rule is said and checked first, from what the critical path says of each task. Rules in `src/lib/hammock-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { t, tn, type StringKey } from "#lib/i18n.svelte.js";
  import { formatDateRange } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { PpmClient, type Cpm, type Hammock, type Task } from "#lib/api/ppm.js";
  import { hammockRefusalKey, linkProblem, makeProblem, unscheduledKey, type HammockAction } from "#lib/hammock-view.js";

  let { pid, cpm, onchanged }: { pid: string; cpm: Cpm | null; onchanged: () => void } = $props();

  const ppm = PpmClient.withFetch();
  let hammocks = $state<Hammock[] | null>(null);
  let tasks = $state<Task[] | null>(null);
  let loadError = $state<string | null>(null);
  let problem = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);
  let makePid = $state("");
  let linkHammock = $state("");
  let linkMember = $state("");

  async function load() {
    try {
      const [h, ts] = await Promise.all([ppm.hammocks(pid), ppm.listTasks(pid)]);
      hammocks = h.hammocks;
      tasks = ts.tasks;
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.hammock.could_not_load");
    }
  }
  onMount(load);

  const candidates = $derived((tasks ?? []).filter((x) => !x.is_hammock));
  const titleOf = (p: string) => tasks?.find((x) => x.pid === p)?.title ?? hammocks?.find((h) => h.pid === p)?.title ?? t("p.hammock.untitled");

  function why(action: HammockAction, err: unknown): string {
    const key = hammockRefusalKey(action, err);
    return key ? t(key) : errorText(err, "p.hammock.could_not_save");
  }

  async function act(action: HammockAction, run: () => Promise<unknown>, done: StringKey, words: Record<string, string>) {
    busy = true;
    problem = null;
    try {
      await run();
      notice = t(done, words);
      await load();
      onchanged();
    } catch (err) {
      problem = why(action, err);
      await load();
    } finally {
      busy = false;
    }
  }

  async function make(e: SubmitEvent) {
    e.preventDefault();
    notice = "";
    const p = makeProblem(makePid, cpm);
    if (p) {
      problem = t(p);
      return;
    }
    problem = null;
    const title = titleOf(makePid);
    if (!(await askConfirm(t("p.hammock.confirm_make", { title })))) return;
    const target = makePid;
    makePid = "";
    await act("make", () => ppm.setHammock(pid, target, true), "p.hammock.made", { title });
  }

  async function unmake(h: Hammock) {
    notice = "";
    if (!(await askConfirm(tn("p.hammock.confirm_unmake", h.links.length, { title: h.title, n: h.links.length })))) return;
    await act("unmake", () => ppm.setHammock(pid, h.pid, false), "p.hammock.unmade", { title: h.title });
  }

  async function link(e: SubmitEvent) {
    e.preventDefault();
    notice = "";
    const p = linkHammock ? linkProblem(linkHammock, linkMember, hammocks ?? []) : "p.hammock.problem_choose_hammock";
    if (p) {
      problem = t(p);
      return;
    }
    const words = { hammock: titleOf(linkHammock), title: titleOf(linkMember) };
    const [h, m] = [linkHammock, linkMember];
    linkMember = "";
    await act("link", () => ppm.linkHammock(pid, h, m), "p.hammock.linked", words);
  }

  async function unlink(h: Hammock, memberPid: string, title: string) {
    notice = "";
    const words = { hammock: h.title, title };
    if (!(await askConfirm(t("p.hammock.confirm_unlink", words)))) return;
    await act("unlink", () => ppm.unlinkHammock(pid, h.pid, memberPid), "p.hammock.unlinked", words);
  }
</script>

<section aria-labelledby="hammock-h" data-testid="hammocks">
  <h2 id="hammock-h">{t("p.hammock.heading")}</h2>
  <p class="small muted">{t("p.hammock.says")}</p>
  {#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  <p class="small" role="status">{notice}</p>

  {#if hammocks}
    {#if hammocks.length === 0}
      <p>{t("p.hammock.none")}</p>
    {:else}
      <ul class="hammocks">
        {#each hammocks as h (h.pid)}
          {@const why = unscheduledKey(h.span)}
          <li>
            <strong>{h.title}</strong>
            {#if h.span.state === "scheduled"}
              <span>{t("p.hammock.spans", { range: formatDateRange(h.span.start, h.span.finish) })}</span>
            {:else if why}
              <span><span aria-hidden="true">○ </span>{t(why)}</span>
            {/if}
            <button class="button small" type="button" disabled={busy} onclick={() => void unmake(h)}>{t("p.hammock.unmake_named", { title: h.title })}</button>
            {#if h.links.length > 0}
              <ul class="small">
                {#each h.links as l (l.pid)}
                  {@const title = l.title ?? t("p.hammock.untitled")}
                  <li>
                    {l.is_hammock ? t("p.hammock.member_hammock", { title }) : title}
                    <button class="button small" type="button" disabled={busy} onclick={() => void unlink(h, l.pid, title)} aria-label={t("p.hammock.unlink_named", { hammock: h.title, title })}>{t("p.hammock.unlink")}</button>
                  </li>
                {/each}
              </ul>
            {/if}
          </li>
        {/each}
      </ul>
    {/if}
  {/if}

  <form class="stack" novalidate onsubmit={make} aria-label={t("p.hammock.make_form")}>
    <label>{t("p.hammock.f_task")}
      <select bind:value={makePid} disabled={!tasks}>
        <option value="">{t("p.hammock.choose_task")}</option>
        {#each candidates as c (c.pid)}<option value={c.pid}>{c.title}</option>{/each}
      </select>
    </label>
    <div><button class="button" type="submit" disabled={!tasks || busy}>{t("p.hammock.make")}</button></div>
  </form>

  {#if hammocks && hammocks.length > 0}
    <form class="stack" novalidate onsubmit={link} aria-label={t("p.hammock.link_form")}>
      <label>{t("p.hammock.f_hammock")}
        <select bind:value={linkHammock} disabled={!tasks}>
          <option value="">{t("p.hammock.choose_hammock")}</option>
          {#each hammocks as h (h.pid)}<option value={h.pid}>{h.title}</option>{/each}
        </select>
      </label>
      <label>{t("p.hammock.f_member")}
        <select bind:value={linkMember} disabled={!tasks}>
          <option value="">{t("p.hammock.choose_member")}</option>
          {#each tasks ?? [] as c (c.pid)}<option value={c.pid}>{c.is_hammock ? t("p.hammock.member_hammock", { title: c.title }) : c.title}</option>{/each}
        </select>
      </label>
      <div><button class="button" type="submit" disabled={!tasks || busy}>{t("p.hammock.link")}</button></div>
    </form>
  {/if}
</section>

<style>
  .hammocks > li { margin-block: 0.5rem; }
  .hammocks > li > span { margin-inline: 0.5rem; }
  form { margin-block: 0.75rem; max-inline-size: 28rem; }
  label { display: grid; gap: 0.2rem; }
</style>
