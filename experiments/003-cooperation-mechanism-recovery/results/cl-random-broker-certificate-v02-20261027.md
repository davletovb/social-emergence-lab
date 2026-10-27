# Experiment 003 — five-family H4 rematching invariant, reviewed extension v0.2

**UTC date:** 2026-10-27. **Status:** author-derived mathematical candidate + independent-formula, same-author numerical checks. **Not** a completed independent P0 verification, confidence-set result, or sociological observation.

This is the [PR #63 review](https://github.com/davletovb/social-emergence-lab/pull/63#pullrequestreview-5664605720) correction to the [original v0.1 report](cl-random-broker-certificate-20261027.md). The original one-E v0.1 computation, [JSON](cl-random-broker-certificate-20261027.json), and runs remain intact. The expanded proof is given in the [appendix to the mathematical certificate](../cl-random-broker-certificate-20261027.md#2026-10-27--review-driven-extension-v02-preserves-original-one-e-certificate). Source model remains A2 v0.2.2, with no newly invented sociological agents or interventions.

## New scope: every partner-independent family

Under public `z_2=random`, current matching `μ_2` is independent of the entire past and latent actor states. For fixed actor0 fine, the M/N/S/L/R cooperation offset at t2 does not read the **current** assigned partner. Thus, for any first-period public action vector `E_a`, the two joint events `B_{0,a},B_{1,a}` obtained by setting `μ_2=0` versus `μ_2=1` and holding t2 random regime, actor0 fine zero and t2 actor0 cooperation fixed have equal probabilities **under any M, N, S, L or R H4 population representation**, even at four unknown correlated actor-mixture atoms.

The selected homogeneous C truth with `β=κ=1, ε=0, η=0` violates the equality when the previous period's actor1 and actor2 actions differ. The *same selected C truth* lies outside every rival M/N/S/L/R class for each policy A and A0 at **T≥2**, *assuming independent mathematical scrutiny confirms the joint-conditioning proof*. This shows a candidate **one-direction C-not-subset** result for five distinct pairs, not five disjoint classes and not any reverse noninclusion. In particular, constructive **M⊆C** remains true, and the old T=1 class equalities remain unchanged.

## Stronger yet finite observable-statistic bound

Instead of fixing only the original `E=[1,0,1,0]`, partition by all 16 public first-period action vectors. On the eight patterns `a_1≠a_2`, use the sign `a_1−a_2` to align the matching-0 vs matching-1 contrast. The sum defines a **single measurable function `h∈[-1,1]`** because each observed history has exactly one first-period action vector and at most one of the two matching events. For **every** admitted partner-independent H4 law, `E[h]=0`; for the C witness, `E_C[h]=k[σ(0)−σ(−1)]P_C(a_{1,1}≠a_{2,1})`.

Consequently `TV(P_C,P_m)≥|E_C[h]|/2` for **each** m=M,N,S,L,R. The exact-finite first-period calculation gives:

| Policy | Pr(first-period actions of actors 1 and 2 differ) | Original single-E lower bound | Valid pooled all-pattern lower bound |
|---|---:|---:|---:|
| A | 0.490002474801063 | 0.000578193679 | **0.004717469806363266** |
| A0 | 0.470007424403189 | 0.002126768857 | **0.018099874571358386** |

This is an **analytic population-law TV lower bound for one specified C truth**, not a finite-sample effect size, error probability or experimental power. Four-actor aggregation was not claimed: summing overlapping actor observables without boundedness would invalidate the TV argument. The improvement here comes from aggregating **disjoint first-period action patterns for actor 0**.

## Corrected execution evidence

**One-off reviewed revision:** [GitHub Actions run #40479399016](https://github.com/davletovb/social-emergence-lab/actions/runs/40479399016), source checkout `395f123ca4abf44981acb9048e6ca11392dd3de5`. It ran `cargo test --locked --manifest-path tools/exp003-exact/Cargo.toml --bin cl_invariant` (**5 Rust tests passed**) and `python3 tools/exp003-exact/check_cl_h4_invariant.py` (**4 Python tests passed**). The source-separated same-author Python checks now (a) independently enumerate all public first-period matching/fine allocations for C rather than merely transcribe the Rust formulas, (b) compute the **two** L t2 matching event probabilities separately, and (c) enumerate the pooled bound across all 16 action patterns. The Rust tests check two-atom examples in each of the five families M/N/S/L/R and numerically pin the pooled bounds. None of these tests constitutes independent blind mathematical verification across all H4 nuisance parameters.

**Raw reviewed JSON SHA-256:** `14e99e792dca58d8c25bedceafc136af642aec23342692495e666808b788e01d`. Its [machine-readable GitHub companion](cl-random-broker-certificate-v02-20261027.json) adds provenance. Workflow artifact `exp003-cl-fivefamily-invariant-v02`, ID `12721986906`, ZIP SHA-256 `afbd2b2a3d110de39929230a0174d3a6487bbe52c195bb1929c5a70e9d1afdd3`, expires **2026-11-26 09:15:57 UTC**. Download and store independently for durable custody.

**Original provenance corrected without changing its numbers:** original mathematical-document commit `2ce2572ab3f17ed9576fe214950f1e39de47450f` was *not* the executed Git checkout; Rust implementation was introduced in `67ebffa3bf976a176de5717cf4e162b873b130e6`, and the successful original Rust CI checkout was `e1e15f268b1d50a27504267573b45d8e890a0352`. The original Python [run #40476930928](https://github.com/davletovb/social-emergence-lab/actions/runs/40476930928) is now explicitly recorded in the [v0.1 JSON provenance](cl-random-broker-certificate-20261027.json).

## What remains open

1. A **source-isolated adversarial mathematics reviewer** should test the random-matching and random-fine conditioning, whether any legal H4 mixture induces matching-dependent t2 actor response after conditioning on the public event, and the all-pattern bounded TV statistic.
2. **All reverse directions** (N⊄C, S⊄C, L⊄C, R⊄C) remain UNKNOWN; M⊆C remains a proved constructive inclusion. No complete 15-pair diagram is claimed.
3. H4 finite-data composite-null inference, **95% coverage**, power calibration, independent Python P0, and human sociological validity remain OPEN.
