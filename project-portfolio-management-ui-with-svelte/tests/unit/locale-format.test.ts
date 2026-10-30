// Numbers, percentages and dates in the page's locale (WW-1.16 and WW-1.24 in part; UI spec "Numbers, dates and
// plurals"), and a scan of `src/` that fails on new hand-rolled formatting: `toFixed`, `toLocale*`, a percent sign
// glued to a number, a timestamp sliced to a date, an ISO date field put straight into markup or a `t` placeholder.
// Ported from the workforce planning management UI's test of the same name (WPM-T375), with PPM's locales.

import { readdirSync, readFileSync, statSync } from "node:fs";
import { join, relative } from "node:path";
import { describe, expect, it } from "vitest";

import {
  DASH,
  arrow,
  formatBasisPoints,
  formatDateRange,
  formatDate,
  formatDateTime,
  formatDecimal,
  formatInteger,
  formatMonth,
  formatNumber,
  formatPercent,
  formatPercentValue,
  formatSigned,
  formatTime,
  numberToInput,
  parseNumber,
  parseScaled,
} from "../../src/lib/locale-format";
import { percent } from "../../src/lib/api/tba";
import { percentText } from "../../src/lib/okr-view";
import { i18n } from "../../src/lib/i18n.svelte";
import { businessDays } from "../../src/lib/units";

const NBSP = " ";
const NNBSP = " ";

describe("numbers in the page's locale", () => {
  it("groups and marks decimals the locale's way, in its own digits", () => {
    expect(formatNumber(1234.5, {}, "en-001")).toBe("1,234.5");
    expect(formatNumber(1234.5, {}, "de-001")).toBe("1.234,5");
    expect(formatNumber(1234.5, {}, "fr-001")).toBe(`1${NNBSP}234,5`);
    expect(formatNumber(1234567, {}, "hi-001")).toBe("12,34,567");
    expect(formatNumber(1234.5, {}, "es-001")).toBe("1234,5");
    // A regional locale formats as its region does.
    expect(formatNumber(1234.5, {}, "de-de")).toBe("1.234,5");
  });

  it("shows an unknown figure as a dash, never 0", () => {
    expect(formatNumber(null)).toBe(DASH);
    expect(formatNumber(undefined)).toBe(DASH);
    expect(formatNumber(Number.NaN)).toBe(DASH);
    expect(formatNumber(0, {}, "en-001")).toBe("0");
    expect(formatPercent(null)).toBe(DASH);
    expect(formatBasisPoints(null)).toBe(DASH);
  });

  it("writes fixed decimals, integers and signs", () => {
    expect(formatDecimal(2.5, 2, "en-001")).toBe("2.50");
    expect(formatDecimal(2.5, 2, "de-001")).toBe("2,50");
    expect(formatInteger(1234.4, "en-001")).toBe("1,234");
    expect(formatSigned(3, {}, "en-001")).toBe("+3");
    expect(formatSigned(-1.5, {}, "en-001")).toBe("-1.5");
    expect(formatSigned(0, {}, "en-001")).toBe("0");
  });

  it("writes a percentage the locale's way, sign and space included", () => {
    expect(formatPercent(0.25, 0, "en-001")).toBe("25%");
    expect(formatPercent(0.25, 0, "de-001")).toBe(`25${NBSP}%`);
    expect(formatPercent(0.25, 0, "fr-001")).toBe(`25${NBSP}%`);
    expect(formatPercentValue(8.3, 1, "en-001")).toBe("8.3%");
    expect(formatPercentValue(8.3, 1, "de-001")).toBe(`8,3${NBSP}%`);
  });

  it("places the point of basis points and centi-FTE exactly", () => {
    expect(formatBasisPoints(10456, 2, "en-001")).toBe("104.56%");
    expect(formatBasisPoints(-5, 2, "en-001")).toBe("-0.05%");
    expect(formatBasisPoints(909, 2, "de-001")).toBe(`9,09${NBSP}%`);
  });

  it("is what the null-not-zero percentage helpers now write", () => {
    expect(percent(0.25)).toBe("25%");
    expect(percent(0.125, 1)).toBe("12.5%");
    expect(percent(null)).toBe("—");
    expect(percentText(4000)).toBe("40%");
    expect(percentText(4050)).toBe("40.5%");
  });
});

describe("dates in the page's locale", () => {
  it("writes an ISO date the locale's way, the same day in every time zone", () => {
    expect(formatDate("2026-10-30", "medium", "en-001")).toBe("30 Oct 2026");
    expect(formatDate("2026-10-30", "medium", "de-de")).toBe("30.10.2026");
    expect(formatDate("2026-10-30", "medium", "zh-cn")).toBe("2026年10月30日");
    expect(formatDate("2026-10-30", "medium", "cy-001")).toBe("30 Hyd 2026");
    expect(formatDate("2026-01-01", "medium", "en-001")).toBe("1 Jan 2026");
  });

  it("shows a missing date as a dash and leaves text that is not a date alone", () => {
    expect(formatDate(null)).toBe(DASH);
    expect(formatDate("")).toBe(DASH);
    expect(formatDate("soon")).toBe("soon");
    expect(formatDate("2026-13-45", "medium", "en-001")).toBe("2026-13-45");
  });

  it("writes a timestamp as a date, a date and time, or a time", () => {
    const at = "2026-10-30T12:00:00Z";
    expect(formatDate(at, "medium", "en-001")).toMatch(/Oct 2026$/);
    expect(formatDateTime(at, "en-001")).toMatch(/2026.*\d{1,2}[:.]\d{2}/);
    expect(formatTime(at, "en-001")).toMatch(/^\d{1,2}[:.]\d{2}/);
    // A civil date has no time of day.
    expect(formatDateTime("2026-10-30", "en-001")).toBe("30 Oct 2026");
  });

  it("writes a month as a month and year", () => {
    expect(formatMonth("2026-10", "short", "en-001")).toBe("Oct 2026");
    expect(formatMonth("2026-10-01", "long", "en-001")).toBe("October 2026");
    expect(formatMonth("2026-10", "long", "de-001")).toBe("Oktober 2026");
  });

  it("writes a range with the locale's own separator, and an arrow along the writing direction", () => {
    expect(
      formatDateRange("2026-09-01", "2026-09-30", "medium", "en-001"),
    ).toBe("1–30 Sept 2026");
    expect(formatDateRange("2026-09-01", null, "medium", "en-001")).toBe(
      "1 Sept 2026 – …",
    );
    expect(arrow("en-001")).toBe("→");
    expect(arrow("ar-001")).toBe("←");
  });
});

describe("numbers typed in the page's locale", () => {
  it("reads the locale's decimal mark, through money.ts's parser", () => {
    expect(parseNumber("2,5", "de-001")).toBe(2.5);
    expect(parseNumber("2.5", "en-001")).toBe(2.5);
    expect(parseNumber("1.234,5", "de-001")).toBe(1234.5);
    expect(parseNumber("two", "en-001")).toBeNull();
    expect(parseNumber("", "en-001")).toBeNull();
  });

  it("scales a typed percentage to the API's integer without float error", () => {
    expect(parseScaled("2,5", 2, "de-001")).toBe(250);
    expect(parseScaled("0.8", 4, "en-001")).toBe(8000);
    // 1.005 * 100 is 100.49999…; "1.005e2" is exactly 100.5.
    expect(parseScaled("1.005", 2, "en-001")).toBe(101);
    expect(parseScaled("x", 2, "en-001")).toBeNull();
  });

  it("writes a number back for a text input with no grouping", () => {
    expect(numberToInput(2.5, "de-001")).toBe("2,5");
    expect(numberToInput(1234.5, "en-001")).toBe("1234.5");
    expect(numberToInput(null)).toBe("");
  });
});

describe("the views follow the page's locale", () => {
  it("writes a percentage, a date and a count in German and Welsh pages", () => {
    try {
      i18n.set("de-de");
      expect(percent(0.125, 1)).toBe(`12,5${NBSP}%`);
      expect(percentText(4050)).toBe(`40,5${NBSP}%`);
      expect(formatDate("2026-10-30")).toBe("30.10.2026");
      i18n.set("cy-001");
      expect(formatDate("2026-10-30")).toBe("30 Hyd 2026");
      expect(businessDays(1)).toBe("1 diwrnod gwaith");
    } finally {
      i18n.set("en-001");
    }
  });
});

// The scan. A line that matches is hand-rolled formatting of something a person reads. Allowed: comments, CSS, and a
// line in ALLOW with its reason. The string catalogues are not scanned (WPM's are JSON, which its scan does not read
// either); what they still hold is counted by the next describe block.
const here = join(__dirname, "..", "..");
const ALLOW: Record<string, string> = {
  "src/lib/capacity-commitments-view.ts:const key = `${c.pool_pid}|${c.plan_pid}|${c.month}`;":
    "a map key, never shown",
  'src/lib/capacity-heatmap-view.ts:key: `${p.pool_pid ?? "none"}:${c.month}`,':
    "a row key, never shown",
  "src/lib/ev-view.ts:.map((p) => `${x(at(p.date)).toFixed(1)},${y(p.pv_minor).toFixed(1)}`)":
    "SVG path coordinates, never shown",
  'src/routes/capacity-heatmap/+page.svelte:<td class={`cell tone-${v.tone}`} data-testid={`cell-${p.pool_pid ?? "none"}-${c.month}`} data-state={c.state}>':
    "a test id, never shown",
};
const CATALOGUES = /i18n\.svelte\.ts$|i18n-pages(?:\.cy)?\.ts$/;
const FIELD = String.raw`[\w.?]*?(?:\w+_(?:on|at|date|start|end|until|from|due|day)\b|decide_by|remedy_by|as_of|\.date\b|\.on\b|\.month\b)`;
const RULES: [string, RegExp][] = [
  ["toFixed", /\.toFixed\(/],
  ["toLocale*", /\.toLocale(?:String|DateString|TimeString)\(/],
  ["a percent sign glued to a number", /(?:\$\{[^}]*\}|\}|\))%(?![\w;"])/],
  [
    "a timestamp sliced to a date",
    /\b(?:\w+(?:_at|_on|_date|_off|month)|String\(v\))\b[^;\n]*?\.slice\(0, (?:7|10|16|19)\)/,
  ],
  [
    "an ISO date in markup",
    new RegExp(String.raw`\{\s*${FIELD}\s*(?:\?\?[^}]*)?\}`),
  ],
  [
    "an ISO date in a placeholder",
    new RegExp(
      String.raw`\btn?\([^)]*?\b\w+:\s*\(?${FIELD}(?!\()\s*(?:\?\?\s*[^,}]+)?\)?\s*[,}]`,
    ),
  ],
  [
    "an ISO date in a template string",
    new RegExp(String.raw`\$\{\s*${FIELD}\s*\}`),
  ],
];

function walk(dir: string): string[] {
  const out: string[] = [];
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) out.push(...walk(path));
    else if (/\.(svelte|ts)$/.test(path)) out.push(path);
  }
  return out;
}

/** Hand-rolled formatting in one file's text, as "rule: line". */
export function handRolled(text: string, file = ""): string[] {
  const found: string[] = [];
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (/^(\/\/|\*|\/\*|<!--)/.test(line)) continue;
    if (/style=|width:|height:|type="date"/.test(line)) continue;
    if (ALLOW[`${file}:${line}`]) continue;
    for (const [name, re] of RULES)
      if (re.test(line)) found.push(`${name}: ${line.slice(0, 120)}`);
  }
  return found;
}

describe("no hand-rolled number or date formatting in src/", () => {
  it("finds it when it is there (the scan works)", () => {
    expect(handRolled("const x = (n / 100).toFixed(2);")).toHaveLength(1);
    expect(handRolled("const s = `${Math.round(v * 100)}%`;")).toHaveLength(1);
    expect(handRolled("<td>{row.target_on}</td>")).toHaveLength(1);
    expect(handRolled('{t("p.x.due", { day: r.due_on })}')).toHaveLength(1);
    expect(handRolled('{t("p.x", { date: r.decide_by })}')).toHaveLength(1);
    expect(handRolled("{a.performed_at.slice(0, 16)}")).toHaveLength(1);
    expect(handRolled("new Date(x).toLocaleTimeString([])")).toHaveLength(1);
    // …and not when the value goes through the module.
    expect(handRolled("<td>{formatDate(row.target_on)}</td>")).toEqual([]);
    expect(handRolled('{t("p.x.due", { day: formatDate(r.due_on) })}')).toEqual(
      [],
    );
    expect(handRolled('<div style="width: {pct}%"></div>')).toEqual([]);
  });

  it("finds none", () => {
    const offenders: string[] = [];
    for (const file of walk(join(here, "src"))) {
      if (/locale-format\.ts$|money\.ts$/.test(file) || CATALOGUES.test(file))
        continue;
      const rel = relative(here, file);
      for (const f of handRolled(readFileSync(file, "utf8"), rel))
        offenders.push(`${rel}: ${f}`);
    }
    expect(offenders, offenders.join("\n")).toEqual([]);
  });
});

// A catalogue string that puts "%" after a number placeholder writes the percentage the English way ("25%", where
// German writes "25 %" and Turkish "%25"); the number itself is formatted by the caller. Moving each to
// `formatPercent` changes English and Welsh catalogue text and is not done yet (WW-1.24, remaining). This count may
// only go down.
describe("catalogue strings with a percent sign after a placeholder", () => {
  it("are no more than when the scan was added", () => {
    const glued = (text: string) =>
      text.split("\n").filter((line) => line.includes("}%")).length;
    const en = readFileSync(join(here, "src/lib/i18n-pages.ts"), "utf8");
    const cy = readFileSync(join(here, "src/lib/i18n-pages.cy.ts"), "utf8");
    expect(glued(en)).toBeLessThanOrEqual(30);
    expect(glued(cy)).toBeLessThanOrEqual(30);
  });
});
