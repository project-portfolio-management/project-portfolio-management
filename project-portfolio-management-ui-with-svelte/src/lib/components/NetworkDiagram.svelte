<!--
  The precedence (network) diagram (PJ-12): tasks as boxes in columns by how
  many tasks must come before them, dependencies as arrows, **critical chains
  drawn heavier with a ● marker** (never colour alone). A relationship that is
  not plain finish-to-start, or has a lag or a lead, is labelled on its arrow
  ("SS +1 business day"). The list under the picture says the same thing in words.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { businessDays } from "#lib/units.js";
  import type { CpmEdge, CpmNode } from "#lib/api/ppm.js";
  import { floatState, layoutNetwork } from "#lib/cpm-view.js";

  let {
    nodes,
    edges,
    chains,
    selected = null,
    onselect,
  }: {
    nodes: CpmNode[];
    edges: CpmEdge[];
    chains: string[][];
    selected?: string | null;
    onselect?: (pid: string) => void;
  } = $props();

  const W = 168;
  const H = 52;
  const GX = 64;
  const GY = 22;

  const layout = $derived(
    layoutNetwork(
      nodes.map((n) => n.pid),
      edges.map((e) => ({ from: e.predecessor_pid, to: e.successor_pid })),
      chains,
    ),
  );
  const pos = $derived(
    new Map(layout.nodes.map((n) => [n.pid, { x: n.layer * (W + GX) + 8, y: n.row * (H + GY) + 8 }])),
  );
  const rows = $derived(Math.max(1, ...layout.nodes.map((n) => n.row + 1)));
  const width = $derived(layout.layers * (W + GX) - GX + 16);
  const height = $derived(rows * (H + GY) - GY + 16);
  const byPid = $derived(new Map(nodes.map((n) => [n.pid, n])));
  const edgeInfo = $derived(
    new Map(edges.map((e) => [`${e.predecessor_pid}>${e.successor_pid}`, e])),
  );

  function path(from: string, to: string): string {
    const a = pos.get(from);
    const b = pos.get(to);
    if (!a || !b) return "";
    const x1 = a.x + W;
    const y1 = a.y + H / 2;
    const x2 = b.x;
    const y2 = b.y + H / 2;
    const mid = (x1 + x2) / 2;
    return `M${x1},${y1} C${mid},${y1} ${mid},${y2} ${x2},${y2}`;
  }

  function label(e: CpmEdge | undefined): string {
    if (!e) return "";
    const lag = e.lag_days === 0 ? "" : ` ${e.lag_days > 0 ? "+" : "−"}${businessDays(Math.abs(e.lag_days))}`;
    return e.kind === "fs" && lag === "" ? "" : `${e.kind.toUpperCase()}${lag}`;
  }

  function trim(s: string, n = 22): string {
    return s.length > n ? `${s.slice(0, n - 1)}…` : s;
  }
</script>

{#if nodes.length === 0}
  <p class="muted">{t("c.NetworkDiagram.no_scheduled_tasks_to_draw")}</p>
{:else}
  <div class="net">
    <svg
      {width}
      {height}
      viewBox="0 0 {width} {height}"
      role="group"
      aria-label={t("c.NetworkDiagram.task_network_boxes_are_tasks")}
      data-testid="network-diagram"
    >
      <defs>
        <marker id="arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
          <path d="M0,0 L10,5 L0,10 z" fill="currentColor" />
        </marker>
      </defs>
      {#each layout.edges as e (e.from + ">" + e.to)}
        <path class="edge" class:critical={e.critical} d={path(e.from, e.to)} marker-end="url(#arrow)" fill="none" />
        {@const info = edgeInfo.get(`${e.from}>${e.to}`)}
        {#if label(info)}
          {@const a = pos.get(e.from)}
          {@const b = pos.get(e.to)}
          {#if a && b}
            <text class="edge-label" x={(a.x + W + b.x) / 2} y={(a.y + b.y) / 2 + H / 2 - 4} text-anchor="middle">{label(info)}</text>
          {/if}
        {/if}
      {/each}
      {#each nodes as n (n.pid)}
        {@const p = pos.get(n.pid)}
        {@const st = floatState(n)}
        {#if p}
          <g
            class="node {st.key}"
            class:selected={selected === n.pid}
            transform="translate({p.x},{p.y})"
            tabindex="0"
            role="button"
            aria-label={t(st.key === "critical" ? "c.NetworkDiagram.task_summary_critical" : "c.NetworkDiagram.task_summary", { title: n.title ?? t("c.NetworkDiagram.task_fallback"), early_start: n.early_start, early_finish: n.early_finish, total_float: n.total_float })}
            onclick={() => onselect?.(n.pid)}
            onkeydown={(ev) => {
              if (ev.key === "Enter" || ev.key === " ") {
                ev.preventDefault();
                onselect?.(n.pid);
              }
            }}
          >
            <title>{n.title}</title>
            <rect width={W} height={H} rx="6" />
            <text x="8" y="19" class="t">{st.marker} {trim(n.title ?? "(untitled)")}</text>
            <text x="8" y="38" class="s">{t("c.NetworkDiagram.float", { slice: n.early_start.slice(5), slice2: n.early_finish.slice(5), total_float: n.total_float })}</text>
          </g>
        {/if}
      {/each}
    </svg>
    <details>
      <summary>{t("c.NetworkDiagram.the_same_network_as_a")}</summary>
      <ul class="small">
        {#each edges as e (e.pid)}
          <li>
            {byPid.get(e.predecessor_pid)?.title ?? "?"} → {byPid.get(e.successor_pid)?.title ?? "?"}
            ({e.kind.toUpperCase()}{e.lag_days === 0 ? "" : Math.abs(e.lag_days) === 1 ? t("c.NetworkDiagram.lag_one", { lag_days: e.lag_days }) : t("c.NetworkDiagram.lag_other", { lag_days: e.lag_days })})
          </li>
        {/each}
        {#if edges.length === 0}<li>{t("c.NetworkDiagram.no_dependencies")}</li>{/if}
      </ul>
    </details>
  </div>
{/if}

<style>
  .net {
    overflow-x: auto;
  }
  svg {
    color: var(--mxi-color-fg, #333);
    max-width: none;
  }
  .edge {
    stroke: currentColor;
    stroke-width: 1.2;
    opacity: 0.7;
  }
  .edge.critical {
    stroke: #a4262c;
    stroke-width: 3;
    opacity: 1;
  }
  .edge-label {
    font-size: 11px;
    fill: currentColor;
  }
  .node {
    cursor: pointer;
  }
  .node rect {
    fill: var(--mxi-color-bg, #fff);
    stroke: currentColor;
    stroke-width: 1.2;
  }
  .node.critical rect {
    stroke: #a4262c;
    stroke-width: 3.5;
  }
  .node.near rect {
    stroke-dasharray: 6 3;
    stroke-width: 2.5;
  }
  .node.late rect {
    stroke: #7f1d1d;
    stroke-width: 3;
    stroke-dasharray: 2 3;
  }
  .node.selected rect {
    fill: color-mix(in srgb, var(--mxi-color-focus, #1d4ed8) 14%, transparent);
  }
  .node:focus-visible rect {
    outline: 3px solid var(--mxi-color-focus, #1d4ed8);
  }
  .node .t {
    font-size: 13px;
    font-weight: 600;
    fill: currentColor;
  }
  .node .s {
    font-size: 11px;
    fill: currentColor;
    opacity: 0.8;
  }
</style>
