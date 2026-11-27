// The rules of a product plan's public roadmap page (`/plans/[pid]/roadmap`, RE-1, RE-3; NC-1 slice 12 part l), kept out of
// the component so they are unit-tested: the service's limits (a title 1 to 200 characters, a vision, a description
// or an outcome up to 2,000, a phase up to 100, at most 20 tags of 1 to 50 characters with no comma and no repeat,
// a slug of lower-case letters, digits and single hyphens up to 64, at most 50 detail lines of up to 300 characters at
// levels 0 to 2, at most 50 organizations), what a change sends (only what changed), how
// the items fall into their placements, and where a move puts one.

/** Every placement, in page order (`public_roadmap::PLACEMENTS`); an item with none is unplaced, shown last. */
export const PLACEMENTS = [
  "recently_delivered",
  "now",
  "next",
  "later",
  "delivered_this_year",
  "not_doing",
] as const;
export type Placement = (typeof PLACEMENTS)[number];

export const TITLE_MAX = 200;
export const TEXT_MAX = 2000;
export const PHASE_MAX = 100;
export const TAGS_MAX = 20;
export const TAG_MAX = 50;
export const SLUG_MAX = 64;
export const DETAILS_MAX = 50;
export const DETAIL_MAX = 300;
export const DETAIL_LEVEL_MAX = 2;
export const ORGANIZATIONS_MAX = 50;

/** One line of an item's details; level 0 is the top, at most one deeper than the line above, at most 2. */
export interface Detail {
  text: string;
  level: number;
}

/** The details as their text area holds them: a line each, indented two spaces a level. */
export const detailsText = (details: readonly Detail[]): string =>
  details.map((d) => `${"  ".repeat(d.level)}${d.text}`).join("\n");

/**
 * The details a text area says (a tab is two spaces; two spaces make a level; blank lines dropped), with every
 * problem as catalogue keys: too many lines, a line too long, a line more than one level deeper than the one above or
 * deeper than 2.
 */
export function parseDetails(text: string): {
  details: Detail[];
  problems: string[];
} {
  const problems = new Set<string>();
  const details: Detail[] = [];
  let above: number | null = null;
  for (const raw of text.replaceAll("\t", "  ").split("\n")) {
    const t = raw.trim();
    if (t === "") continue;
    const level = Math.floor((raw.length - raw.trimStart().length) / 2);
    const deepest = above === null ? 0 : Math.min(above + 1, DETAIL_LEVEL_MAX);
    if ([...t].length > DETAIL_MAX)
      problems.add("p.product_roadmap.detail_long");
    if (level > deepest) problems.add("p.product_roadmap.detail_deep");
    details.push({ text: t, level });
    above = level;
  }
  if (details.length > DETAILS_MAX)
    problems.add("p.product_roadmap.details_many");
  return { details, problems: [...problems] };
}

const chars = (s: string) => [...s.trim()].length;

/** Whether `s` is a slug the service takes. */
export function isSlug(s: string): boolean {
  return (
    s.length > 0 &&
    s.length <= SLUG_MAX &&
    /^[a-z0-9](?:[a-z0-9-]*[a-z0-9])?$/.test(s) &&
    !s.includes("--")
  );
}

/** The tags typed as one comma-separated line, trimmed, blanks dropped. */
export const tagsOf = (line: string): string[] =>
  line
    .split(",")
    .map((t) => t.trim())
    .filter((t) => t !== "");

/** Every problem with a list of tags, as catalogue keys. */
function tagProblems(tags: readonly string[], out: string[]) {
  if (tags.length > TAGS_MAX) out.push("p.product_roadmap.tags_many");
  const seen = new Set<string>();
  for (const t of tags) {
    if (chars(t) > TAG_MAX) out.push("p.product_roadmap.tag_long");
    else if (seen.has(t.toLowerCase())) out.push("p.product_roadmap.tag_twice");
    seen.add(t.toLowerCase());
  }
}

/** The roadmap's own fields as its form holds them; `category` is a category's id, or "" for none. */
export interface RoadmapForm {
  title: string;
  slug: string;
  vision: string;
  service_description: string;
  category: string;
}

/** What `PUT /api/plans/{pid}/roadmap` takes: an absent field is kept, `""` clears a text, `null` clears the category. */
export interface RoadmapBody {
  title?: string;
  slug?: string;
  vision?: string;
  service_description?: string;
  category_pid?: string | null;
}

/** What the service holds of them. */
export interface RoadmapFields {
  title: string;
  slug: string;
  vision: string | null;
  service_description: string | null;
  category_pid?: string | null;
}

export const roadmapFormOf = (r: RoadmapFields | null): RoadmapForm => ({
  title: r?.title ?? "",
  slug: r?.slug ?? "",
  vision: r?.vision ?? "",
  service_description: r?.service_description ?? "",
  category: r?.category_pid ?? "",
});

/**
 * What a change to the roadmap's own fields sends (`PUT /api/plans/{pid}/roadmap`): only what changed, an emptied text
 * as `""` (which clears it); for a new roadmap the title, and the slug only when one is typed (the service makes one
 * from the title). Every problem first, as catalogue keys; `null` when nothing changed.
 */
export function roadmapChange(
  f: RoadmapForm,
  r: RoadmapFields | null,
): { body: RoadmapBody } | { problems: string[] } | null {
  const problems: string[] = [];
  const title = f.title.trim();
  const slug = f.slug.trim();
  if (title === "" || chars(title) > TITLE_MAX)
    problems.push("p.product_roadmap.title_bad");
  if ((slug !== "" || r) && !isSlug(slug))
    problems.push("p.product_roadmap.slug_bad");
  if (chars(f.vision) > TEXT_MAX || chars(f.service_description) > TEXT_MAX)
    problems.push("p.product_roadmap.text_long");
  if (problems.length > 0) return { problems };
  const body: RoadmapBody = {};
  if (!r) {
    body.title = title;
    if (slug !== "") body.slug = slug;
    if (f.vision.trim() !== "") body.vision = f.vision.trim();
    if (f.service_description.trim() !== "")
      body.service_description = f.service_description.trim();
    if (f.category !== "") body.category_pid = f.category;
    return { body };
  }
  if (title !== r.title) body.title = title;
  if (slug !== r.slug) body.slug = slug;
  if (f.vision.trim() !== (r.vision ?? "")) body.vision = f.vision.trim();
  if (f.service_description.trim() !== (r.service_description ?? ""))
    body.service_description = f.service_description.trim();
  if (f.category !== (r.category_pid ?? ""))
    body.category_pid = f.category === "" ? null : f.category;
  return Object.keys(body).length === 0 ? null : { body };
}

/** One item, as the page reads it. */
export interface RoadmapItem {
  pid: string;
  public_id: string;
  title: string;
  description: string | null;
  expected_outcome: string | null;
  phase: string | null;
  tags: string[];
  /** RE-7: the organizations it is for, by id. */
  organization_pids?: string[];
  details?: Detail[];
  placement: Placement | null;
  position: number;
  /** RE-6: by language, each translated field's value and whether its source changed since. */
  translations?: ShownTranslations;
}

/** An item as its form holds it; the tags as one comma-separated line. */
export interface ItemForm {
  title: string;
  description: string;
  expected_outcome: string;
  phase: string;
  tags: string;
  /** The details, a line each, indented two spaces a level. */
  details: string;
  /** The organizations it is for, by id. */
  organizations: string[];
}

export const itemFormOf = (i: RoadmapItem | null): ItemForm => ({
  title: i?.title ?? "",
  description: i?.description ?? "",
  expected_outcome: i?.expected_outcome ?? "",
  phase: i?.phase ?? "",
  tags: (i?.tags ?? []).join(", "),
  details: detailsText(i?.details ?? []),
  organizations: [...(i?.organization_pids ?? [])],
});

export interface ItemBody {
  title?: string;
  description?: string;
  expected_outcome?: string;
  phase?: string;
  tags?: string[];
  details?: Detail[];
  organization_pids?: string[];
  placement?: Placement;
}

/**
 * What adding an item (`i` null) or changing one sends: for a change only what changed (an emptied text as `""`, the
 * tags as the whole list), for an add what is filled in, and the placement. Every problem first; `null` when a
 * change changes nothing.
 */
export function itemChange(
  f: ItemForm,
  i: RoadmapItem | null,
  placement?: Placement | null,
): { body: ItemBody } | { problems: string[] } | null {
  const problems: string[] = [];
  const title = f.title.trim();
  if (title === "" || chars(title) > TITLE_MAX)
    problems.push("p.product_roadmap.title_bad");
  if (chars(f.description) > TEXT_MAX || chars(f.expected_outcome) > TEXT_MAX)
    problems.push("p.product_roadmap.text_long");
  if (chars(f.phase) > PHASE_MAX) problems.push("p.product_roadmap.phase_long");
  const tags = tagsOf(f.tags);
  tagProblems(tags, problems);
  const d = parseDetails(f.details ?? "");
  problems.push(...d.problems);
  const orgs = [...new Set(f.organizations ?? [])];
  if (orgs.length > ORGANIZATIONS_MAX)
    problems.push("p.product_roadmap.orgs_many");
  if (problems.length > 0) return { problems: [...new Set(problems)] };
  const body: ItemBody = {};
  const texts = ["description", "expected_outcome", "phase"] as const;
  if (!i) {
    body.title = title;
    for (const k of texts) if (f[k].trim() !== "") body[k] = f[k].trim();
    if (tags.length > 0) body.tags = tags;
    if (d.details.length > 0) body.details = d.details;
    if (orgs.length > 0) body.organization_pids = orgs;
    if (placement) body.placement = placement;
    return { body };
  }
  if (title !== i.title) body.title = title;
  for (const k of texts)
    if (f[k].trim() !== (i[k] ?? "")) body[k] = f[k].trim();
  if (JSON.stringify(tags) !== JSON.stringify(i.tags)) body.tags = tags;
  if (JSON.stringify(d.details) !== JSON.stringify(i.details ?? []))
    body.details = d.details;
  const had = [...(i.organization_pids ?? [])].sort();
  if (JSON.stringify([...orgs].sort()) !== JSON.stringify(had))
    body.organization_pids = orgs;
  return Object.keys(body).length === 0 ? null : { body };
}

/** The items in page order, grouped: each placement that has items, then the unplaced. */
export function groups(
  items: readonly RoadmapItem[],
): { placement: Placement | null; items: RoadmapItem[] }[] {
  const order: (Placement | null)[] = [...PLACEMENTS, null];
  return order
    .map((placement) => ({
      placement,
      items: items
        .filter((i) => (i.placement ?? null) === placement)
        .sort((a, b) => a.position - b.position),
    }))
    .filter((g) => g.items.length > 0);
}

/** The move that takes an item one place up or down within its placement, or `null` at the end it cannot pass. */
export function step(
  items: readonly RoadmapItem[],
  item: RoadmapItem,
  by: -1 | 1,
): { placement: Placement | null; index: number } | null {
  const same = items
    .filter((i) => (i.placement ?? null) === (item.placement ?? null))
    .sort((a, b) => a.position - b.position);
  const at = same.findIndex((i) => i.pid === item.pid);
  const to = at + by;
  if (at < 0 || to < 0 || to >= same.length) return null;
  return { placement: item.placement ?? null, index: to };
}

// ---- publishing as one change set (`POST …/roadmap/publish/preview`, then `…/publish`; RE-2) ----

/** One field of a change set: what the editor first saw, and what it wants. */
export interface FieldChange {
  base: unknown;
  value: unknown;
}

/** A change set: the roadmap's own fields, existing items' fields, and new items. */
export interface ChangeSet {
  roadmap?: Record<string, FieldChange>;
  items?: { pid: string; fields: Record<string, FieldChange> }[];
  new?: ItemBody[];
}

/** One line of a preview or of what was applied. */
export interface ChangeLine {
  subject: "roadmap" | "item" | "new_item";
  field?: string | null;
  item?: string;
  from?: unknown;
  to?: unknown;
  title?: string | null;
}

/** One conflict: someone else changed the field to something else, or the item is gone. */
export interface ConflictLine {
  subject: "roadmap" | "item";
  field?: string | null;
  item?: string;
  base?: unknown;
  value?: unknown;
  current?: unknown;
  gone?: boolean;
}

/** The change set that publishes the roadmap (`true`) or takes it off the public reader, from what was read. */
export const publishSet = (
  publishedNow: boolean,
  publish: boolean,
): ChangeSet => ({
  roadmap: { published: { base: publishedNow, value: publish } },
});

/**
 * A field's value in words for a preview line: yes or no for `published`, a placement by name, a list joined, a text as
 * written, and `null` (or an empty list) as none.
 */
export function valueWords(
  field: string | null | undefined,
  v: unknown,
  words: {
    yes: string;
    no: string;
    none: string;
    placement: (p: string) => string;
  },
): string {
  if (v === null || v === undefined || (Array.isArray(v) && v.length === 0))
    return words.none;
  if (field === "published") return v ? words.yes : words.no;
  if (field === "placement" && typeof v === "string") return words.placement(v);
  if (Array.isArray(v))
    return v
      .map((x) =>
        typeof x === "object" && x && "text" in x ? String(x.text) : String(x),
      )
      .join(", ");
  return String(v);
}

/** The catalogue key naming a field of the roadmap or of an item. */
export const fieldKey = (field: string | null | undefined) =>
  `p.product_roadmap.field.${field ?? "item"}`;

// ---- translations (`PUT …/roadmap/translations/{lang}` and `…/items/{ipid}/translations/{lang}`, RE-6) ----

/** A record's translations as the service shows them: language, then field, then its value and whether its source moved. */
export type ShownTranslations = Record<
  string,
  Record<string, { value: unknown; needs_review: boolean }>
>;

/** The roadmap's translatable fields, and an item's (its details are not offered here). */
export const ROADMAP_TRANSLATABLE = [
  "title",
  "vision",
  "service_description",
] as const;
export const ITEM_TRANSLATABLE = [
  "title",
  "description",
  "expected_outcome",
  "phase",
  "tags",
] as const;

const MAXES: Record<string, number> = { title: TITLE_MAX, phase: PHASE_MAX };

/** The text a field's translation input holds: tags as one comma-separated line. */
export function translationInput(value: unknown): string {
  if (Array.isArray(value)) return value.map(String).join(", ");
  return value === null || value === undefined ? "" : String(value);
}

/** A record's fields by name, for reading its translatable sources without a cast. */
export function fieldsOf(record: object): Record<string, unknown> {
  return Object.fromEntries(Object.entries(record));
}

/**
 * What a translation into one language sends: each changed field (a field with no source text is never sent), an
 * emptied one as `""`, which removes that translation; tags as a list as long as the source's. Every problem first as
 * catalogue keys (with the field); `null` when nothing changed.
 */
export function translationChange(
  fields: readonly string[],
  sources: Record<string, unknown>,
  shown: Record<string, { value: unknown }> | undefined,
  inputs: Record<string, string>,
):
  | { body: Record<string, unknown> }
  | { problems: { key: string; field: string }[] }
  | null {
  const body: Record<string, unknown> = {};
  const problems: { key: string; field: string }[] = [];
  for (const f of fields) {
    const source = sources[f];
    if (
      source === null ||
      source === undefined ||
      (Array.isArray(source) && source.length === 0)
    )
      continue;
    const typed = (inputs[f] ?? "").trim();
    const before = translationInput(shown?.[f]?.value).trim();
    if (typed === before) continue;
    if (typed === "") {
      body[f] = "";
      continue;
    }
    if (f === "tags") {
      const tags = tagsOf(typed);
      if (tags.length !== (source as unknown[]).length)
        problems.push({ key: "p.product_roadmap.tr_tags_count", field: f });
      else if (tags.some((t) => [...t].length > TAG_MAX))
        problems.push({ key: "p.product_roadmap.tr_tag_long", field: f });
      else body[f] = tags;
      continue;
    }
    if ([...typed].length > (MAXES[f] ?? TEXT_MAX))
      problems.push({ key: "p.product_roadmap.tr_too_long", field: f });
    else body[f] = typed;
  }
  if (problems.length > 0) return { problems };
  return Object.keys(body).length === 0 ? null : { body };
}

/** Every language a record and its items are translated into, sorted, and how many of their fields need review. */
export function translationSummary(
  records: readonly (ShownTranslations | undefined)[],
): { lang: string; review: number }[] {
  const out = new Map<string, number>();
  for (const r of records)
    for (const [lang, fields] of Object.entries(r ?? {}))
      out.set(
        lang,
        (out.get(lang) ?? 0) +
          Object.values(fields).filter((f) => f.needs_review).length,
      );
  return [...out.entries()]
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([lang, review]) => ({ lang, review }));
}
