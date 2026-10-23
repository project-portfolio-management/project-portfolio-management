import { t } from "./i18n.svelte.js";
// Operator onboarding guide (T-28p): a role-by-role "first hour"
// walkthrough, each step naming the route it lands on. Pure data —
// no API calls, no claim about how long any of it actually takes.
//
// This makes **no time-to-productivity claim**: nobody has measured
// how long a first hour with this guide actually takes, and a guide
// that invented a number would be exactly the kind of confident-
// looking figure this family's other views refuse to produce from
// nothing. See `spec/index.md` §13 T-28p.

export type OnboardingStep = {
  /** The route this step lands on — a real page in this app. */
  route: string;
  /** What the operator does or looks at on this page. */
  label: string;
  /** Why this step matters for this role. */
  note: string;
};

export type OnboardingRole = {
  id: string;
  title: string;
  summary: string;
  steps: OnboardingStep[];
};

export const ONBOARDING_ROLES: OnboardingRole[] = [
  {
    id: "executive",
    get title() {
      return t("l.onboarding.executive");
    },
    get summary() {
      return t("l.onboarding.outcomes_and_exposure_across_the");
    },
    steps: [
      {
        route: "/executive",
        get label() {
          return t("l.onboarding.read_the_executive_summary");
        },
        get note() {
          return t("l.onboarding.health_decisions_benefits_realized_a");
        },
      },
      {
        route: "/dashboard",
        get label() {
          return t("l.onboarding.open_the_at_a_glance");
        },
        get note() {
          return t("l.onboarding.the_whole_estate_s_rag");
        },
      },
      {
        route: "/board",
        get label() {
          return t("l.onboarding.review_the_board_pack");
        },
        get note() {
          return t("l.onboarding.the_pack_a_board_meeting");
        },
      },
      {
        route: "/financials",
        get label() {
          return t("l.onboarding.check_budget_variance_and_exposure");
        },
        get note() {
          return t("l.onboarding.where_spend_is_diverging_from");
        },
      },
      {
        route: "/scenarios",
        get label() {
          return t("l.onboarding.compare_two_candidate_portfolios");
        },
        get note() {
          return t("l.onboarding.what_if_scenarios_side_by");
        },
      },
      {
        route: "/prioritization",
        get label() {
          return t("l.onboarding.see_what_s_ranked_highest");
        },
        get note() {
          return t("l.onboarding.the_smart_score_queue_every");
        },
      },
    ],
  },
  {
    id: "pmo",
    get title() {
      return t("l.onboarding.pmo_portfolio_manager");
    },
    get summary() {
      return t("l.onboarding.intake_governance_and_the_schedule");
    },
    steps: [
      {
        route: "/proposals",
        get label() {
          return t("l.onboarding.open_the_intake_pipeline");
        },
        get note() {
          return t("l.onboarding.every_proposal_s_stage_from");
        },
      },
      {
        route: "/plans/new",
        get label() {
          return t("l.onboarding.register_a_plan");
        },
        get note() {
          return t("l.onboarding.the_first_plan_you_create");
        },
      },
      {
        route: "/plans",
        get label() {
          return t("l.onboarding.browse_the_plan_registry");
        },
        get note() {
          return t("l.onboarding.one_recursive_collection_kind_is");
        },
      },
      {
        route: "/plans/[pid]/governance",
        get label() {
          return t("l.onboarding.open_a_plan_s_governance");
        },
        get note() {
          return t("l.onboarding.gate_reviews_risks_and_budget");
        },
      },
      {
        route: "/gantt",
        get label() {
          return t("l.onboarding.look_at_the_cross_plan");
        },
        get note() {
          return t("l.onboarding.dependencies_the_critical_path_and");
        },
      },
      {
        route: "/risk",
        get label() {
          return t("l.onboarding.check_the_risk_register");
        },
        get note() {
          return t("l.onboarding.everything_open_or_mitigating_ranked");
        },
      },
      {
        route: "/compliance",
        get label() {
          return t("l.onboarding.review_the_compliance_register");
        },
        get note() {
          return t("l.onboarding.standing_findings_across_the_estate");
        },
      },
      {
        route: "/reports",
        get label() {
          return t("l.onboarding.run_a_saved_report");
        },
        get note() {
          return t("l.onboarding.a_filter_field_projection_saved");
        },
      },
    ],
  },
  {
    id: "resource-manager",
    get title() {
      return t("l.onboarding.resource_manager");
    },
    get summary() {
      return t("l.onboarding.who_is_allocated_where_and");
    },
    steps: [
      {
        route: "/capacity",
        get label() {
          return t("l.onboarding.check_the_capacity_rollup");
        },
        get note() {
          return t("l.onboarding.who_is_over_allocated_this");
        },
      },
      {
        route: "/plans/[pid]/board",
        get label() {
          return t("l.onboarding.open_a_plan_s_task");
        },
        get note() {
          return t("l.onboarding.the_kanban_board_that_task");
        },
      },
      {
        route: "/plans/[pid]/flow",
        get label() {
          return t("l.onboarding.read_that_plan_s_flow");
        },
        get note() {
          return t("l.onboarding.cycle_time_lead_time_and");
        },
      },
      {
        route: "/engineering",
        get label() {
          return t("l.onboarding.scan_blocked_items_and_the");
        },
        get note() {
          return t("l.onboarding.what_s_stuck_the_moscow");
        },
      },
      {
        route: "/calendar",
        get label() {
          return t("l.onboarding.look_at_the_milestone_calendar");
        },
        get note() {
          return t("l.onboarding.every_milestone_across_every_plan");
        },
      },
      {
        route: "/automations",
        get label() {
          return t("l.onboarding.see_what_automations_are_configured");
        },
        get note() {
          return t("l.onboarding.what_fires_on_a_field");
        },
      },
      {
        route: "/reviews",
        get label() {
          return t("l.onboarding.open_collaborative_reviews");
        },
        get note() {
          return t("l.onboarding.where_a_plan_gets_a");
        },
      },
    ],
  },
];
