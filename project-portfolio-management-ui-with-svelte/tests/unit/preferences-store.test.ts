import { describe, it, expect, vi, beforeEach } from "vitest";

vi.mock("$app/env", () => ({ browser: true }));

const listSavedViews = vi.fn();
const createSavedView = vi.fn();
const deleteSavedView = vi.fn();
vi.mock("#lib/api/ppm.js", () => ({
  PpmClient: {
    withFetch: () => ({ listSavedViews, createSavedView, deleteSavedView }),
  },
}));

const STORAGE_KEY = "mxi.display";

async function fresh() {
  vi.resetModules();
  return (await import("../../src/lib/preferences.svelte")).display;
}

beforeEach(() => {
  localStorage.clear();
  listSavedViews.mockReset().mockResolvedValue([]);
  createSavedView.mockReset().mockResolvedValue({ pid: "new" });
  deleteSavedView.mockReset().mockResolvedValue(undefined);
});

describe("the display preferences store", () => {
  it("starts from what the device remembers", async () => {
    localStorage.setItem(
      STORAGE_KEY,
      JSON.stringify({ simple: true, dyslexia: true }),
    );
    const display = await fresh();
    expect(display.prefs).toEqual({ simple: true, dyslexia: true });
    expect(display.dyslexia).toBe(true);
  });

  it("signed out, a choice is kept on the device only", async () => {
    const display = await fresh();
    await display.set({ simple: true }, false);
    expect(display.prefs).toEqual({ simple: true });
    expect(JSON.parse(localStorage.getItem(STORAGE_KEY) ?? "{}")).toEqual({
      simple: true,
    });
    expect(display.status).toBe("device");
    expect(createSavedView).not.toHaveBeenCalled();
  });

  it("signed in, it replaces the user's own preferences view on the account", async () => {
    listSavedViews.mockResolvedValue([{ pid: "old1" }, { pid: "old2" }]);
    const display = await fresh();
    await display.set({ dyslexia: true }, true);
    expect(listSavedViews).toHaveBeenCalledWith("/__preferences");
    expect(deleteSavedView.mock.calls.map((c) => c[0])).toEqual([
      "old1",
      "old2",
    ]);
    expect(createSavedView).toHaveBeenCalledWith({
      route: "/__preferences",
      name: "display",
      filter: { dyslexia: true },
      sort: {},
      columns: [],
    });
    expect(display.status).toBe("account");
  });

  it("if the account cannot be reached the choice still holds on the device, and it says so", async () => {
    listSavedViews.mockRejectedValue(new Error("down"));
    const display = await fresh();
    await display.set({ simple: false }, true);
    expect(display.prefs).toEqual({ simple: false });
    expect(JSON.parse(localStorage.getItem(STORAGE_KEY) ?? "{}")).toEqual({
      simple: false,
    });
    expect(display.status).toBe("account-failed");
  });

  it("clearing a choice returns it to the default for the role", async () => {
    const display = await fresh();
    await display.set({ simple: true, dyslexia: true }, false);
    await display.set({ simple: undefined }, false);
    expect(display.prefs).toEqual({ dyslexia: true });
  });

  it("on sign-in it adopts what the account holds", async () => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify({ simple: false }));
    listSavedViews.mockResolvedValue([
      {
        pid: "p",
        route: "/__preferences",
        name: "display",
        filter: { simple: true, dyslexia: true },
      },
    ]);
    const display = await fresh();
    await display.sync(true);
    expect(display.prefs).toEqual({ simple: true, dyslexia: true });
    expect(JSON.parse(localStorage.getItem(STORAGE_KEY) ?? "{}")).toEqual({
      simple: true,
      dyslexia: true,
    });
  });

  it("on sign-in with nothing on the account, the device's choices are kept there", async () => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify({ dyslexia: true }));
    const display = await fresh();
    await display.sync(true);
    expect(createSavedView).toHaveBeenCalledOnce();
    expect(display.prefs).toEqual({ dyslexia: true });
  });

  it("does not touch the account when signed out, and syncs once per sign-in state", async () => {
    const display = await fresh();
    await display.sync(false);
    expect(listSavedViews).not.toHaveBeenCalled();
    await display.sync(true);
    await display.sync(true);
    expect(listSavedViews).toHaveBeenCalledTimes(1);
  });
});
