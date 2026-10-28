import { describe, it, expect } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import { errorReason, errorText } from "../../src/lib/error-text";
import { i18n, t } from "../../src/lib/i18n.svelte";

describe("errorText (CQ-20): no page shows the service's text", () => {
  const fallback = t("ppm.common.loadFailed");

  it("shows the catalogue's sentence for a refusal, never the service's reason", () => {
    i18n.set("en-001");
    expect(
      errorText(
        new ApiError(422, "title must not be blank"),
        "ppm.common.loadFailed",
      ),
    ).toBe(t("p.error.status_422"));
    expect(errorReason(new ApiError(409, "already decided"))).toBe(
      t("p.error.status_409"),
    );
  });

  it("shows the page's own message for a server fault and for anything that is not an API refusal", () => {
    expect(
      errorText(
        new ApiError(500, "Internal Server Error"),
        "ppm.common.loadFailed",
      ),
    ).toBe(fallback);
    expect(errorText("boom", "ppm.common.loadFailed")).toBe(fallback);
    expect(errorText(undefined, "ppm.common.loadFailed")).toBe(fallback);
    expect(errorText(new Error("network down"), "ppm.common.loadFailed")).toBe(
      fallback,
    );
    expect(errorReason(new Error("network down"))).toBe("");
  });

  it("says a general sentence for a status with no sentence of its own", () => {
    expect(errorReason(new ApiError(418, "teapot"))).toBe(t("p.error.client"));
  });

  it("is in the reader's language and carries no English from the service", () => {
    i18n.set("cy-001");
    try {
      const reason = errorReason(new ApiError(409, "already decided"));
      expect(reason).toBe(t("p.error.status_409"));
      expect(reason).not.toMatch(/^p\.error\./);
      expect(reason).not.toContain("already decided");
      expect(errorReason(new ApiError(500, "Internal Server Error"))).toBe(
        t("p.error.server"),
      );
    } finally {
      i18n.set("en-001");
    }
  });
});
