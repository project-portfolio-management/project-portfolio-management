// People, skill pools, availability, allocations, capacity commitments and cost rates.

import { get, post, put, tryPost, log, day } from "./lib.mjs";
import { PEOPLE, ref } from "./core.mjs";

const list = (r, key) => (Array.isArray(r) ? r : r?.[key] ?? []);
const ROLE = {
  "example-person": "Programme coordinator",
  "amara-okafor": "Programme director", "ben-whitfield": "Signalling engineer", "chloe-martin": "Project manager",
  "dev-patel": "Software developer", "elena-rossi": "Accessibility lead", "farid-haddad": "Test engineer",
  "grace-liu": "Finance partner", "hugo-bennett": "Operations manager",
};

export async function seedResources(plans) {
  log("Resources: roles, skill pools, availability, allocations, capacity, cost rates");
  for (const [slug] of PEOPLE) { try { await put("/api/resource-roles", { person_ref: ref(slug), role: ROLE[slug] }); } catch (e) { log(`  (role ${slug}: ${e.message.slice(0, 160)})`); } }

  const pools = list(await get("/api/skill-pools"), "skill_pools");
  const poolDefs = [
    ["Signalling engineers", ["Signalling engineer"], ["interlocking", "trackside"]],
    ["Software developers", ["Software developer"], ["typescript", "rust"]],
    ["Test engineers", ["Test engineer"], ["bench testing"]],
    ["Delivery managers", ["Project manager", "Programme director"], ["planning"]],
  ];
  const poolPid = {};
  for (const [name, roles, skills] of poolDefs) {
    const found = pools.find((p) => p.name === name);
    poolPid[name] = (found ?? (await post("/api/skill-pools", { name, roles, skills }))).pid;
  }

  const avail = list(await get(`/api/availability?person_ref=${encodeURIComponent(ref("ben-whitfield"))}`), "availability");
  if (avail.length === 0) {
    await tryPost("/api/availability", { person_ref: ref("ben-whitfield"), from: day(20), to: day(34), max_percent: 0, note: "Annual leave" }, "availability");
    await tryPost("/api/availability", { person_ref: ref("dev-patel"), from: day(-30), to: day(200), max_percent: 80, note: "Four days a week" }, "availability");
    await tryPost("/api/availability", { person_ref: ref("farid-haddad"), from: day(40), to: day(60), max_percent: 50, note: "Training course" }, "availability");
  }

  const [i1, i2] = ["Interlocking Replacement, North Line", "Trackside Equipment Upgrade"];
  const allocs = [
    [i1, "ben-whitfield", "Signalling engineer", 70, -190, 190, "committed"],
    [i2, "ben-whitfield", "Signalling engineer", 50, -120, 210, "committed"],
    [i1, "farid-haddad", "Test engineer", 60, -60, 200, "committed"],
    [i2, "farid-haddad", "Test engineer", 60, -60, 200, "committed"],
    ["Fleet Maintenance System", "dev-patel", "Software developer", 80, -100, 200, "committed"],
    ["Depot Wi-Fi and Sensors", "dev-patel", "Software developer", 40, -60, 140, "committed"],
    ["Step-free Access, Central Station", "elena-rossi", "Accessibility lead", 80, -150, 210, "committed"],
    ["Wayfinding and Announcements", "chloe-martin", "Project manager", 50, -90, 150, "committed"],
    [i1, "chloe-martin", "Project manager", 40, -190, 190, "committed"],
    ["Depot Wi-Fi and Sensors", "hugo-bennett", "Operations manager", 60, -60, 140, "committed"],
    [i1, null, "Signalling engineer", 100, 30, 120, "proposed"],
  ];
  for (const [plan, slug, role, percent, from, to, booking] of allocs) {
    const pid = plans[plan];
    const have = list(await get(`/api/plans/${pid}/allocations`), "allocations");
    const wanted = slug ? ref(slug) : "";
    if (have.some((a) => (a.person_ref ?? "") === wanted && a.role === role)) continue;
    const body = slug
      ? { person_ref: ref(slug), role, percent, start_date: day(from), end_date: day(to), booking }
      : { placeholder: true, role, percent, start_date: day(from), end_date: day(to), booking: "proposed", skills_required: ["interlocking"] };
    await tryPost(`/api/plans/${pid}/allocations`, body, `allocation ${plan} ${slug ?? "placeholder"}`);
  }

  // Capacity: supply and demand by pool and month, for the heatmap. A commitment is to a programme.
  const sig = plans["Signalling Renewal"];
  const acc = plans["Station Accessibility"];
  const dep = plans["Depot Digitisation"];
  const months = [0, 1, 2, 3, 4, 5].map((n) => day(n * 30).slice(0, 7));
  for (const [i, m] of months.entries()) {
    const rows = [
      [poolPid["Signalling engineers"], sig, "supply", 2.0],
      [poolPid["Signalling engineers"], sig, "demand", 1.5],
      [poolPid["Signalling engineers"], acc, "demand", 0.5 + (i % 3) * 0.5],
      [poolPid["Software developers"], dep, "supply", 2.0],
      [poolPid["Software developers"], dep, "demand", 1.5],
      [poolPid["Test engineers"], sig, "supply", 1.0],
      [poolPid["Test engineers"], sig, "demand", 0.8],
    ];
    for (const [pool_pid, plan_pid, kind, committed_fte] of rows) {
      try {
        await put("/api/capacity-commitments", { pool_pid, plan_pid, kind, month: m, committed_fte });
      } catch (e) {
        log(`  (skipped commitment ${kind} ${m}: ${e.message.split("-> ")[1]?.slice(0, 160)})`);
      }
    }
  }

  // Cost rates.
  const tables = list(await get("/api/cost-rate-tables"), "tables").concat(list(await get("/api/cost-rate-tables").catch(() => []), "cost_rate_tables"));
  for (const [role, perHour] of [["Signalling engineer", 6500], ["Software developer", 5800], ["Test engineer", 5200], ["Project manager", 6000]]) {
    if (tables.some((t) => t.role === role)) continue;
    const t = await tryPost("/api/cost-rate-tables", { name: `${role} rate`, role, currency: "GBP" }, "rate table");
    if (t?.pid) await tryPost(`/api/cost-rate-tables/${t.pid}/rates`, { effective_from: "2026-01-01", per_hour_minor: perHour }, "rate");
  }
  return poolPid;
}
