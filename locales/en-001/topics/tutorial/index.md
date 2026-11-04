# Tutorial

*Run the demonstration on your own computer and tour a whole portfolio.*

You will start a complete copy of the software, filled with a made-up organization called the Northgate Transport Authority. All the people, plans, money and dates are fictitious, so feel free to change anything.

**Keep it on your own computer.** The demonstration trusts a published test key, so never put it on a shared network or use it with real data.

## What you need

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) with Compose

## Step 1: get the code

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Step 2: start the demonstration

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

This builds the software, starts the database, fills it with the fictitious portfolio and starts the interface. The first build takes a while.

## Step 3: sign in

Open <http://localhost:3000/en-001/signin>, leave the address as `example@example.com` and press **Sign in to the demonstration**. No email is sent.

You land on the **demonstration tour**, which lists every feature in groups, with what to look for and a link. A yellow banner on every page reminds you that the data is fictitious.

## Step 4: see the portfolio at a glance

From the tour, open **Plans** to see the portfolio, its programmes and its projects, then open the **Portfolio dashboard**. Notice that any plan can contain any other.

## Step 5: follow one project

Open the **North Line interlocking** project through the tour's delivery group. Look at its **Gantt** chart, then its **Critical path**, then its **Earned value**.

## Step 6: see your own work

Open **My work** and **My kanban board**. You are signed in as Example Person, who has some open tasks. Move a card to see how it works.

## Step 7: see how it speaks up

Open **Capacity heatmap** to see who is over-booked. Then open **Start queue**, where one programme has been refused by the work-in-progress limit and is waiting. Finish with **Director packs**, the monthly summary that is frozen once issued.

## Step 8: stop and clean up

This stops everything and deletes the demonstration data:

```sh
podman compose --profile demo down -v
```

## Where to go next

- [Install it properly](../../../../INSTALL.md), with login and your own identity provider
- Read the [role briefs](../../for/) for senior roles
- Read the [full demonstration guide](../../../../operations/demo.md)
- [See every feature](../features/index.md)

Stuck? Please [open an issue](https://github.com/project-portfolio-management/project-portfolio-management/issues).
