<!--
  Adding and changing skill pools (`/skill-pools`, DP-10, NC-1 slice 11): a pool groups resource roles and skills, so
  the capacity heatmap counts demand and supply once; a role or skill is in at most one pool. Add a pool with its
  roles and skills, change a pool's roles and skills, and retire or restore it. Every check the service makes (a name
  of 2 to 120 characters, not taken; a role or skill another pool holds) is said before anything is sent.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type SkillPool } from "#lib/api/ppm.js";
  import { POOL_NAME_MAX, clashes, membersBody, parseList, poolNameProblem } from "#lib/skill-pool-view.js";

  let { pools, onChanged }: { pools: SkillPool[]; onChanged: () => Promise<void> } = $props();

  const ppm = PpmClient.withFetch();
  let busy = $state(false);

  let name = $state("");
  let roles = $state("");
  let skills = $state("");
  let addError = $state<string | null>(null);
  let addNotice = $state("");

  let chosen = $state("");
  let editRoles = $state("");
  let editSkills = $state("");
  let editError = $state<string | null>(null);
  let editNotice = $state("");
  const pool = $derived(pools.find((p) => p.pid === chosen) ?? null);

  const clashText = (cs: Array<{ value: string; pool: string }>) =>
    t("p.skill_pools.problem_clash", { list: cs.map((c) => t("p.skill_pools.clash_item", { value: c.value, pool: c.pool })).join("; ") });
  const refusal = (err: unknown, key: Parameters<typeof errorText>[1]) =>
    err instanceof ApiError && err.status === 409 ? t("p.skill_pools.conflict_members") : errorText(err, key);

  async function add(e: SubmitEvent) {
    e.preventDefault();
    addError = null;
    addNotice = "";
    const problem = poolNameProblem(name, pools);
    if (problem) {
      addError = t(problem);
      return;
    }
    const r = parseList(roles);
    const s = parseList(skills);
    const cs = clashes(r, s, pools, null);
    if (cs.length > 0) {
      addError = clashText(cs);
      return;
    }
    busy = true;
    try {
      await ppm.createSkillPool({ name: name.trim(), roles: r, skills: s });
      addNotice = t("p.skill_pools.added", { name: name.trim() });
      name = "";
      roles = "";
      skills = "";
      await onChanged();
    } catch (err) {
      addError = refusal(err, "p.skill_pools.could_not_add");
    } finally {
      busy = false;
    }
  }

  function choose() {
    editError = null;
    editNotice = "";
    editRoles = pool ? pool.roles.join(", ") : "";
    editSkills = pool ? pool.skills.join(", ") : "";
  }

  async function saveMembers(e: SubmitEvent) {
    e.preventDefault();
    if (!pool) return;
    editError = null;
    editNotice = "";
    const r = parseList(editRoles);
    const s = parseList(editSkills);
    const cs = clashes(r, s, pools, pool.pid);
    if (cs.length > 0) {
      editError = clashText(cs);
      return;
    }
    const body = membersBody(pool, r, s);
    if (!body) {
      editNotice = t("p.skill_pools.unchanged");
      return;
    }
    busy = true;
    try {
      await ppm.updateSkillPool(pool.pid, body);
      editNotice = t("p.skill_pools.members_saved", { name: pool.name });
      await onChanged();
    } catch (err) {
      editError = refusal(err, "p.skill_pools.could_not_save");
    } finally {
      busy = false;
    }
  }

  async function setActive(active: boolean) {
    if (!pool) return;
    editError = null;
    editNotice = "";
    busy = true;
    try {
      await ppm.updateSkillPool(pool.pid, { active });
      editNotice = active ? t("p.skill_pools.restored", { name: pool.name }) : t("p.skill_pools.retired_done", { name: pool.name });
      await onChanged();
    } catch (err) {
      editError = errorText(err, "p.skill_pools.could_not_save");
    } finally {
      busy = false;
    }
  }
</script>

<section aria-labelledby="pool-add-h">
  <h2 id="pool-add-h">{t("p.skill_pools.add_heading")}</h2>
  <p class="small muted">{t("p.skill_pools.add_says")}</p>
  {#if addError}<p class="banner error" role="alert">{addError}</p>{/if}
  <p class="small" role="status">{addNotice}</p>
  <form class="stack" novalidate onsubmit={add}>
    <label>{t("p.skill_pools.f_name")}<input type="text" bind:value={name} maxlength={POOL_NAME_MAX} /></label>
    <label>{t("p.skill_pools.f_roles")}<textarea bind:value={roles} rows="2"></textarea></label>
    <label>{t("p.skill_pools.f_skills")}<textarea bind:value={skills} rows="2"></textarea></label>
    <p class="small muted">{t("p.skill_pools.list_says")}</p>
    <div class="row"><button class="button" type="submit" disabled={busy}>{t("p.skill_pools.add")}</button></div>
  </form>
</section>

<section aria-labelledby="pool-edit-h">
  <h2 id="pool-edit-h">{t("p.skill_pools.members_heading")}</h2>
  <label>
    {t("p.skill_pools.f_pool")}
    <select bind:value={chosen} onchange={choose}>
      <option value="">{t("p.skill_pools.choose_pool")}</option>
      {#each pools as p (p.pid)}<option value={p.pid}>{p.name}</option>{/each}
    </select>
  </label>
  {#if editError}<p class="banner error" role="alert">{editError}</p>{/if}
  <p class="small" role="status">{editNotice}</p>
  {#if pool}
    <form class="stack" novalidate onsubmit={saveMembers}>
      <label>{t("p.skill_pools.f_roles")}<textarea bind:value={editRoles} rows="2"></textarea></label>
      <label>{t("p.skill_pools.f_skills")}<textarea bind:value={editSkills} rows="2"></textarea></label>
      <div class="row">
        <button class="button" type="submit" disabled={busy}>{t("p.skill_pools.save_members")}</button>
        <button class="button" type="button" disabled={busy} onclick={() => setActive(!pool.active)}>
          {pool.active ? t("p.skill_pools.retire") : t("p.skill_pools.restore")}
        </button>
      </div>
    </form>
    <p class="small muted">{t("p.skill_pools.retire_says")}</p>
  {/if}
</section>

<style>
  section {
    margin-block-start: 2rem;
  }
  form {
    max-width: 40rem;
  }
</style>
