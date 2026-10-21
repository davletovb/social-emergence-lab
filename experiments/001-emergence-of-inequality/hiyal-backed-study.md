# Hiyal-backed sensitivity, inequality trajectories and performance (exploratory)

**Status:** scientific tooling implementation, not preregistered confirmation. The earlier independent SplitMix64 sensitivity grid remains a separate reference model and is **not** Hiyal-generated evidence.

## What the new Hiyal runner does

Run paired branches (uniform opportunity allocation vs linear wealth feedback) through Hiyal's pinned ChaCha20 engine. The uniform control ignores the feedback offset, so each **(actors, initial wealth, horizon, seed) group** has one 0-event parent and one shared control, and every offset cell in that group forks its own treatment from the same parent (directories `<group>/seed-N/arms/control` and `arms/treatment-offset-K`). Each arm records its own audited actions, named RNG state, snapshot and fork attribution. Every final state is independently verified using Hiyal's event-hash validation **and** an independent plain-vector replay before any derived statistic is published.

The fixed exploratory parameter grid has 18 cells:
- actor population **10, 20, 40**;
- wealth-feedback offset **1, 10, 100**; equal initial wealth 10;
- horizon **100, 500 accepted opportunities**;
- up to **10 paired society seeds per cell**, with a total cap of 120,000 accepted events (shared controls plus treatments; the full 10-pair grid needs 72,000).

To prevent expensive accidental runs, the grid is **manual-only** and not part of ordinary PR CI. A two-cell, one-pair smoke is the default PR integration. Use unique output directories per scientific plan. The runner creates immutable \`study-plan.json\` (its Hiyal revision is read from \`Cargo.lock\`); resuming a directory with a changed plan is refused.

Interrupted runs resume safely: parents are published atomically, a lone arm left at the fork boundary is discarded and re-forked (an arm with committed events is never replaced), a leftover \`study-plan.tmp\` is ignored, and arms that stopped part-way continue to the horizon with results identical to an uninterrupted run. Existing arms must descend from *this* seed's parent, so arms copied between seeds or cells are refused. Completed arms are only inspected, never reopened for writing.

## Run

\`\`\`sh
cargo test --locked
cargo run --locked --bin hiyal-grid -- smoke /tmp/hiyal-studies-smoke 1 62000
cargo run --locked --release --bin hiyal-grid -- grid /tmp/hiyal-studies-grid 10 63000
\`\`\`

**Exploratory-only**: the grid's seed range, effect estimates, and statistical intervals are not held-out confirmatory tests. Running it does not register a hypothesis.

Outputs: \`hiyal-trajectories.csv\`, \`hiyal-benchmarks.csv\`, \`hiyal-summary.csv\`, \`hiyal-study-report.json\` and an immutable \`study-plan.json\`. Every derived CSV includes an \`initial\` (starting wealth) column, since it is part of each cell's identity. The authoritative Hiyal journals/snapshots/fork bundles remain in per-cell and per-seed directories. Derived files are *recomputed from fully verified histories*, not treated as evidence of accepted steps.

## Trajectories and mobility

- Gini and top-quintile wealth share are measured at approximately 25%, 50%, and 100% of each horizon.
- **Rank persistence** is a tie-aware Spearman correlation of each actor's midrank at the 25% baseline versus a later checkpoint. It is undefined when all actors tie at either checkpoint, represented by an empty CSV value.
- **Rank movement** is mean absolute midrank change divided by \`n(n - 1)\`. This is a descriptive agent-level *summary within one society*, not a statistical sample of independent agents.
- The baseline checkpoint itself has rank persistence 1 if non-tied, otherwise undefined, and movement 0. All subsequent comparisons use the **same baseline**.
- Rankings cannot be identified at initial equality, hence mobility is not measured from step zero.

These are descriptive properties of the artificial actors, not evidence of human class mobility.

## Performance/retention experiment

\`hiyal-benchmarks.csv\` is an **append-only log with one row per run segment**, written durably as soon as the segment is verified, so timings survive interruption and are never overwritten. A completed rerun adds no rows. Each row records:
- \`start_step\` and \`new_accepted_steps\`: the logical step the segment started from and the opportunities it committed (a gap between segments means a run was killed before its row was written);
- \`run_ms\`: wall-clock time to execute those new steps plus snapshot;
- \`verification_ms\`: one full Hiyal ledger validation plus a single independent reference replay that also reconstructs the trajectory checkpoints;
- \`journal_bytes\`, \`snapshot_bytes\`, \`state_json_bytes\`: on-disk event and snapshot sizes and serialized final state size.

The default Hiyal durability is **EveryEvent**, intentionally left unchanged for crash safety. The measurements include local disk I/O and scheduler overhead. No peak RSS measurement is claimed; \`state_json_bytes\` is a serialization size, **not process memory**. These are machine-specific profiles, not comparisons to the original scalar counter benchmark.

For longer horizons, journal validation and checkpoint economics should be benchmarked and assessed before increasing the grid cap. Wall time is diagnostic and cannot serve as a version-independent performance guarantee.

## Statistical plan

One independent *paired society seed* is the unit of analysis. Per-cell difference equals mean Gini(treatment-control), where the control is the group's shared control. **The three offset rows of a group reuse the same control societies, so they are correlated and must not be treated as independent specifications** (the same convention as the reference sensitivity grid). Study plans use schema 2; directories written by the earlier per-cell-control runner (schema 1) are refused rather than mixed. with a paired percentile bootstrap 95% interval for **at least 10** paired societies. With fewer pairs, the interval is omitted; we do not give false precision for a one-pair smoke. Report all grid cells including negative/null effects; do not infer any universal sociological mechanism from this synthetic experiment.

## Remaining before scientific confirmation

- [x] Hiyal-executed exploratory parameters, independent replay validation and frozen plan per directory.
- [x] Trajectory-level Gini, top share, rank persistence and rank movement.
- [x] Per-arm runtime, Hiyal journal/snapshot size and verification-cost measurements.
- [x] Green GitHub CI and a 1-pair-per-cell 18-cell Hiyal grid (shared controls, schema 2) with derived results retained; [source and caveats](results/hiyal-grid-one-seed-64000-README.md).
- [ ] Full replicated 18-cell Hiyal grid (10 paired societies/cell), with authoritative journals and run-level provenance preserved.
- [ ] Machine/CPU/disk provenance, compiler/toolchain + lab Git SHA in each run manifest; explicit process RSS profiling.
- [ ] Valid power/precision calculation, held-out seeds, pre-specified terminal contrast/decision rule, error handling and stopping criteria.
- [ ] Robustness to altered agent-interaction mechanisms and independent real-world operationalization.

The code and the synthetic model cannot prove that a real social system obeys cumulative advantage.
