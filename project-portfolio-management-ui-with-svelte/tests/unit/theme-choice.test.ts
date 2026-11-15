import { describe, expect, it } from "vitest";
import {
  CHOICE_KEY,
  LEGACY_KEY,
  nextChoice,
  readChoice,
  systemTheme,
  writeChoice,
  type ChoiceStore,
} from "../../src/lib/theme-choice";

function store(
  initial: Record<string, string> = {},
): ChoiceStore & { data: Record<string, string> } {
  const data = { ...initial };
  return {
    data,
    getItem: (k) => (k in data ? data[k] : null),
    setItem: (k, v) => {
      data[k] = v;
    },
    removeItem: (k) => {
      delete data[k];
    },
  };
}

describe("the UI follows the browser's light/dark setting unless the person picked a theme", () => {
  it("maps the browser's setting to light or dark", () => {
    expect(systemTheme(true)).toBe("dark");
    expect(systemTheme(false)).toBe("light");
  });

  it("has no choice on a first visit", () => {
    expect(readChoice(store())).toBe("");
  });

  it("drops a legacy light or dark, which was the picker's own guess", () => {
    for (const guessed of ["light", "dark"]) {
      const s = store({ [LEGACY_KEY]: guessed });
      expect(readChoice(s)).toBe("");
      expect(s.data).toEqual({});
    }
  });

  it("keeps a legacy theme that was picked, once, under the new key", () => {
    const s = store({ [LEGACY_KEY]: "cupcake" });
    expect(readChoice(s)).toBe("cupcake");
    expect(s.data).toEqual({ [CHOICE_KEY]: "cupcake" });
  });

  it("does not let a legacy value overwrite a choice already made", () => {
    const s = store({ [LEGACY_KEY]: "cupcake", [CHOICE_KEY]: "nord" });
    expect(readChoice(s)).toBe("nord");
  });

  it("keeps a theme other than the browser's, and forgets the browser's own", () => {
    expect(nextChoice("cupcake", "light")).toBe("cupcake");
    expect(nextChoice("dark", "light")).toBe("dark");
    expect(nextChoice("light", "light")).toBe("");
    const s = store();
    writeChoice(s, "cupcake");
    expect(s.data).toEqual({ [CHOICE_KEY]: "cupcake" });
    writeChoice(s, "");
    expect(s.data).toEqual({});
  });

  it("survives storage that throws", () => {
    const broken: ChoiceStore = {
      getItem: () => {
        throw new Error("blocked");
      },
      setItem: () => {
        throw new Error("blocked");
      },
      removeItem: () => {
        throw new Error("blocked");
      },
    };
    expect(readChoice(broken)).toBe("");
    expect(() => writeChoice(broken, "cupcake")).not.toThrow();
  });
});
