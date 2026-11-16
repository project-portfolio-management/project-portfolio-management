// The rules for what the site keeps, and the cookie store (src/lib/choices.ts), without a browser. Run with `node --test`.
import { test } from "node:test";
import assert from "node:assert/strict";
import {
  CHOICE_KEY,
  COOKIE_MAX_AGE,
  LEGACY_KEY,
  cookieStore,
  memoryStore,
  nextChoice,
  STORED_KEYS,
  TIME_ZONE_KEY,
  choiceStore,
  keepPicked,
  readChoice,
  readValue,
  writeChoice,
  type CookieJar,
} from "../../src/lib/choices.ts";

/** A jar that keeps what was written the way a browser would: the last value per name, a Max-Age of 0 removing it. */
function jar(): CookieJar & { written: string[] } {
  const live = new Map<string, string>();
  const written: string[] = [];
  return {
    written,
    get cookie() {
      return [...live].map(([k, v]) => `${k}=${v}`).join("; ");
    },
    set cookie(line: string) {
      written.push(line);
      const [pair] = line.split(";");
      const at = pair!.indexOf("=");
      const name = pair!.slice(0, at);
      if (/Max-Age=0(;|$)/.test(line)) live.delete(name);
      else live.set(name, pair!.slice(at + 1));
    },
  };
}

test("a pick other than the browser's theme is kept; the browser's own theme forgets it", () => {
  assert.equal(nextChoice("nord", "light"), "nord");
  assert.equal(nextChoice("dark", "dark"), "");
});

test("the old key: light or dark was the picker's guess and is dropped, any other theme moves across", () => {
  const a = memoryStore();
  a.setItem(LEGACY_KEY, "dark");
  assert.equal(readChoice(a), "");
  assert.equal(a.getItem(LEGACY_KEY), null);
  const b = memoryStore();
  b.setItem(LEGACY_KEY, "nord");
  assert.equal(readChoice(b), "nord");
  assert.equal(b.getItem(CHOICE_KEY), "nord");
});

test("the cookie store writes one first-party cookie with a path, a year and SameSite, Secure on HTTPS", () => {
  const j = jar();
  const store = cookieStore(j, true);
  writeChoice(store, "nord");
  assert.equal(j.written.at(-1), `${CHOICE_KEY}=nord; Max-Age=${COOKIE_MAX_AGE}; Path=/; SameSite=Lax; Secure`);
  assert.equal(readChoice(store), "nord");
  writeChoice(store, "");
  assert.match(j.written.at(-1)!, /^ppm-theme-choice=; Max-Age=0; Path=\/; SameSite=Lax; Secure$/);
  assert.equal(readChoice(store), "");
  writeChoice(cookieStore(j, false), "nord");
  assert.doesNotMatch(j.written.at(-1)!, /Secure/);
});

test("the cookie store reads its own name among others and survives a value it cannot decode", () => {
  const j = jar();
  j.cookie = "other=1";
  j.cookie = "ppm-theme-choice-x=no";
  j.cookie = `${CHOICE_KEY}=night%20owl`;
  assert.equal(cookieStore(j, false).getItem(CHOICE_KEY), "night owl");
  j.cookie = `${CHOICE_KEY}=%E0%A4%A`;
  assert.equal(cookieStore(j, false).getItem(CHOICE_KEY), null);
});

test("localStorage is the default store; with cookie, each choice in localStorage moves into a cookie once", () => {
  const local = memoryStore();
  assert.equal(choiceStore("localStorage", local, jar(), false), local);

  local.setItem(LEGACY_KEY, "nord");
  local.setItem(TIME_ZONE_KEY, "Asia/Tokyo");
  const j = jar();
  const store = choiceStore("cookie", local, j, false);
  assert.equal(readChoice(store), "nord");
  assert.equal(readValue(store, TIME_ZONE_KEY), "Asia/Tokyo");
  for (const key of [...STORED_KEYS, LEGACY_KEY]) assert.equal(local.getItem(key), null, key);

  // A cookie already there wins over localStorage.
  local.setItem(CHOICE_KEY, "cupcake");
  assert.equal(readChoice(choiceStore("cookie", local, j, false)), "nord");
});

test("a picker's first report as it mounts is its own start, not a choice; later ones are the reader's", () => {
  const store = memoryStore();
  const seen: string[] = [];
  const onChange = keepPicked(store, TIME_ZONE_KEY, (v) => seen.push(v));
  onChange("Europe/London");
  assert.equal(store.getItem(TIME_ZONE_KEY), null);
  onChange("Asia/Tokyo");
  assert.equal(store.getItem(TIME_ZONE_KEY), "Asia/Tokyo");
  assert.deepEqual(seen, ["Europe/London", "Asia/Tokyo"]);
});
