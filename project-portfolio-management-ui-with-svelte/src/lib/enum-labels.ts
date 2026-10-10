// Display labels for the closed sets of values the service sends as bare words (a task status,
// a plan kind, a proposal status, ...). The wire value is never shown to a person directly: it
// is looked up in the page catalogue under `l.enum.<group>.<value>`, so it renders in the
// reader's language. A value the catalogue does not know (a custom status, say) is shown as is.

import { t } from "./i18n.svelte.js";
import { PAGE_KEYS } from "./i18n-pages";

export type EnumGroup =
  | "task_status"
  | "kind"
  | "plan_status"
  | "collection"
  | "proposal_status"
  | "band";

/** The label for `value` in `group`, in the current language. */
export function enumLabel(group: EnumGroup, value: unknown): string {
  const text = typeof value === "string" ? value : String(value);
  const key = `l.enum.${group}.${text}`;
  return PAGE_KEYS.has(key) ? t(key as never) : text;
}
