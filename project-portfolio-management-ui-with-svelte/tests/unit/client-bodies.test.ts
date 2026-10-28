// CQ-9: what the API client sends as a request body is what the service says it accepts. The service writes its
// OpenAPI document to `openapi.json` (kept current by its own test, and every request body in it is derived from
// the type its handler deserialises). This reads every `this.http.post|put|patch(path, { body })` in the client with
// the TypeScript checker, takes the type of `body`, and compares its fields with the document's schema for that
// route: a field the client sends that the service does not know, or a required field the client leaves out,
// fails here instead of in production.

import { describe, it, expect } from "vitest";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import ts from "typescript";

const ROOT = process.cwd();
const DOC = JSON.parse(
  readFileSync(
    join(
      ROOT,
      "../project-portfolio-management-service-with-rust/openapi.json",
    ),
    "utf8",
  ),
);

type Schema = {
  $ref?: string;
  type?: string;
  properties?: Record<string, Schema>;
  required?: string[];
  additionalProperties?: boolean | Schema;
  allOf?: Schema[];
  oneOf?: Schema[];
  anyOf?: Schema[];
};

function resolve(s: Schema | undefined): Schema | undefined {
  let cur = s;
  for (let i = 0; i < 8 && cur?.$ref; i++) {
    const name = cur.$ref.replace("#/components/schemas/", "");
    cur = DOC.components?.schemas?.[name];
  }
  return cur;
}

/** The properties and required names of an object schema, or `null` when it is not a closed object. */
function objectShape(
  s: Schema | undefined,
): { props: Set<string>; required: Set<string>; open: boolean } | null {
  const r = resolve(s);
  if (!r) return null;
  if (r.oneOf || r.anyOf) return null; // a choice of shapes: not compared
  const props = new Set<string>();
  const required = new Set<string>();
  let open =
    r.additionalProperties === true ||
    (typeof r.additionalProperties === "object" &&
      r.additionalProperties !== null);
  for (const part of r.allOf ?? [r]) {
    const p = resolve(part);
    if (!p) continue;
    Object.keys(p.properties ?? {}).forEach((k) => props.add(k));
    (p.required ?? []).forEach((k) => required.add(k));
    if (p.additionalProperties === true) open = true;
  }
  if (props.size === 0 && !r.properties) return null; // free-form
  return { props, required, open };
}

/** The OpenAPI path template that a client path (`{}` for a value) names. */
function templateFor(method: string, path: string): string | null {
  const want = path.split("/");
  for (const tpl of Object.keys(DOC.paths)) {
    const seg = tpl.split("/");
    if (seg.length !== want.length || !DOC.paths[tpl][method]) continue;
    if (
      seg.every((s, i) =>
        s.startsWith("{") ? want[i] === "{}" : s === want[i],
      )
    )
      return tpl;
  }
  return null;
}

/** The path a call names: a string, or a template literal with `{}` for each value. */
function pathOf(node: ts.Expression, checker: ts.TypeChecker): string | null {
  let n: ts.Expression = node;
  if (ts.isIdentifier(n)) {
    const decl = checker.getSymbolAtLocation(n)?.valueDeclaration;
    if (decl && ts.isVariableDeclaration(decl) && decl.initializer)
      n = decl.initializer;
    else return null;
  }
  if (ts.isStringLiteral(n) || ts.isNoSubstitutionTemplateLiteral(n))
    return n.text.split("?")[0] ?? null;
  if (ts.isTemplateExpression(n)) {
    let out = n.head.text;
    for (const span of n.templateSpans) {
      const expr = span.expression.getText();
      out += expr === "this.base()" ? "/api/plans" : "{}";
      out += span.literal.text;
    }
    // A trailing `${query}` (a query string) is not part of the path.
    return (
      out
        .replace(/\{\}$/, (m, off: number) => (out[off - 1] === "/" ? m : ""))
        .split("?")[0] ?? null
    );
  }
  if (ts.isCallExpression(n) && n.expression.getText() === "this.base")
    return "/api/plans";
  return null;
}

function program(): ts.Program {
  const files = readdirSync(join(ROOT, "src/lib/api"))
    .filter((f) => f.endsWith(".ts") && !f.endsWith(".d.ts"))
    .map((f) => join(ROOT, "src/lib/api", f));
  return ts.createProgram(files, {
    target: ts.ScriptTarget.ES2022,
    module: ts.ModuleKind.ESNext,
    moduleResolution: ts.ModuleResolutionKind.Bundler,
    strict: true,
    noEmit: true,
    skipLibCheck: true,
    baseUrl: ROOT,
    paths: { "#lib/*": ["src/lib/*"] },
    lib: ["lib.es2022.d.ts", "lib.dom.d.ts"],
  });
}

type Finding = { at: string; route: string; problem: string };

function check(): { findings: Finding[]; compared: number; unchecked: number } {
  const prog = program();
  const checker = prog.getTypeChecker();
  const findings: Finding[] = [];
  let compared = 0;
  let unchecked = 0;
  for (const sf of prog.getSourceFiles()) {
    if (!sf.fileName.includes("/src/lib/api/") || sf.fileName.endsWith(".d.ts"))
      continue;
    const visit = (node: ts.Node) => {
      if (
        ts.isCallExpression(node) &&
        ts.isPropertyAccessExpression(node.expression) &&
        /^this\.http$/.test(node.expression.expression.getText()) &&
        ["post", "put", "patch"].includes(node.expression.name.text)
      ) {
        const method = node.expression.name.text;
        const opts = node.arguments[1];
        const at = `${sf.fileName.split("/src/lib/api/")[1]}:${sf.getLineAndCharacterOfPosition(node.getStart()).line + 1}`;
        if (opts && ts.isObjectLiteralExpression(opts)) {
          const bodyProp = opts.properties.find(
            (p) => p.name && p.name.getText() === "body",
          );
          const path = node.arguments[0]
            ? pathOf(node.arguments[0], checker)
            : null;
          const tpl = path ? templateFor(method, path) : null;
          const reqBody = tpl
            ? DOC.paths[tpl][method]?.requestBody?.content?.["application/json"]
                ?.schema
            : undefined;
          const shape = objectShape(reqBody);
          if (bodyProp && tpl && shape) {
            const expr = ts.isShorthandPropertyAssignment(bodyProp)
              ? bodyProp.name
              : ts.isPropertyAssignment(bodyProp)
                ? bodyProp.initializer
                : null;
            const type = expr ? checker.getTypeAtLocation(expr) : null;
            const props =
              type &&
              !type.isUnion() &&
              !(type.flags & (ts.TypeFlags.Any | ts.TypeFlags.Unknown))
                ? type.getProperties()
                : [];
            const stringIndexed = type?.getStringIndexType() !== undefined;
            if (!type || props.length === 0 || stringIndexed) {
              unchecked += 1;
            } else {
              compared += 1;
              const route = `${method} ${tpl}`;
              const have = new Map(
                props.map((p) => [
                  p.name,
                  (p.flags & ts.SymbolFlags.Optional) === 0,
                ]),
              );
              if (!shape.open) {
                for (const name of have.keys()) {
                  if (!shape.props.has(name))
                    findings.push({
                      at,
                      route,
                      problem: `sends \`${name}\`, which the service does not accept`,
                    });
                }
              }
              for (const name of shape.required) {
                if (!have.has(name))
                  findings.push({
                    at,
                    route,
                    problem: `leaves out \`${name}\`, which the service requires`,
                  });
                else if (have.get(name) === false)
                  findings.push({
                    at,
                    route,
                    problem: `types \`${name}\` as optional, but the service requires it`,
                  });
              }
            }
          } else if (bodyProp && tpl) {
            unchecked += 1;
          }
        }
      }
      ts.forEachChild(node, visit);
    };
    visit(sf);
  }
  return { findings, compared, unchecked };
}

describe("the client's request bodies match the service's", () => {
  it("sends only fields the service accepts and leaves out none it requires", () => {
    const { findings, compared, unchecked } = check();
    if (process.env.SHOW_COUNTS)
      console.log(`compared ${compared}, not compared ${unchecked}`);
    // It must actually be comparing something.
    expect(compared).toBeGreaterThan(20);
    expect(findings.map((f) => `${f.at} ${f.route}: ${f.problem}`)).toEqual([]);
  }, 60000);
});
