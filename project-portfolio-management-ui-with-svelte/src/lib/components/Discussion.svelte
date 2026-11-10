<!--
  The talk and the files on one subject (NC-1 slice 5): following it (RM-3), reactions and comments (LT-4), and
  attachments (LT-5). Used on the task page; written for any subject kind the service takes. The rules are in
  `#lib/discussion-view.js`; the service decides. Nothing a person uploaded is ever drawn in the page: a file is a link
  to the service's own download, which says `Content-Disposition: attachment` and `nosniff`, so the browser saves it.
  Refusals say the catalogue's sentence, never the service's own text.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { resource } from "#lib/resource.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { t, tn, type StringKey } from "#lib/i18n.svelte.js";
  import { formatDateTime, formatNumber } from "#lib/locale-format.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { ApiError } from "#lib/api/client.js";
  import {
    PpmClient,
    type Attachment,
    type Comment,
    type CommentEdit,
    type DiscussionSubjectKind,
    type ReactionKind,
    type ReactionSummary,
    type WatchStatus,
  } from "#lib/api/ppm.js";
  import {
    ACCEPT,
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
    type CommentProblem,
    type DiscussionStep,
    type FileProblem,
    type FileTypeWord,
  } from "#lib/discussion-view.js";

  let { subjectKind, subjectPid }: { subjectKind: DiscussionSubjectKind; subjectPid: string } = $props();

  const ppm = PpmClient.withFetch();
  const comments = resource<Comment[]>(() => ppm.comments(subjectKind, subjectPid), "p.discussion.could_not_load_comments");
  const reactions = resource<ReactionSummary>(() => ppm.reactions(subjectKind, subjectPid), "p.discussion.could_not_load_reactions");
  const files = resource<Attachment[]>(() => ppm.attachments(subjectKind, subjectPid), "p.discussion.could_not_load_files");

  // Following answers 401 without a sign-in, which is not a fault: the page says to sign in instead.
  let watch = $state<WatchStatus | null>(null);
  let watchSignedOut = $state(false);
  let watchError = $state<string | null>(null);

  let draft = $state("");
  let editing = $state<string | null>(null);
  let editDraft = $state("");
  let history = $state<Record<string, CommentEdit[]>>({});
  let chosen = $state<File | null>(null);
  let fileInput = $state<HTMLInputElement | null>(null);
  let notice = $state("");
  let actionError = $state<string | null>(null);
  let busy = $state(false);

  const left = $derived(charsLeft(draft));

  const COMMENT_PROBLEM: Record<NonNullable<CommentProblem>, StringKey> = {
    blank: "p.discussion.problem_blank",
    too_long: "p.discussion.problem_too_long",
  };
  const FILE_PROBLEM: Record<NonNullable<FileProblem>, StringKey> = {
    choose_file: "p.discussion.problem_choose_file",
    empty: "p.discussion.problem_empty",
    too_large: "p.discussion.problem_too_large",
    type: "p.discussion.problem_type",
  };
  const REACTION_WORDS: Record<ReactionKind, StringKey> = {
    thumbs_up: "p.discussion.r_thumbs_up",
    heart: "p.discussion.r_heart",
    celebrate: "p.discussion.r_celebrate",
    eyes: "p.discussion.r_eyes",
    question: "p.discussion.r_question",
  };
  const TYPE_WORDS: Record<FileTypeWord, StringKey> = {
    pdf: "p.discussion.type_pdf",
    png: "p.discussion.type_png",
    jpeg: "p.discussion.type_jpeg",
    gif: "p.discussion.type_gif",
    webp: "p.discussion.type_webp",
    text: "p.discussion.type_text",
    csv: "p.discussion.type_csv",
    markdown: "p.discussion.type_markdown",
    json: "p.discussion.type_json",
    word: "p.discussion.type_word",
    excel: "p.discussion.type_excel",
    powerpoint: "p.discussion.type_powerpoint",
  };
  const SIZE_WORDS = { b: "p.discussion.size_b", kb: "p.discussion.size_kb", mb: "p.discussion.size_mb" } as const;

  function sizeText(bytes: number): string {
    const s = sizeParts(bytes);
    return t(SIZE_WORDS[s.unit], { n: formatNumber(s.value, { maximumFractionDigits: 1 }) });
  }
  function typeText(contentType: string): string {
    const w = typeWord(contentType);
    return w ? t(TYPE_WORDS[w]) : contentType;
  }
  const who = (sub: string | null): string => sub ?? t("p.discussion.no_author");

  async function loadWatch() {
    watchError = null;
    try {
      watch = await ppm.watchStatus(subjectKind, subjectPid);
      watchSignedOut = false;
    } catch (err) {
      if (err instanceof ApiError && err.status === 401) watchSignedOut = true;
      else watchError = errorText(err, "p.discussion.could_not_load_watch");
    }
  }

  onMount(() => {
    void comments.reload();
    void reactions.reload();
    void files.reload();
    void loadWatch();
  });

  /** Say why a step was refused, in the catalogue's words; read again what it was about. */
  async function refused(err: unknown, step: DiscussionStep) {
    const key = err instanceof ApiError ? discussionRefusalKey(step, err.status) : null;
    actionError = key ? t(key) : errorText(err, "ppm.common.actionFailed");
    if (err instanceof ApiError && err.status === 404) {
      if (step === "remove_file" || step === "upload") await files.reload();
      else if (step === "react") await reactions.reload();
      else if (step === "watch") await loadWatch();
      else await comments.reload();
    }
  }

  /** Run one step: clear the last message, hold the buttons, say what happened. */
  async function run(step: DiscussionStep, action: () => Promise<void>) {
    busy = true;
    try {
      await action();
    } catch (err) {
      await refused(err, step);
    } finally {
      busy = false;
    }
  }

  function begin() {
    actionError = null;
    notice = "";
  }

  async function post(e: SubmitEvent) {
    e.preventDefault();
    begin();
    const problem = commentProblem(draft);
    if (problem) {
      actionError = t(COMMENT_PROBLEM[problem]);
      return;
    }
    await run("comment", async () => {
      await ppm.addComment({ subject_kind: subjectKind, subject_pid: subjectPid, body: draft });
      draft = "";
      notice = t("p.discussion.posted");
      await comments.reload();
      // Commenting follows the thread unless the person switched that off.
      await loadWatch();
    });
  }

  function startEdit(c: Comment) {
    begin();
    editing = c.pid;
    editDraft = c.body;
  }

  async function saveEdit(e: SubmitEvent, c: Comment) {
    e.preventDefault();
    begin();
    const problem = commentProblem(editDraft);
    if (problem) {
      actionError = t(COMMENT_PROBLEM[problem]);
      return;
    }
    await run("edit_comment", async () => {
      await ppm.editComment(c.pid, { body: editDraft });
      editing = null;
      notice = t("p.discussion.edited");
      delete history[c.pid];
      await comments.reload();
    });
  }

  async function remove(c: Comment) {
    begin();
    if (!(await askConfirm(t("p.discussion.confirm_delete_comment")))) return;
    await run("delete_comment", async () => {
      await ppm.deleteComment(c.pid);
      notice = t("p.discussion.deleted");
      await comments.reload();
    });
  }

  async function toggleHistory(c: Comment) {
    begin();
    if (history[c.pid]) {
      delete history[c.pid];
      return;
    }
    await run("read_history", async () => {
      history[c.pid] = await ppm.commentHistory(c.pid);
    });
  }

  async function react(kind: ReactionKind) {
    begin();
    const mine = reactions.data?.mine.includes(kind) ?? false;
    await run("react", async () => {
      const body = { subject_kind: subjectKind, subject_pid: subjectPid, kind };
      if (mine) await ppm.removeReaction(body);
      else await ppm.addReaction(body);
      await reactions.reload();
    });
  }

  async function toggleWatch() {
    begin();
    const body = { subject_kind: subjectKind, subject_pid: subjectPid };
    const following = watch?.mine ?? false;
    await run("watch", async () => {
      if (following) await ppm.unwatch(body);
      else await ppm.watch(body);
      notice = t(following ? "p.discussion.unfollowed" : "p.discussion.followed");
      await loadWatch();
    });
  }

  function choose(e: Event) {
    const input = e.currentTarget as HTMLInputElement;
    chosen = input.files?.[0] ?? null;
  }

  async function upload(e: SubmitEvent) {
    e.preventDefault();
    begin();
    const problem = fileProblem(chosen);
    if (problem || !chosen) {
      actionError = t(FILE_PROBLEM[problem ?? "choose_file"]);
      return;
    }
    const file = chosen;
    await run("upload", async () => {
      const stored = await ppm.uploadAttachment(subjectKind, subjectPid, file, file.name, uploadType(file));
      notice = t("p.discussion.attached", { name: stored.filename });
      chosen = null;
      if (fileInput) fileInput.value = "";
      await files.reload();
    });
  }

  async function removeFile(a: Attachment) {
    begin();
    if (!(await askConfirm(t("p.discussion.confirm_remove_file", { name: a.filename })))) return;
    await run("remove_file", async () => {
      await ppm.removeAttachment(a.pid);
      notice = t("p.discussion.removed_file", { name: a.filename });
      await files.reload();
    });
  }
</script>

<section aria-labelledby="disc-h" class="discussion">
  <h2 id="disc-h">{t("p.discussion.heading")}</h2>
  {#if actionError}<p class="banner" role="alert">{actionError}</p>{/if}
  <p class="small" role="status">{notice}</p>

  <div class="follow" data-testid="follow">
    {#if watchSignedOut}
      <p class="small muted">{t("p.discussion.sign_in_to_follow")}</p>
    {:else if watchError}
      <p class="banner" role="alert">{watchError}</p>
    {:else if watch}
      <p class="small">
        {watch.mine ? t("p.discussion.you_follow") : t("p.discussion.you_do_not_follow")}
        {tn("p.discussion.followers", watch.count, { n: watch.count })}
      </p>
      <button class="button small" type="button" disabled={busy} onclick={() => void toggleWatch()}>
        {watch.mine ? t("p.discussion.unfollow") : t("p.discussion.follow")}
      </button>
    {/if}
  </div>

  {#if reactions.error}<p class="banner" role="alert">{reactions.error}</p>{/if}
  {#if reactions.data}
    {@const summary = reactions.data}
    <div class="reactions" role="group" aria-label={t("p.discussion.reactions_label")}>
      {#each REACTIONS as r (r.kind)}
        {@const n = reactionCount(summary.counts, r.kind)}
        <button
          class="button small"
          type="button"
          disabled={busy}
          aria-pressed={summary.mine.includes(r.kind)}
          aria-label={tn("p.discussion.reaction_named", n, { reaction: t(REACTION_WORDS[r.kind]), n })}
          onclick={() => void react(r.kind)}
        ><span aria-hidden="true">{r.symbol}</span> {formatNumber(n)}</button>
      {/each}
    </div>
  {/if}

  <h3 id="disc-comments-h">{t("p.discussion.comments_heading")}</h3>
  {#if comments.error}<p class="banner" role="alert">{comments.error}</p>{/if}
  {#if comments.data}
    {#if comments.data.length === 0}
      <p>{t("p.discussion.no_comments")}</p>
    {:else}
      <ol class="comments" data-testid="comments" aria-labelledby="disc-comments-h">
        {#each comments.data as c (c.pid)}
          <li>
            <p class="small muted">
              {t("p.discussion.by_at", { who: who(c.author_sub), when: formatDateTime(c.created_at) })}
              {#if c.edited_at}· {t("p.discussion.edited_at", { when: formatDateTime(c.edited_at) })}{/if}
            </p>
            {#if editing === c.pid}
              <form class="disc-form" onsubmit={(e) => void saveEdit(e, c)}>
                <label>{t("p.discussion.f_edit")}
                  <textarea bind:value={editDraft} rows="3"></textarea>
                </label>
                <div class="row">
                  <button class="button" type="submit" disabled={busy}>{t("p.discussion.save_edit")}</button>
                  <button class="button" type="button" onclick={() => (editing = null)}>{t("p.discussion.cancel")}</button>
                </div>
              </form>
            {:else}
              <p class="comment-body">{c.body}</p>
              <div class="row">
                <button class="button small" type="button" disabled={busy}
                  aria-label={t("p.discussion.edit_named", { who: who(c.author_sub), when: formatDateTime(c.created_at) })}
                  onclick={() => startEdit(c)}>{t("p.discussion.edit")}</button>
                <button class="button small" type="button" disabled={busy}
                  aria-label={t("p.discussion.delete_named", { who: who(c.author_sub), when: formatDateTime(c.created_at) })}
                  onclick={() => void remove(c)}>{t("p.discussion.delete")}</button>
                {#if c.edited_at}
                  <button class="button small" type="button" disabled={busy}
                    aria-expanded={history[c.pid] !== undefined}
                    onclick={() => void toggleHistory(c)}>{t("p.discussion.earlier")}</button>
                {/if}
              </div>
              {#if history[c.pid]}
                {@const versions = history[c.pid] ?? []}
                {#if versions.length === 0}
                  <p class="small muted">{t("p.discussion.no_earlier")}</p>
                {:else}
                  <ol class="history" aria-label={t("p.discussion.earlier")}>
                    {#each versions as v, i (i)}
                      <li>
                        <p class="small muted">{t("p.discussion.replaced_at", { when: formatDateTime(v.created_at) })}</p>
                        <p class="comment-body">{v.previous_body}</p>
                      </li>
                    {/each}
                  </ol>
                {/if}
              {/if}
            {/if}
          </li>
        {/each}
      </ol>
    {/if}
  {:else if !comments.error}
    <p class="muted">{t("p.discussion.loading")}</p>
  {/if}

  <form class="disc-form" onsubmit={post} novalidate>
    <label>{t("p.discussion.f_comment")}
      <textarea bind:value={draft} rows="3" aria-describedby="disc-left"></textarea>
    </label>
    <p id="disc-left" class="small muted">{left >= 0
        ? t("p.discussion.chars_left", { n: formatNumber(left), max: formatNumber(MAX_COMMENT_CHARS) })
        : t("p.discussion.chars_over", { n: formatNumber(-left), max: formatNumber(MAX_COMMENT_CHARS) })}</p>
    <div class="row">
      <button class="button" type="submit" disabled={busy}>{t("p.discussion.post")}</button>
    </div>
  </form>
</section>

<section aria-labelledby="files-h" class="discussion">
  <h2 id="files-h">{t("p.discussion.files_heading")}</h2>
  <p class="small muted">{t("p.discussion.files_intro")}</p>
  {#if files.error}<p class="banner" role="alert">{files.error}</p>{/if}
  {#if files.data}
    {#if files.data.length === 0}
      <p>{t("p.discussion.no_files")}</p>
    {:else}
      <ul class="files" data-testid="attachments">
        {#each files.data as a (a.pid)}
          <li>
            <a href={ppm.attachmentDownloadUrl(a.pid)} rel="noopener">{a.filename}</a>
            <span class="small muted">{t("p.discussion.file_facts", {
              type: typeText(a.content_type),
              size: sizeText(a.size_bytes),
              who: who(a.uploaded_by),
              when: formatDateTime(a.created_at),
            })}</span>
            <button class="button small" type="button" disabled={busy}
              aria-label={t("p.discussion.remove_file_named", { name: a.filename })}
              onclick={() => void removeFile(a)}>{t("p.discussion.remove_file")}</button>
          </li>
        {/each}
      </ul>
    {/if}
  {:else if !files.error}
    <p class="muted">{t("p.discussion.loading")}</p>
  {/if}

  <form class="disc-form" onsubmit={upload} novalidate>
    <label>{t("p.discussion.f_file")}
      <input type="file" accept={ACCEPT} bind:this={fileInput} onchange={choose} aria-describedby="files-rules" />
    </label>
    <p id="files-rules" class="small muted">{t("p.discussion.files_rules")}</p>
    <div class="row">
      <button class="button" type="submit" disabled={busy}>{t("p.discussion.attach")}</button>
    </div>
  </form>
</section>

<style>
  .discussion {
    margin-block: 1.5rem;
  }
  .disc-form {
    display: grid;
    gap: 0.5rem;
    max-inline-size: 36rem;
    margin-block: 1rem;
  }
  .disc-form label {
    display: grid;
    gap: 0.2rem;
  }
  .reactions,
  .follow {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
    margin-block: 0.5rem;
  }
  .comments,
  .files {
    display: grid;
    gap: 0.75rem;
    padding-inline-start: 1.25rem;
  }
  .comment-body {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
    margin-block: 0.25rem;
  }
  .history {
    padding-inline-start: 1.25rem;
    border-inline-start: 2px solid currentColor;
  }
</style>
