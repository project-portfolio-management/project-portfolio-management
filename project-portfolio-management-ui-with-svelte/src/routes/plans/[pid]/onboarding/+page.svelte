<!--
  Onboarding and offboarding (`/plans/[pid]/onboarding`): checklists for someone
  joining or leaving the plan and for taking the plan in or closing it out, what
  stands between the plan and being closed, and the record of access granted and
  revoked. Status is a word and a shape, never colour alone. The access record is
  a hand-kept record: this service cannot change anyone's real accounts. Anything
  about a person needs the permission to manage people; without it the page says so.
-->
<script lang="ts">
  import { localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { ApiError } from "#lib/api/client.js";
  import {
    PpmClient,
    type AccessRecords,
    type Closeout,
    type OnboardingCase,
    type OnboardingCaseSummary,
    type OnboardingKind,
  } from "#lib/api/ppm.js";
  import {
    KIND_TEXT,
    accessBody,
    caseBody,
    closeoutText,
    isPersonKind,
    itemState,
    progressText,
  } from "#lib/onboarding-view.js";

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");
  let cases = $state<OnboardingCaseSummary[]>([]);
  let hidden = $state(0);
  let open = $state<OnboardingCase | null>(null);
  let closeout = $state<Closeout | null>(null);
  let access = $state<AccessRecords | null>(null);
  let accessDenied = $state(false);
  let error = $state<string | null>(null);
  let status = $state("");

  async function load() {
    try {
      const r = await ppm.onboardingCases(pid);
      cases = r.cases;
      hidden = r.hidden_people_cases;
      closeout = await ppm.closeout(pid);
      try {
        access = await ppm.accessRecords(pid);
        accessDenied = false;
      } catch (e) {
        if (e instanceof ApiError && e.status === 403) accessDenied = true;
        else throw e;
      }
      if (open) open = await ppm.onboardingCase(open.pid);
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : "Could not load";
    }
  }
  onMount(load);

  async function show(c: OnboardingCaseSummary) {
    try {
      open = await ppm.onboardingCase(c.pid);
    } catch (err) {
      status = err instanceof Error ? err.message : "Could not open the checklist";
    }
  }

  let kind = $state<OnboardingKind>("plan_intake");
  let subject = $state("");
  let date = $state("");
  async function start(e: SubmitEvent) {
    e.preventDefault();
    const body = caseBody(kind, subject, date);
    if ("error" in body) return void (status = body.error);
    try {
      open = await ppm.startOnboarding(pid, body);
      status = "Checklist started.";
      subject = "";
      await load();
    } catch (err) {
      status = err instanceof Error ? err.message : "Could not start";
    }
  }

  let owners = $state<Record<string, string>>({});
  let dues = $state<Record<string, string>>({});
  let notes = $state<Record<string, string>>({});
  async function update(itemPid: string, body: Parameters<PpmClient["updateOnboardingItem"]>[2]) {
    if (!open) return;
    try {
      open = await ppm.updateOnboardingItem(open.pid, itemPid, body);
      status = "";
      await load();
    } catch (err) {
      status = err instanceof Error ? err.message : "Could not save";
    }
  }
  async function finish() {
    if (!open) return;
    try {
      open = await ppm.completeOnboarding(open.pid);
      status = "Checklist completed.";
      await load();
    } catch (err) {
      status = err instanceof Error ? err.message : "Could not complete";
    }
  }
  let cancelReason = $state("");
  async function cancel() {
    if (!open) return;
    if (!cancelReason.trim()) return void (status = "Say why you are cancelling.");
    try {
      open = await ppm.cancelOnboarding(open.pid, cancelReason);
      cancelReason = "";
      status = "Checklist cancelled.";
      await load();
    } catch (err) {
      status = err instanceof Error ? err.message : "Could not cancel";
    }
  }

  let aPerson = $state("");
  let aWhat = $state("");
  let aAction = $state<"granted" | "revoked">("revoked");
  async function recordAccess(e: SubmitEvent) {
    e.preventDefault();
    const body = accessBody(aPerson, aWhat, aAction);
    if ("error" in body) return void (status = body.error);
    try {
      await ppm.recordAccess(pid, { ...body, case_pid: open?.pid });
      aWhat = "";
      status = "Recorded.";
      await load();
    } catch (err) {
      status = err instanceof Error ? err.message : "Could not record";
    }
  }
</script>

<svelte:head><title>Onboarding and offboarding — PPM</title></svelte:head>

<h1>Onboarding and offboarding</h1>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>← Plan</a> · <a href={localeHref(`/plans/${pid}/handover`)}>Hand over someone's work</a></p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if status}<p role="status" class="small">{status}</p>{/if}

{#if closeout}
  <section class="surface" aria-labelledby="co-h" data-testid="closeout">
    <h2 id="co-h">Ready to close?</h2>
    <p>{closeout.ready ? "●" : "▲"} {closeoutText(closeout)}</p>
  </section>
{/if}

<h2>Checklists</h2>
{#if cases.length === 0}<p class="muted">None yet.</p>{/if}
<ul class="stack plain" data-testid="cases">
  {#each cases as c (c.pid)}
    <li class="surface">
      <button class="button" type="button" onclick={() => show(c)} aria-label={`Open ${c.title}`}>{c.title}</button>
      <span class="small muted">{c.status} · from {c.effective_date} · {progressText(c.progress)}</span>
    </li>
  {/each}
</ul>
{#if hidden}<p class="small muted">{hidden} checklist{hidden === 1 ? "" : "s"} about people {hidden === 1 ? "is" : "are"} not shown to you.</p>{/if}

{#if open}
  <section class="surface stack" aria-labelledby="case-h" data-testid="case">
    <h2 id="case-h">{open.title}</h2>
    <p class="small muted">
      {open.status}{open.subject_ref ? ` · ${open.subject_ref}` : ""} · effective {open.effective_date} · {progressText(open.progress)}
    </p>
    <ol class="plain stack">
      {#each open.items as i (i.pid)}
        {@const st = itemState(i)}
        <li>
          <strong><span aria-hidden="true">{st.symbol}</span> {i.title}</strong>
          <span class="small"> — {st.word}{i.due_date ? `, due ${i.due_date}` : ""}{i.owner_ref ? `, ${i.owner_ref}` : ""}</span>
          {#if i.note}<span class="small muted"> ({i.note})</span>{/if}
          {#if open.status === "open"}
            <div class="row">
              <input aria-label={`Owner of ${i.title}`} placeholder="person:…" value={owners[i.pid] ?? ""} oninput={(e) => (owners[i.pid] = e.currentTarget.value)} />
              <input type="date" aria-label={`Due date of ${i.title}`} value={dues[i.pid] ?? ""} oninput={(e) => (dues[i.pid] = e.currentTarget.value)} />
              <button class="button" type="button" onclick={() => update(i.pid, { ...(owners[i.pid] !== undefined ? { owner_ref: owners[i.pid] } : {}), ...(dues[i.pid] ? { due_date: dues[i.pid] } : {}) })}>Save</button>
              {#if i.resolution}
                <button class="button" type="button" onclick={() => update(i.pid, { resolution: "open" })}>Reopen</button>
              {:else}
                <button class="button primary" type="button" onclick={() => update(i.pid, { resolution: "done" })}>Done</button>
                <input aria-label={`Why skip ${i.title}`} placeholder="Why skip?" value={notes[i.pid] ?? ""} oninput={(e) => (notes[i.pid] = e.currentTarget.value)} />
                <button class="button" type="button" onclick={() => update(i.pid, { resolution: "skipped", note: notes[i.pid] })}>Skip</button>
              {/if}
            </div>
          {/if}
        </li>
      {/each}
    </ol>
    {#if open.status === "open"}
      <div class="row">
        <button class="button primary" type="button" onclick={finish}>Complete checklist</button>
        <input aria-label="Reason for cancelling" placeholder="Reason" bind:value={cancelReason} />
        <button class="button danger" type="button" onclick={cancel}>Cancel checklist</button>
      </div>
    {/if}
  </section>
{/if}

<form class="surface stack" onsubmit={start} aria-labelledby="start-h">
  <h2 id="start-h">Start a checklist</h2>
  <label>
    What
    <select bind:value={kind}>
      {#each Object.entries(KIND_TEXT) as [k, v]}<option value={k}>{v}</option>{/each}
    </select>
  </label>
  {#if isPersonKind(kind)}
    <label>Person <input bind:value={subject} placeholder="person:…" /></label>
  {/if}
  <label>Takes effect <input type="date" bind:value={date} /></label>
  <button class="button primary" type="submit">Start</button>
</form>

<h2>Access record</h2>
{#if accessDenied}
  <p class="muted" data-testid="access-denied">Who holds what access is about people, so it needs the permission to manage people.</p>
{:else if access}
  <p class="small muted">{access.note}</p>
  {#if access.outstanding.length === 0}
    <p>● No access is outstanding.</p>
  {:else}
    <ul data-testid="outstanding">
      {#each access.outstanding as o}<li>▲ {o.person_ref}: {o.access} (since {o.granted_on})</li>{/each}
    </ul>
  {/if}
  <form class="surface stack" onsubmit={recordAccess} aria-labelledby="acc-h">
    <h3 id="acc-h">Record access</h3>
    <label>Person <input bind:value={aPerson} placeholder="person:…" /></label>
    <label>Access <input bind:value={aWhat} placeholder="repo write" /></label>
    <label>
      Action
      <select bind:value={aAction}><option value="revoked">Revoked</option><option value="granted">Granted</option></select>
    </label>
    <button class="button" type="submit">Record</button>
  </form>
{/if}

<style>
  .plain {
    list-style: none;
    padding: 0;
  }
  label {
    display: block;
  }
</style>
