import { describe, expect, it } from "vitest";
import {
  buildWbs,
  looksLikeWbsCode,
  siblingMove,
  taskHref,
  type WbsInput,
} from "../../src/lib/wbs";
import { buildRbs, roleText } from "../../src/lib/rbs";
import {
  diagramFits,
  NETWORK_DIAGRAM_LIMIT,
  restrictedText,
} from "../../src/lib/cpm-view";

const t = (
  pid: string,
  parent: string | null,
  at: number,
  title = pid,
): WbsInput => ({
  pid,
  title,
  parent_task_pid: parent,
  created_at: `2026-10-0${at}T00:00:00Z`,
});

describe("WBS codes", () => {
  it("numbers siblings by age and nests the codes", () => {
    const rows = buildWbs([
      t("c", "a", 3),
      t("a", null, 1),
      t("b", null, 2),
      t("d", "a", 4),
      t("e", "c", 5),
    ]);
    expect(rows.map((r) => `${r.code} ${r.key}`)).toEqual([
      "1 a",
      "1.1 c",
      "1.1.1 e",
      "1.2 d",
      "2 b",
    ]);
    expect(rows.map((r) => r.depth)).toEqual([0, 1, 2, 1, 0]);
    expect(rows[0]?.children).toBe(2);
  });

  it("recomputes the codes when a sibling goes or a task moves", () => {
    const before = buildWbs([
      t("a", null, 1),
      t("b", null, 2),
      t("c", null, 3),
    ]);
    expect(before.find((r) => r.key === "c")?.code).toBe("3");
    const afterDelete = buildWbs([t("a", null, 1), t("c", null, 3)]);
    expect(afterDelete.find((r) => r.key === "c")?.code).toBe("2");
    const afterMove = buildWbs([
      t("a", null, 1),
      t("b", null, 2),
      t("c", "a", 3),
    ]);
    expect(afterMove.find((r) => r.key === "c")?.code).toBe("1.1");
    expect(afterMove.find((r) => r.key === "b")?.code).toBe("2");
  });

  it("keys and links come from the pid, never the code", () => {
    const rows = buildWbs([t("a", null, 1), t("b", "a", 2)]);
    for (const r of rows) {
      expect(looksLikeWbsCode(r.key)).toBe(false);
      expect(taskHref("plan", r.key)).toBe(`/plans/plan/tasks/${r.key}`);
      expect(taskHref("plan", r.key)).not.toContain(r.code + "/");
    }
    expect(looksLikeWbsCode("1.2.3")).toBe(true);
    expect(looksLikeWbsCode("1.2.x")).toBe(false);
    expect(looksLikeWbsCode("a1")).toBe(false);
  });

  it("puts tasks under a parent the reader cannot see beneath a placeholder with no name and no id", () => {
    const rows = buildWbs([
      t("a", null, 1),
      t("kid", "secret-parent", 2, "Visible child"),
    ]);
    const ph = rows.find((r) => r.restricted);
    expect(ph).toMatchObject({ title: "Restricted item", code: "2" });
    expect(JSON.stringify(rows)).not.toContain("secret-parent");
    expect(rows.find((r) => r.key === "kid")?.code).toBe("2.1");
  });

  it("never loops on a cycle and loses nothing", () => {
    const rows = buildWbs([t("a", "b", 1), t("b", "a", 2), t("c", "c", 3)]);
    expect(rows.map((r) => r.key).sort()).toEqual(["a", "b", "c"]);
  });

  it("is repeatable: the same tasks in any order give the same tree", () => {
    const tasks = [
      t("a", null, 1),
      t("b", "a", 2),
      t("c", "a", 3),
      t("d", null, 4),
    ];
    const x = buildWbs([...tasks]);
    const y = buildWbs([...tasks].reverse());
    expect(y).toEqual(x);
  });
});

const plan = (over: object) =>
  ({
    scope: { kind: "all" },
    from: "2026-10-01",
    to: "2026-12-31",
    period: "month",
    group_by: "role",
    overlapping: false,
    periods: [],
    placeholders: [],
    people_by_role: null,
    masked: false,
    note: "",
    ...over,
  }) as never;
const row = (
  key: string,
  demand: number,
  supply: number | null,
  headcount = 0,
) =>
  ({
    key,
    demand_fte: demand,
    named_fte: demand,
    placeholder_fte: 0,
    proposed_fte: 0,
    supply_fte: supply,
    gap_fte: null,
    headcount,
  }) as never;

describe("RBS", () => {
  it("groups people by role and sums demand and supply across the window", () => {
    const roles = plan({
      periods: [
        {
          from: "a",
          to: "b",
          days: 30,
          rows: [row("Engineer", 1.5, 2, 2), row("Designer", 0.5, 1, 1)],
          total_demand_fte: 2,
          total_placeholder_fte: 0,
          total_proposed_fte: 0,
        },
        {
          from: "b",
          to: "c",
          days: 30,
          rows: [row("Engineer", 1, 2, 2)],
          total_demand_fte: 1,
          total_placeholder_fte: 0,
          total_proposed_fte: 0,
        },
      ],
      people_by_role: {
        Engineer: ["person:x", "person:y"],
        Designer: ["person:z"],
      },
    });
    const r = buildRbs(roles, null);
    expect(r.roles.map((x) => x.role)).toEqual(["Designer", "Engineer"]);
    const eng = r.roles.find((x) => x.role === "Engineer")!;
    expect([
      eng.demand_fte,
      eng.supply_fte,
      eng.headcount,
      eng.people.length,
    ]).toEqual([2.5, 4, 2, 2]);
    expect(roleText(eng, false)).toBe(
      "Engineer: 2 named, demand 2.5 FTE, supply 4 FTE",
    );
  });

  it("a reader who may not see people gets a headcount and no names", () => {
    const roles = plan({
      masked: true,
      periods: [
        {
          from: "a",
          to: "b",
          days: 30,
          rows: [row("Engineer", 1, 2, 3)],
          total_demand_fte: 1,
          total_placeholder_fte: 0,
          total_proposed_fte: 0,
        },
      ],
      people_by_role: null,
    });
    const r = buildRbs(roles, null);
    expect(r.masked).toBe(true);
    expect(r.roles[0]?.people).toEqual([]);
    expect(roleText(r.roles[0]!, true)).toBe(
      "Engineer: 3 people, demand 1 FTE, supply 2 FTE",
    );
  });

  it("skills carry demand only, overlapping, with no supply and no people", () => {
    const skills = plan({
      group_by: "skill",
      periods: [
        {
          from: "a",
          to: "b",
          days: 30,
          rows: [row("rust", 1, null), row("sql", 0.5, null)],
          total_demand_fte: 1,
          total_placeholder_fte: 0,
          total_proposed_fte: 0,
        },
      ],
    });
    const r = buildRbs(plan({}), skills);
    expect(r.skills).toEqual([
      { skill: "rust", demand_fte: 1 },
      { skill: "sql", demand_fte: 0.5 },
    ]);
    expect(r.skillNote).toMatch(/not known for skills/);
    expect(
      roleText(
        {
          role: "x",
          people: [],
          headcount: 0,
          demand_fte: 0,
          supply_fte: null,
        },
        true,
      ),
    ).toMatch(/supply not known/);
  });
});

describe("a very large network", () => {
  it("is drawn up to a limit and beyond it the table is the view", () => {
    expect(diagramFits(NETWORK_DIAGRAM_LIMIT)).toBe(true);
    expect(diagramFits(NETWORK_DIAGRAM_LIMIT + 1)).toBe(false);
    expect(diagramFits(0)).toBe(true);
  });
  it("says how many hidden dependencies are left out, never what they are", () => {
    expect(restrictedText(0)).toBe("");
    expect(restrictedText(1)).toBe(
      "1 dependency to a plan you cannot see is left out of these dates.",
    );
    expect(restrictedText(3)).toBe(
      "3 dependencies to a plan you cannot see are left out of these dates.",
    );
  });
});

describe("WBS sibling order", () => {
  const p = (pid: string, parent: string | null, at: number, pos?: number): WbsInput => ({
    ...t(pid, parent, at),
    sibling_position: pos ?? null,
  });

  it("orders by position first, unpositioned after, then by age", () => {
    const rows = buildWbs([p("a", null, 1), p("b", null, 2, 1), p("c", null, 3, 0), p("d", null, 4)]);
    expect(rows.map((r) => r.key)).toEqual(["c", "b", "a", "d"]);
    expect(rows.map((r) => r.code)).toEqual(["1", "2", "3", "4"]);
  });

  it("moves a task one place and names every sibling", () => {
    const tasks = [p("a", null, 1), p("b", null, 2), p("c", null, 3), p("a1", "a", 4), p("a2", "a", 5)];
    expect(siblingMove(tasks, "c", "up")).toEqual({ parent_task_pid: null, order: ["a", "c", "b"] });
    expect(siblingMove(tasks, "a2", "up")).toEqual({ parent_task_pid: "a", order: ["a2", "a1"] });
  });

  it("offers no move at an edge, for an unknown task, or under a parent the reader cannot see", () => {
    const tasks = [p("a", null, 1), p("b", null, 2), p("x", "hidden", 3)];
    expect(siblingMove(tasks, "a", "up")).toBeNull();
    expect(siblingMove(tasks, "b", "down")).toBeNull();
    expect(siblingMove(tasks, "nope", "up")).toBeNull();
    expect(siblingMove(tasks, "x", "up")).toBeNull();
  });
});
