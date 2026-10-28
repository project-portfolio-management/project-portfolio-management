// Cross-app bridges: links between this app and its two sibling apps, contact-relationship-management
// (CRM) and workforce-planning-management (WFP).
//
// A bridge is a plain link, with no runtime coupling: nothing here calls the other app, shares a
// session, or fails if the other app is down. A deployment names the other apps' base URLs
// (`CONTACTS_APP_URL`, `WORKFORCE_APP_URL`); with none set, no link is shown.
//
// Outbound, a reference this service stores becomes a link only where the kind says which app owns
// it: `worker:<id>` is a workforce worker (`/workers/<id>`), `contact:<id>` a CRM contact
// (`/contacts/<id>`). `person:<id>` is not linked: nothing says that id is the same in either app.
//
// Inbound, the other apps may link to the routes in `INBOUND_ROUTES`; `tests/unit/bridge.test.ts`
// fails if one stops existing.

export interface Bridges {
  /** Base URL of contact-relationship-management, or `null`. */
  contacts: string | null;
  /** Base URL of workforce-planning-management, or `null`. */
  workforce: string | null;
}

export type BridgeKind = "contacts" | "workforce";

/** Routes in this app that the sibling apps may link to; `{pid}` is a plan's id. Part of the contract. */
export const INBOUND_ROUTES = [
  "/plans",
  "/plans/{pid}",
  "/plans/{pid}/work",
  "/plans/{pid}/raci",
  "/me",
] as const;

const NONE: Bridges = { contacts: null, workforce: null };

/** A configured base URL: `http(s)` only, no trailing slash, `null` when blank or not a web URL. */
export function baseUrl(raw: string | undefined | null): string | null {
  const value = (raw ?? "").trim();
  if (!value) return null;
  try {
    const url = new URL(value);
    if (url.protocol !== "https:" && url.protocol !== "http:") return null;
    if (url.username || url.password) return null;
    return url.origin + url.pathname.replace(/\/+$/, "");
  } catch {
    return null;
  }
}

const UUID = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

/** The sibling app that owns `ref`, and where its page for it is, or `null` when no bridge applies. */
export function bridgeLink(
  ref: string,
  bridges: Bridges = NONE,
): { kind: BridgeKind; href: string } | null {
  const [prefix, id] = ref.split(":", 2);
  if (!id || !UUID.test(id)) return null;
  if (prefix === "worker" && bridges.workforce) {
    return { kind: "workforce", href: `${bridges.workforce}/workers/${id}` };
  }
  if (prefix === "contact" && bridges.contacts) {
    return { kind: "contacts", href: `${bridges.contacts}/contacts/${id}` };
  }
  return null;
}
