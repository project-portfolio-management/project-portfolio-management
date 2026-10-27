import { describe, it, expect } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import { resource } from "../../src/lib/resource.svelte";
import { t } from "../../src/lib/i18n.svelte";

/** A promise settled by hand, so a test chooses the order in which loads finish. */
function deferred<T>() {
  let resolve!: (v: T) => void;
  let reject!: (e: unknown) => void;
  const promise = new Promise<T>((res, rej) => {
    resolve = res;
    reject = rej;
  });
  return { promise, resolve, reject };
}

describe("resource (CQ-20)", () => {
  it("starts empty, then holds the loaded value", async () => {
    const r = resource(async () => 7, "ppm.common.loadFailed");
    expect(r.data).toBeNull();
    expect(r.error).toBeNull();
    expect(r.loading).toBe(false);
    const done = r.reload();
    expect(r.loading).toBe(true);
    await done;
    expect(r.data).toBe(7);
    expect(r.loading).toBe(false);
  });

  it("shows the service's reason for a refusal and the page's message for a server fault", async () => {
    let err: unknown = new ApiError(422, "title must not be blank");
    const r = resource(async () => {
      throw err;
    }, "ppm.common.loadFailed");
    await r.reload();
    expect(r.error).toBe("title must not be blank");
    err = new ApiError(500, "Internal Server Error");
    await r.reload();
    expect(r.error).toBe(t("ppm.common.loadFailed"));
    expect(r.data).toBeNull();
  });

  it("keeps the last good value when a reload fails, and clears the error when one works", async () => {
    let fail = false;
    const r = resource(async () => {
      if (fail) throw new ApiError(404, "gone");
      return "kept";
    }, "ppm.common.loadFailed");
    await r.reload();
    fail = true;
    await r.reload();
    expect(r.data).toBe("kept");
    expect(r.error).toBe("gone");
    fail = false;
    await r.reload();
    expect(r.error).toBeNull();
  });

  it("lets only the latest call set the state when loads overlap", async () => {
    const first = deferred<string>();
    const second = deferred<string>();
    const queue = [first, second];
    const r = resource(() => queue.shift()!.promise, "ppm.common.loadFailed");
    const a = r.reload();
    const b = r.reload();
    second.resolve("new");
    await b;
    expect(r.data).toBe("new");
    first.resolve("stale");
    await a;
    expect(r.data).toBe("new");
    expect(r.loading).toBe(false);
  });

  it("ignores an old failure that arrives after a newer success", async () => {
    const first = deferred<string>();
    const second = deferred<string>();
    const queue = [first, second];
    const r = resource(() => queue.shift()!.promise, "ppm.common.loadFailed");
    const a = r.reload();
    const b = r.reload();
    second.resolve("fresh");
    await b;
    first.reject(new ApiError(500, "late"));
    await a;
    expect(r.error).toBeNull();
    expect(r.data).toBe("fresh");
  });
});
