// One pending yes/no question at a time, answered by `ConfirmHost` (mounted once in the layout).
// Replaces the browser's `confirm()`, whose text and buttons follow the browser, not the chosen locale.

type Pending = { message: string; resolve: (yes: boolean) => void };

export const confirmState = $state<{ pending: Pending | null }>({
  pending: null,
});

/** Ask the person to confirm. Resolves true on OK, false on Cancel or Escape. A second question while
 * one is open answers the first "no", so no promise is left hanging. */
export function askConfirm(message: string): Promise<boolean> {
  confirmState.pending?.resolve(false);
  return new Promise((resolve) => {
    confirmState.pending = { message, resolve };
  });
}

/** Close the open question with `yes`. */
export function answerConfirm(yes: boolean): void {
  const p = confirmState.pending;
  confirmState.pending = null;
  p?.resolve(yes);
}
