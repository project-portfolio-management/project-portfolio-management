<!--
  CEO dashboard (`/ceo`): the whole picture on ONE screen of an iPad (9th gen,
  1080 × 810 CSS pixels, landscape or portrait), with no scrolling of the page.
  Six tiles — portfolio health, risk and money, milestones to watch, KPIs that
  need a look, recent decisions, announcements. Each tile shows a few lines and a link to the
  full page, so nothing is hidden, only one tap away. A tile that cannot load
  says so on its own; the others still show. Status is a word and a shape,
  never colour alone. Touch targets are at least 44 px.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import {
    PpmClient,
    type AnnouncementList,
    type DecisionEntry,
    type ExecutiveHealth,
    type KpiList,
    type MilestoneList,
  } from "#lib/api/ppm.js";
  import { STATUS_SYMBOL, STATUS_TEXT } from "#lib/kpi-view.js";
  import {
    decisionLines,
    escalatedRisks,
    healthSentence,
    milestonesToWatch,
    needsAttention,
    overdueMilestones,
    overrunPortfolios,
    ragTotals,
    worstStatus,
  } from "#lib/ceo-view.js";
  import { SEVERITY_SYMBOL, SEVERITY_TEXT, ageText, excerpt } from "#lib/announcement-view.js";
  import { localeHref } from "#lib/i18n.svelte.js";

  const ppm = PpmClient.withFetch();
  // Each tile loads on its own, so one failure does not blank the screen.
  let health = $state<ExecutiveHealth | null>(null);
  let milestones = $state<MilestoneList | null>(null);
  let kpis = $state<KpiList | null>(null);
  let news = $state<AnnouncementList | null>(null);
  let decisions = $state<DecisionEntry[] | null>(null);
  let failed = $state<Record<string, string>>({});
  const now = new Date();

  const fail = (k: string, e: unknown) =>
    (failed[k] = e instanceof Error ? e.message : "Could not load");
  onMount(() => {
    ppm.executiveHealth().then((v) => (health = v), (e) => fail("health", e));
    ppm.portfolioMilestones().then((v) => (milestones = v), (e) => fail("milestones", e));
    ppm.kpis().then((v) => (kpis = v), (e) => fail("kpis", e));
    ppm.executiveDecisions(30).then((v) => (decisions = v.decisions), (e) => fail("decisions", e));
    ppm.announcements({ limit: 3 }).then((v) => (news = v), (e) => fail("news", e));
  });

  const RAG_TEXT = { red: "Red", amber: "Amber", green: "Green" } as const;
  const RAG_SYMBOL = { red: "■", amber: "▲", green: "●" } as const;
  const MS_TEXT = { achieved: "Achieved", late: "Late", due_soon: "Due soon", upcoming: "Upcoming" } as const;
</script>

<svelte:head><title>CEO dashboard — PPM</title></svelte:head>

<div class="ceo" data-testid="ceo">
  <h1>CEO dashboard</h1>

  <section class="tile" aria-labelledby="t-health" data-testid="tile-health">
    <h2 id="t-health">Portfolio health</h2>
    {#if failed.health}<p class="banner error" role="alert">{failed.health}</p>
    {:else if !health}<p class="muted">Loading…</p>
    {:else}
      {@const worst = worstStatus(health)}
      {@const t = ragTotals(health)}
      <p class="big">
        {#if worst}<span aria-hidden="true">{RAG_SYMBOL[worst]}</span> {RAG_TEXT[worst]}{:else}No data yet{/if}
      </p>
      <p class="small">{healthSentence(health)}</p>
      <p class="small muted">■ {t.red} red · ▲ {t.amber} amber · ● {t.green} green</p>
    {/if}
    <a class="more" href={localeHref("/executive")}>Executive detail</a>
  </section>

  <section class="tile" aria-labelledby="t-risk" data-testid="tile-risk">
    <h2 id="t-risk">Risk and money</h2>
    {#if failed.health}<p class="banner error" role="alert">{failed.health}</p>
    {:else if !health}<p class="muted">Loading…</p>
    {:else}
      {@const over = overrunPortfolios(health)}
      <p class="big">{escalatedRisks(health)} <span class="small">escalated risks</span></p>
      <p class="small">{overdueMilestones(health)} overdue milestones</p>
      <p class="small">
        {#if over.length}▲ Over budget: {over.join(", ")}{:else}● No portfolio is over budget{/if}
      </p>
    {/if}
    <a class="more" href={localeHref("/risk")}>Risk register</a>
  </section>

  <section class="tile" aria-labelledby="t-ms" data-testid="tile-milestones">
    <h2 id="t-ms">Milestones to watch</h2>
    {#if failed.milestones}<p class="banner error" role="alert">{failed.milestones}</p>
    {:else if !milestones}<p class="muted">Loading…</p>
    {:else}
      {@const w = milestonesToWatch(milestones.milestones, 4)}
      {#if w.shown.length === 0}
        <p class="small">No open milestones.</p>
      {:else}
        <ul>
          {#each w.shown as m (m.pid)}
            <li><span class="small">{MS_TEXT[m.status]}</span> {m.name} <span class="muted small">{m.due}</span></li>
          {/each}
        </ul>
        {#if w.more}<p class="small muted">and {w.more} more</p>{/if}
      {/if}
    {/if}
    <a class="more" href={localeHref("/milestones")}>All milestones</a>
  </section>

  <section class="tile" aria-labelledby="t-kpi" data-testid="tile-kpis">
    <h2 id="t-kpi">Key measures needing a look</h2>
    {#if failed.kpis}<p class="banner error" role="alert">{failed.kpis}</p>
    {:else if !kpis}<p class="muted">Loading…</p>
    {:else}
      {@const a = needsAttention(kpis.kpis, 4)}
      {#if a.shown.length === 0}
        <p class="small">{kpis.kpis.length === 0 ? "No KPIs defined yet." : "● Every KPI is on target."}</p>
      {:else}
        <ul>
          {#each a.shown as k (k.kpi.pid)}
            <li><span aria-hidden="true">{STATUS_SYMBOL[k.status]}</span> {k.kpi.name} <span class="muted small">{STATUS_TEXT[k.status]}</span></li>
          {/each}
        </ul>
        {#if a.more}<p class="small muted">and {a.more} more</p>{/if}
      {/if}
    {/if}
    <a class="more" href={localeHref("/kpis")}>All key measures</a>
  </section>

  <section class="tile" aria-labelledby="t-dec" data-testid="tile-decisions">
    <h2 id="t-dec">Recent decisions</h2>
    {#if failed.decisions}<p class="banner error" role="alert">{failed.decisions}</p>
    {:else if !decisions}<p class="muted">Loading…</p>
    {:else}
      {@const d = decisionLines(decisions, 4)}
      {#if d.lines.length === 0}
        <p class="small">No decisions recorded yet.</p>
      {:else}
        <ul>
          {#each d.lines as l (l.key)}
            <li>{l.text} <span class="muted small">{l.at}</span></li>
          {/each}
        </ul>
        {#if d.more}<p class="small muted">and {d.more} more</p>{/if}
      {/if}
    {/if}
    <a class="more" href={localeHref("/executive")}>Decision log</a>
  </section>

  <section class="tile" aria-labelledby="t-news" data-testid="tile-news">
    <h2 id="t-news">Announcements</h2>
    {#if failed.news}<p class="banner error" role="alert">{failed.news}</p>
    {:else if !news}<p class="muted">Loading…</p>
    {:else if news.announcements.length === 0}
      <p class="small">Nothing has been announced.</p>
    {:else}
      <ul>
        {#each news.announcements as a (a.pid)}
          <li>
            <span aria-hidden="true">{SEVERITY_SYMBOL[a.severity]}</span>
            <strong>{a.title}</strong>
            <span class="muted small">{SEVERITY_TEXT[a.severity]} · {ageText(a.publish_at, now)}</span>
            <span class="small"> — {excerpt(a.body, 90)}</span>
          </li>
        {/each}
      </ul>
    {/if}
    <a class="more" href={localeHref("/announcements")}>All announcements</a>
  </section>
</div>

<style>
  /* One screen: the page never scrolls at 1080×810 or 810×1080. The grid is two
     columns, three rows; a tile shows a few lines and clips the rest (each has a
     link to the full page). */
  .ceo {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    grid-template-rows: auto repeat(3, minmax(0, 1fr));
    gap: 0.75rem;
    /* The header above (89px) and the page padding below (24px) are measured, with a little spare. */
    height: calc(100dvh - 7.25rem);
    min-height: 30rem;
    max-width: 1080px;
    margin: 0 auto;
  }
  h1 {
    grid-column: 1 / -1;
    margin: 0;
    font-size: 1.5rem;
  }
  .tile {
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--mxi-color-surface);
    border: 1px solid var(--mxi-color-border);
    border-radius: var(--mxi-radius);
    padding: 0.75rem 1rem;
    min-width: 0;
  }
  h2 {
    font-size: 1rem;
    margin: 0 0 0.25rem;
  }
  .big {
    font-size: 2rem;
    margin: 0;
    line-height: 1.2;
  }
  p {
    margin: 0 0 0.25rem;
  }
  ul {
    margin: 0;
    padding: 0;
    list-style: none;
    overflow: hidden;
  }
  li {
    padding: 0.1rem 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .more {
    margin-top: auto;
    display: inline-flex;
    align-items: center;
    min-height: 44px;
    text-decoration: underline;
  }
  @media (max-width: 640px) {
    .ceo {
      grid-template-columns: minmax(0, 1fr);
      grid-template-rows: auto;
      height: auto;
    }
  }
</style>
