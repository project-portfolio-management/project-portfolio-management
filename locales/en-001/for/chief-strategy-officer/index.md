# For the Chief Strategy Officer

*Chief Strategy Officer: how the software connects strategy to the work, what it shows you, and what it cannot yet.*

## What it does for strategy

- **One registry of everything funded.** Portfolios, programmes, projects and products are one recursive record, so the whole estate can be read as a tree ([purpose and vision](../../../../spec/01-purpose-and-vision.md)).
- **Duplicate detection.** Explainable matching shows when two parts of the organization have chartered the same initiative, before both are funded.
- **Strategic aims and the portfolio map.** You mark objectives as strategic aims and name one primary aim for each programme. `GET /api/portfolio-map` and the monthly director pack group programmes by primary aim. A programme appears once, with its other aims named; a programme with no primary aim is listed under "no strategic aim", and totals are never double-counted.
- **Delivery confidence beside computed status.** People state their confidence in delivery; the software shows it next to the computed red/amber/green and flags a two-step gap. A plan nobody has assessed shows "not assessed", never green.
- **A frozen monthly pack.** What the directors were told is kept exactly as issued, with the trend measured against the previous issued pack.

## What it does not do yet

- The pack does not yet carry gate decisions, funding by financial year, the buffer fever chart, earned value, validated benefits or the capacity heatmap (`DP-5` to `DP-10` in [`tasks.md`](../../../../tasks.md)). The pack says which sections are not built.
- "Programme" means a plan whose kind is `Program`, because the data has no other definition; "top-tier" aims are whatever you mark, since objectives have no hierarchy.
- There is no scenario planning, and no screen yet for the map or the pack: they are available as data.
- It is not certified, and it has one maintainer ([production readiness](../../../../governance/production-readiness.md)).

## Decisions that are yours

Which objectives are strategic aims; the primary aim for each programme; who states and who independently assesses delivery confidence; how many months of trend the directors should see.

## Read next

[Roadmap](../../../../spec/15-roadmap.md) · [Open questions](../../../../spec/16-open-questions.md) · [Scope](../../../../spec/02-scope.md)
