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

**Types by plan (OC-5).** A plan can limit itself to some of the types, as a Redmine project selects its trackers. An
administrator sets the list with `PUT /api/plans/{pid}/issue-types`; an empty list offers every type. A person can only choose a
type the plan offers (`422`). An issue that already carries a type the plan leaves out keeps it and can still be saved, and the
service's own paths are not limited, as with a retired type. Only an administrator defines a type or changes a plan's list.

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

**Changing a workflow (OC-6).** `PUT /api/workflows/{pid}` replaces an issue workflow's name, states and moves in one change, with
the same checks as registering one. An administrator only, audited with what it was. What it governs (its plan, type and default
flag) cannot change: withdraw it and register another. A task workflow is not changed in place (`422`), because withdrawing one
checks for live work in a state that would go; withdraw and register it, as before.

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
save that does both needs both. A resolution given with a move into a finished or terminal state counts as part of the move; given
with any other save, it is an edit. Removing the plan's last member ends enforcement. Converting a control action into an issue needs `issue_add` too, because the
text of an action is whatever the caller typed.

**The token stays the ceiling.** A write must pass the token's policy before any of this runs, so a role can only narrow what a
caller may do. A role cannot make a read-only token write, cannot open a private plan, and cannot make anyone an administrator:
that comes from `access=admin` alone, and a test proves that holding a role named `admin` gives nothing. A person cannot be given
a role on a private plan until they are on its access list (`422`), because the membership would do nothing.

**Removing an issue.** On an enforced plan, `issue_delete` is enough. On a plan with no members, removing an issue under a workflow
that restricts moves or fields needs an administrator (`403`), because removing an issue and raising another would step round them.
An audit row keeps the issue's title, type and status.

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
it). It is read from the state the issue is in before the save, so a state can be locked and a move out of it is still possible. A
read-only rule is only as strong as the moves around it: in a workflow with no restricted moves, someone can move an issue out, edit
it and move it back. To hold a field, also restrict the moves out of that state.
It does not bind an administrator, who has to be able to correct a record.

A field cannot be both required and read-only in one state, whoever each rule binds, because a blank one could never be saved
(`422`). Task workflows cannot carry field rules (`422`). A type change under a workflow with field rules needs an administrator,
for the same reason as under one that restricts moves.

## Translated labels (OC-6)

An issue type, a role and a status can each carry `labels`, a label per language code (`cy`, `zh-cn`, `en-gb-oxendict`), beside
their one `label`. The interface shows the label for the reader's language, then for the language without its region, then the
one `label`. An organization writes these itself; nothing is machine-translated, and they are not in the translation reviews.

The cross-plan register has no workflow to hand, so the service adds the words: each issue carries `status_label` and
`status_labels` when a live workflow on its plan and type declares its status, nearest scope first. `summary.status_words` gives
the words for the counts; if two plans word one status differently, the first one seen speaks for both. A status no live workflow
declares has no words, and shows as its key in plain words.

## Saves and hidden plans

A save is refused (`409`) if the issue's status or type changed after the save read it, so a stale save cannot undo a gated move.
A workflow is read and written through its plan's visibility: to someone who cannot read the plan, the plan and its workflows are
`404`, for every method and whatever else is wrong with the request. An issue's audit rows are hidden with its plan, in the recent
feed and by pid. "One open issue per source" looks at this plan only. Linking an issue to a change request is not an edit of the
issue, needs no permission here and is audited on the change request.

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
`DELETE /api/plans/{pid}/role-members/{member_pid}`. The lists take `?include_retired=true`. `POST /api/workflows` takes `issue_type`; `GET /api/workflows/{pid}` reads one with its states and moves; `PUT /api/workflows/{pid}`
changes an issue workflow.
`GET`, `PUT /api/plans/{pid}/issue-types` read and set a plan's list. `GET /api/plans/{pid}/issues/options` returns each type with
the workflow in force for it on that plan, marked `offered` if a person can choose it, retired ones marked too,
and the caller's roles, permissions and whether the plan is enforced, so a form can leave out what the service would refuse. An issue's body takes `status` as optional: absent
means the workflow's initial state on a new issue and no change on an existing one. An issue's response adds `status_category`.

## The interface (OC-7)

`/issue-settings` covers the whole organization: types and roles with their translations, what each role allows, and
deployment-wide workflows. `/plans/{pid}/issue-settings` covers one plan: who holds which role, which types it offers, and its own
workflows. Both use one workflow editor, which registers a workflow, changes one in place, and sets each status's translations and
field rules. Translations are typed as `cy=Risg; zh-cn=风险`. Anyone who can see these pages can read them; the service refuses a
change from anyone but an administrator, and the page says so in the catalogue's words.

A plan's issue form offers the types the plan offers, starts where the workflow says, offers only the caller's moves, leaves out what
their roles do not allow, disables a field the status has locked for them, and names any required field left empty, in the reader's
language. It falls back to the built-ins if it cannot read the options. The service decides in every case. The cross-plan register
shows the words the service sends with each issue, in the reader's language where the organization has given one.

## Not built

- An audit row about a role keeps the person's reference in its snapshot, which erasure does not search; see the retention schedule.
  The membership itself is deleted on erasure under the delete policy.
- Control readings and actions are reached by their own pids, outside the plan guard. Conversion now checks the plan; reading them
  and adding to them do not, which predates this work.
- Only an administrator manages a plan's role members. Redmine also lets a project member with the permission do it.
- Permissions cover issue actions only. Redmine's roles also govern queries, notes, watchers, relations and more, and can narrow
  which issues a role sees. Here, what a person sees is decided by the plan's privacy alone.
- No group of people can hold a role; each person is added.
- Field rules cover the listed fields. Redmine's also cover custom fields.
- Only an administrator chooses a plan's types. Redmine lets a project member with the permission do it. A type defines no fields of its own.
- A plan's own issue workflow is registered through the API only.
- A task workflow cannot be changed in place, only withdrawn and registered again.
- `severity` is not configurable.
- Task workflows do not require an administrator (above). That gap predates this work and needs the owner's decision.
- Migrating down fails while an issue carries a custom type or status, by design.
