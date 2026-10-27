# Experiment 003 / R-06 — adversarial reverse-C identification audit

**Date:** 2026-10-27 UTC  
**Normative model inputs:** [v0.2.2 J and H4 law](../../docs/research/r06/a2-v022-joint-law-hierarchy-20261025.md), inherited [v0.2.1 six action/update kernels](../../docs/research/r06/a2-v021-six-kernel-design-20261025.md).  
**Relationship to prior work:** builds on the merged [PR #63](https://github.com/davletovb/social-emergence-lab/pull/63), which proposed the *opposite* C-not-subset directions using fresh random matching.

**Provenance and independence:** This is **author-side, adversarial rederivation in the same research conversation**. Although the formulas are expressed without importing the Rust simulator in [the stdlib Python checker](../../tools/exp003-exact/check_reverse_c_invariants.py), this is **NOT** a source-isolated independent mathematician or a separately authored P0 submission. No independent scientific gate is complete. All findings are theorems *conditional on the proposed synthetic v0.2.2 law*, not empirical sociological findings.

## 1. Universal observable C inequality

At any period `t≥2`, write `j=j(μ_t,0)` for actor 0's **newly assigned partner**. Define the publicly visible event

`D_t(G) = G(H_{t-1}) ∩ {z_t=random, f_{0,t}=0, a_{j,t-1}=0}`,

where `G` is any chosen restriction on the full public history through `t−1`; `f_{0,t}` refers to the fine on actor **0's dyad**, not the fine on an arbitrary actor index. Let `B_t(G)=D_t(G) ∩ {a_{0,t}=1}`.

Under **every legal C H4 rival**, the current partner's previous public contribution is zero on `D_t`. Hence **regardless of** the actor-iid H4 joint atom distribution, all signs/correlations of `κ_C,η`, previous matched partners, posterior selection and the choice-broker nuisance,

`p_C(a_{0,t}=1|X,H,z_t=random,μ_t,f_{0,t}=0,D_t)=ε/2+(1−ε)σ(−β)`.

The v0.2.2 admissible domain `β∈[0.1,10]`, `ε∈[0,0.5]` gives the **universal and attainable** ceiling

```text
c* = max_{β,ε} [ε/2+(1−ε)σ(−β)]
   = 1/4 + (1/2)σ(−0.1)
   = 0.48751040626053 < 1/2.
```

It follows that `P_C(B_t)≤c* P_C(D_t)`. The inequality is valid even for alternative C parameters with **four correlated joint atoms**, arbitrary legal κ signs, and nuisance-dependent posterior histories, because it holds pointwise before mixing. If `P_C(D_t)=0`, the inequality still holds.

**Certified population-law TV dual:** Let `g=1_{B_t}−c*1_{D_t}`. Its only possible values are `−c*,0,1−c*`, so its oscillation is **exactly 1**. For any law `P` and any legal C rival `Q`,

```text
TV(P,Q) ≥ E_P[g] − E_Q[g] ≥ P(D_t) [P(a_{0,t}=1|D_t)−c*],
```

provided the target conditional probability exists and exceeds c*. (Shift `g` by c* to obtain a function in [0,1] and apply the definition of total variation.) **This is a deterministic mathematical inequality about complete joint observable-history laws, not a fitted likelihood, finite-sample CI or event-count power estimate.**

## 2. N ⊄ C, S ⊄ C and L ⊄ C already at T≥2

Choose the legal homogeneous one-atom **N** truth:

`β=1, ε=0, ρ=1, η=0, ν=3/2`.

At zero fine, actor 0's cooperation probability is constant over periods and previous partner actions:

`p_N=σ(−1+3/2)=σ(1/2)=0.6224593312018546>c*`.

Choose `t=2`, `G` unrestricted, and `D_2={z_2=random, f_{0,2}=0, a_{partner(μ_2,0),1}=0}`. At t1 the neutral-prior broker is uniform; homogeneous N actor contributions have probability `p_0=σ(1/2)` if unfined and `p_{1/2}=σ(1)=0.7310585786300049` under a half-fine. Independently sampled random t2 matching makes the currently matched partner a uniformly drawn member of the other three, so the probability of the previous partner action being zero equals the marginal probability of any given actor's t1 defection.

Thus `P_N(D_2)` equals:

```text
A:  (1/2 random regime)*(1/2 actor0 fine zero)
    * [1 − (σ(1/2)+σ(1))/2]
  = 0.08081026127101756.

A0: 1 * 1 * [1−σ(1/2)]
   = 0.3775406687981454.
```

The universal C-dual gives:

| Policy | P_N(D₂) | TV(selected N law, **any** C H4) ≥ |
|---|---:|---:|
| A | 0.08081026127101756 | **0.010905257882751379** |
| A0 | 0.3775406687981454 | **0.050948707375938415** |

This proves the one-truth witness `𝓟_N^{d,T,H4} ⊄ 𝓟_C^{d,T,H4}` for both policies at **every T≥2**, subject to adversarial independent review of the specified synthetic model.

**Two additional directions follow constructively from this SAME truth:** Define one-atom **S** with `τ=+1, δ=3/2, ω=0, η=0`, and one-atom **L** with `κ_L=3, α_L=0, ζ=0, η=0`, preserving β/ε/ρ. All are within the v0.2.2 domains. S contributes with `δτ=1.5`; L has `q_t=1/2` for all t and hence `κq=1.5`; their broker appraisal also equals N's. These are *exact full-history law representations* of the N truth, not approximate witness fits. Therefore this single construction also proves **S⊄C** and **L⊄C** at T≥2.

Combine, but do not conflate, this with merged PR #63's opposite one-direction **C⊄N/S/L** witness to propose **pairwise incomparability** of C with N, S and L for T≥2. Both sides remain **author-derived**, not independently accepted. The T=1 C=N=L family-law collapse is explicitly preserved.

## 3. R ⊄ C at T≥3 through a positive-probability public path

No unknown-parameter optimization is needed. Select a homogeneous one-atom **R** truth with `β=1, ε=0, ρ=1, η=0, α_R=1, λ_A=0`, legal aspiration `A_1=0`, context Q entries initialized to zero and `Qmax=10`.

Define the observable prehistory:

| Period | Regime | Matching | Dyad fines | Public actions |
|---|---|---|---|---|
| t1 | Random | `μ0=01|23` | `[0,0]` | `[1,1,0,0]` |
| t2 | Random | `μ1=02|13` | `[0,0]` | `[1,0,1,0]` |

Actor 0 cooperates with a cooperator in **both** periods at zero fine, receiving payoff `u_0=1` each time. The R transition updates its `Q_0(1,0)` **after** each period by `α_R(u_0−A_0)`, with aspiration fixed at 0 because `λ_A=0`: `Q_1(1,0)=0 → Q_2(1,0)=1 → Q_3(1,0)=2`; `Q_3(0,0)=0`. Neither update is clipped.

At t3 let `D_3` add `z_3=random, μ_3=μ0`, and actor 0's current dyad fine zero. Now actor 0's current partner is actor 1, who defected at t2 (`a_{1,2}=0`). Under **every C H4** law, conditional current cooperation is ≤c*. Under the selected R law, actor 0's zero-fine cooperation is

`p_R=σ(−1+[Q_3(1,0)−Q_3(0,0)])=σ(1)=0.7310585786300049>c*`.

All specified histories are **strictly positive-probability events** in the R truth because the action logits are finite and all specified regime/matching/fine draws have positive assignment probability. The exact masses require only small products:

```text
q = σ(−1)
t1 actions 1100 mass = q²(1−q)²
t2 actions 1010 conditional on t1 = (1/2)*(1/2)*q*(1−q)

Under A: t1 & t2 allocations each (1/2)*(1/3)*(1/4)=1/24;
         t3 allocation (1/2)*(1/3)*(1/2)=1/12.
Under A0: t1 & t2 allocations each 1/3; t3 allocation 1/3.

P_R(D₃) = [first assignment]^2 * [third assignment]
          * q²(1−q)² * (1/4)q(1−q).
```

| Policy | P_R(D₃) | TV(selected R law, **any** C H4) ≥ |
|---|---:|---:|
| A | 2.748944044845174e-7 | **6.69500298067994e-8** |
| A0 | 7.037296754803645e-5 | **1.7139207630540646e-5** |

These tiny but strictly positive lower bounds prove a **witnessed direction R⊄C for T≥3**, conditional on the frozen R learning rule. They imply neither sample-efficient classification nor R⊄C at T=2, which **remains UNKNOWN**. Together with PR #63's opposite **C⊄R** result, they propose *C/R incomparability only for T≥3*. **No human interpretation is warranted.**

## 4. Updated conservative 15-pair scientific register

The table below distinguishes previously constructive embeddings from newly proposed author-side nonmembership witnesses. `⊆` denotes an existing constructive inclusion; `⊄` denotes a witnessed counterexample to inclusion, **not** disjointness. Every direction not explicitly supported remains UNKNOWN. These are conditional on the stated horizon.

| Pair | Previously established direction | New proposed direction | Current conservative conclusion |
|---|---|---|---|
| M/N | M⊆N | — | reverse unknown except already documented particular N-not-M witness |
| M/C | M⊆C; PR63 C⊄M | — | strict M⊂C for T≥2 (still await independent P0) |
| M/S | M⊆S | — | reverse unknown |
| M/L | M⊆L | — | reverse unknown |
| M/R | M⊆R | — | reverse unknown |
| N/C | PR63 C⊄N | **N⊄C, T≥2** | candidate incomparable, T≥2 |
| N/S | S⊆N | — | reverse unknown |
| N/L | N⊆L | — | reverse unknown |
| N/R | — | — | UNKNOWN |
| C/S | PR63 C⊄S | **S⊄C, T≥2** | candidate incomparable, T≥2 |
| C/L | PR63 C⊄L | **L⊄C, T≥2** | candidate incomparable, T≥2 |
| C/R | PR63 C⊄R | **R⊄C, T≥3** | candidate incomparable T≥3; reverse T2 UNKNOWN |
| S/L | S⊆L via N | — | reverse unknown |
| S/R | — | — | UNKNOWN |
| L/R | — | — | UNKNOWN |

**Status:** This table is **not** an accepted independent 15-pair classification. Existing structural overlaps remain; all zero-effect and boundary cases belong to the H4 family and must not be removed to manufacture exclusive labels. In particular the old T=1 equalities C=N=L and R=M still hold, and PR63's C exclusion at T≥2 cannot be applied at T=1.

## 5. Adversarial audit: strongest failure modes to test

The proof above is deliberately framed as **complete public-event and history-law** inequalities, not as conditional kernel comparisons at an arbitrarily selected latent vector. A real independent reviewer must attempt to break each step:

1. **Conditional selection:** Does the bounded C response still hold after conditioning on the current random matching, fine, partner's previous contribution, and arbitrary earlier public history? Yes *if and only if* the C offset is exactly κ times the **current** partner's previous *binary* action and no other latent offset enters that decision.
2. **R timing/contexts:** Verify that the frozen R update reads the previous aspiration, that `λ_A=0` keeps aspiration zero, that both prior fine contexts are 0, that action 1's Q increments twice and action 0's Q never updates, and that `Qmax=10` does not clip these two steps.
3. **Legality of the conditioned events:** Check all matching indices, dyad-fine indices, signed parameter bounds, and finite positive action probabilities in both policies.
4. **All-H4 rival coverage:** No restriction to a fixed-vector F or to one/two atoms is needed for the *C* inequality: the per-actor pointwise ceiling is independent of its atom tuple. The witnesses are chosen *one-atom truths*, which is an allowed subcase.
5. **Dual bound normalization:** Do not use `TV≥|E h|/2` when `h` has oscillation 1; the valid sharper inequality is `TV≥E_truth h−E_C h` for `h=1_B−c*1_D`. The proof must keep `D` and `B` as public events; payoff is fully determined, not extra random mass.
6. **Scope and causality:** The event could be extraordinarily rare (especially for R), so a strictly positive total-variation bound is not a finite-K power assertion. Neither exogenous fine/matching policy measures empirical realism.

The [stdlib Python audit](../../tools/exp003-exact/check_reverse_c_invariants.py) evaluates *only the stated closed-form values and path arithmetic*; it neither imports the Rust simulator nor proves all-H4 coverage by running a finite sample. **No independent mathematical reviewer was engaged; P0 remains OPEN.**

## 6. Acceptance tasks

1. Obtain an **isolated mathematical verifier** reading **only the frozen v0.2.2 J/H4 specification, inherited action/update rules and a neutral statement of the candidate propositions**, not the author-side proof or Rust code; register accessed materials, toolchain, SHA hashes and any counterexamples.
2. Independently reconstruct C's all-H4 ceiling, N/S/L legal embeddings, and the R-positive-path updated Q/aspiration histories. Accept, modify or refute each *direction separately*.
3. Independently check the full 15-pair register for horizon/policy and any new counterexample; do not mark any unknown reverse direction by symmetry.
4. Keep the composite-null confidence set, simultaneous coverage, model adequacy and real-world generalization gates separate. **Independent P0, global 15-pair resolution and 95% coverage remain OPEN.**
