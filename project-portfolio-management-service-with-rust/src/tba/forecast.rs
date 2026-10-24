//! Monte-Carlo delivery forecasting (spec §15 TBA-11).

use super::{Serialize, percentile};

/// Calendar days in a forecasting period. A week is the conventional unit:
/// shorter buckets are dominated by weekday effects, longer ones give
/// too few samples to draw from.
pub const DEFAULT_PERIOD_DAYS: i64 = 7;

/// Periods of history below which a forecast is refused.
///
/// Six weeks is not a principled number, but forecasting from two is
/// arithmetic on noise, and a confident-looking figure derived from
/// nothing is what discredits the whole method.
pub const MIN_THROUGHPUT_PERIODS: usize = 6;

/// Simulation trials, and the ceiling a caller may ask for.
pub const DEFAULT_TRIALS: usize = 10_000;

/// The most trials a caller may request.
pub const MAX_TRIALS: usize = 100_000;

/// Per-trial period ceiling. A history that contains only zeroes would
/// otherwise accumulate forever; this turns a hang into a stated
/// refusal.
pub const MAX_PERIODS_PER_TRIAL: usize = 520;

/// A deterministic xorshift64\* generator.
///
/// Determinism is a **feature**, not a testing convenience: a forecast
/// that changes every time you reload it is not one anybody will trust
/// or act on. The same history and the same question give the same
/// answer, and the seed is an input so a caller can vary it
/// deliberately.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        // A zero seed would make xorshift emit only zeroes.
        Self(if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        })
    }

    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    /// A uniform index into a slice of length `len` (`len > 0`).
    fn index(&mut self, len: usize) -> usize {
        #[expect(clippy::cast_possible_truncation)] // modulo of a bounded len
        {
            (self.next() % len as u64) as usize
        }
    }
}

/// The window of whole periods ending **at** `now_ms`, as `(from_ms, to_ms)`.
///
/// [`throughput_history`] buckets over the half-open interval `[from, to)`, so
/// an event stamped in the very millisecond of `now_ms` would fall outside a
/// window that simply ended at `now_ms` — and a forecast asked for right after
/// something happened would miss it, some of the time (a real intermittent
/// failure: three approvals counted as two). The window therefore ends one
/// millisecond after `now_ms`, which puts everything recorded up to and
/// including `now_ms` inside it while leaving the number of periods unchanged.
#[must_use]
pub fn history_window(now_ms: i64, periods: usize, period_ms: i64) -> (i64, i64) {
    let to_ms = now_ms.saturating_add(1);
    let from_ms = to_ms.saturating_sub(
        i64::try_from(periods)
            .unwrap_or(0)
            .saturating_mul(period_ms),
    );
    (from_ms, to_ms)
}

/// Bucket completion instants into fixed periods, oldest first.
///
/// This is the input a batch forecast actually needs — **not** the
/// cycle-time distribution (see [`forecast_batch`] for why).
#[must_use]
pub fn throughput_history(
    completed_ms: &[i64],
    from_ms: i64,
    to_ms: i64,
    period_ms: i64,
) -> Vec<usize> {
    if period_ms <= 0 || to_ms <= from_ms {
        return Vec::new();
    }
    let span = to_ms.saturating_sub(from_ms);
    // A partial trailing period would understate throughput, so only
    // whole periods are counted.
    let periods = usize::try_from(span / period_ms).unwrap_or(0);
    let mut buckets = vec![0usize; periods];
    for at in completed_ms {
        if *at < from_ms
            || *at
                >= from_ms.saturating_add(
                    i64::try_from(periods)
                        .unwrap_or(0)
                        .saturating_mul(period_ms),
                )
        {
            continue;
        }
        let index = usize::try_from((at - from_ms) / period_ms).unwrap_or(0);
        if let Some(bucket) = buckets.get_mut(index) {
            *bucket += 1;
        }
    }
    buckets
}

/// "How long will these N items take?"
#[derive(Clone, Debug, Serialize)]
pub struct BatchForecast {
    /// Items asked about.
    pub items: usize,
    /// Simulation trials run.
    pub trials: usize,
    /// Periods of history drawn from.
    pub history_periods: usize,
    /// Calendar days per period.
    pub period_days: i64,
    /// The typical outcome — a coin-flip, not a commitment.
    pub p50_days: Option<f64>,
    /// The figure to quote: 85% of trials finished by here.
    pub p85_days: Option<f64>,
    /// The cautious figure.
    pub p95_days: Option<f64>,
    /// Trials that hit the per-trial ceiling without finishing. A
    /// non-zero count means the percentiles above are floors.
    pub trials_hit_ceiling: usize,
    /// Why the forecast is null, when it is.
    pub reason: Option<String>,
    /// Which direction is conservative, stated in the payload so it
    /// cannot be read the wrong way round.
    pub note: &'static str,
}

/// Forecast how long a batch of `items` will take, by sampling the
/// team's own **throughput** history.
///
/// **Throughput, not cycle time — and the difference is the whole
/// point.** The cycle-time distribution answers a question about *one
/// item* ("this will finish within 11 calendar days at 85% confidence"), which
/// is what the service level expectation already reports. Using it for
/// a batch means implicitly assuming items are worked one at a time:
/// sum twenty cycle times for a team running five in parallel and the
/// answer is roughly five times too pessimistic. Throughput sampling
/// makes no such assumption, because parallelism is already baked into
/// how many items the team actually finished per week.
///
/// Each trial draws periods at random **with replacement** from the
/// history and accumulates until the batch is covered. Sampling with
/// replacement is what makes the result a distribution rather than a
/// replay of the past in order.
///
/// For "how long", the **higher** percentile is the more conservative
/// one — the opposite of [`forecast_items`], which is why both carry a
/// `note` saying so.
#[must_use]
pub fn forecast_batch(
    history: &[usize],
    items: usize,
    trials: usize,
    period_days: i64,
    seed: u64,
) -> BatchForecast {
    let trials = trials.clamp(1, MAX_TRIALS);
    let base = BatchForecast {
        items,
        trials,
        history_periods: history.len(),
        period_days,
        p50_days: None,
        p85_days: None,
        p95_days: None,
        trials_hit_ceiling: 0,
        reason: None,
        note: "higher percentile = more conservative: 85% of simulated runs                finished by the p85 figure",
    };

    if history.len() < MIN_THROUGHPUT_PERIODS {
        return BatchForecast {
            reason: Some(format!(
                "only {} periods of throughput history; a forecast needs at least                  {MIN_THROUGHPUT_PERIODS} or it is arithmetic on noise",
                history.len()
            )),
            ..base
        };
    }
    if items == 0 {
        return BatchForecast {
            p50_days: Some(0.0),
            p85_days: Some(0.0),
            p95_days: Some(0.0),
            ..base
        };
    }
    if history.iter().sum::<usize>() == 0 {
        return BatchForecast {
            reason: Some(
                "nothing was completed in the sample window, so there is no                  throughput to forecast from — the honest answer is `never`,                  not a number"
                    .to_string(),
            ),
            ..base
        };
    }

    let mut rng = Rng::new(seed);
    let mut results: Vec<i64> = Vec::with_capacity(trials);
    let mut hit_ceiling = 0usize;
    for _ in 0..trials {
        let mut done = 0usize;
        let mut periods = 0usize;
        while done < items && periods < MAX_PERIODS_PER_TRIAL {
            done = done.saturating_add(history[rng.index(history.len())]);
            periods += 1;
        }
        if done < items {
            hit_ceiling += 1;
        }
        results.push(i64::try_from(periods).unwrap_or(i64::MAX));
    }
    results.sort_unstable();

    #[expect(clippy::cast_precision_loss)] // display figures
    let as_days = |p: f64| percentile(&results, p).map(|periods| (periods * period_days) as f64);
    BatchForecast {
        p50_days: as_days(0.50),
        p85_days: as_days(0.85),
        p95_days: as_days(0.95),
        trials_hit_ceiling: hit_ceiling,
        ..base
    }
}

/// "How many items will we finish in the next N periods?"
#[derive(Clone, Debug, Serialize)]
pub struct ItemsForecast {
    /// Periods asked about.
    pub periods: usize,
    /// The same as calendar days.
    pub days: i64,
    /// Simulation trials run.
    pub trials: usize,
    /// Periods of history drawn from.
    pub history_periods: usize,
    /// The typical outcome — a coin-flip, not a commitment.
    pub median_items: Option<i64>,
    /// **At least** this many, with 85% confidence.
    ///
    /// This is the **15th** percentile of the simulated distribution,
    /// not the 85th. For "how many", the conservative end is the low
    /// one, and quoting the p85 here would promise the best case while
    /// sounding careful.
    pub at_least_items: Option<i64>,
    /// The optimistic end, labelled as such.
    pub at_most_items: Option<i64>,
    /// Why the forecast is null, when it is.
    pub reason: Option<String>,
    /// Which direction is conservative, stated in the payload.
    pub note: &'static str,
}

/// Forecast how many items a period window will deliver.
///
/// The mirror of [`forecast_batch`], and the percentile direction
/// **reverses**: 85% confidence of finishing *at least* N items is the
/// 15th percentile of the distribution, not the 85th. Getting this
/// backwards produces a forecast that reads as cautious and promises
/// the best case, so both figures are named for what they mean rather
/// than for the percentile they came from.
#[must_use]
pub fn forecast_items(
    history: &[usize],
    periods: usize,
    trials: usize,
    period_days: i64,
    seed: u64,
) -> ItemsForecast {
    let trials = trials.clamp(1, MAX_TRIALS);
    let base = ItemsForecast {
        periods,
        days: i64::try_from(periods)
            .unwrap_or(0)
            .saturating_mul(period_days),
        trials,
        history_periods: history.len(),
        median_items: None,
        at_least_items: None,
        at_most_items: None,
        reason: None,
        note: "at_least_items is the 15th percentile: for `how many`, the                conservative end is the low one",
    };

    if history.len() < MIN_THROUGHPUT_PERIODS {
        return ItemsForecast {
            reason: Some(format!(
                "only {} periods of throughput history; a forecast needs at least                  {MIN_THROUGHPUT_PERIODS} or it is arithmetic on noise",
                history.len()
            )),
            ..base
        };
    }
    if periods == 0 || periods > MAX_PERIODS_PER_TRIAL {
        return ItemsForecast {
            reason: Some(format!(
                "periods must be between 1 and {MAX_PERIODS_PER_TRIAL}"
            )),
            ..base
        };
    }

    let mut rng = Rng::new(seed);
    let mut results: Vec<i64> = Vec::with_capacity(trials);
    for _ in 0..trials {
        let mut done = 0usize;
        for _ in 0..periods {
            done = done.saturating_add(history[rng.index(history.len())]);
        }
        results.push(i64::try_from(done).unwrap_or(i64::MAX));
    }
    results.sort_unstable();

    ItemsForecast {
        median_items: percentile(&results, 0.50),
        at_least_items: percentile(&results, 0.15),
        at_most_items: percentile(&results, 0.85),
        ..base
    }
}
