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
~~~

Replace the final word with no-false-prior, no-selection-bias, fixed-graph, or random-churn for distinct mechanistic controls. The output directory must be new and retains replayable parent and four fork journals plus report.json. A single exploratory parent seed cannot yield confidence intervals, statistical significance or replication claims.

For more informative *manual* exploratory pilots, explore N=6,d=2; N=10,d=3; N=20,d=4 with the declared 200/600-event schedule and independent exploratory parent seeds. Benchmark setup-sampler acceptance before using N=20,d=4. Keep broad grids out of automatic PR CI and publish all negative/null cases.

## Limitations and open gates

- Graph generator's 200,000-attempt ceiling can fail; failed seeds must be reported and not silently excluded.
- Need independent reference code, degree-only and annealed-mixing controls, link-turnover sensitivity and graph-family robustness.
- Need per-parent variance and covariance estimates before choosing confirmatory N/estimands; Experiment 001 sample-size planning does not transfer.
- Need full reproducibility provenance, signed raw-evidence archive, human researcher critique, stronger citation search and empirical construct validation.
- Simulations cannot establish a novel sociological law or external causal effect.

See [formal model](formal-model.md), [falsification gates](design-review-and-tests.md), and [scientific critique](scientific-review-2026.md).
