// Lean fetch wrapper for the Case Service.
//
// The service is an Axum app: handlers return RAW JSON (no
// {success,data,error} envelope), so this client returns the parsed
// body directly and throws ApiError on non-2xx.
//
// Auth is handled by the same-origin BFF proxy (it injects a server-
// exchanged PASETO), so this client attaches no bearer of its own — only
// an explicit per-request `token` override, if ever passed. See
// `agents/share/authentication-sessions.md`.

import { tDynamic } from "../i18n.svelte.js";

/**
 * Construction-time configuration for {@link ApiClient}.
 */
export interface ClientOptions {
  /** Absolute origin (and optional prefix) of the Case Service, e.g. `http://localhost:5150`. Trailing slashes are trimmed. */
  baseUrl: string;
  /** Override the `fetch` implementation (SSR, tests, instrumentation). Defaults to the global `fetch`. */
  fetch?: typeof fetch;
}

/**
 * Per-request options shared by every verb on {@link ApiClient}.
 */
export interface RequestOptions {
  /** JSON request body; serialized with `JSON.stringify`. Omit for bodyless requests (GET/DELETE). */
  body?: unknown;
  /** A body sent as is (a CSV file, say); wins over `body`. Set the `content-type` header with it. */
  rawBody?: string;
  /** Override the bearer for this request: a string sets the header, `null`/`""` suppresses it. Omit to send no bearer (the BFF injects one). */
  token?: string | null;
  /** Extra request headers, merged over (and able to override) the JSON defaults. */
  headers?: Record<string, string>;
  /** Abort signal for cancellation (e.g. component teardown). */
  signal?: AbortSignal;
  /**
   * The `Idempotency-Key` for a `POST`. Omit to let the client choose one: a fresh key per call, reused
   * while an identical `POST` (same path and body) is still in flight, so a double click is one request
   * to the service. Pass your own to reuse a key across a retry; pass `false` to send none.
   */
  idempotencyKey?: string | false;
}

/** A fresh key. `crypto.randomUUID` needs a secure context; the fallback is for plain http and tests. */
function newIdempotencyKey(): string {
  const c = globalThis.crypto;
  if (c && typeof c.randomUUID === "function") return c.randomUUID();
  return `k-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 12)}`;
}

/**
 * Error thrown for any non-2xx response. Carries the HTTP status and the
 * parsed (or raw) error body so callers can branch on them.
 */
export class ApiError extends Error {
  /** The HTTP status code of the failed response. */
  readonly status: number;
  /** The parsed JSON error body, or `undefined` when the body was empty / not JSON. */
  readonly body: unknown;

  /**
   * @param status HTTP status code of the failed response.
   * @param message Human-readable message (server-supplied where possible, else `HTTP <status>`).
   * @param body Parsed error body, if any.
   */
  constructor(status: number, message: string, body?: unknown) {
    super(message);
    this.name = "ApiError";
    this.status = status;
    this.body = body;
  }

  /** True when the status is 401 — the session token is missing/expired. */
  get isUnauthorized(): boolean {
    return this.status === 401;
  }
  /** True when the status is 400 — a malformed request. */
  get isBadRequest(): boolean {
    return this.status === 400;
  }
}

/** The page size to ask for. The rest of a list is read by following {@link Page.next}. */
export interface PageRequest {
  /** Page size; omitted ⇒ the service default. Above the service's ceiling is a `422`, not clamped. */
  limit?: number;
}

/**
 * One page of a collection. There is no total: the service sends none, because it would count rows the caller may
 * not see (FR-46). `more` says whether rows follow, and `next` is the path of the next page, with its signed cursor.
 */
export interface Page<T> {
  /** The rows in this page. */
  items: T[];
  /** Whether more rows follow this page. */
  more: boolean;
  /** The next page's path, from `Link: <…>; rel="next"`, to pass back to {@link ApiClient.getPage}; null on the last. */
  next: string | null;
}

/** The URL in a `Link: <url>; rel="next"` header, or null. */
export function nextLink(link: string | null): string | null {
  if (link === null) return null;
  for (const part of link.split(",")) {
    const match = /^\s*<([^>]*)>\s*;\s*rel="?next"?\s*$/.exec(part);
    if (match) return match[1] ?? null;
  }
  return null;
}

/**
 * Lean fetch wrapper for the Case Service's raw-JSON (no envelope) API.
 * Resolves to the parsed response body, or throws {@link ApiError} on any
 * non-2xx status. Holds no request state, so a single instance is reusable.
 */
export class ApiClient {
  /** Normalized base URL (trailing slashes stripped). */
  private readonly baseUrl: string;
  /** The `fetch` implementation to use for every request. */
  private readonly fetchFn: typeof fetch;
  /** Keys of `POST`s in flight, by method, path and body, so an identical double submit shares one key. */
  private readonly inflight = new Map<string, { key: string; count: number }>();

  /**
   * @param options Base URL, plus optional `fetch` seam.
   */
  constructor(options: ClientOptions) {
    this.baseUrl = options.baseUrl.replace(/\/+$/, "");
    this.fetchFn = options.fetch ?? globalThis.fetch.bind(globalThis);
  }

  /**
   * Issue a GET request.
   * @typeParam T Expected shape of the parsed response body.
   * @param path Request path (absolute or leading-slash-optional).
   * @param opts Optional headers/token/signal.
   * @returns The parsed response body.
   * @throws {ApiError} on any non-2xx response.
   */
  get<T>(path: string, opts?: RequestOptions): Promise<T> {
    return this.request<T>("GET", path, opts);
  }
  /**
   * Issue a POST request.
   * @typeParam T Expected shape of the parsed response body.
   * @param path Request path.
   * @param opts Options; usually carries `body`.
   * @returns The parsed response body.
   * @throws {ApiError} on any non-2xx response.
   */
  post<T>(path: string, opts?: RequestOptions): Promise<T> {
    return this.request<T>("POST", path, opts);
  }
  /**
   * Issue a PUT request.
   * @typeParam T Expected shape of the parsed response body.
   * @param path Request path.
   * @param opts Options; usually carries `body`.
   * @returns The parsed response body.
   * @throws {ApiError} on any non-2xx response.
   */
  put<T>(path: string, opts?: RequestOptions): Promise<T> {
    return this.request<T>("PUT", path, opts);
  }
  /**
   * Issue a PATCH request (partial update, e.g. a board status move).
   * @typeParam T Expected shape of the parsed response body.
   */
  patch<T>(path: string, opts?: RequestOptions): Promise<T> {
    return this.request<T>("PATCH", path, opts);
  }
  /**
   * Issue a DELETE request. Defaults `T` to `void` since soft-delete
   * returns an empty body.
   * @typeParam T Expected shape of the parsed response body.
   * @param path Request path.
   * @param opts Optional headers/token/signal.
   * @returns The parsed response body (typically `undefined`).
   * @throws {ApiError} on any non-2xx response.
   */
  delete<T = void>(path: string, opts?: RequestOptions): Promise<T> {
    return this.request<T>("DELETE", path, opts);
  }

  /**
   * Core request pipeline shared by all verbs: assemble headers, resolve
   * the bearer token, send, then parse the body and map non-2xx to
   * {@link ApiError}.
   * @typeParam T Expected shape of the parsed response body.
   * @throws {ApiError} on any non-2xx response.
   */
  /**
   * `GET` one page of a collection, returning the body **and** whether more follow (`agents/restful.md`).
   *
   * The plain {@link get} discards response headers, which is fine for a single record and useless for a
   * collection: the next page lives in the `Link` header, so that the body stays the array every existing caller
   * already parses. Pass a page's {@link Page.next} back as `path` to read the next one.
   */
  async getPage<T>(
    path: string,
    page: PageRequest = {},
    opts: RequestOptions = {},
  ): Promise<Page<T>> {
    const query = new URLSearchParams();
    if (page.limit !== undefined) query.set("limit", String(page.limit));
    const suffix = query.toString();
    const { data, response } = await this.requestWithResponse<T[]>(
      "GET",
      suffix ? `${path}${path.includes("?") ? "&" : "?"}${suffix}` : path,
      opts,
    );
    const next = nextLink(response.headers.get("link"));
    return { items: data ?? [], more: next !== null, next };
  }

  private async request<T>(
    method: string,
    path: string,
    opts: RequestOptions = {},
  ): Promise<T> {
    const { data } = await this.requestWithResponse<T>(method, path, opts);
    return data;
  }

  /**
   * As {@link request}, but also returns the raw {@link Response} so a
   * caller can read headers (see {@link getPage}).
   */
  private async requestWithResponse<T>(
    method: string,
    path: string,
    opts: RequestOptions = {},
  ): Promise<{ data: T; response: Response }> {
    // JSON defaults first, then spread caller overrides so they win.
    const headers: Record<string, string> = {
      "content-type": "application/json",
      accept: "application/json",
      ...opts.headers,
    };
    // A POST carries an Idempotency-Key (a caller's own header wins). The service honours it where it
    // can replay (bulk import) and ignores it elsewhere; sending it is harmless and lets a retry be safe.
    let inflightKey: string | undefined;
    const hasKey = Object.keys(headers).some(
      (h) => h.toLowerCase() === "idempotency-key",
    );
    if (method === "POST" && !hasKey && opts.idempotencyKey !== false) {
      if (opts.idempotencyKey) {
        headers["idempotency-key"] = opts.idempotencyKey;
      } else {
        const sig = `${path}\n${opts.rawBody ?? JSON.stringify(opts.body ?? null)}`;
        const held = this.inflight.get(sig);
        if (held) held.count += 1;
        else this.inflight.set(sig, { key: newIdempotencyKey(), count: 1 });
        headers["idempotency-key"] = (held ?? this.inflight.get(sig))!.key;
        inflightKey = sig;
      }
    }
    // The BFF proxy injects the bearer server-side; this client sends a
    // bearer only when one is explicitly passed for this request.
    if (opts.token) {
      headers.authorization = `Bearer ${opts.token}`;
    }

    const init: RequestInit = { method, headers, signal: opts.signal };
    if (opts.rawBody !== undefined) {
      init.body = opts.rawBody;
    } else if (opts.body !== undefined) {
      init.body = JSON.stringify(opts.body);
    }

    // Resolve `path` as a *relative* reference against the base (its
    // leading slash is stripped): `api/persons` against `<origin>/api/proxy/`
    // resolves to `<origin>/api/proxy/api/persons`. A still-absolute path
    // (one kept starting with `/`) would instead replace the base URL's
    // entire path per the URL spec, silently discarding `/api/proxy` —
    // the bug this fixed 2026-08-03 (tasks.md FE-2).
    const url = new URL(
      path.startsWith("/") ? path.slice(1) : path,
      `${this.baseUrl}/`,
    ).toString();
    let response: Response;
    try {
      response = await this.fetchFn(url, init);
    } finally {
      if (inflightKey !== undefined) {
        const held = this.inflight.get(inflightKey);
        if (held && (held.count -= 1) <= 0) this.inflight.delete(inflightKey);
      }
    }

    // Read as text first so we can handle empty bodies (soft-delete) and
    // non-JSON error pages without a hard parse failure.
    const text = await response.text();
    let parsed: unknown = undefined;
    if (text.length > 0) {
      try {
        parsed = JSON.parse(text);
      } catch {
        // Non-JSON body: only surface it as an error on a failed
        // response (truncated); a non-JSON 2xx body stays `undefined`.
        if (!response.ok) {
          throw new ApiError(
            response.status,
            localizeApiMessage(text.slice(0, 200), response.status),
          );
        }
      }
    }

    if (!response.ok) {
      // Prefer a server-supplied message; otherwise a generic status line.
      const message = localizeApiMessage(
        extractMessage(parsed),
        response.status,
      );
      throw new ApiError(response.status, message, parsed);
    }
    return { data: parsed as T, response };
  }
}

const API_CODES: Record<string, string> = {
  unauthorized: "l.enum.api.unauthorized",
  forbidden: "l.enum.api.forbidden",
  "not found": "l.enum.api.not_found",
  not_found: "l.enum.api.not_found",
  "internal error": "l.enum.api.internal_error",
  rate_limited: "l.enum.api.rate_limited",
};

/** A service error code in the reader's language; any other text from the service is shown as sent. */
function localizeApiMessage(
  message: string | undefined,
  status: number,
): string {
  if (message === undefined)
    return tDynamic("l.enum.api.http_status", { status });
  const key = API_CODES[message.trim().toLowerCase()];
  return key ? tDynamic(key) : message;
}

/**
 * Best-effort extraction of a human-readable message from a parsed error
 * body, trying the common JSON keys in order.
 * @param body Parsed response body (any shape).
 * @returns The first string found under `description`/`message`/`error`, else `undefined`.
 */
function extractMessage(body: unknown): string | undefined {
  if (body && typeof body === "object") {
    const record = body as Record<string, unknown>;
    // The service sends an RFC 9457 problem, {type, title, status, detail?, code}: the reason (`detail`) is what a
    // person needs, then the code; `title` is all a 500 carries. `message` is SvelteKit's own error body.
    for (const key of ["detail", "message", "code", "title"]) {
      const value = record[key];
      if (typeof value === "string") return value;
    }
  }
  return undefined;
}
