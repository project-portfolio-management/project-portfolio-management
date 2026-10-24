// The one place a table's untyped row becomes the page's own row type (CQ-21).
//
// The data grid hands every callback a `DataGridRow` (`Record<string, unknown>`), because one component
// serves every table. A page knows what it put in `rows`, so it says so here, by name, instead of casting
// at each use. This is a cast, and as unchecked as one: the type argument must be the type of the objects
// passed as `rows` to the same table. Nothing verifies it.

import type { DataGridRow } from "@lilydesignsystem/svelte-data-grid";

/** `row` as the `T` this table's `rows` hold. */
export function rowOf<T extends object>(row: DataGridRow): T {
  return row as unknown as T;
}
