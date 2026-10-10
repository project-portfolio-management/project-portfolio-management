import { defineConfig, devices } from "@playwright/test";

// Smoke tests run against the Vite dev server. The backend is stubbed
// per-test via `page.route`, so no running Rust service is required.
export default defineConfig({
  testDir: "tests/e2e",
  timeout: 30_000,
  fullyParallel: true,
  use: {
    baseURL: "http://localhost:4173",
    trace: "on-first-retry",
  },
  projects: [
    { name: "chromium", use: { ...devices["Desktop Chrome"] }, testIgnore: /mobile\.spec\.ts/ },
    // T-28k (repo tasks.md EV-1): a phone-width audit across every
    // route — 390x844 per the task's own spec (roughly an iPhone 12
    // mini). A separate project rather than a per-test viewport
    // override, so `pnpm test:e2e --project=mobile` runs it in
    // isolation, matching the acceptance text's "runs the mobile
    // project in CI".
    {
      name: "mobile",
      testMatch: /mobile\.spec\.ts/,
      use: { ...devices["Desktop Chrome"], viewport: { width: 390, height: 844 } },
    },
  ],
  // Test the production build via `vite preview`: it serves static,
  // correctly-typed ES modules, avoiding the `vite dev` cold-start
  // dependency-optimization race that flakes module loading.
  webServer: [
    {
      command: "npm run build && npm run preview -- --port 4173 --strictPort",
      url: "http://localhost:4173",
      reuseExistingServer: !process.env.CI,
      timeout: 120_000,
      // Direct OpenID Connect sign-in (HD-12) is switched on for the browser suite and pointed
      // at the mock Entra-compatible provider and mock API below.
      env: {
        PUBLIC_OIDC_DIRECT_ENABLED: "true",
        PUBLIC_OIDC_DIRECT_PROVIDER: "Microsoft Entra ID",
        OIDC_ISSUER: "http://localhost:4180/11111111-2222-3333-4444-555555555555/v2.0",
        OIDC_CLIENT_ID: "mock-client-id",
        OIDC_TENANT_ID: "11111111-2222-3333-4444-555555555555",
        OIDC_END_SESSION: "true",
        PROJECT_PORTFOLIO_MANAGEMENT_API_URL: "http://localhost:4181",
      },
    },
    {
      // A minimal Microsoft Entra ID-compatible OIDC provider (4180) and an API that only
      // accepts tokens it issued (4181).
      command: "node tests/e2e/support/entra-mock.mjs",
      url: "http://localhost:4180/__health",
      reuseExistingServer: !process.env.CI,
      timeout: 30_000,
    },
  ],
});
