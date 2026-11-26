# Frequently asked questions

[Getting started](#getting-started) · [Imports and duplicates](#bringing-existing-work-into-the-registry) · [Delivery and capacity](#managing-delivery-and-capacity) · [Reporting and approvals](#reporting-and-approvals) · [Sharing and access](#sharing-language-and-access)

## Getting started

### What should we register as a plan?

Register an initiative you want to identify, fund or track in its own right. Give it a name. You can label it a portfolio, programme, project or product and link it to a parent plan. For example, a transport portfolio can contain a rail programme, which contains a station upgrade. “Inspect the platform” belongs on the upgrade’s task list. You can start with the upgrade and add its parent later. [Plan model](../../spec/05-domain-model.md)

### How can I try the software before entering our portfolio?

From the repository root, run the command below on a private machine with Podman and Compose. Open `http://localhost:3000/en-001/signin`, use `example@example.com` and select **Sign in to the demonstration**. In the tour, open the **North Line interlocking** project to see how its tasks, schedule and costs fit together. The demo uses a published test key. Keep it isolated and use fictitious data only. [Demo walkthrough](../../locales/en-001/topics/tutorial/index.md)

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

### Can we host it ourselves and use our organisation’s sign-in?

Yes. Your organisation can run the web interface and API with its own PostgreSQL database. An administrator configures the identity provider. For Microsoft Entra ID, the sign-in guide documents a tested route through Keycloak. The workforce and contact-management services are optional. [Installation](../../INSTALL.md) · [Entra sign-in guide](../../operations/sign-in-with-entra.md) · [Optional integrations](../../README.md#works-on-its-own)

### Which licence can our organisation use?

Choose any one of the seven licences listed in the repository. They include MIT, Apache-2.0 and the UK Open Government Licence v3.0. The licence guide links to each full text and gives the contact for custom terms. [Licence choices](../../LICENSE.md)

### Can I use it on a phone or work offline?

The web interface has a phone layout. The repository also contains early iOS, iPadOS and Android apps for finding or creating a plan, reading its tasks and moving a task to another status. The native apps require a connection and keep plan and task text in memory only. Both are unreleased and unavailable in app stores. [Mobile scope and data handling](../../spec/mobile-apps.md)

## Bringing existing work into the registry

### We have a spreadsheet with one row per project. How do we load it?

Use the plans bulk-import API, `POST /api/plans/import`. It accepts CSV with a header row or JSON Lines with one plan per line. Include a stable identifier, such as the same Jira project key on every import of that project. Use the returned job ID to check how many rows were created, updated, unchanged, held for review or rejected. The error report identifies the rows to fix. [File formats, job results and worked rules](../../spec/bulk-import-export.md)

### Can I bring tasks across from Jira, Asana or Microsoft Project?

Open the destination plan’s **Import** page. Choose a Jira or Asana CSV export, or save a Microsoft Project schedule as MSPDI XML first. A native `.mpp` file must be exported to XML before uploading. Preview the file to see which tasks would be created or updated and why any rows, links or fields were skipped. Confirm when the preview looks right. [Import screen](../../project-portfolio-management-ui-with-svelte/src/routes/plans/%5Bpid%5D/import/+page.svelte)

### If I import the same Jira or Asana file again, what happens to our edits?

Within the same plan, the importer recognises the Jira issue key or Asana task ID. It updates matching tasks in place. Titles, dates and parent links can change on re-import. Existing task statuses and descriptions are preserved. Imported assignee names are recorded in the description, so assign the tasks to the appropriate people here. [Tracker import rules](../../project-portfolio-management-service-with-rust/src/controllers/tracker_import.rs)

### Why did two plans with different names or kinds match?

Start with the match breakdown. Two plans can share a Jira project key, the same identifying URL, or a code issued by the same organisation. Without an exact identifier, the matcher compares evidence such as alternate names, goals, dates and parent plan. A team that calls its work a “product” can therefore find the same initiative registered elsewhere as a “project”. [How match evidence is calculated](../../agents/matching.md)

### Does a duplicate warning merge our records?

No. A likely duplicate found during a bulk import without a stable key goes to the review queue by default. Confirm the incoming row to create a separate plan, or reject it to discard the row. If both plans already exist and represent the same initiative, use **Merge** to compare them, choose which record to keep and confirm. The duplicate is then soft-deleted and the merge appears in the history. [Review queue decisions](../../spec/bulk-import-export.md#implementation-notes-what-differs-from-the-design-above) · [Merge screen](../../project-portfolio-management-ui-with-svelte/src/routes/plans/merge/+page.svelte)

### Can we export our register?

Yes. The plans export API produces CSV or JSON Lines for plans you have permission to read. By default the export masks identifying fields such as the lead reference. A full export requires additional permission. Download the result before the job’s expiry time. To recover the whole service, including its operational records, use the database backup and restore procedure. [Export profiles](../../spec/bulk-import-export.md#8-export) · [Backup and restore](../../operations/backup-and-restore.md)

## Managing delivery and capacity

### Where do I see everything assigned to me today?

Open **My work**. It groups your open tasks across the plans you can read into overdue, today, this week, later and undated, using your browser’s time zone. If you expected a task to appear, check its assignee, whether it is still open and whether you can read its plan. [Personal work walkthrough](../../locales/en-001/topics/tutorial/index.md#step-6-review-personal-work)

### Will a delay to my task move the project finish date?

Check **Critical path** on the plan or the float shown beside your task in **My work**. Float is the scheduling margin in business days. With zero float, a delay can push the planned finish out. To test a specific change, open the plan’s **What-if** view, stage the edit and preview the before-and-after dates. Only committing the preview changes the live schedule. [Schedule walkthrough](../../locales/en-001/topics/tutorial/index.md#step-5-review-one-project) · [What-if screen](../../project-portfolio-management-ui-with-svelte/src/routes/plans/%5Bpid%5D/what-if/+page.svelte)

### Why does the capacity heatmap say supply is “unknown”?

For a skill pool linked to Workforce Planning Management, the heatmap uses the supply that service can disclose to you. Read the reason for the affected month. The service might be unreachable, your sign-in might lack access, or a small count might be hidden for privacy. Only a reported figure tells you how much capacity is available, measured in full-time equivalents. [Reading the heatmap and troubleshooting the link](../../operations/wpm-bridge.md#4-read-supply-on-the-heatmap)

### Why is our programme in the start queue?

Open **Start queue** to see the reason next to the programme. The organisation may already have its permitted number of programmes in delivery, or a required skill pool may be short in particular months. The queue shows requests in the order they were made. Use the named limit or pool to decide what must change before trying again. [Start queue walkthrough](../../locales/en-001/topics/tutorial/index.md#step-7-review-capacity-start-control-and-reporting) · [Start queue screen](../../project-portfolio-management-ui-with-svelte/src/routes/start-queue/+page.svelte)

### Can we compare funding options before choosing which plans to back?

Yes. In **Scenarios**, create candidate portfolios, choose the plans to include and set a budget cap. Evaluate the alternatives against live data. You can also generate a draft that must include particular plans and inspect why others were left out. Committing a feasible scenario is a separate decision. [Scenario planning screen](../../project-portfolio-management-ui-with-svelte/src/routes/scenarios/+page.svelte)

## Reporting and approvals

### Who decides whether a plan is on track?

A person records the assessment on the plan’s **Status** page. Add the evidence behind the call, such as a milestone or a measured figure, and set a cadence if updates are due regularly. A reader can see the latest judgement, its evidence and when another update is due. [Status updates and evidence](../../project-portfolio-management-ui-with-svelte/src/routes/plans/%5Bpid%5D/status/+page.svelte)

### Why is a cost or schedule performance figure blank?

On **Earned value**, check the reason beside the missing figure. For example, planned value needs a phased budget baseline, and a cost performance index needs actual cost greater than zero. Check the baseline, actuals and progress for the currency and date you are reading. Amounts in different currencies stay separate. [Earned value calculations and worked cases](../../tasks.md#scheduling-and-cost-control-canon-sc-n)

### How do I get a change to scope or budget approved?

Open **Change requests** on the plan, draft the change and link the issues or risks that explain it. Submit it to a named approver. The person who raised it cannot approve it. After approval, record implementation when the work is done. If the change warrants a new baseline, take that as a separate step. [Change request workflow](../../project-portfolio-management-ui-with-svelte/src/routes/plans/%5Bpid%5D/change-requests/+page.svelte)

### Why do the board papers differ from today’s dashboard?

An issued monthly director pack preserves the figures prepared for its audience and cut-off. Open **Director packs** and check the month, version and cut-off on the cover. Later edits to plans can change a live dashboard. To correct an issued pack, reissue it with a change note so readers can still see the version the board received. [Director pack rules](../../tasks.md#monthly-director-pack-dp-n)

## Sharing, language and access

### Why can my colleague open a plan link when I get “not found”?

Check that you are signed in with the intended account and that the link is current. A private plan returns the same “not found” response to a non-member as a missing plan. A handling mark above your clearance can also hide it. Ask your administrator to check your membership and clearance. [Plan access policy](../../governance/security-policy.md)

### Can a partner read our product roadmap without an account?

Yes, if your administrator enables the public roadmap reader and the product team publishes the roadmap. The public API serves only placed items from published roadmaps of live, non-private plans at the lowest handling level. It excludes internal IDs and person-reference fields. Write titles and descriptions for a public audience because those words can appear as entered. [Public reader and publication rules](../../project-portfolio-management-service-with-rust/src/controllers/public_reader.rs)

### We corrected a roadmap yesterday. Why does it still say “stale”?

Freshness measures the time since someone explicitly reviewed the roadmap. Editing or publishing a correction leaves the review date in place. After reviewing the whole roadmap, record the review date. With the default setting it becomes stale on day 43. A roadmap with no recorded review has unknown freshness. [Roadmap review rules](../../spec/06-functional-requirements.md)

### Which languages can I use, and how can I make the interface easier to read?

Choose a language in the web header. Shared links keep the chosen language. English is the source language. Welsh and the other translations are machine-written drafts awaiting speaker review, and some service messages appear in English. Under **Display**, you can choose simple mode and a dyslexia-friendly typeface. Charts have table alternatives. The accessibility statement identifies where manual and screen-reader testing is still needed. [Language coverage](../../governance/language-statement.md) · [Accessibility statement](../../governance/accessibility-statement.md)

### Is this suitable for patient records or a regulated deployment?

The project recommends starting with a pilot using non-clinical, non-secret portfolio data. Patient records and clinical decision support are outside its intended scope. For a deployment decision, review the assurance checklist, the evidence for implemented controls and the outstanding independent assessments with your organisation’s assurance team. [Production readiness](../../governance/production-readiness.md) · [Assurance checklist](../../governance/assurance-checklist.md)

### How do I report a problem or request a feature?

For an ordinary bug or feature request, use the project’s issue tracker and describe the action you took, what you expected and what happened. For a suspected vulnerability, use the private reporting channel in the security policy. [Contributing guide](../../CONTRIBUTING.md) · [Private security reporting](../../SECURITY.md)

## How the questions were chosen

These are anticipated questions from the project’s portfolio office, sponsor, project lead and public reader roles. The repository records no interviews with those users. The following sources informed the choice of topics. Each answer above links to this project’s own guidance or implementation. [Project personas](../../spec/personas/index.md)

- How a portfolio office organises programmes and projects and reports across them. [OpenProject portfolio guide](https://www.openproject.org/docs/use-cases/portfolio-management/)
- What teams migrating between tools need to know about CSV files and field mapping. [Atlassian’s Jira import guide](https://support.atlassian.com/jira-cloud-administration/docs/import-data-from-a-csv-file/)
- How project managers ask about critical tasks and the delay a schedule can absorb. [Zoho Projects critical path FAQ](https://help.zoho.com/portal/en/kb/projects/faqs/critical-path/articles/critical-path-9-2-2024)
- How portfolio managers check available capacity across projects. [Asana’s workload guide](https://help.asana.com/s/article/how-to-use-portfolio-workload)
- What a publicly funded product team can share with partners and the public. [Digital Health and Care Wales delivery playbook](https://gigcymru.github.io/dhcw-delivery-playbook/working-in-the-open/)
