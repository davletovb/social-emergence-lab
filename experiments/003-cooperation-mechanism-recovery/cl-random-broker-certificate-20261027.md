# Experiment 003 — random-rematching invariant and full-H4 L exclusion witness

**Date:** 2026-10-27 UTC · **Contract:** A2 v0.2.2 J/H4, with inherited C/L offsets and period updates.  
**Status:** New author-derived *mathematical model-law witness*, with planned independent-formula numerical checks. NOT a blinded independent scientific P0, full 15-pair classification, confidence-set procedure, or empirical sociological claim.

## Precise proposition

For both policies **A and A0**, at every **T ≥ 2**, the specific **homogeneous one-atom C** truth

`β=1, ε=0, ρ=1, η=0, κ_C=1` (four actors drawn iid from that one atom)

is **not** in the entire permitted **L H4 ≤4-atom observable-law class**. Therefore at either policy and T≥2,

`𝓟_C^{d,T,H4} ⊄ 𝓟_L^{d,T,H4}`.

This is a **one-direction witness** and does **not** prove `L ⊄ C`, global C/L incomparability, disjointness (their zero-effect laws overlap), or anything about the other seven previously UNKNOWN family pairs. This result assumes the **declared random-matching regime is publicly observed** and independently sampled as specified; it would need re-analysis if matching were permanently fixed, random-regime data suppressed or history/action observation changed.

## Entire H4 L rival: observable random-matching invariant

Let E be the publicly observable first-period event `a_1=[1,0,1,0]`, marginalizing *all* remaining t1 events (the first regime, matching and both dyad fines). Set

`B_m = E ∩ {z_2=random, μ_2=m, f_{0,2}=0, a_{0,2}=1}`

for `m∈{0,1}`, where `f_{0,2}` is the fine of **actor 0's dyad**. In canonical matchings, actor 0's partner is actor 1 when `μ=0` and actor 2 when `μ=1`; actor 0 is in dyad 0 in both, so conditioning on its zero fine is unambiguous. The other dyad fine and the other three t2 actor actions are **marginalized**.

For **every legal L H4 representation**, including **≤4 arbitrarily correlated joint atoms**, let X denote all actor coefficients and states after t1. Since in period 2 the regime and the random matching are freshly and independently assigned, conditional on E and `z_2=random` the t2 matching `μ_2` is **uniform and independent of X**. The dyad fine is assigned *after* pairing, independent of X. L actor 0's current contribution offset is `κ_0 q_{0,2}`, where `q_{0,2}` was updated from the **period-1 matched partner and t1 fine**. It does **not** read the current period-2 partner. Hence, for the *same* actor-0 fine (zero),

`P_L(a_{0,2}=1 | E,z_2=random,μ_2=0,f_{0,2}=0)`

`= P_L(a_{0,2}=1 | E,z_2=random,μ_2=1,f_{0,2}=0)`,

even after Bayesian marginalization over arbitrarily correlated H4 actor parameters/states. In particular **for every** rival `P_L∈𝓟_L^{d,T,H4}`, `P_L(B_0)=P_L(B_1)`. There is no optimizer, atom-weight assumption, smoothness assumption or restriction to one-atom L in this step. The proof uses only random-regime independence, exogenous fine assignment, and the definition of L's updated own belief.

## Chosen C truth violates the invariant

In the chosen C model, actor 0's contribution offset in period 2 is `κ_C a_{partner(μ_2,0),1}`: the **current t2 partner's publicly observed prior t1 action**, rather than actor 0's own previously recorded partner. Given E and zero current fine:

- Under `μ_2=0`, actor 0 pairs with actor 1, whose a1 was 0, so `p_C(a_{0,2}=1)=σ(-1)=0.2689414214…`.
- Under `μ_2=1`, actor 0 pairs with actor 2, whose a1 was 1, so `p_C(a_{0,2}=1)=σ(0)=0.5`.

Write `q=σ(-1/2)`, `r=σ(0)=1/2`, and `Δ=σ(0)−σ(-1)>0`. First-period selected C actor probabilities use `q` under no fine and `r` under half fine; t1 random/choice matching are both uniform because the declared t1 neutral prior makes all matching valuations equal. Therefore the **exact** first-action-event probabilities are

`P_C^{A0}(E)=q²(1−q)²`

and

`P_C^{A}(E)=(2g²+uv)/3`,

where `g=[q(1−q)+r(1−r)]/2`, `u=(q²+r²)/2`, and `v=[(1−q)²+(1−r)²]/2`.

Let `k_d=P(z_2=random)P(μ_2=m|random)P(f_{0,2}=0)`. This is `k_A=(1/2)(1/3)(1/2)=1/12` and `k_A0=(1)(1/3)(1)=1/3`. Then

`P_C(B_0)−P_C(B_1)=−P_C(E) k_d Δ`,

which is strictly negative. Hence the C law violates an exact equality satisfied by **every** admissible L H4 law. For the measurable function `h=1_{B_0}−1_{B_1}` taking values in [-1,1], the variational bound `|E_C h−E_L h|≤2 TV(P_C,P_L)` yields the following **uniform-over-rival TV lower bounds** for this **one selected C truth**:

| Policy | P_C(E) (numeric) | k | TV(P_C, *any* L H4) ≥ |
|---|---:|---:|---:|
| A | 0.0600568408621989 | 1/12 | **0.0005781936786095032** |
| A0 | 0.055226744748529843 | 1/3 | **0.0021267688573262324** |

This is a *model-law* inequality for joint public histories, **not** a test-power bound at any K, calibrated confidence set, or claim about real sociology. Rounded decimals are merely reproducible evaluations of the displayed exact exponential formulas.

## Conformance and independent scrutiny

Source `tools/exp003-exact/src/bin/cl_invariant.rs` re-derives the C first-event mass analytically and checks it against **complete marginalization over the 2 × 3 × 4 first-period allocations** using the merged v0.2.2 kernel, plus evaluates one heterogeneously correlated two-atom L mixture across all 16 actor latent configurations. The latter is illustrative; the **general all-H4 L property is established by the random-broker/state-law argument above**, not by checking 16 examples. Numerical checks use IEEE-754 f64, not formalized interval proofs.

The argument must be independently scrutinized for conditioning, latent posterior selection, whether the random regime is indeed observed, dyad-fine support and payoff ordering, and the distinction between one-sided C⊄L and full 15-pair hierarchy classification. Preserve UNKNOWN for `L⊆C` and do not retroactively tick the remaining P0 items. A source-separated Python scalar check of `q,r,g,u,v,Δ` is useful numerical corroboration but cannot substitute for blinded reviewer provenance.

**Scientific acceptance:** R-06 P0 remains open; the proof is a candidate for independent role-A review. It advances a *specific theorem about this synthetic model* beyond the v0.1/v0.2 local searches without implying human behavior.
