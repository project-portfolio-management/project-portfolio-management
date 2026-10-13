// A second preview server (port 4174) with demonstration mode on, for tests/e2e/demo.spec.ts.
// It waits for the main preview server (4173), whose command also builds the app, then serves that build.
import { spawn } from "node:child_process";

for (let i = 0; i < 240; i++) {
  try {
    const r = await fetch("http://localhost:4173/");
    if (r.status < 500) break;
  } catch {
    /* not up yet */
  }
  await new Promise((r) => setTimeout(r, 1000));
}
const child = spawn("npx", ["vite", "preview", "--port", "4174", "--strictPort"], {
  stdio: "inherit",
  env: { ...process.env, PPM_DEMO_MODE: "true", PROJECT_PORTFOLIO_MANAGEMENT_API_URL: "http://localhost:4181" },
});
child.on("exit", (code) => process.exit(code ?? 0));
process.on("SIGTERM", () => child.kill("SIGTERM"));
