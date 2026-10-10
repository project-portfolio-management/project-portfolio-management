/**
 * Pure helpers for the RACI matrix view. No DOM, no Svelte: unit-tested.
 *
 * R Responsible (does the work), A Accountable (answers for it; **one per task**),
 * C Consulted (asked before), I Informed (told after). A person may hold several.
 * The cell state is a letters string in R-A-C-I order, like the service's.
 */

import type { RaciFinding } from "./api/ppm";

export type Letter = "R" | "A" | "C" | "I";
export const LETTERS: readonly Letter[] = ["R", "A", "C", "I"];

export const LETTER_WORD: Record<Letter, string> = {
  R: "Responsible",
  A: "Accountable",
  C: "Consulted",
  I: "Informed",
};

/** One-line meaning of each letter, for the legend. */
export const LETTER_MEANING: Record<Letter, string> = {
  R: "does the work",
  A: "answers for it and signs it off (one per task)",
  C: "is asked before",
  I: "is told after",
};

/** The letters of a cell, normalized to R-A-C-I order. */
export function normalize(letters: string): string {
  return LETTERS.filter((l) => letters.toUpperCase().includes(l)).join("");
}

/** A short name for a person reference: the part after the scheme, first eight characters. */
export function shortRef(ref: string): string {
  const id = ref.includes(":") ? ref.slice(ref.indexOf(":") + 1) : ref;
  return id.length > 8 ? `${id.slice(0, 8)}…` : id;
}

/** The result of toggling a letter. */
export interface Toggled {
  cells: Record<string, string>;
  /** Someone who lost Accountable because another person was just given it. */
  accountableMovedFrom: string | null;
}

/**
 * Toggle `letter` for `person` in a row. Giving someone **A** takes it from whoever
 * had it (a task has one Accountable person), and says so; a person left with no
 * letters drops out of the row.
 */
export function toggleLetter(
  cells: Readonly<Record<string, string>>,
  person: string,
  letter: Letter,
): Toggled {
  const next: Record<string, string> = { ...cells };
  const has = (next[person] ?? "").includes(letter);
  let movedFrom: string | null = null;
  if (has) {
    next[person] = normalize((next[person] ?? "").replace(letter, ""));
  } else {
    if (letter === "A") {
      for (const [p, l] of Object.entries(next)) {
        if (p !== person && l.includes("A")) {
          next[p] = normalize(l.replace("A", ""));
          movedFrom = p;
        }
      }
    }
    next[person] = normalize((next[person] ?? "") + letter);
  }
  for (const [p, l] of Object.entries(next)) if (l === "") delete next[p];
  return { cells: next, accountableMovedFrom: movedFrom };
}

/** The payload for the service: one entry per person who holds a letter. */
export function toAssignments(
  cells: Readonly<Record<string, string>>,
): { person_ref: string; roles: string }[] {
  return Object.entries(cells)
    .filter(([, l]) => l !== "")
    .map(([person_ref, roles]) => ({ person_ref, roles: normalize(roles) }));
}

/** What a row is missing, in words. */
export function findingText(f: RaciFinding): string {
  switch (f) {
    case "no_accountable":
      return "Nobody is accountable";
    case "no_responsible":
      return "Nobody is responsible";
    default:
      return "More than one person is accountable";
  }
}

/** A sentence for a cell, for assistive technology: "Design: ann is Responsible and Accountable". */
export function cellSentence(
  task: string,
  person: string,
  letters: string,
): string {
  const words = LETTERS.filter((l) => letters.includes(l)).map(
    (l) => LETTER_WORD[l],
  );
  const who = shortRef(person);
  return words.length === 0
    ? `${task}: ${who} has no role`
    : `${task}: ${who} is ${words.join(" and ")}`;
}

/** A person reference the page will accept as a new column. */
export function validPersonRef(ref: string): boolean {
  return /^(person|worker):[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/.test(
    ref.trim(),
  );
}
