# The demonstration

A try-out of every feature with **fictitious data**, signed in as `example@example.com`. Nothing in it is real:
the Northgate Transport Authority, its eight people, four organizations, ten plans, money and dates were made up
for this. It is for showing the product, not for running one.

> **Never** point the demonstration at real data, switch demonstration mode on in front of a real deployment, or
> put it on a shared network you do not control. The API in it trusts a **published test key** (anyone can
> mint a token for it), and the sign-in page accepts one fixed address.

## Start it

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo   # or docker compose
```

That builds the API and UI images, starts the database, applies the migrations, starts the demonstration API,
fills it with the fictitious portfolio (the one-shot `seed-demo` job), and starts the UI. When it returns, open
<http://localhost:3000/en-001/signin>, leave the address as `example@example.com`, and press **Sign in to the
demonstration**. You land on the **demonstration tour**: every feature, grouped, each with what to look for and a
link. A yellow banner on every page says the data is fictitious.

Stop it and delete the data with `podman compose --profile demo down -v`.

### What starts

| Service | What it is |
| --- | --- |
| `api-demo` | The API with login **on**, trusting one published test key; a work-in-progress limit of two programmes; a deployment approver. |
| `seed-demo` | One-shot: runs `scripts/seed-demo.mjs` against `api-demo`, then exits. |
| `ui-demo` | The UI with `PPM_DEMO_MODE=true`, on port 3000. |
| `postgres`, `redis`, `migrate` | As in the normal stack. |

Use the service names above (`ui-demo` pulls in the rest). Starting the whole profile also starts the normal `api`,
which refuses to start without token keys.

## What the sign-in does

With `PPM_DEMO_MODE=true` the sign-in page accepts only `example@example.com` (any other address is refused), sends no
email and asks no identity provider. It starts a session in the UI server's own memory and signs a PASETO token for
"Example Person" (a made-up person id, administrator and write access, and the co-signing role `accountable_professional_lead`) with a **published test key**. The UI's proxy sends that token
to the demonstration API, which trusts exactly that public key. So the signed-in person has a real identity:
**My work** (`/me`), the **kanban board** (`/kanban`), saved views and the administrator screens all work, and
the seed gives Example Person some open tasks.

The key's private half is in the repository on purpose (`src/lib/server/demo-token.ts`, `scripts/demo/token.mjs`; a
unit test keeps the two the same and checks the public half in `compose.yaml`). That is only safe because the
demonstration holds fictitious data on its own network: **never trust that key anywhere else**. Turn the mode off by
not setting the variable. The sign-in code is `src/routes/signin/+page.server.ts`.

## The data

`scripts/seed-demo.mjs` fills a running API through its own endpoints, so every feature works on it, and you can
change things. It is safe to run twice (each group looks for what is there). It signs in with the same published test key,
so it works against the demonstration API (use `PPM_API_TOKEN` to give it another token). It seeds, in order:

- **Core:** four organizations; a portfolio, three programmes and six projects; eight tasks on each project with
  dependencies; three milestones, three risks, budget lines with spend, and two benefits on each project.
- **Start control:** two programmes are moved into delivery; the third is refused by the limit and waits in the queue.
- **Strategy:** gate reviews, five proposals at different stages, four ideas with votes, issues, a change request,
  three objectives with key results and check-ins, three measures with readings, two announcements.
- **Resources:** roles, four skill pools, availability, allocations (some over-booked), capacity commitments for the
  heatmap, cost rates.
- **Escalation:** funding sources and allocations with lines traced, a delivery model, a milestone needing sign-off, key
  people, thresholds and a triage, an open cutover decision, a deployment move waiting, a scorecard with
  de-escalation evidence, director packs.
- **Critical chain:** aggressive and safe estimates, a frozen buffer plan with a snapshot and a recovery action, the
  drum, a schedule baseline, an earned-value snapshot, a scenario.

To seed another API that trusts the demonstration key: `PPM_API_URL=http://127.0.0.1:5150 node scripts/seed-demo.mjs`.

## What is and is not covered

- **Covered and checked:** the tour lists 41 features. Each link was opened on a stack started as above and loaded
  without an error page (see the changelog for the date and the result).
- **Not in the demonstration:** email (nothing is sent, so the deliveries list is empty), a real identity provider
  (the token is signed by the published test key), and the thresholds, triage and start-control override screens (the data is seeded; those are
  API-only still).
- **Fictitious by design:** people are references like `person:<uuid>` with made-up ids; the UI shows the references
  where it has no name directory.
- **The tour's Welsh** is an unreviewed draft, like the rest of the Welsh.
