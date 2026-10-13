// The escalation and money features: funding and the ledger, delivery model, sign-offs, key people,
// thresholds and triage, start control, a cutover decision, deployment moves, scorecards and
// de-escalation, director packs.

import { get, post, put, tryPost, log, day } from "./lib.mjs";
import { ref } from "./core.mjs";

const list = (r, key) => (Array.isArray(r) ? r : r?.[key] ?? []);

async function putTry(path, body, what) {
  try {
    return await put(path, body);
  } catch (e) {
    // A phase already reached (a second run) is refused as a backward move: that is "already done".
    if (!/backward move/.test(e.message)) log(`  (skipped ${what}: ${e.status} ${e.message.split("-> ")[1]?.slice(0, 200)})`);
    return null;
  }
}

export async function seedEscalation(plans, pools) {
  log("Escalation: funding, ledger, sign-offs, key people, thresholds, start control, cutover, moves, scorecards, packs");
  const north = plans["Interlocking Replacement, North Line"];
  const signalling = plans["Signalling Renewal"];
  const orgs = (await get("/api/organizations")).organizations;
  const org = (name) => orgs.find((o) => o.name === name)?.pid;

  // Funding sources, allocations, lines attached, cost codes.
  const sources = list(await get("/api/funding-sources"), "funding_sources");
  const src = {};
  for (const name of ["Treasury modernisation grant", "Authority reserves"]) {
    src[name] = (sources.find((s) => s.name === name) ?? (await post("/api/funding-sources", { name }))).pid;
  }
  const allocs = list(await get("/api/funding-allocations"), "allocations");
  if (allocs.length === 0) {
    const a1 = await post("/api/funding-allocations", { funding_source_pid: src["Treasury modernisation grant"], amount_minor: 2_400_000_00, currency: "GBP", allocated_on: "2026-04-01", reference: "FY27 grant letter" });
    await post("/api/funding-allocations", { funding_source_pid: src["Authority reserves"], amount_minor: 600_000_00, currency: "GBP", allocated_on: "2026-05-15", reference: "Reserve draw-down" });
    for (const name of ["Interlocking Replacement, North Line", "Trackside Equipment Upgrade"]) {
      const board = await get(`/api/plans/${plans[name]}/budget-lines`);
      const first = list(board, "lines")[0] ?? list(board, "budget_lines")[0];
      if (first) {
        await putTry(`/api/plans/${plans[name]}/budget-lines/${first.pid}/funding-source`, { funding_source_pid: src["Treasury modernisation grant"] }, "line funding source");
        await putTry(`/api/plans/${plans[name]}/budget-lines/${first.pid}/funding-allocation`, { funding_allocation_pid: a1.pid }, "attach line");
        await putTry("/api/cost-codes", { cost_code: `NG-${name.slice(0, 3).toUpperCase()}-EQ`, budget_line_pid: first.pid }, "cost code");
      }
    }
  }

  // Delivery model (EX-2).
  await putTry(`/api/plans/${signalling}/delivery-model`, { delivery_model: "central_platform_local_deployment", delivery_owner_organization_pid: org("Brightline Engineering") }, "delivery model");

  // Sign-off needed on the go-live milestone (EX-4).
  const ms = list(await get(`/api/plans/${north}/milestones`), "milestones");
  const goLive = ms.find((m) => m.name === "Go-live");
  if (goLive) {
    await putTry(`/api/plans/${north}/milestones/${goLive.pid}/signoff-required`, { organization_pids: [org("Harbour Rail Partners"), org("Kestrel Signalling Ltd")] }, "signoff required");
    await tryPost(`/api/plans/${north}/milestones/${goLive.pid}/signoffs`, { organization_pid: org("Kestrel Signalling Ltd"), action: "sign" }, "sign-off");
  }

  // Key people (EX-11).
  const kp = await get("/api/key-persons").catch(() => ({ entries: [] }));
  if ((kp.entries ?? []).length === 0) {
    await putTry("/api/key-persons", { subject_kind: "person", subject: ref("ben-whitfield"), readiness: "none", note: "Only person who knows the interlocking logic" }, "key person");
    await putTry("/api/key-persons", { subject_kind: "role", subject: "Release manager", successor_ref: ref("chloe-martin"), readiness: "ready" }, "key role");
    await putTry("/api/key-persons", { subject_kind: "person", subject: ref("dev-patel"), successor_ref: ref("farid-haddad"), readiness: "partly", skill_pool_pid: pools["Software developers"] }, "key person");
    await putTry("/api/key-persons/contingent", { person_ref: ref("farid-haddad"), contingent: true }, "contingent");
  }

  // Thresholds and triage (EX-7).
  await putTry(`/api/plans/${signalling}/thresholds`, { kind: "confidence_red_packs", count: 2 }, "threshold");
  await putTry(`/api/plans/${signalling}/thresholds`, { kind: "forecast_past", date: "2027-06-30" }, "threshold");
  await putTry(`/api/plans/${signalling}/thresholds`, { kind: "p80_cap", cap_minor: 900_000_00, currency: "GBP" }, "threshold");
  await putTry(`/api/plans/${plans["Depot Digitisation"]}/triage`, { outcome: "fix", reason: "Vendor behind plan; fix the integration before more spend." }, "triage");

  // A cutover decision (EX-6), left open with one criterion met and one co-signature.
  const cut = list(await get(`/api/plans/${north}/cutover-decisions`), "decisions");
  if (cut.length === 0) {
    const d = await post(`/api/plans/${north}/cutover-decisions`, { title: "North line interlocking cutover", criteria: ["Rollback rehearsed twice with the operations team", "Safety case signed off by the independent assessor", "All drivers briefed on the new aspects"] });
    await tryPost(`/api/plans/${north}/cutover-decisions/${d.pid}/publish`, {}, "publish");
    await tryPost(`/api/plans/${north}/cutover-decisions/${d.pid}/open`, {}, "open");
    await putTry(`/api/plans/${north}/cutover-decisions/${d.pid}/criteria/1/mark`, { met: true, evidence: "Rehearsal log, 4 and 6 October" }, "mark");
    await tryPost(`/api/plans/${north}/cutover-decisions/${d.pid}/cosign`, { role: "accountable_professional_lead" }, "cosign");
  }

  // Deployment moves (EX-5).
  const tasks = list(await get(`/api/plans/${north}/tasks`), "tasks");
  const install = tasks.find((t) => t.title === "Install on site");
  const moves = list(await get(`/api/plans/${north}/deployment-moves`), "moves");
  if (install && moves.length === 0) {
    await tryPost(`/api/plans/${north}/deployment-moves`, { task_pid: install.pid, new_start: day(40), reason: "Gas main diversion needs two more weeks", approver_ref: ref("amara-okafor") }, "move");
  }

  // Scorecard and de-escalation (EX-10).
  const cards = list(await get("/api/scorecards"), "scorecards");
  if (cards.length === 0) {
    const kpis = list(await get("/api/kpis"), "kpis");
    const c = await tryPost("/api/scorecards", { name: "Northgate escalation scorecard", for_deescalation: true }, "scorecard");
    if (c?.pid && kpis.length) await putTry(`/api/scorecards/${c.pid}/kpis`, { kpi_pids: kpis.map((k) => k.kpi?.pid ?? k.pid) }, "scorecard kpis");
  }
  await putTry("/api/deescalation/level", { level: "amber", note: "Raised after the March slippage." }, "level");
  const crit = await tryPost("/api/deescalation/criteria", { criterion: "Schedule confidence at amber or better" }, "criterion");
  if (crit?.pid) {
    await putTry(`/api/deescalation/criteria/${crit.pid}/evidence`, { quarter: "2026-Q2", met: true, note: "Confidence reported amber." }, "evidence");
    await putTry(`/api/deescalation/criteria/${crit.pid}/evidence`, { quarter: "2026-Q3", met: true, note: "Confidence reported amber." }, "evidence");
  }

  // Director packs.
  const packs = list(await get("/api/director-packs"), "packs");
  if (packs.length === 0) {
    for (const audience of ["executive", "board"]) {
      const p = await tryPost("/api/director-packs", { period: "2026-09", audience }, `pack ${audience}`);
      if (p?.pid && audience === "executive") await tryPost(`/api/director-packs/${p.pid}/issue`, {}, "issue pack");
    }
  }
}

/**
 * Start control (EX-8). Run BEFORE the resources are seeded: with nobody allocated yet no pool is short, so
 * the first programmes start; a later one is then refused by the work-in-progress limit (the demonstration
 * stack sets PROJECT_PORTFOLIO_MANAGEMENT_WIP_LIMIT=2) and waits in the queue.
 */
export async function seedStartControl(plans) {
  log("Start control: programmes into delivery");
  for (const name of ["Signalling Renewal", "Station Accessibility", "Depot Digitisation"]) {
    for (const phase of ["initiating", "planning", "executing"]) {
      const r = await putTry(`/api/plans/${plans[name]}/phase`, { phase }, `${name} -> ${phase}`);
      if (!r) break;
    }
  }

}
