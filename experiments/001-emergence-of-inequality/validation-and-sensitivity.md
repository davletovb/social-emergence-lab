# Experiment 001 — validation, uncertainty and sensitivity

**Status:** implemented as exploratory research tooling. Neither the previous pilot nor subsequent sensitivity output is a preregistered confirmatory test.

## Three independent checks

1. **Hiyal integrity:** `Store::inspect_read_only` validates the event journal and reconstructed state.
2. **Mechanism reference:** `reference::replay` parses accepted recipients from JSONL using an independently written plain-vector reducer. It replays exactly the journal prefix Hiyal verified and compares every final actor balance and accepted event count to the verified Hiyal state, adding the fork's starting opportunity count so arms forked after parent steps validate correctly.
3. **Metric reference:** an ordered weighted-sum Gini implementation is compared with the primary pairwise absolute-difference definition, and the top-quintile metric is recomputed from a balance histogram rather than by sorting, with its ceil(n/5) definition pinned by hand-computed cases (including n=7, n=12 and ties). Tests include exhaustive three-agent label permutations, accounting, ties and known inequalities.

These are *software and formal-mechanism* checks; they do not establish that modeled wealth or reinforcement corresponds to actual human wealth or status.

## Inferential summaries

`analyze` pairs rows by **society seed**, rejects duplicate/unpaired/mismatched-horizon samples and malformed Gini, top-quintile or chain-head fields (attempts may exceed accepted opportunities because Hiyal also counts rejected and pause records), and reports the mean control Gini, mean treatment Gini, the **mean within-society difference**, and sample standard deviation across independent societies. For 10 or more matched societies it also reports a *nonparametric paired percentile bootstrap 95% interval* using 10,000 resamples and fixed bootstrap seed `0x5eed20261021`. This is an exploratory precision diagnostic, **not** a prespecified significance test; coverage may be poor for small or highly skewed samples. Fewer than 10 pairs yield means only.

Agents and individual steps are **never** treated as independent observations for inference. The random seed is an identifier for a simulated society, not a real-world sampling frame.

## Hiyal pilot analysis

```sh
cargo test --locked
cargo run --locked -- pilot /tmp/lab-v0 20 200 2000
cargo run --locked --bin validate -- /tmp/lab-v0/seed-00000000000000002000/arms/control
cargo run --locked --bin validate -- /tmp/lab-v0/seed-00000000000000002000/arms/treatment
cargo run --locked --bin analyze -- /tmp/lab-v0/results.csv
```

Pilot output is Hiyal-event based and must be archived with its actual model and Hiyal revisions, manifest hashes and full run parameters. **Do not interpret earlier superseded sampler results as findings.**

## Independent reference sensitivity grid

```sh
cargo run --locked --bin sensitivity -- /tmp/lab-reference-grid 20 40000
```

This grid runs an **independent stochastic implementation** (SplitMix64, not Hiyal's ChaCha20) of the same specified allocation mechanism across 18 specifications:

| Factor | Grid |
| --- | --- |
| Actor count | 10, 20, 40 |
| Starting tokens | 10 per actor |
| Reinforcement offset | 1, 10, 100 |
| Accepted opportunities | 100, 500 |
| Feedback | uniform control versus linear wealth reinforcement |
| Pairing | common seed per control/treatment pair, distinct reference trials |

It exports `reference-sensitivity-raw.csv` (one line per arm/run) and `reference-sensitivity-summary.csv` (paired mean Gini difference and seeded bootstrap interval per specification). The independent reference is **not** a byte-for-byte simulator or a second Hiyal executor. Its differences help diagnose mechanism-level robustness across specifications but do not replace Hiyal-integrated sensitivity tests or empirical calibration.

The grid is exploratory; all choices and outcomes must be disclosed. No positive-only filtering or post hoc declarations of statistical significance. Effects may depend on initial wealth, horizon, offset and finite population size.

## Pending before confirmatory runs

- [x] Independent journal replay and wealth/metric comparison.
- [x] Label permutation, small exact cases and restart/tamper checks.
- [x] Uncertainty estimates computed on paired societies rather than agents/steps.
- [x] Standalone exploratory reference sensitivity grid.
- [ ] CI green on this PR and measured real-state runtime/storage.
- [ ] Independent Hiyal-engine sensitivity with multiple model configurations and trajectory statistics.
- [ ] Immutable protocol revision and full lab/config SHA provenance; held-out preassigned seeds.
- [ ] Power/precision design, predefined primary end time, specified estimand and exclusions.
- [ ] External/empirical construct validation and independent reproduction.
