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
