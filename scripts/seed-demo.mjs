#!/usr/bin/env node
// Fill a running API with the FICTITIOUS Northgate demonstration portfolio (see operations/demo.md).
//
//   PPM_API_URL=http://127.0.0.1:5150 node scripts/seed-demo.mjs
//
// Safe to run twice: each group looks for what is already there. NEVER run it against a deployment that
// holds real data. The API should run with login off (the demonstration stack does), because the script
// sends no token.

import { API, get, log } from "./demo/lib.mjs";
import { seedCore } from "./demo/core.mjs";
import { seedStrategy } from "./demo/strategy.mjs";
import { seedResources } from "./demo/resources.mjs";
import { seedEscalation, seedStartControl } from "./demo/escalation.mjs";
import { seedChain } from "./demo/chain.mjs";

log(`Seeding the demonstration portfolio into ${API}`);
for (let i = 0; ; i++) {
  try {
    await get("/_health");
    break;
  } catch (e) {
    if (i >= 60) throw new Error(`the API at ${API} did not answer: ${e.message}`);
    await new Promise((r) => setTimeout(r, 2000));
  }
}

const plans = await seedCore();
await seedStartControl(plans);
await seedStrategy(plans);
const pools = await seedResources(plans);
await seedEscalation(plans, pools);
await seedChain(plans, pools);
log("Done. Everything seeded is fictitious.");
