# Organization-defined issue types, statuses and roles

Task OC-1 in [`tasks.md`](../tasks.md). An organization can define its own issue vocabulary, as Redmine does with trackers,
statuses, roles and a workflow matrix. It extends the issues register (FR-14) and the custom workflows (FR-26). With nothing
configured, every issue behaves as it did before.

## The model

| Redmine | Here | Where |
| --- | --- | --- |
| Tracker | Issue type | `issue_types`, `/api/issue-types` |
| Issue status | A state of an issue workflow | `workflow_states`, `/api/workflows` |
| Role | A role and its permissions | `organization_roles`, `/api/organization-roles` |
| Project member | A person's role on a plan | `plan_role_members`, `/api/plans/{pid}/role-members` |
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
read from the token's `role` attribute and the roles held on the plan, as for tasks (RM-2). An issue workflow must make `closed` terminal, and cannot start in
a finished or terminal state (`422`). Where moves are constrained, a new issue must start in the initial
state, so that nobody raises an issue past a gate.

**A type change cannot step round a gate.** A type decides the workflow, so changing the type of an issue needs an
administrator (`403`) if the workflow in force for the old type or the new one restricts moves or fields. With neither restricted there
is no gate to step round, and anyone who can write may change it.

**A workflow can change under an issue.** The issue keeps its status and stored category, and a save that does not move it
succeeds, though it cannot lose a resolution it has. It may move to any state the workflow in force declares, as long as a
gate holds: if every move into that state is role-restricted, the caller needs one of those roles. A status filter accepts a built-in status, a state of a deployment-wide
issue workflow (withdrawn ones included), and any status declared or carried on a plan the caller can read, so such issues stay
findable. Any other word is refused (`422`), so a typo is not answered with an empty list, and a word only a hidden plan uses
gets the same answer as a word nobody uses (OC-2).

## What a role does (OC-3)

This is the part of the feature with a security consequence. Redmine's rule is the model: a role is a named set of permissions,
each member holds one or more roles on a project, and a person's permissions are those of all their roles combined. The same
person can hold different roles on different plans.

**A role carries permissions.** `issue_add` raises an issue, by hand or from a risk or a blocked task. `issue_edit` changes its
fields or type. `issue_move` changes its status, with the resolution that goes with that change. `issue_delete` removes it. The
list is fixed; any other word is refused (`422`).

**A plan chooses to be enforced.** A plan with no entries in `plan_role_members` behaves as it did before, so nothing changes for
anyone until an administrator adds the first member. From then on, each of those actions on that plan needs the permission
(`403`), from a role that is not retired, whether held on the plan or carried by the token's `role` attribute. Reading is not
affected. A save that moves an issue and changes nothing else needs only `issue_move`; any other save needs `issue_edit`, and a
save that does both needs both. Removing the plan's last member ends enforcement. An issue raised by the service's own paths on a
control action is not checked, because the person is acting on the control.

**The token stays the ceiling.** A write must pass the token's policy before any of this runs, so a role can only narrow what a
caller may do. A role cannot make a read-only token write, cannot open a private plan, and cannot make anyone an administrator:
that comes from `access=admin` alone, and a test proves that holding a role named `admin` gives nothing. A person cannot be given
a role on a private plan until they are on its access list (`422`), because the membership would do nothing.

**Roles open gates.** A role held on a plan counts, with the token's, for that plan's role-restricted moves, on issues and on
tasks. It counts on that plan only. A retired role held on a plan counts for nothing. A gate satisfied by a role in the token keeps
working, as it always has.

**Naming a role.** The catalogue also names the roles a deployment recognizes, so that a transition cannot be restricted to a
misspelt role, which would lock everyone out of that move. Once the catalogue has an entry, registering a workflow that names a
role outside it, or a retired one, is refused (`422`). An empty catalogue checks nothing, so a deployment that has not defined
roles is unchanged. Workflows already registered keep working.

## Field rules by status (OC-4)

A state of an issue workflow can carry `field_rules`, as Redmine's workflow field permissions do. Each rule names a field
(`title`, `description`, `severity`, `priority`, `owner_ref`, `due_date` or `resolution`), makes it `required` or `read_only`, and
binds everyone or only the roles it lists. Roles here are the caller's roles on the plan, as for a gate.

**Required** binds a save that leaves the issue in that state, by hand or by a move: the field must be filled (`422`, naming it).
It binds an administrator too. An issue the service raises from a risk, a blocked task or a control action is not checked, because
nobody has had the chance to fill anything; the next person to save it must.

**Read-only** binds a save while the issue is in that state: the field cannot change, and clearing it is a change (`403`, naming
it). It is read from the state the issue is in before the save, so a state can be locked and a move out of it is still possible.
It does not bind an administrator, who has to be able to correct a record.

A field cannot be both required and read-only in one state, whoever each rule binds, because a blank one could never be saved
(`422`). Task workflows cannot carry field rules (`422`). A type change under a workflow with field rules needs an administrator,
for the same reason as under one that restricts moves.

## Who can change what

Anyone signed in can read the catalogues, and anyone who can see a plan can read its role members. Only an administrator
(`access=admin`) can change a catalogue or a plan's role members, or register or withdraw an **issue** workflow, because that workflow decides who may move an issue. Every change is audited, and a workflow's
audit row carries its pid, type, plan and moves with their roles.

Before this feature issue workflows did nothing, and anyone who could write could register one. **The migration withdraws
them**, so that none gains authority nobody gave it. An administrator registers the ones wanted.

`POST` and `DELETE /api/workflows` do not require an administrator for a **task** workflow, so a task workflow's role restriction
can be replaced by anyone who can write. That was already so and is left as it is; see "Not built".

## The API

`GET`, `POST /api/issue-types`; `PUT /api/issue-types/{key}`; `GET`, `POST /api/organization-roles`;
`PUT /api/organization-roles/{key}`, whose bodies take `permissions`; `GET`, `POST /api/plans/{pid}/role-members`;
`DELETE /api/plans/{pid}/role-members/{member_pid}`. The lists take `?include_retired=true`. `POST /api/workflows` takes `issue_type`.
`GET /api/plans/{pid}/issues/options` returns each type with the workflow in force for it on that plan, retired types marked,
and the caller's roles, permissions and whether the plan is enforced, so a form can leave out what the service would refuse. An issue's body takes `status` as optional: absent
means the workflow's initial state on a new issue and no change on an existing one. An issue's response adds `status_category`.

## The interface

`/issue-settings` lists the types and roles, adds, retires and restores them, and registers and withdraws deployment-wide issue
workflows. A plan's issue form offers the types and statuses from the options above, and falls back to the built-ins if it cannot
read them. The service decides in every case. The cross-plan register has no workflow to hand, so it shows a custom status's key
as words (`with_supplier` reads "With supplier").

## Not built

- Only an administrator manages a plan's role members. Redmine also lets a project member with the permission do it.
- Permissions cover issue actions only. Redmine's roles also govern queries, notes, watchers, relations and more, and can narrow
  which issues a role sees. Here, what a person sees is decided by the plan's privacy alone.
- No group of people can hold a role; each person is added.
- Field rules cover the listed fields. Redmine's also cover custom fields.
- Issue types are deployment-wide: no per-plan choice of types.
- A plan's own issue workflow is registered through the API only.
- A registered workflow cannot be edited, only withdrawn and registered again.
- `severity` is not configurable.
- The custom labels are one language: an organization's own type and status names are not translated.
- A custom status has no entry in the UI's translated labels. The cross-plan register shows it as words from its key.
- Task workflows do not require an administrator (above). That gap predates this work and needs the owner's decision.
- Migrating down fails while an issue carries a custom type or status, by design.
