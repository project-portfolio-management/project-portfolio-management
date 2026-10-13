// The core of the fictitious portfolio: organizations, the plan tree, tasks and their dependencies,
// milestones, risks, issues, budget, benefits. Returns the plans by name for the later groups.

import { get, post, put, tryPost, ensure, log, day } from "./lib.mjs";

// Fictitious people. The API wants `person:<uuid>` references, so each has a fixed made-up id.
export const PEOPLE = [
  ["amara-okafor", "Amara Okafor", "Programme director", "00000000-0000-4000-8000-0000000000a1"],
  ["ben-whitfield", "Ben Whitfield", "Signalling engineer", "00000000-0000-4000-8000-0000000000a2"],
  ["chloe-martin", "Chloe Martin", "Project manager", "00000000-0000-4000-8000-0000000000a3"],
  ["dev-patel", "Dev Patel", "Software developer", "00000000-0000-4000-8000-0000000000a4"],
  ["elena-rossi", "Elena Rossi", "Accessibility lead", "00000000-0000-4000-8000-0000000000a5"],
  ["farid-haddad", "Farid Haddad", "Test engineer", "00000000-0000-4000-8000-0000000000a6"],
  ["grace-liu", "Grace Liu", "Finance partner", "00000000-0000-4000-8000-0000000000a7"],
  ["hugo-bennett", "Hugo Bennett", "Operations manager", "00000000-0000-4000-8000-0000000000a8"],
];
export const ref = (slug) => `person:${PEOPLE.find((p) => p[0] === slug)[3]}`;

const ORGS = [
  ["Northgate Transport Authority", "sponsor"],
  ["Brightline Engineering", "delivery"],
  ["Harbour Rail Partners", "partner"],
  ["Kestrel Signalling Ltd", "supplier"],
];

// name, kind, parent name, start offset (days from 12 Oct 2026), length (days), lead
const PLANS = [
  ["Northgate Rail Modernisation", "Portfolio", null, -190, 720, "amara-okafor"],
  ["Signalling Renewal", "Program", "Northgate Rail Modernisation", -190, 520, "ben-whitfield"],
  ["Station Accessibility", "Program", "Northgate Rail Modernisation", -150, 480, "elena-rossi"],
  ["Depot Digitisation", "Program", "Northgate Rail Modernisation", -100, 420, "hugo-bennett"],
  ["Interlocking Replacement, North Line", "Project", "Signalling Renewal", -190, 380, "ben-whitfield"],
  ["Trackside Equipment Upgrade", "Project", "Signalling Renewal", -120, 330, "farid-haddad"],
  ["Step-free Access, Central Station", "Project", "Station Accessibility", -150, 360, "elena-rossi"],
  ["Wayfinding and Announcements", "Project", "Station Accessibility", -90, 240, "chloe-martin"],
  ["Fleet Maintenance System", "Project", "Depot Digitisation", -100, 300, "dev-patel"],
  ["Depot Wi-Fi and Sensors", "Project", "Depot Digitisation", -60, 200, "hugo-bennett"],
];

// Per project: [title, status, assignee slug, points, start offset in the project, duration days]
const TASKS = [
  ["Survey the existing equipment", "done", "ben-whitfield", 3, 0, 15],
  ["Design the replacement", "done", "ben-whitfield", 8, 15, 30],
  ["Order long-lead items", "done", "grace-liu", 2, 40, 10],
  ["Build and bench-test", "in_progress", "dev-patel", 8, 55, 45],
  ["Install on site", "todo", "farid-haddad", 13, 100, 40],
  ["Safety assurance", "blocked", "amara-okafor", 5, 130, 25],
  ["Train operators", "todo", "hugo-bennett", 3, 150, 15],
  ["Commission and hand over", "todo", "chloe-martin", 5, 170, 20],
];

const MILESTONES = [["Design approved", 45], ["Equipment delivered", 100], ["Go-live", 190]];

const RISKS = [
  ["Supplier delivery slips by a month", 4, 4, "delivery"],
  ["Possession of the track is refused", 3, 5, "compliance"],
  ["Key engineer leaves the project", 2, 4, "other"],
];

export async function seedCore() {
  log("Core: organizations, plans, tasks, milestones, risks, budget, benefits");
  const orgs = (await get("/api/organizations")).organizations;
  for (const [name, kind] of ORGS) {
    await ensure(orgs, (o) => o.name === name, () => post("/api/organizations", { name, kind }));
  }

  const existing = await get("/api/plans");
  const plans = {};
  for (const [name, kind, parent, startOff, len, lead] of PLANS) {
    const found = existing.find((p) => p.name === name);
    if (found) { plans[name] = found.pid; continue; }
    const created = await post("/api/plans", {
      kind, name, status: "Active", lead_ref: ref(lead), owner_org_name: "Northgate Transport Authority",
      parent_ref: parent ? plans[parent] : null,
      start_date: day(startOff), target_date: day(startOff + len),
      keywords: ["demonstration", "fictitious"], tags: ["northgate"],
    });
    plans[name] = created.pid;
  }

  for (const [name, kind] of PLANS.map((p) => [p[0], p[1]])) {
    if (kind !== "Project") continue;
    const pid = plans[name];
    const start = PLANS.find((p) => p[0] === name)[3];
    const have = (await get(`/api/plans/${pid}/tasks`)).tasks;
    if (have.length === 0) {
      const made = [];
      for (const [title, status, who, points, off, dur] of TASKS) {
        const t = await post(`/api/plans/${pid}/tasks`, {
          title: `${title}`, status, assignee_ref: ref(who), points,
          start_date: day(start + off), duration_days: dur, due_date: day(start + off + dur),
        });
        made.push(t);
      }
      for (let i = 0; i + 1 < made.length; i++) {
        await tryPost("/api/dependencies", { predecessor_pid: made[i].pid, successor_pid: made[i + 1].pid, lag_days: 0 }, "dependency");
      }
    }
    const haveMs = (await get(`/api/plans/${pid}/milestones`)).milestones ?? (await get(`/api/plans/${pid}/milestones`));
    const msList = Array.isArray(haveMs) ? haveMs : [];
    if (msList.length === 0) {
      for (const [mname, off] of MILESTONES) {
        await post(`/api/plans/${pid}/milestones`, { name: mname, due: day(start + off) });
      }
    }
    const haveRisks = await get(`/api/plans/${pid}/risks`);
    if ((Array.isArray(haveRisks) ? haveRisks : haveRisks.risks ?? []).length === 0) {
      for (const [title, probability, impact, category] of RISKS) {
        await post(`/api/plans/${pid}/risks`, {
          title, probability, impact, category, owner_ref: ref("amara-okafor"),
          mitigation: "Agreed with the supplier and reviewed monthly.", review_date: day(30),
        });
      }
    }
    const budget = await get(`/api/plans/${pid}/budget-lines`);
    if ((budget.lines ?? budget.budget_lines ?? []).length === 0) {
      const lines = [["capex", "Equipment", 480_000_00], ["opex", "Labour", 220_000_00], ["opex", "Training", 35_000_00]];
      for (const [category, description, planned] of lines) {
        const l = await post(`/api/plans/${pid}/budget-lines`, { category, description, currency: "GBP", planned_minor: planned, period_start: day(start), period_end: day(start + 380) });
        await tryPost(`/api/plans/${pid}/budget-lines/${l.pid}/actual`, { amount_minor: Math.round(planned * 0.42), note: "To date" }, "actual");
      }
    }
    const bens = await get(`/api/plans/${pid}/benefits`);
    if ((bens.benefits ?? []).length === 0) {
      await post(`/api/plans/${pid}/benefits`, { title: "Fewer delay minutes a year", category: "quality", expected_on: day(start + 400), target_note: "30% fewer delay minutes" });
      await post(`/api/plans/${pid}/benefits`, { title: "Lower maintenance cost", category: "cost_saving", currency: "GBP", target_minor: 150_000_00, expected_on: day(start + 420) });
    }
  }
  return plans;
}
