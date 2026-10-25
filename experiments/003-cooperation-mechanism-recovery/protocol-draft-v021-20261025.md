# Experiment 003 — Cooperation mechanism identification and model recovery

> **Historical v0.2.1 model draft — superseded for P0 conformance by [v0.2.2 protocol draft](protocol-draft-v022-20261025.md).** The newer document fixes the formerly unspecified action-vector coupling and actor-parameter hierarchy. This v0.2.1 file remains intact below as a versioned source; first-pass v0.2.1 test results do **not** establish v0.2.2 verification. Neither version approves simulations.


**R-06 PROTOCOL DRAFT v0.2.1 · 2026-10-25 · DESIGN ONLY, NOT FROZEN, NOT VERIFIED, NOT RUN**

A2 remains owner-selected ([R-05 decision](../../docs/research/r05/decision.md)). The [v0.2.1 complete generator specification](../../docs/research/r06/a2-v021-six-kernel-design-20261025.md) and [v0.2.1 set-inference criterion](../../docs/research/r06/a2-v021-set-estimand-20261025.md) are **the normative source for subsequent P0**. Earlier [v0.2 draft](protocol-draft-v02-20261025.md) and [v0.1 draft](protocol-draft-20261025.md) remain intact as historical hypotheses, not alternative current models. [New review on Issue #50](https://github.com/davletovb/social-emergence-lab/issues/50#issuecomment-6395811896), [PR #53 exact desk check](https://github.com/davletovb/social-emergence-lab/pull/53) are source for the *reasons to amend*, not independent validation.

## 1. Synthetic research question and claim ceiling

Given exactly the same public interventions, matching opportunities and observation budget for six predefined synthetic families, **which structurally distinguishable generator-family sets can be recovered without excluding the true family, and which remain indistinguishable even at infinite data?** Honest **non-identification**, missing-model/empty-set results and null intervention gains are valid outcomes. We make **no claim about mechanisms in human societies**.

## 2. Physical treatment and sham

Four actors with binary contributions in two dyads each round. Under all six `u_i=2a_j-a_i-f(1-a_i)`; payoff is a genuine PD at `f=0` and `f=1/2`, with actor's material contribution advantage `f-1`. At t=1 an openly stated **neutral prior 1/2** is available in lieu of a real past action; at t≥2 genuine actions and payoffs from previous period are public. Hidden partner-history despite transparent payoffs remains prohibited. All actors rank partners via a shared public history and private tastes; a common broker chooses actual pairs; **no private valuations go to primary classifier**. Dyadic fine announced after pairs assigned, before actions.

**Candidate intervention A**: each independent society-period chooses random or stochastic mutual-acceptability **choice** matching with equal probability, followed by independently randomized `f=0` or `1/2` **per matched dyad** with equal probability. **Candidate sham A0**: all periods uniformly random matching and `f=0` on each dyad. Same public histories, K independent four-actor societies, T periods, actors observed, total fit and calibration cap. Detailed physical rules and proposed RNG namespace are in [kernel spec](../../docs/research/r06/a2-v021-six-kernel-design-20261025.md); **K,T and random-seed domains are not frozen**.

## 3. Six mechanism kernels and unbiased matching access

All families share `p_i=eps/2+(1-eps)*sigmoid(beta*[f-1+H_i])`, same nuisance/noise family `beta,eps`, and the *same* physically possible cardinal preference submissions/matching broker. H:
- **M**: 0; pure material incentives.
- **N**: stable signed actor-specific `nu_i`.
- **C**: actor-specific signed `kappa_C,i` times matched partner's actual previous action (neutral 1/2 at t1).
- **S**: `delta*tau_i`, persistent binary `tau_i`; possible additional type-linked partner appraisal `omega*tau_i`.
- **L**: actor-specific signed `kappa_L,i*q_i,t`, with `q_i,1=1/2` and actor-specific `alpha_L,i,zeta_i`, learning from observed partners with fine-mask weights.
- **R**: context-dependent action values `Q_i(a,f)`, all initialized zero, aspiration `A_i,1=0`, actor-specific reinforcement/aspiration learning and provisional common clipping `Qmax=10`.

Fix matching valuation scale `gamma=1`; broker uses `W(mu)=sum min(v_i(j),v_j(i))` and softmax temperature `rho>0` across three complete pairings for choice, or uniform 1/3 for random. **All families** can have nonzero actor-specific partner appraisal nuisance `eta_i`; its joint distribution with latent contribution/learning parameters is **unrestricted in the primary scientific claim**, conditional on as-yet-to-be-fixed allowed domains. Raw preferences remain internal broker inputs, not classroom/lab observables for mechanism classifier.

**Known S/N equivalence**: take `nu_i=delta*tau_i`, `eta_i^N=eta_i^S+omega*tau_i`. Under permitted parameters/correlated tastes, the **joint match/action laws coincide**; S is not uniquely recoverable from N on that region. Restricting independence of eta and action traits would be a **separate scientific assumption and sensitivity design**, not the primary truth. Other nested corners (M within N/C/S/L/R; N within L when alpha=0 and magnitudes permit; C/L with fixed matching and aligned actor-specific kappas) remain reportable.

## 4. Meaningful primary set-based outcome

Define a finite-data admitted-family set `C_alpha(D)` for a **classified dataset D of K independent societies governed by one true generator family**; J independently generated datasets measure study-to-study uncertainty. The classifier cannot read truth labels or private states. The admitted set requires **both**:
1. non-rejection by an **absolute held-out joint goodness-of-fit gate**, and
2. non-rejection by a **family-wise max-over-rivals outperformance gate** with correct within-family simultaneous calibration.

**Empty**, **one**, **multiple**, or **all six** admitted families are legitimate outcomes. Target `Pr(true family∈C)≥0.95` *per family and declared strata* only if the proposed levels and worst-case composite-null calibration **actually establish it**; current draft merely proposes `alpha_abs=alpha_rel=0.025`. A nominal split does **not** prove finite-sample coverage. Do not artificially restrict parameters to force unique labels.

Primary **conditional design comparison** at fixed budget: `Delta_B=E[|C(D_A0)\{m_truth}|]-E[|C(D_A)\{m_truth}|]`, **only if both design policies meet the predeclared true-family coverage criterion**. The other retained labels can be genuinely structurally compatible, not “false mechanisms.” Show separate family/strata; a macro-average is secondary, with explicit weights.

When independent identification proofs are available, separate **irreducible structurally compatible alternatives** `I_A(P)\{m_truth}` from **extra finite-data ambiguity** `C\I`, and report **incorrect loss** of other structurally compatible mechanisms `I\C`. If structural I remains unknown, report **UNKNOWN** rather than a zero irreducible ambiguity. Report false true-family exclusions, in-family false-empty, out-of-family correct-empty, wrong singleton, undecided-set distribution and forecast accuracy separately. No relative-fit winner is a mechanism finding.

## 5. Verification and approval gates — all still OPEN

1. **Independent P0**: two genuinely source-separated implementations review/payoff and type/noise/valuation/time rules, use exact fixture and deterministic kernel checks, and independently derive the new R Q updates, the actor-specific L sign case, gamma normalization, S/N mapping, t1 and terminal learning, and stochastic matching/joint-action law. PR #53 is **internal author-side review only**, not independent verification of this version.
2. **All 15 family pairs and fully free rival parameters**: proof or certified-bounded disjointness/equivalence under A0/A, including possible exact S/N observational aliasing. Underived claims labeled UNKNOWN; finite grids are not global separation proofs.
3. **Set-calibration methodology**: review composite-model absolute tests, within-family max-rival significance, union-bound claims, boundary/rare strata, retained structurally equivalent families and empty-set OOD detection. Fix nuisance ranges and hypotheses *before* a trial.
4. **Only then budget**: choose K, T, J, actor-level heterogeneous parameter distributions, fair optimization/calibration budgets, treatment probabilities, untouched seed domains, and manual tiny pilot cost. Freeze dated R-06 protocol with independent methodological approval and explicit GO/REDESIGN/STOP.
5. **Only by separate owner authorization** may Hiyal implementation, stochastic pilot, CI or expensive sweeps start. No new human-data rights or empirical human interpretation authorized.

**Current disposition:** v0.2.1 **definition draft complete**, but P0, 15-pair map, 95%-coverage validation, experimental budget and R-06 freeze **NOT DONE**. No simulation or independent scientific verification was performed.
