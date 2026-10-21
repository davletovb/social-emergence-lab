# Experiment 002: V0 synthetic reputation implementation

**Scientific status: exploratory software only. Not preregistered and not a claim about human societies.**

## Implemented model

- Identical latent performance quality 0.5, equal wealth 10, balanced randomized false reputational labels and private/public beliefs.
- Initial and independent repair graphs are sampled by uniformly permuting directed regular-graph stubs, rejecting loops/duplicate edges, with a hard setup retry limit. Sampling accepted configurations is exactly uniform over simple d-regular directed graphs, *conditional on acceptance*; acceptance rate and tail cost remain unbenchmarked at N=20,d=4.
- Hiyal schedules the opportunity allocator; each selects from its d directed links with a softmax-plus-uniform decision rule; recipient gains one token; an unbiased performance signal updates private/public beliefs; scheduled rewirings replace one directed edge using private removal beliefs and public addition reputation.
- All randomness uses trial-owned setup and acting-agent policy streams. Reducers and interventions are deterministic. Every action records the performance signal, link replacement, and exact pre-event signed expectation; conservation, eligible recipients and graph constraints are validated.
- Four intervention forks at one parent checkpoint: sham, truthful information, independent graph repair, and both. Repair forks use the same graph drawn independently at genesis. Hiyal original trial, ancestry, event replay and fork attribution are independently checked.
- Report records full-window and late-window signed group opportunity differences; conditional expected flow split into **continuously surviving** checkpoint graph exposure (removed-then-readded edges count as new), post-checkpoint graph exposure and belief-mediated selection; sampling noise and realized rewiring count. Independent inference units are parent societies, **not** actors or four forks.

## Bounded developer smoke

~~~sh
cargo test --locked --test reputation_v0
cargo run --locked --bin reputation-pilot -- /tmp/reputation-v0-62000 62000 6 2 20 60 default
cargo run --locked --bin reputation-pilot -- verify /tmp/reputation-v0-62000
~~~

Replace the final word with no-false-prior, no-selection-bias, fixed-graph, or random-churn for distinct mechanistic controls. The output directory must be new and retains replayable parent and four fork journals plus report.json; `verify` re-derives that report from the journals alone. `scripts/verify-002.sh` checks a whole batch written by `scripts/pilot-002.sh`. A single exploratory parent seed cannot yield confidence intervals, statistical significance or replication claims.

For more informative *manual* exploratory pilots, explore N=6,d=2; N=10,d=3; N=20,d=4 with the declared 200/600-event schedule and independent exploratory parent seeds. Benchmark setup-sampler acceptance before using N=20,d=4. Keep broad grids out of automatic PR CI and publish all negative/null cases.


## First fixed-seed development batch (21 October 2026)

The initial 12-parent development batch passed. Its [complete derived summary](results/dev-v0-20261021/batch-summary.json), [study plan](results/dev-v0-20261021/batch-plan.json) and [archive details](results/dev-v0-20261021/README.md) are retained in Git. The full parent/child Hiyal evidence bundle is a **temporary** GitHub Actions artifact; archive it permanently before expiry if needed for future study provenance.

| Development configuration | Independent parents | Information-only signed late opportunity gap | Truth + repair difference in late gap | Sham full gap | Repair-only full gap |
| --- | ---: | ---: | ---: | ---: | ---: |
| N=6,d=2, default reputation mechanism | 4 | +0.1000 | +0.3250 | +0.5125 | +0.3500 |
| N=10,d=3, default reputation mechanism | 4 | +0.0625 | +0.0125 | +0.5500 | +0.5750 |

The last two columns are H2's inputs (`abs(E[D_S]) < abs(E[D_U])`), from the per-arm [arm-summary.json](results/dev-v0-20261021/arm-summary.json).

The four negative-control configurations were each run for **one seed only**; they cannot estimate a null distribution or establish significance. With just 40 postintervention events per branch, large random signed gaps are possible even under null mechanisms. These numbers are descriptive within-model outputs and do not support a scientific discovery or causal claim about humans. They are not from the canonical 20-actor, 200/600-step design.

## Limitations and open gates

- Graph generator's 200,000-attempt ceiling can fail; failed seeds must be reported and not silently excluded.
- Need independent reference code, degree-only and annealed-mixing controls, link-turnover sensitivity and graph-family robustness.
- Need per-parent variance and covariance estimates before choosing confirmatory N/estimands; Experiment 001 sample-size planning does not transfer.
- Need full reproducibility provenance, signed raw-evidence archive, human researcher critique, stronger citation search and empirical construct validation.
- Simulations cannot establish a novel sociological law or external causal effect.

See [formal model](formal-model.md), [falsification gates](design-review-and-tests.md), and [scientific critique](scientific-review-2026.md).
