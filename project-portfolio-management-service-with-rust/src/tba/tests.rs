use super::*;

const T0: i64 = 1_700_000_000_000;

fn at(days: i64) -> i64 {
    T0 + days * DAY_MS
}

fn tr(from: Option<&str>, to: &str, day: i64) -> Transition {
    Transition {
        from_status: from.map(ToString::to_string),
        to_status: to.to_string(),
        at_ms: at(day),
        assignee_ref: None,
        backfilled: false,
        blocked_reason: None,
    }
}

fn classes() -> BTreeMap<String, String> {
    default_classes()
}

// -- the cycle-versus-lead-time distinction (§6.1) --------------------

#[test]
fn cycle_time_and_lead_time_are_different_numbers() {
    // Created day 0, sat in `todo` for 21 calendar days, built in 2.
    // Cycle 2, lead 23. Quoting the cycle time as the delivery time
    // is a tenfold flattering misreport — this is the regression
    // test against ever conflating them.
    let transitions = vec![
        tr(None, "todo", 0),
        tr(Some("todo"), "in_progress", 21),
        tr(Some("in_progress"), "done", 23),
    ];
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: Some(at(23)),
    };
    let a = analyze(&transitions, clock, &classes(), at(23));
    assert_eq!(a.lead_time_ms, 23 * DAY_MS);
    assert_eq!(a.cycle_time_ms, Some(2 * DAY_MS));
    assert_eq!(
        a.queue_time_ms,
        21 * DAY_MS,
        "the backlog dwell is reported"
    );
    assert!(a.finished);
}

#[test]
fn a_task_that_never_started_has_no_cycle_time() {
    let transitions = vec![tr(None, "todo", 0)];
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: None,
    };
    let a = analyze(&transitions, clock, &classes(), at(10));
    assert_eq!(a.cycle_time_ms, None);
    assert!(a.cycle_time_reason.is_some(), "a null must say why");
    assert_eq!(
        a.lead_time_ms,
        10 * DAY_MS,
        "but it has been waiting 10 calendar days"
    );
    assert_eq!(
        a.age_ms, None,
        "age is measured from the start, and it has none"
    );
    assert_eq!(a.flow_efficiency.value, None);
}

// -- the partition (§6.3, §12.3) --------------------------------------

#[test]
fn statuses_partition_the_cycle_time() {
    let transitions = vec![
        tr(None, "todo", 0),
        tr(Some("todo"), "in_progress", 10),
        tr(Some("in_progress"), "in_review", 12),
        tr(Some("in_review"), "blocked", 15),
        tr(Some("blocked"), "in_progress", 20),
        tr(Some("in_progress"), "done", 21),
    ];
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: Some(at(21)),
    };
    let a = analyze(&transitions, clock, &classes(), at(21));
    let cycle = a.cycle_time_ms.expect("cycle time");
    assert_eq!(cycle, 11 * DAY_MS, "day 10 → 21");
    assert_eq!(a.lead_time_ms, 21 * DAY_MS, "creation → done");
    // The statuses partition the **lead** time, not the cycle time:
    // the 10 calendar days in the backlog are real time the requester waited
    // and have to land somewhere (§6.3).
    let status_total: i64 = a.by_status.iter().map(|s| s.ms).sum();
    assert_eq!(status_total, a.lead_time_ms, "statuses sum to lead time");
    let category_total: i64 = a.by_category.iter().map(|c| c.ms).sum();
    assert_eq!(category_total, a.lead_time_ms, "categories sum too");
    assert_eq!(a.queue_time_ms, 10 * DAY_MS, "and the dwell is named");
    // in_progress: day 10-12 and 20-21 = 3 calendar days of the 11.
    assert_eq!(a.work_time_ms, 3 * DAY_MS);
    assert_eq!(a.blocked_time_ms, 5 * DAY_MS, "day 15 → 20");
    let efficiency = a.flow_efficiency.value.expect("efficiency");
    assert!((efficiency - 3.0 / 11.0).abs() < 1e-9, "got {efficiency}");
    assert!((0.0..=1.0).contains(&efficiency));
}

fn blocked(day: i64, reason: Option<&str>) -> Transition {
    Transition {
        blocked_reason: reason.map(ToString::to_string),
        ..tr(Some("in_progress"), "blocked", day)
    }
}

#[test]
fn blocked_time_is_split_by_the_reason_given_and_sums_to_the_total() {
    let transitions = vec![
        tr(None, "todo", 0),
        tr(Some("todo"), "in_progress", 1),
        blocked(2, Some("external_dependency")),
        tr(Some("blocked"), "in_progress", 5),
        blocked(6, None),
        tr(Some("blocked"), "in_progress", 8),
        blocked(9, Some("external_dependency")),
        tr(Some("blocked"), "done", 10),
    ];
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: Some(at(10)),
    };
    let a = analyze(&transitions, clock, &classes(), at(10));
    assert_eq!(a.blocked_by_reason["external_dependency"], 4 * DAY_MS);
    assert_eq!(a.blocked_by_reason[UNRECORDED_REASON], 2 * DAY_MS);
    assert_eq!(
        a.blocked_by_reason.values().sum::<i64>(),
        a.blocked_time_ms,
        "the parts are the whole"
    );
}

#[test]
fn a_blocked_reason_is_allowed_only_on_a_move_into_blocked_and_only_from_the_list() {
    assert!(check_blocked_reason("in_progress", None).is_ok());
    assert!(check_blocked_reason("blocked", None).is_ok());
    for reason in BLOCKED_REASONS {
        assert!(check_blocked_reason("blocked", Some(reason)).is_ok());
    }
    assert!(check_blocked_reason("blocked", Some("lunch")).is_err());
    assert!(check_blocked_reason("in_progress", Some("other")).is_err());
    assert!(check_blocked_reason("done", Some("other")).is_err());
}

#[test]
fn done_is_terminal_and_does_not_accrue() {
    let transitions = vec![
        tr(None, "todo", 0),
        tr(Some("todo"), "in_progress", 1),
        tr(Some("in_progress"), "done", 3),
    ];
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: Some(at(3)),
    };
    // Analysed a hundred calendar days later: the clock must not have run on.
    let a = analyze(&transitions, clock, &classes(), at(103));
    assert_eq!(a.cycle_time_ms, Some(2 * DAY_MS));
    assert!(
        !a.by_status.iter().any(|s| s.status == FINISHED_STATUS),
        "`done` is not an interval"
    );
}

#[test]
fn an_open_item_accrues_to_as_of_and_reports_its_age() {
    let transitions = vec![tr(None, "todo", 0), tr(Some("todo"), "in_progress", 2)];
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: None,
    };
    let a = analyze(&transitions, clock, &classes(), at(9));
    assert!(!a.finished);
    assert_eq!(a.cycle_time_ms, Some(7 * DAY_MS), "day 2 → now");
    assert_eq!(a.age_ms, Some(7 * DAY_MS));
    assert_eq!(a.work_time_ms, 7 * DAY_MS, "still in progress");
}

// -- classification ---------------------------------------------------

#[test]
fn the_default_classification_is_the_disclosed_one() {
    let c = default_classes();
    assert_eq!(classify(&c, "in_progress"), CATEGORY_VALUE_ADDING);
    assert_eq!(classify(&c, "in_review"), CATEGORY_NECESSARY);
    assert_eq!(classify(&c, BACKLOG_STATUS), CATEGORY_UNNECESSARY);
    assert_eq!(classify(&c, BLOCKED_STATUS), CATEGORY_UNNECESSARY);
    assert_eq!(
        classify(&c, "something_new"),
        CATEGORY_UNNECESSARY,
        "an unclassified status counts against you, so adding a board \
         column cannot silently improve the flow efficiency"
    );
    assert_eq!(waste_for(BACKLOG_STATUS), Some("inventory"));
    assert_eq!(waste_for(BLOCKED_STATUS), Some("waiting"));
    assert_eq!(waste_for("in_progress"), None);
}

#[test]
fn an_override_applies_whole_or_not_at_all() {
    let good = parse_classes(Some(r#"{"in_review":"value_adding"}"#)).expect("parsed");
    assert_eq!(classify(&good, "in_review"), CATEGORY_VALUE_ADDING);
    // An unknown category invalidates the whole override rather than
    // half-applying it — a figure matching no stated classification
    // is worse than one matching the documented default.
    assert_eq!(parse_classes(Some(r#"{"in_review":"lovely"}"#)), None);
    assert_eq!(parse_classes(Some("not json")), None);
    assert_eq!(parse_classes(Some("  ")), None);
    assert_eq!(parse_classes(Some("{}")), None);
    assert_eq!(parse_classes(None), None);
}

// -- rework (§6.5) ----------------------------------------------------

#[test]
fn backwards_moves_are_rework_but_blocking_is_not() {
    assert!(is_backwards("in_review", "in_progress"));
    assert!(is_backwards("done", "todo"));
    assert!(!is_backwards("todo", "in_progress"), "forwards");
    assert!(!is_backwards("in_progress", "in_progress"), "no move");
    assert!(
        !is_backwards("in_progress", BLOCKED_STATUS),
        "blocking is orthogonal to progress"
    );
    assert!(
        !is_backwards(BLOCKED_STATUS, "in_progress"),
        "and so is unblocking"
    );
    assert!(!is_backwards("mystery", "in_progress"), "unknown status");
}

#[test]
fn rework_and_first_pass_are_counted_per_item() {
    let clean = vec![
        tr(None, "todo", 0),
        tr(Some("todo"), "in_progress", 1),
        tr(Some("in_progress"), "done", 2),
    ];
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: Some(at(2)),
    };
    let a = analyze(&clean, clock, &classes(), at(2));
    assert_eq!(a.rework_count, 0);
    assert!(a.first_pass);

    let bounced = vec![
        tr(None, "todo", 0),
        tr(Some("todo"), "in_progress", 1),
        tr(Some("in_progress"), "in_review", 2),
        tr(Some("in_review"), "in_progress", 3),
        tr(Some("in_progress"), "in_review", 4),
        tr(Some("in_review"), "done", 5),
    ];
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: Some(at(5)),
    };
    let b = analyze(&bounced, clock, &classes(), at(5));
    assert_eq!(b.rework_count, 1);
    assert!(!b.first_pass);
}

// -- handoffs ---------------------------------------------------------

#[test]
fn handoffs_count_assignee_changes_not_people() {
    let mut transitions = vec![
        tr(None, "todo", 0),
        tr(Some("todo"), "in_progress", 1),
        tr(Some("in_progress"), "in_review", 2),
    ];
    transitions[1].assignee_ref = Some("worker:a".to_string());
    transitions[2].assignee_ref = Some("worker:b".to_string());
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: None,
    };
    let a = analyze(&transitions, clock, &classes(), at(3));
    assert_eq!(a.handoffs, 2, "none → a, a → b");
    assert_eq!(a.distinct_assignees, 2);
}

// -- degenerate input -------------------------------------------------

#[test]
fn degenerate_input_yields_stated_nulls_not_panics() {
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: None,
    };
    // No transitions at all: five calendar days of unknown history, charged
    // to `todo` rather than left unattributed, so it cannot be lost
    // from a report.
    let empty = analyze(&[], clock, &classes(), at(5));
    assert_eq!(empty.cycle_time_ms, None);
    assert_eq!(empty.transitions, 0);
    assert_eq!(empty.queue_time_ms, 5 * DAY_MS);
    assert_eq!(empty.lead_time_ms, 5 * DAY_MS);

    // `as_of` before the last transition (clock skew): a zero-length
    // final interval, never a negative one.
    let skewed = vec![tr(None, "todo", 0), tr(Some("todo"), "in_progress", 10)];
    let a = analyze(&skewed, clock, &classes(), at(3));
    assert!(a.by_status.iter().all(|s| s.ms >= 0));
    assert_eq!(a.lead_time_ms, 3 * DAY_MS);
    assert_eq!(
        a.cycle_time_ms, None,
        "as of day 3 the day-10 move has not happened, so the item has \
         not started — a future transition must not retroactively start \
         the clock"
    );

    // Out-of-order input is sorted, not trusted.
    let jumbled = vec![
        tr(Some("in_progress"), "done", 5),
        tr(None, "todo", 0),
        tr(Some("todo"), "in_progress", 2),
    ];
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: Some(at(5)),
    };
    let b = analyze(&jumbled, clock, &classes(), at(5));
    assert_eq!(b.cycle_time_ms, Some(3 * DAY_MS), "day 2 → 5");

    // Duplicate timestamps.
    let duplicated = vec![
        tr(None, "todo", 0),
        tr(Some("todo"), "in_progress", 1),
        tr(Some("in_progress"), "in_review", 1),
    ];
    let clock = TaskClock {
        created_ms: at(0),
        done_ms: None,
    };
    let c = analyze(&duplicated, clock, &classes(), at(4));
    let total: i64 = c.by_status.iter().map(|s| s.ms).sum();
    assert_eq!(total, c.lead_time_ms);
}

// -- percentiles and the SLE ------------------------------------------

#[test]
fn nearest_rank_percentiles_return_observed_values() {
    let sample = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(percentile(&sample, 0.5), Some(5));
    assert_eq!(percentile(&sample, 0.85), Some(9));
    assert_eq!(percentile(&sample, 1.0), Some(10));
    assert_eq!(percentile(&sample, 0.0), Some(1));
    assert_eq!(percentile(&[], 0.5), None);
    assert_eq!(percentile(&[7], 0.85), Some(7));
    assert_eq!(percentile(&sample, 1.5), None);
    assert_eq!(percentile(&sample, f64::NAN), None);
    for p in [0.0, 0.5, 0.85, 0.95, 1.0] {
        let v = percentile(&sample, p).expect("value");
        assert!(sample.contains(&v), "every percentile is an observation");
    }
}

#[test]
fn the_sle_refuses_to_forecast_from_noise() {
    let few: Vec<i64> = (1..=5).map(|d| d * DAY_MS).collect();
    let thin = service_level_expectation(&few, 0.85, None);
    assert_eq!(thin.within_ms, None);
    assert!(thin.reason.is_some(), "must say why, not return a number");
    assert_eq!(thin.sample, 5);

    let enough: Vec<i64> = (1..=20).map(|d| d * DAY_MS).collect();
    let sle = service_level_expectation(&enough, 0.85, None);
    assert_eq!(sle.within_ms, Some(17 * DAY_MS), "p85 of 1..=20");
    assert_eq!(sle.reason, None);
}

#[test]
fn the_sle_scores_an_explicit_commitment() {
    let cycles: Vec<i64> = (1..=20).map(|d| d * DAY_MS).collect();
    let met = service_level_expectation(&cycles, 0.85, Some(20.0));
    assert_eq!(met.target_achieved_ratio, Some(1.0));
    assert_eq!(met.target_met, Some(true));
    let missed = service_level_expectation(&cycles, 0.85, Some(5.0));
    assert_eq!(missed.target_achieved_ratio, Some(0.25));
    assert_eq!(missed.target_met, Some(false));
    let none = service_level_expectation(&cycles, 0.85, None);
    assert_eq!(none.target_met, None, "no commitment, no verdict");
}

#[test]
fn distribution_reports_the_shape_and_names_its_method() {
    let d = distribution(&[5, 1, 3, 2, 4]).expect("distribution");
    assert_eq!((d.n, d.min_ms, d.max_ms, d.p50_ms), (5, 1, 5, 3));
    assert_eq!(d.method, "nearest_rank");
    assert!(distribution(&[]).is_none());
}

// -- plan rollup ------------------------------------------------------

fn finished_task(queue_days: i64, work_days: i64, bounced: bool) -> TaskAnalysis {
    let mut transitions = vec![
        tr(None, "todo", 0),
        tr(Some("todo"), "in_progress", queue_days),
    ];
    let mut day = queue_days + work_days;
    if bounced {
        transitions.push(tr(Some("in_progress"), "in_review", day));
        transitions.push(tr(Some("in_review"), "in_progress", day + 1));
        day += 2;
    }
    transitions.push(tr(Some("in_progress"), "done", day));
    analyze(
        &transitions,
        TaskClock {
            created_ms: at(0),
            done_ms: Some(at(day)),
        },
        &classes(),
        at(day),
    )
}

#[test]
fn the_plan_rollup_reports_throughput_beside_first_pass_yield() {
    let analyses: Vec<TaskAnalysis> = (0..8).map(|i| finished_task(3, 2, i % 4 == 0)).collect();
    let p = plan(&analyses);
    assert_eq!(p.tasks, 8);
    assert_eq!(p.finished, 8);
    assert_eq!(p.rework_count, 2, "two of eight bounced");
    let rfpy = p.rolled_first_pass_yield.expect("rfpy");
    assert!((rfpy - 0.75).abs() < 1e-9, "got {rfpy}");
    assert!(p.cycle_time.is_some() && p.lead_time.is_some());
    // Lead time must always be reported beside cycle time, so the
    // flattering number cannot travel alone (§12.3).
    let cycle = p.cycle_time.as_ref().expect("cycle");
    let lead = p.lead_time.as_ref().expect("lead");
    assert!(
        lead.p50_ms > cycle.p50_ms,
        "the backlog dwell is in the lead time"
    );
}

/// One item that started, blocked for nearly a year, and finished:
/// a 300-calendar-day cycle with 2 calendar days of work in it.
fn long_stalled_task() -> TaskAnalysis {
    let transitions = vec![
        tr(None, "todo", 0),
        tr(Some("todo"), "in_progress", 1),
        tr(Some("in_progress"), BLOCKED_STATUS, 2),
        tr(Some(BLOCKED_STATUS), "in_progress", 300),
        tr(Some("in_progress"), "done", 301),
    ];
    analyze(
        &transitions,
        TaskClock {
            created_ms: at(0),
            done_ms: Some(at(301)),
        },
        &classes(),
        at(301),
    )
}

#[test]
fn the_plan_rollup_separates_the_typical_item_from_the_system_ratio() {
    // Nine quick items and one that sat for a year: the aggregate is
    // dragged down, the median is not, and the divergence is itself
    // the finding (§7.2).
    let mut analyses: Vec<TaskAnalysis> = (0..9).map(|_| finished_task(0, 5, false)).collect();
    analyses.push(long_stalled_task());
    let p = plan(&analyses);
    assert_eq!(p.waste_shape, "concentrated");
    let agg = p.aggregate_flow_efficiency.value.expect("aggregate");
    let med = p.median_flow_efficiency.expect("median");
    assert!(agg < med, "the long item drags the aggregate down");
}

#[test]
fn an_empty_plan_is_null_not_zero() {
    let p = plan(&[]);
    assert_eq!(p.tasks, 0);
    assert!(p.cycle_time.is_none());
    assert_eq!(p.aggregate_flow_efficiency.value, None);
    assert_eq!(p.rolled_first_pass_yield, None);
    assert_eq!(p.waste_shape, "insufficient_data");
    assert!(constraints(&[], &p).is_empty());
}

#[test]
fn constraints_rank_by_recoverable_time_and_name_their_rule() {
    let analyses = vec![finished_task(30, 2, true)];
    let p = plan(&analyses);
    let findings = constraints(&analyses, &p);
    assert!(!findings.is_empty());
    for pair in findings.windows(2) {
        assert!(pair[0].recoverable_ms >= pair[1].recoverable_ms);
    }
    assert!(findings.iter().any(|f| f.rule == "backlog_dwell"));
    assert!(findings.iter().any(|f| f.rule == "rework"));
    assert!(findings.iter().all(|f| !f.rule.is_empty()));
}

#[test]
fn aging_scores_an_open_item_against_the_expectation() {
    let past = aging(20 * DAY_MS, Some(11 * DAY_MS));
    assert!(past.past_sle);
    assert!(past.sle_ratio.unwrap_or(0.0) > 1.0);
    let within = aging(3 * DAY_MS, Some(11 * DAY_MS));
    assert!(!within.past_sle);
    let unknown = aging(3 * DAY_MS, None);
    assert_eq!(unknown.sle_ratio, None);
    assert!(!unknown.past_sle, "no expectation means no breach");
}

// -- flow -------------------------------------------------------------

#[test]
fn littles_law_labels_the_three_regimes() {
    // κ=100, μ=1/day ⇒ τ̂=100 calendar days, against 10 observed.
    let growing = flow(100, 200, 100, 100, Some(10 * DAY_MS));
    assert_eq!(growing.interpretation, "wip_growing");
    assert!((growing.implied_cycle_time_days.unwrap_or(0.0) - 100.0).abs() < 1e-9);

    let steady = flow(100, 100, 100, 10, Some(10 * DAY_MS));
    assert_eq!(steady.interpretation, "steady_state");

    let draining = flow(100, 100, 100, 1, Some(10 * DAY_MS));
    assert_eq!(draining.interpretation, "queue_draining");
}

#[test]
fn flow_refuses_to_invent_numbers_it_cannot_have() {
    let nothing_done = flow(30, 10, 0, 10, None);
    assert_eq!(nothing_done.utilization, None);
    assert!(nothing_done.utilization_reason.is_some());
    assert_eq!(nothing_done.implied_cycle_time_days, None);
    assert_eq!(nothing_done.interpretation, "insufficient_data");

    let zero_window = flow(0, 5, 5, 5, Some(DAY_MS));
    assert_eq!(zero_window.arrival_rate_per_day, None);
    assert_eq!(zero_window.utilization, None);
}

#[test]
fn utilization_is_demand_over_capacity() {
    let f = flow(10, 20, 10, 5, Some(DAY_MS));
    assert_eq!(f.arrival_rate_per_day, Some(2.0));
    assert_eq!(f.throughput_per_day, Some(1.0));
    assert_eq!(
        f.utilization,
        Some(2.0),
        "twice as much arriving as leaving"
    );
}

// -- cumulative flow --------------------------------------------------

#[test]
fn cumulative_flow_tracks_the_board_over_time() {
    // One task created day 0, started day 2, finished day 4.
    let history = TaskHistory {
        created_ms: at(0),
        transitions: vec![
            tr(None, "todo", 0),
            tr(Some("todo"), "in_progress", 2),
            tr(Some("in_progress"), "done", 4),
        ],
    };
    let samples = cumulative_flow(&[history], at(0), at(5), DAY_MS);
    assert_eq!(samples.len(), 6, "days 0..=5 inclusive");
    let count = |day: usize, status: &str| samples[day].counts[status];
    assert_eq!(count(0, "todo"), 1);
    assert_eq!(count(1, "todo"), 1);
    assert_eq!(count(2, "in_progress"), 1);
    assert_eq!(count(3, "in_progress"), 1);
    assert_eq!(count(4, "done"), 1);
    assert_eq!(count(5, "done"), 1, "it stays done");
    assert_eq!(samples[3].work_in_progress, 1);
    assert_eq!(samples[4].work_in_progress, 0);
    assert_eq!(samples[4].done, 1);
    // Every band is present at every sample, including at zero, so a
    // stacked chart never has to guess whether a gap means zero.
    for sample in &samples {
        for status in BOARD_ORDER.iter().chain(std::iter::once(&BLOCKED_STATUS)) {
            assert!(sample.counts.contains_key(*status), "missing {status}");
        }
        let banded: usize = sample.counts.values().sum();
        assert_eq!(banded, sample.total, "bands sum to the total");
    }
}

#[test]
fn a_task_does_not_appear_before_it_existed() {
    let history = TaskHistory {
        created_ms: at(3),
        transitions: vec![tr(None, "todo", 3)],
    };
    let samples = cumulative_flow(&[history], at(0), at(5), DAY_MS);
    assert_eq!(samples[0].total, 0, "not created yet");
    assert_eq!(samples[2].total, 0);
    assert_eq!(samples[3].total, 1, "created on day 3");
    assert_eq!(samples[5].total, 1);
}

#[test]
fn a_task_with_no_transitions_reads_as_backlog_not_as_absent() {
    // A backfilled board can hold a task whose only transition is
    // later than an early sample. It must not vanish from the chart
    // and reappear — it was somewhere, and `todo` is the
    // pessimistic guess.
    let history = TaskHistory {
        created_ms: at(0),
        transitions: vec![tr(None, "in_progress", 4)],
    };
    let samples = cumulative_flow(&[history], at(0), at(5), DAY_MS);
    assert_eq!(samples[0].counts[BACKLOG_STATUS], 1);
    assert_eq!(samples[3].counts[BACKLOG_STATUS], 1);
    assert_eq!(samples[4].counts["in_progress"], 1);
    assert!(samples.iter().all(|s| s.total == 1), "never disappears");
}

#[test]
fn cumulative_flow_is_bounded_and_ends_on_the_window() {
    // A step that does not divide the window still lands its last
    // sample exactly on `to_ms`, so the chart ends at "now".
    let history = TaskHistory {
        created_ms: at(0),
        transitions: vec![tr(None, "todo", 0)],
    };
    let samples = cumulative_flow(std::slice::from_ref(&history), at(0), at(10), 3 * DAY_MS);
    assert_eq!(samples.last().expect("last").at_ms, at(10));

    // Degenerate inputs yield nothing rather than looping forever.
    assert!(cumulative_flow(std::slice::from_ref(&history), at(0), at(5), 0).is_empty());
    assert!(cumulative_flow(std::slice::from_ref(&history), at(0), at(5), -1).is_empty());
    assert!(cumulative_flow(std::slice::from_ref(&history), at(5), at(0), DAY_MS).is_empty());
    assert_eq!(cumulative_flow(&[], at(0), at(5), DAY_MS).len(), 6);

    // A very fine step over a wide window is capped, not unbounded.
    let capped = cumulative_flow(&[history], at(0), at(100_000), 1);
    assert_eq!(capped.len(), MAX_FLOW_SAMPLES);
}

#[test]
fn status_at_answers_the_three_cases() {
    let history = TaskHistory {
        created_ms: at(1),
        transitions: vec![tr(None, "todo", 1), tr(Some("todo"), "in_progress", 3)],
    };
    assert_eq!(status_at(&history, at(0)), None, "before creation");
    assert_eq!(status_at(&history, at(1)), Some("todo"));
    assert_eq!(status_at(&history, at(2)), Some("todo"), "between moves");
    assert_eq!(
        status_at(&history, at(3)),
        Some("in_progress"),
        "on the move"
    );
    assert_eq!(status_at(&history, at(9)), Some("in_progress"), "after");
}

// -- cross-plan rollup (§15 TBA-9) -------------------------------------

/// A uuid from a small integer, so tree tests read legibly.
fn node(n: u8) -> uuid::Uuid {
    uuid::Uuid::from_bytes([n; 16])
}

/// Build a parent → children map from `(parent, child)` pairs.
fn tree(pairs: &[(u8, u8)]) -> BTreeMap<uuid::Uuid, Vec<uuid::Uuid>> {
    let mut map: BTreeMap<uuid::Uuid, Vec<uuid::Uuid>> = BTreeMap::new();
    for (parent, child) in pairs {
        map.entry(node(*parent)).or_default().push(node(*child));
    }
    map
}

#[test]
fn the_walk_covers_the_tree_breadth_first() {
    // 1 ─┬─ 2 ─── 4
    //    └─ 3
    let children = tree(&[(1, 2), (1, 3), (2, 4)]);
    let walk = walk_descendants(&children, node(1), 500, 32);
    assert_eq!(
        walk.nodes,
        vec![
            RollupNode {
                pid: node(1),
                depth: 0
            },
            RollupNode {
                pid: node(2),
                depth: 1
            },
            RollupNode {
                pid: node(3),
                depth: 1
            },
            RollupNode {
                pid: node(4),
                depth: 2
            },
        ]
    );
    assert!(!walk.truncated);
    assert_eq!(walk.revisits, 0);
}

#[test]
fn a_leaf_rolls_up_to_itself() {
    let walk = walk_descendants(&BTreeMap::new(), node(1), 500, 32);
    assert_eq!(
        walk.nodes,
        vec![RollupNode {
            pid: node(1),
            depth: 0
        }]
    );
    assert!(!walk.truncated);
}

#[test]
fn a_cycle_terminates_and_is_reported() {
    // 1 → 2 → 3 → 1. The write path refuses this, but a rollup that
    // trusts that is one direct UPDATE away from hanging the
    // service — and it must say what it found rather than absorbing
    // it silently.
    let children = tree(&[(1, 2), (2, 3), (3, 1)]);
    let walk = walk_descendants(&children, node(1), 500, 32);
    assert_eq!(walk.nodes.len(), 3, "each plan is visited once");
    assert_eq!(walk.revisits, 1, "the back-edge is counted");
    assert!(!walk.truncated, "a cycle is not a cap firing");
}

#[test]
fn a_diamond_counts_the_second_path_as_a_revisit() {
    // 1 ─┬─ 2 ─┐
    //    └─ 3 ─┴─ 4   — 4 is reached twice, counted once.
    let children = tree(&[(1, 2), (1, 3), (2, 4), (3, 4)]);
    let walk = walk_descendants(&children, node(1), 500, 32);
    assert_eq!(walk.nodes.len(), 4, "no double counting");
    assert_eq!(walk.revisits, 1);
}

#[test]
fn the_node_cap_fires_and_says_so() {
    let children = tree(&[(1, 2), (1, 3), (1, 4), (1, 5)]);
    let walk = walk_descendants(&children, node(1), 3, 32);
    assert_eq!(walk.nodes.len(), 3);
    assert!(
        walk.truncated,
        "a cap that fires silently reads as full coverage"
    );
}

#[test]
fn the_depth_cap_fires_only_when_there_was_more_to_walk() {
    // Depth 1 with grandchildren present ⇒ truncated.
    let deep = tree(&[(1, 2), (2, 3)]);
    let walk = walk_descendants(&deep, node(1), 500, 1);
    assert_eq!(walk.nodes.len(), 2);
    assert!(walk.truncated);

    // Depth 1 with nothing below the children ⇒ complete.
    let shallow = tree(&[(1, 2), (1, 3)]);
    let walk = walk_descendants(&shallow, node(1), 500, 1);
    assert_eq!(walk.nodes.len(), 3);
    assert!(!walk.truncated, "the tree ran out, no cap fired");
}

#[test]
fn a_deep_chain_stops_at_the_documented_depth() {
    let pairs: Vec<(u8, u8)> = (1..60).map(|i| (i, i + 1)).collect();
    let walk = walk_descendants(&tree(&pairs), node(1), MAX_ROLLUP_NODES, MAX_ROLLUP_DEPTH);
    assert_eq!(
        walk.nodes.len(),
        MAX_ROLLUP_DEPTH + 1,
        "root plus the depth"
    );
    assert!(walk.truncated);
    assert!(
        walk.nodes.iter().all(|n| n.depth <= MAX_ROLLUP_DEPTH),
        "no node past the cap"
    );
}

#[test]
fn a_self_parent_does_not_loop() {
    let walk = walk_descendants(&tree(&[(1, 1)]), node(1), 500, 32);
    assert_eq!(walk.nodes.len(), 1);
    assert_eq!(walk.revisits, 1);
}

// -- Rollup attrition record (§15 TBA-12, repo tasks.md PA-3) ----------

/// Defaults matching a small, uncapped, untruncated rollup; tests
/// override only the fields they care about.
fn rollup_inputs() -> RollupAttritionInputs {
    RollupAttritionInputs {
        root_note: "plan root".to_string(),
        walked_plans: 5,
        walk_truncated: false,
        walk_revisits: 0,
        tasks_scanned: 20,
        tasks_capped_plans: 0,
        finished: 12,
        work_in_progress: 3,
        not_started: 5,
    }
}

#[test]
fn rollup_attrition_trail_produces_the_documented_labels_in_order() {
    let trail = rollup_attrition_trail(&rollup_inputs());
    let labels: Vec<&str> = trail.iter().map(|s| s.label).collect();
    assert_eq!(
        labels,
        [
            "root_plan",
            "walked_plans",
            "tasks_scanned",
            "finished",
            "work_in_progress",
            "not_started",
        ]
    );
}

#[test]
fn rollup_attrition_trail_chains_parents_correctly() {
    let trail = rollup_attrition_trail(&rollup_inputs());
    assert_eq!(trail[0].parent, None, "root has no parent");
    assert_eq!(trail[1].parent, Some(0), "walked_plans narrows root_plan");
    assert_eq!(
        trail[2].parent,
        Some(1),
        "tasks_scanned narrows walked_plans"
    );
    for leaf in &trail[3..] {
        assert_eq!(
            leaf.parent,
            Some(ROLLUP_ATTRITION_PARTITION_PARENT),
            "every partition leaf forks from tasks_scanned"
        );
    }
}

#[test]
fn rollup_attrition_trail_partition_sums_to_tasks_scanned() {
    let trail = rollup_attrition_trail(&rollup_inputs());
    let tasks_scanned = trail[2].instances;
    let partition_sum: usize = trail[3..].iter().map(|s| s.instances).sum();
    assert_eq!(partition_sum, tasks_scanned);
}

#[test]
fn rollup_attrition_trail_names_a_truncated_walk() {
    let mut truncated_inputs = rollup_inputs();
    truncated_inputs.walked_plans = 500;
    truncated_inputs.walk_truncated = true;
    truncated_inputs.walk_revisits = 2;
    let truncated = rollup_attrition_trail(&truncated_inputs);
    assert!(truncated[1].operation.contains("cap stopped the walk"));
    assert!(
        truncated[1].operation.contains('2'),
        "reports the revisit count"
    );

    let complete = rollup_attrition_trail(&rollup_inputs());
    assert!(complete[1].operation.contains("covered the whole subtree"));
}

#[test]
fn rollup_attrition_trail_names_task_capped_plans() {
    let mut capped_inputs = rollup_inputs();
    capped_inputs.tasks_scanned = 1000;
    capped_inputs.tasks_capped_plans = 2;
    let capped = rollup_attrition_trail(&capped_inputs);
    assert!(capped[2].operation.contains("2 plan(s) reached"));

    let uncapped = rollup_attrition_trail(&rollup_inputs());
    assert!(uncapped[2].operation.contains("none at the per-plan cap"));
}

#[test]
fn rollup_attrition_step_labels_matches_the_linear_chain() {
    let trail = rollup_attrition_trail(&rollup_inputs());
    for (index, expected) in ROLLUP_ATTRITION_STEP_LABELS.iter().enumerate() {
        assert_eq!(trail[index].label, *expected);
    }
}

// -- Monte-Carlo forecasting (§15 TBA-11) ------------------------------

#[test]
fn an_event_stamped_at_now_is_inside_the_history_window() {
    let week = 7 * DAY_MS;
    let now = at(100);
    // The last of three completions is stamped in the same millisecond as "now".
    let completed = vec![now - 3 * DAY_MS, now - DAY_MS, now];
    let (from, to) = history_window(now, 6, week);
    let history = throughput_history(&completed, from, to, week);
    assert_eq!(history.len(), 6, "the number of periods is unchanged");
    assert_eq!(
        history.iter().sum::<usize>(),
        3,
        "nothing recorded up to now is dropped"
    );
    assert_eq!(history[5], 3);
    // A window that simply ended at `now` would have dropped it.
    let naive = throughput_history(&completed, now - 6 * week, now, week);
    assert_eq!(naive.iter().sum::<usize>(), 2);
    // An event after now is still outside.
    let later = throughput_history(&[now + 5], from, to, week);
    assert_eq!(later.iter().sum::<usize>(), 0);
}

#[test]
fn throughput_history_buckets_completions_into_whole_periods() {
    let week = 7 * DAY_MS;
    // Three finished in week 0, one in week 1, none in week 2.
    let completed = vec![at(0), at(1), at(2), at(8)];
    let history = throughput_history(&completed, at(0), at(21), week);
    assert_eq!(history, vec![3, 1, 0]);
}

#[test]
fn throughput_history_ignores_a_partial_trailing_period() {
    let week = 7 * DAY_MS;
    // Ten calendar days of window is one whole week plus a stub. Counting the
    // stub as a period would understate throughput.
    let history = throughput_history(&[at(1), at(8)], at(0), at(10), week);
    assert_eq!(history, vec![1], "only whole periods count");
}

#[test]
fn throughput_history_drops_anything_outside_the_window() {
    let week = 7 * DAY_MS;
    let history = throughput_history(&[at(-5), at(3), at(99)], at(0), at(14), week);
    assert_eq!(history, vec![1, 0]);
    assert!(throughput_history(&[at(1)], at(0), at(14), 0).is_empty());
    assert!(throughput_history(&[at(1)], at(14), at(0), week).is_empty());
}

#[test]
fn a_batch_forecast_is_deterministic() {
    // A forecast that changes every time you reload it is not one
    // anybody will act on.
    let history = vec![3, 5, 2, 4, 6, 3, 5, 4];
    let first = forecast_batch(&history, 20, 2000, 7, 42);
    let repeat = forecast_batch(&history, 20, 2000, 7, 42);
    assert_eq!(first.p85_days, repeat.p85_days);
    assert_eq!(first.p50_days, repeat.p50_days);
    // A different seed may differ, but must stay in the same region.
    let reseeded = forecast_batch(&history, 20, 2000, 7, 7);
    let (left, right) = (
        first.p85_days.expect("seeded"),
        reseeded.p85_days.expect("reseeded"),
    );
    assert!((left - right).abs() <= 14.0, "{left} vs {right}");
}

#[test]
fn a_batch_forecast_is_ordered_and_scales_with_the_batch() {
    let history = vec![3, 5, 2, 4, 6, 3, 5, 4];
    let f = forecast_batch(&history, 40, 4000, 7, 1);
    let (p50, p85, p95) = (
        f.p50_days.expect("p50"),
        f.p85_days.expect("p85"),
        f.p95_days.expect("p95"),
    );
    assert!(p50 <= p85 && p85 <= p95, "{p50} {p85} {p95}");
    // Roughly 4 items a week, so 40 items is about ten weeks.
    assert!((49.0..=105.0).contains(&p50), "p50 was {p50} calendar days");
    // A bigger batch cannot finish sooner.
    let bigger = forecast_batch(&history, 80, 4000, 7, 1);
    assert!(bigger.p85_days.expect("p85") >= p85);
    assert_eq!(f.trials_hit_ceiling, 0);
}

#[test]
fn a_batch_forecast_refuses_rather_than_guessing() {
    // Too little history.
    let thin = forecast_batch(&[3, 4, 5], 10, 1000, 7, 1);
    assert_eq!(thin.p85_days, None);
    assert!(thin.reason.is_some_and(|r| r.contains("noise")));

    // A history of zeroes: the honest answer is `never`, and the
    // per-trial ceiling turns what would be an infinite loop into a
    // stated refusal.
    let dead = forecast_batch(&[0; 8], 10, 1000, 7, 1);
    assert_eq!(dead.p85_days, None);
    assert!(dead.reason.is_some_and(|r| r.contains("never")));

    // Zero items is zero calendar days, not a refusal.
    let none = forecast_batch(&[3, 4, 5, 2, 3, 4], 0, 100, 7, 1);
    assert_eq!(none.p85_days, Some(0.0));
    assert_eq!(none.reason, None);
}

#[test]
fn a_mostly_dead_history_reports_the_trials_that_never_finished() {
    // One good week in eight. Some trials will not cover the batch
    // inside the ceiling, and the percentiles are then floors — so
    // the count is reported rather than silently rolled in.
    let history = vec![0, 0, 0, 0, 0, 0, 0, 1];
    let f = forecast_batch(&history, 5000, 200, 7, 1);
    assert!(
        f.trials_hit_ceiling > 0,
        "the ceiling must be visible, not silent"
    );
}

#[test]
fn an_items_forecast_puts_the_conservative_figure_at_the_low_end() {
    // The direction reverses from `forecast_batch`: 85% confidence
    // of at-least-N is the 15th percentile. Quoting the 85th here
    // would promise the best case while sounding careful.
    let history = vec![1, 3, 5, 2, 8, 4, 6, 2];
    let f = forecast_items(&history, 4, 4000, 7, 3);
    let least = f.at_least_items.expect("at_least");
    let median = f.median_items.expect("median");
    let most = f.at_most_items.expect("at_most");
    assert!(least <= median && median <= most, "{least} {median} {most}");
    assert!(f.note.contains("15th percentile"));
    assert_eq!(f.days, 28, "four weeks");
}

#[test]
fn an_items_forecast_refuses_rather_than_guessing() {
    let thin = forecast_items(&[3, 4, 5], 4, 1000, 7, 1);
    assert_eq!(thin.median_items, None);
    assert!(thin.reason.is_some_and(|r| r.contains("noise")));

    let history = vec![1, 3, 5, 2, 8, 4, 6, 2];
    assert!(forecast_items(&history, 0, 100, 7, 1).reason.is_some());
    assert!(
        forecast_items(&history, MAX_PERIODS_PER_TRIAL + 1, 100, 7, 1)
            .reason
            .is_some()
    );
}

#[test]
fn trials_are_clamped_rather_than_trusted() {
    let history = vec![3, 4, 5, 2, 3, 4];
    assert_eq!(forecast_batch(&history, 5, 0, 7, 1).trials, 1);
    assert_eq!(
        forecast_batch(&history, 5, usize::MAX, 7, 1).trials,
        MAX_TRIALS
    );
    assert_eq!(forecast_items(&history, 2, 0, 7, 1).trials, 1);
}

#[test]
fn a_zero_seed_still_produces_variety() {
    // xorshift emits only zeroes from a zero state, which would
    // make every trial identical and every percentile the same.
    let history = vec![1, 9, 2, 8, 3, 7];
    let f = forecast_items(&history, 5, 2000, 7, 0);
    assert!(
        f.at_least_items.expect("least") < f.at_most_items.expect("most"),
        "a zero seed must not collapse the distribution"
    );
}

// -- flow gauges (§15 TBA-10) -----------------------------------------

/// A plan whose board holds `n` finished tasks.
fn sample_of(pid: &str, n: usize) -> PlanFlowSample {
    let analyses: Vec<TaskAnalysis> = (0..n).map(|_| finished_task(1, 2, false)).collect();
    let cycles: Vec<i64> = analyses.iter().filter_map(|a| a.cycle_time_ms).collect();
    PlanFlowSample {
        plan_pid: pid.to_string(),
        analysis: plan(&analyses),
        sle: service_level_expectation(&cycles, 0.85, None),
        columns_over_limit: 0,
    }
}

#[test]
fn small_boards_are_suppressed_not_exported() {
    // A flow efficiency over two tasks describes two people's week,
    // and §12.4 refuses per-person measurement; a public
    // `/metrics.prom` label that reaches it by arithmetic is the
    // same thing through a side door.
    let samples = vec![
        sample_of("big", 20),
        sample_of("tiny", 2),
        sample_of("empty", 0),
    ];
    let set = flow_metric_rows(&samples, 50, 5);
    assert_eq!(set.rows.len(), 1);
    assert_eq!(set.rows[0].plan_pid, "big");
    assert_eq!(set.suppressed_plans, 1);
    assert!(!set.rows.iter().any(|row| row.plan_pid == "tiny"));
    // An empty board is neither exported nor counted as suppressed:
    // there is nothing to withhold.
    assert!(!set.rows.iter().any(|row| row.plan_pid == "empty"));
}

#[test]
fn the_series_cap_keeps_the_largest_boards_and_says_what_it_dropped() {
    let samples: Vec<PlanFlowSample> = (1..=8)
        .map(|i| sample_of(&format!("p{i:02}"), i * 5))
        .collect();
    let set = flow_metric_rows(&samples, 3, 5);
    assert_eq!(set.rows.len(), 3);
    assert_eq!(
        set.rows.iter().map(|r| r.tasks).collect::<Vec<_>>(),
        vec![40, 35, 30],
        "largest board first"
    );
    assert_eq!(set.dropped_plans, 5, "the cap is never silent");
}

#[test]
fn the_p85_gauge_inherits_the_sle_refusal_to_forecast_from_noise() {
    // Eight finished tasks is below MIN_SLE_SAMPLE, so the
    // expectation is null — and the gauge must stay null rather
    // than re-deciding the question with a number from noise.
    let thin = sample_of("thin", 8);
    assert!(thin.sle.within_days.is_none(), "precondition");
    let set = flow_metric_rows(std::slice::from_ref(&thin), 50, 5);
    assert_eq!(set.rows[0].cycle_time_p85_days, None);

    let thick = sample_of("thick", 20);
    let set = flow_metric_rows(std::slice::from_ref(&thick), 50, 5);
    assert!(set.rows[0].cycle_time_p85_days.is_some());
}

#[test]
fn selection_is_stable_across_passes() {
    // A dashboard must not reshuffle between scrapes.
    let samples: Vec<PlanFlowSample> = ["c", "a", "b"]
        .iter()
        .map(|pid| sample_of(pid, 9))
        .collect();
    let first = flow_metric_rows(&samples, 2, 5);
    let mut shuffled = samples.clone();
    shuffled.reverse();
    let second = flow_metric_rows(&shuffled, 2, 5);
    assert_eq!(first.rows, second.rows);
    assert_eq!(
        first
            .rows
            .iter()
            .map(|r| r.plan_pid.as_str())
            .collect::<Vec<_>>(),
        vec!["a", "b"]
    );
}

#[test]
fn an_empty_estate_exports_nothing_rather_than_zeroes() {
    let set = flow_metric_rows(&[], 50, 5);
    assert!(set.rows.is_empty());
    assert_eq!(set.suppressed_plans, 0);
    assert_eq!(set.dropped_plans, 0);
}

// -- property-style sweep ---------------------------------------------

#[test]
fn invariants_hold_over_a_generated_sweep() {
    let mut state: u64 = 0x2545_F491_4F6C_DD1D;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let statuses = ["todo", "in_progress", "in_review", "blocked", "done"];
    for _ in 0..500 {
        let count = usize::try_from(next() % 15).unwrap_or(0);
        let mut day = 0i64;
        let mut previous: Option<String> = None;
        let mut transitions = Vec::new();
        let mut done_day: Option<i64> = None;
        for _ in 0..count {
            day += i64::try_from(next() % 5).unwrap_or(0);
            let to = statuses[usize::try_from(next() % 5).unwrap_or(0)];
            transitions.push(tr(previous.as_deref(), to, day));
            previous = Some(to.to_string());
            if to == FINISHED_STATUS && done_day.is_none() {
                done_day = Some(day);
            }
        }
        let as_of = at(day + i64::try_from(next() % 10).unwrap_or(0));
        let clock = TaskClock {
            created_ms: at(0),
            done_ms: done_day.map(at),
        };
        let a = analyze(&transitions, clock, &classes(), as_of);

        let cycle = a.cycle_time_ms.unwrap_or(0);
        let status_total: i64 = a.by_status.iter().map(|s| s.ms).sum();
        let category_total: i64 = a.by_category.iter().map(|c| c.ms).sum();
        assert_eq!(status_total, a.lead_time_ms, "statuses partition lead time");
        assert_eq!(
            category_total, a.lead_time_ms,
            "categories partition it too"
        );
        assert!(a.work_time_ms <= a.process_time_ms);
        assert!(
            a.process_time_ms <= cycle,
            "process {} > cycle {cycle}",
            a.process_time_ms
        );
        assert!(a.lead_time_ms >= 0);
        assert!(a.by_status.iter().all(|s| s.ms >= 0));
        if let Some(v) = a.flow_efficiency.value {
            assert!((0.0..=1.0).contains(&v), "efficiency out of range: {v}");
        }
        // A finished item's cycle time never exceeds its lead time.
        if a.finished {
            assert!(
                cycle <= a.lead_time_ms,
                "cycle {cycle} > lead {}",
                a.lead_time_ms
            );
        }
    }
}
