# R-06 Experiment 003 — v0.2 redesign decision record
**Dated 2026-10-25** · owner instruction: “Redesign v0.2: correct payoffs, partner-choice rules, reinforcement learning, and shared noise assumptions. Redefine the success criterion: preserve structurally indistinguishable mechanisms and report identifiable sets rather than force a unique winner.”  
**Disposition:** proposed **DESIGN CHOICES RECORDED**, **not a P0 verification, frozen R-06 protocol, cost approval or GO to run**.

## Decisions now taken for the v0.2 candidate

| Design issue found in R-06 review | v0.2 decision | Why / residual verification risk |
| --- | --- | --- |
| No joint surplus; v0.1 stage `R=P=0` | Use common `u_i=2a_j-a_i-f(1-a_i)`, genuine PD at `f=0` | Exact payoff/matching/human-paper comparator must be separately verified; old R fixtures DO NOT validate new payoff |
| Material-indifference fine at `f=1` | Primary levels `f={0,1/2}`; f=1 kept only as separately declared negative boundary diagnostic | Fine strength may be too small/large for meaningful effects; no pilot/calibration performed |
| Sorting generator alone submitted useful rankings | All six may rank using common public previous-action information, common baseline and actor-specific appraisal nuisance `η_i`; only S couples additional affinity to fixed behavioral type | New S/N and other overlap may persist with correlated nuisance; do not assert S identifiability |
| Homogeneous linear argmax cancels preferences; positive ω scale ineffective | One shared **stochastic mutual-acceptability matching** over three complete pairings, weights `exp(ρ Σ min(v_i(j),v_j(i)))`; random arm uniform over three | Not a stable-roommate equilibrium. Realistic ranking observability, model invariances and ω effects still require P0 |
| R learner lacks immediate fine response and has unique decision noise | All action kernels share **material difference `f-1`** and identical `β,ε` logit+lapse; R adds **contextual Q(a,f)** reinforcement to common material term, with payoff-driven aspiration; no separate `β_R` | May make R observationally nest M at α=0. Need genuine joint-likelihood verification at nontrivial parameters |
| N artificially only positive while S can be negative | Allow stable N and C preference/response offsets to be signed; explicitly retain nesting/aliasing | A label must not be separable because of a domain sign restriction |
| No valid t=1 past partner action | At t=1 announce a **neutral prior 1/2 explicitly not a realized action**; from t=2 publish actual last actions for all | Must confirm learning update/timing under chosen protocol |
| Fine/history timing ambiguous | Public history → matching regime → public valuations/broker pairing → **fine per matched dyad** → simultaneous action → public payoff → learning | Finite-cost matching and assignment independence still require review |
| Primary min-over-six singleton rate bounded by nesting | RETIRE as primary. Use **coverage-gated reduction in alternative-family ambiguity**, retain complete set distributions and structural ID sets | Both coverage and family-set adequacy need independent calibration. No claim 95% coverage yet |
| Relative best model always produces a “winner” | Absolute held-out goodness-of-fit gate, `C(D)` allowed to be **empty** | Must predeclare OOD generators, nuisance calibration and false-empty rates |
| Classification unit unclear | **One dataset = K independent four-actor societies under one true family**; J independent datasets for Monte Carlo precision | K,T,J not chosen or budgeted; full data independence/seed plan outstanding |
| Only corner-vs-corner contrasts available | Require all **15 family pairs** with *free rival parameters* and correct **joint match+action** laws, including constructive overlap and certified separation/UNKNOWN categories | Not yet computed; no assumption that a finite grid proves globally positive divergence |

**Authoritative v0.2 design documents:** [six kernel and world rules](a2-v02-six-kernel-design-20261025.md) and [set-valued identification/estimand](a2-v02-set-identification-estimand-20261025.md). The [Experiment 003 draft v0.2](../../../experiments/003-cooperation-mechanism-recovery/protocol-draft-v02-20261025.md) references them; v0.1 artifacts are preserved as **historical, not simultaneously valid**. [PR #51 exact desk checker](a2_kernel_desk_check.py) exercises **v0.1 ONLY**; no outputs are being rebranded as v0.2 checks.

## Decisions **not** made, on purpose

**No P0 verification now:** no independent code, re-derived payoff/R/choice reference fixtures, actual full 15-pair comparison, source-methods recheck or stochastic run. No claim that the new S coupling is distinguishable from a nuisance-correlated N/C/L model; no claim that all six families can ever be uniquely inferred. The selected result may be **non-identification**.

**No P1 statistical freeze now:** unknown `β, ε, γ, ρ, η` domains/priors and cross-family nuisance constraints; κ/ν/δ/R update ranges; separate family conditional truth distributions, near-overlap/OOD mix, practical margins, acceptance thresholds, bootstrap, absolute-fit statistic, calibration guarantees; K,T,J, intervention randomization probabilities, costs, effect sizes and untouched confirmatory seeds. Choose these only after source-independent P0 and pairwise identifiability checks and before authorized simulation. The target nominal 95% family-wise coverage is an **aspiration requiring verification**, not a statistical result.

**No execution authorization:** no Hiyal implementation, pilot, GitHub Actions/CI, automated sweep, human data handling, publishing a discovery, or declaring R-06 frozen. Topic selection A2 for Experiment 003 remains unchanged.

## Verification handoff (next user-approved stage)

1. Independent and source-separate verification of the **v0.2** common PD payoffs, time order, actual joint matching distribution, all six action and learning updates, identical noise/lapse and correct observation permissions. Regenerate all affected R/PD/f=1/2 fixtures; do **not** reuse v0.1 Q traces.
2. Full **15-pair, free-parameter** population identification map under random/choice matching and fine arms, with explicit structural equalities and well-founded positive-divergence bounds or `UNKNOWN`.
3. Evaluate the set-admission methodology under nesting, parameter boundaries, correlated matching tastes and out-of-family truth; document calibration limitations, then freeze K/T/J, fitting/test resources and independent seed domains.
4. Only after those gates pass: independent science/novelty check, written `GO / REDESIGN / STOP`, and separately requested tiny pilot approval.

**Decision rule:** do not change the model to make it easier to identify after seeing any verification or pilot outcome; record a versioned design change and re-verify, including negative results.
