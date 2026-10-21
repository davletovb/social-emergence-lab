# Social Emergence Lab

Reproducible Rust agent-based research on social emergence, built as a **separate Hiyal consumer**. Research protocols, simulation models, experiments and evidence live here; generic engine infrastructure lives in [Hiyal](https://github.com/davletovb/hiyal).

## Scientific status

**Experiment 001: exploratory benchmark completed, not a new sociological discovery.** The linear reinforcement model is an established Pólya urn. A 10-paired-society × 18-configuration Hiyal exploratory grid has been executed with independent event-log and archive validation. All [summaries and trajectory outputs](experiments/001-emergence-of-inequality/results/replicated-74000/README.md) are retained in Git, while original raw evidence is temporarily stored in GitHub Actions. It is **not preregistered or evidence about human societies**.

- [Exact mathematical baseline](experiments/001-emergence-of-inequality/analytical-baseline.md) — Pólya urn and Dirichlet–multinomial moments.
- [Conservative paired-society precision planning](experiments/001-emergence-of-inequality/sample-size-and-registration.md) — primary pilot variance and candidate sample size.
- [Candidate confirmatory protocol](experiments/001-emergence-of-inequality/confirmatory-protocol-draft.md) — **not registered, not executed**.
- [Candidate Experiment 002: reputation correction vs opportunity-network repair](experiments/002-reputation-lock-in/research-question.md) — [targeted literature review](experiments/002-reputation-lock-in/literature-review.md), [formal model and paired 2×2 correction/repair design](experiments/002-reputation-lock-in/formal-model.md), [validation and novelty gates](experiments/002-reputation-lock-in/design-review-and-tests.md), [2026 scientific desk review](experiments/002-reputation-lock-in/scientific-review-2026.md) and [empirical-data feasibility](experiments/002-reputation-lock-in/empirical-alignment.md). **Conceptual work only; no Experiment 002 simulations or confirmatory claim.**

## Run and verify

```sh
cargo test --locked
cargo run --locked --bin design -- experiments/001-emergence-of-inequality/results/replicated-74000/hiyal-trajectories.csv
cargo run --locked --bin hiyal-grid -- smoke /tmp/social-hiyal-smoke 1 62000
```

See the [research roadmap](docs/research-roadmap.md), [theoretical framework](docs/theoretical-framework.md), [methodology](docs/methodology.md), and for Experiment 001 the [plan](experiments/001-emergence-of-inequality/plan.md), [protocol checklist](experiments/001-emergence-of-inequality/protocol.md), [implementation](experiments/001-emergence-of-inequality/implementation.md), [independent validation and exploratory sensitivity](experiments/001-emergence-of-inequality/validation-and-sensitivity.md) and [Hiyal-backed study](experiments/001-emergence-of-inequality/hiyal-backed-study.md). Larger exploratory sweeps are opt-in to avoid recurring CI costs.

## Original evidence preservation

The entire original Hiyal artifact is available from [GitHub Actions run 39548747760](https://github.com/davletovb/social-emergence-lab/actions/runs/39548747760) until **November 20, 2026**. The derived CSV files are not a substitute. See [archive issue #6](https://github.com/davletovb/social-emergence-lab/issues/6) for permanent preservation steps.
