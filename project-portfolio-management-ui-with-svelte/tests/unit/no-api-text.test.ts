import { describe, it, expect } from "vitest";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

// No UI page shows API error text (CQ-20): a route that reads `err.message`, `error.message` or a settled
// promise's `result.reason.message` would
// put the service's English on screen, or branch on it. Use `errorText` / `errorReason`, or test
// `err instanceof ApiError && err.status === ...`.

function files(dir: string): string[] {
  return readdirSync(dir).flatMap((name) => {
    const path = join(dir, name);
    return statSync(path).isDirectory() ? files(path) : [path];
  });
}

describe("no page shows API error text", () => {
  it("no route or component reads an error's message", () => {
    const offenders = [...files("src/routes"), ...files("src/lib/components")]
      .filter((f) => f.endsWith(".svelte"))
      .filter((f) =>
        /(\b(err|error|e|reason|cause)|\.reason)\.message\b/.test(
          readFileSync(f, "utf8"),
        ),
      );
    expect(offenders).toEqual([]);
  });
});
