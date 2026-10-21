# One-seed Hiyal-backed exploratory grid: seed 64000

Source: local run on 2026-10-21 at code revision `93c7f705cd3a2dd317f1f3e7c35aa332057b6d0b` (study plan schema 2, one shared control per group), pinned Hiyal revision `c6f3d5d650457077eaff4e26a0060bc6cee5dbdd` and Rust 1.89.0, in a 4-vCPU Linux container (Intel Xeon @ 2.80GHz). The invocation was run twice into separate directories; the summary and trajectory files were byte-identical.

Run invocation:
```sh
cargo run --locked --release --bin hiyal-grid -- grid /tmp/hiyal-grid-profile 1 64000
```

The full 18-cell *one paired society per cell* grid ran successfully. Its 18 cells form 6 (actors, horizon) groups, each with one shared uniform control and three treatments (offsets 1, 10, 100). It executed **7,200 accepted events** across 24 arm histories (6 controls + 18 treatments). Aggregate measured **run+checkpoint wall time** was 3781 ms, with a further 777 ms of independent ledger/reference verification and trajectory reconstruction. Total journal bytes across the 24 arm files: **16,681,726 bytes** (15.91 MiB). These sums exclude compilation, process startup, parent/fork creation, disk cache effects and machine contention, and are machine-specific. They are not controlled benchmarks of optimization alternatives.

Derived outputs, copied unchanged from the run directory:
- [All 18 Hiyal grid cell outcomes](hiyal-grid-one-seed-64000-summary.csv)
- [Per-segment runtime and on-disk storage](hiyal-grid-one-seed-64000-benchmarks.csv) (append-only log; control rows have an empty offset)
- [Inequality trajectories and rank movement](hiyal-grid-one-seed-64000-trajectories.csv) (control rows appear once per group with an empty offset)
- [Frozen study plan](hiyal-grid-one-seed-64000-study-plan.json) and [run report](hiyal-grid-one-seed-64000-report.json)

These files contain the derived results only. **The authoritative Hiyal journals, snapshots, fork ancestry and full manifests were not retained**; the code reproduces them at the cited revision, but this checked-in evidence is not itself a complete replication archive. Model output does not provide independent empirical validation of social inequality.

**Inference warning:** one society pair per cell; confidence intervals are intentionally blank and no statistical significance is claimed. Within each group the three offset rows reuse the same control society, so they are correlated, not independent replications. 13 of the 18 differences are positive and 5 are negative (five of the six 40-actor cells). A confirmatory study requires a frozen protocol and adequately sized independent paired societies.

**Supersedes:** the earlier [GitHub Actions run #39543708177](https://github.com/davletovb/social-emergence-lab/actions/runs/39543708177) at commit `685376706bc198ce776983a453f18ad308000317`, archived here at commit `9b90fb152341f7c29c854a1dabb73fbfd4c9f539`, used one control per cell (plan schema 1). Its trial IDs and random streams differ, so its values are not reproducible with the current code; they remain in git history.
