#!/usr/bin/env python3
"""Write project-portfolio-management-service-with-rust/src/openapi_bodies.rs (CQ-8).

For every route in a controller's `routes()` whose handler takes `Json(x): Json<Type>`, emit one line that
attaches Type's request-body schema (derived by schemars from the type the handler really deserialises) to
that route's `post`/`put`/`patch` operation. A route and its body can then not disagree: the type is the
handler's own. Routes whose handler is a closure, or whose body is a bare `serde_json::Value`, are left to the
hand-written schemas in `openapi_schemas.rs`.

  python3 scripts/generate-openapi-bodies.py           rewrite the file
  python3 scripts/generate-openapi-bodies.py --check   exit 1 if it is stale (the Docs job runs this)
"""
import json, re, pathlib, sys

SERVICE = pathlib.Path(__file__).resolve().parent.parent / "project-portfolio-management-service-with-rust"
CTRL = SERVICE / "src" / "controllers"
OUT = SERVICE / "src" / "openapi_bodies.rs"
METHODS = ("post", "put", "patch")


def balanced(s, i, open_c="(", close_c=")"):
    d = 0
    for j in range(i, len(s)):
        if s[j] == open_c:
            d += 1
        elif s[j] == close_c:
            d -= 1
            if d == 0:
                return j
    return -1


def handlers(src):
    """fn name -> body type path (as written), for handlers with a Json<T> extractor."""
    out = {}
    for m in re.finditer(r"(?:pub(?:\(crate\))? )?async fn (\w+)\s*(?:<[^>]*>)?\s*\(", src):
        end = balanced(src, m.end() - 1)
        params = src[m.end() - 1 : end + 1]
        b = re.search(r"Json\(\s*\w+\s*\)\s*:\s*Json<\s*([A-Za-z_][\w:]*)\s*>", params)
        if b:
            out[m.group(1)] = b.group(1)
    return out


def doc_of(src, name):
    """The `///` comment above `async fn name` (attributes in between are skipped), as one string."""
    m = re.search(r"(?:pub(?:\(crate\))? )?async fn " + name + r"\b", src)
    if not m:
        return ""
    lines = src[: m.start()].split("\n")[:-1]
    out = []
    for line in reversed(lines):
        t = line.strip()
        if t.startswith("///"):
            out.append(t[3:].strip())
        elif t.startswith("#[") or t.startswith("#!["):
            continue
        else:
            break
    return " ".join(reversed(out)).strip()


def signatures(src):
    """fn name -> its parameter list as written."""
    out = {}
    for m in re.finditer(r"(?:pub(?:\(crate\))? )?async fn (\w+)\s*(?:<[^>]*>)?\s*\(", src):
        end = balanced(src, m.end() - 1)
        out[m.group(1)] = src[m.end() - 1 : end + 1]
    return out


def routes(src):
    """(method, path) -> handler name, from `Routes::new().prefix(P).add(PATH, method(handler)...)`."""
    found = []
    for rm in re.finditer(r"Routes::new\(\)", src):
        # the chain runs to the first `;` or closing of the function at depth 0
        i = rm.end()
        depth = 0
        j = i
        while j < len(src):
            c = src[j]
            if c in "({[":
                depth += 1
            elif c in ")}]":
                depth -= 1
                if depth < 0:
                    break
            elif c == ";" and depth == 0:
                break
            j += 1
        chain = src[i:j]
        pm = re.search(r'\.prefix\(\s*"([^"]*)"\s*\)', chain)
        prefix = pm.group(1) if pm else ""
        k = 0
        while True:
            am = re.search(r"\.add\(", chain[k:])
            if not am:
                break
            op = k + am.end() - 1
            cl = balanced(chain, op)
            args = chain[op + 1 : cl]
            k = cl + 1
            pm2 = re.match(r'\s*"([^"]*)"\s*,\s*(.*)$', args, re.S)
            if not pm2:
                continue  # a computed path (format!): left to the hand-written schemas
            path, rest = pm2.group(1), pm2.group(2)
            for mm in re.finditer(r"\b(get|post|put|patch|delete)\(\s*(\w+)\s*\)", rest):
                found.append((mm.group(1), prefix + path, mm.group(2)))
    return found


def main():
    lines = []
    no_body = []
    raw_body = []
    docs = []
    for f in sorted(CTRL.glob("*.rs")):
        src = f.read_text()
        if "Routes::new()" not in src:
            continue
        hs = handlers(src)
        sigs = signatures(src)
        mod = f.stem
        for method, path, h in routes(src):
            if len(path) > 1:
                path = path.rstrip("/")
            if mod != "docs":
                docs.append((path, method, mod, doc_of(src, h) if h in sigs else ""))
            if method in METHODS and h not in hs and h in sigs:
                sg = sigs[h]
                if "Bytes" in sg:
                    raw_body.append(f"{method} {path}")
                elif "Json<" not in sg and "Multipart" not in sg:
                    no_body.append(f"{method} {path}")
            if method not in METHODS or h not in hs:
                continue
            ty = hs[h]
            if ty in ("Value",) or ty.endswith("::Value"):
                continue
            if "::" not in ty and not re.search(r"JsonSchema\)\]\s*(?:#\[[^\n]*\]\s*)*(?:pub(?:\([a-z]+\))? )?(?:struct|enum) " + ty + r"\b", src):
                continue  # a type from another crate (the matcher's Plan, Goal): documented by hand
            tpath = ty if "::" in ty else f"crate::controllers::{mod}::{ty}"
            if ty.startswith("crate::") or ty.startswith("super::"):
                tpath = ty.replace("super::", "crate::controllers::", 1)
            lines.append((path, method, tpath))
    lines.sort()
    no_body = sorted(set(no_body))
    raw_body = sorted(set(raw_body))
    check = "--check" in sys.argv
    docs = sorted(set(docs))
    docs_rs = "\n".join(f'    ({json.dumps(m, ensure_ascii=False)}, {json.dumps(p, ensure_ascii=False)}, {json.dumps(mod, ensure_ascii=False)}, {json.dumps(d, ensure_ascii=False)}),' for p, m, mod, d in docs)
    no_body_rs = "\n".join(f'    "{r}",' for r in no_body)
    raw_body_rs = "\n".join(f'    "{r}",' for r in raw_body)
    body = "\n".join(f'    body!("{m}", "{p}", {t});' for p, m, t in lines)
    text = f'''//! GENERATED by `scripts/generate-openapi-bodies.py`; do not edit (CQ-8).
//!
//! The request body of every write route whose handler takes `Json<Type>`, derived by `schemars` from that very
//! type, so a route and its documented body cannot drift apart. Applied only where the hand-written schemas in
//! [`crate::openapi_schemas`] have not already given the operation a body.

use schemars::generate::SchemaSettings;
use serde_json::{{Value, json}};

/// Write routes whose handler takes no request body at all (no `Json`, `Bytes` or multipart extractor): an
/// action on the resource in the path. They need no `requestBody`, and this list is how the census knows.
pub const TAKES_NO_BODY: &[&str] = &[
{no_body_rs}
];

/// Write routes whose handler reads the raw request body (`Bytes`): an uploaded file or an import, whose media
/// type and format the operation's description states.
pub const RAW_BODY: &[&str] = &[
{raw_body_rs}
];

/// Every route a controller mounts with a named handler: `(method, path, controller, the handler's doc comment)`.
/// Used to give an operation to any mounted route nobody wrote one for, so no route is undocumented; the
/// description is the handler's own doc comment, or says there is none.
pub const ROUTE_DOCS: &[(&str, &str, &str, &str)] = &[
{docs_rs}
];

/// Give every route in [`ROUTE_DOCS`] that has no operation yet a plain one (summary and description from the
/// handler's doc comment, its path parameters, a generic `200`). Hand-written operations are left as they are.
pub fn document_missing(paths: &mut Value) {{
    for (method, path, module, doc) in ROUTE_DOCS {{
        let item = paths.as_object_mut().map(|m| m.entry((*path).to_string()).or_insert_with(|| json!({{}})));
        let Some(item) = item.and_then(Value::as_object_mut) else {{ continue }};
        if item.contains_key(*method) {{
            continue;
        }}
        let summary = doc.split(". ").next().unwrap_or("").trim_end_matches('.');
        let (summary, description) = if doc.is_empty() {{
            (format!("{{}} {{path}}", method.to_uppercase()), "No description has been written for this route yet.".to_string())
        }} else {{
            (summary.to_string(), (*doc).to_string())
        }};
        let params: Vec<Value> = path
            .split('/')
            .filter_map(|seg| seg.strip_prefix('{{').and_then(|s| s.strip_suffix('}}')))
            .map(|name| json!({{ "name": name, "in": "path", "required": true, "schema": {{ "type": if name == "index" {{ "integer" }} else {{ "string" }} }} }}))
            .collect();
        item.insert((*method).to_string(), json!({{
            "tags": [module],
            "summary": summary,
            "description": description,
            "parameters": params,
            "responses": {{ "200": {{ "description": "OK; the response shape is not documented yet." }} }},
        }}));
    }}
}}

/// Attach the derived request bodies to `paths`.
#[rustfmt::skip]
#[expect(clippy::too_many_lines, reason = "one line per route, generated")]
pub fn attach(paths: &mut Value) {{
    let settings = SchemaSettings::openapi3().with(|s| s.inline_subschemas = true);
    let generator = settings.into_generator();
    macro_rules! body {{
        ($method:expr, $path:expr, $ty:ty) => {{
            if let Some(op) = paths.get_mut($path).and_then(|p| p.get_mut($method)) {{
                if op.get("requestBody").is_none() {{
                    let mut schema = serde_json::to_value(generator.clone().into_root_schema_for::<$ty>())
                        .unwrap_or_else(|_| json!({{ "type": "object" }}));
                    if let Some(o) = schema.as_object_mut() {{
                        o.remove("$schema");
                        o.remove("title");
                    }}
                    op["requestBody"] = json!({{ "required": true, "content": {{ "application/json": {{ "schema": schema }} }} }});
                }}
            }}
        }};
    }}
{body}
    for route in RAW_BODY {{
        if let Some((method, path)) = route.split_once(' ')
            && let Some(op) = paths.get_mut(path).and_then(|p| p.get_mut(method))
            && op.get("requestBody").is_none()
        {{
            op["requestBody"] = json!({{ "required": true, "content": {{ "application/octet-stream": {{ "schema": {{ "type": "string", "format": "binary" }} }} }}, "description": "The raw file; the formats accepted are in the operation description." }});
        }}
    }}
}}
'''
    if check:
        if not OUT.exists() or OUT.read_text() != text:
            print(f"{OUT.name} is stale: run python3 scripts/generate-openapi-bodies.py", file=sys.stderr)
            sys.exit(1)
        print(f"{len(lines)} bodies, current")
        return
    OUT.write_text(text)
    print(f"{len(lines)} bodies")


if __name__ == "__main__":
    main()
