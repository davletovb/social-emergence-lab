//! Experiment 002 V3: frozen exploratory factorial protocol; no new evidence claims.
//! Keep this separate from legacy v0/v1/v2 defaults and journal semantics.
use crate::reputation::{Config, Removal};

pub const PERIODS: [u64; 3] = [2, 10, 20];
pub const FIRST_FEASIBILITY_SEED: u64 = 120_200;

/// Main grid, frozen before any V3 outcome: 48 fresh parents per cell, sized
/// from the m=10 full-feedback variance of the 36 V2 pairs.
pub const MAIN_GRID_PARENTS_PER_CELL: usize = 48;
pub const FIRST_MAIN_GRID_SEED: u64 = 130_000;
/// Equivalence margin and t quantile (df = 47) used by scripts/v3-decision-002.jq.
pub const DECISION_MARGIN: f64 = 0.05;
pub const DECISION_CRITICAL_T: f64 = 2.011741;

/// Seed of parent `index` in row-major `cell` (0..9, as in the pilot).
pub fn main_grid_seed(cell: usize, index: usize) -> Result<u64, String> {
    if cell >= PERIODS.len() * Mechanism::ALL.len() || index >= MAIN_GRID_PARENTS_PER_CELL {
        return Err("V3 main-grid cell must be 0..9 and parent index 0..48".into());
    }
    Ok(FIRST_MAIN_GRID_SEED + 100 * cell as u64 + index as u64)
}

/// Engineering dry run of the grid runner: same model settings, three
/// parents per cell on seeds outside every frozen range. Not evidence.
pub const SMOKE_PARENTS_PER_CELL: usize = 3;
pub const FIRST_SMOKE_SEED: u64 = 140_000;
/// t quantile for the smoke run's df = 2.
pub const SMOKE_CRITICAL_T: f64 = 4.302653;

/// Amendment 1, adopted after Stage A and before any Stage B run: Stage B
/// cells are decided by this rule, which adds a compensation category and
/// distribution-free checks. Stage A keeps the frozen scripts/v3-decision-002.jq,
/// so its plans, and with them its sealed batches, are unchanged.
pub const AMENDMENT_1_DECISION_RULE: &str = "v3-decision-002-amendment-1.jq";
/// t quantiles at parents - 2 df for the rule's control-variate interval:
/// df = 46 for the grid, df = 1 for the smoke run.
pub const CONTROL_VARIATE_CRITICAL_T: f64 = 2.012896;
pub const SMOKE_CONTROL_VARIATE_CRITICAL_T: f64 = 12.706205;

/// Rewire period and mechanism of row-major `cell` (0..9).
pub fn cell_design(cell: usize) -> Result<(u64, Mechanism), String> {
    let n = Mechanism::ALL.len();
    if cell >= PERIODS.len() * n {
        return Err("V3 cell must be 0..9".into());
    }
    Ok((PERIODS[cell / n], Mechanism::ALL[cell % n]))
}

/// The frozen design of one main-grid cell (or its smoke dry run), as the
/// grid scripts write and verify it. Stage B plans also name Amendment 1's
/// decision rule and its control-variate t quantile.
pub fn main_grid_plan(cell: usize, smoke: bool) -> Result<serde_json::Value, String> {
    let (period, mechanism) = cell_design(cell)?;
    let (parents, first, t, cv_t, status) = if smoke {
        (
            SMOKE_PARENTS_PER_CELL,
            FIRST_SMOKE_SEED,
            SMOKE_CRITICAL_T,
            SMOKE_CONTROL_VARIATE_CRITICAL_T,
            "ENGINEERING_SMOKE_NOT_EVIDENCE",
        )
    } else {
        (
            MAIN_GRID_PARENTS_PER_CELL,
            FIRST_MAIN_GRID_SEED,
            DECISION_CRITICAL_T,
            CONTROL_VARIATE_CRITICAL_T,
            "EXPLORATORY_V3_MAIN_GRID_FROZEN_2026_10_22",
        )
    };
    let seeds: Vec<u64> = (0..parents)
        .map(|i| first + 100 * cell as u64 + i as u64)
        .collect();
    let stage_a = mechanism == Mechanism::FullFeedback;
    let mut plan = serde_json::json!({
        "protocol": "experiment-002-v3-20261022",
        "scientific_status": status,
        "smoke": smoke,
        "stage": if stage_a { "A" } else { "B" },
        "cell": cell,
        "rewire_every": period,
        "mechanism": mechanism.name(),
        "parents": parents,
        "seeds": seeds,
        "actors": 20,
        "degree": 4,
        "checkpoint": 200,
        "total": 600,
        "critical_t": t,
        "margin": DECISION_MARGIN,
    });
    if !stage_a {
        plan["decision_rule"] = AMENDMENT_1_DECISION_RULE.into();
        plan["control_variate_critical_t"] = cv_t.into();
    }
    Ok(plan)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mechanism {
    RandomTurnover,
    BeliefDeletion,
    FullFeedback,
}

impl Mechanism {
    pub const ALL: [Self; 3] = [
        Self::RandomTurnover,
        Self::BeliefDeletion,
        Self::FullFeedback,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::RandomTurnover => "random-turnover",
            Self::BeliefDeletion => "belief-deletion",
            Self::FullFeedback => "full-feedback",
        }
    }

    pub fn parse(name: &str) -> Result<Self, String> {
        Self::ALL
            .into_iter()
            .find(|mechanism| mechanism.name() == name)
            .ok_or_else(|| format!("unknown V3 mechanism: {name}"))
    }
}

/// Only removal and kappa change across mechanism levels. Beta=4 and other
/// defaults stay fixed, including the repair/control forks and delta=0.2.
pub fn cell_config(
    actors: usize,
    degree: usize,
    rewire_every: u64,
    mechanism: Mechanism,
) -> Result<Config, String> {
    if !PERIODS.contains(&rewire_every) {
        return Err("V3 rewire period must be exactly 2, 10 or 20".into());
    }
    let mut c = Config::default_for(actors, degree);
    c.rewire_every = rewire_every;
    match mechanism {
        Mechanism::RandomTurnover => {
            c.removal = Removal::Uniform;
            c.kappa = 0.0;
        }
        Mechanism::BeliefDeletion => {
            c.removal = Removal::LowestBelief;
            c.kappa = 0.0;
        }
        Mechanism::FullFeedback => {
            c.removal = Removal::LowestBelief;
            c.kappa = 6.0;
        }
    }
    c.validate()?;
    Ok(c)
}

/// Prediction for uniform edge-removal hazards only. Biased deletion does
/// NOT obey this formula in general, so never treat it as an exact null there.
pub fn uniform_edge_survival(events_since_checkpoint: u64, period: u64, links: usize) -> f64 {
    if period == 0 || links == 0 {
        return f64::NAN;
    }
    let removals = events_since_checkpoint / period;
    (1.0 - 1.0 / links as f64).powf(removals as f64)
}

pub fn uniform_half_life_events(period: u64, links: usize) -> f64 {
    if period == 0 || links <= 1 {
        return f64::NAN;
    }
    period as f64 * (0.5f64.ln() / (1.0 - 1.0 / links as f64).ln())
}
