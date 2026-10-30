// The top navigation's destinations, shared by the layout (which draws them) and the Display page (where a person
// chooses which to hide, BR-17). `href` is the identity of a section: a hidden section is stored by it.

import { COLLECTIONS } from "#lib/api/types.js";
import { t } from "#lib/i18n.svelte.js";

export interface NavEntry {
  href: string;
  label: string;
}

/** Every destination, in the order the bar shows them, labelled in the current language. */
export function buildNavItems(): NavEntry[] {
  return [
    { href: "/", label: t("brand.name") },
    ...COLLECTIONS.map((collection) => ({
      href: `/${collection}`,
      get label() {
        return t(`c.NavItems.${collection}`);
      },
    })),
    // Record merge (fold a confirmed duplicate plan into a survivor).
    { href: "/plans/merge", label: t("nav.merge") },
    // PPM catalogue views (fully localized 2026-07-18).
    // LT-3: the individual worker's personal task view. A deployment opts a
    // persona into it as its landing page with `view=me` (T-28f).
    {
      href: "/me",
      get label() {
        return t("p.me.my_work");
      },
    },
    { href: "/dashboard", label: t("ppm.nav.dashboard") },
    {
      href: "/prioritization",
      get label() {
        return t("p.prioritization.prioritization");
      },
    },
    {
      href: "/lifecycle",
      get label() {
        return t("p.lifecycle.lifecycle");
      },
    },
    {
      href: "/reviews",
      get label() {
        return t("p._layout_svelte.reviews");
      },
    },
    {
      href: "/automations",
      get label() {
        return t("p._layout_svelte.automations");
      },
    },
    { href: "/proposals", label: t("ppm.nav.proposals") },
    { href: "/ideas", label: t("ppm.nav.ideas") },
    { href: "/scenarios", label: t("ppm.nav.scenarios") },
    { href: "/objectives", label: t("ppm.nav.objectives") },
    {
      href: "/okrs",
      get label() {
        return t("p._layout_svelte.goals_okrs");
      },
    },
    {
      href: "/milestones",
      get label() {
        return t("p.milestones.milestones");
      },
    },
    {
      href: "/kpis",
      get label() {
        return t("p.kpis.key_measures_kpis");
      },
    },
    { href: "/gantt", label: t("ppm.nav.gantt") },
    { href: "/engineering", label: t("ppm.nav.engineering") },
    { href: "/calendar", label: t("ppm.nav.calendar") },
    {
      href: "/ceo",
      get label() {
        return t("p.ceo.ceo_dashboard");
      },
    },
    {
      href: "/announcements",
      get label() {
        return t("p.ceo.announcements");
      },
    },
    { href: "/executive", label: t("ppm.nav.executive") },
    { href: "/financials", label: t("ppm.nav.financials") },
    { href: "/technology", label: t("ppm.nav.technology") },
    { href: "/board", label: t("ppm.nav.board") },
    { href: "/auditor", label: t("ppm.nav.auditor") },
    { href: "/compliance", label: t("ppm.nav.compliance") },
    { href: "/risk", label: t("ppm.nav.risk") },
    {
      href: "/issues",
      get label() {
        return t("p.issues.issues");
      },
    },
    {
      href: "/change-requests",
      get label() {
        return t("p.change_requests.change_requests");
      },
    },
    {
      href: "/director-packs",
      get label() {
        return t("p.director_pack.director_packs");
      },
    },
    {
      href: "/pipeline",
      get label() {
        return t("p.critical_chain.nav_pipeline");
      },
    },
    {
      href: "/start-queue",
      get label() {
        return t("p.start_queue.nav");
      },
    },
    {
      href: "/kanban",
      get label() {
        return t("p.kanban.nav");
      },
    },
    {
      href: "/ledger",
      get label() {
        return t("p.ledger.nav");
      },
    },
    {
      href: "/milestone-signoffs",
      get label() {
        return t("p.signoffs.nav");
      },
    },
    {
      href: "/key-persons",
      get label() {
        return t("p.key_persons.nav");
      },
    },
    {
      href: "/organizations",
      get label() {
        return t("p.organizations.nav");
      },
    },
    {
      href: "/email-deliveries",
      get label() {
        return t("p.email_deliveries.nav");
      },
    },
    { href: "/security", label: t("ppm.nav.security") },
    { href: "/regulator", label: t("ppm.nav.regulator") },
    { href: "/capacity", label: t("ppm.nav.capacity") },
    // FB-6: demand and supply by skill pool and month, with the workforce service's supply for a linked pool.
    {
      href: "/capacity-heatmap",
      get label() {
        return t("p.capacity_heatmap.nav");
      },
    },
    {
      href: "/skill-pools",
      get label() {
        return t("p.skill_pools.nav");
      },
    },
    {
      href: "/resources",
      get label() {
        return t("p.resources.resources");
      },
    },
    {
      href: "/cost-risk",
      get label() {
        return t("p.plans.pid.joint_risk.cost_risk");
      },
    },
    {
      href: "/cost-rates",
      get label() {
        return t("p.plans.pid.costs.cost_rates");
      },
    },
    { href: "/reports", label: t("ppm.nav.reports") },
    {
      href: "/onboarding",
      get label() {
        return t("p._layout_svelte.onboarding");
      },
    },
    {
      href: "/documents",
      get label() {
        return t("p.documents.documents");
      },
    },
    {
      href: "/approval-processes",
      get label() {
        return t("p.processes.processes");
      },
    },
    {
      href: "/balance",
      get label() {
        return t("p.balance.balance");
      },
    },
    {
      href: "/tolerances",
      get label() {
        return t("p.tolerances.tolerances");
      },
    },
    {
      href: "/escalations",
      get label() {
        return t("p.escalations.escalations");
      },
    },
    { href: "/tour", label: t("nav.tour") },
  ];
}
