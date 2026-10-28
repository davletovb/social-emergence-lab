# Experiment 003 — cooperation mechanism identification and model recovery

**Research type:** synthetic-model identification and structural non-identification, not a study of human cooperation. **Topic selected:** 2026-10-25 ([R-05 decision](../../docs/research/r05/decision.md)). **Current reference:** A2 **v0.2.2** full stochastic law/H4 hierarchy, inheriting unchanged v0.2.1 action, matching and updating equations. **Last status reconciliation:** [2026-10-28](../../docs/current-research-status.md).

**Implemented? Yes.** **Small exploratory pilot executed? Yes.** **Independently verified full H4 identification and 95% inference? No.** The closed historical R-06 [Issue #50](https://github.com/davletovb/social-emergence-lab/issues/50) does not replace the **open scientific acceptance tracker [#65](https://github.com/davletovb/social-emergence-lab/issues/65)**.

## What the experiment is asking

Can the same **four-actor, two-dyad repeated cooperation world** distinguish competing microscopic cooperation rules from a feasible **publicly observed** history of matching, fines, contributions, payoffs and time? Which structural model families overlap so that *no amount of the same type of observations* uniquely identifies them?

| Family | Description | Action offset (schematic) |
| --- | --- | --- |
| M | Material incentive only | 0 |
| N | Stable private normative preference | Actor's ν |
| C | Conditional cooperation | κ × **currently matched partner's** previous public action |
| S | Fixed-type sorting and cooperation | δ × actor's binary type τ |
| L | Fine-masked social-belief learning | κ × updated belief q |
| R | Contextual payoff reinforcement | Q(cooperate,fine) − Q(defect,fine) |

The full model includes the shared material payoff `2a_partner − a_self − fine(1−a_self)`, neutral first-period priors, stochastic broker, shared lapse/logistic response and exact update order. **Do not use this schematic table in place of the equations.** The authoritative definition is the combination of [J/H4 v0.2.2](../../docs/research/r06/a2-v022-joint-law-hierarchy-20261025.md) and [v0.2.1 six kernels](../../docs/research/r06/a2-v021-six-kernel-design-20261025.md).

**A:** randomly chosen matching regime (random vs broker choice) and independently randomized dyad fines (0 vs 1/2). **A0:** always random matching, no fines. The actors' latent tuples are **iid across actors** from the same deterministic family-specific ≤4-atom **joint** distribution G in a society; within-tuple parameter correlation is permitted. The same G and global parameters govern independently sampled societies in one classified dataset. Joint public histories, not individual actor-periods, are the data units.

## Current software and its limits

- [Rust six-mechanism implementation](../../src/experiment003.rs), [manual exploratory runner](../../src/bin/exp003-pilot.rs), shared [Cargo.toml](../../Cargo.toml) pin Hiyal at `050d1de094f5d0813817e2020ec5c5da97d5c42e`.
- [First bounded 24-society pilot](results/exploratory-pilot-20261026.md): executed 2026-10-26, 2 societies per family/policy, 4 actors × 4 periods, 384 actor-periods; setup/replay and public export checks succeeded. The A/A0 histories use **different seeds**, not paired counterfactual forks. This is an **engineering smoke diagnostic**, not a statistically powered mechanism-recovery study.
- [Finite-support benchmark](results/exact-support-benchmark-20261026.md): exhaustively enumerates a **single** A period's 384 categorical public outcomes (48 under A0) and selected conditional second-period laws. `f64` arithmetic, limited chosen H4 mixtures; **not** exhaustive over parameter domains.
- [Restricted free-rival feasibility](results/restricted-h4-free-rival-v02-20261027.md): analytic N-versus-M witness and **bounded** C-versus-L local/grid exploration. Inconclusive local C/L fitting is **not** a globally solved continuous optimization.
- [Author-derived conditional-invariant C witness](cl-random-broker-certificate-20261027.md), [reverse-C witnesses](reverse-C-H4-witnesses-20261027.md) and [remaining directed-pair invariants](remaining-h4-pair-invariants-20261027.md). Their mathematical arguments and executed regression checks support **candidate** H4 law-separation results but are **not independently signed P0 proofs**. The A-policy values were revised with explicit post-review provenance. N⊄S remains **UNKNOWN** because a proposed cross-actor moment/Hankel test incorrectly assumed shared latent propensity instead of actor-iid H4; R⊄L under **A0** also remains **UNKNOWN**.

**Model nesting is a first-class negative result:** M is contained in each of the other families; S⊆N⊆L. Thus a law can be compatible with more than one family. An honest recovery procedure must return **sets** (possibly multi-family or empty under misspecification), not force a winner for every dataset.

## Reading order and version preservation

1. Read the **current proposed normative definition** in [v0.2.2 J/H4](../../docs/research/r06/a2-v022-joint-law-hierarchy-20261025.md) and [v0.2.1 kernels](../../docs/research/r06/a2-v021-six-kernel-design-20261025.md). The [v0.2.2 protocol](protocol-draft-v022-20261025.md) is still an **unfrozen draft**.
2. Read the [experimental implementation record](exploratory-implementation-20261026.md) **with its new status update**: its original “no pilot launched” sentence was true when written and was later superseded by the executed pilot.
3. Read the [pilot](results/exploratory-pilot-20261026.md), [exact benchmark](results/exact-support-benchmark-20261026.md), [restricted v0.2 feasibility](results/restricted-h4-free-rival-v02-20261027.md) and v0.2 [mathematical witnesses](results/additional-h4-pair-invariants-v02-20261028.json). Each analysis has its own provenance, hypotheses and limitations. Results JSON files are **truth-labelled audit evidence**, not classifier training/validation input.
4. The [initial 2026-10-25 draft](protocol-draft-20261025.md), [v0.2](protocol-draft-v02-20261025.md) and [v0.2.1](protocol-draft-v021-20261025.md) are **historical design versions**, not coequal active alternatives. Keep earlier results and errata traceable, never silently replace their initial success criteria.
5. Check the [R-06 research index](../../docs/research/r06/README.md) and [Issue #65](https://github.com/davletovb/social-emergence-lab/issues/65) for all outstanding independent verification, inference and experiment-design gates.

## Reproduce selected engineering and deterministic checks

Rust 1.89+, a pinned Git Hiyal dependency and Python 3 are required for the respective commands.

```sh
cargo test --locked --lib experiment003
cargo test --locked --manifest-path tools/exp003-exact/Cargo.toml

python3 tools/exp003-exact/check_cl_h4_invariant.py
python3 tools/exp003-exact/check_reverse_c_invariants.py
python3 tools/exp003-exact/check_remaining_pair_invariants.py

# Manual engineering diagnostic, NOT a confirmatory run. New empty path only.
cargo run --locked --bin exp003-pilot -- /absolute/path/to/new-exp003-pilot
```

Use a physically separated ingest path for public-only exports; do **not** allow private journals, truth labels, hidden types or run-manifest identifiers to enter a family classifier. Do not assume unit tests are a second independent mathematical implementation: the Rust exact tool reuses the simulator, and Python checks are separately expressed **by the same research effort**.

## Scientific acceptance and evidence custody

The [24-society pilot](results/exploratory-pilot-20261026.md) generated original Hiyal event evidence in a **time-limited Actions artifact** (reported expiration 2026-11-25); it was retrieved and hash-checked but no durable independent institutional custody has been established. Exact-analysis Actions artifacts have separate expiry dates recorded in their reports. Archival manifests and a frozen versioned confirmatory procedure are not yet complete.

**Open under [Issue #65](https://github.com/davletovb/social-emergence-lab/issues/65):** source-isolated independent J/H4 model mathematics and common-fixture second implementation; accepted 15-pair directed inclusion map for A/A0 and horizons; nuisance-boundary, zero-effect and correlated-mixture tests; composite-null fitting and valid familywise/identified-set 95% calibration; out-of-family goodness-of-fit/empty-set protection; prospective seed and K/T/J budgets; external scientific or empirical validity review. **There is no approved confirmatory Experiment 003 GO, no accepted globally identifiable six-family classifier, and no finding about human behavior.**
