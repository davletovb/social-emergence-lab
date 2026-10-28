# Methodology and reproducibility policy

## Experimental units and identification
A **society/run**, not an individual agent or time step, is the independent experimental unit for the current synthetic models (except where multiple forks descend from a common parent, in which case the **parent society** is the relevant independent unit). Compare precommitted control/treatment variants using paired seeds when the protocol actually defines paired forks, common baseline configurations and identical reporting intervals. If interventions change random-draw order, do not assume common random numbers guarantee perfect pairing. **Experiment 003's first 24-society pilot does not pair its A/A0 seeds**, so its descriptive rate differences cannot be interpreted as matched treatment effects.

For Experiment 003 H4, each of **K independent four-actor societies in a classified dataset** shares one fixed unknown family-specific **joint** population measure G with ≤4 atoms and common scalars. The four actor tuples are **iid sampled from G**, but observable actor actions become dependent through common matching/fines, histories and private-state updates. Do not substitute the fixed-actor F oracle for H4; do not count actors, periods, or multiple forks of one parent as independent K. An outer Monte Carlo study may produce **J independent datasets**, which are a distinct replication level.

## Reproducibility manifest
For every run record: study/protocol revision; Hiyal Git SHA and Rust toolchain; lab Git SHA and Cargo.lock digest; world/policy/intervention hashes; full model parameters; RNG seed/profile and stream names; number of actors, attempts and accepted events; scheduler/durability mode; runtime environment; log/checkpoint digests; termination and validation status.

Use independent validation of event histories for final publication evidence. Trusted snapshot shortcuts are optional performance tradeoffs, not equivalent to independently verifying the historical journal.

## Tests
- Conservation when the transfer mechanism is closed, and accounting identities when resources enter or leave.
- Symmetry and zero-feedback negative control; deterministic replay and restart identity.
- Agent-label permutation invariance under a matched random-number mapping.
- Boundary and numerical tests for ties, overflow, population sizes, and zero wealth where the model can reach it.
- Comparison to a small, independently written reference implementation.
- Check that analysis recomputes metrics from exported agent states rather than trusting one codepath.

## Analysis
Use per-run endpoints and trajectories. Report estimates with uncertainty intervals across independent trials, not pseudoreplicated time steps. Correct or clearly label multiplicity in secondary analyses. Use sensitivity sweeps for actor count, time horizon, stochastic-input parameters defined by each model, feedback strength, observation window and initialization. Report every planned arm and specification, including null/negative outcomes.

## Infrastructure and cost
Start local, CPU-only, with no model provider calls or credentials. Use separate Rust application crates depending on Hiyal via a pinned Git revision. Hiyal's built-in sweep limits are not assumptions about a custom scientific runner. Avoid automatic heavy CI: test on PRs and run large simulations manually. Keep large outputs in checksum-indexed external storage rather than committing generated data.


## Scientific status and verification tiers (2026-10-28 reconciliation)

Implementation tests, a clean event-journal replay, independent-formula checks authored by the same investigator, source-isolated mathematical verification, statistically calibrated identification, and empirical validity are **different evidence tiers**. A check passing one tier does not imply another has passed. In particular:

- **001:** known analytic Pólya urn + exploratory reproduced Hiyal experiments; confirmatory protocol draft not registered.
- **002:** V2/V3 original evidence and counterfactual analyses have known post hoc amendments; report structural-memory/active-regeneration **negative findings** with the original seed and criterion provenance. V0/V3 archives are retained as GitHub Releases; V1/V2/001 independent long-term custody is still a separate obligation.
- **003:** six-family Rust model and a bounded 24-society pilot exist, along with categorical enumerators, limited free-rival searches and author-derived H4 model-law witnesses. **No independent source-isolated P0 acceptance**, no globally solved 15 directed-pair classification, no properly calibrated composite-null 95% family-set retention, and no confirmatory GO. Cross-actor first-period moments under H4 are powers of **mean propensity** when actor tuples are independent, not moments of one latent propensity shared across actors: do not reintroduce the rejected N/S Hankel argument. [P0 tracker #65](https://github.com/davletovb/social-emergence-lab/issues/65) remains open.

See the [evidence-grounded current-status ledger](current-research-status.md) for precise artifacts, commitments, dates and reproducible commands. Dated protocol drafts and execution records remain unchanged in their original temporal context; superseding status should be recorded via a clear amendment/link, **never by rewriting a historical run's precommitment or source hash**. Artifacts with truth labels, latent parameter tuples or private replay journals must be stored apart from the public-only classifier input; a filename marked PRIVATE is not an enforced information boundary.
