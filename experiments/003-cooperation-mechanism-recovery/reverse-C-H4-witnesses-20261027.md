# Experiment 003 / R-06 — adversarial reverse-C identification audit

> **2026-10-27 review amendment (v0.2):** The reviewer identified a shorter **R⊄C witness at T=2**, superseding the original T≥3 **minimum-horizon claim**. The original T3 path and original output remain valid historical, nonminimal evidence. The corrected result is that **N⊄C, S⊄C, L⊄C, and R⊄C all hold at T≥2** for the selected legal truths, under A and A0. See the new T2 argument in §2A and the separate v0.2 [result JSON](results/reverse-C-H4-witnesses-v02-20261027.json). This is an author-side amendment, still awaiting independent mathematical P0 acceptance.

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

## 2A. Review correction: R ⊄ C already at T≥2

The frozen R update makes this direction possible after only **one**, rather than two, positive cooperative payoffs. Choose the same homogeneous legal R truth as the historical T3 example: `β=1, ε=0, α_R=1, λ_A=0, η=0`, initially `Q_1=A_1=0`.

Let the positive-probability first-period observable event be `H_1={z_1=random, μ_1=0=01|23, fine bits [0,0], actions [1,1,0,0]}`. At T1 every actor has `p=σ(−1)=q` because R's initial Q values are zero. Thus `P(H_1)=w_d q²(1−q)²`, where `w_A=(1/2)(1/3)(1/4)=1/24` and `w_A0=1/3`. Actor 0 received payoff `u_{0,1}=1`, so the **post-T1** transition with `α_R=1, λ_A=0` raises **only** `Q_{0,2}(1,0): 0→1` and leaves `Q_{0,2}(0,0)=0`; its aspiration remains zero.

At T2 define the *entirely public* condition `D_2=H_1∩{z_2=random, μ_2=1=02|13, f_{0,2}=0}`. Here the new partner of actor 0 is actor 2, who defected at T1 (`a_{2,1}=0`). Every admissible C H4 rival has conditional actor-0 cooperation probability **≤ c* = 0.48751040626053** on `D_2`. The selected R's conditional probability is instead

`p_R(a_{0,2}=1|D_2)=σ(−1+Q_{0,2}(1,0)−Q_{0,2}(0,0))=σ(0)=1/2>c*`.

The **other** T2 dyad fine and actors 1,2,3 T2 actions are marginalized. The T2 random matching and fine assignment are independent of previous histories, so

```text
P_R(D2) = w_d q²(1−q)² v_d,
v_A=(1/2)(1/3)(1/2)=1/12; v_A0=1/3.

P_R(D2): A=0.00013422309823941982,
         A0=0.004295139143661434.

TV(P_R^d,P_C^d) ≥ P_R(D2) × (0.5−c*)
  A:  0.0000016763919674633255
  A0: 0.000053644542958826415
```

As in §1, the **range-one** measurable function `g=1_{D_2∩{a_{0,2}=1}}−c*1_{D_2}` gives the bound directly for **every H4 C rival**, independent of latent κ/η distribution or conditioning. The selected one-atom R truth and both policies make `P_R(D_2)>0`. This proves a **candidate R⊄C at T≥2**, not sample-efficient detection. It also resolves the original author's T2 UNKNOWN as a genuine previously overlooked direction.

The original two-update T3 example in the following section remains mathematically possible, but its TV lower bounds are **smaller** because they use an unnecessarily rare history. That section is retained for reproducibility and is **not** a statement of minimal horizon.

## 3. Historical (nonminimal) R ⊄ C at T≥3 through a public path

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

These **historical, nonminimal T3** bounds remain strictly positive under the frozen R law. The **T2** argument in §2A establishes the stronger minimum horizon, so the candidate C/R two-direction noninclusion already holds for **T≥2** when combined with PR #63. Neither argument supplies finite-K power or independently accepted scientific certification. **No human interpretation is warranted.**

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
| C/R | PR63 C⊄R | **R⊄C, T≥2 (review amendment)** | candidate incomparable T≥2; T1 R=M remains |
| S/L | S⊆L via N | — | reverse unknown |
| S/R | — | — | UNKNOWN |
| L/R | — | — | UNKNOWN |

**Status:** This table is **not** an accepted independent 15-pair classification. Existing structural overlaps remain; all zero-effect and boundary cases belong to the H4 family and must not be removed to manufacture exclusive labels. In particular the old T=1 equalities C=N=L and R=M still hold, and PR63's C exclusion at T≥2 cannot be applied at T=1. In the corrected R direction, one reinforcement update is enough to exceed C's fine-0 ceiling at T2.

## 5. Adversarial audit: strongest failure modes to test

The proof above is deliberately framed as **complete public-event and history-law** inequalities, not as conditional kernel comparisons at an arbitrarily selected latent vector. A real independent reviewer must attempt to break each step:

1. **Conditional selection:** Does the bounded C response still hold after conditioning on the current random matching, fine, partner's previous contribution, and arbitrary earlier public history? Yes *if and only if* the C offset is exactly κ times the **current** partner's previous *binary* action and no other latent offset enters that decision.
2. **R timing/contexts:** For the **new minimal T2 witness**, verify that the first-period zero-fine cooperation payoff 1 increments actor0 Q(1,0) once *after* T1 and that this is used at T2. The original T3 example still checks the second increment, prior aspiration, `λ_A=0`, Qmax clipping and proper fine/action context. An implementation test should apply both public records to all four actors without writing the payoff/update result by hand.
3. **Legality of the conditioned events:** Check all matching indices, dyad-fine indices, signed parameter bounds, and finite positive action probabilities in both policies.
4. **All-H4 rival coverage:** No restriction to a fixed-vector F or to one/two atoms is needed for the *C* inequality: the per-actor pointwise ceiling is independent of its atom tuple. The witnesses are chosen *one-atom truths*, which is an allowed subcase.
5. **Dual bound normalization:** The general `TV≥|E_P h−E_Q h|/2` bound remains **valid but loose** when `h` has oscillation one; a sharper inequality is `TV≥E_truth h−E_C h` for `h=1_B−c*1_D`. The proof must keep `D` and `B` as public events; payoff is fully determined, not extra random mass.
6. **Scope and causality:** The event could be extraordinarily rare (especially for R), so a strictly positive total-variation bound is not a finite-K power assertion. Neither exogenous fine/matching policy measures empirical realism.

The reviewed [stdlib Python audit](../../tools/exp003-exact/check_reverse_c_invariants.py) now **derives the R public-history action likelihoods, material payoffs, contextual Q/aspiration updates and actor-specific t2 cooperation for all four actors**, rather than hard-coding t2 mass. It does not import the Rust implementation. A separate [Rust model conformance checker](../../tools/exp003-exact/src/bin/reverse_c.rs) applies the stated public records to the *actual simulator*, and its R T2/T3 probabilities agree with Python to numerical precision. These same-author checks do **not** by themselves prove full-H4 coverage or constitute independent P0. **No source-isolated independent mathematical reviewer was engaged; P0 remains OPEN.**

## 6. Acceptance tasks

1. Obtain an **isolated mathematical verifier** reading **only the frozen v0.2.2 J/H4 specification, inherited action/update rules and a neutral statement of the candidate propositions**, not the author-side proof or Rust code; register accessed materials, toolchain, SHA hashes and any counterexamples.
2. Independently reconstruct C's all-H4 ceiling, N/S/L legal embeddings, and the R-positive-path updated Q/aspiration histories. Accept, modify or refute each *direction separately*.
3. Independently check the full 15-pair register for horizon/policy and any new counterexample; do not mark any unknown reverse direction by symmetry.
4. Keep the composite-null confidence set, simultaneous coverage, model adequacy and real-world generalization gates separate. **Independent P0, global 15-pair resolution and 95% coverage remain OPEN.**


## 7. Executed review-amended checks (v0.2)

The original [v0.1 executed result](results/reverse-C-H4-witnesses-20261027.json) is preserved unchanged as historical T3 evidence. The [distinct v0.2 report](results/reverse-C-H4-witnesses-v02-20261027.md) and machine-readable summary record the corrected T2 witness. The initial targeted review run [#40503052644](https://github.com/davletovb/social-emergence-lab/actions/runs/40503052644) caught a **Rust float literal syntax typo before testing the model**; this was fixed in the PR and the corrected targeted [run #40503316441](https://github.com/davletovb/social-emergence-lab/actions/runs/40503316441) succeeded with **3 Rust + 6 Python tests**. It also ran both independently expressed code paths and compared full selected R truth event probabilities and bounds across languages. Results are still author-side, not independent scientific validation.
