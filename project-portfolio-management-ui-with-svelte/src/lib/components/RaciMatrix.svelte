<!--
  The RACI matrix: tasks down, people across, and in each cell four toggles —
  R Responsible, A Accountable, C Consulted, I Informed. Every toggle is a real
  button with a sentence ("Design: ann is Accountable"), pressed or not; the
  letters are visible text, so colour is never the only signal. Giving someone A
  takes it from whoever had it (one Accountable per task) and the page says so.
  A row that is missing an Accountable or a Responsible is named in words.
-->
<script lang="ts">
  import type { RaciMatrix } from "#lib/api/ppm.js";
  import {
    LETTERS,
    LETTER_WORD,
    cellSentence,
    findingText,
    shortRef,
    toggleLetter,
    type Letter,
  } from "#lib/raci-view.js";

  let {
    matrix,
    people,
    onchange,
  }: {
    matrix: RaciMatrix;
    /** The columns to show (everyone with a letter, plus any added and still empty). */
    people: string[];
    /** Called with the task and its new cells when a toggle changes a row. */
    onchange: (taskPid: string, cells: Record<string, string>, notice: string | null) => void;
  } = $props();

  function toggle(taskPid: string, cells: Record<string, string>, person: string, letter: Letter) {
    const t = toggleLetter(cells, person, letter);
    onchange(
      taskPid,
      t.cells,
      t.accountableMovedFrom ? `Accountable moved from ${shortRef(t.accountableMovedFrom)} to ${shortRef(person)}: a task has one.` : null,
    );
  }
</script>

<div class="scroll">
  <table data-testid="raci-matrix">
    <caption class="sr-only">Who does what (RACI matrix): who is Responsible, Accountable, Consulted and Informed on each task</caption>
    <thead>
      <tr>
        <th scope="col">Task</th>
        {#each people as p (p)}
          <th scope="col" title={p}>{shortRef(p)}</th>
        {/each}
        <th scope="col">Gaps</th>
      </tr>
    </thead>
    <tbody>
      {#each matrix.rows as row (row.task_pid)}
        <tr class:gap={row.findings.length > 0}>
          <th scope="row">{row.title}</th>
          {#each people as p (p)}
            <td>
              <span class="toggles" role="group" aria-label={cellSentence(row.title, p, row.cells[p] ?? "")}>
                {#each LETTERS as l (l)}
                  {@const on = (row.cells[p] ?? "").includes(l)}
                  <button
                    type="button"
                    class="l"
                    class:on
                    aria-pressed={on}
                    aria-label={LETTER_WORD[l]}
                    title={LETTER_WORD[l]}
                    onclick={() => toggle(row.task_pid, row.cells, p, l)}>{l}</button
                  >
                {/each}
              </span>
            </td>
          {/each}
          <td class="small">
            {#if row.findings.length === 0}<span aria-label="complete">✓</span>{:else}
              {#each row.findings as f (f)}<span class="gap-text">⚠ {findingText(f)}. </span>{/each}
            {/if}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  .scroll {
    overflow-x: auto;
  }
  table {
    border-collapse: collapse;
    font-size: 0.9em;
  }
  th,
  td {
    border: 1px solid var(--border, #ccc);
    padding: 0.25rem 0.4rem;
    text-align: start;
    vertical-align: middle;
  }
  tr.gap th[scope="row"] {
    border-inline-start: 4px solid #b45309;
  }
  .toggles {
    display: inline-flex;
    gap: 2px;
  }
  .l {
    min-width: 1.7rem;
    min-height: 1.7rem;
    border: 1px solid currentColor;
    background: none;
    font: inherit;
    font-weight: 700;
    cursor: pointer;
    opacity: 0.55;
  }
  .l.on {
    opacity: 1;
    background: var(--mxi-color-focus, #1d4ed8);
    color: #fff;
    border-color: transparent;
  }
  .l:focus-visible {
    outline: 3px solid var(--mxi-color-focus, #1d4ed8);
    outline-offset: 1px;
  }
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    overflow: hidden;
    clip-path: inset(50%);
    white-space: nowrap;
  }
</style>
