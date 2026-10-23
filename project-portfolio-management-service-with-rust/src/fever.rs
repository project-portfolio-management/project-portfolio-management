//! The buffer fever chart (DP-7). Pure rules; the database work is in [`crate::controllers::fever`].
//!
//! A programme's **project buffer** is a number of business days. On the chart, *x* is how much of the
//! baselined critical work is done (chain completed %) and *y* is how much of the buffer the forecast
//! slip has used (buffer consumed %). Two sloped lines divide the chart into green, amber and red. The
//! default lines are a convention of this project, not a published standard; they are configurable and
//! always printed with the figures.
//!
//! A point on the lower line is **green** and a point on the upper line is **amber**: the boundary
//! belongs to the kinder zone, and the unit tests below pin it. A figure below 0 % (recovery) or above
//! 100 % (overrun) is plotted at the edge with its true number alongside.

#![expect(missing_docs)]

use serde::Serialize;

const EPS: f64 = 1e-9;

/// A sloped line, given by its height at 0 % and at 100 % chain completed.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Line {
    pub at_start: f64,
    pub at_end: f64,
}

impl Line {
    /// The line's height (buffer consumed %) at a chain completed %.
    #[must_use]
    pub fn at(&self, x_pct: f64) -> f64 {
        self.at_start + (self.at_end - self.at_start) * (x_pct / 100.0)
    }
}

/// The two lines: below `green_amber` is green, up to `amber_red` is amber, above is red.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Lines {
    pub green_amber: Line,
    pub amber_red: Line,
}

/// The default lines.
pub const DEFAULT_LINES: Lines = Lines {
    green_amber: Line {
        at_start: 30.0,
        at_end: 70.0,
    },
    amber_red: Line {
        at_start: 60.0,
        at_end: 100.0,
    },
};

/// The lines in force: `PROJECT_PORTFOLIO_MANAGEMENT_FEVER_LINES` as JSON
/// `{"green_amber":{"at_start":30,"at_end":70},"amber_red":{"at_start":60,"at_end":100}}`, else the
/// default. The second flag is true when a value was set but unusable (so the pack can say so).
#[must_use]
pub fn configured_lines() -> (Lines, bool) {
    let Ok(raw) = std::env::var("PROJECT_PORTFOLIO_MANAGEMENT_FEVER_LINES") else {
        return (DEFAULT_LINES, false);
    };
    match parse_lines(&raw) {
        Some(l) => (l, false),
        None => (DEFAULT_LINES, true),
    }
}

/// Parse and check a lines setting: finite, 0 to 200, and the upper line never below the lower.
#[must_use]
pub fn parse_lines(raw: &str) -> Option<Lines> {
    #[derive(serde::Deserialize)]
    struct L {
        at_start: f64,
        at_end: f64,
    }
    #[derive(serde::Deserialize)]
    struct Both {
        green_amber: L,
        amber_red: L,
    }
    let b: Both = serde_json::from_str(raw).ok()?;
    let ok = |v: f64| v.is_finite() && (0.0..=200.0).contains(&v);
    let all = [
        b.green_amber.at_start,
        b.green_amber.at_end,
        b.amber_red.at_start,
        b.amber_red.at_end,
    ];
    if !all.iter().all(|v| ok(*v)) {
        return None;
    }
    if b.amber_red.at_start < b.green_amber.at_start || b.amber_red.at_end < b.green_amber.at_end {
        return None;
    }
    Some(Lines {
        green_amber: Line {
            at_start: b.green_amber.at_start,
            at_end: b.green_amber.at_end,
        },
        amber_red: Line {
            at_start: b.amber_red.at_start,
            at_end: b.amber_red.at_end,
        },
    })
}

/// The zone of a point.
#[must_use]
pub fn zone(x_pct: f64, y_pct: f64, lines: &Lines) -> &'static str {
    if y_pct <= lines.green_amber.at(x_pct) + EPS {
        "green"
    } else if y_pct <= lines.amber_red.at(x_pct) + EPS {
        "amber"
    } else {
        "red"
    }
}

/// Why there is no point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Absent {
    NoBuffer,
    NoBaseline,
    NoChain,
    NoForecast,
}

impl Absent {
    /// The reason in words.
    #[must_use]
    pub fn reason(self) -> &'static str {
        match self {
            Self::NoBuffer => "no project buffer is declared for this programme",
            Self::NoBaseline => "the programme has no schedule baseline",
            Self::NoChain => {
                "the baseline has no critical work with a duration to measure completion against"
            }
            Self::NoForecast => {
                "the programme's schedule cannot be forecast now (no scheduled finish, or its dependencies form a cycle)"
            }
        }
    }
}

/// One plotted point, with the true figures and where it was drawn.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Point {
    /// Chain completed %, true figure.
    pub chain_completed_pct: f64,
    /// Buffer consumed %, true figure: below 0 is recovery, above 100 is overrun.
    pub buffer_consumed_pct: f64,
    pub plotted_x_pct: f64,
    pub plotted_y_pct: f64,
    /// `below` or `above` when the true figure was drawn at an edge.
    pub edge: Option<&'static str>,
    pub zone: &'static str,
    pub slip_days: i64,
    pub buffer_days: i64,
}

/// Build the point. `done` and `total` are the baselined critical work, in the same unit (days);
/// `slip_days` is the forecast finish less the baselined finish in business days (negative is early).
///
/// # Errors
///
/// [`Absent`] when there is no buffer or no critical work to measure against.
#[expect(clippy::cast_precision_loss)] // day counts are tiny; f64 is exact far beyond them
pub fn point(
    done: i64,
    total: i64,
    slip_days: i64,
    buffer_days: i64,
    lines: &Lines,
) -> Result<Point, Absent> {
    if buffer_days < 1 {
        return Err(Absent::NoBuffer);
    }
    if total < 1 {
        return Err(Absent::NoChain);
    }
    let x = (done.clamp(0, total) as f64 * 100.0) / total as f64;
    let y = (slip_days as f64 * 100.0) / buffer_days as f64;
    let py = y.clamp(0.0, 100.0);
    let edge = if y < 0.0 {
        Some("below")
    } else if y > 100.0 {
        Some("above")
    } else {
        None
    };
    Ok(Point {
        chain_completed_pct: x,
        buffer_consumed_pct: y,
        plotted_x_pct: x,
        plotted_y_pct: py,
        edge,
        // The zone is that of the true figure, so an overrun is red and a recovery is green.
        zone: zone(x, y, lines),
        slip_days,
        buffer_days,
    })
}

/// The buffer a stated rule gives: half the baselined critical work, rounded up, at least one day.
#[must_use]
pub fn half_of_critical(total_days: i64) -> i64 {
    ((total_days + 1) / 2).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const L: Lines = DEFAULT_LINES;

    #[test]
    fn a_point_on_the_lower_line_is_green_and_just_above_is_amber() {
        // Default lower line: 30 at x=0, 70 at x=100 → 50 at x=50.
        assert_eq!(zone(0.0, 30.0, &L), "green");
        assert_eq!(zone(0.0, 30.000_001, &L), "amber");
        assert_eq!(zone(50.0, 50.0, &L), "green");
        assert_eq!(zone(50.0, 50.001, &L), "amber");
        assert_eq!(zone(100.0, 70.0, &L), "green");
    }

    #[test]
    fn a_point_on_the_upper_line_is_amber_and_just_above_is_red() {
        assert_eq!(zone(0.0, 60.0, &L), "amber");
        assert_eq!(zone(0.0, 60.001, &L), "red");
        assert_eq!(zone(50.0, 80.0, &L), "amber");
        assert_eq!(zone(50.0, 80.001, &L), "red");
        assert_eq!(zone(100.0, 100.0, &L), "amber");
        assert_eq!(zone(100.0, 100.001, &L), "red");
    }

    #[test]
    fn the_point_is_built_from_days() {
        // 6 of 12 days of critical work done; 3 of 10 buffer days used → (50, 30): green.
        let p = point(6, 12, 3, 10, &L).unwrap();
        assert_eq!(
            (p.chain_completed_pct, p.buffer_consumed_pct, p.zone, p.edge),
            (50.0, 30.0, "green", None)
        );
        // 6 of 12 done, 8 of 10 used → exactly on the upper line at x=50: amber.
        let p = point(6, 12, 8, 10, &L).unwrap();
        assert_eq!((p.buffer_consumed_pct, p.zone), (80.0, "amber"));
        // 9 of 10 used → red.
        assert_eq!(point(6, 12, 9, 10, &L).unwrap().zone, "red");
    }

    #[test]
    fn recovery_and_overrun_are_drawn_at_the_edge_with_the_true_number() {
        let rec = point(6, 12, -4, 10, &L).unwrap();
        assert_eq!(
            (
                rec.buffer_consumed_pct,
                rec.plotted_y_pct,
                rec.edge,
                rec.zone
            ),
            (-40.0, 0.0, Some("below"), "green")
        );
        let over = point(6, 12, 25, 10, &L).unwrap();
        assert_eq!(
            (
                over.buffer_consumed_pct,
                over.plotted_y_pct,
                over.edge,
                over.zone
            ),
            (250.0, 100.0, Some("above"), "red")
        );
        // Exactly 0 % and exactly 100 % are on the chart, not past it.
        assert_eq!(point(6, 12, 0, 10, &L).unwrap().edge, None);
        assert_eq!(point(6, 12, 10, 10, &L).unwrap().edge, None);
    }

    #[test]
    fn nothing_to_measure_is_absent_with_a_reason_not_zero() {
        assert_eq!(point(0, 12, 3, 0, &L), Err(Absent::NoBuffer));
        assert_eq!(point(0, 0, 3, 10, &L), Err(Absent::NoChain));
        assert!(Absent::NoBaseline.reason().contains("baseline"));
        assert!(Absent::NoForecast.reason().contains("forecast"));
    }

    #[test]
    fn done_work_beyond_the_total_does_not_pass_100() {
        assert_eq!(point(20, 12, 0, 10, &L).unwrap().chain_completed_pct, 100.0);
        assert_eq!(point(-5, 12, 0, 10, &L).unwrap().chain_completed_pct, 0.0);
    }

    #[test]
    fn the_stated_rule_is_half_the_critical_work_rounded_up() {
        assert_eq!(half_of_critical(12), 6);
        assert_eq!(half_of_critical(13), 7);
        assert_eq!(half_of_critical(1), 1);
        assert_eq!(half_of_critical(0), 1);
    }

    #[test]
    fn configured_lines_are_checked() {
        let ok = r#"{"green_amber":{"at_start":20,"at_end":60},"amber_red":{"at_start":50,"at_end":90}}"#;
        assert_eq!(parse_lines(ok).unwrap().green_amber.at(100.0), 60.0);
        // The upper line below the lower, a number out of range, and text that is not JSON are all refused.
        assert!(parse_lines(r#"{"green_amber":{"at_start":50,"at_end":60},"amber_red":{"at_start":40,"at_end":90}}"#).is_none());
        assert!(parse_lines(r#"{"green_amber":{"at_start":-1,"at_end":60},"amber_red":{"at_start":50,"at_end":90}}"#).is_none());
        assert!(parse_lines("30,0.4").is_none());
    }
}
