import { describe, expect, it } from "vitest";
import {
  componentBody,
  leadOk,
  nameTaken,
  refusalKey,
} from "../../src/lib/component-view";

describe("component-view (PL-11)", () => {
  it("takes a person: or worker: reference as a lead, nothing else", () => {
    expect(leadOk("person:3f2c")).toBe(true);
    expect(leadOk(" worker:x ")).toBe(true);
    expect(leadOk("Ana")).toBe(false);
    expect(leadOk("person:")).toBe(false);
  });
  it("trims, checks every limit before sending, and clears a blank lead", () => {
    expect(componentBody(" API ", "")).toEqual({
      body: { name: "API", lead_ref: null },
    });
    expect(componentBody("API", " person:1 ")).toEqual({
      body: { name: "API", lead_ref: "person:1" },
    });
    expect(componentBody("  ", "")).toEqual({
      problem: "p.components.name_required",
    });
    expect(componentBody("x".repeat(121), "")).toEqual({
      problem: "p.components.name_too_long",
    });
    expect(componentBody("API", "Ana")).toEqual({
      problem: "p.components.lead_bad",
    });
  });
  it("finds a taken name ignoring case, but not the component's own", () => {
    const others = [{ pid: "a", name: "API" }];
    expect(nameTaken("api", others)).toBe(true);
    expect(nameTaken("api", others, "a")).toBe(false);
    expect(nameTaken("UI", others)).toBe(false);
  });
  it("has a sentence of its own for each refusal", () => {
    expect(refusalKey("save", 409)).toBe("p.components.taken");
    expect(refusalKey("save", 422)).toBe("p.components.too_many");
    expect(refusalKey("remove", 409)).toBe("p.components.in_use_refused");
    expect(refusalKey("remove", 500)).toBe("p.components.could_not_remove");
  });
});
