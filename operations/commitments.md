# Operating commitments

One page: what this project commits to about running the service, and what it does not. Every line links to the
evidence or says **not offered**. This is a single-maintainer open-source project ([`GOVERNANCE.md`](../GOVERNANCE.md));
it ships software and an operations pack, not a hosted service. Where a line depends on how **you** run it, the line
says so, and the number is the one the pack is designed and tested to, not a guarantee.

| Commitment | Position | Evidence |
|---|---|---|
| **Backup frequency** | Nightly full logical backup of the database and the artifact store is the design default; run `scripts/backup.sh` more often to tighten it. The schedule is yours to run. | [Recovery objectives](recovery-objectives.md), [Backup and restore](backup-and-restore.md) |
| **How far back a restore goes** | To the moment of the newest backup you keep. How many you keep is your retention; the retention schedule suggests 35 daily plus 12 monthly as an example, not a setting. | [Backup and restore](backup-and-restore.md), [Retention schedule](../governance/retention-schedule.md#backups) |
| **Recovery point objective** | 24 hours with the nightly run (Mode A); 15 minutes with continuous archiving the deployment provides (Mode B). | [Recovery objectives](recovery-objectives.md) |
| **Recovery time objective** | 4 hours from the decision to restore to a service answering requests; measure it on your data. | [Recovery objectives](recovery-objectives.md), `scripts/restore-test.sh` |
| **That a backup restores** | Tested in CI: the whole suite runs on a restored copy. `scripts/check-backup-age.sh` fails when the newest backup is older than the RPO. | `scripts/restore-test.sh`, `scripts/check-backup-age.sh` |
| **Uptime objective** | **Not offered** as a number. The app tier is stateless and scales horizontally, so availability is what your deployment makes it; no figure has been measured. | [NFR-3](../spec/07-non-functional-requirements.md), [Production readiness](../governance/production-readiness.md) |
| **Support response targets** | **Not offered.** There is no service-level agreement and a single maintainer. | [`GOVERNANCE.md`](../GOVERNANCE.md), [Production readiness](../governance/production-readiness.md) |
| **List of processors** | None is built in. Hosting, database, backups, mail, telemetry and the identity provider are yours to list. | [Record of processing, section 7](../governance/record-of-processing.md#7-processors-and-sub-processors) |
| **Data processing agreement template** | **Not offered.** The project processes no data for anyone; whoever runs it is the controller or processor and writes their own. | [DPIA](../governance/dpia.md) |
| **Time to delete or anonymise personal data after a request** | **No fixed time is offered.** An administrator can export everything held about a person and replace the reference with a pseudonym everywhere, at once, in the live system. Backups keep the old reference until they age out; free text, uploads and audit snapshots are not found automatically. | [Retention schedule, known gaps](../governance/retention-schedule.md#known-gaps), [DPIA](../governance/dpia.md) |
| **Notice before a policy change** | **Not offered** as a period. Changes to behaviour are made in the specification first and recorded in the changelog; a deployment decides when to upgrade. | [Change control](../spec/18-change-control.md), [`CHANGELOG.md`](../CHANGELOG.md) |
| **Single sign-on** | OpenID Connect sign-in is supported; Microsoft Entra ID is documented and tested. | [Sign in with Entra](sign-in-with-entra.md) |
| **SCIM provisioning** | **Not offered.** | none |

## What this page is not

It is not a contract. A buyer who needs an uptime figure, support hours, a data processing agreement or a deletion
deadline gets them from whoever operates the service for them. If a line here becomes untrue, change the line in the
same commit as the cause.
