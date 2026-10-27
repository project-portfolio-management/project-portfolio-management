// CQ-9: every path the API client calls is a route the service mounts. The service writes its
// mounted routes to `api-routes.txt` (kept current by its own test); this reads that file and
// the client source, so a renamed or removed route fails here instead of in production.

import { describe, it, expect } from "vitest";
import { readFileSync } from "node:fs";
import { join } from "node:path";

const ROUTES = readFileSync(
  join(
    process.cwd(),
    "../project-portfolio-management-service-with-rust/api-routes.txt",
  ),
  "utf8",
)
  .split("\n")
  .filter(Boolean)
  .map((l) => {
    const [method, path] = l.split(" ");
    return { method, segments: path.split("/") };
  });

/** Whether `method path` is mounted: a value (`{}`) must sit where the route has a parameter,
 *  and a written-out segment where the route has the same word. */
function mounted(method: string, path: string): boolean {
  const want = path.split("/");
  return ROUTES.some(
    (r) =>
      r.method === method &&
      r.segments.length === want.length &&
      r.segments.every((s, i) =>
        s.startsWith("{") ? want[i] === "{}" : s === want[i],
      ),
  );
}

/** Read a template literal starting at `i` (on the backtick); returns its parts and the end. */
function readTemplate(
  src: string,
  i: number,
): { parts: (string | { expr: string })[]; end: number } {
  const parts: (string | { expr: string })[] = [];
  let text = "";
  i += 1;
  while (src[i] !== "`") {
    if (src[i] === "\\") {
      text += src[i + 1];
      i += 2;
    } else if (src[i] === "$" && src[i + 1] === "{") {
      parts.push(text);
      text = "";
      let depth = 1;
      let j = i + 2;
      while (depth > 0) {
        if (src[j] === "`") j = readTemplate(src, j).end;
        else {
          if (src[j] === "{") depth += 1;
          if (src[j] === "}") depth -= 1;
          j += 1;
        }
      }
      parts.push({ expr: src.slice(i + 2, j - 1) });
      i = j;
    } else {
      text += src[i];
      i += 1;
    }
  }
  parts.push(text);
  return { parts, end: i + 1 };
}

/** The string and template literals inside an expression, as path alternatives. */
function literalsIn(expr: string): string[][] {
  const out: string[][] = [];
  for (let i = 0; i < expr.length; i++) {
    if (expr[i] === "`") {
      const t = readTemplate(expr, i);
      out.push(expand(t.parts));
      i = t.end - 1;
    } else if (expr[i] === '"') {
      const j = expr.indexOf('"', i + 1);
      out.push([expr.slice(i + 1, j)]);
      i = j;
    }
  }
  return out;
}

/** The string literals a name may hold, when its type is a union of them, else `null`. */
type Union = (name: string) => string[] | null;
const noUnion: Union = () => null;

/** Every concrete path a template can produce, with `{}` for a plain value. */
function expand(
  parts: (string | { expr: string })[],
  union: Union = noUnion,
): string[] {
  let paths = [""];
  parts.forEach((p, k) => {
    if (typeof p === "string") {
      paths = paths.map((x) => x + p);
      return;
    }
    const expr = p.expr.trim();
    const before =
      typeof parts[k - 1] === "string" ? (parts[k - 1] as string) : "";
    if (expr === "this.base()") {
      paths = paths.map((x) => x + "/api/plans");
      return;
    }
    const branches = literalsIn(expr).flat();
    if (branches.length > 0) {
      // A choice between written-out pieces: every branch is a real call.
      paths = paths.flatMap((x) => branches.map((a) => x + a));
    } else if (before.endsWith("/")) {
      // A name typed as a union of words (`action: "submit" | "approve"`) is each of them.
      const words = /^[A-Za-z_]\w*$/.test(expr) ? union(expr) : null;
      paths = words
        ? paths.flatMap((x) => words.map((w) => x + w))
        : paths.map((x) => x + "{}");
    }
    // Otherwise a suffix such as `${qs}` or `${query({...})}`: the query string, not the path.
  });
  return paths.map((x) => x.split("?")[0]);
}

const CALL =
  /this\.http\.(get|getPage|post|put|patch|delete)\s*(?:<(?:[^<>]|<[^<>]*>)*>)?\(\s*/g;

function calls(
  file: string,
): { at: string; method: string; paths: string[] }[] {
  const src = readFileSync(join(process.cwd(), "src/lib/api", file), "utf8");
  // The wire types moved to `ppm-types.ts`; a union a call's name is typed as may be declared in either.
  const typeSrc =
    src + readFileSync(join(process.cwd(), "src/lib/api/ppm-types.ts"), "utf8");
  const out: { at: string; method: string; paths: string[] }[] = [];
  for (const m of src.matchAll(CALL)) {
    const i = (m.index ?? 0) + m[0].length;
    const method = m[1] === "getPage" ? "GET" : m[1].toUpperCase();
    const line = src.slice(0, i).split("\n").length;
    const union: Union = (name) => {
      const window = src.slice(Math.max(0, i - 800), i);
      const decl = [
        ...window.matchAll(
          new RegExp(
            `\\b${name}\\??:\\s*([A-Za-z_]\\w*|"[^"]+"(?:\\s*\\|\\s*"[^"]+")*)`,
            "g",
          ),
        ),
      ].pop()?.[1];
      if (!decl) return null;
      const literals = decl.startsWith('"')
        ? decl
        : new RegExp(`type ${decl} =\\s*("[^"]+"(?:\\s*\\|\\s*"[^"]+")*)`).exec(
            typeSrc,
          )?.[1];
      return literals
        ? [...literals.matchAll(/"([^"]+)"/g)].map((w) => w[1])
        : null;
    };
    let paths: string[];
    if (src[i] === "`") paths = expand(readTemplate(src, i).parts, union);
    else if (src[i] === '"')
      paths = [src.slice(i + 1, src.indexOf('"', i + 1))];
    else if (src.startsWith("this.base()", i)) paths = ["/api/plans"];
    else {
      // A name: the nearest `const <name> = \`…\`` before the call.
      const name = /^[A-Za-z_]\w*/.exec(src.slice(i))?.[0];
      const decl = name ? src.lastIndexOf(`const ${name} = \``, i) : -1;
      paths =
        decl >= 0
          ? expand(readTemplate(src, src.indexOf("`", decl)).parts, union)
          : [];
    }
    out.push({ at: `${file}:${line}`, method, paths });
  }
  return out;
}

describe("the API client calls only mounted routes (CQ-9)", () => {
  const all = ["ppm.ts", "plans.ts", "capabilities.ts", "tba.ts"].flatMap(
    calls,
  );

  it("reads every call", () => {
    expect(all.length).toBeGreaterThan(250);
    expect(all.filter((c) => c.paths.length === 0).map((c) => c.at)).toEqual(
      [],
    );
  });

  it("finds each path in the service's route table", () => {
    const missing = all.flatMap((c) =>
      c.paths
        .filter((p) => !mounted(c.method, p))
        .map((p) => `${c.at} ${c.method} ${p}`),
    );
    expect(missing).toEqual([]);
  });
});
