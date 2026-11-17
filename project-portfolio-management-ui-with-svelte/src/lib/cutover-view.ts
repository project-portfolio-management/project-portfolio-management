// Cutover stop criteria (EX-6, NC-1 slice 11): the service's rules, said before anything is sent. A decision has 1 to
// 30 stop criteria, each 5 to 500 characters after trimming; they can be changed only while the decision is a draft.

/** The fewest and most stop criteria a decision may have, and each criterion's shortest and longest length. */
export const CRITERIA_MIN = 1;
export const CRITERIA_MAX = 30;
export const CRITERION_MIN = 5;
export const CRITERION_MAX = 500;

/** One criterion per line, trimmed, blank lines dropped, as the service will keep them. */
export function parseCriteria(text: string): string[] {
  return text
    .split("\n")
    .map((l) => l.trim())
    .filter(Boolean);
}

/** The catalogue key of the first rule `criteria` breaks, with the line it breaks it on, or `null`. */
export function criteriaProblem(
  criteria: string[],
):
  | { key: "p.cutover.problem_count" }
  | { key: "p.cutover.problem_length"; line: number }
  | null {
  if (criteria.length < CRITERIA_MIN || criteria.length > CRITERIA_MAX)
    return { key: "p.cutover.problem_count" };
  const i = criteria.findIndex(
    (c) => [...c].length < CRITERION_MIN || [...c].length > CRITERION_MAX,
  );
  return i >= 0 ? { key: "p.cutover.problem_length", line: i + 1 } : null;
}

/** Whether `next` is the same list, in the same order, as `current`. */
export function sameCriteria(current: string[], next: string[]): boolean {
  return (
    current.length === next.length && current.every((c, i) => c === next[i])
  );
}
