# Operations pack

Everything an operator needs to run the service, keep it safe, and bring it
back. Each guide says what was **tested** and what was not.

| Guide | Use it to |
|---|---|
| [Deployment](deployment.md) | stand the service up with Compose, configure it, put it behind TLS, scale it |
| [Recovery objectives](recovery-objectives.md) | read the stated recovery point and recovery time targets, and what each backup mode guarantees |
| [Backup and restore](backup-and-restore.md) | back up the database and the artifact store, restore them, and run the restore drill |
| [Upgrade and migration](upgrade-and-migration.md) | move to a new version safely, and roll back |
| [Runbook](runbook.md) | diagnose a symptom and act: refusals at start-up, `401`, `429`, `500`, slow, disk, search, keys |
| [Sign in with Microsoft Entra ID](sign-in-with-entra.md) | let people sign in through Entra (OpenID Connect), directly or through Keycloak |

The scripts live in [`../scripts`](../scripts): `backup.sh`, `restore.sh`,
`restore-test.sh`, `check-backup-age.sh`. The CI job `restore` runs the restore
drill on every push, including the test suite against the restored database.

Related: the information governance pack is in [`../governance`](../governance).
