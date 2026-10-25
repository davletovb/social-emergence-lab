# R-06 — Experiment 003 (A2) working files

**Date:** 2026-10-25 · **Status:** AI-assisted desk check of the [v0.1 six-kernel specification](../r05/a2-six-generator-formal-spec-20261025.md), ahead of P0 in [Issue #50](https://github.com/davletovb/social-emergence-lab/issues/50). It is **not** the source-independent verification P0 requires, not external review, and it freezes or approves nothing. No simulation, CI, Hiyal code or human data.

## Files

- [`a2_kernel_desk_check.py`](a2_kernel_desk_check.py) — stdlib-only script written from the text of the v0.1 specification and the [information-repair note](../r05/a2-information-repair-identifiability-20261025.md). Exact rational arithmetic and exhaustive enumeration over short histories; no random numbers.
- [`a2-kernel-desk-check-output-20261025.txt`](a2-kernel-desk-check-output-20261025.txt) — recorded output at `0239ddc`.

Run from anywhere with `python3 docs/research/r06/a2_kernel_desk_check.py`. It infers the repository root and exits non-zero if any check fails.

## Result

27 checks: **0 FAIL**, 13 FINDING. The Codex review's terminal-withdrawal counterexample is now covered: an unfined final round leaves L and N observationally identical within the study; an unfined earlier round **may**, but need not, separate their next observed action. A FINDING is true as stated but flags a design issue for the v0.2 decisions discussed in the review comment on #50.

**Confirmed:**
- All 23 [oracle-fixture](../r05/a2-oracle-fixture-20261025.csv) rows and the 19 numeric [information-check](../r05/a2-information-checks-20261025.csv) rows reproduce exactly from the kernel text.
- C ≡ L under fixed matching (`α_L=1, ζ=0, q₁=m₁, κ_L=κ`) on every node of an exhaustive T=4 enumeration, for κ ∈ {½, 1, 3/2}.
- M ≡ N(ν=0) ≡ C(κ=0) ≡ S(δ=0) ≡ L(κ_L=0), as specification §3.1 states.

**Findings:**
- At f=0 the stage game has R=P=0 and is constant-sum, so it is not a prisoner's dilemma.
- N is nested in L at `α_L=0`, exactly, for every fine schedule; specification §3 does not list this.
- L with ζ=1 equals N on an always-fined schedule **or a schedule whose only unfined round is terminal**; some earlier unfined rounds can create divergent future choices, provided observation continues.
- Among the logit families, only S can cooperate below the material baseline.
- Argmax clearing gives the same matching for every ω>0. Under sum-of-scores clearing, preferences that weight partner cooperation equally cannot sort; a stable-matching rule sorts on them.
- At f=1 the R learner's reinforcement depends only on the partner's action.

The output ends with proposed exact multi-period fixtures for L, R and the Q clip. They are candidates for P0, not frozen references.

## 2026-10-25 — v0.2 DESIGN RECORDED; next gate is INDEPENDENT VERIFICATION

**Current v0.2 proposal**, superseding the v0.1 desk check as the design to verify:

- [Dated redesign choices](a2-v02-redesign-decisions-20261025.md) — explicit decision log for new PD payoff, reduced fine range, shared matching broker, contextual RL and shared noise.
- [v0.2 six-kernel/physical-world specification](a2-v02-six-kernel-design-20261025.md) — full equations, t=1 neutral initialization, one common action/noise law and stochastic mutual-acceptability matching accessible to every family.
- [v0.2 set-valued identification and primary estimand](a2-v02-set-identification-estimand-20261025.md) — true-family coverage gate, multiple admitted explanations, absolute adequacy/empty set, 15 pairwise full-joint-law checks, meaningful design comparison in place of impossible unconditional minimum singleton-rate.
- [Experiment 003 R-06 protocol DRAFT v0.2](../../../experiments/003-cooperation-mechanism-recovery/protocol-draft-v02-20261025.md) — the single current draft for future verification, **not frozen**.

**Historical v0.1 only:** `a2_kernel_desk_check.py`, its [dated output](a2-kernel-desk-check-output-20261025.txt), and the exact 23+21 earlier fixtures. The desk checker still checks old stage-game and old RL updates; **do not call its 27 checks v0.2 validation**. Re-derive affected payoffs, fine probabilities, contextual R/Q, full matching event distribution and thresholds independently during the next P0 stage. No v0.2 code, simulation, CI, budget or verification is being claimed by this update.

## 2026-10-25 — v0.2 design check (review artifact, not P0)

- [`a2_v02_design_check.py`](a2_v02_design_check.py) — stdlib-only check written from the text of the [v0.2 kernel design](a2-v02-six-kernel-design-20261025.md), backing the [second review comment on #50](https://github.com/davletovb/social-emergence-lab/issues/50#issuecomment-6395811896). It uses exact rational arithmetic on logit and valuation arguments, and floats only for broker probabilities. No random numbers or simulation. Where v0.2 is silent, the R check uses the placeholders A₁=0 and Qmax=10.
- [`a2-v02-design-check-output-20261025.txt`](a2-v02-design-check-output-20261025.txt) — recorded output at `582c497`.

Run with `python3 docs/research/r06/a2_v02_design_check.py`; it exits non-zero if any check fails.

**Result:** 19 checks: **0 FAIL**, 10 FINDING, 1 NOTE (a calibration-cost illustration).

**Confirmed:**
- At both primary fines the stage game is a prisoner's dilemma: T=2, R=1, P=0, S=−1 at f=0, and T=3/2, R=1, P=−½, S=−1 at f=½.
- The leak identity and the own C−D difference `f−1` hold in every cell.
- Every valuation is ½ at t=1, so the broker starts uniform.
- `q` stays in [0,1].
- C ≡ L under fixed matching for a common κ.
- R ≡ M for α_R=0 on every enumerated node.

**Findings:**
- The valuation reduces to `σ(γ·x_j·(⅓ + η_i + 1_S·ωτ_i))`, so γ is not separately identified.
- Common tastes now sort under the `min` broker, and with identical histories the broker sorts on latent η.
- S is exactly N with `ν_i = δτ_i` and `η_i^N = η_i + ωτ_i` whenever η may correlate with ν. In the random arm S always reduces to two-point N.
- With a common κ_L, every L offset shares one sign, so mixed-sign heterogeneity excludes L by domain.
- The C ≡ L fixed-match equivalence covers only homogeneous-κ C.

Like the v0.1 check, this is AI-assisted and **not** the source-independent P0 verification.

## 2026-10-25 — v0.2.1 second-review amendments (CURRENT DESIGN, NOT VERIFIED)

The [second AI-assisted review on Issue #50](https://github.com/davletovb/social-emergence-lab/issues/50#issuecomment-6395811896) and merged [PR #53](https://github.com/davletovb/social-emergence-lab/pull/53) (19 v0.2 checks, zero reported failures, ten design findings and one cost note) found an **S/N exact alias**, actor-level L asymmetry, unidentifiable matching scale, undefined sham, weak absolute-only family admission, and missing Q/aspiration constants. **PR #53 is internal review evidence only**, not a completed independent P0.

The **current source-of-design truth** for the next verification is:
- [v0.2.1 six-kernel/physical-world model](a2-v021-six-kernel-design-20261025.md): signed actor-level L learning, gamma normalized to 1, honest S/N observational equivalence under unrestricted correlated nuisance, shared matching and fine assignments, explicit R `A1=0` and `Qmax=10`.
- [v0.2.1 set-admission and estimand](a2-v021-set-estimand-20261025.md): A0 = all random pairings/no fine, A = randomized matching and dyadic fines, proposed absolute + max-rival calibrated two-gate admissibility, with empty/multiple sets and structural ambiguity decomposition.
- [six-point dated decision log](a2-v021-review-decisions-20261025.md) and [Experiment 003 protocol DRAFT v0.2.1](../../../experiments/003-cooperation-mechanism-recovery/protocol-draft-v021-20261025.md).

**Version rule:** v0.1 exact fixtures, [PR #51](https://github.com/davletovb/social-emergence-lab/pull/51) and v0.2 [PR #53](https://github.com/davletovb/social-emergence-lab/pull/53) desk checks retain **their original version-specific meanings**. **None** is a verification of v0.2.1, no P0 source-separate dual implementation exists, and no new stochastic simulation, CI or Hiyal implementation is approved. Original equation source, statistical method, parameter ranges and model robustness remain for independent verification.

## 2026-10-25 — v0.2.2 model-completion amendment after four first-pass verifiers

**CURRENT P0 MODEL CONTRACT** is now [v0.2.2 full joint action distribution and H4 population hierarchy](a2-v022-joint-law-hierarchy-20261025.md), inheriting the six unchanged v0.2.1 substantive action/matching families. The four user-submitted first-pass reports independently identified **missing conditional simultaneous-action coupling and unbounded/undefined random-effects model class**, so their version cannot be deemed fully verified. [Issue #50 reconciliation](https://github.com/davletovb/social-emergence-lab/issues/50#issuecomment-6401214504) tracks agreements and errors without rewriting the original reports.

- [v0.2.2 unfrozen Experiment 003 protocol draft](../../../experiments/003-cooperation-mechanism-recovery/protocol-draft-v022-20261025.md)
- [Version-matched neutral identical fixture inputs (JSON)](a2-v022-fixture-inputs-20261025.json)
- [Cross-language event schema, exact test matrix and acceptance](a2-v022-common-fixtures-20261025.md)
- [Fresh isolated prompts for mathematical, Python, Rust and statistical verifiers](a2-v022-independent-verifier-prompts-20261025.md)

**Named distinction:** **F** = fixed actor-vector deterministic conformance oracle only. **H4** = actor-iid **joint** parameter distributions with ≤4 atoms (within-actor correlation allowed), common family/global constants, independent societies at fixed hyperlaw; v0.2.2 primary joint actions use independent new uniforms conditional on complete latent state and current assignments. Domains are chosen as synthetic research-model parameters, not empirical estimates. Constructive S→N→L nesting preserved under actual hierarchy bounds, but **global 15-pair H4 identification and statistical coverage remain unknown**.

**First-pass outputs are not P0.** B's independently reported Python source/tests/JSON have **not been submitted as executable attachments** with the four reports, and C's Rust report source appendix has not been replayed against a common B input. Do not check P0 until a genuinely independent v0.2.2 reconciliation of common event probabilities, pairwise proof review and limits is complete. No Hiyal implementation, stochastic pilot, routine CI, human data or R-06 GO.
