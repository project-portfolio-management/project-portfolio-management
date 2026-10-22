// When a best-effort read fails, a page may stay quiet only if the failure means
// "not there for this reader" (404, 403). Any other failure is shown as a note,
// so a fault never looks like an empty or unmarked answer (CQ-19).

import { ApiError } from "./api/client";

/** Whether `err` only says the thing is absent, or hidden from this reader. */
export function isAbsent(err: unknown): boolean {
  return err instanceof ApiError && (err.status === 404 || err.status === 403);
}
