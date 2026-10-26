<!--
  PasteTasks (BR-15) — paste rows copied from a spreadsheet and turn the good ones into tasks of this plan.
  The rules are in `#lib/paste-view.ts`. Every row is shown with its problems; only the good rows go on. The first
  button asks the service what it would do (the import's dry run, which writes nothing); the second does it. Pasting
  the same rows again updates the tasks it made before and never duplicates them.
-->
<script lang="ts">
  import DataTable from "#lib/components/DataTable.svelte";
  import { PpmClient, type TrackerImportReport } from "#lib/api/ppm.js";
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { summaryText, unmappedGroups } from "#lib/import-view.js";
  import { counts, parsePasted, toImportCsv, type Problem } from "#lib/paste-view.js";

  let { pid, onDone }: { pid: string; onDone?: () => void } = $props();

  const ppm = PpmClient.withFetch();
  let text = $state("");
  let preview = $state<TrackerImportReport | null>(null);
  let done = $state<TrackerImportReport | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);

  const pasted = $derived(parsePasted(text));
  const tally = $derived(counts(pasted.rows));

  const say = (p: Problem): string => {
    switch (p.key) {
      case "no_title": return t("c.PasteTasks.problem_no_title");
      case "title_long": return t("c.PasteTasks.problem_title_long", { max: p.max });
      case "date_bad": return t("c.PasteTasks.problem_date_bad", { value: p.value });
      case "notes_long": return t("c.PasteTasks.problem_notes_long", { max: p.max });
      case "same_as": return t("c.PasteTasks.problem_same_as", { line: p.line });
      case "beyond_limit": return t("c.PasteTasks.problem_beyond_limit", { max: p.max });
    }
  };

  const columns = $derived([
    { id: "line", header: t("c.PasteTasks.col_line"), align: "end" as const },
    { id: "title", header: t("c.PasteTasks.col_title"), rowHeader: true },
    { id: "due", header: t("c.PasteTasks.col_due") },
    { id: "notes", header: t("c.PasteTasks.col_notes") },
    { id: "status", header: t("c.PasteTasks.col_status") },
  ]);
  const tableRows = $derived(
    pasted.rows.map((r) => ({
      line: r.line,
      title: r.title,
      due: r.due ?? "",
      notes: r.notes,
      status:
        r.problems.length === 0
          ? t("c.PasteTasks.ready")
          : t("c.PasteTasks.left_out", { why: r.problems.map(say).join("; ") }),
    })),
  );

  // Typing or pasting starts again: what the service said belongs to the rows it was asked about. (Done on input,
  // not in an effect, so clearing the box after a successful create does not wipe the "Done" report.)
  function restart() {
    preview = done = null;
    error = null;
  }

  async function go() {
    busy = true;
    try {
      const csv = toImportCsv(pasted.rows);
      if (!preview) {
        preview = await ppm.importTracker(pid, "asana", csv, true);
      } else {
        done = await ppm.importTracker(pid, "asana", csv, false);
        preview = null;
        text = "";
        onDone?.();
      }
      error = null;
    } catch (err) {
      error = errorText(err, "c.PasteTasks.failed");
    } finally {
      busy = false;
    }
  }
  const shown = $derived(done ?? preview);
  const label = $derived(
    !preview
      ? t("c.PasteTasks.check")
      : tally.good === 1
        ? t("c.PasteTasks.create_one")
        : t("c.PasteTasks.create_other", { n: tally.good }),
  );
</script>

<details class="surface paste-tasks" open={text !== "" || done !== null}>
  <summary>{t("c.PasteTasks.title")}</summary>
  <div class="stack">
    <p class="small" id="paste-help">{t("c.PasteTasks.help")}</p>
    <label>
      {t("c.PasteTasks.label")}
      <textarea bind:value={text} oninput={restart} rows="6" aria-describedby="paste-help" spellcheck="false"></textarea>
    </label>
    {#if error}<p class="banner error" role="alert">{error}</p>{/if}
    {#if pasted.rows.length > 0}
      <p role="status">{t("c.PasteTasks.tally", { good: tally.good, left: tally.left })}</p>
      <DataTable label={t("c.PasteTasks.title")} {columns} rows={tableRows} rowId={(r) => String(r.line)} sortable={false} exportable={false} testid="paste-rows" />
      <div class="row">
        <button class="button" type="button" disabled={busy || tally.good === 0} onclick={go}>{label}</button>
        <button class="button" type="button" onclick={() => { text = ""; restart(); }}>{t("c.PasteTasks.clear")}</button>
      </div>
    {/if}
    {#if shown}
      <section aria-labelledby="paste-report" data-testid="paste-report">
        <h3 id="paste-report">{done ? t("c.PasteTasks.done") : t("c.PasteTasks.would")}</h3>
        <p role="status">{summaryText(shown)}</p>
        {#each unmappedGroups(shown.unmapped) as g (g.title)}
          <h4>{g.title}</h4>
          <ul>{#each g.reasons as r (r)}<li>{r}</li>{/each}</ul>
        {/each}
      </section>
    {/if}
  </div>
</details>
