// The rules of the checklist templates page (`/onboarding-templates`; NC-1 slice 12 part n), kept out of the component
// so they are unit-tested: what a template sends and every problem with it (the service's `template_problems`: 1 to
// 50 items, each a title up to 200 characters and an offset of -365 to 365 days), and how an offset reads.

/** One item as the service holds it. */
export interface Seed {
  title: string;
  offset_days: number;
}

/** One item as its row in the form holds it. */
export interface SeedRow {
  title: string;
  offset: string;
}

export const MAX_ITEMS = 50;
export const TITLE_MAX = 200;
export const OFFSET_MAX = 365;

/** The rows for a stored checklist. */
export const rowsOf = (items: readonly Seed[]): SeedRow[] =>
  items.map((s) => ({ title: s.title, offset: String(s.offset_days) }));

/** A problem with a template: its catalogue key, and the 1-based item it is about when there is one. */
export interface TemplateProblem {
  key: string;
  n?: number;
}

/** The items a template sends, or every problem with it, as the service would report them. */
export function templateBody(
  rows: readonly SeedRow[],
): { items: Seed[] } | { problems: TemplateProblem[] } {
  const problems: TemplateProblem[] = [];
  if (rows.length === 0 || rows.length > MAX_ITEMS)
    problems.push({ key: "p.onboarding_templates.count_bad" });
  const items: Seed[] = [];
  rows.forEach((r, i) => {
    const title = r.title.trim();
    if (title === "" || [...title].length > TITLE_MAX)
      problems.push({ key: "p.onboarding_templates.title_bad", n: i + 1 });
    const o = r.offset.trim();
    const days = /^[-+]?\d+$/.test(o) ? Number(o) : Number.NaN;
    if (!Number.isInteger(days) || Math.abs(days) > OFFSET_MAX)
      problems.push({ key: "p.onboarding_templates.offset_bad", n: i + 1 });
    items.push({ title, offset_days: days });
  });
  return problems.length > 0 ? { problems } : { items };
}

/** Whether two checklists are the same, item for item, so an unchanged save sends nothing. */
export function sameItems(a: readonly Seed[], b: readonly Seed[]): boolean {
  return (
    a.length === b.length &&
    a.every(
      (s, i) => s.title === b[i]!.title && s.offset_days === b[i]!.offset_days,
    )
  );
}

/** How an item's offset reads: its catalogue key and the number of days, never negative. */
export function offsetKey(days: number): { key: string; n: number } {
  if (days < 0) return { key: "p.onboarding_templates.before", n: -days };
  if (days > 0) return { key: "p.onboarding_templates.after", n: days };
  return { key: "p.onboarding_templates.on_the_day", n: 0 };
}
