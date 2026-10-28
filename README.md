# Social Emergence Lab

**Reproducible experiments in computational social science.** We build explicit agent-based worlds, test competing explanations for emergent social patterns, and investigate what simulated observations *can and cannot identify*. Research is deliberately broader than any one mechanism: inequality, reputation, cooperation, institutional dynamics and future simulation-based explorations all fit here.

The project is a **separate Rust research consumer of [Hiyal](https://github.com/davletovb/hiyal)**. Hiyal supplies generic event-sourced simulation and replay infrastructure; Social Emergence Lab owns the scientific models, interventions, protocols, exact analyses, results and interpretation.

**Current as of 28 October 2026:** [Research and implementation status](docs/current-research-status.md) · [Roadmap](docs/research-roadmap.md) · [Scientific methods](docs/methodology.md)

## Research portfolio

| Experiment | Question | Implemented / executed | Current scientific interpretation |
| --- | --- | --- | --- |
| [001 — Emergence of inequality](experiments/001-emergence-of-inequality/plan.md) | How does preferential allocation amplify differences? | Mathematical Pólya-urn baseline; Hiyal exploratory 18-configuration, 10-paired-society grid with replay-checked results | An established reference mechanism, **not** a new sociological discovery |
| [002 — Reputation and structural memory](experiments/002-reputation-lock-in/research-question.md) | Does inaccurate reputation have lasting effects after beliefs are corrected, or do inherited opportunities decay? | Rust V0–V3; 36 paired-parent V2; 432-parent, nine-condition V3; sealed evidence and independent replay | Studied outcomes favor **slow relaxation of inherited network structure**, not demonstrable continuing post-correction active lock-in |
| [003 — Cooperation mechanism identification](experiments/003-cooperation-mechanism-recovery/README.md) | Which different cooperation mechanisms are distinguishable from the same public histories and interventions? | Six-family A2 v0.2.2 Rust model; 24-society exploratory pilot; exact categorical tools and author-side H4 mathematical witnesses | Several **candidate structural noninclusions and unavoidable overlaps**; **independent mathematical verification and calibrated inference remain open** |

All three are **synthetic-model research**. None is a human-subject experiment, a preregistered confirmatory finding about real society, or an independently certified new sociological law. We publish informative **negative and non-identification findings**, not just apparent effects.

## Selected findings and boundaries

**001 — a known benchmark, used as a calibration exercise.** The [analytical baseline](experiments/001-emergence-of-inequality/analytical-baseline.md) is the classical Pólya urn / Dirichlet–multinomial result. [Exploratory Hiyal results](experiments/001-emergence-of-inequality/results/replicated-74000/README.md) check model behavior and reproducibility, but the [confirmatory protocol](experiments/001-emergence-of-inequality/confirmatory-protocol-draft.md) is not registered or executed.

**002 — structural inheritance can imitate persistent reputation effects.** In [36 paired V2 societies](experiments/002-reputation-lock-in/results/v2-36-paired-20261021/README.md), the late signed access gap was **+0.2183**, largely associated with surviving checkpoint links; the new-link plus selection component was **+0.0016** (exploratory 95% interval **−0.0214 to +0.0246**). [Nine-condition V3](experiments/002-reputation-lock-in/results/v3-stage-b-20261022/README.md) found **no cell satisfying its active-regeneration rule**. Both stages are exploratory; Stage B's criterion was adopted after Stage A. Read the [critical reanalysis](docs/review/experiment-002-v3-critical-reanalysis-20261022.md) before drawing a causal or novelty conclusion.

**003 — the goal is *set-valued* recovery, not forced classification.** The selected [A2 model](docs/research/r06/a2-v022-joint-law-hierarchy-20261025.md) contains six synthetic families: **M** material fines, **N** stable preferences, **C** conditional cooperation, **S** type-linked matching, **L** social-belief learning, and **R** contextual reinforcement. Under the specified full-H4 population class, **every M law embeds in the other families, and S ⊆ N ⊆ L**. Therefore one observed law may legitimately admit multiple explanations. [Our authored mathematical studies](experiments/003-cooperation-mechanism-recovery/README.md) derive additional candidate noninclusion witnesses using random rematching, temporal behavior and fine response. An independently accepted full 15-pair map and statistically valid 95% family-set coverage **do not yet exist**. For example, **N ⊄ S** and **R ⊄ L under A0** remain unresolved.

## What is in the repository?

- **Research models and CLI runners:** [`src/`](src/) and [`tests/`](tests/). Experiment 003 is implemented in [`src/experiment003.rs`](src/experiment003.rs) with a [manual pilot runner](src/bin/exp003-pilot.rs).
- **Experiment protocols, data and results:** [`experiments/`](experiments/). Keep a dated draft or earlier result in its original historical context rather than retroactively treating it as a preregistered claim.
- **Methodology, theory, review and decision records:** [`docs/`](docs/). The [current status index](docs/current-research-status.md) points to the controlling versions and open gates; the [R-05 topic-selection decision](docs/research/r05/decision.md) explicitly records incomplete screening prerequisites.
- **Opt-in exact-law tools:** [`tools/exp003-exact/`](tools/exp003-exact/) contains a separately locked Rust categorical benchmark and standalone stdlib Python checks. These are useful regression and author-side mathematics checks, **not a blinded independent scientific verifier**.

## Run locally

Requires **Rust 1.89+** and a Rust toolchain with network access to obtain the pinned Hiyal Git dependency. Normal Rust tests and explicitly bounded benchmarks use CPU only; they require no LLM API keys or hosted model calls.

```sh
# Repository tests
cargo test --locked

# Focused Experiment 003 model tests
cargo test --locked --lib experiment003

# Opt-in exact categorical tests (independent Cargo.lock)
cargo test --locked --manifest-path tools/exp003-exact/Cargo.toml

# Author-side mathematical cross-checks; no random simulation
python3 tools/exp003-exact/check_remaining_pair_invariants.py
python3 tools/exp003-exact/check_reverse_c_invariants.py

# Manual bounded 24-society engineering pilot: writes a NEW output directory
cargo run --locked --bin exp003-pilot -- /absolute/path/to/new-exp003-pilot
```

**Do not treat the pilot's audit/truth manifests or private Hiyal journals as classifier features.** Only the separately exported public records (period, regime, matching, dyad fines, actions and payoffs) belong in the proposed recovery workflow. Pilot seeds/results are developmental, not a predeclared independent held-out evaluation. Further simulation and heavy CI remain opt-in.

## Reproducibility, evidence and scientific gates

Original data and derivatives have different custody requirements. The [Experiment 001 sealed archive](experiments/001-emergence-of-inequality/results/replicated-74000/README.md) is indexed by hashes and time-limited Actions storage ([permanent archive issue #6](https://github.com/davletovb/social-emergence-lab/issues/6)). Experiment 002 V0 and V3 **original raw artifacts** were replay-verified and published as [GitHub Releases](docs/experiment-002-evidence-custody-20261022.md); V1/V2 permanence is still tracked separately ([#14](https://github.com/davletovb/social-emergence-lab/issues/14), [#17](https://github.com/davletovb/social-emergence-lab/issues/17)). The Experiment 003 [bounded pilot evidence](experiments/003-cooperation-mechanism-recovery/results/exploratory-pilot-20261026.md) and selected exact-check artifacts have **time-limited GitHub Actions retention**, so long-term off-platform preservation is not established.

The Experiment 003 scientific acceptance tracker is **[Issue #65](https://github.com/davletovb/social-emergence-lab/issues/65)**. The historical [R-06 Issue #50](https://github.com/davletovb/social-emergence-lab/issues/50) was administratively closed, **not** scientifically approved. Outstanding work includes a source-isolated independent mathematical reviewer, a separate common-fixture implementation, the accepted full-H4 directed 15-pair map, properly calibrated composite-null/set-valued admission (including empty sets for out-of-family truths), and a frozen confirmatory design. See [current status](docs/current-research-status.md), [methodology](docs/methodology.md) and [theoretical framework](docs/theoretical-framework.md).

## Licence

Copyright © 2026 Behruz Davletov.

- **Code: [MIT](LICENSE).** This covers source, tests, scripts, build and CI configuration, including code inside documentation/research directories and the separate `tools/exp003-exact` crate.
- **Documentation, protocols, results and data: [CC BY 4.0](LICENSE-CC-BY-4.0.txt).** This includes the repository-owned Experiment 002 raw-evidence release assets. Credit “Behruz Davletov, Social Emergence Lab” and link to the repository.
- **Third-party material retains its own rights.** [Hiyal](https://github.com/davletovb/hiyal) is a separate MIT-licensed project.
