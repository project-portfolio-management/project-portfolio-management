# Container build for the Project Portfolio Management UI (SvelteKit, adapter-node).
#
# BUILD CONTEXT IS THE REPOSITORY ROOT, as for the service image (compose.yaml builds both):
#
#   podman build -f project-portfolio-management-ui-with-svelte/Dockerfile -t project-portfolio-management-ui .
#
# The server side of the UI is a back-end for the browser (sign-in, session cookie, a proxy to the API),
# so it needs to know where the API is: PROJECT_PORTFOLIO_MANAGEMENT_API_URL and AUTH_API_URL.
# Behind a TLS-terminating proxy, have it send X-Forwarded-Proto: https (container-server.mjs explains why);
# with no proxy the server treats requests as plain http.
# The Node version equals `engines.node` in package.json.

FROM docker.io/library/node:26-slim@sha256:e4db4c914bf8ad241bd24804fc25d272d89bad97e68cdc8a1414862790a02c39 AS builder
RUN npm install --global pnpm@12.10.1
WORKDIR /ui
COPY project-portfolio-management-ui-with-svelte/package.json project-portfolio-management-ui-with-svelte/pnpm-lock.yaml project-portfolio-management-ui-with-svelte/pnpm-workspace.yaml project-portfolio-management-ui-with-svelte/.npmrc ./
RUN pnpm install --frozen-lockfile
COPY project-portfolio-management-ui-with-svelte/ ./
RUN pnpm run build \
 && pnpm install --frozen-lockfile --prod

FROM docker.io/library/node:26-slim@sha256:e4db4c914bf8ad241bd24804fc25d272d89bad97e68cdc8a1414862790a02c39
# BODY_SIZE_LIMIT: adapter-node refuses a request body over 512 KiB by default, which would stop an attachment (LT-5,
# NC-1 slice 5) long before the service's 10 MiB limit. 10304K is the service's own transport ceiling for the upload
# route (the limit plus 64 KiB). It applies to every route of this server; raise both together if
# PROJECT_PORTFOLIO_MANAGEMENT_ATTACHMENT_MAX_BYTES is raised.
ENV NODE_ENV=production PORT=3000 HOST=0.0.0.0 PROTOCOL_HEADER=x-forwarded-proto BODY_SIZE_LIMIT=10304K
WORKDIR /app
COPY --from=builder /ui/build ./build
COPY --from=builder /ui/node_modules ./node_modules
COPY --from=builder /ui/package.json ./package.json
COPY project-portfolio-management-ui-with-svelte/container-server.mjs ./container-server.mjs
USER node
EXPOSE 3000
HEALTHCHECK --interval=5s --timeout=3s --retries=20 --start-period=5s \
  CMD ["node", "-e", "fetch('http://127.0.0.1:3000/').then(r=>process.exit(r.status<500?0:1),()=>process.exit(1))"]
CMD ["node", "container-server.mjs"]
