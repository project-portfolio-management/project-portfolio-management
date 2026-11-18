<!--
  A plan's benefits: classification and validation (`/plans/[pid]/benefits`, DP-9, NC-1 slice 11). Each benefit says
  whether it recurs, its yearly amount, its classification and, for a non-financial one, the KPI that measures it. A
  realized amount is a claim; it is validated only by a recorded validation from someone outside the programme, each
  stating the amount validated to date, and the latest counts. The service decides who may; a refusal is the
  catalogue's sentence. Below (NC-1 slice 12 part f), the plan's realized value read live (return on investment, time
  to value, adoption, the business case) and an observed value recorded (`value-view.ts`, said first).
-->
<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { errorText } from "#lib/error-text.js";
  import { enumLabel } from "#lib/enum-labels.js";
  import { localeHref, t, tn } from "#lib/i18n.svelte.js";
  import { formatBasisPoints, formatDate, formatInteger } from "#lib/locale-format.js";
  import { money } from "#lib/money.js";
  import { PpmClient, type BenefitBoard, type BenefitValidations, type KpiView, type ValueRealization } from "#lib/api/ppm.js";
  import { askConfirm } from "#lib/confirm.svelte.js";
  import {
    METHODS,
    METHOD_KEY,
    absentKey,
    sourceKey,
    valuePointBody,
    valuePointRefusalKey,
    type ValuePointForm,
  } from "#lib/value-view.js";
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

  // The plan's realized value, read live, and an observed value (NC-1 slice 12 part f).
  let value = $state<ValueRealization | null>(null);
  let valueError = $state<string | null>(null);
  let pointForm = $state<ValuePointForm>({ value: "", currency: "", method: "measured", first: false, evidence: "" });
  let pointProblem = $state<string | null>(null);
  let pointNotice = $state("");
  async function loadValue() {
    try {
      value = await ppm.valueRealization(pid);
      valueError = null;
    } catch (err) {
      valueError = errorText(err, "p.value.could_not_read");
    }
  }
  async function recordPoint(e: SubmitEvent) {
    e.preventDefault();
    pointNotice = "";
    const r = valuePointBody(pointForm);
    if ("problems" in r) {
      pointProblem = r.problems.map((k) => t(k)).join(" ");
      return;
    }
    pointProblem = null;
    if (r.body.is_first_measurable && !(await askConfirm(t("p.value.confirm_first")))) return;
    busy = true;
    try {
      await ppm.recordValuePoint(pid, r.body);
      pointNotice = t("p.value.point_recorded");
      pointForm = { value: "", currency: pointForm.currency, method: pointForm.method, first: false, evidence: "" };
      await loadValue();
    } catch (err) {
      const k = valuePointRefusalKey(err, Boolean(r.body.is_first_measurable));
      pointProblem = k ? t(k) : errorText(err, "p.value.could_not_record_point");
    } finally {
      busy = false;
    }
  }

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
    void loadValue();
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

<section aria-labelledby="value-h" data-testid="value">
  <h2 id="value-h">{t("p.value.heading")}</h2>
  <p class="small muted">{t("p.value.says")}</p>
  {#if valueError}<p class="banner error" role="alert">{valueError}</p>{/if}
  {#if value}
    {@const roi = value.transformation_roi}
    {@const ttv = value.time_to_value}
    {@const ad = value.adoption}
    <ul class="small figures">
      <li data-testid="value-roi">
        {#if roi.basis_points !== null}
          {t("p.value.roi", { pct: formatBasisPoints(roi.basis_points, 1) })}
        {:else}
          {t("p.value.roi_absent", { reason: t(absentKey(roi.absent)) })}
        {/if}
        {#if roi.measured_share_basis_points !== null}{` · ${t("p.value.measured_share", { pct: formatBasisPoints(roi.measured_share_basis_points, 0) })}`}{/if}
      </li>
      <li data-testid="value-ttv">
        {#if ttv.absent === null && ttv.p50_days !== null}
          {tn("p.value.ttv", ttv.observations, { p50: formatInteger(ttv.p50_days), p85: formatInteger(ttv.p85_days ?? ttv.p50_days), n: ttv.observations })}
        {:else}
          {t("p.value.ttv_absent", { reason: t(absentKey(ttv.absent)) })}
        {/if}
      </li>
      <li data-testid="value-adoption">
        {#if ad.measured && ad.basis_points != null}
          {t("p.value.adoption", { pct: formatBasisPoints(ad.basis_points, 0), active: formatInteger(ad.active_users ?? 0), target: formatInteger(ad.target_users ?? 0), days: formatInteger(ad.window_days ?? 0) })}{#if ad.definition}{` · ${t("p.value.adoption_definition", { definition: ad.definition })}`}{/if}
        {:else if ad.absent}
          {t("p.value.adoption_absent", { reason: t(absentKey(ad.absent)) })}
        {:else}
          {t("p.value.adoption_none")}
        {/if}
      </li>
    </ul>
    <h3>{t("p.value.case_heading")}</h3>
    {#if value.performance_to_business_case.length === 0}
      <p class="small muted">{t("p.value.no_targets")}</p>
    {:else}
      <ul class="small" data-testid="value-targets">
        {#each value.performance_to_business_case as c, i (i)}
          <li>{t("p.value.target_line", { metric: c.metric, baseline: formatInteger(c.baseline_value), target: formatInteger(c.target_value), source: t(sourceKey(c.source)), date: formatDate(c.approved_at) })}</li>
        {/each}
      </ul>
    {/if}
    <p class="small"><a href={localeHref(`/plans/${pid}/costs`)}>{t("p.value.to_costs")}</a></p>
  {/if}

  <h3>{t("p.value.point_heading")}</h3>
  <p class="small muted">{t("p.value.point_says")}</p>
  <form class="grid" novalidate onsubmit={recordPoint} data-testid="value-point-form">
    <label>{t("p.value.f_value")}<input type="text" inputmode="decimal" bind:value={pointForm.value} /></label>
    <label>{t("p.value.f_currency")}<input bind:value={pointForm.currency} maxlength="3" size="4" placeholder={t("p.money.currency_hint")} /></label>
    <label>
      {t("p.value.f_method")}
      <select bind:value={pointForm.method}>
        {#each METHODS as m (m)}<option value={m}>{t(METHOD_KEY[m])}</option>{/each}
      </select>
    </label>
    <label>{t("p.value.f_evidence")}<input bind:value={pointForm.evidence} /></label>
    <label class="check"><input type="checkbox" bind:checked={pointForm.first} /> {t("p.value.f_first")}</label>
    <div class="row"><button class="button small" type="submit" disabled={busy}>{t("p.value.record_point")}</button></div>
  </form>
  {#if pointProblem}<p class="banner error" role="alert">{pointProblem}</p>{/if}
  {#if pointNotice}<p class="small" role="status">{pointNotice}</p>{/if}
</section>

<style>
  #value-h {
    margin-block-start: 2rem;
  }
  .figures {
    padding-inline-start: 1.25rem;
  }
  .grid .check {
    display: flex;
    align-items: center;
    gap: 0.35rem;
  }
  .check input {
    inline-size: 1.5rem;
    block-size: 1.5rem;
    margin: 0;
  }
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
