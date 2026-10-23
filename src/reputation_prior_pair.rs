//! Matched-setup, independently replayable contrast for randomized false priors.
//! This is a synthetic within-seed policy comparison, not human causal inference.
use crate::reputation::{Action, Config, ReputationWorld};
use crate::reputation_pilot::{PilotReport, run, verify};
use hiyal::storage::Store;
use serde::Serialize;
use std::{fs, path::Path};

#[derive(Clone, Debug, Serialize)]
pub struct PriorPairReport {
    pub scientific_status: &'static str,
    pub seed: u64,
    pub config: Config,
    pub checkpoint: u64,
    pub total_steps: u64,
    pub prior_information_late_gap: f64,
    pub no_prior_information_late_gap: f64,
    pub matched_seed_prior_effect: f64,
    pub prior_repair_after_truth: f64,
    pub no_prior_repair_after_truth: f64,
    pub paired_repair_difference: f64,
    pub matched_setup: bool,
    pub hiyal_revision: String,
    pub limitations: &'static str,
}

pub async fn run_pair(
    root: &Path,
    seed: u64,
    config: Config,
    checkpoint: u64,
    total: u64,
) -> Result<PriorPairReport, String> {
    config.validate()?;
    if config.delta <= 0.0 {
        return Err("paired false-prior experiment requires delta > 0".into());
    }
    if root.exists() {
        return Err("refusing existing paired evidence directory".into());
    }
    fs::create_dir_all(root).map_err(|e| e.to_string())?;
    let mut null = config.clone();
    null.delta = 0.0;
    run(&root.join("false-prior"), seed, config, checkpoint, total).await?;
    run(&root.join("no-false-prior"), seed, null, checkpoint, total).await?;
    let report = derive(root)?;
    fs::write(
        root.join("paired-report.json"),
        serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(report)
}

/// Verify both independently signed Hiyal run trees and re-derive the paired
/// estimate only from their journals, without trusting their cached reports.
pub fn verify_pair(root: &Path) -> Result<PriorPairReport, String> {
    let report = derive(root)?;
    let recorded = fs::read(root.join("paired-report.json")).map_err(|e| e.to_string())?;
    if recorded != serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())? {
        return Err("paired-report.json differs from verified journals".into());
    }
    Ok(report)
}
fn derive(root: &Path) -> Result<PriorPairReport, String> {
    let with = verify(&root.join("false-prior"))?;
    let without = verify(&root.join("no-false-prior"))?;
    let a = Store::<ReputationWorld, Action>::inspect_read_only_cached(
        root.join("false-prior/parent"),
    )?;
    let b = Store::<ReputationWorld, Action>::inspect_read_only_cached(
        root.join("no-false-prior/parent"),
    )?;
    let wa = &a.genesis.initial_world;
    let wb = &b.genesis.initial_world;
    if with.seed != without.seed
        || with.checkpoint != without.checkpoint
        || with.total_steps != without.total_steps
        || with.config.actors != without.config.actors
        || with.config.degree != without.config.degree
        || with.config.delta <= 0.0
        || without.config.delta != 0.0
        || wa.boost != wb.boost
        || wa.graph != wb.graph
        || wa.repair_graph != wb.repair_graph
        || a.genesis.initial_streams != b.genesis.initial_streams
        || with.hiyal_revision != without.hiyal_revision
    {
        return Err("not a matched-setup false-prior pair".into());
    }
    let mut expected = with.config.clone();
    expected.delta = 0.0;
    // Check all other parameters exactly, not just N and degree.
    if serde_json::to_value(&expected).map_err(|e| e.to_string())?
        != serde_json::to_value(&without.config).map_err(|e| e.to_string())?
    {
        return Err("non-prior configuration differs between paired runs".into());
    }
    Ok(pair_report(&with, &without))
}
fn pair_report(with: &PilotReport, without: &PilotReport) -> PriorPairReport {
    PriorPairReport {
        scientific_status: "EXPLORATORY_PAIRED_SYNTHETIC_PRIOR_NULL_ONLY",
        seed: with.seed,
        config: with.config.clone(),
        checkpoint: with.checkpoint,
        total_steps: with.total_steps,
        prior_information_late_gap: with.primary_information_late_gap,
        no_prior_information_late_gap: without.primary_information_late_gap,
        matched_seed_prior_effect: with.primary_information_late_gap
            - without.primary_information_late_gap,
        prior_repair_after_truth: with.repair_after_truth_late,
        no_prior_repair_after_truth: without.repair_after_truth_late,
        paired_repair_difference: with.repair_after_truth_late - without.repair_after_truth_late,
        matched_setup: true,
        hiyal_revision: with.hiyal_revision.clone(),
        limitations: "One matched seed is not inference. With delta=0 the expected gap is exactly zero, so the false-prior gap alone estimates the delta effect; the matched difference adds variance unless the two sides correlate above about sd(no-prior)/(2 sd(prior)). Matching genesis graphs/labels and RNG state does not force identical stochastic histories after selection changes.",
    }
}
