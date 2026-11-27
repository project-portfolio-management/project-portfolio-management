<!--
  Custom fields (`/custom-fields`, RM-1; NC-1 slice 12 part n): every live definition for plans and for tasks with its
  key, type, whether it is required, the plan kinds it is limited to and a list's choices (`GET /api/custom-fields`);
  a field declared (`POST`) with every problem the service would find said first, all at once, and a key already
  taken for that subject type refused before sending; a definition withdrawn after a question (`DELETE`; values
  already stored stay readable, marked withdrawn). A search (`GET …/search`) finds the plans or tasks whose value for
  a field equals one given in the field's own terms (a list field: a choice; a multi-choice field: contains it), only
  those the reader may see. Rules in `src/lib/custom-field-view.ts`. Values on one plan or task are not edited here.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { formatInteger } from "#lib/locale-format.js";
  import { ApiError } from "#lib/api/client.js";
  import { PpmClient, type CustomFieldDef, type CustomFieldSearch } from "#lib/api/ppm.js";
  import {
    APPLIES_TO,
    EMPTY_DEF,
    FIELD_TYPES,
    MAX_SHORT,
    declareRefusalKey,
    defBody,
    isList,
    keyTaken,
    searchValue,
    type DefForm,
  } from "#lib/custom-field-view.js";

  const ppm = PpmClient.withFetch();

  let defs = $state<CustomFieldDef[] | null>(null);
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let problems = $state<string[]>([]);
  let notice = $state<string | null>(null);
  let busy = $state(false);
  let form = $state<DefForm>({ ...EMPTY_DEF });

  let sApplies = $state<string>("plan");
  let sKey = $state("");
  let sValue = $state("");
  let sError = $state<string | null>(null);
  let found = $state<{ applies: string; result: CustomFieldSearch } | null>(null);

  async function load() {
    try {
      defs = await ppm.customFields();
      loadError = null;
    } catch (err) {
      loadError = errorText(err, "p.custom_fields.could_not_load");
    }
  }
  onMount(load);

  const searchable = $derived((defs ?? []).filter((d) => d.applies_to === sApplies));
  const sDef = $derived(searchable.find((d) => d.key === sKey));

  async function declare(e: SubmitEvent) {
    e.preventDefault();
    notice = error = null;
    const r = defBody(form);
    if ("problems" in r) {
      problems = r.problems;
      return;
    }
    problems = [];
    if (keyTaken(r.body.key, r.body.applies_to, defs ?? [])) {
      error = t("p.custom_fields.key_taken");
      return;
    }
    busy = true;
    try {
      await ppm.declareCustomField(r.body);
      notice = t("p.custom_fields.declared", { label: r.body.label });
      form = { ...EMPTY_DEF };
      await load();
    } catch (err) {
      const s = err instanceof ApiError ? err.status : 500;
      error = s === 409 ? tDynamic(declareRefusalKey(s)) : errorText(err, "p.custom_fields.could_not_declare");
    } finally {
      busy = false;
    }
  }

  async function withdraw(d: CustomFieldDef) {
    if (!(await askConfirm(t("p.custom_fields.withdraw_ask", { label: d.label })))) return;
    busy = true;
    notice = error = null;
    try {
      await ppm.withdrawCustomField(d.pid);
      notice = t("p.custom_fields.withdrawn", { label: d.label });
      if (sKey === d.key) found = null;
    } catch (err) {
      error = errorText(err, "p.custom_fields.could_not_withdraw");
    } finally {
      busy = false;
      await load();
    }
  }

  async function search(e: SubmitEvent) {
    e.preventDefault();
    found = null;
    if (!sDef) {
      sError = t("p.custom_fields.field_required");
      return;
    }
    const v = searchValue(sDef.field_type, sValue);
    if ("problem" in v) {
      sError = tDynamic(v.problem);
      return;
    }
    sError = null;
    try {
      found = { applies: sApplies, result: await ppm.searchCustomFields(sApplies, sDef.key, v.value) };
    } catch (err) {
      sError = errorText(err, "p.custom_fields.could_not_search");
    }
  }
</script>

<svelte:head><title>{t("p.custom_fields.title_ppm")}</title></svelte:head>

<h1>{t("p.custom_fields.title")}</h1>
<p class="lede">{t("p.custom_fields.intro")}</p>
{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{#if error}<p class="banner error" role="alert" data-testid="cf-error">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="cf-notice">{notice}</p>{/if}

{#if !defs && !loadError}
  <p class="muted">{t("p.custom_fields.loading")}</p>
{:else if defs}
  {#each APPLIES_TO as a (a)}
    {@const rows = defs.filter((d) => d.applies_to === a)}
    <section class="surface stack" aria-labelledby={`h-${a}`}>
      <h2 id={`h-${a}`}>{tDynamic(`p.custom_fields.for.${a}`)}</h2>
      {#if rows.length === 0}
        <p class="muted" data-testid={`cf-none-${a}`}>{t("p.custom_fields.none")}</p>
      {:else}
        <ul class="items" data-testid={`cf-${a}`}>
          {#each rows as d (d.pid)}
            <li>
              <strong id={`def-${d.pid}`}>{d.label}</strong>
              <code>{d.key}</code>
              <span class="small">{tDynamic(`p.custom_fields.type.${d.field_type}`)}</span>
              {#if d.required}<span class="small">{t("p.custom_fields.is_required")}</span>{/if}
              {#if d.kinds.length > 0}
                <span class="small muted">{t("p.custom_fields.kinds_only", { kinds: d.kinds.join(", ") })}</span>
              {/if}
              {#if d.options.length > 0}
                <span class="small muted">{t("p.custom_fields.choices", { options: d.options.join(", ") })}</span>
              {/if}
              <button
                class="button small danger"
                type="button"
                disabled={busy}
                onclick={() => withdraw(d)}
                aria-describedby={`def-${d.pid}`}>{t("p.custom_fields.withdraw")}</button
              >
            </li>
          {/each}
        </ul>
      {/if}
    </section>
  {/each}

  <form class="surface stack" onsubmit={declare} aria-labelledby="declare-h" novalidate>
    <h2 id="declare-h">{t("p.custom_fields.declare_h")}</h2>
    {#if problems.length > 0}
      <ul class="banner error" role="alert" data-testid="cf-problems">
        {#each problems as p (p)}<li>{tDynamic(p)}</li>{/each}
      </ul>
    {/if}
    <label>{t("p.custom_fields.key_field")} <input bind:value={form.key} maxlength={MAX_SHORT} autocomplete="off" /></label>
    <label>{t("p.custom_fields.label_field")} <input bind:value={form.label} maxlength={MAX_SHORT} /></label>
    <label
      >{t("p.custom_fields.type_field")}
      <select bind:value={form.field_type}>
        {#each FIELD_TYPES as ft (ft)}<option value={ft}>{tDynamic(`p.custom_fields.type.${ft}`)}</option>{/each}
      </select></label
    >
    <label
      >{t("p.custom_fields.applies_field")}
      <select bind:value={form.applies_to}>
        {#each APPLIES_TO as a (a)}<option value={a}>{tDynamic(`p.custom_fields.applies.${a}`)}</option>{/each}
      </select></label
    >
    {#if form.applies_to === "plan"}
      <label>{t("p.custom_fields.kinds_field")} <input bind:value={form.kinds} /></label>
    {/if}
    {#if isList(form.field_type)}
      <label>{t("p.custom_fields.options_field")} <textarea bind:value={form.options} rows="4"></textarea></label>
    {/if}
    <label class="check"><input type="checkbox" bind:checked={form.required} /> {t("p.custom_fields.required_field")}</label>
    <button class="button primary" type="submit" disabled={busy}>{t("p.custom_fields.declare")}</button>
  </form>

  <form class="surface stack" onsubmit={search} aria-labelledby="search-h">
    <h2 id="search-h">{t("p.custom_fields.search_h")}</h2>
    {#if sError}<p class="banner error" role="alert">{sError}</p>{/if}
    <label
      >{t("p.custom_fields.applies_field")}
      <select bind:value={sApplies} onchange={() => ((sKey = ""), (sValue = ""), (found = null))}>
        {#each APPLIES_TO as a (a)}<option value={a}>{tDynamic(`p.custom_fields.applies.${a}`)}</option>{/each}
      </select></label
    >
    <label
      >{t("p.custom_fields.field_field")}
      <select bind:value={sKey} onchange={() => ((sValue = ""), (found = null))}>
        <option value="">{t("p.custom_fields.choose")}</option>
        {#each searchable as d (d.pid)}<option value={d.key}>{d.label}</option>{/each}
      </select></label
    >
    {#if sDef}
      <label
        >{t("p.custom_fields.value_field")}
        {#if sDef.field_type === "bool"}
          <select bind:value={sValue}>
            <option value="">{t("p.custom_fields.choose")}</option>
            <option value="true">{t("p.custom_fields.yes")}</option>
            <option value="false">{t("p.custom_fields.no")}</option>
          </select>
        {:else if isList(sDef.field_type)}
          <select bind:value={sValue}>
            <option value="">{t("p.custom_fields.choose")}</option>
            {#each sDef.options as o (o)}<option value={o}>{o}</option>{/each}
          </select>
        {:else if sDef.field_type === "date"}
          <input type="date" bind:value={sValue} />
        {:else}
          <input bind:value={sValue} inputmode={sDef.field_type === "integer" ? "numeric" : undefined} />
        {/if}
      </label>
    {/if}
    <button class="button" type="submit">{t("p.custom_fields.search")}</button>
    {#if found}
      <div role="status" data-testid="cf-found">
        <p>{tDynamic(`p.custom_fields.found.${found.applies}`, { n: formatInteger(found.result.count) })}</p>
        {#if found.result.truncated}<p class="small muted">{t("p.custom_fields.truncated")}</p>{/if}
        {#if found.result.pids.length > 0}
          <ul>
            {#each found.result.pids as p (p)}
              <li>
                {#if found.applies === "plan"}<a href={localeHref(`/plans/${p}`)}>{p}</a>{:else}<code>{p}</code>{/if}
              </li>
            {/each}
          </ul>
        {/if}
      </div>
    {/if}
  </form>
{/if}

<style>
  .items {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 0.5rem;
  }
  .items li {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: baseline;
  }
  h2 {
    font-size: 1.05rem;
  }
  label {
    display: block;
  }
</style>
