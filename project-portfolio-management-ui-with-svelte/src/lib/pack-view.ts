// The director pack as sections of lines and tables, for the page (DP-11). Pure: it reads the issued
// pack's JSON and the catalogue, and nothing else, so it can show no figure the JSON does not. Every
// fixed word comes through `deps.t` / `deps.label`; free text (names, causes, decisions) is shown as
// entered. A server sentence in the JSON (`note`, `basis`) is not shown: the catalogue carries it.

export type Tone = "red" | "amber" | "green" | "grey";

/** One table cell: text, and for a reading a symbol and a tone, so colour is never alone. */
export interface Cell {
  text: string;
  symbol?: string;
  tone?: Tone;
}

export interface Table {
  /** Names the table for assistive technology. */
  caption: string;
  columns: string[];
  rows: Cell[][];
}

export interface Section {
  id: string;
  title: string;
  lines: string[];
  tables: Table[];
}

export interface Deps {
  t: (key: string, values?: Record<string, string | number>) => string;
  label: (group: string, value: unknown) => string;
  locale: string;
}

type J = Record<string, unknown>;

const obj = (v: unknown): J => (v && typeof v === "object" && !Array.isArray(v) ? (v as J) : {});
const arr = (v: unknown): J[] => (Array.isArray(v) ? v.map(obj) : []);
const str = (v: unknown): string => (typeof v === "string" ? v : "");
const numOrNull = (v: unknown): number | null => (typeof v === "number" && Number.isFinite(v) ? v : null);

const K = "p.director_pack.";

/** Digits after the decimal point a currency's minor unit has, as the platform's own formatter knows. */
export function exponentOf(currency: string, locale: string): number {
  try {
    return new Intl.NumberFormat(locale, { style: "currency", currency }).resolvedOptions().maximumFractionDigits ?? 2;
  } catch {
    return 2;
  }
}

/** `minor` units of `currency` in the reader's own number format. Always names the currency. */
export function money(minor: number | null, currency: string, locale: string): string {
  if (minor === null) return "–";
  const e = exponentOf(currency, locale);
  try {
    return new Intl.NumberFormat(locale, { style: "currency", currency }).format(minor / 10 ** e);
  } catch {
    return `${currency} ${minor}`;
  }
}

const fixed = (n: number | null, places: number, locale: string): string =>
  n === null ? "–" : new Intl.NumberFormat(locale, { minimumFractionDigits: places, maximumFractionDigits: places }).format(n);

const TONE: Record<string, Tone> = {
  green: "green",
  amber_green: "green",
  amber: "amber",
  amber_red: "red",
  red: "red",
  not_assessed: "grey",
};
const SYMBOL: Record<string, string> = {
  green: "●",
  amber_green: "◐",
  amber: "◆",
  amber_red: "▲",
  red: "▲",
  not_assessed: "–",
};
const HEAT: Record<string, { symbol: string; tone: Tone }> = {
  shortfall: { symbol: "▲", tone: "red" },
  tight: { symbol: "◆", tone: "amber" },
  enough: { symbol: "●", tone: "green" },
  no_supply_recorded: { symbol: "–", tone: "grey" },
  no_demand: { symbol: "○", tone: "grey" },
  nothing_recorded: { symbol: "", tone: "grey" },
};

const plain = (text: string): Cell => ({ text });

/** What changed: the service writes these as English sentences; this reads the three shapes it uses. */
function changedLine(line: string, d: Deps): string {
  const m = /^(.+?): (\d+)(?: \(no earlier pack to compare with\)|, unchanged|, was (\d+))$/.exec(line);
  if (!m) return line;
  const [, what, now, before] = m as unknown as [string, string, string, string | undefined];
  const key =
    what === "Open issues" ? "changed_issues" : what === "Change requests waiting for a decision" ? "changed_changes" : null;
  if (!key) return line;
  const change = line.includes("no earlier pack")
    ? d.t(`${K}changed_first`)
    : before !== undefined
      ? d.t(`${K}changed_was`, { before })
      : d.t(`${K}changed_unchanged`);
  return d.t(`${K}${key}`, { now, change });
}

/** Build the sections of a pack's body. */
export function buildPackView(body: Record<string, unknown>, d: Deps): Section[] {
  const out: Section[] = [];
  const add = (id: string, lines: string[] = [], tables: Table[] = []): Section => {
    const s = { id, title: d.t(`${K}s_${id}`), lines, tables };
    out.push(s);
    return s;
  };
  const dash = "–";
  const dq = obj(body.data_quality);
  const missing = (Array.isArray(dq.sections_not_yet_built) ? dq.sections_not_yet_built : []).map(String);
  add("quality", [
    d.t(`${K}plans_included`, { n: numOrNull(dq.plans_included) ?? 0 }),
    d.t(`${K}plans_left_out`, { n: numOrNull(dq.plans_left_out_private_or_marked) ?? 0 }),
    missing.length ? d.t(`${K}sections_missing`, { list: missing.join(", ") }) : d.t(`${K}all_sections`),
  ]);

  const changed = (Array.isArray(body.what_changed) ? body.what_changed : []).map((x) => changedLine(String(x), d));
  add("changed", changed.length ? changed : [d.t(`${K}nothing_here`)]);

  // The portfolio map.
  const map = obj(body.portfolio_map);
  const mapRows: Cell[][] = [];
  for (const g of arr(map.groups)) {
    const aim = g.aim_pid === null || g.aim_pid === undefined ? d.t(`${K}no_aim`) : str(g.aim);
    for (const p of arr(g.programmes)) {
      const c = obj(p.confidence);
      const v = str(c.value);
      const marks = [c.marked_self === true ? d.t(`${K}self_assessed`) : "", c.stale === true ? d.t(`${K}stale`) : ""].filter(Boolean);
      mapRows.push([
        plain(str(p.name)),
        plain(aim),
        plain(d.label("pk_status", str(p.programme_status) || "active")),
        plain(p.stage ? d.label("pk_gate", str(p.stage)) : dash),
        plain(str(p.phase) || dash),
        { text: d.label("pk_conf", v) + (marks.length ? ` (${marks.join(", ")})` : ""), symbol: SYMBOL[v] ?? dash, tone: TONE[v] ?? "grey" },
        plain(d.label("pk_trend", str(p.trend))),
      ]);
    }
  }
  add("map", [], mapRows.length ? [{
    caption: d.t(`${K}s_map`),
    columns: ["c_programme", "c_aim", "c_status", "c_stage", "c_phase", "c_confidence", "c_trend"].map((k) => d.t(`${K}${k}`)),
    rows: mapRows,
  }] : []);
  if (!mapRows.length) out[out.length - 1]!.lines.push(d.t(`${K}nothing_here`));

  // Exceptions.
  const ex = obj(body.exceptions);
  const exLines = [d.t(`${K}ex_window`, { hours: numOrNull(ex.window_hours) ?? 0 })];
  const exTables: Table[] = [];
  for (const [key, label] of [["open", "ex_open"], ["breached", "ex_breached"], ["closed_in_period", "ex_closed"]] as const) {
    const rows = arr(ex[key]);
    exLines.push(d.t(`${K}${label}`, { n: rows.length }));
    if (rows.length) {
      exTables.push({
        caption: d.t(`${K}${label}`, { n: rows.length }),
        columns: ["c_plan", "c_worst", "c_need", "c_owner_role", "c_decide_by", "c_hours_open"].map((k) => d.t(`${K}${k}`)),
        rows: rows.map((e) => [
          plain(str(e.plan)),
          { text: d.label("pk_conf", str(e.worst_value)), symbol: SYMBOL[str(e.worst_value)] ?? dash, tone: TONE[str(e.worst_value)] ?? "grey" },
          plain(str(e.decision_needed) || dash),
          plain(str(e.owner_role) || dash),
          plain(str(e.decide_by) || dash),
          plain(String(numOrNull(e.hours_open) ?? dash)),
        ]),
      });
    }
  }
  add("exceptions", exLines, exTables);

  // Gate decisions.
  const g = obj(body.gate_decisions);
  const gLines: string[] = [];
  const gTables: Table[] = [];
  for (const [key, label] of [["passed", "g_passed"], ["failed", "g_failed"], ["deferred", "g_deferred"]] as const) {
    const rows = arr(g[key]);
    gLines.push(d.t(`${K}${label}`, { n: rows.length }));
    if (rows.length) {
      gTables.push({
        caption: d.t(`${K}${label}`, { n: rows.length }),
        columns: ["c_plan", "c_gate", "c_decided", "c_planned", "c_reason", "c_days_late"].map((k) => d.t(`${K}${k}`)),
        rows: rows.map((l) => [
          plain(str(l.plan)),
          plain(d.label("pk_gate", str(l.gate))),
          plain(str(l.decided_on) || dash),
          plain(str(l.planned_on) || dash),
          plain(l.reason === "reviewed as hold" ? d.t(`${K}reason_hold`) : l.reason === "planned date passed without a review" ? d.t(`${K}reason_no_review`) : dash),
          plain(String(numOrNull(l.days_late) ?? dash)),
        ]),
      });
    }
  }
  for (const [key, label] of [["paused", "g_paused"], ["stopped", "g_stopped"]] as const) {
    const rows = arr(g[key]);
    gLines.push(d.t(`${K}${label}`, { n: rows.length }));
    if (rows.length) {
      gTables.push({
        caption: d.t(`${K}${label}`, { n: rows.length }),
        columns: ["c_programme", "c_since", "c_reason", "c_allocs"].map((k) => d.t(`${K}${k}`)),
        rows: rows.map((p) => [
          plain(str(p.plan)),
          plain(str(p.since) || dash),
          plain(str(p.reason) || dash),
          plain(String(numOrNull(obj(p.still_held).allocations_flagged) ?? dash)),
        ]),
      });
    }
  }
  add("gates", gLines, gTables);

  // Funding.
  const f = obj(body.funding);
  const fy = obj(f.financial_year);
  const fLines = [d.t(`${K}fy_line`, { label: str(fy.label), start: str(fy.start), end: str(fy.end) })];
  const fTables: Table[] = [];
  for (const p of arr(f.programmes)) {
    const rows = arr(p.by_funding_source);
    if (rows.length) {
      fTables.push({
        caption: str(p.name),
        columns: ["c_source", "c_class", "c_budget", "c_spend", "c_variance"].map((k) => d.t(`${K}${k}`)),
        rows: rows.map((r) => {
          const cur = str(r.currency);
          return [
            plain(r.funding_source_pid === null || r.funding_source_pid === undefined ? d.t(`${K}no_funding_source`) : str(r.funding_source)),
            plain(d.label("pk_class", str(r.class))),
            plain(money(numOrNull(r.budget_minor), cur, d.locale)),
            plain(money(numOrNull(r.spend_minor), cur, d.locale)),
            plain(money(numOrNull(r.variance_minor), cur, d.locale)),
          ];
        }),
      });
    }
    for (const fr of arr(obj(p.whole_life_forecast).rows)) {
      const cur = str(fr.currency);
      fLines.push(`${str(p.name)}: ${d.t(`${K}forecast_line`, {
        currency: cur,
        actual: money(numOrNull(fr.actual_minor), cur, d.locale),
        eac: money(numOrNull(fr.estimate_at_completion_minor), cur, d.locale),
      })}`);
    }
  }
  const fq = obj(f.data_quality);
  fLines.push(d.t(`${K}unfunded_lines`, { n: numOrNull(fq.lines_with_no_funding_source) ?? 0, dated: numOrNull(fq.lines_with_no_dates) ?? 0 }));
  add("funding", fLines, fTables);

  // Buffer fever chart: a table, which is its own twin.
  const fev = obj(body.buffer_fever);
  const feverRows: Cell[][] = [];
  const feverLines: string[] = [];
  for (const p of arr(fev.programmes)) {
    const pt = obj(p.point);
    if (p.point === null || p.point === undefined) {
      const code = str(obj(p.no_fever_chart_point).code);
      feverLines.push(`${str(p.plan)}: ${d.t(`${K}fever_none_${code || "no_buffer"}`)}`);
      continue;
    }
    const zone = str(pt.zone);
    const edge = pt.edge === "below" ? ` — ${d.t(`${K}edge_below`)}` : pt.edge === "above" ? ` — ${d.t(`${K}edge_above`)}` : "";
    feverRows.push([
      plain(str(p.plan)),
      plain(fixed(numOrNull(pt.chain_completed_pct), 1, d.locale)),
      plain(fixed(numOrNull(pt.buffer_consumed_pct), 1, d.locale) + edge),
      { text: d.label("pk_zone", zone), symbol: SYMBOL[zone] ?? dash, tone: TONE[zone] ?? "grey" },
      plain(String(numOrNull(pt.slip_days) ?? dash)),
      plain(String(numOrNull(pt.buffer_days) ?? dash)),
      plain(p.previous && obj(p.previous).zone ? d.label("pk_zone", str(obj(p.previous).zone)) : dash),
    ]);
  }
  const lines = obj(fev.lines);
  const ga = obj(lines.green_amber);
  const ar = obj(lines.amber_red);
  if (numOrNull(ga.at_start) !== null) {
    feverLines.push(d.t(`${K}fever_lines`, {
      ga0: fixed(numOrNull(ga.at_start), 0, d.locale), ga1: fixed(numOrNull(ga.at_end), 0, d.locale),
      ar0: fixed(numOrNull(ar.at_start), 0, d.locale), ar1: fixed(numOrNull(ar.at_end), 0, d.locale),
    }));
  }
  add("fever", feverLines, feverRows.length ? [{
    caption: d.t(`${K}s_fever`),
    columns: ["c_programme", "c_chain", "c_buffer", "c_zone", "c_slip", "c_buffer_days", "c_previous"].map((k) => d.t(`${K}${k}`)),
    rows: feverRows,
  }] : []);

  // Earned value.
  const ev = obj(body.earned_value);
  const evRows: Cell[][] = [];
  const evLines: string[] = [d.t(`${K}evm_ratio_note`)];
  const ratio = (v: unknown) => fixed(numOrNull(obj(v).bp) === null ? null : (numOrNull(obj(v).bp) as number) / 10000, 2, d.locale);
  for (const p of arr(ev.programmes)) {
    if (p.state === "no_baseline") evLines.push(`${str(p.name)}: ${d.t(`${K}no_baseline`)}`);
    for (const c of arr(p.currencies)) {
      const cur = str(c.currency);
      if (c.state === "included") {
        const tot = obj(c.totals);
        evRows.push([
          plain(str(p.name)), plain(cur),
          plain(money(numOrNull(tot.bac_minor), cur, d.locale)),
          plain(money(numOrNull(tot.ev_minor), cur, d.locale)),
          plain(money(numOrNull(tot.ac_minor), cur, d.locale)),
          plain(ratio(c.spi)), plain(ratio(c.cpi)),
        ]);
      } else {
        evLines.push(`${str(p.name)}: ${d.t(`${K}below_threshold`, { bac: money(numOrNull(c.bac_minor), cur, d.locale) })}`);
      }
    }
  }
  add("evm", evLines, evRows.length ? [{
    caption: d.t(`${K}s_evm`),
    columns: ["c_programme", "c_currency", "c_bac", "c_ev", "c_ac", "c_spi", "c_cpi"].map((k) => d.t(`${K}${k}`)),
    rows: evRows,
  }] : []);

  // Benefits.
  const ben = obj(body.benefits);
  const benRows = arr(ben.totals).map((r) => {
    const cur = str(r.currency);
    return [
      plain(cur),
      plain(d.label("pk_recur", str(r.recurrence))),
      plain(d.label("pk_cls", str(r.classification))),
      plain(money(numOrNull(r.planned_minor), cur, d.locale)),
      plain(money(numOrNull(r.claimed_minor), cur, d.locale)),
      plain(money(numOrNull(r.validated_minor), cur, d.locale)),
      plain(money(numOrNull(r.unvalidated_claim_minor), cur, d.locale)),
    ];
  });
  add("benefits", [d.t(`${K}benefits_note`)], benRows.length ? [{
    caption: d.t(`${K}s_benefits`),
    columns: ["c_currency", "c_recurrence", "c_classification", "c_planned", "c_claimed", "c_validated", "c_unvalidated"].map((k) => d.t(`${K}${k}`)),
    rows: benRows,
  }] : []);

  // Capacity: pool by month, a word and a symbol as well as a tone in every cell.
  const cap = obj(body.capacity_heatmap);
  const months = (Array.isArray(cap.months) ? cap.months : []).map(String);
  const capRows: Cell[][] = [];
  for (const p of arr(cap.pools)) {
    const name = p.pool_pid === null || p.pool_pid === undefined ? d.t(`${K}no_pool`) : str(p.name);
    for (const c of arr(p.cells)) {
      const st = str(c.state);
      const h = HEAT[st] ?? { symbol: "", tone: "grey" as Tone };
      capRows.push([
        plain(name),
        plain(str(c.month)),
        { text: d.label("pk_heat", st) + (c.partner_delivery_short === true ? `; ${d.t(`${K}partners_short`)}` : ""), symbol: h.symbol, tone: h.tone },
        plain(fixed(numOrNull(c.demand_fte), 2, d.locale)),
        plain(fixed(numOrNull(c.supply_fte), 2, d.locale)),
        plain(fixed(numOrNull(c.gap_fte), 2, d.locale)),
      ]);
    }
  }
  void months;
  add("capacity", [d.t(`${K}capacity_note`)], capRows.length ? [{
    caption: d.t(`${K}s_capacity`),
    columns: ["c_pool", "c_month", "c_reading", "c_demand", "c_supply", "c_gap"].map((k) => d.t(`${K}${k}`)),
    rows: capRows,
  }] : []);

  // Milestones that need partner sign-off: delivered and met are separate, and the gap is a row.
  const so = obj(body.milestone_signoffs);
  const tot = obj(so.totals);
  const SO: Record<string, { key: string; symbol: string; tone: Tone }> = {
    met: { key: "so_met", symbol: "●", tone: "green" },
    delivered_not_met: { key: "so_delivered_not_met", symbol: "▲", tone: "red" },
    not_delivered: { key: "so_not_delivered", symbol: "–", tone: "grey" },
  };
  const soRows: Cell[][] = arr(so.milestones).map((m) => {
    const sg = obj(m.signoff);
    const st = SO[str(sg.state)] ?? { key: "so_not_delivered", symbol: "–", tone: "grey" as Tone };
    return [
      plain(str(m.name)),
      plain(str(m.due)),
      { text: d.t(`${K}${st.key}`), symbol: st.symbol, tone: st.tone },
      plain(str(sg.delivered_on)),
      plain(str(sg.met_on)),
      plain(Array.isArray(sg.outstanding) ? sg.outstanding.map(String).join(", ") : ""),
    ];
  });
  const soShare = numOrNull(tot.met_share_of_delivered_bp);
  add(
    "signoffs",
    [
      d.t(`${K}signoffs_note`),
      d.t(`${K}signoffs_totals`, {
        n: numOrNull(tot.requiring_signoff) ?? 0,
        delivered: numOrNull(tot.delivered) ?? 0,
        met: numOrNull(tot.met) ?? 0,
        gap: numOrNull(tot.delivered_not_met) ?? 0,
        share: soShare === null ? d.t(`${K}so_no_share`) : `${fixed(soShare / 100, 0, d.locale)}%`,
      }),
    ],
    soRows.length ? [{
      caption: d.t(`${K}s_signoffs`),
      columns: ["c_milestone", "c_due", "c_reading", "c_delivered_on", "c_met_on", "c_outstanding"].map((k) => d.t(`${K}${k}`)),
      rows: soRows,
    }] : [],
  );

  add("issues", [d.t(`${K}open_issues`, { n: numOrNull(obj(body.issues).open) ?? 0 })]);
  add("changes", [d.t(`${K}awaiting`, { n: numOrNull(obj(body.change_requests).awaiting_decision) ?? 0 })]);
  return out;
}
