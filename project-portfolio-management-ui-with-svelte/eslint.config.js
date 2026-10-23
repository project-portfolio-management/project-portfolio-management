import js from "@eslint/js";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import ts from "typescript-eslint";

export default ts.config(
  {
    ignores: [
      ".svelte-kit/",
      "build/",
      "node_modules/",
      "test-results/",
      "static/",
    ],
  },
  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs["flat/recommended"],
  {
    languageOptions: { globals: { ...globals.browser, ...globals.node } },
  },
  {
    files: ["**/*.svelte", "**/*.svelte.ts"],
    languageOptions: { parserOptions: { parser: ts.parser } },
  },
  {
    // The app is served at the root with no `paths.base`, so `resolve()` adds nothing (120 links).
    rules: { "svelte/no-navigation-without-resolve": "off" },
  },
  {
    // Found when ESLint was added (CQ-22): shown as warnings so a new one is seen, and
    // fixed in later passes. Anything else in the recommended sets fails the gate.
    rules: {
      "@typescript-eslint/no-unused-vars": [
        "warn",
        { argsIgnorePattern: "^_", varsIgnorePattern: "^_" },
      ],
      "svelte/require-each-key": "warn",
      "svelte/no-unused-svelte-ignore": "warn",
      "svelte/prefer-svelte-reactivity": "warn",
      "svelte/no-useless-mustaches": "warn",
      "no-useless-assignment": "warn",
      "@typescript-eslint/no-explicit-any": "warn",
    },
  },
  {
    files: ["src/app.d.ts"],
    rules: { "@typescript-eslint/no-empty-object-type": "off" },
  },
);
