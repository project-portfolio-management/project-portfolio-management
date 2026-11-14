// RE-13: the rules for a form whose session ends, or that is left with changes not saved.
import { describe, expect, it } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import { isSessionEnded, shouldAskBeforeLeaving } from "../../src/lib/unsaved";

describe("unsaved work (RE-13)", () => {
  it("a 401 on save is an ended session, nothing else is", () => {
    expect(isSessionEnded(new ApiError(401, "HTTP 401", undefined))).toBe(true);
    expect(isSessionEnded(new ApiError(403, "HTTP 403", undefined))).toBe(
      false,
    );
    expect(isSessionEnded(new ApiError(422, "bad", undefined))).toBe(false);
    expect(isSessionEnded(new Error("network"))).toBe(false);
  });

  it("asks before leaving only with changes not yet saved", () => {
    expect(shouldAskBeforeLeaving(false, false)).toBe(false);
    expect(shouldAskBeforeLeaving(true, false)).toBe(true);
    expect(shouldAskBeforeLeaving(true, true)).toBe(false);
  });
});
