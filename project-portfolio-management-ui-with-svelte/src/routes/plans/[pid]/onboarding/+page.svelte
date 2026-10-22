<!--
  Onboarding and offboarding (`/plans/[pid]/onboarding`): checklists for someone
  joining or leaving the plan and for taking the plan in or closing it out, what
  stands between the plan and being closed, and the record of access granted and
  revoked. Status is a word and a shape, never colour alone. The access record is
  a hand-kept record: this service cannot change anyone's real accounts. Anything
  about a person needs the permission to manage people; without it the page says so.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t, localeHref } from "#lib/i18n.svelte.js";
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
      error = errorText(err, "p.plans.pid.onboarding.could_not_load");
    }
  }
  onMount(load);

  async function show(c: OnboardingCaseSummary) {
    try {
      open = await ppm.onboardingCase(c.pid);
    } catch (err) {
      status = errorText(err, "p.plans.pid.onboarding.could_not_open_the_checklist");
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
      status = t("p.plans.pid.onboarding.checklist_started");
      subject = "";
      await load();
    } catch (err) {
      status = errorText(err, "p.plans.pid.onboarding.could_not_start");
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
      status = errorText(err, "p.plans.pid.onboarding.could_not_save");
    }
  }
  async function finish() {
    if (!open) return;
    try {
      open = await ppm.completeOnboarding(open.pid);
      status = t("p.plans.pid.onboarding.checklist_completed");
      await load();
    } catch (err) {
      status = errorText(err, "p.plans.pid.onboarding.could_not_complete");
    }
  }
  let cancelReason = $state("");
  async function cancel() {
    if (!open) return;
    if (!cancelReason.trim()) return void (status = t("p.plans.pid.onboarding.say_why_you_are_cancelling"));
    try {
      open = await ppm.cancelOnboarding(open.pid, cancelReason);
      cancelReason = "";
      status = t("p.plans.pid.onboarding.checklist_cancelled");
      await load();
    } catch (err) {
      status = errorText(err, "p.plans.pid.onboarding.could_not_cancel");
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
      status = t("p.plans.pid.earned_value.recorded");
      await load();
    } catch (err) {
      status = errorText(err, "p.plans.pid.onboarding.could_not_record");
    }
  }
</script>

<svelte:head><title>{t("p.plans.pid.onboarding.onboarding_and_offboarding_ppm")}</title></svelte:head>

<h1>{t("p.plans.pid.onboarding.onboarding_and_offboarding")}</h1>
<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plans.pid.onboarding.plan")}</a> · <a href={localeHref(`/plans/${pid}/handover`)}>{t("p.plans.pid.onboarding.hand_over_someone_s_work")}</a></p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if status}<p role="status" class="small">{status}</p>{/if}

{#if closeout}
  <section class="surface" aria-labelledby="co-h" data-testid="closeout">
    <h2 id="co-h">{t("p.plans.pid.onboarding.ready_to_close")}</h2>
    <p>{closeout.ready ? "●" : "▲"} {closeoutText(closeout)}</p>
  </section>
{/if}

<h2>{t("p.plans.pid.onboarding.checklists")}</h2>
{#if cases.length === 0}<p class="muted">{t("p.plans.pid.onboarding.none_yet")}</p>{/if}
<ul class="stack plain" data-testid="cases">
  {#each cases as c (c.pid)}
    <li class="surface">
      <button class="button" type="button" onclick={() => show(c)} aria-label={`Open ${c.title}`}>{c.title}</button>
      <span class="small muted">{t("p.plans.pid.onboarding.from", { status: c.status, effective_date: c.effective_date, progressText: progressText(c.progress) })}</span>
    </li>
  {/each}
</ul>
{#if hidden}<p class="small muted">{(hidden === 1 ? t("p.plans.pid.onboarding.checklist_about_people_not_shown_one", { hidden }) : t("p.plans.pid.onboarding.checklist_about_people_not_shown_other", { hidden }))}</p>{/if}

{#if open}
  <section class="surface stack" aria-labelledby="case-h" data-testid="case">
    <h2 id="case-h">{open.title}</h2>
    <p class="small muted">
      {t("p.plans.pid.onboarding.effective", { status: open.status, v: open.subject_ref ? ` · ${open.subject_ref}` : "", effective_date: open.effective_date, progressText: progressText(open.progress) })}
    </p>
    <ol class="plain stack">
      {#each open.items as i (i.pid)}
        {@const st = itemState(i)}
        <li>
          <strong><span aria-hidden="true">{st.symbol}</span> {i.title}</strong>
          <span class="small"> — {st.word}{i.due_date ? t("p.plans.pid.onboarding.due", { date: i.due_date }) : ""}{i.owner_ref ? `, ${i.owner_ref}` : ""}</span>
          {#if i.note}<span class="small muted"> ({i.note})</span>{/if}
          {#if open.status === "open"}
            <div class="row">
              <input aria-label={t("p.plans.pid.onboarding.owner_of", { title: i.title })} placeholder={t("p.plans.pid.onboarding.person")} value={owners[i.pid] ?? ""} oninput={(e) => (owners[i.pid] = e.currentTarget.value)} />
              <input type="date" aria-label={t("p.plans.pid.onboarding.due_date_of", { title: i.title })} value={dues[i.pid] ?? ""} oninput={(e) => (dues[i.pid] = e.currentTarget.value)} />
              <button class="button" type="button" onclick={() => update(i.pid, { ...(owners[i.pid] !== undefined ? { owner_ref: owners[i.pid] } : {}), ...(dues[i.pid] ? { due_date: dues[i.pid] } : {}) })}>{t("p.plans.pid.onboarding.save")}</button>
              {#if i.resolution}
                <button class="button" type="button" onclick={() => update(i.pid, { resolution: "open" })}>{t("p.plans.pid.onboarding.reopen")}</button>
              {:else}
                <button class="button" type="button" onclick={() => update(i.pid, { resolution: "done" })}>{t("p.plans.pid.onboarding.done")}</button>
                <input aria-label={t("p.plans.pid.onboarding.why_skip_title", { title: i.title })} placeholder={t("p.plans.pid.onboarding.why_skip")} value={notes[i.pid] ?? ""} oninput={(e) => (notes[i.pid] = e.currentTarget.value)} />
                <button class="button" type="button" onclick={() => update(i.pid, { resolution: "skipped", note: notes[i.pid] })}>{t("p.plans.pid.onboarding.skip")}</button>
              {/if}
            </div>
          {/if}
        </li>
      {/each}
    </ol>
    {#if open.status === "open"}
      <div class="row">
        <button class="button primary" type="button" onclick={finish}>{t("p.plans.pid.onboarding.complete_checklist")}</button>
        <input aria-label={t("p.plans.pid.onboarding.reason_for_cancelling")} placeholder={t("p.plans.pid.onboarding.reason")} bind:value={cancelReason} />
        <button class="button danger" type="button" onclick={cancel}>{t("p.plans.pid.onboarding.cancel_checklist")}</button>
      </div>
    {/if}
  </section>
{/if}

<form class="surface stack" onsubmit={start} aria-labelledby="start-h">
  <h2 id="start-h">{t("p.plans.pid.onboarding.start_a_checklist")}</h2>
  <label>
    {t("p.plans.pid.onboarding.what")}
    <select bind:value={kind}>
      {#each Object.entries(KIND_TEXT) as [k, v]}<option value={k}>{v}</option>{/each}
    </select>
  </label>
  {#if isPersonKind(kind)}
    <label>{t("p.plans.pid.onboarding.person_2")} <input bind:value={subject} placeholder={t("p.plans.pid.onboarding.person")} /></label>
  {/if}
  <label>{t("p.plans.pid.onboarding.takes_effect")} <input type="date" bind:value={date} /></label>
  <button class="button" class:primary={!open} type="submit">{t("p.plans.pid.onboarding.start")}</button>
</form>

<h2>{t("p.plans.pid.onboarding.access_record")}</h2>
{#if accessDenied}
  <p class="muted" data-testid="access-denied">{t("p.plans.pid.onboarding.who_holds_what_access_is")}</p>
{:else if access}
  <p class="small muted">{access.note}</p>
  {#if access.outstanding.length === 0}
    <p>{t("p.plans.pid.onboarding.no_access_is_outstanding")}</p>
  {:else}
    <ul data-testid="outstanding">
      {#each access.outstanding as o}<li>{t("p.plans.pid.onboarding.since", { person_ref: o.person_ref, access: o.access, granted_on: o.granted_on })}</li>{/each}
    </ul>
  {/if}
  <form class="surface stack" onsubmit={recordAccess} aria-labelledby="acc-h">
    <h3 id="acc-h">{t("p.plans.pid.onboarding.record_access")}</h3>
    <label>{t("p.plans.pid.onboarding.person_2")} <input bind:value={aPerson} placeholder={t("p.plans.pid.onboarding.person")} /></label>
    <label>{t("p.plans.pid.onboarding.access")} <input bind:value={aWhat} placeholder={t("p.plans.pid.onboarding.repo_write")} /></label>
    <label>
      {t("p.plans.pid.onboarding.action")}
      <select bind:value={aAction}><option value="revoked">{t("p.plans.pid.onboarding.revoked")}</option><option value="granted">{t("p.plans.pid.onboarding.granted")}</option></select>
    </label>
    <button class="button" type="submit">{t("p.plans.pid.onboarding.record")}</button>
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
