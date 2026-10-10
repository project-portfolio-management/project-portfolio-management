// Translated constants that stay translated. A module-level `const X = ["Monday", ...]` is
// evaluated once, in whatever language was current when the module loaded. These helpers hold
// catalogue KEYS instead and look the text up each time it is read, so a value read in a
// template follows the language.

import { t, type StringKey } from "./i18n.svelte.js";

/** An array whose elements are translated when read. */
export function lazyArray(keys: readonly StringKey[]): readonly string[] {
  const out: string[] = new Array<string>(keys.length);
  keys.forEach((key, i) => {
    Object.defineProperty(out, i, { enumerable: true, get: () => t(key) });
  });
  return out;
}
