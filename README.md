# Social Emergence Lab

Reproducible Rust agent-based research on social emergence, built as a **separate Hiyal consumer**. Research protocols, simulation models, experiments and evidence live here; generic engine infrastructure lives in [Hiyal](https://github.com/davletovb/hiyal).

## Scientific status

**Experiment 001: exploratory benchmark completed, not a new sociological discovery.** The linear reinforcement model is an established Pólya urn. A 10-paired-society × 18-configuration Hiyal exploratory grid has been executed with independent event-log and archive validation. All [summaries and trajectory outputs](experiments/001-emergence-of-inequality/results/replicated-74000/README.md) are retained in Git, while original raw evidence is temporarily stored in GitHub Actions. It is **not preregistered or evidence about human societies**.

- [Exact mathematical baseline](experiments/001-emergence-of-inequality/analytical-baseline.md) — Pólya urn and Dirichlet–multinomial moments.
- [Conservative paired-society precision planning](experiments/001-emergence-of-inequality/sample-size-and-registration.md) — primary pilot variance and candidate sample size.
- [Candidate confirmatory protocol](experiments/001-emergence-of-inequality/confirmatory-protocol-draft.md) — **not registered, not executed**.
- [Candidate Experiment 002: reputation correction vs opportunity-network repair](experiments/002-reputation-lock-in/research-question.md) — [targeted literature review](experiments/002-reputation-lock-in/literature-review.md), [formal model and paired 2×2 correction/repair design](experiments/002-reputation-lock-in/formal-model.md), [validation and novelty gates](experiments/002-reputation-lock-in/design-review-and-tests.md), [2026 scientific desk review](experiments/002-reputation-lock-in/scientific-review-2026.md) and [empirical-data feasibility](experiments/002-reputation-lock-in/empirical-alignment.md). **[Rust V0](experiments/002-reputation-lock-in/implementation-v0.md) passed its initial replay and negative-control tests. [V1 validation](experiments/002-reputation-lock-in/validation-v1.md) adds independent probability checks, network-mixing and turnover controls, and bounded graph benchmarks. All runs remain exploratory; no confirmatory or empirical sociological finding.**

## Run and verify

```sh
cargo test --locked
cargo run --locked --bin design -- experiments/001-emergence-of-inequality/results/replicated-74000/hiyal-trajectories.csv
cargo run --locked --bin hiyal-grid -- smoke /tmp/social-hiyal-smoke 1 62000
```

See the [research roadmap](docs/research-roadmap.md), [theoretical framework](docs/theoretical-framework.md), [methodology](docs/methodology.md), and for Experiment 001 the [plan](experiments/001-emergence-of-inequality/plan.md), [protocol checklist](experiments/001-emergence-of-inequality/protocol.md), [implementation](experiments/001-emergence-of-inequality/implementation.md), [independent validation and exploratory sensitivity](experiments/001-emergence-of-inequality/validation-and-sensitivity.md) and [Hiyal-backed study](experiments/001-emergence-of-inequality/hiyal-backed-study.md). Larger exploratory sweeps are opt-in to avoid recurring CI costs.

## Original evidence preservation

The entire original, SHA-256-sealed Hiyal archive is available from [GitHub Actions run 39553947265](https://github.com/davletovb/social-emergence-lab/actions/runs/39553947265) (artifact `12324369551`) until **January 19, 2027**; its `SHA256SUMS` is committed with the [derived results](experiments/001-emergence-of-inequality/results/replicated-74000/README.md), so a preserved copy can be checked against Git. The derived CSV files are not a substitute. See [archive issue #6](https://github.com/davletovb/social-emergence-lab/issues/6) for permanent preservation steps.

## Experiment 002 V2 research progress

[Measured V1/V2 performance and completed 36 matched parent-seed pairs](experiments/002-reputation-lock-in/results/v2-36-paired-20261021/README.md) are available with source commits, uncertainty estimates, per-seed outcomes and original sealed Hiyal artifacts. In-model, the late post-correction gap is carried by surviving checkpoint links, which the formal model's H1 classifies as slow relaxation rather than lock-in. This is **exploratory, synthetic model evidence**; no claim of new sociology or human-world generalizability.
