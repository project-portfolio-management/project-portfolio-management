# Organization-defined issue types, statuses and roles

Task OC-1 in [`tasks.md`](../tasks.md). An organization can define its own issue vocabulary, as Redmine does with trackers,
statuses, roles and a workflow matrix. It extends the issues register (FR-14) and the custom workflows (FR-26). With nothing
configured, every issue behaves as it did before.

## The model

| Redmine | Here | Where |
| --- | --- | --- |
| Tracker | Issue type | `issue_types`, `/api/issue-types` |
| Issue status | A state of an issue workflow | `workflow_states`, `/api/workflows` |
| Role | A role name | `organization_roles`, `/api/organization-roles` |
| Workflow matrix (role × tracker) | A workflow's transitions and their roles, with `workflows.issue_type` | `workflow_transitions`, `/api/workflows` |

**Issue types.** `issues.kind` is a key in `issue_types`, enforced by a foreign key. The migration seeds the six built-in
kinds. A type is retired, never deleted: it can no longer be chosen, and an issue that carries it stays valid and can still be
saved. The key cannot change, because issues store it. A body that leaves the type out does not change it. The service's own paths (an issue raised from a risk, a blocked task or a
control action) use the `risk`, `blocker` and `concern` keys even if those are retired.

**Statuses.** An issue's status is a state of the workflow in force for its type. Each state declares a category, and the
category is stored on the issue as `status_category`, set by the service and checked by the database. "Open", "overdue", the
register's counts and the director summaries read the category and never a status's name, so a status called `with_supplier`
in the `waiting` category counts as open. A **terminal** state needs a resolution. The database's own check still requires one
for `closed`.

**The workflow in force** for an issue of a type on a plan is the first of: the plan's workflow for the type; the plan's
workflow; the deployment default for the type; the deployment default; the built-in vocabulary. There is at most one in each
scope (`409`, and unique indexes). A workflow's **allowed moves** apply as they do to tasks: none listed means any move, and
once one is listed only listed moves are allowed (`422` otherwise). A move can be restricted to named roles (`403` otherwise),
read from the token's `role` attribute, as for tasks (RM-2). An issue workflow must make `closed` terminal, and cannot start in
a finished or terminal state (`422`). Where moves are constrained, a new issue must start in the initial
state, so that nobody raises an issue past a gate.

**A type change cannot step round a gate.** A type decides the workflow, so changing the type of an issue needs an
administrator (`403`) if the workflow in force for the old type or the new one restricts moves. With neither restricted there
is no gate to step round, and anyone who can write may change it.

**A workflow can change under an issue.** The issue keeps its status and stored category, and a save that does not move it
succeeds, though it cannot lose a resolution it has. It may move to any state the workflow in force declares, as long as a
gate holds: if every move into that state is role-restricted, the caller needs one of those roles. A status filter accepts a built-in status, a state of a deployment-wide
issue workflow (withdrawn ones included), and any status declared or carried on a plan the caller can read, so such issues stay
findable. Any other word is refused (`422`), so a typo is not answered with an empty list, and a word only a hidden plan uses
gets the same answer as a word nobody uses (OC-2).

## Roles grant nothing

This is the one rule here with a security consequence. **A role in `organization_roles` grants nothing, and the table is never
read to decide what a caller may do.** Authority comes from the attributes in the caller's token. The catalogue names the
roles a deployment recognizes, so that a transition cannot be restricted to a misspelt role, which would lock everyone out of
that move. Once the catalogue has an entry, registering a workflow that names a role outside it, or a retired one, is refused
(`422`). An empty catalogue checks nothing, so a deployment that has not defined roles is unchanged. Workflows already registered
keep working. A test proves that holding a role named `admin` does not make someone an administrator.

## Who can change what

Anyone signed in can read the catalogues. Only an administrator (`access=admin`) can change a catalogue, or register or
withdraw an **issue** workflow, because that workflow decides who may move an issue. Every change is audited, and a workflow's
audit row carries its pid, type, plan and moves with their roles.

Before this feature issue workflows did nothing, and anyone who could write could register one. **The migration withdraws
them**, so that none gains authority nobody gave it. An administrator registers the ones wanted.

`POST` and `DELETE /api/workflows` do not require an administrator for a **task** workflow, so a task workflow's role restriction
can be replaced by anyone who can write. That was already so and is left as it is; see "Not built".

## The API

`GET`, `POST /api/issue-types`; `PUT /api/issue-types/{key}`; `GET`, `POST /api/organization-roles`;
`PUT /api/organization-roles/{key}`. The lists take `?include_retired=true`. `POST /api/workflows` takes `issue_type`.
`GET /api/plans/{pid}/issues/options` returns each type with the workflow in force for it on that plan, retired types marked,
and the caller's roles, so a form can leave out a move the service would refuse. An issue's body takes `status` as optional: absent
means the workflow's initial state on a new issue and no change on an existing one. An issue's response adds `status_category`.

## The interface

`/issue-settings` lists the types and roles, adds, retires and restores them, and registers and withdraws deployment-wide issue
workflows. A plan's issue form offers the types and statuses from the options above, and falls back to the built-ins if it cannot
read them. The service decides in every case. The cross-plan register has no workflow to hand, so it shows a custom status's key
as words (`with_supplier` reads "With supplier").

## Not built

- Roles carry no permissions, and no per-project role membership. Redmine has both.
- No per-status or per-role field rules (Redmine's read-only and required fields).
- Issue types are deployment-wide: no per-plan choice of types.
- A plan's own issue workflow is registered through the API only.
- A registered workflow cannot be edited, only withdrawn and registered again.
- `severity` is not configurable.
- The custom labels are one language: an organization's own type and status names are not translated.
- A custom status has no entry in the UI's translated labels. The cross-plan register shows it as words from its key.
- Task workflows do not require an administrator (above). That gap predates this work and needs the owner's decision.
- Migrating down fails while an issue carries a custom type or status, by design.
