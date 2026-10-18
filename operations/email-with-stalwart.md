# Email with Stalwart

The service sends email for notifications and for "email" sinks (`PROJECT_PORTFOLIO_MANAGEMENT_EMAIL_FROM`,
`SMTP_HOST`, `SMTP_PORT`, `SMTP_USER`, `SMTP_PASSWORD`; see [deployment](deployment.md)). For a development
stack or a small site that has no mail host, `compose.yaml` has an optional container running
[Stalwart](https://stalw.art/), an all-in-one mail server (SMTP, IMAP, JMAP). It is a **profile**: nothing
starts unless you ask.

## Start it

```sh
export PPM_MAIL_ADMIN='admin:<choose a password>'      # optional: pins the bootstrap administrator
podman compose --profile mail up -d mail               # the mail server on its own
```

- The image is pinned to the `v0.16` line (`docker.io/stalwartlabs/stalwart:v0.16`), not `latest`.
- Data is in the volumes `mail-config` and `mail-data`. They are **not** in the database backup: back them up
  with the rest of the deployment ([backup and restore](backup-and-restore.md)) or accept losing mail.
- Everything is published on `127.0.0.1` only: administration and web mail `8081`, SMTP in `2525`,
  submission `2587` and `2465`, IMAP `2143` and `2993`. (A rootless Podman cannot bind ports under 1024, so
  the host ports are not the standard ones; inside the compose network the server is `mail:587` and so on.)

## First-time setup (by hand, once)

Stalwart starts in **bootstrap mode**: only the administration page is open, until it is configured.

1. Open `http://127.0.0.1:8081/` and sign in as `admin` with the password you pinned, or the one-time password in `podman compose --profile mail logs mail`.
2. Complete the setup: your **domain**, the **hostname**, and a **certificate** for it.
3. Create an **account** for the API to send as (for example `ppm@your-domain`) and give it a password.
4. Point the API at it and restart:

   ```sh
   SMTP_HOST=mail SMTP_PORT=587 SMTP_USER=ppm@your-domain SMTP_PASSWORD='<its password>' \
   EMAIL_FROM=ppm@your-domain \
   podman compose --profile mail up -d
   ```

5. Check in the service: add an email sink and send a test notification
   (`/api/sinks`); the outcome is recorded with the delivery.

## How the service sends

Every email goes through a **sender** (`src/email.rs`, the `Sender` trait): the SMTP mailer the settings above
configure, or none (then each attempt fails for good and says "no mailer is configured"). Each message goes
to **one recipient at a time**, and each recipient has a row in `email_deliveries`: `queued`, `retry`, `sent`
or `failed`, the attempts, the last error and which sender tried. A transient failure (a connection refused)
is retried after 1, 5, 25 and 125 minutes, five attempts in all; a permanent one ends at once. Asking again
for the same message and recipient never sends twice. Administrators read the list at
`GET /api/email-deliveries`; nobody else can (404), because it holds addresses. Today the notification sinks
of kind `email` use it; other callers (for example, mailing a pack's subscribers) would use the same
`deliver` function.

## What is and is not verified

- **Verified here:** the sender, the per-recipient rows, retry and no-duplicate behaviour against a recording sender and the test mailer (`tests/requests/email.rs`, `tests/sinks.rs`); the profile starts the pinned image, it reports healthy, the pinned administrator
  password is accepted by the administration API, and the compose file parses with and without the profile.
- **Not verified:** the setup steps above (they use Stalwart's administration page, whose screens change
  between releases), a message actually delivered from the service through Stalwart to a mailbox (the service has been tested against a recording sender and the test stub, not against this server), DNS (MX, SPF, DKIM, DMARC)
  and any delivery to the outside world. Public mail needs DNS records and a reputation that a container
  cannot give you.
- **The API's mailer expects a trusted TLS certificate** (`secure: true` in `config/production.yaml`). A
  self-signed certificate made by Stalwart will be refused; use a certificate from a CA the API container
  trusts, or send through a relay you already run.
- The bootstrap password is `PPM_MAIL_ADMIN` or the one-time log line; once Stalwart has provisioned its
  permanent administrator, stop using it and rotate. The variable is optional so the rest of the stack
  starts without it (it was once required, which broke `podman compose` for every profile).
