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
