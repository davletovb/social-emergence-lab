# Methodology and reproducibility policy

## Experimental units and identification
A **society/run**, not an individual agent or time step, is the independent experimental unit. Compare precommitted control/treatment variants using paired seeds, common baseline configurations and identical reporting intervals. If interventions change random-draw order, do not assume common random numbers guarantee perfect pairing.

## Reproducibility manifest
For every run record: study/protocol revision; Hiyal Git SHA and Rust toolchain; lab Git SHA and Cargo.lock digest; world/policy/intervention hashes; full model parameters; RNG seed/profile and stream names; number of actors, attempts and accepted events; scheduler/durability mode; runtime environment; log/checkpoint digests; termination and validation status.

Use independent validation of event histories for final publication evidence. Trusted snapshot shortcuts are optional performance tradeoffs, not equivalent to independently verifying the historical journal.

## Tests
- Conservation when the transfer mechanism is closed, and accounting identities when resources enter or leave.
- Symmetry and zero-shock negative control; deterministic replay and restart identity.
- Agent-label permutation invariance under a matched random-number mapping.
- Boundary and numerical tests for zero wealth, ties, overflow, and population sizes.
- Comparison to a small, independently written reference implementation.
- Check that analysis recomputes metrics from exported agent states rather than trusting one codepath.

## Analysis
Use per-run endpoints and trajectories. Report estimates with uncertainty intervals across independent trials, not pseudoreplicated time steps. Correct or clearly label multiplicity in secondary analyses. Use sensitivity sweeps for actor count, time horizon, shock variance, feedback strength, observation window and initialization. Report every planned arm and specification, including null/negative outcomes.

## Infrastructure and cost
Start local, CPU-only, with no model provider calls or credentials. Use separate Rust application crates depending on Hiyal via a pinned Git revision. Hiyal's built-in sweep limits are not assumptions about a custom scientific runner. Avoid automatic heavy CI: test on PRs and run large simulations manually. Keep large outputs in checksum-indexed external storage rather than committing generated data.
