<!--
  Which organizations must sign each of a plan's milestones off (EX-4, NC-1 slice 11), on the governance page. A
  milestone is met only when every required organization's sign-off is in force; delivered and met are separate dates
  (`/milestone-signoffs` shows them and records the sign-offs). The set shown is the milestone's whole required set,
  signed or not, because saving replaces it. Every rule is said first.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { PpmClient, type MilestoneView, type Organization } from "#lib/api/ppm.js";
  import { REQUIRED_MAX, requiredPids, requiredProblem, samePids } from "#lib/signoff-view.js";

  let { pid }: { pid: string } = $props();
  const ppm = PpmClient.withFetch();

  let milestones = $state<MilestoneView[]>([]);
  let orgs = $state<Organization[]>([]);
  let loaded = $state(false);
  let loadError = $state<string | null>(null);
  let editing = $state<string | null>(null);
  let chosen = $state<string[]>([]);
  let error = $state<string | null>(null);
  let notice = $state("");
  let busy = $state(false);

  async function load() {
    try {
      const [m, o] = await Promise.all([ppm.portfolioMilestones({ plan: pid }), ppm.organizations()]);
      milestones = m.milestones;
      orgs = o.organizations;
      loaded = true;
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.signoffs.could_not_load");
    }
  }
  onMount(() => void load());

  // Active organizations, and any inactive one this milestone still requires (so it is seen, and unticked on purpose).
  const offered = (m: MilestoneView) => {
    const req = new Set(requiredPids(m.signoff));
    return orgs.filter((o) => o.active || req.has(o.pid));
  };

  function start(m: MilestoneView) {
    editing = m.pid;
    chosen = requiredPids(m.signoff);
    error = null;
    notice = "";
  }

  async function save(e: SubmitEvent, m: MilestoneView) {
    e.preventDefault();
    error = null;
    notice = "";
    const problem = requiredProblem(chosen, orgs);
    if (problem) {
      error = problem.key === "p.signoffs.problem_inactive" ? t(problem.key, { name: problem.name }) : t(problem.key, { max: REQUIRED_MAX });
      return;
    }
    if (samePids(requiredPids(m.signoff), chosen)) {
      notice = t("p.signoffs.required_unchanged");
      editing = null;
      return;
    }
    busy = true;
    try {
      await ppm.setSignoffRequired(pid, m.pid, chosen);
      notice = chosen.length === 0 ? t("p.signoffs.required_cleared", { name: m.name }) : t("p.signoffs.required_saved", { name: m.name });
      editing = null;
      await load();
    } catch (err) {
      error = errorText(err, "p.signoffs.could_not_save");
    } finally {
      busy = false;
    }
  }
</script>

<section aria-labelledby="signoff-req-h">
  <h3 id="signoff-req-h">{t("p.signoffs.required_heading")}</h3>
  <p class="small muted">{t("p.signoffs.required_says")} <a href={localeHref("/milestone-signoffs")}>{t("p.signoffs.required_link")}</a></p>
  {#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}
  {#if error}<p class="banner error" role="alert">{error}</p>{/if}
  <p class="small" role="status">{notice}</p>
  {#if loaded}
    <ul class="small">
      {#each milestones as m (m.pid)}
        <li>
          <strong>{m.name}</strong>:
          {#if m.signoff && m.signoff.required.length > 0}
            {m.signoff.required
              .map((r) => (r.signed_on ? t("p.signoffs.required_signed", { name: r.name, date: formatDate(r.signed_on) }) : t("p.signoffs.required_waiting", { name: r.name })))
              .join("; ")}
          {:else}
            {t("p.signoffs.required_none")}
          {/if}
          {#if editing !== m.pid}
            <button class="button small" type="button" disabled={busy} onclick={() => start(m)} aria-label={t("p.signoffs.required_change_for", { name: m.name })}>{t("p.signoffs.required_change")}</button>
          {:else}
            <form class="stack" novalidate onsubmit={(e) => save(e, m)}>
              <fieldset>
                <legend>{t("p.signoffs.required_legend", { name: m.name, max: REQUIRED_MAX })}</legend>
                {#each offered(m) as o (o.pid)}
                  <label class="check">
                    <input type="checkbox" value={o.pid} bind:group={chosen} />
                    {o.active ? o.name : t("p.signoffs.required_inactive", { name: o.name })}
                  </label>
                {/each}
                {#if offered(m).length === 0}<p class="muted">{t("p.signoffs.required_no_orgs")}</p>{/if}
              </fieldset>
              <div class="row">
                <button class="button small" type="submit" disabled={busy}>{t("p.signoffs.required_save")}</button>
                <button class="button small" type="button" onclick={() => (editing = null)}>{t("p.signoffs.required_cancel")}</button>
              </div>
            </form>
          {/if}
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  .check {
    display: flex;
    gap: 0.4rem;
    align-items: center;
    min-block-size: 24px;
  }
  fieldset {
    display: grid;
    gap: 0.25rem;
  }
</style>
