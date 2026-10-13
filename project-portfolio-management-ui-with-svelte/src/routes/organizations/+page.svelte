<!--
  Partner organizations (`/organizations`, EX-3): who delivers, sponsors, supplies or partners on the
  portfolio. Add one, retire one or restore it.
-->
<script lang="ts">
  import { t } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type Organization } from "#lib/api/ppm.js";

  const KINDS = ["delivery", "partner", "sponsor", "supplier"] as const;
  const ppm = PpmClient.withFetch();
  let orgs = $state<Organization[] | null>(null);
  let error = $state<string | null>(null);
  let busy = $state(false);
  let name = $state("");
  let kind = $state<string>("partner");

  async function load() {
    try {
      orgs = (await ppm.organizations()).organizations;
      error = null;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.organizations.could_not_load");
    }
  }
  onMount(() => void load());

  async function run(job: () => Promise<unknown>) {
    busy = true;
    try {
      await job();
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.organizations.could_not_save");
    } finally {
      busy = false;
    }
  }
  const add = async () => {
    await run(() => ppm.addOrganization(name.trim(), kind));
    if (!error) name = "";
  };
  const kindWord = (k: string) => t(`p.organizations.kind_${k}` as never);
</script>

<svelte:head><title>{t("p.organizations.title")}</title></svelte:head>

<h1>{t("p.organizations.h1")}</h1>
<p class="lede">{t("p.organizations.lede")}</p>
{#if error}<p class="banner error" role="alert">{error}</p>{/if}
{#if !orgs && !error}<p class="muted">{t("p.organizations.loading")}</p>{/if}

<section aria-labelledby="org-add">
  <h2 id="org-add">{t("p.organizations.add_h2")}</h2>
  <form onsubmit={(e) => { e.preventDefault(); if (name.trim()) void add(); }}>
    <label>{t("p.organizations.name")} <input bind:value={name} required maxlength="200" /></label>
    <label>{t("p.organizations.kind")}
      <select bind:value={kind}>{#each KINDS as k (k)}<option value={k}>{kindWord(k)}</option>{/each}</select>
    </label>
    <button class="button" type="submit" disabled={busy || !name.trim()}>{t("p.organizations.add")}</button>
  </form>
</section>

{#if orgs}
  {#if orgs.length === 0}
    <p>{t("p.organizations.none")}</p>
  {:else}
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div class="scroll" role="region" aria-label={t("p.organizations.h1")} tabindex="0">
      <table>
        <caption class="sr-only">{t("p.organizations.h1")}</caption>
        <thead>
          <tr>
            <th scope="col">{t("p.organizations.col_name")}</th>
            <th scope="col">{t("p.organizations.col_kind")}</th>
            <th scope="col">{t("p.organizations.col_state")}</th>
            <th scope="col">{t("p.organizations.col_audience")}</th>
            <th scope="col">{t("p.organizations.col_act")}</th>
          </tr>
        </thead>
        <tbody>
          {#each orgs as o (o.pid)}
            <tr>
              <th scope="row">{o.name}</th>
              <td>{kindWord(o.kind)}</td>
              <td>{o.active ? t("p.organizations.active") : t("p.organizations.retired")}</td>
              <td><code>{o.audience}</code></td>
              <td>
                <button class="button" type="button" disabled={busy}
                  aria-label={t(o.active ? "p.organizations.retire" : "p.organizations.restore", { name: o.name })}
                  onclick={() => void run(() => ppm.changeOrganization(o.pid, { active: !o.active }))}
                >{o.active ? t("p.organizations.retire_short") : t("p.organizations.restore_short")}</button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
{/if}

<style>
  .scroll { overflow-x: auto; margin-block: 1rem; }
  form { display: flex; flex-wrap: wrap; gap: 0.75rem; align-items: end; }
</style>
