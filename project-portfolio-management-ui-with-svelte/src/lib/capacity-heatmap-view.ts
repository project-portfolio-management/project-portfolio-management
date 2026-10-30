// FB-6: the skill-pool link and the capacity heatmap, as the service sends them (entity spec §8.5a). Pure: the
// pages render what this returns.
//
// The service's rules are kept, never recomputed: for a pool linked to the workforce service the supply is that
// service's alone, its reading is `live`, `last_received` (with its age), `hidden` (under the workforce service's
// floor) or `unknown` (with the reason); what was entered here is shown beside it and never added; a month with no
// figure is unknown, never zero. Every cell carries a word and a symbol as well as a tone (the service's `shape`).

import { currentLocale } from "./money.js";
import { enumLabel } from "./enum-labels.js";
import { t } from "./i18n.svelte.js";

/** The workforce service's supply for one month of a linked pool. */
export interface PeerSupply {
  source: "wpm";
  status: "live" | "last_received" | "hidden" | "unknown";
  supply_fte: number | null;
  /** When the workforce service took the figure. */
  as_of: string | null;
  age_seconds: number | null;
  /** Why there is no live figure (English, from the service). */
  reason: string | null;
}

/** One pool and month of `GET /api/capacity-heatmap`. */
export interface HeatmapCell {
  month: string;
  state: string;
  word: string;
  shape: string;
  colour: string;
  demand_fte: number;
  own_demand_fte: number;
  partner_demand_fte: number;
  supply_fte: number | null;
  own_supply_fte: number | null;
  partner_supply_committed_fte: number | null;
  partner_supply_delivered_fte: number | null;
  gap_fte: number | null;
  partner_delivery_short: boolean;
  partners: number;
  supply_source: "wpm" | "here";
  /** Only for a linked pool: what is held here on the supply side, never added to the workforce service's. */
  entered_here_supply_fte?: number | null;
  /** Only for a linked pool. */
  peer_supply?: PeerSupply;
}

/** One pool's row. `pool_pid` is null for the "no pool assigned" row. */
export interface HeatmapPool {
  pool_pid: string | null;
  name: string;
  peer_pool_ref: string | null;
  cells: HeatmapCell[];
}

/** `GET /api/capacity-heatmap`. */
export interface CapacityHeatmap {
  months: string[];
  pools: HeatmapPool[];
  partner_names_shown: boolean;
  note: string;
}

export type Tone = "red" | "amber" | "green" | "grey" | "none";

/** The service's shape, drawn as a symbol. A shape the page does not know is drawn as nothing (the word remains). */
const SYMBOL: Record<string, string> = {
  triangle: "▲",
  diamond: "◆",
  circle: "●",
  "empty circle": "○",
  dash: "–",
  "question mark": "?",
  blank: "",
};
const TONES: readonly Tone[] = ["red", "amber", "green", "grey", "none"];

/** Whether `text` is the workforce service's pool id as it writes it: a lower-case hyphenated UUID. The same rule
 *  as the service's (`peer_pool_ref_ok`); the text is trimmed first, as the service trims it. */
export function isPeerPoolRef(text: string): boolean {
  return /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/.test(
    text.trim(),
  );
}

/** What a refused link change means: another pool is linked to it already (409), the id was not accepted (422),
 *  the pool is gone (404), or something else. */
export function linkRefusal(
  status: number,
): "taken" | "invalid" | "missing" | "other" {
  if (status === 409) return "taken";
  if (status === 422) return "invalid";
  if (status === 404) return "missing";
  return "other";
}

/** A figure in FTE, in the reader's locale, to at most two places. */
export function fte(n: number, locale: string = currentLocale()): string {
  return new Intl.NumberFormat(locale, { maximumFractionDigits: 2 }).format(n);
}

/** `2026-11` as the reader's month and year; the text as given when it is not a month. */
export function monthLabel(
  month: string,
  locale: string = currentLocale(),
): string {
  const m = /^(\d{4})-(\d{2})$/.exec(month);
  if (!m) return month;
  const d = new Date(Date.UTC(Number(m[1]), Number(m[2]) - 1, 1));
  return new Intl.DateTimeFormat(locale, {
    month: "short",
    year: "numeric",
    timeZone: "UTC",
  }).format(d);
}

/** A moment as the reader's date and time; the text as given when it is not a date. */
export function momentLabel(
  iso: string,
  locale: string = currentLocale(),
): string {
  const d = new Date(iso);
  if (Number.isNaN(d.getTime())) return iso;
  return new Intl.DateTimeFormat(locale, {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(d);
}

/** How long ago, in the reader's words ("5 minutes ago"): seconds, minutes, hours or days. */
export function ageLabel(
  seconds: number,
  locale: string = currentLocale(),
): string {
  const s = Math.max(0, Math.round(seconds));
  const rtf = new Intl.RelativeTimeFormat(locale, { numeric: "always" });
  if (s < 60) return rtf.format(-s, "second");
  if (s < 3600) return rtf.format(-Math.floor(s / 60), "minute");
  if (s < 86400) return rtf.format(-Math.floor(s / 3600), "hour");
  return rtf.format(-Math.floor(s / 86400), "day");
}

/** One cell as shown. */
export interface CellView {
  symbol: string;
  tone: Tone;
  /** The reading in words ("shortfall", "supply unknown"). */
  word: string;
  demand: string;
  /** The supply figure, or the words for why there is none ("unknown", "none recorded"); never a zero made up. */
  supply: string;
  /** Whether a supply figure is shown (false: `supply` is words). */
  supplyKnown: boolean;
  gap: string | null;
  /** Where the supply comes from: the workforce service or what was entered here. */
  source: string;
  linked: boolean;
  /** For a linked pool: the workforce service's reading in words. */
  peerStatus: PeerSupply["status"] | null;
  peerText: string | null;
  /** For a linked pool: what was entered here, shown beside and never added. */
  enteredHere: string | null;
  partnersShort: boolean;
}

/** The workforce service's reading for a month, in words. */
export function peerText(
  p: PeerSupply,
  locale: string = currentLocale(),
): string {
  const when = p.as_of ? momentLabel(p.as_of, locale) : null;
  const reason = p.reason ?? t("p.capacity_heatmap.no_reason");
  switch (p.status) {
    case "live":
      return when
        ? t("p.capacity_heatmap.peer_live", { when })
        : t("p.capacity_heatmap.peer_live_no_time");
    case "last_received":
      return t("p.capacity_heatmap.peer_last", {
        age: p.age_seconds === null ? "?" : ageLabel(p.age_seconds, locale),
        when: when ?? "?",
        reason,
      });
    case "hidden":
      return t("p.capacity_heatmap.peer_hidden");
    default:
      return t("p.capacity_heatmap.peer_unknown", { reason });
  }
}

/** A cell as shown: the service's word, shape and tone, its figures in the reader's locale, and for a linked pool
 *  the source, the reading and what was entered here, side by side. */
export function cellView(
  c: HeatmapCell,
  locale: string = currentLocale(),
): CellView {
  const linked = c.supply_source === "wpm";
  const peer = linked ? (c.peer_supply ?? null) : null;
  const tone = (TONES as readonly string[]).includes(c.colour)
    ? (c.colour as Tone)
    : "grey";
  const supplyKnown = c.supply_fte !== null && c.state !== "supply_unknown";
  const here = c.entered_here_supply_fte ?? null;
  return {
    symbol: SYMBOL[c.shape] ?? "",
    tone,
    word: enumLabel("pk_heat", c.state),
    demand: fte(c.demand_fte, locale),
    supply: supplyKnown
      ? fte(c.supply_fte as number, locale)
      : linked
        ? t("p.capacity_heatmap.unknown")
        : t("p.capacity_heatmap.none_recorded"),
    supplyKnown,
    gap: supplyKnown && c.gap_fte !== null ? fte(c.gap_fte, locale) : null,
    source: linked
      ? t("p.capacity_heatmap.source_wpm")
      : t("p.capacity_heatmap.source_here"),
    linked,
    peerStatus: peer?.status ?? (linked ? "unknown" : null),
    peerText: linked
      ? peer
        ? peerText(peer, locale)
        : t("p.capacity_heatmap.peer_unknown", {
            reason: t("p.capacity_heatmap.no_reason"),
          })
      : null,
    enteredHere: linked
      ? here === null
        ? t("p.capacity_heatmap.entered_here_none")
        : t("p.capacity_heatmap.entered_here", { fte: fte(here, locale) })
      : null,
    partnersShort: c.partner_delivery_short,
  };
}

/** One line of the long table: every pool and month with its figures. */
export interface HeatmapRow {
  key: string;
  pool: string;
  month: string;
  reading: string;
  demand: string;
  supply: string;
  source: string;
  status: string;
  entered_here: string;
}

/** The pool's name for display: the "no pool assigned" row is named in the reader's language. */
export function poolName(p: HeatmapPool): string {
  return p.pool_pid === null ? t("p.capacity_heatmap.no_pool") : p.name;
}

/** Every cell as a row of words, in the service's order. */
export function heatmapRows(
  h: CapacityHeatmap,
  locale: string = currentLocale(),
): HeatmapRow[] {
  return h.pools.flatMap((p) =>
    p.cells.map((c) => {
      const v = cellView(c, locale);
      return {
        key: `${p.pool_pid ?? "none"}:${c.month}`,
        pool: poolName(p),
        month: monthLabel(c.month, locale),
        reading: v.word,
        demand: v.demand,
        supply: v.supply,
        source: v.source,
        status: v.peerText ?? "—",
        entered_here: v.enteredHere ?? "—",
      };
    }),
  );
}
