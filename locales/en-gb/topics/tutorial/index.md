# Tutorial

*Steps to run the demonstration on a local computer and review a sample portfolio.*

The demonstration is a complete copy of the software, loaded with data for a fictitious organisation, the Northgate Transport Authority. All people, plans, figures and dates are fictitious, and the data can be changed freely.

**Security note.** The demonstration trusts a published test key. It must not be run on a shared network or used with real data.

## Requirements

- [Git](https://git-scm.com/)
- [Podman](https://podman.io/) with Compose

## Step 1: download the code

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management
```

## Step 2: start the demonstration

```sh
POSTGRES_PASSWORD=demo podman compose --profile demo up -d --build --wait ui-demo
```

This command builds the software, starts the database, loads the fictitious portfolio and starts the interface. The first build takes longer than later ones.

## Step 3: sign in

Open <http://localhost:3000/en-001/signin>, leave the address as `example@example.com` and press **Sign in to the demonstration**. No email is sent.

The first page is the **demonstration tour**, which lists every feature by group, with a description and a link. A yellow banner on every page states that the data is fictitious.

## Step 4: review the portfolio

From the tour, open **Plans** to view the portfolio, its programmes and its projects. Then open the **Portfolio dashboard**.

## Step 5: review one project

Open the **North Line interlocking** project through the tour's delivery group. Review its **Gantt** chart, **Critical path** and **Earned value**.

## Step 6: review personal work

Open **My work** and **My kanban board**. The session is signed in as Example Person, who has open tasks.

## Step 7: review capacity, start control and reporting

Open **Capacity heatmap** to view over-booked resources. Open **Start queue**, where the work-in-progress limit has held back one programme. Open **Director packs**, the monthly summary that is frozen once issued.

## Step 8: stop the demonstration

This command stops all services and deletes the demonstration data:

```sh
podman compose --profile demo down -v
```

## Next steps

- [Installation guide](../../../../INSTALL.md), covering login and identity providers
- [Role briefs](../../for/) for senior roles
- [Demonstration guide](../../../../operations/demo.md)
- [Feature list](../features/index.md)

Problems can be reported through the [issue tracker](https://github.com/project-portfolio-management/project-portfolio-management/issues).
