<!--
  A plan's benefits: classification and validation (`/plans/[pid]/benefits`, DP-9, NC-1 slice 11). Each benefit says
  whether it recurs, its yearly amount, its classification and, for a non-financial one, the KPI that measures it. A
  realized amount is a claim; it is validated only by a recorded validation from someone outside the programme, each
  stating the amount validated to date, and the latest counts. The service decides who may; a refusal is the
  catalogue's sentence.
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { errorText } from "#lib/error-text.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { localeHref, t } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { money } from "#lib/money.js";
  import { PpmClient, type BenefitBoard, type BenefitValidations, type KpiView } from "#lib/api/ppm.js";
  import {
    STANDING_LABELS,
    classifyBody,
    formOf,
    validationBody,
    validationRefusalKey,
    type ClassifyForm,
  } from "#lib/benefit-view.js";

  type Benefit = BenefitBoard["benefits"][number];

  const ppm = PpmClient.withFetch();
  const pid = $derived(page.params.pid ?? "");

  let benefits = $state<Benefit[] | null>(null);
  let kpis = $state<KpiView[]>([]);
  let loadError = $state<string | null>(null);
  let forms = $state<Record<string, ClassifyForm>>({});
  let validations = $state<Record<string, BenefitValidations>>({});
  let amounts = $state<Record<string, string>>({});
  let notes = $state<Record<string, string>>({});
  let errors = $state<Record<string, string>>({});
  let notices = $state<Record<string, string>>({});
  let busy = $state(false);

  async function loadBenefits() {
    benefits = (await ppm.benefits(pid)).benefits;
    forms = Object.fromEntries(benefits.map((b) => [b.pid, formOf(b)]));
  }
  async function loadValidations(b: Benefit) {
    if (!b.currency) return;
    try {
      validations = { ...validations, [b.pid]: await ppm.benefitValidations(pid, b.pid) };
    } catch {
      // the validation history is then not shown for this benefit; the classification still works
    }
  }
  onMount(async () => {
    try {
      await loadBenefits();
      for (const b of benefits ?? []) void loadValidations(b);
    } catch (err) {
      loadError = errorText(err, "p.benefits.could_not_load");
    }
    ppm.kpis({ plan: pid }).then((r) => (kpis = r.kpis)).catch(() => (kpis = []));
  });

  function say(b: Benefit, error: string | null, notice = "") {
    errors = { ...errors, [b.pid]: error ?? "" };
    notices = { ...notices, [b.pid]: notice };
  }

  async function classify(e: SubmitEvent, b: Benefit) {
    e.preventDefault();
    const r = classifyBody(forms[b.pid]!, b);
    if ("problems" in r) {
      say(b, r.problems.map((k) => t(k)).join(" "));
      return;
    }
    busy = true;
    try {
      await ppm.classifyBenefit(pid, b.pid, r.body);
      await loadBenefits();
      say(b, null, t("p.benefits.classified"));
    } catch (err) {
      say(b, errorText(err, "p.benefits.could_not_save"));
    } finally {
      busy = false;
    }
  }

  async function validate(e: SubmitEvent, b: Benefit) {
    e.preventDefault();
    const r = validationBody(amounts[b.pid] ?? "", b.currency ?? "", notes[b.pid] ?? "");
    if ("problem" in r) {
      say(b, t(r.problem));
      return;
    }
    busy = true;
    try {
      await ppm.validateBenefit(pid, b.pid, r.body);
      amounts = { ...amounts, [b.pid]: "" };
      notes = { ...notes, [b.pid]: "" };
      await loadValidations(b);
      say(b, null, t("p.benefits.validated_done"));
    } catch (err) {
      const k = validationRefusalKey(err);
      say(b, k ? t(k) : errorText(err, "p.benefits.could_not_validate"));
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head><title>{t("p.benefits.title_ppm")}</title></svelte:head>

<p class="small"><a href={localeHref(`/plans/${pid}`)}>{t("p.plan_modules.back")}</a></p>
<h1>{t("p.benefits.title")}</h1>
<p class="small muted">{t("p.benefits.intro")}</p>
{#if loadError}<p class="banner error" role="alert">{loadError}</p>{/if}

{#if benefits}
  {#if benefits.length === 0}
    <p class="muted">{t("p.benefits.none")}</p>
  {:else}
    <ul class="benefits">
      {#each benefits as b (b.pid)}
        {@const v = validations[b.pid]}
        <li>
          <article aria-labelledby={`ben-${b.pid}`}>
            <h2 id={`ben-${b.pid}`}>{b.title}</h2>
            <p class="small">
              {#if b.currency}
                {t("p.benefits.money_line", {
                  target: b.target_minor != null ? money(b.target_minor, b.currency) : "–",
                  realized: money(b.realized_minor, b.currency),
                })}
              {:else}
                {t("p.benefits.non_financial")}
              {/if}
            </p>
            {#if errors[b.pid]}<p class="banner error" role="alert">{errors[b.pid]}</p>{/if}
            <p class="small" role="status">{notices[b.pid] ?? ""}</p>

            <form class="grid" novalidate onsubmit={(e) => classify(e, b)}>
              <label>
                {t("p.benefits.f_recurrence")}
                <select bind:value={forms[b.pid]!.recurrence}>
                  <option value="">{enumLabel("pk_recur", "not_stated")}</option>
                  <option value="recurring">{enumLabel("pk_recur", "recurring")}</option>
                  <option value="one_off">{enumLabel("pk_recur", "one_off")}</option>
                </select>
              </label>
              {#if b.currency && forms[b.pid]!.recurrence === "recurring"}
                <label>
                  {t("p.benefits.f_annual", { currency: b.currency })}
                  <input type="text" inputmode="decimal" bind:value={forms[b.pid]!.annual} />
                </label>
              {/if}
              <label>
                {t("p.benefits.f_classification")}
                <select bind:value={forms[b.pid]!.classification}>
                  <option value="">{enumLabel("pk_cls", "not_stated")}</option>
                  <option value="cash_releasing">{enumLabel("pk_cls", "cash_releasing")}</option>
                  <option value="non_cash_releasing">{enumLabel("pk_cls", "non_cash_releasing")}</option>
                  <option value="societal">{enumLabel("pk_cls", "societal")}</option>
                </select>
              </label>
              {#if !b.currency}
                <label>
                  {t("p.benefits.f_kpi")}
                  <select bind:value={forms[b.pid]!.kpi}>
                    <option value="">{t("p.benefits.no_kpi")}</option>
                    {#each kpis as k (k.kpi.pid)}<option value={k.kpi.pid}>{k.kpi.name}</option>{/each}
                  </select>
                </label>
              {/if}
              <div class="row"><button class="button small" type="submit" disabled={busy}>{t("p.benefits.save")}</button></div>
            </form>
            <p class="small muted">{t("p.benefits.classify_says")}</p>

            {#if b.currency}
              <section aria-labelledby={`val-${b.pid}`}>
                <h3 id={`val-${b.pid}`}>{t("p.benefits.validation_heading")}</h3>
                {#if v}
                  <p>
                    <strong>{t(STANDING_LABELS[v.standing] ?? "p.benefits.standing_claimed")}</strong>
                    {#if v.validated_minor != null} · {t("p.benefits.validated_to_date", { amount: money(v.validated_minor, b.currency) })}{/if}
                  </p>
                  {#if v.history.length > 0}
                    <ol class="small history" reversed>
                      {#each [...v.history].reverse() as h, i (i)}
                        <li>{money(h.amount_minor, h.currency)} · {formatDate(h.validated_on)}{#if h.note} · {h.note}{/if}</li>
                      {/each}
                    </ol>
                  {/if}
                {/if}
                <form class="grid" novalidate onsubmit={(e) => validate(e, b)}>
                  <label>
                    {t("p.benefits.f_validated", { currency: b.currency })}
                    <input type="text" inputmode="decimal" bind:value={amounts[b.pid]} />
                  </label>
                  <label>{t("p.benefits.f_note")}<input type="text" bind:value={notes[b.pid]} /></label>
                  <div class="row"><button class="button small" type="submit" disabled={busy}>{t("p.benefits.validate")}</button></div>
                </form>
                <p class="small muted">{t("p.benefits.validation_says")}</p>
              </section>
            {:else}
              <p class="small muted">{t("p.benefits.measured_by_kpi")}</p>
            {/if}
          </article>
        </li>
      {/each}
    </ul>
  {/if}
{/if}

<style>
  .benefits {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 1.5rem;
  }
  article h2 {
    font-size: 1.1rem;
    margin-block: 0 0.25rem;
  }
  article p {
    margin-block: 0 0.25rem;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));
    gap: 0.75rem;
    align-items: end;
    margin-block: 0.5rem;
  }
  .grid label {
    display: grid;
    gap: 0.25rem;
  }
  .history {
    padding-inline-start: 1.5rem;
  }
</style>
