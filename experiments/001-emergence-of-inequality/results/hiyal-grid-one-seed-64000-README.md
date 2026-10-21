# One-seed Hiyal-backed exploratory grid: seed 64000

Source: [GitHub Actions run #39543708177](https://github.com/davletovb/social-emergence-lab/actions/runs/39543708177), executed on PR #4 at commit `685376706bc198ce776983a453f18ad308000317`, using pinned Hiyal revision `c6f3d5d650457077eaff4e26a0060bc6cee5dbdd` and Rust 1.89.

The full 18-cell *one paired society per cell* grid ran successfully. It executed **10,800 accepted events** across 36 arm histories. Aggregate measured **run+checkpoint wall time** was 5197 ms, with a further 1538 ms of independent fork/journal/reference verification and trajectory reconstruction. Total journal bytes across the 36 arm files: **25,022,591 bytes** (23.86 MiB). These sums are not inclusive of compilation, process startup, parent/fork creation, repeated platform overhead, disk cache effects, or CI allocation. They are not controlled benchmarks of optimization alternatives.

Evidence copied **verbatim from CI stdout** into:
- [All 18 Hiyal grid cell outcomes](hiyal-grid-one-seed-64000-summary.csv)
- [Per-arm runtime and on-disk storage](hiyal-grid-one-seed-64000-benchmarks.csv)
- [Inequality trajectories and rank movement](hiyal-grid-one-seed-64000-trajectories.csv)

Run invocation:
```sh
cargo run --locked --release --bin hiyal-grid -- grid /tmp/hiyal-grid-profile 1 64000
```

These files contain the derived results only. **The authoritative Hiyal journals, snapshots, fork ancestry and full manifests were not retained** after the CI runner; the code can reproduce them locally at the pinned revision, but this checked-in evidence is not itself a complete replication archive. Model output does not provide independent empirical validation of social inequality.

**Inference warning:** one society pair per cell; confidence intervals are intentionally blank and no statistical significance is claimed. There are negative, zero-ish, and positive differences. A confirmatory study requires a frozen protocol and adequately sized independent paired societies.
