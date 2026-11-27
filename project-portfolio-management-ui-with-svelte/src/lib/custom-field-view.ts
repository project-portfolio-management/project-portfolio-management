// The rules of the custom fields page (`/custom-fields`, RM-1; NC-1 slice 12 part n), kept out of the component so
// they are unit-tested: what a declaration sends and every problem with it (the service's `def_problems`, checked
// before sending), how a search value is written for each type, and which sentence a refusal is.

/** The value types, in the order the picker offers them (`custom_fields::TYPES`). */
export const FIELD_TYPES = [
  "text",
  "date",
  "bool",
  "integer",
  "list",
  "multi_list",
] as const;
export type FieldType = (typeof FIELD_TYPES)[number];
/** What a field may apply to. */
export const APPLIES_TO = ["plan", "task"] as const;
export type AppliesTo = (typeof APPLIES_TO)[number];
/** The service's caps: a key, a label and an option are at most 100 characters; a list has at most 100 options. */
export const MAX_SHORT = 100;
export const MAX_OPTIONS = 100;

/** A definition as the form holds it. `kinds` is comma-separated, `options` one per line. */
export interface DefForm {
  key: string;
  label: string;
  field_type: FieldType;
  applies_to: AppliesTo;
  kinds: string;
  required: boolean;
  options: string;
}

export const EMPTY_DEF: DefForm = {
  key: "",
  label: "",
  field_type: "text",
  applies_to: "plan",
  kinds: "",
  required: false,
  options: "",
};

/** `POST /api/custom-fields`. */
export interface DefBody {
  key: string;
  label: string;
  type: string;
  applies_to: string;
  kinds: string[];
  required: boolean;
  options: string[];
}

export const isList = (t: string) => t === "list" || t === "multi_list";
const chars = (s: string) => [...s].length;
const split = (s: string, by: RegExp) =>
  s
    .split(by)
    .map((x) => x.trim())
    .filter((x) => x !== "");

/** A key is a lowercase letter, then lowercase letters, digits and underscores, at most 100 characters. */
export function validKey(key: string): boolean {
  return /^[a-z][a-z0-9_]*$/.test(key) && key.length <= MAX_SHORT;
}

/**
 * The body a declaration sends, or every problem with it as catalogue keys (all of them, as the service reports
 * them). Kinds are sent only for a plan field and options only for a list one, so a choice changed after typing
 * them does not send what no longer applies.
 */
export function defBody(
  f: DefForm,
): { body: DefBody } | { problems: string[] } {
  const problems: string[] = [];
  const key = f.key.trim();
  const label = f.label.trim();
  const kinds = f.applies_to === "plan" ? split(f.kinds, /,/) : [];
  const options = isList(f.field_type) ? split(f.options, /\n/) : [];
  if (!validKey(key)) problems.push("p.custom_fields.key_bad");
  if (label === "" || chars(label) > MAX_SHORT)
    problems.push("p.custom_fields.label_bad");
  if (isList(f.field_type)) {
    if (options.length === 0) problems.push("p.custom_fields.options_none");
    else if (
      options.length > MAX_OPTIONS ||
      options.some((o) => chars(o) > MAX_SHORT)
    )
      problems.push("p.custom_fields.options_long");
    else if (new Set(options).size !== options.length)
      problems.push("p.custom_fields.options_twice");
  }
  if (problems.length > 0) return { problems };
  return {
    body: {
      key,
      label,
      type: f.field_type,
      applies_to: f.applies_to,
      kinds,
      required: f.required,
      options,
    },
  };
}

/** Whether a live definition already holds this key for the subject type (the service's 409, said first). */
export function keyTaken(
  key: string,
  applies: string,
  defs: readonly { key: string; applies_to: string }[],
): boolean {
  return defs.some((d) => d.key === key.trim() && d.applies_to === applies);
}

/**
 * The search value as the query sends it, or the key of the problem: a whole number for an integer field, a real
 * day for a date, `true`/`false` for a yes-or-no one, any non-blank text otherwise.
 */
export function searchValue(
  type: string,
  raw: string,
): { value: string } | { problem: string } {
  const v = raw.trim();
  if (v === "") return { problem: "p.custom_fields.value_required" };
  if (type === "integer" && !/^-?\d+$/.test(v))
    return { problem: "p.custom_fields.value_integer" };
  if (type === "date" && !realDay(v))
    return { problem: "p.custom_fields.value_date" };
  if (type === "bool" && v !== "true" && v !== "false")
    return { problem: "p.custom_fields.value_required" };
  return { value: v };
}

/** Whether `YYYY-MM-DD` names a day on the calendar. */
export function realDay(s: string): boolean {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(s);
  if (!m) return false;
  const [y, mo, d] = [Number(m[1]), Number(m[2]), Number(m[3])];
  const at = new Date(Date.UTC(y, mo - 1, d));
  return (
    at.getUTCFullYear() === y &&
    at.getUTCMonth() === mo - 1 &&
    at.getUTCDate() === d
  );
}

/** A refused declaration: 409 is a key taken meanwhile; anything else the plain sentence with the service's reason. */
export function declareRefusalKey(status: number): string {
  return status === 409
    ? "p.custom_fields.key_taken"
    : "p.custom_fields.could_not_declare";
}

// ---- one plan's values (`/plans/[pid]/fields`) ----

/** The service's cap on a text value, in characters (`custom_fields::MAX_TEXT`). */
export const MAX_TEXT = 1000;

/** What the definition says about a value; the page's own subset of `CustomFieldDef`. */
export interface ValueDef {
  key: string;
  label: string;
  field_type: string;
  required: boolean;
  options: string[];
  kinds: string[];
}

/** A stored value as `GET /api/custom-fields/values` answers it. */
export interface StoredValue {
  value: unknown;
  label: string | null;
  definition: "live" | "withdrawn";
}

/** A value as its input holds it: a string for every type, a list for a multi-choice one. */
export interface ValueInput {
  text: string;
  many: string[];
}

/** Whether a definition limited to `kinds` applies to a plan of this kind (none: every kind; case ignored). */
export function kindApplies(
  kinds: readonly string[],
  planKind: string | null | undefined,
): boolean {
  if (kinds.length === 0) return true;
  const k = (planKind ?? "").toLowerCase();
  return k !== "" && kinds.some((x) => x.toLowerCase() === k);
}

/** The input for a stored value (or none). */
export function inputOf(def: ValueDef, stored: unknown): ValueInput {
  if (stored === null || stored === undefined) return { text: "", many: [] };
  if (def.field_type === "multi_list")
    return { text: "", many: Array.isArray(stored) ? stored.map(String) : [] };
  return { text: String(stored), many: [] };
}

/** The JSON value an input sends (`null` clears), or the key of the problem with it. */
export function valueOf(
  def: ValueDef,
  input: ValueInput,
): { value: unknown } | { problem: string } {
  const v = input.text.trim();
  let value: unknown;
  switch (def.field_type) {
    case "multi_list":
      value =
        input.many.length === 0
          ? null
          : def.options.filter((o) => input.many.includes(o));
      break;
    case "bool":
      value = v === "" ? null : v === "true";
      break;
    case "integer":
      if (v !== "" && (!/^-?\d+$/.test(v) || !Number.isSafeInteger(Number(v))))
        return { problem: "p.custom_fields.v_integer" };
      value = v === "" ? null : Number(v);
      break;
    case "date":
      if (v !== "" && !realDay(v)) return { problem: "p.custom_fields.v_date" };
      value = v === "" ? null : v;
      break;
    case "list":
      if (v !== "" && !def.options.includes(v))
        return { problem: "p.custom_fields.v_choice" };
      value = v === "" ? null : v;
      break;
    default:
      // Text keeps its own spacing; only a value of nothing but spaces is none.
      if ([...input.text].length > MAX_TEXT)
        return { problem: "p.custom_fields.v_too_long" };
      value = v === "" ? null : input.text;
  }
  if (value === null && def.required)
    return { problem: "p.custom_fields.v_required" };
  return { value };
}

const same = (a: unknown, b: unknown) =>
  JSON.stringify(a ?? null) === JSON.stringify(b ?? null);

/**
 * What a save sends: only the values that changed (a cleared one as `null`), or every problem, each with the label
 * of the field it is about. A required field never set and still empty is a problem too, so the page says it first.
 */
export function valueChanges(
  defs: readonly ValueDef[],
  inputs: Readonly<Record<string, ValueInput>>,
  stored: Readonly<Record<string, StoredValue>>,
):
  | { values: Record<string, unknown> }
  | { problems: { key: string; label: string }[] } {
  const values: Record<string, unknown> = {};
  const problems: { key: string; label: string }[] = [];
  for (const d of defs) {
    const r = valueOf(d, inputs[d.key] ?? { text: "", many: [] });
    if ("problem" in r) {
      problems.push({ key: r.problem, label: d.label });
      continue;
    }
    if (!same(r.value, stored[d.key]?.value)) values[d.key] = r.value;
  }
  return problems.length > 0 ? { problems } : { values };
}

/** How a stored value reads: yes or no for a flag, choices joined, anything else as written. */
export function shownValue(value: unknown, yes: string, no: string): string {
  if (typeof value === "boolean") return value ? yes : no;
  if (Array.isArray(value)) return value.join(", ");
  return String(value ?? "");
}
