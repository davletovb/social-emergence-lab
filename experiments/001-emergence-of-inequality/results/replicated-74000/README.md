# Hiyal Experiment 001 — 10-pair replicated exploratory grid

**Status: exploratory (not registered or confirmatory).** Source: [successful GitHub Actions run #39548747760](https://github.com/davletovb/social-emergence-lab/actions/runs/39548747760), October 21, 2026.

## All specifications, not a favorable-only subset

The study ran **18 configured cells × 10 paired society seeds (74000–74009)**. The actual engine used Hiyal's pinned ChaCha20 scheduler and durable event journal, with **one uniform control reused across three treatment offsets for each population/horizon/seed**. Accordingly, these 18 comparisons are *correlated*, not 18 independent replication experiments.

Outputs copied verbatim from the successful CI job:
- [Full 18-cell summary with paired-bootstrap intervals](hiyal-summary.csv).
- [720 rows of Gini, top share and mobility trajectories](hiyal-trajectories.csv).
- [240 per-arm runtime and storage measurements](hiyal-benchmarks.csv).
- [Exact experiment parameters](study-plan.json), [study report](hiyal-study-report.json) and [environment/source provenance](reproducibility-manifest.json).

## Evidence archive

The complete original Hiyal genesis, event journals, checkpoints, paired-fork metadata and copied parent ancestry were uploaded as the GitHub Actions artifact **experiment-001-10pair-full-evidence** associated with [run #39548747760](https://github.com/davletovb/social-emergence-lab/actions/runs/39548747760). Artifact ID: `12322437801`; compressed size **9,659,885 bytes**, service-reported SHA-256 digest `86ec82866a594b4670e7c90aefba6eb44d2a178896b9436c60cb26627ccc8dd4`. Its internal `SHA256SUMS` validates all individual files, and `scripts/verify-001.sh` verified 240 original arm journals, complete evidence bundles, 18 cells and internal checksums.

**This GitHub Actions artifact expires November 20, 2026.** It has **not** been copied to permanent archival storage. Download and preserve it in controlled long-term storage before expiry. The CSVs stored in Git are not substitutes for the original raw journals.

## Verification summary and performance

- **72,000 accepted events** across **240 original arm histories** in six shared-control groups, no LLM calls.
- Summed segment run+checkpoint time **24,786 ms**; summed reference/fork/ledger verification time **2,691 ms**. These are not full job/CPU wall times, and the machine was not used for a controlled hardware benchmark.
- Summed per-arm event journal sizes at verified completion: **166,817,663 bytes** (not compressed archive size; duplicated ancestral evidence also consumes space).
- Lab source checked out for the scientific execution: `bd0a75bb4be49838ff16b9da02e627a540be93ee` (a GitHub pull-request synthetic merge commit); underlying implementation PR head at run: `27fa0e7b286ffa7d9d4d4ce6e0abc8cb6dc47dd2`. Hiyal pinned to `c6f3d5d650457077eaff4e26a0060bc6cee5dbdd`. The source PR is subsequently amended only for documentation/script/workflow cleanup, not the Hiyal simulation model. Full Cargo.lock digest, Rust toolchain and CPU/OS appear in the provenance file.

## Scientific interpretation

For 20 actors with offset=1 at 500 accepted opportunities, mean Gini difference (treatment-control) was **+0.058128571429** with an exploratory paired percentile bootstrap 95% interval **[+0.043928571429,+0.070985714286]**. This is **within-model exploratory evidence**, not a preregistered test or demonstration of causal mechanisms in real human populations.

With only 10 paired societies per cell, correlated offset treatments and multiple exploratory comparisons, treat intervals as uncertainty diagnostics. Every negative/null effect remains in the CSV. The candidate confirmatory protocol is a separate draft and no confirmatory dataset has been accessed.
