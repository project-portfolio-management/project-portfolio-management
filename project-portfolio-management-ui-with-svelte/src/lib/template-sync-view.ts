/**
 * Rules behind a plan's template changes (`/plans/[pid]/template`) and the template page's plans and revision
 * (NC-12; NC-1 slice 12 part k). Pure; unit-tested. The service's English reasons are never shown.
 */

import { t, tDynamic } from "./i18n.svelte.js";
import { formatInteger } from "./locale-format.js";
import type {
  TemplateChange,
  TemplateSyncBody,
  TemplateSynced,
  TemplateTask,
} from "./api/ppm";

const SYMBOL = { add: "+", change: "~", remove: "−" } as const;

/** A change's kind as a word and a symbol. */
export function kindOf(c: TemplateChange): { symbol: string; text: string } {
  return c.kind in SYMBOL
    ? {
        symbol: SYMBOL[c.kind],
        text: tDynamic(`p.template_sync.kind_${c.kind}`),
      }
    : { symbol: "?", text: t("p.template_sync.kind_unknown") };
}

/** What a changed task's description and points were and become, only the parts that differ. */
export function differences(c: TemplateChange): string[] {
  const out: string[] = [];
  const a: Partial<TemplateTask> = c.from ?? {};
  const b: Partial<TemplateTask> = c.to ?? {};
  const none = t("p.template_sync.none_value");
  if ((a.title ?? "") !== (b.title ?? "") && c.kind === "change")
    out.push(
      t("p.template_sync.diff_title", {
        from: a.title || none,
        to: b.title || none,
      }),
    );
  if ((a.description ?? "") !== (b.description ?? "") && c.kind === "change")
    out.push(
      t("p.template_sync.diff_description", {
        from: a.description || none,
        to: b.description || none,
      }),
    );
  if ((a.points ?? null) !== (b.points ?? null) && c.kind === "change")
    out.push(
      t("p.template_sync.diff_points", {
        from: a.points == null ? none : formatInteger(a.points),
        to: b.points == null ? none : formatInteger(b.points),
      }),
    );
  return out;
}

/** The body for the ticked changes: an overwrite is sent only for a ticked change the plan edited. */
export function syncBody(
  changes: readonly TemplateChange[],
  accepted: ReadonlySet<string>,
  overwrite: ReadonlySet<string>,
): TemplateSyncBody | null {
  const accept = changes.filter((c) => accepted.has(c.id)).map((c) => c.id);
  if (accept.length === 0) return null;
  const overwrite_local = changes
    .filter(
      (c) => accepted.has(c.id) && c.locally_edited && overwrite.has(c.id),
    )
    .map((c) => c.id);
  return overwrite_local.length > 0 ? { accept, overwrite_local } : { accept };
}

/** Why each refused change was refused, in the reader's words, from what the page knew when it sent them. */
export function refusalText(
  r: TemplateSynced["refused"][number],
  changes: readonly TemplateChange[],
  overwrite: ReadonlySet<string>,
): string {
  const c = changes.find((x) => x.id === r.id);
  const title = c?.title ?? r.id;
  return c?.locally_edited && !overwrite.has(r.id)
    ? t("p.template_sync.refused_edited", { title })
    : t("p.template_sync.refused_removed", { title });
}

/** The catalogue key for a refused revision of a template. */
export function reviseRefusalKey(status: number | undefined): string {
  return status === 422
    ? "p.template_sync.revise_shape"
    : "p.template_sync.could_not_revise";
}
