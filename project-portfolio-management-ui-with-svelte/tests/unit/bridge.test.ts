import { describe, it, expect } from "vitest";
import { existsSync } from "node:fs";
import { INBOUND_ROUTES, baseUrl, bridgeLink } from "../../src/lib/bridge";

const ID = "6f1f3c2e-8a4b-4c5d-9e0f-1a2b3c4d5e6f";
const both = {
  contacts: "https://contacts.example.org",
  workforce: "https://workforce.example.org/app",
};

describe("cross-app bridges", () => {
  it("links a worker to workforce planning and a contact to contacts", () => {
    expect(bridgeLink(`worker:${ID}`, both)).toEqual({
      kind: "workforce",
      href: `https://workforce.example.org/app/workers/${ID}`,
    });
    expect(bridgeLink(`contact:${ID}`, both)).toEqual({
      kind: "contacts",
      href: `https://contacts.example.org/contacts/${ID}`,
    });
  });

  it("shows no link when the app is not configured, or the reference is not owned by one", () => {
    expect(bridgeLink(`worker:${ID}`)).toBeNull();
    expect(
      bridgeLink(`worker:${ID}`, { contacts: both.contacts, workforce: null }),
    ).toBeNull();
    // nothing says a person's id is the same in either app
    expect(bridgeLink(`person:${ID}`, both)).toBeNull();
    expect(bridgeLink("worker:not-an-id", both)).toBeNull();
    expect(bridgeLink("", both)).toBeNull();
    expect(bridgeLink(`worker:${ID}/../../x`, both)).toBeNull();
  });

  it("accepts only an http(s) base URL, without credentials, and strips a trailing slash", () => {
    expect(baseUrl("https://a.example.org/")).toBe("https://a.example.org");
    expect(baseUrl(" http://localhost:5174 ")).toBe("http://localhost:5174");
    expect(baseUrl("javascript:alert(1)")).toBeNull();
    expect(baseUrl("ftp://a.example.org")).toBeNull();
    expect(baseUrl("https://user:pw@a.example.org")).toBeNull();
    expect(baseUrl("not a url")).toBeNull();
    expect(baseUrl("")).toBeNull();
    expect(baseUrl(undefined)).toBeNull();
  });

  it("keeps every route the other apps may link to", () => {
    for (const route of INBOUND_ROUTES) {
      const dir = route
        .replace("{pid}", "[pid]")
        .split("/")
        .filter(Boolean)
        .join("/");
      expect(
        existsSync(`src/routes/${dir}/+page.svelte`),
        `${route} has no page`,
      ).toBe(true);
    }
  });
});
