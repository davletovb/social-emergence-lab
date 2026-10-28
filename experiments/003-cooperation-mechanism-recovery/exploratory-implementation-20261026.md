# Experiment 003 — exploratory implementation track (2026-10-26)

> **Status update — 2026-10-28:** This file is an **as-written, pre-execution engineering record**. Its dated statements that the pilot had not yet launched and some checks had not run were true **at that point**, but were **superseded** by merged [PR #59](https://github.com/davletovb/social-emergence-lab/pull/59), the executed and replay-verified [24-society pilot](results/exploratory-pilot-20261026.md), [exact-support benchmark](results/exact-support-benchmark-20261026.md), [reviewed bounded free-rival checks](results/restricted-h4-free-rival-v02-20261027.md) and [subsequent H4 mathematical witness work](README.md). These later results are **exploratory and author-side**, not independent P0 certification. See [the current Experiment 003 overview](README.md) and [open acceptance tracker #65](https://github.com/davletovb/social-emergence-lab/issues/65) before treating any older “not yet performed” item as current.


**Research version:** A2 v0.2.2 J/H4, inheriting the v0.2.1 six mechanisms. **Status:** engineering implementation, NOT a preregistered confirmatory experiment, complete independent verifier, model-recovery proof or empirically validated sociology. This explicit owner authorization supersedes the earlier *no implementation/pilot* restriction **for the separate exploratory track only**. Issue #50 R-06 scientific acceptance remains unchecked.

## Current branch
- `src/experiment003.rs`: six named Rust kernels M/N/C/S/L/R, four actors, broker (all three matchings), two dyad fines, conditional product-Bernoulli actions, all six actor parameter domains, H4 at-most-four-atom actor-iid setup, per-period state updates and classifier-facing public-record type.
- `src/bin/exp003-pilot.rs`: manual-only, bounded Hiyal `Trial` runner, setup/replay verification and separated private journals / public exports. Hiyal is pinned by existing Cargo.lock and Cargo.toml.
- No **new** GitHub Actions workflow, third-party crates, cloud services or LLM agents. **Important:** the existing `Experiment 001 smoke` PR workflow is triggered by `src/**` edits, so ordinary PR CI does run and consume minutes. This run must pass formatting and tests before the PR is ready.

## Manual execution

```bash
cargo test --lib experiment003
cargo run --bin exp003-pilot -- /absolute/path/to/new-exp003-pilot
```

The pilot runner refuses an existing output directory and records 2 independent societies per family/policy combination, 4 periods each, 24 societies and 384 actor-periods overall, with explicit version and fixed exploratory seeds 303000–303023. A random H4 actor tuple is sampled **once** per society, iid across four actors, with a **preselected two-atom** distribution. This is a smoke sample, not a predeclared `G_evaluation` supporting inference. Hiyal trial identities and stream domains isolate all worlds; the A/A0 worlds are *not* common-random-number forks and are **not** controlled individual matched pairs. The public-only data contain the actual regime, matching, dyad fines, actions, payoffs and period; **matching nuisance eta changes only choice-broker valuations after period 1 and never enters the contribution logit** (by v0.2.1 definition). Under A0 eta has no influence on actions or matching. Private Hiyal journals, source configurations, the truth manifest (including randomized opaque filename mapping), and implementation run order must never enter a classifier. Public filenames use OS entropy for export identifiers only; this does not affect Hiyal model randomness or scientific replay. The pilot output itself must never be treated as held-out data after development. Internal Hiyal `World::observe` intentionally sees the full latent world, while `PublicRecord` serialization is explicitly tested to contain only six public fields.

**Execution status:** GitHub branch files are committed. Rust cannot run in the chat's local container because `rustc` and `cargo` are absent. Existing GitHub Actions PR smoke **does** compile and test when formatting passes. The bounded stochastic pilot CLI is not launched automatically and no pilot outcomes are claimed. Check PR checks for the exact commit before treating CI tests as passed. In particular, there is no measured intervention effect, classification rate or evidence for coverage.

## Discovered structural and statistical boundaries to preserve

1. **M embedded in N/C/S/L/R, S⊆N⊆L**: parameter pushforward results concern full H4 laws and cannot be recast as success at singleton mechanism labeling. Existing corrected eight inclusion directions remain the conservative baseline.
2. **C versus L under rematching:** C reads the *currently matched partner's* previous public action. L observes last period's *then-matched partner*. Their naive parameter map fails on rematching. A different free-rival H4 law representation remains an unknown mathematical question.
3. **M/N fine intervention:** when effect and inverse temperature trade off at A0, the fine arm can distinguish certain representations. This does not guarantee any finite-data family-recovery power.
4. **T=1 collapse:** family law equalities C=N=L and R=M require horizon-specific reports, not a single 15-pair table.
5. **H4 coverage:** independent composite-null calibration, whole-compatible-set retention, absolute adequacy and computational feasibility all remain open. There is no justified 95% claim.

## Follow-on research activities (NOT yet performed)

- Verify the branch with a local Rust toolchain and `cargo test --lib experiment003`, then run the manual pilot and archive verified Hiyal journals, public-only projections, seeds and pin.
- Independent reference cross-check of the generated records and at least one neutral fixture subset; add tests for full mapped H4 S→N→L and M corner equalities.
- Test the C/L rematching discrepancy under observed histories and investigate M/N discriminability and numerical boundaries as *exploratory* analyses with free rival parameter fits, not fixed-corner proof claims.
- Design an exactly tractable T=1 or bounded T=2 benchmark; do not claim an operational uniform-confidence set until the optimizer/test is specified, checked and calibrated.

## Scientific gates unchanged

Independent B Python implementation and raw artifacts; independent math/15-pair H4 map; rigorous entire-set and out-of-family testing; scientific-source and novelty checks; appropriately fixed K/T/J, error rates, and independent held-out seed domains; versioned confirmatory freeze. Those do **not** block continuing engineering and small exploratory diagnostics.

**Security/observability caution:** Internal Hiyal event journals store truth and private actor state for replay. Their existence is necessary for engineering provenance but means the public-only export is an enforced *workflow boundary*, not a cryptographic sandbox. A model-selection evaluator must ingest only the public schema from a physically separate working directory.
