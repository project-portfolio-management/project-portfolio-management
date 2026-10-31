// WW-1.1: every supported locale other than English has a translation review record, and no record claims a
// review it does not log. The translations are machine drafts; this test keeps that stated (AGENTS.md: never claim
// a translation is reviewed).

import { describe, expect, it } from "vitest";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { LOCALES } from "../../src/lib/locales";

const governance = join(process.cwd(), "..", "governance");
const recordOf = (locale: string): string =>
  locale === "cy-001"
    ? join(governance, "welsh-translation-review.md")
    : join(governance, "translation-reviews", `${locale}.md`);

describe("translation review records", () => {
  const others = LOCALES.filter((l) => l !== "en-001");

  it("exist for every supported locale but English", () => {
    expect(others.filter((l) => !existsSync(recordOf(l)))).toEqual([]);
  });

  it("are listed in the index, one row per locale", () => {
    const index = readFileSync(
      join(governance, "translation-reviews", "README.md"),
      "utf8",
    );
    expect(others.filter((l) => !index.includes(`| \`${l}\` |`))).toEqual([]);
  });

  it("say the translation is a draft until a review is logged", () => {
    const claims = others.filter((l) => {
      const text = readFileSync(recordOf(l), "utf8");
      const draft = /not (yet )?reviewed/i.test(text);
      const logged = /No review yet\./.test(text) === false;
      // A record may drop "not reviewed" only once its log has a row.
      return !draft && !logged;
    });
    expect(claims).toEqual([]);
  });

  it("each have a working file for the reviewer", () => {
    const missing = others
      .filter((l) => l !== "cy-001")
      .filter((l) => {
        const p = join(governance, "translation-reviews", `${l}.csv`);
        return (
          !existsSync(p) ||
          !readFileSync(p, "utf8").startsWith(
            "key,english,translation,reviewed_by,reviewed_on,verdict\n",
          )
        );
      });
    expect(missing).toEqual([]);
  });

  // WW-1.17: the widget-label packs are a stated limit: each locale that uses one names it in its record, and a
  // pack that says it is reviewed must have that review logged here, not just claimed by the package.
  it("name the widget-label pack each locale uses, and claim no review of it", () => {
    const pkg = join(
      process.cwd(),
      "node_modules",
      "@lilydesignsystem",
      "svelte-locale-translations",
    );
    const version = (
      JSON.parse(readFileSync(join(pkg, "package.json"), "utf8")) as {
        version: string;
      }
    ).version;
    const packOf = (l: string) =>
      l === "de-de" ? "de-001" : l === "zh-cn" ? "zh-001" : l;
    const wrong = others
      .filter((l) => l !== "cy-001")
      .filter((l) => {
        const text = readFileSync(recordOf(l), "utf8");
        const pack = JSON.parse(
          readFileSync(join(pkg, "label-packs", `${packOf(l)}.json`), "utf8"),
        ) as {
          reviewed: boolean;
        };
        const names = text.includes(
          `\`${packOf(l)}\` pack of \`@lilydesignsystem/svelte-locale-translations\` ${version}`,
        );
        const logged = /No review yet\./.test(text) === false;
        return (
          !names ||
          (pack.reviewed && !logged) ||
          !/not reviewed/.test(text.split("## Widget labels")[1] ?? "")
        );
      });
    expect(wrong).toEqual([]);
  });
});
