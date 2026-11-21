import { describe, expect, it } from "vitest";
import {
  migrateThemeChoice,
  OLD_CHOICE_KEY,
  PICKER_KEY,
  type ChoiceStore,
} from "#lib/theme-choice.js";

function store(initial: Record<string, string> = {}): ChoiceStore & {
  data: Record<string, string>;
} {
  const data = { ...initial };
  return {
    data,
    getItem: (k) => (k in data ? data[k]! : null),
    setItem: (k, v) => void (data[k] = v),
    removeItem: (k) => void delete data[k],
  };
}

describe("migrateThemeChoice", () => {
  it("does nothing for a visitor who never chose", () => {
    const s = store();
    migrateThemeChoice(s);
    expect(s.data).toEqual({});
  });

  it("moves a theme the person chose under the old UI key to the picker's key", () => {
    const s = store({ [OLD_CHOICE_KEY]: "cupcake" });
    migrateThemeChoice(s);
    expect(s.data).toEqual({ [PICKER_KEY]: "cupcake" });
  });

  it("lets the old UI key win over a guess the picker kept", () => {
    const s = store({ [OLD_CHOICE_KEY]: "cupcake", [PICKER_KEY]: "dark" });
    migrateThemeChoice(s);
    expect(s.data).toEqual({ [PICKER_KEY]: "cupcake" });
  });

  it.each(["light", "dark"])(
    "drops %s under the picker's key, a guess from the browser",
    (guess) => {
      const s = store({ [PICKER_KEY]: guess });
      migrateThemeChoice(s);
      expect(s.data).toEqual({});
    },
  );

  it.each(["cupcake", "system"])("keeps %s under the picker's key", (kept) => {
    const s = store({ [PICKER_KEY]: kept });
    migrateThemeChoice(s);
    expect(s.data).toEqual({ [PICKER_KEY]: kept });
  });

  it("survives a store that refuses access", () => {
    const refusing: ChoiceStore = {
      getItem: () => {
        throw new Error("denied");
      },
      setItem: () => {},
      removeItem: () => {},
    };
    expect(() => migrateThemeChoice(refusing)).not.toThrow();
  });
});
