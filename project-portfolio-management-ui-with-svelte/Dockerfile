# Container build for the Project Portfolio Management UI (SvelteKit, adapter-node).
#
# BUILD CONTEXT IS THE REPOSITORY ROOT, as for the service image (compose.yaml builds both):
#
#   podman build -f project-portfolio-management-ui-with-svelte/Dockerfile -t project-portfolio-management-ui .
#
# The server side of the UI is a back-end for the browser (sign-in, session cookie, a proxy to the API),
# so it needs to know where the API is: PROJECT_PORTFOLIO_MANAGEMENT_API_URL and AUTH_API_URL.
# It also needs ORIGIN, the public address people type (adapter-node refuses a form post otherwise).
# The Node version equals `engines.node` in package.json.

FROM docker.io/library/node:26-slim AS builder
RUN npm install --global pnpm@12.10.1
WORKDIR /ui
COPY project-portfolio-management-ui-with-svelte/package.json project-portfolio-management-ui-with-svelte/pnpm-lock.yaml project-portfolio-management-ui-with-svelte/pnpm-workspace.yaml project-portfolio-management-ui-with-svelte/.npmrc ./
RUN pnpm install --frozen-lockfile
COPY project-portfolio-management-ui-with-svelte/ ./
RUN pnpm run build \
 && pnpm install --frozen-lockfile --prod

FROM docker.io/library/node:26-slim
ENV NODE_ENV=production PORT=3000 HOST=0.0.0.0
WORKDIR /app
COPY --from=builder /ui/build ./build
COPY --from=builder /ui/node_modules ./node_modules
COPY --from=builder /ui/package.json ./package.json
USER node
EXPOSE 3000
HEALTHCHECK --interval=5s --timeout=3s --retries=20 --start-period=5s \
  CMD ["node", "-e", "fetch('http://127.0.0.1:3000/').then(r=>process.exit(r.status<500?0:1),()=>process.exit(1))"]
CMD ["node", "build"]
