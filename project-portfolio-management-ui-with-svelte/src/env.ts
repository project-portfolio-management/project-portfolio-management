import { defineEnvVars } from "@sveltejs/kit/env";

// @migration-task Review usage of dynamic environment variables. They fall back to the empty string if not present, which may not be what you want.
export const variables = defineEnvVars({
  PUBLIC_OIDC_SIGNIN_ENABLED: { public: true, schema: (input) => input ?? "" },
  PROJECT_PORTFOLIO_MANAGEMENT_API_URL: { schema: (input) => input ?? "" },
  AUTH_API_URL: { schema: (input) => input ?? "" },
  // Direct OpenID Connect sign-in (HD-12). Off unless the issuer and client id are set.
  PUBLIC_OIDC_DIRECT_ENABLED: { public: true, schema: (input) => input ?? "" },
  // The name on the button, e.g. "Microsoft Entra ID".
  PUBLIC_OIDC_DIRECT_PROVIDER: { public: true, schema: (input) => input ?? "" },
  OIDC_ISSUER: { schema: (input) => input ?? "" },
  OIDC_CLIENT_ID: { schema: (input) => input ?? "" },
  OIDC_CLIENT_SECRET: { schema: (input) => input ?? "" },
  OIDC_SCOPES: { schema: (input) => input ?? "" },
  OIDC_TENANT_ID: { schema: (input) => input ?? "" },
  OIDC_IDP_HINT: { schema: (input) => input ?? "" },
  OIDC_REDIRECT_URI: { schema: (input) => input ?? "" },
  OIDC_SESSION_HOURS: { schema: (input) => input ?? "" },
  OIDC_IDLE_MINUTES: { schema: (input) => input ?? "" },
  OIDC_SESSION_STORE: { schema: (input) => input ?? "" },
  REDIS_URL: { schema: (input) => input ?? "" },
  OIDC_END_SESSION: { schema: (input) => input ?? "" },
  // Cross-app bridges (see src/lib/bridge.ts): where the sibling apps live. Blank means no link.
  CONTACTS_APP_URL: { schema: (input) => input ?? "" },
  WORKFORCE_APP_URL: { schema: (input) => input ?? "" },
  // Demonstration mode (see src/lib/server/demo.ts): sign in as example@example.com, fictitious data only.
  PPM_DEMO_MODE: { schema: (input) => input ?? "" },
});
