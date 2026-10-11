# For the Chief Information Officer

*Chief Information Officer: integration, identity, data, hosting and the information you will be asked to hand over.*

## Identity and access

- Login is **enforced by default**; the service refuses to start without token-verification keys, and in production with login off.
- Sign-in through an identity provider (including Microsoft Entra ID); multi-factor authentication stays with the provider ([sign-in guide](../../../operations/sign-in-with-entra.md)).
- Authorization uses token attributes (for example `access`, `directorpack`, `confidence`) with an attribute policy. Private plans are hidden from every aggregate; handling marks hide plans above a caller's clearance, administrators included.
- Break-glass access to private plans needs eligibility, a stated reason, at most four hours, and a second person told.
- Administrators can list standing access grants (a recertification aid) and export or pseudonymise one person's data.

## Data and integration

| Topic | Position |
|---|---|
| Data held | Plan, task, risk, budget, milestone, time and cost records; people are opaque references, not names. No patient data, no special-category data by design |
| Inventory | Every personal-data column is listed in the [personal-data inventory](../../../governance/personal-data-inventory.csv); a test keeps it true to the schema |
| Interfaces | REST with a derived OpenAPI document; bulk import and export; calendar feeds; outgoing events and webhooks to public `https` destinations only |
| Backups | Scripted backup with a tested restore, optional encryption, stated recovery targets ([backup and restore](../../../operations/backup-and-restore.md)) |
| Hosting | Container images and `compose.yaml`; the software makes no transfers of its own, so data location is your hosting choice |

## Gaps you must close

- Pen test, independent code review and certification: none done.
- Retention is only partly enforced; purge must be scheduled (see the [retention schedule](../../../governance/retention-schedule.md)).
- The data-subject tool finds references, not names typed into free text or uploaded files.
- The audit trail is not tamper-proof against a database administrator.

## Read next

[Data-flow diagram](../../../governance/data-flow.md) · [Assurance checklist](../../../governance/assurance-checklist.md) · [Deployment](../../../operations/deployment.md) · [INSTALL](../../../INSTALL.md)
