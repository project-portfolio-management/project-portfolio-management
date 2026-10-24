import { describe, expect, it } from "vitest";
import { answerConfirm, askConfirm, confirmState } from "../../src/lib/confirm.svelte";

describe("askConfirm", () => {
  it("resolves true on OK and clears the question", async () => {
    const p = askConfirm("Delete it?");
    expect(confirmState.pending?.message).toBe("Delete it?");
    answerConfirm(true);
    expect(await p).toBe(true);
    expect(confirmState.pending).toBeNull();
  });
  it("resolves false on Cancel", async () => {
    const p = askConfirm("Delete it?");
    answerConfirm(false);
    expect(await p).toBe(false);
  });
  it("answers an earlier question no when a second one opens, leaving none hanging", async () => {
    const first = askConfirm("one");
    const second = askConfirm("two");
    expect(await first).toBe(false);
    expect(confirmState.pending?.message).toBe("two");
    answerConfirm(true);
    expect(await second).toBe(true);
  });
  it("ignores an answer when nothing is open", () => {
    expect(() => answerConfirm(true)).not.toThrow();
  });
});
