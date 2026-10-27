# Experiment 003 — restricted H4 free-rival feasibility v0.2 (post-review amendment)

**UTC date:** 2026-10-27. **Status:** exploratory same-implementation engineering; no certified C/L separation, global H4 theorem, independent verifier, or statistical 95% coverage.

The original [v0.1 results](restricted-h4-free-rival-v01-20261027.md) and [v0.1 JSON](restricted-h4-free-rival-v01-20261027.json) are preserved unchanged numerically, with a prominent retrospective erratum explaining that the first three C/L public prefixes were isomorphic and had zero fines. This revision was specified in the [post-review protocol amendment](../restricted-h4-rival-feasibility-protocol-20261027.md) **before the separate v0.2 executable run**, not independently preregistered before initial code development.

## Specific review corrections

**Nonredundant C/L prefixes:** Compared to v0.1, the three conditioned histories under A are now `μ0/[1,0,1,0]/[0,0]` (two mixed dyads), `μ1/[1,0,0,0]/[0,0]` (one mixed and one all-defect dyad), and `μ2/[1,1,0,0]/[1,0]` (mixed pairs with a half-fined first dyad). Under A0, the last history is `μ2/[1,1,1,1]/[0,0]` (all cooperate, no fines); this is a distinct observable structure. All events are legal and strictly positive probability under the policy. **ζ enters the A conditioned T2 distribution through the fined first-period L-belief update**. With A0 no first-period fine is available, and ζ remains **structurally inert through T2**, explicitly recognized by the design rather than fitted as if identified.

**Optimizer-stalling response:** In addition to the unchanged 7-coordinate, 3-start local search, v0.2 enumerates an **exact T1 law matching** submanifold at `β=1, ε=0, κ_L=1, η=0, ρ=1`, with 11 × 11 feasible α/ζ values under A or 11 feasible α values with ζ=0 under A0. This allows joint movement of the belief dynamics without incurring the nonsmooth `TV(T1)` kink. This is still a finite submanifold mesh and a local heuristic **not certified continuous/global optimization**, and its failure to improve does not establish any H4 exclusion.

**Bound interpretation:** The selected N truth versus all-H4 M rivals has an **analytic structural TV lower bound** `0.11865363843605503` under A, using the public half-fine-and-cooperate event. The finite 2,100-point M grid supplies a **feasible upper bound** `0.33386582682695076`, so a gap `0.21521218839089573` remains between those separate bounds. This is not a statistical interval; the lower bound is guaranteed by the model's allowed β and ε boundaries and the upper bound is achieved by one candidate. Under A0 there is a zero-distance rival representation. The numeric M grid result is unchanged by the review amendment.

## Executed review-amended results

| Quantity | A | A0 |
| --- | ---: | ---: |
| N vs M: grid best full T1 TV (2,100 candidates) | 0.33386583 | 0 |
| N vs all H4 M: analytic TV lower bound | 0.11865364 | Not applicable |
| C vs L: mean TV of **three non-isomorphic** T2 conditional laws at baseline | 0.14284072 | 0.08985611 |
| C vs L: revised baseline diagnostic objective D | 0.11427257 | 0.07188489 |
| C vs L: best local plus T1-preserving α/ζ mesh D | **0.11427257** | **0.07188489** |
| C vs L: objective evaluations | 425 | 311 |

The original v0.1 C/L values, **0.10417252 A** and **0.09242343 A0**, cannot be compared directly to v0.2 as a treatment effect or an optimizer improvement, because the conditioned histories and therefore the objective changed. The v0.2 search *again* did not improve on its own frozen-partner starting map, even with manifold moves. This indicates the limited search did not find a closer fit; it does **not** establish a minimum, inequality, or C/L free-H4 law separation.

C/L objective definition remains the predeclared form `D=0.2 TV(complete first-period law) + (0.8/3) sum_h TV(complete T2 public-record suffix | selected h)`. D is **not** the TV distance on the full 384² two-period law and all numbers are IEEE-754 floating-point diagnostic values.

## Executed provenance and custody

[Branch-scoped v0.2 Actions run #40448588745](https://github.com/davletovb/social-emergence-lab/actions/runs/40448588745), source workflow SHA `6f755996ccc7e758352ba43158057158711fa855`, successfully executed `cargo test --locked --manifest-path tools/exp003-exact/Cargo.toml --bin free_rival` (**4 passed, 0 failed**) and ran the bounded binary, recording full [machine-readable JSON](restricted-h4-free-rival-v02-20261027.json). The new regression checks were added to the proposed PR after the v0.2 values were obtained, and must be checked by a separate execution; they are **retrospective guards**, not preregistered targets.

The original raw v0.2 JSON SHA-256 is `a5ced2a183598072b640bbd458b0d1ca751b0847a61ddd3065ed053014528bd4`; evidence artifact `exp003-restricted-h4-review-v02` ID `12709043959`, ZIP SHA-256 `4ddd9576d0ebd5204515923b8128ed9c290b00fd1344326fd45c5fedcf392011`, expires **2026-11-26 04:14:34 UTC**. The artifact contains original raw JSON, locked dependency graph and source SHA-256. Keep it outside GitHub Actions if long-term verification is required.

A same-author stdlib [Python analytic/grid check](../../../tools/exp003-exact/check_free_rival.py) separately expresses the homogeneous N/M T1 joint action law and analytic all-M event bound without importing the Rust implementation; it does not independently verify the C/L search or the full H4 population model.

**R-06 P0 remains OPEN.** A meaningful next advance requires a rigorously **certified continuous nuisance search** or a provably valid model-law separating invariant, not a bigger local optimizer or the v0.1/v0.2 residuals.
