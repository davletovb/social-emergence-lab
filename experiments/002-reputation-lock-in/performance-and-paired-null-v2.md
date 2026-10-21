# Experiment 002 — V2 performance and matched-prior control

**Status:** independent performance/code-validation milestone, not a preregistered human-science study.

## Performance changes

The original rule-based `HiyalWorld::observe` copied an entire `ReputationWorld`, including the complete and increasing `trace`, at *every* allocator opportunity. The EffectPolicy does not use this observation to decide (it reads the world directly) and has no external effects, so V2 returns the zero-sized `()` observation. For the common persistent-network arm, `decide_with_streams` borrows the world through `Cow`; the annealed-mixing arm retains a separate owned world only when constructing the explicitly resampled graph. **These changes must not alter any policy RNG draws, committed actions, state hashes, or archived V0/V1 replay semantics.** A direct archived V0 fixture replay remains part of CI.

This is a targeted allocation optimization, **not a solution to the full replay cost**. Hiyal still hashes and stores the growing serialized `trace` in each world; long-horizon runtime may still grow superlinearly. Any change that strips or compresses recorded traces needs a new, versioned experimental world schema and independent equivalence checks first. Do not silently alter V0/V1 hash identities.

## Paired zero-false-prior comparison

Previous N=20, d=4 runs compared δ=0.2 in three societies with δ=0 in two *different* seeds. Those were unrelated samples; their mean difference could not isolate the false-prior intervention.

`reputation-paired-prior` now runs **two independently recorded Hiyal trials per seed**: δ=0.2 and δ=0, with **identical initial graph, independently pre-drawn repair graph, boost labels, and initial Hiyal named-stream positions**. The only changed parameter is δ; all other configuration properties must match. Each run includes all four pre/post correction network-repair arms. The CLI verifies each parent and fork (including journal replay and independent conditional-exposure calculations) and re-derives `paired-report.json` from trusted evidence, detecting substitution/tampering.

The **within-seed difference** in late information-only signed opportunity gap is `D_I^late(δ=0.2) − D_I^late(δ=0)`, and the *pairwise difference in repair contrast* is `[D_I^late − D_IS^late]_{δ=0.2} − [D_I^late − D_IS^late]_{δ=0}`. These are in-model intervention effects **of introducing a false-prior bias at genesis**, conditional on the specified coupling—not direct evidence of a causal effect in humans. Once initial choices differ, post-treatment stochastic histories and stream consumption may diverge. The paired seed is the independent inferential unit; **not** two paired runs, four forks, agents, or events.

### Quick smoke (in CI, tiny synthetic worlds)

```sh
cargo test --locked --test reputation_prior_pair
cargo run --locked --bin reputation-paired-prior -- run /tmp/paired-demo-002 72100 6 2 20 60
cargo run --locked --bin reputation-paired-prior -- verify /tmp/paired-demo-002
```

Use a *new* output folder each time. Replay rederivation is authoritative; the top-level cached JSON is not evidence without parent+fork journals.

### Optional exploratory N=20 matched-seed batch (manual only)

```sh
bash scripts/pilot-paired-002.sh /tmp/reputation-paired-v2-002
```

Four **predeclared synthetic parent seeds 92000–92003**, each evaluated at N=20,d=4, checkpoint τ=200, horizon T=600 with δ=0.2 and δ=0. This requires 8 parent trials and 32 fork trials; it is **intentionally excluded from ordinary GitHub PR CI**. The script refuses overwrites and failed/missing seed substitutions, verifies each trial, computes an explicitly descriptive batch summary, and records a SHA256SUMS evidence manifest.

**No 20-agent paired numerical results are claimed until every planned parent and fork verifies and the archive is sealed.** A four-pair batch, even if executed, would be underpowered for reliable inference. Increase the number of *independent parent seeds* and include more controls after benchmarking cost and specifying precision.

## Scientific and engineering gates

- [ ] Check the archived V0 compatibility fixture and ordinary V1 Hiyal replay on optimized code (CI).
- [ ] Benchmark before/after wall-clock and event-log bytes on identical hardware, model revision, seed and step schedule. **Do not assume speedup from fewer clones without measurement.**
- [ ] Run the paired V2 development design only if the engine is fast enough; record all failures.
- [ ] Larger matched control grid covering random churn, annealed networks and rewiring rate; beware graph-family confounding.
- [ ] External novelty review and human-data construct validation before interpretive sociology claims.
- [ ] Store original research journals durably and independently check their SHA-256 seals and source build provenance.

See [V1 validation](validation-v1.md), [formal model](formal-model.md), and [previous complete 5-parent evidence](results/v1-development-20261021/README.md). The previous 15-parent run did **not** complete.
