// The rules of a plan's access page (`/plans/[pid]/access`, RM-8 and PL-7; NC-1 slice 12 part m), kept out of the
// component so they are unit-tested: a private plan's members (user ids, at most 200; the caller is added by the
// service when making a plan private), what a change sends (the whole setting, only when it changed), a guest grant
// (a user id, read or comment, an optional end that must be in the future), and each grant's state.

export const MEMBERS_MAX = 200;
export const GUEST_ACCESS = ["read", "comment"] as const;
export type GuestAccess = (typeof GUEST_ACCESS)[number];

/** A plan's access setting as the service holds it. */
export interface PlanAccess {
  plan_pid?: string;
  private: boolean;
  members: string[];
}

/** What `PUT /api/plans/{pid}/access` takes. */
export interface AccessBody {
  private: boolean;
  members: string[];
}

/** One guest grant, live or not. */
export interface GuestGrant {
  pid: string;
  guest_sub: string;
  access: string;
  expires_at: string | null;
  granted_by: string | null;
  revoked_at: string | null;
  live: boolean;
}

/** What `POST /api/plans/{pid}/guests` takes. */
export interface GrantBody {
  guest_sub: string;
  access: GuestAccess;
  expires_at?: string;
}

const UUID = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

/** Whether `s` is a user id (a UUID). */
export const isUserId = (s: string): boolean => UUID.test(s.trim());

/** The members as their text area holds them, one to a line. */
export const membersText = (members: readonly string[]): string =>
  members.join("\n");

/** The members a text area says (a line or a comma each, blanks dropped, repeats once, lower case), or its problems. */
export function parseMembers(
  text: string,
): { members: string[] } | { problems: string[] } {
  const out: string[] = [];
  const problems = new Set<string>();
  for (const raw of text.split(/[\n,]/)) {
    const m = raw.trim().toLowerCase();
    if (m === "") continue;
    if (!isUserId(m)) problems.add("p.plan_access.member_bad");
    else if (!out.includes(m)) out.push(m);
  }
  if (out.length > MEMBERS_MAX) problems.add("p.plan_access.members_many");
  return problems.size > 0 ? { problems: [...problems] } : { members: out };
}

/** What saving the setting sends: the whole of it, or `null` when neither the privacy nor the members changed. */
export function accessChange(
  isPrivate: boolean,
  membersInput: string,
  now: PlanAccess,
): { body: AccessBody } | { problems: string[] } | null {
  const m = parseMembers(membersInput);
  if ("problems" in m) return m;
  const same =
    isPrivate === now.private &&
    JSON.stringify([...m.members].sort()) ===
      JSON.stringify([...now.members].map((x) => x.toLowerCase()).sort());
  return same ? null : { body: { private: isPrivate, members: m.members } };
}

/**
 * What granting a guest sends, or its problems: a user id, read or comment, and an end (`datetime-local`, read in
 * the browser's zone) after `now`, or none.
 */
export function grantChange(
  guest: string,
  access: string,
  expires: string,
  now: Date,
): { body: GrantBody } | { problems: string[] } {
  const problems: string[] = [];
  if (!isUserId(guest)) problems.push("p.plan_access.guest_bad");
  if (!(GUEST_ACCESS as readonly string[]).includes(access))
    problems.push("p.plan_access.access_bad");
  let at: Date | null = null;
  if (expires.trim() !== "") {
    at = new Date(expires);
    if (Number.isNaN(at.getTime())) problems.push("p.plan_access.expires_bad");
    else if (at.getTime() <= now.getTime())
      problems.push("p.plan_access.expires_past");
  }
  if (problems.length > 0) return { problems };
  const body: GrantBody = {
    guest_sub: guest.trim().toLowerCase(),
    access: access as GuestAccess,
  };
  if (at) body.expires_at = at.toISOString();
  return { body };
}

/** A grant's state: live, revoked, or past its end. */
export const grantState = (g: GuestGrant): "live" | "revoked" | "expired" =>
  g.live ? "live" : g.revoked_at ? "revoked" : "expired";

/** Two ends in order, none last (ISO instants compare as text). */
const byEnd = (a: string | null, b: string | null): number =>
  a === b ? 0 : a === null ? 1 : b === null ? -1 : a < b ? -1 : 1;

/** The grants, live first, then by end (none last). */
export function sortedGrants(grants: readonly GuestGrant[]): GuestGrant[] {
  return [...grants].sort(
    (a, b) =>
      Number(b.live) - Number(a.live) || byEnd(a.expires_at, b.expires_at),
  );
}
