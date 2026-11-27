<!--
  A plan's or a task's custom field values (RM-1; NC-1 slice 12 part n), on `/plans/[pid]/fields` and on a task's page:
  every live field for that subject (for a plan, those that apply to its kind), with its value as an input of its own
  type (text, a day, yes or no, a whole number, one choice, several), and the required ones still unset said first
  (`GET /api/custom-fields/values`). A save sends only what changed, a cleared value as `null` (`PUT`), after every
  problem is said with the field it is about; the service refuses the whole save if one value is wrong. A value whose
  definition was withdrawn stays readable, marked, and cannot be changed. Rules in `src/lib/custom-field-view.ts`.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { errorText } from "#lib/error-text.js";
  import { localeHref, t, tDynamic } from "#lib/i18n.svelte.js";
  import { PlanRepository } from "#lib/api/plans.js";
  import { PpmClient, type CustomFieldDef, type CustomFieldValues } from "#lib/api/ppm.js";
  import {
    MAX_TEXT,
    inputOf,
    kindApplies,
    shownValue,
    valueChanges,
    type ValueInput,
  } from "#lib/custom-field-view.js";

  const ppm = PpmClient.withFetch();
  const repo = PlanRepository.withFetch();
  let {
    appliesTo,
    subjectPid,
    level = 2,
  }: { appliesTo: "plan" | "task"; subjectPid: string; level?: 2 | 3 } = $props();
  const pid = $derived(subjectPid);
  const heading = $derived(`h${level}`);

  let defs = $state<CustomFieldDef[] | null>(null);
  let read = $state<CustomFieldValues | null>(null);
  let inputs = $state<Record<string, ValueInput>>({});
  let loadError = $state<string | null>(null);
  let error = $state<string | null>(null);
  let problems = $state<{ key: string; label: string }[]>([]);
  let notice = $state<string | null>(null);
  let busy = $state(false);

  function fill(all: CustomFieldDef[], values: CustomFieldValues) {
    inputs = Object.fromEntries(all.map((d) => [d.key, inputOf(d, values.values[d.key]?.value)]));
  }

  async function load() {
    try {
      const [all, values, kind] = await Promise.all([
        ppm.customFields(),
        ppm.customFieldValues(appliesTo, pid),
        appliesTo === "plan" ? repo.get(pid).then((p) => p.kind ?? null) : Promise.resolve(null),
      ]);
      // A field limited to plan kinds is a plan field; a task's fields apply to every task.
      defs = all.filter((d) => d.applies_to === appliesTo && (appliesTo !== "plan" || kindApplies(d.kinds, kind)));
      read = values;
      fill(defs, values);
      loadError = null;
    } catch (err) {
      loadError = errorText(
        err,
        appliesTo === "plan" ? "p.custom_fields.values_could_not_load" : "p.custom_fields.task_values_could_not_load",
      );
    }
  }
  onMount(load);

  const withdrawn = $derived(
    Object.entries(read?.values ?? {}).filter(([, v]) => v.definition === "withdrawn"),
  );
  const missing = $derived(
    (read?.missing_required ?? []).map((k) => defs?.find((d) => d.key === k)?.label ?? k),
  );

  async function save(e: SubmitEvent) {
    e.preventDefault();
    notice = error = null;
    if (!defs || !read) return;
    const r = valueChanges(defs, inputs, read.values);
    if ("problems" in r) {
      problems = r.problems;
      return;
    }
    problems = [];
    if (Object.keys(r.values).length === 0) {
      notice = t("p.custom_fields.values_unchanged");
      return;
    }
    busy = true;
    try {
      read = await ppm.setCustomFieldValues({ applies_to: appliesTo, subject_pid: pid, values: r.values });
      fill(defs, read);
      notice = t("p.custom_fields.values_saved");
    } catch (err) {
      error = errorText(err, "p.custom_fields.values_could_not_save");
    } finally {
      busy = false;
    }
  }

  function toggle(key: string, option: string, on: boolean) {
    const cur = inputs[key]?.many ?? [];
    inputs[key] = { text: "", many: on ? [...cur, option] : cur.filter((o) => o !== option) };
  }
</script>

{#if loadError}<p class="banner" role="alert">{loadError}</p>{/if}
{#if error}<p class="banner error" role="alert" data-testid="fv-error">{error}</p>{/if}
{#if notice}<p class="banner" role="status" data-testid="fv-notice">{notice}</p>{/if}

{#if !defs && !loadError}
  <p class="muted">{t("p.custom_fields.loading")}</p>
{:else if defs && read}
  {#if missing.length > 0}
    <p class="banner" data-testid="fv-missing">{t("p.custom_fields.values_missing", { fields: missing.join(", ") })}</p>
  {/if}
  {#if defs.length === 0}
    <p data-testid="fv-none">
      {appliesTo === "plan" ? t("p.custom_fields.values_none") : t("p.custom_fields.task_values_none")}
      <a href={localeHref("/custom-fields")}>{t("p.custom_fields.title")}</a>
    </p>
  {:else}
    <form class="surface stack" onsubmit={save} aria-labelledby={`fv-h-${appliesTo}`} novalidate>
      <svelte:element this={heading} id={`fv-h-${appliesTo}`}>{t("p.custom_fields.values_h")}</svelte:element>
      {#if problems.length > 0}
        <ul class="banner error" role="alert" data-testid="fv-problems">
          {#each problems as p (p.label + p.key)}<li>{tDynamic(p.key, { label: p.label })}</li>{/each}
        </ul>
      {/if}
      {#each defs as d (d.pid)}
        {@const name = d.required ? t("p.custom_fields.values_required_label", { label: d.label }) : d.label}
        {#if d.field_type === "multi_list"}
          <fieldset>
            <legend>{name}</legend>
            {#each d.options as o (o)}
              <label class="check"
                ><input
                  type="checkbox"
                  checked={inputs[d.key]?.many.includes(o)}
                  onchange={(e) => toggle(d.key, o, e.currentTarget.checked)}
                /> {o}</label
              >
            {/each}
          </fieldset>
        {:else if inputs[d.key]}
          <label
            >{name}
            {#if d.field_type === "bool"}
              <select bind:value={inputs[d.key]!.text}>
                <option value="">{t("p.custom_fields.values_unset")}</option>
                <option value="true">{t("p.custom_fields.yes")}</option>
                <option value="false">{t("p.custom_fields.no")}</option>
              </select>
            {:else if d.field_type === "list"}
              <select bind:value={inputs[d.key]!.text}>
                <option value="">{t("p.custom_fields.values_unset")}</option>
                {#each d.options as o (o)}<option value={o}>{o}</option>{/each}
              </select>
            {:else if d.field_type === "date"}
              <input type="date" bind:value={inputs[d.key]!.text} />
            {:else if d.field_type === "integer"}
              <input inputmode="numeric" bind:value={inputs[d.key]!.text} />
            {:else}
              <textarea rows="2" maxlength={MAX_TEXT} bind:value={inputs[d.key]!.text}></textarea>
            {/if}
          </label>
        {/if}
      {/each}
      <button class="button primary" type="submit" disabled={busy}>{t("p.custom_fields.values_save")}</button>
    </form>
  {/if}

  {#if withdrawn.length > 0}
    <section class="surface stack" aria-labelledby={`fv-w-h-${appliesTo}`}>
      <svelte:element this={heading} id={`fv-w-h-${appliesTo}`}>{t("p.custom_fields.values_withdrawn_h")}</svelte:element>
      <ul data-testid="fv-withdrawn">
        {#each withdrawn as [k, v] (k)}
          <li>
            <strong>{v.label ?? k}</strong>: {shownValue(v.value, t("p.custom_fields.yes"), t("p.custom_fields.no"))}
          </li>
        {/each}
      </ul>
    </section>
  {/if}
{/if}

<style>
  label {
    display: block;
  }
  fieldset {
    border: 0;
    padding: 0;
    margin: 0;
  }
</style>
