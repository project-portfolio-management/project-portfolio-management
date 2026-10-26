// The command palette's request line (BR-14): the button in the menu and the dialog mounted at the root are
// separate components (the dialog cannot live inside the collapsed menu), so one asks and the other answers.

/** Bumped each time something asks for the palette; the dialog opens when it changes. */
export const paletteRequests = $state({ count: 0 });

/** Ask for the palette to open. */
export function requestPalette(): void {
  paletteRequests.count++;
}
