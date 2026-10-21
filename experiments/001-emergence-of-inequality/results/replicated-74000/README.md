# Hiyal Experiment 001 — 10-pair replicated exploratory grid

**Status: exploratory (not registered or confirmatory).** Source: [GitHub Actions run #39553947265](https://github.com/davletovb/social-emergence-lab/actions/runs/39553947265), a manual full-grid run on October 21, 2026, at lab commit `9848f0983d16b334a064046dcd29d5800da3e26a` (the head of PR #5, reachable from `main` through merge `3a9092e`). Hiyal is pinned to `c6f3d5d650457077eaff4e26a0060bc6cee5dbdd`. Rust toolchain, Cargo.lock digest, CPU/OS and timestamps are in [reproducibility-manifest.json](reproducibility-manifest.json) (manifest schema 2).

## All specifications, not a favorable-only subset

The study ran **18 configured cells × 10 paired society seeds (74000–74009)**. The actual engine used Hiyal's pinned ChaCha20 scheduler and durable event journal, with **one uniform control reused across three treatment offsets for each population/horizon/seed**. Accordingly, these 18 comparisons are *correlated*, not 18 independent replication experiments.

Committed outputs, each **byte-identical** to the archived file:
- [Full 18-cell summary with paired-bootstrap intervals](hiyal-summary.csv).
- [720 rows of Gini, top share and mobility trajectories](hiyal-trajectories.csv).
- [240 per-segment runtime and storage measurements](hiyal-benchmarks.csv).
- [Exact experiment parameters](study-plan.json), [study report](hiyal-study-report.json) and [environment/source provenance](reproducibility-manifest.json).
- [SHA256SUMS](SHA256SUMS): the archive's seal, 1,926 entries. Its own SHA-256 is `5e585b12060805ff210ac5de26672455cd450ccec8be2762959e5fe19a487b81`, which the CI job also printed.

Check the committed copies against the seal from this directory:

```sh
grep -E '  \./(hiyal-summary\.csv|hiyal-trajectories\.csv|hiyal-benchmarks\.csv|study-plan\.json|hiyal-study-report\.json|reproducibility-manifest\.json)$' SHA256SUMS | sha256sum --check --strict
```

## Evidence archive

The complete original Hiyal genesis, event journals, checkpoints, paired-fork metadata and copied parent ancestry were uploaded as the GitHub Actions artifact **experiment-001-10pair-full-evidence** of [run #39553947265](https://github.com/davletovb/social-emergence-lab/actions/runs/39553947265). Artifact ID: `12324369551`; compressed size **9,824,690 bytes**; service-reported digest `sha256:1eb1160f8a3d9db36a23271e598dce2364c30064e6ce3ab21e881fde9dc5fea5`.

Before upload, `scripts/verify-001.sh` checked in CI that `SHA256SUMS` matches and covers exactly the files present, that the trial directories are exactly the planned 60 parents and 240 arms, and that every parent and arm passes Hiyal ledger, fork-attribution and independent reference replay. The upload includes Hiyal's empty `.writer.lock` markers, which are not evidence and are excluded from the seal. To check a downloaded copy, extract it into a new directory, confirm `cmp <dir>/SHA256SUMS SHA256SUMS`, and run `bash scripts/verify-001.sh <dir>` from a checkout of this repository.

**This GitHub Actions artifact expires January 19, 2027.** It has **not** been copied to permanent archival storage. Download and preserve it in controlled long-term storage before expiry. The CSVs stored in Git are not substitutes for the original raw journals; the committed `SHA256SUMS` lets anyone confirm that a preserved copy is the original.

## Verification summary and performance

- **72,000 accepted events** across **240 original arm histories** and 60 parents in six shared-control groups, no LLM calls.
- Summed segment run+checkpoint time **20,588 ms**; summed ledger/reference verification and trajectory reconstruction **5,034 ms**. These are not full job/CPU wall times, and the machine was not used for a controlled hardware benchmark.
- Summed per-arm event journal sizes at verified completion: **166,817,663 bytes** (not compressed archive size; duplicated ancestral evidence also consumes space).

## Scientific interpretation

For 20 actors with offset=1 at 500 accepted opportunities, mean Gini difference (treatment-control) was **+0.058128571429** with an exploratory paired percentile bootstrap 95% interval **[+0.043928571429,+0.070985714286]**. This is **within-model exploratory evidence**, not a preregistered test or demonstration of causal mechanisms in real human populations.

With only 10 paired societies per cell, correlated offset treatments and multiple exploratory comparisons, treat intervals as uncertainty diagnostics. Every negative/null effect remains in the CSV. The candidate confirmatory protocol is a separate draft and no confirmatory dataset has been accessed.

## Superseded archive

The earlier [run #39548747760](https://github.com/davletovb/social-emergence-lab/actions/runs/39548747760) (artifact `12322437801`) cannot pass its own check: its `SHA256SUMS` listed 600 hidden lock files that the upload dropped. Its summary and trajectories are byte-identical to this run's, but its committed `study-plan.json` and report copies carried an extra trailing newline, and its schema-1 manifest recorded a pull-request synthetic merge commit (`bd0a75bb…`) that no ref keeps alive. It remains in git history and is superseded by this run.
