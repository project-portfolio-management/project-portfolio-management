// NC-1 slice 5 (LT-4, LT-5, RM-3): the checks before posting a comment or sending a file, the type to send, the size
// words, and the refusal sentences. The rules mirror the service's `src/discussion.rs` and `src/artifacts.rs`; the
// service still decides.

import { describe, expect, it } from "vitest";
import {
  ACCEPT,
  ALLOWED_TYPES,
  MAX_ATTACHMENT_BYTES,
  MAX_COMMENT_CHARS,
  REACTIONS,
  charsLeft,
  commentProblem,
  discussionRefusalKey,
  fileProblem,
  reactionCount,
  sizeParts,
  typeWord,
  uploadType,
} from "../../src/lib/discussion-view";

describe("comment checks", () => {
  it("refuses a blank comment and one over 4000 characters, counted by code point as the service counts", () => {
    expect(commentProblem("")).toBe("blank");
    expect(commentProblem("   \n ")).toBe("blank");
    expect(commentProblem("Looks good")).toBeNull();
    expect(commentProblem("a".repeat(MAX_COMMENT_CHARS))).toBeNull();
    expect(commentProblem("a".repeat(MAX_COMMENT_CHARS + 1))).toBe("too_long");
    // An emoji is two UTF-16 units but one character to the service.
    expect(commentProblem("😀".repeat(MAX_COMMENT_CHARS))).toBeNull();
    expect(charsLeft("😀😀")).toBe(MAX_COMMENT_CHARS - 2);
    expect(charsLeft("a".repeat(MAX_COMMENT_CHARS + 3))).toBe(-3);
  });
});

describe("file checks", () => {
  const file = (name: string, size: number, type: string) => ({
    name,
    size,
    type,
  });

  it("takes the service's twelve types and nothing that a browser could run or draw as a page", () => {
    expect(Object.keys(ALLOWED_TYPES)).toHaveLength(12);
    for (const refused of [
      "text/html",
      "image/svg+xml",
      "application/zip",
      "application/x-msdownload",
      "application/javascript",
    ])
      expect(refused in ALLOWED_TYPES).toBe(false);
    expect(ACCEPT).toContain("application/pdf");
    expect(ACCEPT).toContain(".md");
    expect(ACCEPT).not.toContain("html");
  });

  it("refuses no file, an empty one, one over 10 MiB, and a type not on the list", () => {
    expect(fileProblem(null)).toBe("choose_file");
    expect(fileProblem(file("a.pdf", 0, "application/pdf"))).toBe("empty");
    expect(
      fileProblem(file("a.pdf", MAX_ATTACHMENT_BYTES, "application/pdf")),
    ).toBeNull();
    expect(
      fileProblem(file("a.pdf", MAX_ATTACHMENT_BYTES + 1, "application/pdf")),
    ).toBe("too_large");
    expect(fileProblem(file("page.html", 10, "text/html"))).toBe("type");
    expect(fileProblem(file("logo.svg", 10, "image/svg+xml"))).toBe("type");
    expect(fileProblem(file("tool", 10, ""))).toBe("type");
  });

  it("sends the browser's type, and names a text file's type from its name only when the browser gave none", () => {
    expect(uploadType(file("notes.md", 5, ""))).toBe("text/markdown");
    expect(uploadType(file("DATA.CSV", 5, ""))).toBe("text/csv");
    expect(uploadType(file("a.json", 5, ""))).toBe("application/json");
    expect(uploadType(file("a.png", 5, "image/PNG"))).toBe("image/png");
    expect(uploadType(file("a.txt", 5, "text/plain; charset=utf-8"))).toBe(
      "text/plain",
    );
    // A name never turns a type the browser named into another one.
    expect(uploadType(file("page.md", 5, "text/html"))).toBe("text/html");
    expect(fileProblem(file("page.md", 5, "text/html"))).toBe("type");
    expect(uploadType(file("a.exe", 5, ""))).toBe("");
  });

  it("words a stored type, and leaves one it does not know as written", () => {
    expect(typeWord("application/pdf")).toBe("pdf");
    expect(
      typeWord(
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
      ),
    ).toBe("excel");
    expect(typeWord("application/x-unknown")).toBeNull();
  });

  it("sizes in bytes, KB and MB, 1024-based", () => {
    expect(sizeParts(512)).toEqual({ unit: "b", value: 512 });
    expect(sizeParts(1536)).toEqual({ unit: "kb", value: 1.5 });
    expect(sizeParts(MAX_ATTACHMENT_BYTES)).toEqual({ unit: "mb", value: 10 });
  });
});

describe("reactions", () => {
  it("are the service's closed set of five, and an absent count is zero", () => {
    expect(REACTIONS.map((r) => r.kind)).toEqual([
      "thumbs_up",
      "heart",
      "celebrate",
      "eyes",
      "question",
    ]);
    expect(reactionCount({ heart: 2 }, "heart")).toBe(2);
    expect(reactionCount({ heart: 2 }, "eyes")).toBe(0);
  });
});

describe("refusal sentences", () => {
  it("names the author rule, the uploader rule, a gone item and each 422", () => {
    expect(discussionRefusalKey("watch", 401)).toBe(
      "p.discussion.refused_sign_in",
    );
    expect(discussionRefusalKey("edit_comment", 403)).toBe(
      "p.discussion.refused_not_author",
    );
    expect(discussionRefusalKey("delete_comment", 403)).toBe(
      "p.discussion.refused_not_author",
    );
    expect(discussionRefusalKey("remove_file", 403)).toBe(
      "p.discussion.refused_not_uploader",
    );
    expect(discussionRefusalKey("comment", 403)).toBe(
      "p.discussion.refused_forbidden",
    );
    expect(discussionRefusalKey("delete_comment", 404)).toBe(
      "p.discussion.refused_comment_gone",
    );
    expect(discussionRefusalKey("remove_file", 404)).toBe(
      "p.discussion.refused_file_gone",
    );
    expect(discussionRefusalKey("react", 404)).toBe(
      "p.discussion.refused_gone",
    );
    expect(discussionRefusalKey("read_history", 404)).toBe(
      "p.discussion.refused_comment_gone",
    );
    expect(discussionRefusalKey("read_history", 403)).toBe(
      "p.discussion.refused_forbidden",
    );
    expect(discussionRefusalKey("comment", 422)).toBe(
      "p.discussion.refused_comment_invalid",
    );
    expect(discussionRefusalKey("upload", 422)).toBe(
      "p.discussion.refused_file_invalid",
    );
    expect(discussionRefusalKey("upload", 413)).toBe(
      "p.discussion.refused_too_large",
    );
    expect(discussionRefusalKey("upload", 503)).toBe(
      "p.discussion.refused_scan_unavailable",
    );
  });

  it("falls back to the generic text for anything else", () => {
    expect(discussionRefusalKey("react", 422)).toBeNull();
    expect(discussionRefusalKey("comment", 500)).toBeNull();
    expect(discussionRefusalKey("comment", 503)).toBeNull();
  });
});
