# The DHCW Roadmap Editor against this service: a mapping

Written 2026-11-14 (RE-5 part a). It answers one question: if Digital Health and Care Wales replaced the Google
spreadsheet behind its [Roadmap Editor](https://github.com/DHCW-Digital-Health-and-Care-Wales/roadmaps-editor) with
this service, what would each operation of its repository port call, and what is missing?

**Status.** A paper mapping. No adapter has been written, nothing has been sent to DHCW, and nobody at DHCW has been
asked. Whether to offer it is the owner's question OQ-15 ([open questions](../spec/16-open-questions.md)). The port
was read at commit `cc81478` (`src/domain/roadmap/roadmap-repository.ts` and its doc comments, which are its
contract) and its own guide,
[replacing-google-sheets.md](https://github.com/DHCW-Digital-Health-and-Care-Wales/roadmaps-editor/blob/main/docs/operations/replacing-google-sheets.md).
The background is in [the lessons](../spec/dhcw-roadmaps-editor-lessons/index.md).

## Their model and ours

| Theirs | Ours |
| --- | --- |
| A roadmap (one row of the first tab) | A public product roadmap on a `Product` plan (`/api/plans/{pid}/roadmap`, RE-1). The editor's roadmap id is our `slug` |
| `Enabled` = `Yes` | `published` |
| `Last updated` (a date, set only on request) | `reviewed_on`, set only through `PUT …/roadmap/reviewed` (RE-3) |
| An item (one row of the roadmap's tab), with `Item ID` | A roadmap item, with `public_id`: the same shape, eight lower-case letters and digits starting with a letter |
| `Horizon` (`Now`, `Next`, `Later`, and three sections) | `placement` (`now`, `next`, `later`, `recently_delivered`, `delivered_this_year`, `not_doing`, or none) |
| `Labels`, `Details`, `Organisations` | `tags`, `details` (`{ text, level }`, three levels), `organization_pids` |
| `(cy)` columns, never written by the editor | `translations.cy`, kept and marked `needs_review` when the English changes (RE-6) |
| `Families` tab with a colour | A family is the product plan's parent plan; there is no colour (RE-7) |
| `Categories` tab | `/api/roadmap-categories` (RE-7) |
| Organisations list, `National` first | `/api/organizations` with `code` and `national`, national first (RE-7) |
| `Product Lead` (a name as free text) | Not mapped: this service holds people as references, and the public reader names no person |

## Operation by operation

| Port operation | This service | Gap |
| --- | --- | --- |
| `listRoadmaps` | No single route lists every roadmap with its fields for an editor. `GET /api/public-roadmaps/stale` lists only the stale ones; `GET /api/public/roadmaps` only the published ones, with no token | **A list route for editors**: every roadmap the caller may read, published or not |
| `listRoadmapItems(id)` | `GET /api/plans/{pid}/roadmap` returns the roadmap with its items in page order. The editor would need the plan id behind a slug | A lookup by slug for editors, or the plan id carried in the list |
| `listOrganisations` | `GET /api/organizations` (name, code, national, active) | Their `shortName` and a Welsh name have no field |
| `listFamilies` | `GET /api/plans?parent=…` gives a portfolio's children; there is no list of "families" as such | Families as a named list with colours; or the editor drops colours |
| `listCategories` | `GET /api/roadmap-categories` | none |
| `publishRoadmapChanges(id, changes)` | **RE-2**: `POST /api/plans/{pid}/roadmap/publish` with per-field base values, all or nothing, `409` naming each conflict, and a preview | Their item order is the sheet's row order; ours is `position` within a placement, moved by index |
| `addFamily`, `changeFamily` | Creating or renaming a portfolio plan | A family's colour |
| `matchRoadmapToLists` | Not needed: a roadmap links a category by id and keeps no copy, so nothing drifts | none |
| `setRoadmapPublished` | `PUT /api/plans/{pid}/roadmap` with `published` | none |

## Errors

Their guide maps statuses to error kinds. This service answers them as follows:

| Their kind | Status here |
| --- | --- |
| `unauthenticated` | `401` (no token, or an expired one) |
| `forbidden` | `403` (the access policy refuses) |
| `not-found` | `404`, also for a private plan the caller may not read |
| `rate-limited` | `429`, with `Retry-After` |
| `unknown` (retryable) | `5xx` |
| `invalid-data` | `422` (a bad field), `409` (a conflict or a slug clash) |
| `invalid-response` | not expected: every body is JSON |

## Sign-in and CORS

Their guide expects either Google tokens or the API's own sign-in. This service verifies tokens from the deployment's
own identity provider (OpenID Connect or PASETO) and does not accept Google access tokens. The editor would need a new
`AuthClient` adapter for that provider. Calls from the editor's GitHub Pages origin need that origin in the CORS
allow-list (`PROJECT_PORTFOLIO_MANAGEMENT_CORS_ORIGINS`), with the `Authorization` header allowed.

## Moving their spreadsheet in

Part (b) of RE-5 is a dry-run import of their documented spreadsheet format through the bulk-import path, so
every row is accounted for before anything is written. It is **not built**.

## What it would take, in order

1. A list route for editors, with plan ids (the first gap above).
2. RE-2's publish, which mirrors their change set closely.
3. The dry-run import.
4. On their side: an `AuthClient` for the deployment's identity provider, and an `ApiRoadmapRepository` as their guide
   describes.
