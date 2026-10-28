# Experiment 003 / R-06 — further H4 noninclusions from temporal stationarity and fine monotonicity

**Date:** 2026-10-27 UTC · **Normative frozen input:** A2 v0.2.2 `J/H4` and inherited six kernels from `docs/research/r06/a2-v021-six-kernel-design-20261025.md`. **Status:** author-derived mathematical *candidate* results, NOT independent P0 identification acceptance, a 95% inferential procedure, or human-behavior evidence. All six families, including boundary and zero-effect overlaps, retain their full legal nuisance domains.

This new work complements merged [PR #63](https://github.com/davletovb/social-emergence-lab/pull/63) (C outside the other five families at T≥2) and [PR #64](https://github.com/davletovb/social-emergence-lab/pull/64) (N/S/L/R outside C at T≥2). No review of these new directions by an external or source-isolated verifier has been performed.

## 1. Universal H4 invariant: N/S/M have constant fine-conditioned marginal cooperation

For actor 0 and period `t≥1`, define the **public** event *without conditioning on the current matching regime*

`A_t={f_{actor0,t}=0, a_{0,t}=1}`.

Its allocation base rate is `r_A=1/2` and `r_A0=1`. Both current matching regimes are integrated: M/N/S actor offsets do not inspect the newly assigned partner, and the dyad fines are independently randomized **after** matching. N's stable `ν_0`, S's stable `δτ_0`, and M's zero action offset are not updated by the public history. Therefore for every legal H4 population law in each of M/N/S, with arbitrary ≤4-atom correlations of η with actor action coefficients,

`P_m(A_1)=P_m(A_2)=r_d E_G[ε/2+(1−ε)σ(β(−1+H_0^m))]`,

where `H_0^M=0`, `H_0^N=ν_0`, and `H_0^S=δτ_0`. The same common scalar parameters and G are used at both periods, with **no** dataset-level random hyperprior. Latent posterior changes on *conditioned histories* do not invalidate this **unconditional** random-regime marginal identity.

Take the bounded, observable `h=1_{A_1}−1_{A_2}`, so `h∈[-1,1]`, and hence

`TV(P,P_m)≥|E_P h|/2=r_d |p_{P,t1}(0)−p_{P,t2}(0)|/2`

for every `m∈{M,N,S}`, where `p_{P,t}(0)` is the target marginal actor0 cooperation conditional on the actor's **current zero fine**, marginalizing both matching regimes. This is a **valid but possibly conservative** TV lower bound, not a test statistic with calibrated type-I error.

### Selected homogeneous R law is not representable by N or S (and therefore not M) for T≥2

Choose one-atom R with `β=1, ε=0, α_R=1, λ_A=0, η=0`, initial Q and aspiration 0. Let `q=σ(−1)`. For a zero-fine first dyad, the two simultaneous first-period actions `(a_0,a_{partner})` are independently Bernoulli(q). The R action-specific and fine-specific Q update at the end of T1 gives the period-2 **zero-fine** cooperation table:

| First-period `(a0,apartner)` | Probability, given first dyad fine zero | Changed Q | Actor0 p at T2 fine zero |
|---|---:|---|---|
| `(1,1)` | `q²` | `Q(1,0)=+1` | `σ(0)=1/2` |
| `(1,0)` | `q(1−q)` | `Q(1,0)=−1` | `σ(−2)` |
| `(0,1)` | `(1−q)q` | `Q(0,0)=+2` | `σ(−3)` |
| `(0,0)` | `(1−q)²` | `Q(0,0)=0` | `σ(−1)=q` |

Thus `p^{R}_{2|f_{1}=0,f_2=0}=q²/2+q(1−q)σ(−2)+(1−q)qσ(−3)+(1−q)²q=0.21266079407938066`. If the first fine was 1/2, the zero-fine Q context remains at zero, leaving period-2 p=q. Therefore

- A: `p^R_1(0)=q=0.2689414213699951`; `p^R_2(0)=(q+0.21266079407938066)/2=0.24080110772468788`; **TV versus any N/S/M H4 ≥ 0.007035078411326806**.
- A0: `p^R_1(0)=q`; `p^R_2(0)=0.21266079407938066`; **TV versus any N/S/M H4 ≥ 0.028140313645307224**.

The gaps are strict and involve **entire lawful public histories**, not a fixed-vector alternative-fit grid. The attached enumeration script independently sums *all 3 matching × fine × 16 first action cells*, updating all four actors' Q/aspirations; numerical agreement is an implementation check and does not replace the universal proof.

### Selected homogeneous L law is not representable by N or S for T≥2

Choose legal one-atom L: `β=1, ε=0, κ=3, α=1, ζ=0, η=0`, with initial belief `q_{i,1}=1/2`. At T1 with actor0 zero fine, `p^L_1(0)=σ(1/2)=0.6224593312018546`. At the end of T1 `q_{0,2}=a_{partner,1}` (actual partner in period 1). Under A, that previous partner's *marginal* T1 cooperation probability is `b_A=[σ(1/2)+σ(1)]/2=0.6767589549159297`; under A0 it is `b_{A0}=σ(1/2)`. Hence

`p^L_2(0)=b_d σ(2)+(1−b_d)σ(−1)`,

which evaluates to `0.6830202160953489` (A) and `0.6497966841742122` (A0). Both differ strictly from `p^L_1(0)`. The same time-invariant random-regime equality proves **L⊄N** and therefore **L⊄S** (since `S⊆N`) at T≥2:

- **A:** TV versus any N/S H4 ≥ **0.015140221223373607**.
- **A0:** TV versus any N/S H4 ≥ **0.01366867648617881**.

These are `r_d/2` times the marginal time difference, using the same bounded public indicator `h`; they are not likelihood-based composite-null tests.

## 2. Universal H4 invariant: initial R Q values impose a zero-fine ceiling

Every R legal truth starts with `Q_{i,1}(a,f)=0`, so its T1 zero-fine cooperation probability is **bounded** by `c*=1/4+(1/2)σ(−0.1)=0.48751040626053` for any actor latent tuple, shared β/ε or ≤4-atom H4 nuisance mixture. The selected homogeneous N truth `ν=1.5, β=1, ε=0, η=0` instead cooperates with `σ(1/2)=0.6224593312018546` under the same publicly assigned T1 zero-fine condition. With `D={z_1=random,f_{actor0,1}=0}`, `B=D∩{a_{0,1}=1}` and `g=1_B−c*1_D` of **oscillation 1**, the TV bound holds against every R H4:

`TV(P_N,P_R)≥r_d[σ(1/2)−c*]`.

Numerically **0.0674744624706623 (A)** and **0.1349489249413246 (A0)**. The same *entire full-history N law* embeds in S (`τ=+1,δ=1.5,ω=0`) and L (`κ=3,α=0`) by the already-defined legal H4 mappings, establishing **N⊄R, S⊄R and L⊄R**, already at **T≥1**, in both policies. These claims are *one-direction noninclusions*, not disjointness.

Together, the selected R time-instability witness and N T1 witness yield **candidate two-way N/R and S/R noninclusion at T≥2**, while `L⊄R` holds at T1 and above.

## 3. R breaks L's fine monotonicity under A, T≥2

For every legal L H4 candidate, conditional on *any fixed public past*, the current random t2 matching `μ2`, and a current actor0 fine `f`, the actor0 offset is `κ_0 q_{0,2}` **fixed before** current fines. Since β>0 and ε≤1/2, the cooperation response `ε/2+(1−ε)σ[β(f−1+κ_0 q_{0,2})]` is **monotonically increasing** in current f for each latent actor tuple. The fresh dyad fine is independent of latent parameters and public history after matching, so integrating over an arbitrary H4 posterior preserves this same inequality under **A**.

Choose the same homogeneous R truth as §1, and a fixed public prior event `H={t1 random, μ0=01|23, fines[0,0], actions[1,1,0,0]}`. The initial first-period action mass is `q²(1−q)²` and `P_R(H)=q²(1−q)²/24=0.0016106771788730377`. Actor0's Q(1,0) increases to 1 while Q(1,1) remains 0.

Under t2 random μ1, actor0 receives fine zero with probability 1/2 and fine half with probability 1/2. Its action is now:

- **At current fine 0:** `p_R=σ(−1+1)=0.5`.
- **At current fine 1/2:** `p_R=σ(−1/2+0)=0.3775406687981454`.

Let `B_0` and `B_{1/2}` denote disjoint public events fixing **H, actor0's current fine and actor0's current cooperation**, but *marginalizing both current matching regimes, all current matchings*, the other fine and the other actors' current actions. The fine is drawn independently after matching, and the L/R actor0 contribution offsets do not depend on its newly assigned partner. Every L H4 law has `P_L(B_0)≤P_L(B_{1/2})`, while this selected R has

`P_R(B_0)−P_R(B_{1/2}) = P_R(H) (1/2) [0.5−σ(−1/2)] >0`.

The difference of event indicators lies in [-1,1], yielding

`TV(P_R,P_L)≥(P_R(B_0)−P_R(B_{1/2}))/2 ≥ **0.000049310612526720535**`.

Thus candidate **R⊄L under intervention A for T≥2**. Combined with `L⊄R` above, this gives **L/R incomparability under A for T≥2**. The same argument cannot run under **A0**, where half fines are absent; **R⊄L under A0 remains UNKNOWN**. Do not transfer the A witness to A0 or infer a different free-H4 result merely because local fits failed.

## 4. Conservative relationship ledger (A and A0; T≥2 unless noted)

These are mathematical claims **conditional on the frozen synthetic J/H4 law**, awaiting source-isolated adversarial acceptance. Prior constructive embeddings retain their direction; unknown reverse directions are never filled by symmetry.

| Family pair | Constructive or candidate H4 law relation | Horizon/policy limitations |
|---|---|---|
| M/N | M⊆N; N⊄M (N T1 zero-fine witness) | Both, T≥1 |
| M/C | M⊆C; C⊄M (a legal C initial κ/2 offset can exceed the M zero-fine ceiling) | Both, **already T=1** |
| M/S | M⊆S; S⊄M (embedded positive-offset N truth) | Both, T≥1 |
| M/L | M⊆L; L⊄M (embedded positive-offset N truth) | Both, T≥1 |
| M/R | M⊆R; R⊄M (R violates stationarity) | Both, T≥2; equal at T1 |
| N/C | C⊄N (PR63), N⊄C (PR64) | Both, T≥2; equivalent at T1 |
| N/S | S⊆N; N⊄S **UNKNOWN** (reviewer's T1 cross-actor Hankel proposal is invalid under actor-iid H4; see §4A) | Both, all T |
| N/L | N⊆L; **L⊄N** (new temporal witness) | Both, T≥2; equivalent at T1 |
| N/R | **N⊄R** (T1 ceiling), **R⊄N** (new temporal witness) | Both, T≥2; at T1 R⊆N |
| C/S | C⊄S (PR63), S⊄C (PR64) | Both, T≥2 |
| C/L | C⊄L (PR63), L⊄C (PR64) | Both, T≥2 |
| C/R | C⊄R (PR63), R⊄C (PR64) | Both, T≥2 |
| S/L | S⊆L via N; **L⊄S** (new temporal witness) | Both, T≥2 |
| S/R | **S⊄R** (N in S witness), **R⊄S** (new temporal witness) | Both, T≥2 |
| L/R | L⊄R (N in L witness), **R⊄L** (new fine monotonicity) | **A T≥2** incomparable; **A0 R⊄L UNKNOWN** |

**Why a law-set hierarchy cannot be treated as exclusive family labels:** M belongs to every family; S⊆N⊆L. At nested/boundary truths the structural identified family set necessarily has multiple labels. Neither the apparent number of witnessed directions nor the smallest reported TV bound constitutes singleton recovery, type-I calibration, or human-valid causal identification.

**Open independent gates:** verify the probability event conditioning and selected counterexample numerics *without seeing these derivations*, re-check H4 actor-iid joint parameter mixtures and every full path law, check all 15 pairs separately, and design a valid unknown-mixture composite-null test with family-set simultaneous coverage. The initial published R-06 issue appears closed even though these scientific checkboxes remain open; administrative closure is **not** P0 success.

## 5. Reproducibility and limitations

The companion source-separated author-side stdlib `tools/exp003-exact/check_remaining_pair_invariants.py` enumerates the complete **T1 matching × fine × 16 simultaneous-action support** for selected R and L, recursively updating four actors and evaluating the period-2 fine-conditioned action marginal. Its unit tests pin the exact scalar witnesses and compare brute marginalization to separately expressed closed forms. No stochastic draws, model fitting, classifier training, human data, or new dependencies are involved.

A second independent *reviewer* must be recruited as a new isolated session with access only to the **two frozen normative model documents**, not this script, proof or GitHub review discussions, before any scientific P0 gate can be marked complete. The author-side calculation may supply verification targets **only after** the isolated proof is signed and frozen.
