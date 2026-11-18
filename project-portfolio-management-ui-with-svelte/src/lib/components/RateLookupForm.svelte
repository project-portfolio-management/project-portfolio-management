<!--
  The cost rate in force for a person on a day (`/resources`, NC-1 slice 12 part j), read on asking: `GET
  /api/cost-rates/lookup` (the person and the day checked first; the `standard` table not sent). The rate per hour in
  its currency, from when it applies and whether it is the person's own table or their role's; none is said, never a
  zero. Rates are shown to finance only: anyone else is answered as if there were none to see. Rules in
  `src/lib/resource-actions-view.ts`.
-->
<script lang="ts">
  import { errorText } from "#lib/error-text.js";
  import { t } from "#lib/i18n.svelte.js";
  import { formatDate } from "#lib/locale-format.js";
  import { money } from "#lib/money.js";
  import { PpmClient, type RateLookup } from "#lib/api/ppm.js";
  import { rateQuery, rateRefusalKey } from "#lib/resource-actions-view.js";

  const ppm = PpmClient.withFetch();
  let person = $state("");
  let date = $state("");
  let table = $state("standard");
  let data = $state<RateLookup | null>(null);
  let problem = $state<string | null>(null);
  let busy = $state(false);

  async function read(e: SubmitEvent) {
    e.preventDefault();
    const r = rateQuery(person, date, table);
    if ("problems" in r) {
      problem = r.problems.map((k) => t(k)).join(" ");
      return;
    }
    problem = null;
    busy = true;
    try {
      data = await ppm.rateLookup(r.query);
    } catch (err) {
      data = null;
      const key = rateRefusalKey(err);
      problem = key ? t(key) : errorText(err, "p.rate.could_not");
    } finally {
      busy = false;
    }
  }
</script>

<section aria-labelledby="rate-h" data-testid="rate-lookup">
  <h2 id="rate-h">{t("p.rate.heading")}</h2>
  <p class="small muted">{t("p.rate.says")}</p>
  <form class="row" novalidate onsubmit={read} aria-label={t("p.rate.form")}>
    <label>{t("p.rate.f_person")}<input type="text" bind:value={person} /></label>
    <label>{t("p.rate.f_date")}<input type="date" bind:value={date} /></label>
    <label>{t("p.rate.f_table")}<input type="text" bind:value={table} /></label>
    <button class="button" type="submit" disabled={busy}>{t("p.rate.read")}</button>
  </form>
  {#if problem}<p class="banner error" role="alert">{problem}</p>{/if}
  {#if data}
    {#if data.rate}
      <p data-testid="rate-answer">
        {t(data.rate.source === "person" ? "p.rate.answer_person" : "p.rate.answer_role", {
          rate: money(data.rate.per_hour_minor, data.rate.currency),
          from: data.rate.effective_from ? formatDate(data.rate.effective_from) : "—",
          date: formatDate(data.date),
        })}
      </p>
    {:else}
      <p data-testid="rate-answer">{t("p.rate.none", { date: formatDate(data.date) })}</p>
    {/if}
  {/if}
</section>

<style>
  section { margin-block: 1.5rem; }
  .row { display: flex; gap: 0.75rem; align-items: end; flex-wrap: wrap; }
  .row label { display: grid; gap: 0.2rem; }
</style>
