# Recovery objectives

**Recovery point objective (RPO)**: how much recent data you can afford to lose.
**Recovery time objective (RTO)**: how long you can be without the service.
These are the targets this pack is designed and tested to. They are defaults:
a buyer with different needs changes the numbers here, in the schedule that
runs the backups, and in spec NFR-16.

| | Mode A: scheduled logical backups (this repository's scripts) | Mode B: continuous archiving or managed point-in-time recovery (the deployment provides it) |
|---|---|---|
| What it protects | database and artifact store | database (the artifact store still needs Mode A's archive or object-store versioning) |
| **RPO** | **24 hours** with a nightly run; tighten it by running `scripts/backup.sh` more often (each run is a full dump) | **15 minutes** with WAL archiving at that interval |
| **RTO** | **4 hours** from the decision to restore to a service answering requests | **4 hours**, usually much less |
| Guaranteed by | `scripts/check-backup-age.sh` failing when the newest backup is older than the RPO; the restore drill proving a backup restores | the database platform's own monitoring of its archive lag |

## What the 4 hours are made of

Provision or recover a database server, restore the dump, unpack the artifact
store, start the service, smoke-test it. The restore drill in CI times the
database restore and the comparison on a small dataset; **measure it on your
data**: restore the latest backup into a scratch database
(`scripts/restore-test.sh -k`), note the elapsed time, and add your
provisioning time. If the total is not comfortably inside the RTO, change the
design (a warm standby) rather than the number.

## What is not backed up, and why that is acceptable

| Item | Why |
|---|---|
| Full-text search index | Derived from the database; rebuilt with `task search_reindex` (and automatically when it is empty and records exist) |
| Rate-limit state, caches, in-flight requests | Held in memory; lost on restart by design |
| Container images | Pinned by version tag in the registry; redeploy the same tag |
| Secrets and signing keys | Kept in your secret store, not in the backup; the token keys are re-fetched from the issuer |

## Other limits

- **The artifact store and the database are backed up one after the other**, not
  atomically. A file uploaded between the two steps is in the database but not
  the archive (or the reverse). The service tolerates a missing file (the
  download answers `404`); the restore drill compares file counts so a large
  gap is visible. Back up at a quiet time, or stop writes, if you need exactness.
- **A restore returns the system to the backup's moment.** Everything after it
  is gone (that is the RPO); audit history after that moment is lost with it.
- **Mode A's RPO is the interval, not zero.** Do not claim a tighter figure
  than the schedule you actually run.
