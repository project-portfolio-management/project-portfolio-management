// Frameworks (ways of working, governance) and plan links: the choices, checks and summaries the screens use.
// Pure and tested. The service is the authority; these checks only save a round trip.

import type {
  Framework,
  FrameworkFamily,
  FrameworkInForce,
  PlanLink,
} from "#lib/api/ppm.js";
import { t } from "./i18n.svelte.js";

export const METHODOLOGIES = [
  "scrum",
  "kanban",
  "scrumban",
  "waterfall",
  "safe",
  "xp",
  "lean",
  "hybrid",
  "other",
] as const;
export const DECISION_MODELS = [
  "sponsor",
  "steering_committee",
  "product_owner",
  "delegated",
  "consensus",
  "other",
] as const;
export const LINK_KINDS = [
  "docs",
  "repo",
  "ticket",
  "design",
  "chat",
  "tracker",
  "other",
] as const;
export const LINK_SYSTEMS = [
  "jira",
  "trello",
  "asana",
  "linear",
  "monday",
  "clickup",
  "azure_devops",
  "github",
  "other",
] as const;

/** The domains of hosted systems; a link naming one must be on it (the service enforces the same). */
const HOSTED: Record<string, string[]> = {
  trello: ["trello.com"],
  asana: ["asana.com"],
  linear: ["linear.app"],
  monday: ["monday.com"],
  clickup: ["clickup.com"],
  azure_devops: ["dev.azure.com", "visualstudio.com"],
};

export const NAME_MAX = 120;
export const MAX_DAYS = 366;

/** The API path segment and body key of each family. */
export const FAMILY: Record<
  FrameworkFamily,
  { key: "ways_of_working" | "governance_framework" }
> = {
  "ways-of-working": { key: "ways_of_working" },
  "governance-frameworks": { key: "governance_framework" },
};

/** The framework a response puts in force, if any. */
export function recordOf(
  family: FrameworkFamily,
  r: FrameworkInForce | null,
): Framework | null {
  if (!r) return null;
  return (
    (family === "ways-of-working"
      ? r.ways_of_working
      : r.governance_framework) ?? null
  );
}

/** "Set on this plan", "Inherited from a plan above" or "None": in words. */
export function sourceText(r: Pick<FrameworkInForce, "source">): string {
  if (r.source === "own") return t("p.frameworks.source_own");
  if (r.source === "inherited") return t("p.frameworks.source_inherited");
  return t("p.frameworks.source_none");
}

/** A number input binds a number (or nothing), not a string, so every check goes through a string first. */
const text = (raw: unknown): string => String(raw ?? "").trim();
const wholeDays = (raw: unknown, max: number): boolean =>
  /^\d+$/.test(text(raw)) && Number(raw) >= 1 && Number(raw) <= max;

export interface FrameworkForm {
  name: string;
  description: string;
  /** Ways of working. */
  methodology: string;
  cadence_days: string;
  wip_limit: string;
  /** Governance. */
  decision_model: string;
  review_cadence_days: string;
  reporting_cadence_days: string;
  /** Escalation path rows. */
  path: { level: string; body: string; trigger: string }[];
  /** Ways of working: the definitions and the recurring practices. */
  ready: string;
  done: string;
  ceremonies: { name: string; cadence: string; purpose: string }[];
  /** Governance: who decides what, and the standing meetings. */
  rights: { decision: string; decides: string; consulted: string }[];
  forums: { name: string; cadence: string; purpose: string }[];
}

/** Most entries in any one list (the service's rule). */
export const LIST_MAX = 20;
const TEXT_MAX = 10_000;

/** What is wrong with a form, in words; empty when it can be sent. */
export function frameworkProblems(
  family: FrameworkFamily,
  f: FrameworkForm,
): string[] {
  const out: string[] = [];
  const name = f.name.trim();
  if (name.length < 1 || name.length > NAME_MAX)
    out.push(t("p.frameworks.name_problem"));
  if (f.description.length > TEXT_MAX) out.push(t("p.frameworks.text_problem"));
  const optional = (raw: unknown, max: number) =>
    text(raw) === "" || wholeDays(raw, max);
  if (family === "ways-of-working") {
    if (!optional(f.cadence_days, MAX_DAYS) || !optional(f.wip_limit, 1000))
      out.push(t("p.frameworks.number_problem"));
    if (f.ready.length > TEXT_MAX || f.done.length > TEXT_MAX)
      out.push(t("p.frameworks.text_problem"));
    if (
      f.ceremonies.length > LIST_MAX ||
      f.ceremonies.some((c) => c.name.trim() === "" || c.cadence.trim() === "")
    )
      out.push(t("p.frameworks.practice_problem"));
  } else {
    if (
      !optional(f.review_cadence_days, MAX_DAYS) ||
      !optional(f.reporting_cadence_days, MAX_DAYS)
    )
      out.push(t("p.frameworks.number_problem"));
    if (
      f.forums.length > LIST_MAX ||
      f.forums.some((x) => x.name.trim() === "" || x.cadence.trim() === "")
    )
      out.push(t("p.frameworks.practice_problem"));
    if (
      f.rights.length > LIST_MAX ||
      f.rights.some((r) => r.decision.trim() === "" || r.decides.trim() === "")
    )
      out.push(t("p.frameworks.rights_problem"));
    const levels = new Set<string>();
    for (const row of f.path) {
      if (
        !wholeDays(row.level, 9) ||
        row.body.trim() === "" ||
        row.trigger.trim() === "" ||
        levels.has(text(row.level))
      ) {
        out.push(t("p.frameworks.path_problem"));
        break;
      }
      levels.add(text(row.level));
    }
  }
  return out;
}

const num = (raw: unknown): number | null =>
  text(raw) === "" ? null : Number(raw);

/** The body to send. */
export function frameworkBody(
  family: FrameworkFamily,
  f: FrameworkForm,
  _existing: Framework | null,
  planPid: string | null,
): Record<string, unknown> {
  const body: Record<string, unknown> = {
    name: f.name.trim(),
    description: f.description.trim() || null,
  };
  if (planPid) body.plan_pid = planPid;
  if (family === "ways-of-working") {
    Object.assign(body, {
      methodology: f.methodology,
      cadence_days: num(f.cadence_days),
      wip_limit: num(f.wip_limit),
      definition_of_ready: f.ready.trim() || null,
      definition_of_done: f.done.trim() || null,
      ceremonies: f.ceremonies.map((c) => ({
        name: c.name.trim(),
        cadence: c.cadence.trim(),
        purpose: c.purpose.trim() || null,
      })),
    });
  } else {
    Object.assign(body, {
      decision_model: f.decision_model,
      review_cadence_days: num(f.review_cadence_days),
      reporting_cadence_days: num(f.reporting_cadence_days),
      escalation_path: f.path.map((r) => ({
        level: Number(r.level),
        body: r.body.trim(),
        trigger: r.trigger.trim(),
      })),
      decision_rights: f.rights.map((r) => ({
        decision: r.decision.trim(),
        decides: r.decides.trim(),
        consulted: r.consulted.trim() || null,
      })),
      forums: f.forums.map((x) => ({
        name: x.name.trim(),
        cadence: x.cadence.trim(),
        purpose: x.purpose.trim() || null,
      })),
    });
  }
  return body;
}

/** A form filled from an existing record, or an empty one. */
export function formOf(f: Framework | null): FrameworkForm {
  const s = (v: number | null | undefined) =>
    v === null || v === undefined ? "" : String(v);
  return {
    name: f?.name ?? "",
    description: f?.description ?? "",
    methodology: f?.methodology ?? "other",
    cadence_days: s(f?.cadence_days),
    wip_limit: s(f?.wip_limit),
    decision_model: f?.decision_model ?? "other",
    review_cadence_days: s(f?.review_cadence_days),
    reporting_cadence_days: s(f?.reporting_cadence_days),
    path: (f?.escalation_path ?? []).map((e) => ({
      level: String(e.level),
      body: e.body,
      trigger: e.trigger,
    })),
    ready: f?.definition_of_ready ?? "",
    done: f?.definition_of_done ?? "",
    ceremonies: (f?.ceremonies ?? []).map((c) => ({
      name: c.name,
      cadence: c.cadence,
      purpose: c.purpose ?? "",
    })),
    rights: (f?.decision_rights ?? []).map((r) => ({
      decision: r.decision,
      decides: r.decides,
      consulted: r.consulted ?? "",
    })),
    forums: (f?.forums ?? []).map((x) => ({
      name: x.name,
      cadence: x.cadence,
      purpose: x.purpose ?? "",
    })),
  };
}

/** Label and value pairs describing a record, for reading. Empty values are left out. */
export function summary(
  family: FrameworkFamily,
  f: Framework,
): [string, string][] {
  const rows: [string, string | undefined][] =
    family === "ways-of-working"
      ? [
          [
            t("p.frameworks.methodology"),
            f.methodology ? methodologyLabel(f.methodology) : undefined,
          ],
          [
            t("p.frameworks.cadence_days"),
            f.cadence_days
              ? t("p.frameworks.days", { n: f.cadence_days })
              : undefined,
          ],
          [
            t("p.frameworks.wip_limit"),
            f.wip_limit ? String(f.wip_limit) : undefined,
          ],
          [t("p.frameworks.ready"), f.definition_of_ready ?? undefined],
          [t("p.frameworks.done"), f.definition_of_done ?? undefined],
          [
            t("p.frameworks.ceremonies"),
            f.ceremonies?.map((c) => `${c.name} (${c.cadence})`).join("; "),
          ],
        ]
      : [
          [
            t("p.frameworks.decision_model"),
            f.decision_model ? decisionModelLabel(f.decision_model) : undefined,
          ],
          [
            t("p.frameworks.review_cadence"),
            f.review_cadence_days
              ? t("p.frameworks.days", { n: f.review_cadence_days })
              : undefined,
          ],
          [
            t("p.frameworks.reporting_cadence"),
            f.reporting_cadence_days
              ? t("p.frameworks.days", { n: f.reporting_cadence_days })
              : undefined,
          ],
          [
            t("p.frameworks.path"),
            [...(f.escalation_path ?? [])]
              .sort((a, b) => a.level - b.level)
              .map((e) => `${e.level}: ${e.body}`)
              .join("; "),
          ],
          [
            t("p.frameworks.rights"),
            f.decision_rights
              ?.map((d) => `${d.decision} → ${d.decides}`)
              .join("; "),
          ],
          [
            t("p.frameworks.forums"),
            f.forums?.map((x) => `${x.name} (${x.cadence})`).join("; "),
          ],
        ];
  return rows.filter((r): r is [string, string] => !!r[1]);
}

export interface LinkForm {
  kind: string;
  url: string;
  label: string;
  system: string;
  external_id: string;
}

const hostOf = (url: string): string => {
  try {
    return new URL(url).hostname.toLowerCase();
  } catch {
    return "";
  }
};

/** What is wrong with a link form, in words; empty when it can be sent. */
export function linkProblems(f: LinkForm): string[] {
  const out: string[] = [];
  const url = f.url.trim();
  if (!/^https?:\/\/[^\s/@]+/i.test(url)) out.push(t("p.links.url_problem"));
  if (f.label.trim() === "") out.push(t("p.links.label_problem"));
  if (f.kind === "tracker" && f.system === "")
    out.push(t("p.links.system_problem"));
  if (f.system !== "" && f.kind !== "tracker" && f.kind !== "ticket")
    out.push(t("p.links.system_kind_problem"));
  if (
    f.external_id.trim() !== "" &&
    (f.system === "" || /\s/.test(f.external_id.trim()))
  )
    out.push(t("p.links.id_problem"));
  const domains = HOSTED[f.system];
  if (domains && /^https?:\/\//i.test(url)) {
    const host = hostOf(url);
    if (!domains.some((d) => host === d || host.endsWith(`.${d}`)))
      out.push(t("p.links.domain_problem", { domains: domains.join(" / ") }));
  }
  return out;
}

/** The body to send for a link form. */
export function linkBody(f: LinkForm): {
  kind: string;
  url: string;
  label: string;
  system?: string;
  external_id?: string;
} {
  const body: {
    kind: string;
    url: string;
    label: string;
    system?: string;
    external_id?: string;
  } = {
    kind: f.kind,
    url: f.url.trim(),
    label: f.label.trim(),
  };
  if (f.system) body.system = f.system;
  if (f.external_id.trim()) body.external_id = f.external_id.trim();
  return body;
}

/** "Jira PPM-7" or just the label's system. */
export function systemText(
  l: Pick<PlanLink, "system" | "external_id">,
): string {
  if (!l.system) return "";
  const name = systemLabel(l.system);
  return l.external_id ? `${name} ${l.external_id}` : name;
}

const PRODUCT: Record<string, string> = {
  jira: "Jira",
  trello: "Trello",
  asana: "Asana",
  linear: "Linear",
  monday: "Monday",
  clickup: "ClickUp",
  azure_devops: "Azure DevOps",
  github: "GitHub",
  scrum: "Scrum",
  kanban: "Kanban",
  scrumban: "Scrumban",
  safe: "SAFe",
  xp: "XP",
  lean: "Lean",
};

/** A link kind in words. */
export function kindLabel(k: string): string {
  const labels: Record<string, string> = {
    docs: t("l.link.kind_docs"),
    repo: t("l.link.kind_repo"),
    ticket: t("l.link.kind_ticket"),
    design: t("l.link.kind_design"),
    chat: t("l.link.kind_chat"),
    tracker: t("l.link.kind_tracker"),
    other: t("l.link.kind_other"),
  };
  return labels[k] ?? k;
}
/** A tool's name: a product name, or words for "another tool". */
export const systemLabel = (s: string): string =>
  PRODUCT[s] ?? (s === "other" ? t("l.link.system_other") : s);
/** A methodology in words. */
export function methodologyLabel(m: string): string {
  if (PRODUCT[m]) return PRODUCT[m];
  const labels: Record<string, string> = {
    waterfall: t("l.fw.method_waterfall"),
    hybrid: t("l.fw.method_hybrid"),
    other: t("l.fw.method_other"),
  };
  return labels[m] ?? m;
}
/** A decision model in words. */
export function decisionModelLabel(m: string): string {
  const labels: Record<string, string> = {
    sponsor: t("l.fw.model_sponsor"),
    steering_committee: t("l.fw.model_steering_committee"),
    product_owner: t("l.fw.model_product_owner"),
    delegated: t("l.fw.model_delegated"),
    consensus: t("l.fw.model_consensus"),
    other: t("l.fw.model_other"),
  };
  return labels[m] ?? m;
}
