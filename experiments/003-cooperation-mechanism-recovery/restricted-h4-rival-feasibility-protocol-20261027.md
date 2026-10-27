# Experiment 003 R-06 — restricted one-atom H4 rival-fit feasibility protocol

**Version:** `restricted-H4-one-atom-free-rival-feasibility-v0.1` (historical first run), subsequently **review-amended to v0.2 before the separate v0.2 execution** · **2026-10-27**. The original protocol was committed 47 seconds after source code, but **before** the first executable results: this is a documented/frozen engineering design, not independent prospective scientific preregistration.

**Status:** bounded exploratory numerical / engineering feasibility only. No claim of complete H4 optimization, certified confidence-set test, globally identified family, independent mathematical verification, or human-social observation.

## Model and exact restriction

Use merged A2 v0.2.2 Rust implementation and the full J action-vector product kernel. Exactly **four actors sharing the same tuple** (`G_m` has **one atom of probability one**, sampled iid once per society), not four unrelated F fixed actor parameter vectors. Source truths are fixed homogeneous C and N examples. Candidate scalar and actor tuple domains are always enforced by `Society::new_fixed`, including `c=1/3+η∈[-3,3]`. No unstated per-society hyperprior, actor correlation or extra random innovation. No stochastic simulation is run.

All one-period public outcomes are enumerated from the 384 combinations of regime × matching × dyad fines × simultaneous action vector; A0 has 48 positive-support possibilities. The t2 suffix is enumerated from a public, positive-probability t1 history. Payoffs are deterministic functions already checked elsewhere. All probabilities are **f64 IEEE logistic/softmax**, not certified exact real arithmetic.

## Track 1 — N target versus M free one-atom rival

- Truth `N: β=4, ε=0, ρ=1, η=0, ν=0.75`.
- Rival M uses free `β∈{0.1,0.2,...,10}` (100 points), `ε∈{0,0.025,...,0.5}` (21 points), exhaustively checks the **2,100-point grid**. η and ρ do not enter **T1** M action/matching probabilities (t1 matching choice is neutral, regime assignments exogenous). Fix them to zero and one respectively only to remove the T1 nuisance nonidentification, **not** to discard a legitimate T1 candidate direction.
- Optimize **full T1 384-record total-variation** separately under A and A0.
- The minimum across a grid gives **an attainable upper bound** on the unrestricted continuous infimum; positive grid minima have *no exclusion force*.
- **Analytic all-H4 M witness**, derived without optimization: with half-fine at actor 0, every legal M, including arbitrarily correlated ≤4-atom actor-iid H4, has cooperation `≤p_Mmax=0.25+0.5σ(-0.05)≈0.49375` (β≥0.1, ε≤0.5). The chosen homogeneous N has `p_N=σ(1)≈0.73106`. Under A the event `{actor0 assigned fine 0.5, actor0 cooperates}` has probability `0.5p_N` for truth, at most `0.5p_Mmax` for any M. **Hence total-variation of the entire A public observable law is at least `0.5(p_N-p_Mmax)>0`** for this one N truth, and the restriction to T1 is already sufficient. Under A0 there is a known exact M representation with β=1 and ε=0. This argument is conditional on the model's randomized fine after pairing, actor independence, and admissible parameter bounds.

## Track 2 — C target versus L free one-atom rival

- Truth `C: β=1, ε=0, ρ=1, η=0, κ_C=1`, same for both policies.
- Allow all seven L coordinates to vary within legal bounds:
  - `β:[0.1,10]`, `ε:[0,0.5]`, `ρ:[0.1,10]`;
  - `η:[-10/3,8/3]`, `κ:[-4,4]`, `α:[0,1]`, `ζ:[0,1]`.
- Fix **three** preselected t1 public histories: regime random, no fines; matchings `μ=0,1,2` respectively; past actions `[1,0,1,0]`, `[0,1,1,0]`, `[1,1,0,0]`. All are positive probability under A and A0, even when using the rival L. These histories are **not a random sample**.
- Declare *diagnostic discrepancy*, **not a probability distance for a full two-period law**:
  `D=0.2 TV(P_C(t1),P_L(t1)) + (0.8/3)∑_{h∈H3} TV(P_C(t2|h),P_L(t2|h))`.
- Start from three L tuples: the familiar `α=1,ζ=0,κ=1,β=1,ε=0,ρ=1,η=0` frozen-previous-partner mapping, a moderate learning case `(β,ε,ρ,η,κ,α,ζ)=(1.7,.08,2.3,.1,1.4,.5,.5)`, and `(3,.25,5,0,-1,0,1)`.
- Coordinate pattern search in fixed coordinate order, scales `1,.5,.25,.125`, two sweeps each, step vector `(1,.1,1,.25,1,.25,.25)`, plus/minus directions and legal bounds clamping. Hard limit `2,000` objective evaluations per policy. Stable deterministic improvement threshold `1e-13` and start-order tie handling. No other models, atom-count escalation, optimizer restarts based on observations or post-hoc parameter-grid tuning.
- Report original fixed mapping discrepancy, best local diagnostic fit, number of evaluations, and all fitted coordinates under both policies.
- **A positive local residual is NOT proof that an admissible alternative L law does not match full C.** It is not a certified global infimum even in the one-atom subfamily. The C/L *full free-H4* relationship remains UNKNOWN. A near-zero diagnostic discrepancy also would not prove global equality because only three T2 prefixes are examined.

## Resource and verification gates

- Add a **standalone** Rust binary under `tools/exp003-exact/src/bin/free_rival.rs`. Reuse the pinned stand-alone lockfile and model revision; no new dependencies or workflow on main.
- A single **one-off branch-scoped** workflow is allowed for `cargo test --locked --manifest-path tools/exp003-exact/Cargo.toml --bin free_rival` and the bounded binary execution. Record source SHA, output JSON, run-time cost and hashes.
- No search of full 384² histories, no two-atom rival optimization, no L/R full H4 fitting, no stochastic simulations or classifier.
- Explicitly report which quantities are **analytic certified** (the M half-fine event inequality by an elementary proof), which are **complete finite grid** (M 2,100 candidate points), and which are **local heuristic** (L 7-dimensional coordinate optimization). Do not mark R-06's independent P0, global H4 inclusion results or valid 95% statistical coverage complete.

**Next feasibility gate after results:** decide whether a genuinely certified *continuous* one-atom solver with interval/branch-and-bound bounds is computationally practical before advancing to free unknown two-to-four-atom H4. A proprietary black-box optimizer or a finite grid alone is not proof.

## Post-review amendment v0.2 (2026-10-27; authored BEFORE revised execution)

The first [v0.1 execution](results/restricted-h4-free-rival-v01-20261027.md) and its exact results remain immutable and honestly labelled. Reviewer [PR #62](https://github.com/davletovb/social-emergence-lab/pull/62) identified three real v0.1 limitations:

1. The three first-period histories were all relabelings of the same two mixed pairs, so the repeated conditional TVs provided no additional structural contrast.
2. Every prefix used zero fine, rendering `ζ` inert in this objective.
3. Coordinate-wise movements away from exact `T1` matching could incur a nonsmooth TV penalty, obscuring opportunities along its exactly fitting parameter manifold.

**Frozen replacement design for v0.2:** The source C truth, source N truth, fine/cooperation certificate, 2,100-point M grid and seven-dimensional general legal L parameter bounds are unchanged. The three *new* **non-isomorphic** C/L first-period records are all random-regime and have (matching, actions, fine bits) under intervention **A**:

- `(μ0, [1,0,1,0], [0,0])`: two pairs of mixed actions.
- `(μ1, [1,0,0,0], [0,0])`: one mixed and one all-defect dyad.
- `(μ2, [1,1,0,0], [1,0])`: two mixed dyads, with a half-fine on the first dyad. This exercises `ζ` in the L belief update.

Under control **A0**, the third event is `(μ2, [1,1,1,1], [0,0])`: two all-cooperation pairs. Under A0 there are **no fines in any T1 history**, so ζ cannot influence the T2 conditional laws *by construction*; it remains a free but uninformative structural coordinate in the general L family. The restricted α-grid therefore fixes ζ=0 for this A0 diagnostic, **not** as a scientific parameter exclusion.

**Objective unchanged:** `D=0.2 TV(T1 complete law) + (0.8/3)∑ TV(T2 complete law | each of three specified prefixes)`; the new three conditionals are now non-redundant. Not full T2 joint law TV.

**Nonsmooth-manifold guard:** In addition to the v0.1 starts and coordinate search, perform a deterministic **11×11 α/ζ grid** under A while fixing `β=1,ε=0,κ_L=1,η=0,ρ=1` (thus T1 exact match), and **11 α points** with ζ=0 under A0. These are additional *feasible*, non-certifying rival candidates, evaluated before the unchanged three multistart coordinate sweeps; keep the same hard cap of 2,000 evaluations per policy. Any improvement on the previous frozen-partner map is allowed, but a residual **cannot** establish a global one-atom or four-atom H4 noninclusion. Add a regression test establishing that the selected fined A prefix responds to ζ whereas A0 cannot.

**Numerical assertions:** the published v0.1 grid minimum `0.33386582682695076` (A), `0` (A0), and the original v0.1 C/L residuals `0.10417252324126053` and `0.09242343145200195` are *historical* rather than valid predicted v0.2 C/L residuals. Pin the **v0.2** results to a declared tolerance only after the revised code has been executed; do not present retrospectively pinned values as preregistered targets.

**Analytic M TV interval:** the all-M H4 event argument gives a **lower bound** 0.11865, whereas the finite M grid gives a feasible **upper bound** 0.33387 on the unknown continuous minimum. These are not estimates with a confidence interval: the bounds are a provably true structural lower bound and a candidate upper bound, leaving a current **gap ≈0.21521** for the selected N truth under A. Continuous/global optimization could reduce the upper bound without violating the lower bound.

**Cross-language arithmetic:** a separately expressed standard-library Python check may validate the M event bound and the homogeneous M/N T1 full-record TV. Such same-author Python checks do **not** constitute independent P0 scientific verification. No Python cross-check of the full C/L seven-dimensional optimizer is implied.

**Version separation:** Keep v0.1 report+JSON unchanged, create a distinct v0.2 report+JSON and separate one-off workflow artifact. Do not edit or replace past numerical evidence; no permanent/main GitHub Actions workflow. The scientific P0 acceptance gates remain open.
