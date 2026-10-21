# Experiment 002 — V2 exploratory, exact-seed 36-pair development

This study evaluates a **synthetic, specified** reputation-network mechanism, not human societies. It is unregistered, not confirmatory, and not a novel scientific law.

## Why 36 independent parent pairs?

Previous *unarchived, development-only* 20-parent observations (seeds 92000–92003 and 97000–97015) reported approximate standard deviations **0.128** for δ=0.2 information-only late signed gap and **0.147** for matched δ=0.2 minus δ=0 late-gap difference. At N=36, a descriptive 95% t interval would have *estimated* half-width `2.0301 × SD / √36`, or approximately **0.043** and **0.050**, respectively, if variance is stable. Those estimates are not guarantees, power calculations or criteria for declaring success. The zero-prior symmetry baseline is exactly 0 in expectation for this synthetic model. **Primary reported statistic:** the mean late signed information-only opportunity gap with δ=0.2; **secondary:** same-seed difference versus δ=0, plus zero-prior symmetry and repair contrasts. Matching alone was previously found to *increase* variance, so do not assume the paired difference is more precise.

## Fixed model and sample

- Model: 20 synthetic agents, directed graph degree 4, identical quality 0.5, false reputation label δ=0.2 vs exact null δ=0, intervention checkpoint τ=200, end T=600; 400 accepted opportunities after checkpoint. Four matched intervention forks under each parent, with a last-100-event outcome.
- Seeds **98000–98035**, declared before any outcomes for these seeds are inspected. Partition into **three disjoint 12-seed shards**: 98000–98011, 98012–98023, 98024–98035. An unsuccessful or missing seed is a reported failure and must never be silently replaced.
- Each seed receives two independent but setup-matched Hiyal societies; comparisons are at the **parent-seed level**. Ten trial trees (two parent journals and eight four-fork arm journals) per seed, not independent replication of those forks. The two parents' development trajectories need not be identical.
- All inference is within-model; no participant recruitment, naturalistic data, or empirically calibrated human response probabilities. It is not a test of causal effects in human populations.

## Provenance, cost and evidence

`scripts/pilot-paired-002-shard.sh` builds the release Rust binary once per shard, runs exactly 12 fixed parent pairs, emits raw Hiyal parent/fork journals, and verifies ancestry, setup, hashes, independent reference replay and saved `paired-report.json`. Shard manifests `SHA256SUMS` are sealed and verified before upload. A one-time GitHub workflow `.github/workflows/experiment-002-v2-one-time.yml` runs **three independent CI jobs** in parallel, with a maximum **29 minutes each** and a 90-day artifact retention limit. The aggregation job fails if any shard is missing, its seal is wrong, or the union of seed lists differs from the full 36-seed domain.

The 36-pair summary reports the mean, sample standard deviation, cross-arm correlation, all 36 per-pair observations, and approximate exploratory t intervals at df=35 using `t≈2.030108`. No significance threshold, stopping boundary, or novelty claim is registered. Results should be considered only if all three shards and the aggregate finish successfully and every original Hiyal archive is retained.

The workflow and its high-cost jobs are **temporary**. Remove them after results are verified, and retain only a manual script for repetition. Track permanent storage of raw archives independently; GitHub Actions artifacts will expire.

## Decisions after review

Even a positive within-model reputation effect can follow trivially from the explicitly coded feedback mechanism. Scientific novelty needs independent theory critique, stronger null and graph-family sensitivity, calibration against accessible authorized human evidence, and a forward-citation literature search. Cost/profiling improvements should preferably be in Hiyal's journal/state-processing layer rather than replacing reproducible evidence with unverified fast approximations.


## Outcome and completion

All three 12-seed shards completed their parent/fork replay, exact sample inventory and SHA manifest verification at [run 39683450918](https://github.com/davletovb/social-emergence-lab/actions/runs/39683450918). The original run's summary job failed on a jq `sqrt/1` syntax error; [run 39684883492](https://github.com/davletovb/social-emergence-lab/actions/runs/39684883492) downloaded the **same signed shard artifacts** and successfully recomputed descriptive intervals, with **no new trials**. The expensive workflows were removed after use to prevent further PR CI expense. Complete [results and custody details](results/v2-36-paired-20261021/README.md).

A secondary measurement is algebraically redundant in this design: differences in the `repair_after_truth` contrast match the differences in the information-only late gap at every one of 36 observed seeds, because the combined repair arm produces the same outcome after both sides are truth-reset and graph-repaired. This is **not** independent confirmation. The primary δ=0.2 late gap remains an exploratory estimate of a deliberately encoded model mechanism.
