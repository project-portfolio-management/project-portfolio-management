// The built site: every page is there, axe finds no serious or critical WCAG 2.2 AA violation under the light and
// the dark system preference, the locale switcher opens the same brief in another locale, a right-to-left page
// mirrors without overflowing, and the content security policy blocks nothing the site itself does.
import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";
import { lstatSync, readFileSync, readdirSync, readlinkSync } from "node:fs";
import {
  BRIEF_LOCALES,
  COMPARE_PRODUCTS,
  DOC_PAGES,
  HOME_LOCALES,
  ROLES,
  TOPIC_KEYS,
  topicSlug,
} from "../../src/lib/site.ts";
import messages from "../../src/lib/messages.json" with { type: "json" };

// Every theme the picker offers: PickerBar's default list is the package's whole catalogue.
const THEMES = readdirSync("node_modules/@lilydesignsystem/themes/dist")
  .filter((f) => f.endsWith(".css"))
  .map((f) => f.replace(/\.css$/, ""));
const PICKERS = ["Pages", "Search this site", "Theme", "Language", "Time zone", "Text size", "Share this page"];

const docRoutes = DOC_PAGES.map((d) => `/${d.slug}/`);
const homeRoutes = ["/", ...HOME_LOCALES.map((l) => `/${l}/`)];
const topicRoutes = HOME_LOCALES.flatMap((l) => TOPIC_KEYS.map((k) => `/${l}/${topicSlug(l, k)}/`));
const compareRoutes = ["/compare/", ...COMPARE_PRODUCTS.map((p) => `/compare/${p}/`)];
const indexRoutes = BRIEF_LOCALES.map((l) => `/${l}/for/`);
const briefRoutes = BRIEF_LOCALES.flatMap((l) => ROLES.map((r) => `/${l}/for/${r}/`));
// Audited: every document, every locale's list, and one brief per locale (a different role each time).
const audited = [
  ...docRoutes,
  ...homeRoutes,
  ...topicRoutes.filter((r) => /^\/(en-001|cy-gb|zh-cn)\//.test(r)),
  ...compareRoutes,
  ...indexRoutes,
  ...BRIEF_LOCALES.map((l, i) => `/${l}/for/${ROLES[i % ROLES.length]}/`),
  "/search/?portfolio",
];

/** Load a page under a colour scheme and wait until ThemePicker has applied the matching Lily theme. */
async function open(page: Page, route: string, scheme: "light" | "dark") {
  await page.emulateMedia({ colorScheme: scheme });
  await page.goto(route, { waitUntil: "networkidle" });
  await page.waitForFunction(
    (t) =>
      document.documentElement.dataset.theme === t &&
      [...document.styleSheets].some((s) => (s.href ?? "").endsWith(`/assets/themes/${t}.css`)),
    scheme,
  );
}

async function axe(page: Page): Promise<string[]> {
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
    .analyze();
  return results.violations
    .filter((v) => v.impact === "serious" || v.impact === "critical")
    .map((v) => `${v.id}: ${v.nodes.slice(0, 3).map((n) => n.target.join(" ")).join("; ")}`);
}

test("every page is built and answers 200", async ({ request }) => {
  const all = [...docRoutes, ...homeRoutes, ...topicRoutes, ...compareRoutes, ...indexRoutes, ...briefRoutes, "/search/"];
  expect(new Set(all).size).toBe(all.length);
  for (const route of all) {
    const res = await request.get(route);
    expect(res.status(), route).toBe(200);
  }
});

for (const route of audited)
  for (const scheme of ["light", "dark"] as const)
    test(`${route} passes axe (${scheme})`, async ({ page }) => {
      const csp: string[] = [];
      page.on("console", (m) => {
        if (/Content Security Policy/i.test(m.text())) csp.push(m.text());
      });
      await open(page, route, scheme);
      await expect(page.locator("h1").first()).toBeVisible();
      expect(await axe(page)).toEqual([]);
      expect(csp).toEqual([]);
    });

test("a brief's html carries its locale; a document is English", async ({ page }) => {
  await page.goto("/cy-gb/for/chief-risk-officer/");
  await expect(page.locator("html")).toHaveAttribute("lang", "cy-GB");
  await expect(page.locator("html")).toHaveAttribute("dir", "ltr");
  await page.goto("/install/");
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
});

test("the locale switcher opens the same brief in the chosen locale", async ({ page }) => {
  await open(page, "/en-gb/for/chief-risk-officer/", "light");
  await page.getByRole("button", { name: "Language" }).click();
  await page.locator('.locale-picker-option[lang="cy-GB"], .locale-picker-option').filter({ hasText: /Cymraeg|Welsh/ }).first().click();
  await expect(page).toHaveURL(/\/cy-(gb|001)\/for\/chief-risk-officer\/$/);
  await expect(page.locator("html")).toHaveAttribute("lang", /^cy-/);
  await expect(page.locator("h1").first()).toContainText("Risg");
});

/** Choose a locale in the picker. The button's name is in the language the page is in now. */
async function chooseLocale(page: Page, current: string, option: RegExp) {
  await page.locator(".picker-bar").getByRole("button", { name: current }).click();
  await page.locator(".locale-picker-option").filter({ hasText: option }).first().click();
}

test("every language has every word, and no translation is left empty", () => {
  const keys = Object.keys(messages.en).sort();
  for (const lang of ["cy", "zh"] as const) {
    expect(Object.keys(messages[lang]).sort(), lang).toEqual(keys);
    for (const key of keys) expect(messages[lang][key as keyof typeof messages.en], `${lang} ${key}`).not.toBe("");
    for (const key of keys)
      expect(
        messages[lang][key as keyof typeof messages.en].match(/\{\w+\}/g)?.filter((v) => v !== "{n}").sort(),
        `${lang} ${key}`,
      ).toEqual(messages.en[key as keyof typeof messages.en].match(/\{\w+\}/g)?.filter((v) => v !== "{n}").sort());
  }
});

test("on a document the locale picker changes the site's words, stays put, and is remembered", async ({ page }) => {
  await open(page, "/readme/", "light");
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
  expect(page.url()).toMatch(/\/readme\/$/);
  await chooseLocale(page, "Language", /Cymraeg|Welsh/);
  await expect(page.getByRole("link", { name: "Nodweddion" }).first()).toBeVisible();
  await expect(page.locator("header")).toHaveAttribute("lang", /^cy/);
  await expect(page.locator("footer")).toContainText("Wedi'i adeiladu");
  await expect(page.locator(".english-only")).toContainText("Saesneg");
  // The document itself is still English, and says so.
  await expect(page).toHaveURL(/\/readme\/$/);
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
  await page.reload();
  await expect(page.getByRole("link", { name: "Nodweddion" }).first()).toBeVisible();
  await expect(page.locator(".picker-bar").getByRole("button", { name: "Thema" })).toBeVisible();
  await page.getByRole("link", { name: "Briffiau rôl" }).click();
  await expect(page).toHaveURL(/\/cy-(gb|001)\/for\/$/);

  await chooseLocale(page, "Iaith", /中文|Chinese/);
  await expect(page).toHaveURL(/\/zh-(cn|001)\/for\/$/);
  await expect(page.locator("h1").first()).toHaveText("角色简报");
  await expect(page.locator("html")).toHaveAttribute("lang", /^zh/);
});

test("a brief's page is prerendered with the site's words in its language", async ({ request }) => {
  const words = async (route: string) => (await request.get(route)).text();
  expect(await words("/cy-gb/for/")).toContain("Briffiau rôl");
  expect(await words("/zh-cn/for/chief-risk-officer/")).toContain("所有角色简报");
  expect(await words("/en-gb/for/")).toContain("Role briefs");
  // No Hindi site words exist: they are English, and marked so.
  const hindi = await words("/hi-in/for/");
  expect(hindi).toContain("Role briefs");
  expect(hindi).toMatch(/<header[^>]*lang="en"/);
});

test("a chosen language reaches the search page, and a word with no storage still works", async ({ page }) => {
  await open(page, "/", "light");
  await chooseLocale(page, "Language", /中文|Chinese/);
  await page.goto("/search/?zzzz-no-such-word");
  await expect(page.getByRole("status").filter({ hasText: "没有页面匹配" })).toBeVisible();
  await expect(page.locator("main [lang]").first()).toHaveAttribute("lang", /^zh/);
});

for (const route of ["/", "/install/", "/search/", "/en-gb/for/", "/hi-in/for/chief-risk-officer/"])
  test(`${route} has the full picker bar, in order`, async ({ page }) => {
    await open(page, route, "light");
    const buttons = page.locator(".picker-bar").getByRole("button");
    for (const name of PICKERS) await expect(page.locator(".picker-bar").getByRole("button", { name })).toBeVisible();
    const names = await buttons.evaluateAll((els) => els.map((e) => e.getAttribute("aria-label") ?? ""));
    expect(names.filter((n) => n !== "")).toEqual(PICKERS);
  });

test("the link picker goes to a page and marks the current one", async ({ page }) => {
  await open(page, "/install/", "light");
  await page.getByRole("button", { name: "Pages" }).click();
  await expect(page.locator(".link-picker-list [aria-current=page]")).toHaveText("Install");
  await page.locator(".link-picker-list").getByText("Specification").click();
  await expect(page).toHaveURL(/\/spec\/$/);
});

test("the search picker finds a document and a brief, by their words", async ({ page }) => {
  await open(page, "/", "light");
  await page.getByRole("button", { name: "Search this site" }).click();
  await page.getByRole("searchbox", { name: "Search terms" }).fill("Devaux DIPP");
  await page.getByRole("searchbox", { name: "Search terms" }).press("Enter");
  await expect(page).toHaveURL(/\/search\/\?Devaux%20DIPP$/);
  await expect(page.getByRole("status").filter({ hasText: /match/ })).toBeVisible();
  await expect(page.locator('.search-hits a[href$="/spec/"]').first()).toHaveAttribute("href", /\/spec\/$/);
  await page.locator('.search-hits a[href$="/spec/"]').first().click();
  await expect(page.locator("h1").first()).toContainText("Living Specification");

  await page.goto("/search/?Risg");
  await expect(page.locator('.search-hits li[lang^="cy-"]').first()).toBeVisible();
  await page.goto("/search/?zzzz-no-such-word");
  await expect(page.getByRole("status").filter({ hasText: "No page matches" })).toBeVisible();
  await page.goto("/search/?");
  await expect(page.getByRole("status").filter({ hasText: "Enter a word" })).toBeVisible();
});

test("the search page's own field searches again", async ({ page }) => {
  await page.goto("/search/?portfolio");
  await expect(page.locator(".search-hits li").first()).toBeVisible();
  await page.getByRole("searchbox", { name: /Search the documents/ }).fill("zzzz-no-such-word");
  await page.getByRole("searchbox", { name: /Search the documents/ }).press("Enter");
  await expect(page.getByRole("status").filter({ hasText: "No page matches" })).toBeVisible();
  await expect(page.locator(".search-hits")).toHaveCount(0);
});

test("every theme file the picker can ask for is built", async ({ request }) => {
  expect(THEMES.length).toBeGreaterThan(50);
  for (const theme of THEMES)
    expect((await request.get(`/assets/themes/${theme}.css`)).status(), theme).toBe(200);
});

test("choosing a theme applies it and it survives a reload", async ({ page }) => {
  await open(page, "/", "light");
  await page.getByRole("button", { name: "Theme" }).click();
  await page.locator(".theme-picker-option").filter({ hasText: /^Nord$/ }).click();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "nord");
  await page.reload();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "nord");
});

test("the time zone picker records the choice on the page", async ({ page }) => {
  await open(page, "/", "light");
  await page.getByRole("button", { name: "Time zone" }).click();
  await page.locator(".time-zone-picker-option").filter({ hasText: /Japan/ }).first().click();
  await expect(page.locator("html")).toHaveAttribute("data-time-zone", /tokyo/i);
});

test("the share picker offers the destinations and copies the link", async ({ page, context }) => {
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await open(page, "/install/", "light");
  await page.evaluate(() => Object.defineProperty(navigator, "share", { value: undefined }));
  await page.getByRole("button", { name: "Share this page" }).click();
  const list = page.locator(".share-picker-list");
  for (const name of ["Email", "LinkedIn", "Reddit", "Bluesky", "Mastodon", "Copy link"])
    await expect(list.getByText(name)).toBeVisible();
  await list.getByText("Copy link").click();
  await expect(page.locator(".share-picker-status")).toHaveText("Link copied");
  expect(await page.evaluate(() => navigator.clipboard.readText())).toMatch(/\/install\/$/);
});

test("axe finds nothing serious with a picker's popup open, in each language", async ({ page }) => {
  for (const lang of ["Language", "Iaith"]) {
    await open(page, "/", "dark");
    if (lang === "Iaith") await chooseLocale(page, "Language", /Cymraeg|Welsh/);
    const names = lang === "Iaith" ? ["Tudalennau", "Thema", "Iaith", "Cylchfa amser", "Maint testun"] : PICKERS;
    for (const name of names) {
      await page.locator(".picker-bar").getByRole("button", { name }).click();
      // axe wants a scrolling list to be focusable from the page's tab order. These lists take focus themselves when
      // they open and move through their options with aria-activedescendant, which the next test proves for the longest.
      const results = await new AxeBuilder({ page })
        .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
        .disableRules(["scrollable-region-focusable"])
        .analyze();
      const serious = results.violations.filter((v) => v.impact === "serious" || v.impact === "critical");
      expect(serious.map((v) => v.id), name).toEqual([]);
      await page.keyboard.press("Escape");
    }
  }
});

test("the keyboard reaches the last of the 57 themes, scrolled into view", async ({ page }) => {
  await open(page, "/", "light");
  await page.locator(".theme-picker-button").focus();
  await page.keyboard.press("Enter");
  const list = page.locator(".theme-picker-list");
  await expect(list).toBeFocused();
  await page.keyboard.press("End");
  const last = THEMES.length - 1;
  await expect(list).toHaveAttribute("aria-activedescendant", new RegExp(`-${last}$`));
  await expect(page.locator(".theme-picker-option").last()).toBeInViewport();
});

test("every locale's home page has a translated hero and names the translations", async ({ request }) => {
  const heroes: Record<string, [string, string]> = {
    "en-001": ["Project Portfolio Management", "See everything your organization funds"],
    "en-gb": ["Project Portfolio Management", "See everything your organisation funds"],
    "cy-gb": ["Rheoli Portffolio Prosiectau", "Gwelwch bopeth"],
    "cy-001": ["Rheoli Portffolio Prosiectau", "Gwelwch bopeth"],
    "zh-cn": ["项目组合管理", "看清组织出资"],
    "zh-001": ["项目组合管理", "看清组织出资"],
  };
  for (const [locale, [headline, sub]] of Object.entries(heroes)) {
    const html = await (await request.get(`/${locale}/`)).text();
    expect(html, locale).toMatch(new RegExp(`<h1[^>]*>${headline}</h1>`));
    expect(html, locale).toContain(sub);
    for (const name of ["English", "Cymraeg", "中文"]) expect(html, locale).toContain(name);
  }
  expect(await (await request.get("/")).text()).toContain("Translations:");
  expect(await (await request.get("/cy-gb/")).text()).toContain("Cyfieithiadau:");
  expect(await (await request.get("/zh-cn/")).text()).toContain("翻译：");
});

test("the wordmark and the draft notice follow the locale", async ({ page }) => {
  await open(page, "/cy-gb/", "light");
  await expect(page.locator(".brand")).toHaveText("Rheoli Portffolio Prosiectau");
  await expect(page.locator("footer .draft")).toContainText("drafft");
  await open(page, "/en-gb/", "light");
  await expect(page.locator(".brand")).toHaveText("Project Portfolio Management");
  await expect(page.locator("footer .draft")).toHaveCount(0);
});

test("the locale picker opens the same topic under its own slug, by peer id", async ({ page }) => {
  await open(page, "/en-001/features/", "light");
  await chooseLocale(page, "Language", /^Cymraeg - /);
  await expect(page).toHaveURL(/\/cy-gb\/nodweddion\/$/);
  await expect(page.locator("h1").first()).toHaveText("Nodweddion");
  await chooseLocale(page, "Iaith", /^中文 - /);
  await expect(page.locator("h1").first()).toHaveText("功能");
  expect(decodeURI(page.url())).toMatch(/\/zh-cn\/功能\/$/);
  // A Hindi reader has no topics: the briefs stand in, and English site words say so.
  await chooseLocale(page, "语言", /^हिन्दी - /);
  await expect(page).toHaveURL(/\/hi-in\/for\/$/);
});

test("the locale picker on / opens that locale's home page", async ({ page }) => {
  await open(page, "/", "light");
  await chooseLocale(page, "Language", /^中文$/);
  await expect(page).toHaveURL(/\/zh-001\/$/);
  await expect(page.locator("h1").first()).toHaveText("项目组合管理");
});

test("a home page's and a topic's source files follow the rule", () => {
  const root = "../locales";
  const peers = new Map<string, Set<string>>();
  const check = (dir: string, key: string) => {
    expect(lstatSync(`${dir}/index.md`).isFile(), dir).toBe(true);
    expect(readlinkSync(`${dir}/README.md`), dir).toBe("index.md");
    const id = readFileSync(`${dir}/.locale-peer-id`, "utf8");
    expect(id, dir).toMatch(/^[0-9a-f]{32}\n$/);
    peers.set(key, (peers.get(key) ?? new Set()).add(id));
  };
  for (const locale of HOME_LOCALES) {
    check(`${root}/${locale}`, "home");
    for (const key of TOPIC_KEYS) check(`${root}/${locale}/topics/${topicSlug(locale, key)}`, key);
  }
  for (const [key, ids] of peers) expect(ids.size, key).toBe(1);
  expect(new Set([...peers.values()].flatMap((ids) => [...ids])).size).toBe(peers.size);
  // cy-gb is copied to cy-001 and zh-cn to zh-001.
  for (const [from, to] of [["cy-gb", "cy-001"], ["zh-cn", "zh-001"]] as const) {
    expect(readFileSync(`${root}/${from}/index.md`, "utf8")).toBe(readFileSync(`${root}/${to}/index.md`, "utf8"));
    for (const key of TOPIC_KEYS) {
      const path = (l: string) => `${root}/${l}/topics/${topicSlug(l, key)}/index.md`;
      expect(readFileSync(path(from), "utf8"), key).toBe(readFileSync(path(to), "utf8"));
    }
  }
  // Slugs are in the language, not merely English.
  for (const locale of ["cy-gb", "zh-cn"])
    for (const key of TOPIC_KEYS) expect(topicSlug(locale, key), key).not.toBe(key);
});

test("the English dialects spell as the rule asks", () => {
  const read = (l: string) =>
    [`../locales/${l}/index.md`, ...TOPIC_KEYS.map((k) => `../locales/${l}/topics/${k}/index.md`)]
      .map((f) => readFileSync(f, "utf8"))
      .join("\n");
  expect(read("en-us")).not.toMatch(/organis|programme|licence|levelling/i);
  expect(read("en-gb")).not.toMatch(/organiz/i);
  expect(read("en-gb")).toMatch(/programme/);
  expect(read("en-gb-oxendict")).toMatch(/organization/);
  expect(read("en-gb-oxendict")).toMatch(/programme/);
});

test("the sitemaps list every page and only their locale's", async ({ request }) => {
  const res = await request.get("/sitemap.xml");
  expect(res.status()).toBe(200);
  const xml = await res.text();
  expect(xml.startsWith("<?xml")).toBe(true);
  const base = "https://project-portfolio-management.github.io";
  for (const route of [...homeRoutes, ...topicRoutes, ...compareRoutes, ...docRoutes, ...indexRoutes, "/search/"])
    expect(xml, route).toContain(`<loc>${base}${encodeURI(route)}</loc>`);
  for (const route of briefRoutes.slice(0, 3)) expect(xml, route).toContain(route);

  for (const locale of BRIEF_LOCALES) {
    const own = await (await request.get(`/${locale}/sitemap.xml`)).text();
    const urls = [...own.matchAll(/<loc>([^<]+)<\/loc>/g)].map((m) => m[1] ?? "");
    expect(urls.length, locale).toBeGreaterThan(0);
    for (const url of urls) expect(url.startsWith(`${base}/${locale}/`), url).toBe(true);
  }
});

for (const name of PICKERS)
  test(`the ${name} button changes colour on hover, and its dropdown opens beneath it without moving the page`, async ({
    page,
  }) => {
    await open(page, "/", "light");
    const button = page.locator(".picker-bar").getByRole("button", { name });
    const looks = () =>
      button.evaluate((e) => {
        const c = getComputedStyle(e);
        return `${c.backgroundColor} ${c.color} ${c.borderColor} ${c.outlineColor}`;
      });
    await page.mouse.move(0, 0);
    const before = await looks();
    await button.hover();
    await expect.poll(looks).not.toBe(before);

    const main = page.locator("main");
    const top = (await main.boundingBox())?.y;
    await button.click();
    const box = await button.boundingBox();
    const popup = page.locator(".picker-bar [role=listbox]:visible, .picker-bar [role=menu]:visible, .picker-bar [role=search]:visible, .picker-bar ul:visible").first();
    await expect(popup).toBeVisible();
    const under = await popup.boundingBox();
    expect(under && box && under.y >= box.y + box.height - 1, name).toBe(true);
    expect((await main.boundingBox())?.y).toBe(top);
  });

test("a comparison page keeps the honest parts", async ({ page }) => {
  for (const product of COMPARE_PRODUCTS) {
    await page.goto(`/compare/${product}/`);
    const text = (await page.locator("main").innerText()).replace(/\s+/g, " ");
    expect(text, product).toMatch(/Nobody installed or ran/);
    expect(text, product).toMatch(/not ready for unsupervised production use/);
    expect(text, product).toMatch(/not affiliated with or endorsed by/);
    expect(text, product).toMatch(/may suit you better/);
  }
  await page.goto("/compare/trello/");
  await expect(page.locator("main")).toContainText("no Trello importer");
  await expect(page.locator("h1").first()).toContainText("Trello");
});

test("the comparisons page leads to every product, and /compare/ lists them", async ({ page }) => {
  for (const route of ["/compare/", "/en-001/comparisons/", "/cy-gb/cymariaethau/", "/zh-cn/比较/"]) {
    await page.goto(route);
    for (const product of COMPARE_PRODUCTS)
      await expect(page.locator(`main a[href$="/compare/${product}/"]`).first(), `${route} ${product}`).toBeVisible();
  }
});

test("a comparison page shows the note that it is in English to a Welsh reader", async ({ page }) => {
  await open(page, "/readme/", "light");
  await chooseLocale(page, "Language", /Cymraeg/);
  await page.goto("/compare/jira/");
  await expect(page.locator(".english-only")).toContainText("Saesneg");
});

test("links between the site's own documents stay on the site; others go to the repository", async ({ page }) => {
  await page.goto("/docs/");
  const readme = page.locator("main a", { hasText: "README.md" }).first();
  await expect(readme).toHaveAttribute("href", /readme\/$/);
  const tasks = page.locator("main a", { hasText: "tasks.md" }).first();
  await expect(tasks).toHaveAttribute(
    "href",
    "https://github.com/project-portfolio-management/project-portfolio-management/blob/main/tasks.md",
  );
});

for (const route of ["/", "/readme/", "/en-gb/for/", "/zh-cn/for/chief-executive-officer/"])
  for (const scheme of ["light", "dark"] as const)
    test(`${route} under dir=rtl mirrors without overflow and passes axe (${scheme})`, async ({ page }) => {
      await open(page, route, scheme);
      await page.evaluate(() => {
        document.documentElement.dir = "rtl";
        document.querySelector("header")?.setAttribute("dir", "rtl");
      });
      const overflow = await page.evaluate(
        () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
      );
      expect(overflow).toBeLessThanOrEqual(0);
      // The brand sits at the inline start: on the right under rtl.
      const brand = await page.locator(".brand").boundingBox();
      const width = page.viewportSize()?.width ?? 0;
      expect(brand && brand.x + brand.width).toBeGreaterThan(width / 2);
      expect(await axe(page)).toEqual([]);
    });

test("at phone width no page scrolls sideways", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  for (const route of [...docRoutes, "/en-gb/for/", "/hi-in/for/chief-technology-officer/", "/search/?portfolio"]) {
    await page.goto(route);
    const overflow = await page.evaluate(
      () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
    );
    expect(overflow, route).toBeLessThanOrEqual(0);
  }
});

test("no element carries a style attribute (the CSP allows only SvelteKit's announcer)", async ({ page }) => {
  for (const route of ["/", "/spec/", "/cy-001/for/chief-legal-officer/"]) {
    await open(page, route, "dark");
    const styled = await page.evaluate(() =>
      [...document.querySelectorAll("[style]")]
        .filter((e) => e.id !== "svelte-announcer")
        .map((e) => e.outerHTML.slice(0, 120)),
    );
    expect(styled, route).toEqual([]);
  }
});
