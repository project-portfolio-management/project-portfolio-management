import { describe, it, expect } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import { errorText } from "../../src/lib/error-text";
import { t } from "../../src/lib/i18n.svelte";

describe("errorText (CQ-20)", () => {
  const fallback = t("ppm.common.loadFailed");

  it("keeps the service's reason for a refusal it explains", () => {
    expect(errorText(new ApiError(422, "title must not be blank"), "ppm.common.loadFailed")).toBe("title must not be blank");
    expect(errorText(new ApiError(409, "already decided"), "ppm.common.loadFailed")).toBe("already decided");
  });

  it("shows the page's own message for a server fault, whose text says nothing useful", () => {
    expect(errorText(new ApiError(500, "Internal Server Error"), "ppm.common.loadFailed")).toBe(fallback);
    expect(errorText(new ApiError(503, "unavailable"), "ppm.common.loadFailed")).toBe(fallback);
  });

  it("falls back for anything that is not an error, or an error with no message", () => {
    expect(errorText("boom", "ppm.common.loadFailed")).toBe(fallback);
    expect(errorText(undefined, "ppm.common.loadFailed")).toBe(fallback);
    expect(errorText(new Error(""), "ppm.common.loadFailed")).toBe(fallback);
    expect(errorText(new Error("network down"), "ppm.common.loadFailed")).toBe("network down");
  });
});
