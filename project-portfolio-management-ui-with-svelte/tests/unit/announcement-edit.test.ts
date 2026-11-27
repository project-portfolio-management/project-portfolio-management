// Changing an announcement (NC-1 slice 12 part n): what the form sends to `PUT /api/announcements/{pid}`.

import { describe, expect, it } from "vitest";
import {
  announcementEdit,
  formOf,
  localInput,
} from "../../src/lib/announcement-view";
import type { Announcement } from "../../src/lib/api/ppm";

const a: Announcement = {
  pid: "a1",
  title: "Maintenance",
  body: "Down on Sunday",
  severity: "important",
  pinned: false,
  publish_at: "2026-11-27T09:00:30Z",
  expires_at: "2026-11-30T18:00:00Z",
  author_ref: null,
  state: "live",
} as Announcement;

describe("localInput", () => {
  it("writes an instant for a datetime-local input, to the minute, and none as empty", () => {
    const v = localInput(a.publish_at);
    expect(v).toMatch(/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$/);
    expect(new Date(v).getTime()).toBe(
      new Date("2026-11-27T09:00:00Z").getTime(),
    );
    expect(localInput(null)).toBe("");
    expect(localInput("soon")).toBe("");
  });
});

describe("announcementEdit", () => {
  it("sends nothing for an unchanged form, seconds on the stored times included", () => {
    expect(announcementEdit(formOf(a), a)).toBeNull();
  });
  it("sends the whole announcement, the expiry always and the publishing time only when it changed", () => {
    const f = { ...formOf(a), title: " Maintenance window ", pinned: true };
    expect(announcementEdit(f, a)).toEqual({
      body: {
        title: "Maintenance window",
        body: "Down on Sunday",
        severity: "important",
        pinned: true,
        expires_at: a.expires_at,
      },
    });
    const later = { ...formOf(a), publish: localInput("2026-11-28T09:00:00Z") };
    expect(announcementEdit(later, a)).toEqual({
      body: {
        title: "Maintenance",
        body: "Down on Sunday",
        severity: "important",
        pinned: false,
        publish_at: "2026-11-28T09:00:00.000Z",
        expires_at: a.expires_at,
      },
    });
  });
  it("clears an expiry left empty, which the service would otherwise do unasked", () => {
    expect(announcementEdit({ ...formOf(a), expires: "" }, a)).toHaveProperty(
      "body.expires_at",
      null,
    );
  });
  it("says every problem first", () => {
    expect(
      announcementEdit({ ...formOf(a), title: " ", body: "x".repeat(4001) }, a),
    ).toEqual({
      problems: ["p.announcements.title_bad", "p.announcements.body_bad"],
    });
    expect(
      announcementEdit(
        { ...formOf(a), expires: localInput("2026-11-26T09:00:00Z") },
        a,
      ),
    ).toEqual({
      problems: ["p.announcements.expires_bad"],
    });
  });
});
