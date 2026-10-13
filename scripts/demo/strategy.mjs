// Governance and strategy: gate reviews, proposals and ideas, issues, change requests, objectives and key
// results, KPIs with readings, announcements, a scenario. Needs the plans from `core.mjs`.

import { get, post, tryPost, ensure, log, day } from "./lib.mjs";
import { ref } from "./core.mjs";

const list = (r, key) => (Array.isArray(r) ? r : r?.[key] ?? []);

export async function seedStrategy(plans) {
  log("Strategy: gates, proposals, ideas, issues, change requests, objectives, KPIs, announcements");
  const projects = Object.entries(plans).filter(([n]) => !["Northgate Rail Modernisation", "Signalling Renewal", "Station Accessibility", "Depot Digitisation"].includes(n));

  // Gate reviews: the first projects are further along than the later ones.
  const gates = ["g0_concept", "g1_feasibility", "g2_definition", "g3_delivery"];
  for (const [i, [name, pid]] of projects.entries()) {
    const upTo = Math.max(1, gates.length - Math.floor(i / 2));
    for (const gate of gates.slice(0, upTo)) {
      // A gate already passed is refused as out of order: that is the "already done" answer on a second run.
      try {
        await post(`/api/plans/${pid}/gate-reviews`, { gate, decision: "approved", approver_ref: ref("amara-okafor"), conditions: null });
      } catch (e) {
        if (!(e.status === 422 && /out-of-order/.test(e.message))) throw e;
      }
    }
  }

  // Proposals in different states.
  const have = list(await get("/api/proposals"), "proposals");
  const props = [
    ["Level crossing sensors", "Project", 380_000_00, "Cuts false alarms at 14 crossings.", ["submit"]],
    ["Passenger information screens", "Project", 210_000_00, "Real-time boards on every platform.", ["submit", "review"]],
    ["Energy-saving depot lighting", "Project", 95_000_00, "Pays back inside three years.", ["submit", "review", "approve"]],
    ["Ticket-office relocation", "Project", 150_000_00, "Frees the concourse for step-free routes.", ["submit", "review", "reject"]],
    ["Driver rostering tool", "Project", 120_000_00, "Replaces spreadsheets.", []],
  ];
  for (const [title, kind_target, requested_minor, summary, steps] of props) {
    if (have.some((p) => p.title === title)) continue;
    const p = await post("/api/proposals", { title, summary, kind_target, sponsor_ref: ref("amara-okafor"), strategic_rationale: "Supports the Northgate modernisation objectives.", requested_minor, currency: "GBP" });
    for (const step of steps) await tryPost(`/api/proposals/${p.pid}/${step}`, {}, `proposal ${step}`);
  }

  // Ideas with votes.
  const ideas = list(await get("/api/ideas"), "ideas");
  for (const [title, pitch, votes] of [
    ["Solar canopies over car parks", "Generate power and shelter commuters.", 4],
    ["Quiet carriages on the evening peak", "A trial on the north line.", 7],
    ["Bike lockers at every station", "Secure storage, booked in the app.", 3],
    ["Staff mentoring scheme", "Pair new signallers with veterans.", 2],
  ]) {
    const found = ideas.find((i) => i.title === title);
    const idea = found ?? (await post("/api/ideas", { title, pitch, tags: ["demonstration"] }));
    if (!found) for (let v = 0; v < votes; v++) await tryPost(`/api/ideas/${idea.pid}/vote`, {}, "vote");
  }

  // Issues and change requests on two projects.
  const [[, p1], [, p2]] = projects;
  const issues = list(await get(`/api/plans/${p1}/issues`), "issues");
  if (issues.length === 0) {
    await tryPost(`/api/plans/${p1}/issues`, { title: "Cable route blocked by a gas main", description: "Found during the survey.", severity: "high", owner_ref: ref("ben-whitfield"), raised_on: day(-20), due_date: day(10) }, "issue");
    await tryPost(`/api/plans/${p1}/issues`, { title: "Test rig booked twice", severity: "med", owner_ref: ref("farid-haddad"), raised_on: day(-5), due_date: day(7) }, "issue");
    await tryPost(`/api/plans/${p2}/issues`, { title: "Permit for night works delayed", severity: "high", owner_ref: ref("chloe-martin"), raised_on: day(-12), due_date: day(14) }, "issue");
  }
  const crs = list(await get(`/api/plans/${p1}/change-requests`), "change_requests");
  if (crs.length === 0) {
    const cr = await tryPost(`/api/plans/${p1}/change-requests`, { title: "Move go-live by two weeks", description: "Gas main diversion needs two extra weeks.", reason: "Issue: cable route blocked", kind: "schedule", impact_schedule_days: 10, impact_cost_minor: 45_000_00, cost_currency: "GBP", impact_summary: "Two weeks later; 45k more labour." }, "change request");
    if (cr?.pid) await tryPost(`/api/plans/${p1}/change-requests/${cr.pid}/submit`, { approver_ref: ref("amara-okafor") }, "submit change request");
  }

  // Objectives, key results, check-ins, and the plans they align to.
  const objs = list(await get("/api/objectives"), "objectives");
  const defs = [
    ["Reliable journeys", "2026/27", [["Cut delay minutes", "percent", "decrease", 100, 70, 85], ["Trains on time", "percent", "increase", 88, 95, 91]], ["Signalling Renewal", "Interlocking Replacement, North Line", "Trackside Equipment Upgrade"]],
    ["Everyone can travel", "2026/27", [["Step-free stations", "number", "increase", 6, 14, 9], ["Accessibility complaints", "number", "decrease", 80, 30, 52]], ["Station Accessibility", "Step-free Access, Central Station"]],
    ["A modern depot", "2026/27", [["Maintenance jobs digital", "percent", "increase", 10, 90, 45]], ["Depot Digitisation", "Fleet Maintenance System"]],
  ];
  for (const [title, period, krs, linked] of defs) {
    let o = objs.find((x) => x.title === title);
    if (!o) {
      o = await post("/api/objectives", { title, description: "Fictitious objective for the demonstration.", period });
      for (const [kt, metric, direction, start, target, current] of krs) {
        const kr = await tryPost(`/api/objectives/${o.pid}/key-results`, { title: kt, metric, direction, start_value: start, target_value: target, owner_ref: ref("amara-okafor") }, `key result ${kt}`);
        if (kr?.pid) await tryPost(`/api/key-results/${kr.pid}/check-ins`, { value: current, confidence: 7, note: "On track" }, "check-in");
      }
      for (const name of linked) if (plans[name]) await tryPost(`/api/plans/${plans[name]}/objectives`, { objective_pid: o.pid, weight: 5 }, "align");
    }
  }

  // KPIs with a few readings.
  const kpis = list(await get("/api/kpis"), "kpis");
  for (const [name, unit, direction, target, red, series] of [
    ["Delay minutes per 1000 train-km", "min", "lower_better", 40, 80, [70, 64, 58, 55, 52]],
    ["Passenger satisfaction", "%", "higher_better", 90, 70, [78, 80, 83, 84, 86]],
    ["Budget spent to plan", "%", "lower_better", 100, 120, [92, 96, 101, 104, 103]],
  ]) {
    if (kpis.some((k) => (k.kpi?.name ?? k.name) === name)) continue;
    const k = await tryPost("/api/kpis", { name, unit, direction, target_value: target, red_value: red, cadence: "monthly", owner_ref: ref("amara-okafor"), plan_pid: plans["Northgate Rail Modernisation"] }, `kpi ${name}`);
    if (k?.pid) for (const [i, v] of series.entries()) await tryPost(`/api/kpis/${k.pid}/readings`, { value: v, observed_on: day(-120 + i * 30) }, "reading");
  }

  // Announcements.
  const anns = list(await get("/api/announcements"), "announcements");
  if (anns.length === 0) {
    await tryPost("/api/announcements", { title: "Welcome to the Northgate demonstration", body: "Every name, number and date here is fictitious.", severity: "info", pinned: true }, "announcement");
    await tryPost("/api/announcements", { title: "Possession weekend: 24 to 26 October", body: "North line closed for interlocking works.", severity: "important" }, "announcement");
  }
}
