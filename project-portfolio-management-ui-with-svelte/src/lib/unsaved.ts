// RE-13: work typed into a long form outlives an ended session. The rules, kept apart from the component so they are
// unit-tested.
//
// * A save refused with 401 means the session ended, not that the input is wrong: the form keeps everything typed
//   (in memory only, never in browser storage) and offers to sign in again in a new tab, so this tab, and what is in
//   it, stays; then the person saves again.
// * Leaving a form with changes not yet saved asks first.

import { ApiError } from "#lib/api/client.js";

/** Whether a failed save failed because the session ended. */
export function isSessionEnded(err: unknown): boolean {
  return err instanceof ApiError && err.status === 401;
}

/** Whether leaving should ask first: something was changed and it has not been saved since. */
export function shouldAskBeforeLeaving(
  changed: boolean,
  saved: boolean,
): boolean {
  return changed && !saved;
}
