import { describe, it, expect } from "vitest";
import { ApiError } from "../../src/lib/api/client";
import { errorReason, errorText } from "../../src/lib/error-text";
import { i18n, t } from "../../src/lib/i18n.svelte";

describe("errorText (CQ-20)", () => {
  const fallback = t("ppm.common.loadFailed");

  it("keeps the service's reason for a refusal it explains", () => {
    expect(
      errorText(
        new ApiError(422, "title must not be blank"),
        "ppm.common.loadFailed",
      ),
    ).toBe("title must not be blank");
    expect(
      errorText(new ApiError(409, "already decided"), "ppm.common.loadFailed"),
    ).toBe("already decided");
  });

  it("shows the page's own message for a server fault, whose text says nothing useful", () => {
    expect(
      errorText(
        new ApiError(500, "Internal Server Error"),
        "ppm.common.loadFailed",
      ),
    ).toBe(fallback);
    expect(
      errorText(new ApiError(503, "unavailable"), "ppm.common.loadFailed"),
    ).toBe(fallback);
  });

  it("falls back for anything that is not an error, or an error with no message", () => {
    expect(errorText("boom", "ppm.common.loadFailed")).toBe(fallback);
    expect(errorText(undefined, "ppm.common.loadFailed")).toBe(fallback);
    expect(errorText(new Error(""), "ppm.common.loadFailed")).toBe(fallback);
    expect(errorText(new Error("network down"), "ppm.common.loadFailed")).toBe(
      "network down",
    );
  });

  describe("in another language", () => {
    const welshConflict = () => {
      i18n.set("cy-001");
      return t("p.error.status_409");
    };

    it("puts the catalogue's Welsh for a 409 first, and the service's English reason after it", () => {
      const generic = welshConflict();
      try {
        expect(generic).not.toMatch(/^p\.error\./);
        expect(generic).not.toBe(
          "That conflicts with the current state: someone else changed or decided it, or it is already done.",
        );
        expect(
          errorText(
            new ApiError(409, "already decided"),
            "ppm.common.loadFailed",
          ),
        ).toBe(`${generic} (already decided)`);
        expect(errorReason(new ApiError(409, "already decided"))).toBe(
          `${generic} (already decided)`,
        );
      } finally {
        i18n.set("en-001");
      }
    });

    it("shows the localized server-fault sentence for a 5xx, and no English detail", () => {
      i18n.set("cy-001");
      try {
        expect(errorReason(new ApiError(500, "Internal Server Error"))).toBe(
          t("p.error.server"),
        );
        expect(
          errorReason(new ApiError(500, "Internal Server Error")),
        ).not.toContain("Internal");
      } finally {
        i18n.set("en-001");
      }
    });

    it("falls back to a general sentence for a status with no sentence of its own", () => {
      i18n.set("cy-001");
      try {
        expect(errorReason(new ApiError(418, ""))).toBe(t("p.error.client"));
      } finally {
        i18n.set("en-001");
      }
    });
  });

  it("gives the service's own reason in English", () => {
    i18n.set("en-001");
    expect(errorReason(new ApiError(409, "already decided"))).toBe(
      "already decided",
    );
    expect(errorReason(new Error("network down"))).toBe("network down");
    expect(errorReason("boom")).toBe("");
  });
});
