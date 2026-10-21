# Hiyal-backed sensitivity, inequality trajectories and performance (exploratory)

**Status:** scientific tooling implementation, not preregistered confirmation. The earlier independent SplitMix64 sensitivity grid remains a separate reference model and is **not** Hiyal-generated evidence.

## What the new Hiyal runner does

Run two paired branches (uniform opportunity allocation vs linear wealth feedback) through Hiyal's pinned ChaCha20 engine. Each condition is born from the same 0-event parent and records its own audited actions, named RNG state, snapshot and fork attribution. Every final state is independently verified using Hiyal's event-hash validation **and** an independent plain-vector replay before any derived statistic is published.

The fixed exploratory parameter grid has 18 cells:
- actor population **10, 20, 40**;
- wealth-feedback offset **1, 10, 100**; equal initial wealth 10;
- horizon **100, 500 accepted opportunities**;
- up to **10 paired society seeds per cell**, with a total cap of 120,000 accepted events (across control + treatment).

To prevent expensive accidental runs, the grid is **manual-only** and not part of ordinary PR CI. A two-cell, one-pair smoke is the default PR integration. Use unique output directories per scientific plan. The runner creates immutable \`study-plan.json\`; resuming a directory with a changed plan is refused.

## Run

\`\`\`sh
cargo test --locked
cargo run --locked --bin hiyal-grid -- smoke /tmp/hiyal-studies-smoke 1 62000
cargo run --locked --release --bin hiyal-grid -- grid /tmp/hiyal-studies-grid 10 63000
\`\`\`

**Exploratory-only**: the grid's seed range, effect estimates, and statistical intervals are not held-out confirmatory tests. Running it does not register a hypothesis.

Outputs: \`hiyal-trajectories.csv\`, \`hiyal-benchmarks.csv\`, \`hiyal-summary.csv\`, \`hiyal-study-report.json\` and an immutable \`study-plan.json\`. The authoritative Hiyal journals/snapshots/fork bundles remain in per-cell and per-seed directories. Derived files are *recomputed from fully verified histories*, not treated as evidence of accepted steps.

## Trajectories and mobility

- Gini and top-quintile wealth share are measured at approximately 25%, 50%, and 100% of each horizon.
- **Rank persistence** is a tie-aware Spearman correlation of each actor's midrank at the 25% baseline versus a later checkpoint. It is undefined when all actors tie at either checkpoint, represented by an empty CSV value.
- **Rank movement** is mean absolute midrank change divided by \`n(n - 1)\`. This is a descriptive agent-level *summary within one society*, not a statistical sample of independent agents.
- The baseline checkpoint itself has rank persistence 1 if non-tied, otherwise undefined, and movement 0. All subsequent comparisons use the **same baseline**.
- Rankings cannot be identified at initial equality, hence mobility is not measured from step zero.

These are descriptive properties of the artificial actors, not evidence of human class mobility.

## Performance/retention experiment

Every arm logs:
- \`new_accepted_steps\`: only committed opportunities added in **this invocation**; zero after a completed rerun;
- \`run_ms\`: wall-clock time to execute those new steps plus snapshot;
- \`verification_ms\`: complete fork/ledger validation, independent reference replay, and reconstruction of trajectory checkpoints;
- \`journal_bytes\`, \`snapshot_bytes\`, \`state_json_bytes\`: on-disk event and snapshot sizes and serialized final state size.

The default Hiyal durability is **EveryEvent**, intentionally left unchanged for crash safety. The measurements include local disk I/O and scheduler overhead. No peak RSS measurement is claimed; \`state_json_bytes\` is a serialization size, **not process memory**. These are machine-specific profiles, not comparisons to the original scalar counter benchmark.

For longer horizons, journal validation and checkpoint economics should be benchmarked and assessed before increasing the grid cap. Wall time is diagnostic and cannot serve as a version-independent performance guarantee.

## Statistical plan

One independent *paired society seed* is the unit of analysis. Per-cell difference equals mean Gini(treatment-control), with a paired percentile bootstrap 95% interval for **at least 10** paired societies. With fewer pairs, the interval is omitted; we do not give false precision for a one-pair smoke. Report all grid cells including negative/null effects; do not infer any universal sociological mechanism from this synthetic experiment.

## Remaining before scientific confirmation

- [x] Hiyal-executed exploratory parameters, independent replay validation and frozen plan per directory.
- [x] Trajectory-level Gini, top share, rank persistence and rank movement.
- [x] Per-arm runtime, Hiyal journal/snapshot size and verification-cost measurements.
- [ ] Green GitHub CI for this implementation, and a full **manual** 18-cell Hiyal grid with results retained.
- [ ] Machine/CPU/disk provenance, compiler/toolchain + lab Git SHA in each run manifest; explicit process RSS profiling.
- [ ] Valid power/precision calculation, held-out seeds, pre-specified terminal contrast/decision rule, error handling and stopping criteria.
- [ ] Robustness to altered agent-interaction mechanisms and independent real-world operationalization.

The code and the synthetic model cannot prove that a real social system obeys cumulative advantage.
