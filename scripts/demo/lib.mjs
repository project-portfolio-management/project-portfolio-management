// Helpers for the demonstration seed: a tiny API client that fails loudly, and "ensure" helpers so a
// second run finds what is there and adds only what is missing.
//
// Everything seeded is FICTITIOUS: the Northgate Transport Authority and every person, project, amount and
// date below were made up for the demonstration. Do not run this against a real deployment.

export const API = (process.env.PPM_API_URL ?? "http://127.0.0.1:5150").replace(/\/+$/, "");

export class ApiFailure extends Error {
  constructor(method, path, status, body) {
    super(`${method} ${path} -> ${status} ${typeof body === "string" ? body : JSON.stringify(body)}`);
    this.status = status;
  }
}

async function call(method, path, body) {
  const res = await fetch(`${API}${path}`, {
    method,
    headers: { "content-type": "application/json", "accepts-version": "1.0" },
    body: body === undefined ? undefined : JSON.stringify(body),
  });
  const text = await res.text();
  let data;
  try { data = text ? JSON.parse(text) : null; } catch { data = text; }
  if (!res.ok) throw new ApiFailure(method, path, res.status, data);
  return data;
}

export const get = (path) => call("GET", path);
export const post = (path, body = {}) => call("POST", path, body);
export const put = (path, body = {}) => call("PUT", path, body);
export const patch = (path, body = {}) => call("PATCH", path, body);

/** Like `post`, but a refusal is logged and returns null: for steps the API may refuse the second time. */
export async function tryPost(path, body = {}, what = path) {
  try { return await post(path, body); } catch (e) {
    if (e instanceof ApiFailure && e.status >= 400 && e.status < 500) { log(`  (skipped ${what}: ${e.status} ${JSON.stringify(e.message.split("-> ")[1] ?? "").slice(0, 230)})`); return null; }
    throw e;
  }
}

export const log = (...a) => console.log(...a);

/** The first item of `items` for which `match` holds, else the result of `make()`. */
export async function ensure(items, match, make) {
  const found = items.find(match);
  return found ?? (await make());
}

export const day = (offset, from = new Date("2026-10-12T00:00:00Z")) =>
  new Date(from.getTime() + offset * 86_400_000).toISOString().slice(0, 10);
