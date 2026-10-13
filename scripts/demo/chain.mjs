// Scheduling depth: critical-chain estimates and buffers, the drum, schedule baselines, earned value,
// a scenario. Needs the plans and skill pools from the earlier groups.

import { get, post, put, tryPost, log, day } from "./lib.mjs";
import { ref } from "./core.mjs";

const list = (r, key) => (Array.isArray(r) ? r : r?.[key] ?? []);

async function putTry(path, body, what) {
  try {
    return await put(path, body);
  } catch (e) {
    log(`  (skipped ${what}: ${e.status} ${e.message.split("-> ")[1]?.slice(0, 200)})`);
    return null;
  }
}

export async function seedChain(plans, pools) {
  log("Critical chain, drum, baselines, earned value, scenario");
  const north = plans["Interlocking Replacement, North Line"];
  const tasks = list(await get(`/api/plans/${north}/tasks`), "tasks");

  // Aggressive (about 50 percent) and safe (about 90 percent) estimates for each open task.
  for (const t of tasks.filter((x) => x.status !== "done")) {
    const dur = t.duration_days ?? 20;
    await putTry(`/api/plans/${north}/tasks/${t.pid}/ccpm-estimate`, { aggressive_days: Math.max(2, Math.round(dur * 0.6)), safe_days: Math.round(dur * 1.1) }, "ccpm estimate");
  }

  // Freeze the buffer plan, take a snapshot, record a recovery action.
  const plan = await get(`/api/plans/${north}/buffer-plan`).catch(() => null);
  if (!plan || plan.plan == null) {
    await tryPost(`/api/plans/${north}/buffer-plan`, {}, "freeze buffer plan");
    await tryPost(`/api/plans/${north}/buffer-management/snapshots`, {}, "buffer snapshot");
    await tryPost(`/api/plans/${north}/buffer-recovery-actions`, { buffer_key: "project", action: "Add a second tester for the bench-test phase", owner_ref: ref("farid-haddad"), due_date: day(10) }, "recovery action");
  }

  // The drum: the pool the portfolio depends on most.
  if (pools?.["Signalling engineers"]) await putTry("/api/portfolio-drum", { skill_pool_pid: pools["Signalling engineers"] }, "drum");

  // A schedule baseline and an earned-value snapshot on the north line.
  const baselines = list(await get(`/api/plans/${north}/schedule-baselines`), "baselines");
  if (baselines.length === 0) await tryPost(`/api/plans/${north}/schedule-baselines`, { reason: "Baseline agreed at the design gate" }, "baseline");
  await tryPost(`/api/plans/${north}/earned-value/snapshot`, {}, "earned value snapshot");

  // A scenario: which projects fit a budget cap.
  const scenarios = list(await get("/api/scenarios"), "scenarios");
  if (scenarios.length === 0) {
    await tryPost("/api/scenarios", {
      name: "Fund the signalling work first",
      description: "What fits under a 1.2m cap if signalling comes before accessibility.",
      plan_pids: Object.entries(plans).filter(([n]) => !/Portfolio|Modernisation$|Renewal$|Accessibility$|Digitisation$/.test(n) || /Interlocking|Trackside/.test(n)).map(([, p]) => p),
      budget_cap_minor: 1_200_000_00, currency: "GBP", must_include: [north],
    }, "scenario");
  }
}
