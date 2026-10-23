# Experiment 002 — V2 performance and matched-prior control

**Status:** independent performance/code-validation milestone, not a preregistered human-science study.

## Performance changes

The original rule-based `HiyalWorld::observe` copied an entire `ReputationWorld`, including the complete and increasing `trace`, at *every* allocator opportunity. Hiyal's EffectPolicy **requires a policy observation**, so V2 instead constructs a compact `ReputationObservation` with only actor beliefs, public reputation, graph, boost labels, RNG-independent configuration, event step and link-survival state. It excludes the growing trace, wealth history and the unused repair graph. For the common persistent-network arm, `decide_with_streams` borrows that small view through `Cow`; the annealed-mixing arm retains an owned small view only when constructing the explicitly resampled graph. Both production and observation probability calculations call the **same ordered arithmetic helpers**. **These changes must not alter any policy RNG draws, committed actions, state hashes, or archived V0/V1 replay semantics.** A direct archived V0 fixture replay remains part of CI.

This is a targeted allocation optimization, **not a solution to the full replay cost**, and it gives **no measurable speedup**. On one N=20, d=4, 200/600 parent (seed 91000, release build, same machine, two rounds each), main took 20.1–21.5 s to run and about 8.4 s to verify; this branch took 22.4–22.8 s and 8.2–9.3 s, with byte-identical output. The same run on a RAM disk takes 17.3 s, so disk I/O is about a fifth of the time. A sampling profile puts most of the rest in Hiyal's per-event hashing (`storage::digest`: stable-JSON serialization and SHA-256). The growing `trace` is 97% of the final world's JSON, but per-event cost rises only from about 12.4 to 14.6 ms between runs whose mean step differs fourfold, so it explains perhaps 15% of the time; the rest is a fixed per-event cost in the engine. Real speedups would come from Hiyal's per-event hashing. Any change that strips or compresses recorded traces still needs a new, versioned experimental world schema and independent equivalence checks first. Do not silently alter V0/V1 hash identities. The opt-in [journal-trace world kind](journal-trace-kind.md) is that schema.

## Paired zero-false-prior comparison

Previous N=20, d=4 runs compared δ=0.2 in three societies with δ=0 in two *different* seeds. Those were unrelated samples; their mean difference could not isolate the false-prior intervention.

`reputation-paired-prior` now runs **two independently recorded Hiyal trials per seed**: δ=0.2 and δ=0, with **identical initial graph, independently pre-drawn repair graph, boost labels, and initial Hiyal named-stream positions**. The only changed parameter is δ; all other configuration properties must match. Each run includes all four pre/post correction network-repair arms. The CLI verifies each parent and fork (including journal replay and independent conditional-exposure calculations) and re-derives `paired-report.json` from trusted evidence, detecting substitution/tampering.

The **within-seed difference** in late information-only signed opportunity gap is `D_I^late(δ=0.2) − D_I^late(δ=0)`, and the *pairwise difference in repair contrast* is `[D_I^late − D_IS^late]_{δ=0.2} − [D_I^late − D_IS^late]_{δ=0}`. These are in-model intervention effects **of introducing a false-prior bias at genesis**, conditional on the specified coupling—not direct evidence of a causal effect in humans. Once initial choices differ, post-treatment stochastic histories and stream consumption may diverge. The paired seed is the independent inferential unit; **not** two paired runs, four forks, agents, or events.

**Which estimate to use.** With δ=0 nothing in the model reads the labels, so `E[D_I^late(δ=0)] = 0` exactly. The matched difference therefore estimates the same quantity as `D_I^late(δ=0.2)` alone, and pairing can only help by cancelling shared noise: it lowers the variance only if the two sides correlate above about `sd(δ=0)/(2·sd(δ=0.2))`. In a review check of 16 matched pairs (seeds 97000–97015, `reputation-paired-prior run <dir> <seed> 20 4 200 600`, regenerable, not archived), the late information-only gap averaged +0.168 (sd 0.138) at δ=0.2 and −0.019 (sd 0.104) at δ=0, with correlation **0.16**, below the break-even of about 0.38; the matched difference (sd 0.159) was about a third noisier than the δ=0.2 gap alone. The four predeclared pairs below (seeds 92000–92003) were also run once, while testing the script end to end, and were not archived; over all 20 pairs the correlation is 0.27 against a break-even of 0.44, and the matched difference (sd 0.147) is still noisier than the δ=0.2 gap (sd 0.128). Because runs are deterministic, an official run of those four seeds will reproduce the same, already-seen values. So use `prior_information_late_gap` against the exact zero as the primary estimate, and treat the δ=0 runs as a symmetry check (their mean should be near zero) or as a control variate with an estimated coefficient. `paired-summary.json` reports both sides' means and standard deviations and their correlation.

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

Four **predeclared synthetic parent seeds 92000–92003**, each evaluated at N=20,d=4, checkpoint τ=200, horizon T=600 with δ=0.2 and δ=0. This requires 8 parent trials and 32 fork trials, about 5 minutes in the release build the script uses; it is **intentionally excluded from ordinary GitHub PR CI**. The script refuses overwrites and failed/missing seed substitutions, re-derives each pair's report from its journals, computes an explicitly descriptive batch summary (`scripts/paired-summary-002.jq`), records the source and Hiyal revisions, seals a SHA256SUMS evidence manifest, and finally runs `scripts/verify-002.sh` on the sealed batch. A preserved copy is checked the same way: `bash scripts/verify-002.sh <dir> <SHA256SUMS digest>`.

**No 20-agent paired numerical results are claimed until every planned parent and fork verifies and the archive is sealed.** A four-pair batch, even if executed, would be underpowered for reliable inference. Increase the number of *independent parent seeds* and include more controls after benchmarking cost and specifying precision.

## Scientific and engineering gates

- [x] Check the archived V0 compatibility fixture and ordinary V1 Hiyal replay on optimized code: the V0 and V1 archives pass `verify-002.sh` with their recorded digests, and the five V1 runs regenerate byte-identically.
- [x] Benchmark before/after wall-clock on identical hardware, revision, seed and schedule: no speedup (see above).
- [ ] Run the paired V2 development design only if the engine is fast enough; record all failures.
- [ ] Larger matched control grid covering random churn, annealed networks and rewiring rate; beware graph-family confounding.
- [ ] External novelty review and human-data construct validation before interpretive sociology claims.
- [ ] Store original research journals durably and independently check their SHA-256 seals and source build provenance.

See [V1 validation](validation-v1.md), [formal model](formal-model.md), and [previous complete 5-parent evidence](results/v1-development-20261021/README.md). The previous 15-parent run did **not** complete.
