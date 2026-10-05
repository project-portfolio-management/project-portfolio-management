// A Svelte action that repairs ARIA values the embedded SVAR widgets get wrong
// (LT-12). The SVAR grid marks its header row `aria-rowindex="0"`, but ARIA row
// indexes start at 1, so assistive technology is handed an invalid value
// (axe: aria-valid-attr-value, serious). We cannot change the library, so the
// invalid attribute is removed — an absent index is valid, and the grid keeps
// working — now and whenever the widget re-renders.

const INVALID = '[aria-rowindex="0"]';

/** Remove every invalid `aria-rowindex="0"` under `node`; returns how many. */
export function repairRowIndexes(node: ParentNode): number {
  let n = 0;
  node.querySelectorAll(INVALID).forEach((el) => {
    el.removeAttribute("aria-rowindex");
    n += 1;
  });
  return n;
}

/** `use:repairAria` on the element wrapping a SVAR widget. */
export function repairAria(node: HTMLElement): { destroy: () => void } {
  repairRowIndexes(node);
  const observer = new MutationObserver(() => repairRowIndexes(node));
  observer.observe(node, {
    subtree: true,
    childList: true,
    attributes: true,
    attributeFilter: ["aria-rowindex"],
  });
  return { destroy: () => observer.disconnect() };
}
