<!--
  My work (`/me`, LT-3): the signed-in person's open tasks, bucketed by
  due date in THEIR OWN time zone — overdue, today, this week, later, and
  undated. One list, no chrome: the individual worker's answer to "what
  do I do today?". The zone is the browser's IANA zone, sent explicitly
  (`?tz=`) because the server never guesses one; a fixed offset would be
  refused.

  English-first, like the other recent views; localization follows.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, MY_WORK_BUCKETS, type MyWork, type MyWorkBucket } from "#lib/api/ppm.js";
  import { floatText } from "#lib/my-work-view.js";

  const ppm = PpmClient.withFetch();
  let work = $state<MyWork | null>(null);
  let error = $state<string | null>(null);

  const LABELS: Record<MyWorkBucket, string> = {
    overdue: "Overdue",
    today: "Today",
    this_week: "This week",
    later: "Later",
    undated: "No due date",
  };

  async function refresh() {
    try {
      const tz = Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";
      work = await ppm.myTasks(tz);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load your tasks";
    }
  }
  onMount(refresh);
</script>

<svelte:head><title>My work — PPM</title></svelte:head>

<h1>My work</h1>
<p class="small"><a href={localeHref("/me/calendar")}>My calendar</a> — the days you work, so your dates are honest.</p>
{#if error}<p class="banner" role="alert">{error}</p>{/if}

{#if work}
  <p class="small muted">
    {work.today} ({work.tz}) · {work.total} open
    {#if work.omitted_unreadable > 0}
      · {work.omitted_unreadable} in plans you cannot read
    {/if}
    {#if work.truncated}· showing the first 500{/if}
  </p>

  {#if work.reason === "sub_not_addressable"}
    <p class="banner" role="status">
      Your sign-in identity cannot be matched to task assignments, so nothing
      can be listed here yet.
    </p>
  {:else if work.total === 0}
    <p>Nothing assigned to you. Enjoy it.</p>
  {/if}

  {#each MY_WORK_BUCKETS as bucket (bucket)}
    {#if work.counts[bucket] > 0}
      <section aria-labelledby={`b-${bucket}`}>
        <h2 id={`b-${bucket}`} class:overdue={bucket === "overdue"}>
          {LABELS[bucket]} <span class="muted">({work.counts[bucket]})</span>
        </h2>
        <ul class="tasks">
          {#each work.buckets[bucket] as task (task.pid)}
            <li>
              <a href={localeHref(`/plans/${task.plan_pid}`)}>{task.title}</a>
              <span class="small muted">
                — {task.plan_name}{#if task.due_date} · due {task.due_date}{/if}
              </span>
              <!-- SC-10: the task's own float, in words (never colour alone);
                   nothing at all when there is no schedule data. -->
              {#if floatText(task.schedule)}
                {@const f = floatText(task.schedule)}
                {#if f}
                  <span class={`small float ${f.kind}`}>
                    {#if f.kind === "late"}<span aria-hidden="true">⚠ </span>{/if}{f.text}
                  </span>
                {/if}
              {/if}
            </li>
          {/each}
        </ul>
      </section>
    {/if}
  {/each}
{:else if !error}
  <p class="muted">Loading…</p>
{/if}

<style>
  .tasks {
    list-style: none;
    padding: 0;
  }
  .tasks li {
    padding: 0.35rem 0;
  }
  .overdue {
    color: var(--danger, #b00020);
  }
  .float {
    display: block;
  }
  .float.late {
    color: var(--danger, #b00020);
    font-weight: 600;
  }
  .float.critical {
    font-weight: 600;
  }
</style>
