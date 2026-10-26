// Paste rows from a spreadsheet (BR-15): the pure rules. No runes, no `$app/*`; tested in `tests/unit/paste-view.test.ts`.
//
// A person copies cells from a spreadsheet and pastes them as text: one task per line, the cells separated by tabs.
// The columns are Title, Deadline (yyyy-mm-dd) and Notes, in that order (the import stores the date as the task's
// deadline, not its due date); a first line that names the columns is
// skipped. A row with a problem is shown with the reason and is left out; only the good rows are sent on. They go to
// the service's Asana-shaped import (`dry_run=true` first, writing nothing), each with an id made from its own
// content, so pasting the same rows again updates the tasks it made before and never duplicates them.

/** The most rows one paste takes. */
export const MAX_ROWS = 200;
/** The longest title, in characters. */
export const MAX_TITLE = 300;
/** The longest note, in characters. */
export const MAX_NOTES = 5000;

/** One pasted row, as read. */
export interface PastedRow {
  /** The line it came from, counting from 1 (a skipped header line is still counted). */
  line: number;
  title: string;
  /** `yyyy-mm-dd`, or `null` when there is none (or it is not understood: see `problems`). */
  due: string | null;
  notes: string;
  /** Why this row is left out; empty for a good row. */
  problems: string[];
}

/** What a paste came to. */
export interface Pasted {
  rows: PastedRow[];
  /** Whether the first line named the columns and was skipped. */
  header: boolean;
}

/** Split tab-separated text into rows of cells. A cell may be quoted (a spreadsheet does this for a cell with a tab or a line break in it); `""` inside quotes is one quote. */
export function parseTsv(text: string): { cells: string[]; line: number }[] {
  const out: { cells: string[]; line: number }[] = [];
  let cells: string[] = [];
  let cell = "";
  let quoted = false;
  let line = 1;
  let start = 1;
  const src = text.replace(/\r\n?/g, "\n");
  const endCell = () => {
    cells.push(cell);
    cell = "";
  };
  const endRow = () => {
    endCell();
    if (cells.some((c) => c.trim() !== "")) out.push({ cells, line: start });
    cells = [];
  };
  for (let i = 0; i < src.length; i++) {
    const c = src[i] as string;
    if (quoted) {
      if (c === '"' && src[i + 1] === '"') {
        cell += '"';
        i++;
      } else if (c === '"') quoted = false;
      else {
        cell += c;
        if (c === "\n") line++;
      }
    } else if (c === '"' && cell === "") quoted = true;
    else if (c === "\t") endCell();
    else if (c === "\n") {
      endRow();
      line++;
      start = line;
    } else cell += c;
  }
  endRow();
  return out;
}

const HEADER = new Set(["title", "name", "task", "task name", "summary"]);

/** A real calendar date written `yyyy-mm-dd`. */
export function isIsoDate(s: string): boolean {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(s);
  if (!m) return false;
  const [y, mo, d] = [Number(m[1]), Number(m[2]), Number(m[3])];
  const date = new Date(Date.UTC(y, mo - 1, d));
  return (
    date.getUTCFullYear() === y &&
    date.getUTCMonth() === mo - 1 &&
    date.getUTCDate() === d
  );
}

/** The problems with one row; the texts are keys for the caller to say in words, with their values. */
export type Problem =
  | { key: "no_title" }
  | { key: "title_long"; max: number }
  | { key: "date_bad"; value: string }
  | { key: "notes_long"; max: number }
  | { key: "same_as"; line: number }
  | { key: "beyond_limit"; max: number };

/** Read a paste into rows with their problems (as keys with values, not words: the screen words them). */
export function parsePasted(text: string): {
  rows: (Omit<PastedRow, "problems"> & { problems: Problem[] })[];
  header: boolean;
} {
  const raw = parseTsv(text);
  const header =
    raw.length > 0 && HEADER.has((raw[0]?.cells[0] ?? "").trim().toLowerCase());
  const body = header ? raw.slice(1) : raw;
  const seen = new Map<string, number>();
  const rows = body.map((r, index) => {
    const title = (r.cells[0] ?? "").trim();
    const dueText = (r.cells[1] ?? "").trim();
    const notes = (r.cells[2] ?? "").trim();
    const problems: Problem[] = [];
    if (index >= MAX_ROWS)
      problems.push({ key: "beyond_limit", max: MAX_ROWS });
    if (title === "") problems.push({ key: "no_title" });
    else if ([...title].length > MAX_TITLE)
      problems.push({ key: "title_long", max: MAX_TITLE });
    if (dueText !== "" && !isIsoDate(dueText))
      problems.push({ key: "date_bad", value: dueText });
    if ([...notes].length > MAX_NOTES)
      problems.push({ key: "notes_long", max: MAX_NOTES });
    const id = `${title.toLowerCase()}|${dueText}`;
    if (title !== "" && problems.length === 0) {
      const first = seen.get(id);
      if (first !== undefined) problems.push({ key: "same_as", line: first });
      else seen.set(id, r.line);
    }
    return {
      line: r.line,
      title,
      due: dueText !== "" && isIsoDate(dueText) ? dueText : null,
      notes,
      problems,
    };
  });
  return { rows, header };
}

/** A short stable id made from a row's own content (FNV-1a, 32 bits, in hex). */
export function rowId(title: string, due: string | null): string {
  let h = 0x811c9dc5;
  for (const ch of `${title.trim().toLowerCase()}|${due ?? ""}`) {
    for (const byte of new TextEncoder().encode(ch)) {
      h ^= byte;
      h = Math.imul(h, 0x01000193) >>> 0;
    }
  }
  return `paste-${h.toString(16).padStart(8, "0")}`;
}

const quote = (s: string): string =>
  /[",\n\r]/.test(s) ? `"${s.replace(/"/g, '""')}"` : s;

/** The good rows as the Asana-shaped CSV the import reads: Task ID, Name, Due Date, Notes. */
export function toImportCsv(
  rows: {
    title: string;
    due: string | null;
    notes: string;
    problems: unknown[];
  }[],
): string {
  const lines = ["Task ID,Name,Due Date,Notes"];
  for (const r of rows) {
    if (r.problems.length > 0) continue;
    lines.push(
      [rowId(r.title, r.due), r.title, r.due ?? "", r.notes]
        .map(quote)
        .join(","),
    );
  }
  return lines.join("\n") + "\n";
}

/** How many rows are good, and how many are left out. */
export function counts(rows: { problems: unknown[] }[]): {
  good: number;
  left: number;
} {
  const left = rows.filter((r) => r.problems.length > 0).length;
  return { good: rows.length - left, left };
}
