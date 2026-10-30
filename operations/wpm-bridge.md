# Tutorial: the bridge to the workforce service (WPM)

This walk-through connects this portfolio service (PPM) to the workforce planning service (WPM), links one skill
pool on both sides, and reads what crosses: WPM's supply on PPM's capacity heatmap, PPM's demand sent to WPM, and
WPM's people impact for a programme. The rules behind every step are the entity spec's
[§8.5a](../spec/08-architecture.md) and the service spec's
[§9.10a](../project-portfolio-management-service-with-rust/spec/index.md); this page only shows how to use them.

**What has been checked.** The steps below match the code and the tests: `tests/wpm_bridge.rs` runs PPM against a
stand-in WPM on a local port, and the UI's `tests/e2e/wpm-bridge.spec.ts` and `capacity-commitments.spec.ts` drive the
screens against a stubbed service. **The whole walk-through has not been run against a real WPM** (task FB-8 in
[`tasks.md`](../tasks.md) is that check, still open). Treat the sample answers as the shapes the code writes, not as a
recorded session.

## 1. What you need

- PPM's service running (see [`INSTALL.md`](../INSTALL.md)), and WPM's service running from its own repository.
- One bearer token that **both** services accept. PPM forwards the caller's own token to WPM on every call; there is
  no service account. So the two services must verify the same issuer and audience (PPM's compose defaults are
  issuer `authentication-service` and audience `main-x-service`, `compose.yaml`). Writes need `access=write` under
  PPM's default policy.
- A skill pool on each side that means the same skills, and a PPM plan of kind `Program` for the demand.

Both services listen on port 5150 by default. On one machine, start WPM on another port, for example:

```bash
# in WPM's service directory
cargo run -- start -p 5151
```

## 2. Point PPM at WPM

PPM reads one variable. Unset or empty, the bridge is in stub mode: nothing is called, and every reading says so.

| Variable | Meaning |
| --- | --- |
| `PROJECT_PORTFOLIO_MANAGEMENT_WORKER_SERVICE_URL` | WPM's base URL, no trailing slash. Must be `https://…`, or `http://` to `localhost`, `127.0.0.1` or `::1`. Any other URL is refused before a call is made, and the reading says `unreachable` with the reason. |
| `PROJECT_PORTFOLIO_MANAGEMENT_OUTBOUND_STRICT` | Optional (`on`, `true` or `1`). Also requires the URL to resolve to a public address. Leave it off when WPM is on a private network. |

```bash
# in project-portfolio-management-service-with-rust
export PROJECT_PORTFOLIO_MANAGEMENT_WORKER_SERVICE_URL=http://127.0.0.1:5151
cargo run -- start
```

`compose.yaml` does not pass this variable to the API container. Between containers `http://wpm:5150` would be
refused (plain `http` to a host that is not loopback), so in containers WPM needs an `https://` address.

Every call to WPM has a 2-second timeout and follows no redirects. A WPM that is down, slow or refusing never fails
a PPM request; the answer says what happened instead.

In the examples, `PPM=http://localhost:5150`, `WPM=http://127.0.0.1:5151` and `TOKEN` is the shared bearer token.

## 3. Link a pool on both sides

A link is two ids, one set on each side by a person. Nothing is ever matched by name. Each id is the other
service's pool `pid`, written as a lower-case hyphenated UUID.

Find the pools:

```bash
curl -s -H "Authorization: Bearer $TOKEN" "$PPM/api/skill-pools"   # {"skill_pools":[{"pid":…,"name":…,"peer_pool_ref":null,…}]}
curl -s -H "Authorization: Bearer $TOKEN" "$WPM/api/skill-pools"
```

Set PPM's side (`PPM_POOL` is PPM's pool pid, `WPM_POOL` is WPM's):

```bash
curl -s -X PUT -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d "{\"peer_pool_ref\":\"$WPM_POOL\"}" "$PPM/api/skill-pools/$PPM_POOL/peer-link"
```

The answer is the pool with its new `peer_pool_ref`. `404` means no such pool. `409` means another PPM pool is
already linked to that WPM pool (one PPM pool per WPM pool). `422` means the id is not a lower-case hyphenated UUID.
The link is audited (`skill_pool_peer_linked`) with no figure in it.

Then set WPM's side, the same route on WPM, with PPM's pool pid:

```bash
curl -s -X PUT -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d "{\"peer_pool_ref\":\"$PPM_POOL\"}" "$WPM/api/skill-pools/$WPM_POOL/peer-link"
```

On screen, PPM's `/skill-pools` page (under a locale, for example `/en-001/skill-pools`) lists every pool with its
link, checks the id before sending it, and sets or removes it.

**Unlinking** (`{"peer_pool_ref": null}`), or changing the link to another WPM pool, withdraws every demand figure
PPM sent to the old WPM pool: PPM sends each as zero, best effort, under your token. The answer lists them in
`peer_demand_withdrawals`. With no URL or no token they are recorded as not sent, and the figures stay in WPM.

## 4. Read supply on the heatmap

For a linked pool, supply is WPM's and demand is PPM's. The two services' supply figures are never added.

```bash
curl -s -H "Authorization: Bearer $TOKEN" "$PPM/api/capacity-heatmap?from=2026-11&months=6"
```

`from` is `YYYY-MM` (default: this month) and `months` is 1 to 24 (default 6). Each cell of a linked pool carries:

- `supply_source: "wpm"` and `supply_fte`, WPM's figure in FTE;
- `peer_supply`: `{source, status, supply_fte, as_of, age_seconds, reason}`, where `status` is
  - `live`: WPM answered just now;
  - `last_received`: WPM could not be reached, so this is the last figure **your own** token fetched, with its age;
  - `hidden`: WPM hides a month made of fewer workers than its floor;
  - `unknown`: nothing to show, with the `reason` (no URL, no token, WPM refused you, or nothing received yet);
- `entered_here_supply_fte`: what PPM holds for that pool, shown beside WPM's figure and never added to it.

A hidden or unknown month has the state `supply_unknown`. It is never zero, and it never falls back to PPM's figure.
Kept figures live in PPM's memory only, per caller, and are gone after a restart. The director pack, the drum, the
pipeline and the start check never ask WPM and show a linked pool's supply as unknown.

On screen: `/capacity-heatmap` marks a linked pool and shows the reading beside each month.

## 5. Send demand

A demand commitment on a linked pool sends the programme's total for that month to WPM.

```bash
curl -s -X PUT -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d "{\"pool_pid\":\"$PPM_POOL\",\"plan_pid\":\"$PROGRAMME\",\"kind\":\"demand\",\"month\":\"2026-11\",\"committed_fte\":2.5}" \
  "$PPM/api/capacity-commitments"
```

The plan must be of kind `Program` (`422` otherwise). PPM then sends `POST {WPM}/api/capacity/peer-demand` with
the programme named `thing:<plan pid>` and `fte_centi`, the total over every organization's commitments for that
pool and month, in hundredths of an FTE. The answer's `peer_demand_send` says what happened: `started`, `sent`,
`duplicate`, `refused`, `unreachable`, `not_configured`, `not_sent_no_token` or `not_sent_too_large`.

There is no retry and no outbox, because the token is never stored. To send again, set the commitment again. To
withdraw a figure, set it to zero: zero is sent, and WPM drops its claim. `GET /api/capacity-commitments` lists the
latest send for each demand commitment.

On screen: `/capacity-commitments` sets a commitment (open it from a programme's plan page to have the plan filled
in), says what happened to the send, and lists the recent sends.

## 6. Read people impact

```bash
curl -s -H "Authorization: Bearer $TOKEN" "$PPM/api/plans/$PROGRAMME/people-impact?months=12"
```

`months` is 1 to 36 (WPM's horizon; `422` outside it; WPM's default when left out). PPM asks
`GET {WPM}/api/capacity/people-impact?programme=thing:<plan pid>`. When the plan's `target_date` is a full date,
PPM also sends it as `ends_on`; otherwise `plan_ends_on_reason` says why it did not. The answer is WPM's, passed
through and labelled `source: "wpm"`: `workers_committed`, `contingent_ending_before_end` and
`over_committed_pool_months`, each known or unknown with WPM's reason, and a count under WPM's floor `null` with
`hidden: true`. `status` is `live`, or `unknown` with a `reason`. PPM adds no figure and keeps nothing. A plan you
cannot read is `404`, and WPM is not asked.

On screen: on `/plans/[pid]` for a plan of kind Program, the "People impact" box asks when you press its button.

## 7. When something looks wrong

| You see | Why |
| --- | --- |
| Every reading says the bridge is not configured | `PROJECT_PORTFOLIO_MANAGEMENT_WORKER_SERVICE_URL` is not set in the API's environment. |
| `unreachable` with a reason about `https` | The URL is plain `http` to a host that is not loopback, or `OUTBOUND_STRICT` is on and the host is private. |
| `unknown` and "refused" | WPM turned your token away (4xx): issuer, audience or permission differ between the services. |
| A supply figure with an age | WPM could not be reached; this is the last figure your own token fetched. |
| `not_sent_no_token` | The request reached PPM with no bearer token, so there was nothing to forward. |
| Skill gap says `unknown` | Expected: worker skills do not cross the bridge (FB-7). |
