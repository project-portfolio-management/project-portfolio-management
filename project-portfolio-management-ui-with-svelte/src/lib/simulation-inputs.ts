// The inputs the Monte Carlo analyses read (schedule risk, cost risk, joint
// risk): a task's optimistic / pessimistic duration, a budget line's low / high
// cost, and a register risk's probability, schedule impact and cost impact.
// Pure and unit-tested: given what a form holds and what is stored, say which
// calls to make — or what is wrong — without touching the network.
//
// One register risk has ONE probability, shared by the schedule and cost models,
// so a risk with both effects is a single event that strikes once. The two
// endpoints each replace their own fields, and the cost endpoint changes the
// probability only when it is given; the order of the calls below keeps the
// stored state consistent.

/** What the risk form holds (text, as typed). */
export interface RiskInputsForm {
  probability: FieldValue;
  taskPid: FieldValue;
  impactDays: FieldValue;
  /** Cost in major units (e.g. "1500.00"). */
  cost: FieldValue;
  currency: FieldValue;
}

/** What is stored on the risk now. */
export interface RiskInputsCurrent {
  task_pid: string | null;
  schedule_impact_days: number | null;
  probability_percent: number | null;
  cost_impact_minor: number | null;
  cost_currency: string | null;
}

export interface ScheduleCall {
  task_pid: string | null;
  impact_days: number | null;
  probability_percent: number | null;
}

export interface CostCall {
  cost_impact_minor: number | null;
  cost_currency: string | null;
  /** Left out when unchanged. */
  probability_percent?: number;
}

export type RiskCalls =
  { error: string } | { schedule: ScheduleCall | null; cost: CostCall | null };

/** What a bound form control holds: a number input binds a number (or undefined), a text input a string. */
export type FieldValue = string | number | null | undefined;

const str = (v: FieldValue): string =>
  v === null || v === undefined ? "" : String(v);
const blank = (v: FieldValue): boolean => str(v).trim() === "";

/** Whole money in major units to minor units, or `null` if it is not a positive amount. */
export function toMinorStrict(major: FieldValue): number | null {
  const n = Number(str(major));
  if (!Number.isFinite(n) || n <= 0) return null;
  return Math.round(n * 100);
}

/** The form for a risk, pre-filled from what is stored. */
export function formFrom(c: RiskInputsCurrent): RiskInputsForm {
  return {
    probability:
      c.probability_percent === null ? "" : String(c.probability_percent),
    taskPid: c.task_pid ?? "",
    impactDays:
      c.schedule_impact_days === null ? "" : String(c.schedule_impact_days),
    cost:
      c.cost_impact_minor === null
        ? ""
        : (c.cost_impact_minor / 100).toFixed(2),
    currency: c.cost_currency ?? "",
  };
}

/**
 * The calls that make the stored state match the form, or the first thing wrong.
 * `schedule` is null when that endpoint need not be called.
 */
export function planRiskCalls(
  form: RiskInputsForm,
  current: RiskInputsCurrent,
): RiskCalls {
  const probBlank = blank(form.probability);
  let probability: number | null = null;
  if (!probBlank) {
    probability = Number(str(form.probability));
    if (
      !Number.isInteger(probability) ||
      probability < 0 ||
      probability > 100
    ) {
      return { error: "The probability must be a whole number from 0 to 100." };
    }
  }

  const hasTask = !blank(form.taskPid);
  const hasDays = !blank(form.impactDays);
  if (hasTask !== hasDays) {
    return {
      error:
        "Name the task and the days it adds together, or leave both empty.",
    };
  }
  let days: number | null = null;
  if (hasDays) {
    days = Number(str(form.impactDays));
    if (!Number.isInteger(days) || days < 1 || days > 365) {
      return {
        error: "The days a risk adds must be a whole number from 1 to 365.",
      };
    }
  }

  const hasCost = !blank(form.cost);
  const hasCurrency = !blank(form.currency);
  if (hasCost !== hasCurrency) {
    return {
      error: "Give the cost and its currency together, or leave both empty.",
    };
  }
  let minor: number | null = null;
  let currency: string | null = null;
  if (hasCost) {
    minor = toMinorStrict(form.cost);
    if (minor === null)
      return { error: "The cost must be an amount above zero." };
    currency = str(form.currency).trim().toUpperCase();
    if (!/^[A-Z]{3}$/.test(currency))
      return { error: "The currency must be a 3-letter code such as GBP." };
  }

  const wantsSchedule = hasTask && hasDays;
  const wantsCost = hasCost && hasCurrency;
  if ((wantsSchedule || wantsCost) && probability === null) {
    return {
      error:
        "A risk that changes the schedule or the cost needs a probability.",
    };
  }

  const hadSchedule =
    current.task_pid !== null || current.schedule_impact_days !== null;
  const hadProbability = current.probability_percent !== null;
  // Clearing the probability can only be done through the schedule endpoint
  // (it replaces its fields, the probability among them), so it goes first.
  const needSchedule =
    wantsSchedule || hadSchedule || (probBlank && hadProbability);
  const schedule: ScheduleCall | null = needSchedule
    ? wantsSchedule
      ? {
          task_pid: str(form.taskPid).trim(),
          impact_days: days,
          probability_percent: probability,
        }
      : { task_pid: null, impact_days: null, probability_percent: null }
    : null;

  const hadCost = current.cost_impact_minor !== null;
  const needCost =
    wantsCost || hadCost || (probability !== null && !wantsSchedule);
  const cost: CostCall | null = needCost
    ? {
        cost_impact_minor: wantsCost ? minor : null,
        cost_currency: wantsCost ? currency : null,
        ...(probability !== null ? { probability_percent: probability } : {}),
      }
    : null;
  return { schedule, cost };
}

/** Problems with a budget line's low / high against its planned amount (minor units). */
export function lineEstimateProblems(
  lowMajor: string,
  highMajor: string,
  plannedMinor: number,
): string[] {
  const out: string[] = [];
  const parse = (s: FieldValue): number | null | "bad" => {
    if (blank(s)) return null;
    const n = Number(str(s));
    return Number.isFinite(n) && n >= 0 ? Math.round(n * 100) : "bad";
  };
  const lo = parse(lowMajor);
  const hi = parse(highMajor);
  if (lo === "bad" || hi === "bad")
    return ["Amounts must be numbers, zero or more."];
  if (lo !== null && lo > plannedMinor)
    out.push("The low amount cannot be above the planned amount.");
  if (hi !== null && hi < plannedMinor)
    out.push("The high amount cannot be below the planned amount.");
  return out;
}

/** Minor units for a low / high field, or null when it is empty (clears it). */
export function rangeMinor(major: FieldValue): number | null {
  return blank(major) ? null : Math.round(Number(str(major)) * 100);
}

/** Problems with a task's optimistic / pessimistic days against its likely duration. */
export function taskEstimateProblems(
  optimistic: string,
  pessimistic: string,
  likely: number | null,
): string[] {
  const out: string[] = [];
  const parse = (s: FieldValue): number | null | "bad" => {
    if (blank(s)) return null;
    const n = Number(str(s));
    return Number.isInteger(n) && n >= 0 && n <= 3650 ? n : "bad";
  };
  const o = parse(optimistic);
  const p = parse(pessimistic);
  if (o === "bad" || p === "bad")
    return ["Days must be whole numbers from 0 to 3650."];
  if ((o !== null || p !== null) && likely === null) {
    return ["Give the task a duration first: that is its likely value."];
  }
  if (likely !== null && o !== null && o > likely)
    out.push(`Optimistic cannot be above the likely duration (${likely}).`);
  if (likely !== null && p !== null && p < likely)
    out.push(`Pessimistic cannot be below the likely duration (${likely}).`);
  return out;
}

/** The range as words, or a plain statement that there is none. */
export function rangeText(
  lowMinor: number | null,
  highMinor: number | null,
  fmt: (m: number) => string,
): string {
  if (lowMinor === null && highMinor === null)
    return "No range: the simulation uses its fallback band";
  return `${lowMinor === null ? "planned" : fmt(lowMinor)} to ${highMinor === null ? "planned" : fmt(highMinor)}`;
}

/** A task's range as words. */
export function taskRangeText(
  o: number | null | undefined,
  likely: number | null,
  p: number | null | undefined,
): string {
  if ((o ?? null) === null && (p ?? null) === null)
    return "No range: the simulation uses its fallback band";
  return `${o ?? likely ?? "?"} / ${likely ?? "?"} / ${p ?? likely ?? "?"} working days (optimistic / likely / pessimistic)`;
}
