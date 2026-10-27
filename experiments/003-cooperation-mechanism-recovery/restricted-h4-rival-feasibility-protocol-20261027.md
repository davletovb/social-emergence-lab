# Experiment 003 R-06 — restricted one-atom H4 rival-fit feasibility protocol

**Version:** `restricted-H4-one-atom-free-rival-feasibility-v0.1` · **2026-10-27** · **registered before executable results**.

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
