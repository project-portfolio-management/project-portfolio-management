<!--
  Director packs (`/director-packs`, DP-11): the packs you may read, newest first. Each opens as an
  accessible page; the figures are read from the issued pack, never recalculated.
-->
<script lang="ts">
  import { t, localeHref } from "#lib/i18n.svelte.js";
  import { onMount } from "svelte";
  import { PpmClient, type DirectorPackMeta } from "#lib/api/ppm.js";

  const ppm = PpmClient.withFetch();
  let packs = $state<DirectorPackMeta[] | null>(null);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      packs = (await ppm.directorPacks()).packs;
    } catch (err) {
      error = err instanceof Error ? err.message : t("p.director_pack.could_not_load");
    }
  });
</script>

<svelte:head><title>{t("p.director_pack.director_packs_ppm")}</title></svelte:head>

<h1>{t("p.director_pack.director_packs")}</h1>
<p class="small muted">{t("p.director_pack.intro")}</p>

{#if error}<p class="banner error" role="alert">{error}</p>{/if}

{#if packs}
  {#if packs.length === 0}
    <p>{t("p.director_pack.none")}</p>
  {:else}
    <table>
      <thead>
        <tr>
          <th scope="col">{t("p.director_pack.col_period")}</th>
          <th scope="col">{t("p.director_pack.col_audience")}</th>
          <th scope="col">{t("p.director_pack.col_version")}</th>
          <th scope="col">{t("p.director_pack.col_status")}</th>
          <th scope="col">{t("p.director_pack.col_issued")}</th>
        </tr>
      </thead>
      <tbody>
        {#each packs as p (p.pid)}
          <tr>
            <th scope="row">
              <a
                href={localeHref(`/director-packs/${p.pid}`)}
                aria-label={t("p.director_pack.open_pack", { period: p.period, audience: p.audience, version: p.version })}
              >{p.period}</a>
            </th>
            <td>{p.audience}</td>
            <td>{p.version}</td>
            <td>{p.status === "issued" ? t("p.director_pack.status_issued") : t("p.director_pack.status_draft")}</td>
            <td>{p.issued_at ? p.issued_at.slice(0, 10) : "–"}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
{:else if !error}
  <p class="muted">{t("p.director_pack.loading")}</p>
{/if}
