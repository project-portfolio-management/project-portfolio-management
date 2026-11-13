.PHONY: demo test

demo:
	pnpm install
	pnpm dev

# The UI gates from the root AGENTS.md. PW_PORT avoids a preview server another project may hold on 4173.
test:
	pnpm run lint
	pnpm run check
	npx vitest run
	PW_PORT=$${PW_PORT:-4183} CI=true pnpm exec playwright test
