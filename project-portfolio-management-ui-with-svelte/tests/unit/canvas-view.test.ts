// NC-1 slice 9 (LT-1): the canvas pages' rules: names in the reader's language, items by box, and the checks the
// service makes, said before anything is sent.
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import {
  BOX_LABELS,
  CANVAS_TEXT_MAX,
  CANVAS_TITLE_MAX,
  TEMPLATE_LABELS,
  canvasProblems,
  itemProblem,
  itemsByBox,
  labelFor,
  promoteBody,
  promoteTargets,
  promotedLabel,
} from "../../src/lib/canvas-view";
import type { CanvasItem } from "../../src/lib/api/ppm-types";
import { PAGE_KEYS } from "../../src/lib/i18n-pages";

const item = (pid: string, box: string, body = "x"): CanvasItem => ({
  pid,
  canvas_pid: "c",
  box_key: box,
  body,
  owner_ref: null,
  position: 0,
  promoted_to_kind: null,
  promoted_to_pid: null,
  created_at: "",
  updated_at: "",
});

describe("canvas names", () => {
  // The service's own list (src/canvas.rs): every template and box it has has a name in the catalogue.
  const source = readFileSync(
    join(
      process.cwd(),
      "../project-portfolio-management-service-with-rust/src/canvas.rs",
    ),
    "utf8",
  );
  // Only the `TEMPLATES` table, not the file's own tests.
  const start = source.indexOf("pub const TEMPLATES");
  const rules = source.slice(start, source.indexOf("\n];", start));
  it("names every template and box the service has, and nothing it does not", () => {
    const templates = [...rules.matchAll(/key: "([a-z_]+)",\n\s+title:/g)].map(
      (m) => m[1],
    );
    const boxes = new Set(
      [...rules.matchAll(/\("([a-z_]+)", "[^"]+"\)/g)].map((m) => m[1]),
    );
    expect(templates.sort()).toEqual(Object.keys(TEMPLATE_LABELS).sort());
    expect([...boxes].sort()).toEqual(Object.keys(BOX_LABELS).sort());
    for (const k of [
      ...Object.values(TEMPLATE_LABELS),
      ...Object.values(BOX_LABELS),
    ])
      expect(PAGE_KEYS.has(k)).toBe(true);
  });
  it("uses the service's title, then the key, for a name the catalogue lacks", () => {
    const t = (k: string) => `T:${k}`;
    expect(labelFor(TEMPLATE_LABELS, "swot", "SWOT", t)).toBe(
      "T:p.canvas.template_swot",
    );
    expect(labelFor(TEMPLATE_LABELS, "new_one", "New one", t)).toBe("New one");
    expect(labelFor(TEMPLATE_LABELS, "new_one", undefined, t)).toBe("new_one");
  });
});

describe("items by box", () => {
  it("keeps the service's order within a box and leaves empty boxes out", () => {
    const m = itemsByBox([
      item("a", "threats"),
      item("b", "strengths"),
      item("c", "threats"),
    ]);
    expect(m.get("threats")!.map((i) => i.pid)).toEqual(["a", "c"]);
    expect(m.get("strengths")!.map((i) => i.pid)).toEqual(["b"]);
    expect(m.has("weaknesses")).toBe(false);
  });
});

describe("checks before sending", () => {
  it("refuses a blank or long title and a long description, every problem at once", () => {
    expect(canvasProblems("  ", "")).toEqual(["p.canvas.problem_title_blank"]);
    expect(
      canvasProblems(
        "x".repeat(CANVAS_TITLE_MAX + 1),
        "y".repeat(CANVAS_TEXT_MAX + 1),
      ),
    ).toEqual([
      "p.canvas.problem_title_long",
      "p.canvas.problem_description_long",
    ]);
    expect(canvasProblems(" Plan ", "")).toEqual([]);
    // Characters, not UTF-16 units: 200 emoji are a title of 200.
    expect(canvasProblems("😀".repeat(CANVAS_TITLE_MAX), "")).toEqual([]);
  });
  it("refuses blank or long item text", () => {
    expect(itemProblem(" \n ")).toBe("p.canvas.problem_text_blank");
    expect(itemProblem("x".repeat(CANVAS_TEXT_MAX + 1))).toBe(
      "p.canvas.problem_text_long",
    );
    expect(itemProblem("ok")).toBeNull();
  });
});

describe("turning an item into something", () => {
  it("offers a risk only on a plan's canvas", () => {
    expect(promoteTargets(null)).toEqual(["idea", "objective"]);
    expect(promoteTargets("p")).toEqual(["idea", "objective", "risk"]);
  });
  it("sends a risk's scores, never defaulted, and nothing extra for the others", () => {
    expect(promoteBody("idea", "3", "3")).toEqual({ body: { target: "idea" } });
    expect(promoteBody("risk", "", "4")).toEqual({
      problem: "p.canvas.problem_risk_scores",
    });
    expect(promoteBody("risk", "6", "1")).toEqual({
      problem: "p.canvas.problem_risk_scores",
    });
    expect(promoteBody("risk", "2.5", "1")).toEqual({
      problem: "p.canvas.problem_risk_scores",
    });
    expect(promoteBody("risk", "2", "5")).toEqual({
      body: { target: "risk", probability: 2, impact: 5 },
    });
  });
  it("names what an item became", () => {
    expect(promotedLabel("risk")).toBe("p.canvas.became_risk");
    expect(promotedLabel(null)).toBeNull();
  });
});
