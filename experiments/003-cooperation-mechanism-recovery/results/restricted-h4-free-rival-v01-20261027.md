# Experiment 003 — restricted H4 free-rival fitting: first feasibility result

**Retrospective review correction (2026-10-27):** The v0.1 C/L three-prefix diagnostic unintentionally used *three isomorphic relabelings* of two mixed-action dyads, with no fines in any prefix. It effectively tested one conditional pattern three times; ζ was **completely inert** through T2. Moreover, the coordinate optimizer could stall at a nonsmooth exact-T1 match. Its positive residual and “no improvement” result therefore supply **no global, or even strong local, C/L exclusion evidence**. The values below are preserved as *historical original run results*, not silently replaced. A separate, amended v0.2 re-execution uses non-isomorphic histories, an A-fined prefix, and a T1-preserving α/ζ subspace search. See [v0.2 corrected report](restricted-h4-free-rival-v02-20261027.md).

**Date:** 2026-10-27 UTC  
**Model:** `Experiment003-A2-v0.2.2`, [v0.1 pre-execution protocol](../restricted-h4-rival-feasibility-protocol-20261027.md)  
**Scientific status:** **EXPLORATORY / SAME-IMPLEMENTATION MODEL DIAGNOSTIC**. There is no global C/L H4 exclusion, independent free-H4 optimization, completed 15-pair classification, calibrated confidence set, or empirical human social-behavior result.

## Executed scope and custody

- New standalone Rust binary [`tools/exp003-exact/src/bin/free_rival.rs`](../../../tools/exp003-exact/src/bin/free_rival.rs) reuses the merged v0.2.2 J joint action and broker laws and the **committed** stand-alone Cargo.lock. No new dependencies. All actor types in a one-atom H4 candidate are **identical across the four actors**, which is NOT the freely actor-indexed F oracle. No random draws.
- Bounded run: [GitHub Actions #40439330773](https://github.com/davletovb/social-emergence-lab/actions/runs/40439330773), one-off operational branch `ops/exp003-free-rival-once-20261027`, SHA `61ba7ec250b3c9e2d95aec08388c65f0f296c11b`.
- Targeted Rust test: **3 passed, 0 failed**. Includes T1 mass normalization/A0 support, prefix conditional mass support, finite grid plus analytic event lower-bound guard, and multi-start objective monotonicity. Binary successfully produced the audited machine-readable values in [`restricted-h4-free-rival-v01-20261027.json`](restricted-h4-free-rival-v01-20261027.json).
- Output JSON SHA-256 `470b764ad74e4b783aa2a55399bc95a140fc01ec8872329575a47e1269c534f5`. Full evidence ZIP GitHub artifact `exp003-restricted-h4-free-rival-oneoff`, ID `12705991040`, SHA-256 `703bf203c3a759dbad4acaa23a89508b6295e06b602b87938507c0345ec4f81c`, expires **2026-11-26 02:36:11 UTC**. Includes original JSON, lockfile and SHA-256 source/output manifests. Preserve outside GitHub Actions if needed after expiry.
- No long-lived CI workflow was added to `main`; the one-off workflow exists only on the separate operational branch and is not in this PR.

## Result 1: N truth versus fitted M representations

Target: homogeneous one-atom N with `ν=0.75, β=4, ε=0`. All matching-appraisal nuisance parameters for N/M are equal, and t1 broker matching is uniform. A fixed complete **2,100-candidate grid** varies M `β=0.1...10.0` by 0.1 and `ε=0...0.5` by 0.025. These are all relevant one-atom M *T1* parameters because `η/ρ` do not affect M's t1 broker and its action offset is always zero.

| Quantity | Intervention A | Sham control A0 |
| --- | ---: | ---: |
| Best grid M representation `β` | 1.0 | 1.0 |
| Best grid M representation `ε` | 0.0 | 0.0 |
| Full 384-record t1 law total variation, grid minimum | **0.33386583** | **0.00000000** |
| Finite candidates compared | 2,100 | 2,100 |
| Mathematically certified **all-H4 M** TV lower bound for this N truth | **0.11865364** | Not applicable |

The **grid minimum is NOT a certified lower bound** over the continuous M one-atom class or H4 four-atom mixture family. It is the smallest discrepancy *observed among those 2,100 feasible M representations*, hence a feasible **upper bound** on a possible global minimum. More exhaustive numerical optimization might reduce it.

The separate **analytic lower bound is valid for all H4 M candidates** because for every M with `β≥0.1, 0≤ε≤0.5` its half-fine cooperation probability satisfies

```text
p_M(1 | f=1/2) = ε/2 + (1−ε)σ(−β/2)
              ≤ 0.25 + 0.5σ(−0.05)
              = 0.49375130175789483.

p_N(1 | f=1/2) = σ(4(0.5−1+0.75))
              = σ(1) = 0.7310585786300049.
```

The fine is assigned **after** matching, with marginal probability 1/2 to actor 0 under A. Therefore, comparing the single *observable* event `E={actor 0 fined at 1/2 and cooperates}` gives

`TV(P_N^A, P_M^A) ≥ P_N(E) − P_M(E) ≥ ½(0.7310585786−0.4937513018)=0.11865363843605503`.

The bound survives the entire M H4 actor-iid ≤4-atom nuisance mixture and admissible broker selection because M's action link contains no latent offset: mixing and rematching cannot raise the allowed actor's half-fine cooperation response above that supremum. This is a **specific N truth's structural exclusion of M under A**, not globally identified six-family membership, not uniform separation for all N laws, and not statistical power/coverage.

Under A0, M with `β=1, ε=0` matches the chosen N zero-fine action law; the grid exactly recovers that compatible representation.

## Result 2: C truth versus locally searched one-atom L rivals

Source truth: homogeneous C with `β=1, ε=0, ρ=1, η=0, κ_C=1`. Seven L rival degrees of freedom are allowed: `β,ε,ρ,η,κ_L,α_L,ζ`, subject to their full v0.2.2 legal bounds. This search is *not* a free four-atom H4 optimizer.

Because the full 384×384 two-period law would be expensive for a repeated optimization, the **predeclared diagnostic objective** is a weighted sum of the complete first-period 384-category TV and three **complete 384-category second-period conditional laws after preselected positive-probability first-period histories**:

`D=0.2 TV(P_C(t1),P_L(t1)) + (0.8/3)Σ_{h∈H3}TV(P_C(t2|h),P_L(t2|h))`.

**Do not call D a likelihood or a two-period joint-law TV.** It weights *rare selected histories* deliberately; it is a sensitivity diagnostic, not a probabilistically weighted full two-period error function, and any good D does not imply equality of the remaining histories.

| Quantity | Intervention A | Sham control A0 |
| --- | ---: | ---: |
| Starting frozen-partner-map D | 0.10417252 | 0.09242343 |
| Best 7-dimensional bounded local search D | **0.10417252** | **0.09242343** |
| Best fit `(β,ε,ρ,η,κ,α,ζ)` | (1,0,1,0,1,1,0) | (1,0,1,0,1,1,0) |
| Full first-period law TV at best fit | 0 | 0 |
| Mean of three selected conditional T2 TV values | 0.13021565 | 0.11552929 |
| Numerically evaluated candidate tuples | 303 | 300 |

The three starts, step sizes, parameter bounds, pass order and objective weights were frozen in the [protocol](../restricted-h4-rival-feasibility-protocol-20261027.md) **before execution**. The optimizer uses bounded coordinate moves and decreasing step scales; no black-box random search or tuning to the observed output was done.

**Interpretation:** The local one-atom L search found **no improvement over the original failed frozen-partner mapping** on these diagnostics. A positive residual does **not** establish even the continuous one-atom minimum, much less the full free H4 four-atom family: this search can become trapped, covers only three T1 histories, and gives no interval/branch-and-bound certificate. The global `C⊆L`, `L⊆C` and incomparability questions remain **UNKNOWN**.

## Research decision / next gate

This work is worth keeping because it distinguishes two types of progress: **(a) an all-H4 analytical event certificate against M for the selected N truth**, and **(b) a feasible but non-certifying local one-atom fit against L**. Generalizing (a) requires a *new* invariant or certificate; scaling (b) into a big simulation cannot convert local numerical residuals into mathematical nonmembership.

**Suggested next slice:** scope a **certified continuous one-atom** feasibility procedure for only one targeted C/L case (e.g. interval-enclosed branch-and-bound on a carefully chosen observable inequality or test-cell projection), documenting tight lower bounds, parameter boundary degeneracies and runtime. Only then evaluate two-atom → four-atom H4 mixture fitting or a valid statistical composite-null method. Continue full 15-pair proofs and independent reviewer submissions separately.

**Acceptance:** R-06 P0, global 15-pair classification and proposed H4 95% statistical coverage remain **OPEN**. This PR does not modify the signed model contract, private verifiers or existing study acceptance.
