// WW-1.1: page catalogues beyond English and Welsh are loaded per locale (`loadLocale`); unit tests read every
// locale's table, so they are all loaded before any test runs.
import { loadAllLocales } from "../../src/lib/i18n.svelte";

await loadAllLocales();
