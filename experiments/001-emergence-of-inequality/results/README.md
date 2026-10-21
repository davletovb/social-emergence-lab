# Exploratory reference sensitivity snapshot

**Evidence source:** [GitHub Actions successful run](https://github.com/davletovb/social-emergence-lab/actions/runs/39539862586), 2026-10-21. Run invocation: `cargo run --locked --bin sensitivity -- /tmp/lab-reference-smoke 10 40000`. Code revision for this run: `f4257651a3934596b3b33cce1d6127c9d807255b`. This checked-in CSV is a **copy of the CI stdout summary**, not a replacement for the original run's raw per-arm records.

[All 18 configurations — CSV](reference-sensitivity-smoke-40000.csv)

## Prespecified in exploratory grid code (not preregistered)

- Actors: 10, 20, 40.
- Starting wealth: 10 tokens for every agent.
- Linear feedback offset: 1, 10, 100, versus equal-weight control.
- Horizon: 100 or 500 accepted opportunities.
- Paired seeds: 40000 through 40009 (10 independent simulated societies per specification).
- RNG: independent reference-model SplitMix64; not Hiyal's ChaCha20.
- Intervals: paired percentile bootstrap 95% with 2,000 resamples and bootstrap seed `0x51_2026_001`.

## What the tiny exploratory run suggests

All nine 500-step specifications have positive mean treatment-control Gini differences. Larger feedback offsets generally attenuate those mean differences, in accordance with their dilution of wealth-based selection. The 100-step outcomes are smaller and some are negative; at 10 paired seeds, sampling uncertainty is considerable.

Example at 20 actors, offset 1: at 100 steps, mean Gini difference +0.005000 (bootstrap interval −0.010267 to +0.019700); at 500 steps, +0.046686 (+0.031800 to +0.064314).

**These are exploratory model diagnostics.** This is not an empirical test involving humans, nor a formal preregistered contrast. With only ten simulated societies and multiple related specifications, do not interpret bootstrap intervals excluding zero as proof of a general sociological law. There is no control for model-selection multiplicity, empirical validation, or Hiyal-backed parameter grid here.

## Reproduction and missing evidence

On the PR code revision cited above:

```sh
cargo test --locked
cargo run --locked --bin sensitivity -- /tmp/reference-grid 10 40000
diff -u experiments/001-emergence-of-inequality/results/reference-sensitivity-smoke-40000.csv /tmp/reference-grid/reference-sensitivity-summary.csv
```

The raw per-society CSV from this run was not preserved as a CI artifact. It can be reproduced from the exact pinned code, toolchain, seed range and parameters, subject to environment and code determinism. Full confirmation will require retained raw records and complete provenance as part of a frozen protocol.
