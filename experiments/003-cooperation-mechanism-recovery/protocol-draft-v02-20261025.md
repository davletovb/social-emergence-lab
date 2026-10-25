# Experiment 003 — Cooperation Mechanism Identification and Model Recovery

> **Historical Experiment 003 v0.2 protocol — superseded by [current v0.2.1 protocol draft](protocol-draft-v021-20261025.md).** Research direction A2 unchanged; neither protocol is frozen or verified.

**R-06 protocol DRAFT v0.2 · 2026-10-25 · NOT FROZEN · NOT VERIFIED · NOT RUN**

**Owner-selected topic:** A2 ([R-05 decision](../../docs/research/r05/decision.md)) · **R-06 tracker:** [Issue #50](https://github.com/davletovb/social-emergence-lab/issues/50) · **Prior draft:** [v0.1](protocol-draft-20261025.md) (superseded for design; retained for research history).

**Authoritative v0.2 design attachments:** [kernel/physical world](../../docs/research/r06/a2-v02-six-kernel-design-20261025.md), [set identification/statistics](../../docs/research/r06/a2-v02-set-identification-estimand-20261025.md), [dated decisions and verification handoff](../../docs/research/r06/a2-v02-redesign-decisions-20261025.md). These documents define the **proposal**, not confirmed scientific truths.

## Research question and falsification

Can finite, experimentally feasible public records from common randomized fine and partner-choice assignments **reduce the number of generative families compatible with behavior**, without falsely excluding the data-generating mechanism or requiring latent simulator data? Which family pairs are **structurally indistinguishable** under each feasible operator, even with unlimited data?

This is **synthetic structural/partial model identification**, not evidence for which psychological causes real people follow. A result that all six remain compatible, or a predeclared intervention fails to reduce ambiguity, is a valid negative outcome. Do not relabel calibrated prediction as causal mechanism recovery.

## Shared experimental world (v0.2)

- **Four actors, two pairs per period**. One period uses truthful public history; t=1 starts with a stated **neutral prior 1/2**, not a fabricated previously realized action.
- **Same game across all six:** `u_i=2a_j-a_i-f(1-a_i)`; with f=0 the payoff matrix is `T=2,R=1,P=0,S=-1` (prisoner's dilemma). Contribution creates social surplus; unlike v0.1 it is no longer constant-sum.
- **Fine per pair after matching:** `f∈{0,1/2}` in primary arms. `f=1` is an explicitly marked optional **boundary diagnostic**; avoid the material indifference knife-edge as a primary contrast. Current action and exact payoff are public **after** simultaneous choices; no misleading hidden-partner-history arm.
- **Matching:** one public regime per society-period—uniform over the three perfect pairings or a **common stochastic mutual-acceptability broker** using cardinal preferences available to all. All six can prefer partners through the **same public-information/common nuisance layer**; the S family adds a tested hypothesis about alignment between stable behavior type and affinity. The classifier sees **realized pairs**, not latent preferences or submitted private scores.
- **Randomization/timing:** show publicly recorded prior actions (or neutral t1); assign regime; collect preferences/broker assigns dyads; announce pair-specific fine; simultaneous contributions; publish action+payoff; update learning. Separate RNG streams and timestamped observation scopes must be reproduced independently.
- **Same contribution noise/lapse:** `p=ε/2+(1-ε) sigmoid(β Δ)` with shared nuisance `β,ε` for all six. `Δ` includes the same material term `f-1` for every family.
- **Six restricted alternative generators:** **M** material only; **N** stable signed normative preference; **C** current partner's previous-action response; **S** fixed signed contribution type plus type-linked partner affinity; **L** fine-masked updating of observed partner cooperation; **R** fine-contextual action-value/aspiration reinforcement (four Q values for action × fine, plus A), with common initial fine responsiveness. Detailed equations and state updates in [kernel document](../../docs/research/r06/a2-v02-six-kernel-design-20261025.md).

**Known hazards:** M is nested within several families; N can embed in L when α_L=0 with compatible κ_L/q; signed N and correlated shared partner-appraisal nuisances can mimic S. **The model family labels are not assumed mutually exclusive or identifiable**, even with the revised matching broker. The entire **joint matching-and-action distribution**, not merely a logit kernel conditional on a chosen matching, is the target.

## Primary unit, model set and comparator

**One classified independent dataset D comprises K independent four-actor societies generated under one declared family-level process**, each with one realized T-period trajectory. Do not expose cloned counterfactual futures or post-treatment true generator IDs to the classifier. K and T and strata distributions **remain unchosen** until identification review. J independently replicated entire datasets are **a separate Monte Carlo precision cost** and not 4KT independent samples.

For every observed D, compute an **absolutely adequate, calibrated family confidence/admissibility set** `C_α(D)⊆{M,N,C,S,L,R}`; include all not rejected families under the same jointly observed event budget and balanced fitting/calibration. `C` may be **empty** (none of the six adequate), **singleton** (one admitted model, not proof of human motive) or **multi-family** (undetermined). No forced choice.

**Retired primary endpoint:** worst of six *unique-winner recovery rates*. Structural nesting makes this a poor objective; setting artificial model-strength minima must not redefine away honest overlap.

**Proposed replacement design estimand:** the **coverage-guarded reduction in the number of incorrect alternative family labels still admitted** relative to an equal-budget sham policy. On a synthetic generated dataset with known true label `m★` for **evaluation only**, define `B(D,m★)=|C_α(D) \{m★}|`. Contrast `E[B(sham,m★)] - E[B(intervention,m★)]`, report **each of six family-conditional effects** and an explicitly preweighted overall effect **only if truth-family retention/calibrated coverage passes** under each design. At true observational-equivalence sets, some other labels should remain admitted; the shrinking set is a descriptive ambiguity result, **not a proof of global cause**.

Mandatory companion outcomes: `1[m★∈C]` coverage and false exclusion; full `|C|` and explicit multimechanism/undetermined rates; correct and incorrect singleton **secondary**; in-family false-empty; correct empty-set detection on truly out-of-family generators; complete structural population identified-set map or `UNKNOWN`; predictive fit separate from mechanism claim; confidence intervals at the **independent dataset** level.

**Statistical method not yet verified:** calibrate per-family absolute joint goodness-of-fit and composite nuisance via held-out, possibly conservative parametric bootstrap; boundary/simultaneous calibration and model-class misspecification need independent statistical review. Nominal 95% family coverage is **a target, not an achieved result**. A smaller set from undercoverage is disqualified. Out-of-family cases need an absolute adequacy test; an always-nonempty relative ranking is prohibited.

## What scientific verification must establish next

1. **P0 kernels:** independent reconstruction of v0.2 PD payoff, shared schedule, stochastic matching probabilities/ties, all M/N/C/S/L/R state updates, public-only inputs and shared noise. Recompute new L/R/context/fine fixtures and their changed exact/numeric values; the v0.1 Python desk check **is not a v0.2 verifier**.
2. **Free-parameter 15-pair map:** under actual random/choice matching and fine arms, classify each unordered pair as constructively joint-law equivalent, rigorously separated, numeric/conditional, or unknown. **A finite grid is not a global proof**. Include nested boundaries, common nuisance correlations and model-misspecification adversaries.
3. **Calibrated set method:** prove/empirically establish relevant true-family coverage across proposed parameter strata (including boundaries), family-wise intervals, absolute-fit/empty sets, latent-feature exclusions and fairness under equal computing resources. Define an identifiability/classification unit precisely.
4. **Before budget:** specify K,T,J, family truth mixture, noise/nuisance ranges and hyperparameter hierarchy, intervention randomization probabilities, fit/test/calibration seeds, independent confirmatory holdout and expected cost—**after** P0 and pairwise identification map. Do not choose sample size based on an impossible singleton endpoint.
5. **Sources and external review:** source-review actual Galbiati game/fine assignment, Yang & Yue matching and Flache–Macy learner; a named independent methods review and explicit GO/REDESIGN/STOP. This artificial game is **not asserted to reproduce any particular human experimental payoff**.

**Hard STOP:** no unique label justified by access to private ranking scores or simulator truth, no nested zero cases deleted to inflate accuracy, no force-fitting out-of-family truths into one of six, and no post hoc tuning after viewing held-out confirmatory outcomes.

## Execution status and next permission boundary

**v0.2 design decisions are recorded**; **P0, 15-pair comparison, calibration, seed/budget freeze and R-06 GO remain uncompleted**. This document neither implements nor authorizes Hiyal changes, agent runs, Python stochastic scripts, CI workflow changes, participant data access, or pilot simulations. In the next separately authorized stage, independent verification may use source-independent **deterministic** reference implementations before any stochastic simulation or empirical claim.
