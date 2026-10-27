# Experiment 003 R-06 — C versus L random-rematching invariant: executed witness

**Original v0.1 analysis; 2026-10-27 review extension:** See the [separate, executed v0.2 five-family and pooled-event report](cl-random-broker-certificate-v02-20261027.md). The original numeric results below remain unchanged. The new extension enlarges the *proposed* one-direction noninclusion from L to M/N/S/L/R and improves the TV lower bound using disjoint t1 action histories. Independent P0 review is still pending.

**UTC:** 2026-10-27 · **Model:** A2 v0.2.2 J/H4 · **Status:** candidate author-derived *synthetic* law theorem and independent-formula numerical diagnostics, **not** an independently verified P0 theorem or statistical identification procedure.

## Mathematical claim and its precise strength

See the full [self-contained mathematical argument](../cl-random-broker-certificate-20261027.md). This is a substantial improvement over the **inconclusive** C/L seven-parameter local optimization in merged PR #62: a simple observable matching-regime invariant proves that **one selected C H4 law lies outside every admissible L H4 representation** under A and A0 at **T≥2**. Therefore **C⊄L** for the prescribed model-law family at these horizons, assuming its author-side proof withstands independent scrutiny.

It is NOT a proof of **L⊄C** or C/L incomparability/disjointness, and not a statistical test or 95% confidence-set procedure. The earlier conservative 15-pair table should be updated only as a *proposed one-direction theorem*, with reverse relation UNKNOWN until scientifically accepted. Common zero-effect laws still overlap.

## Executed bounded computation

[One-off Rust Actions #40476576980](https://github.com/davletovb/social-emergence-lab/actions/runs/40476576980), executed checkout SHA `e1e15f268b1d50a27504267573b45d8e890a0352` (original Rust implementation introduced in `67ebffa3bf976a176de5717cf4e162b873b130e6`; mathematical document commit `2ce2572ab3f17ed9576fe214950f1e39de47450f`), **3 Rust tests passed**. The executable enumerated all publicly allowed t1 regime/matching/fine combinations for fixed first actions `[1,0,1,0]` and checked joint observable contrasts for the chosen C truth, plus a 2-atom correlated L H4 population integrated across 16 actor type configurations. The tests compare independent analytic first-event C formulas against the Rust v0.2.2 model event masses.

[One-off stdlib Python Actions #40476930928](https://github.com/davletovb/social-emergence-lab/actions/runs/40476930928), **3 Python tests passed**, independently expressing the chosen C exponential formulas and the selected joint two-atom L mixture without importing the Rust model or its binary. This is a **separately expressed same-author arithmetic cross-check**, not an externally isolated Python P0 or exhaustive independent H4 implementation.

The original unrounded C and L masses are retained in [machine-readable JSON](cl-random-broker-certificate-20261027.json). All public calculations are finite categorical sums plus floating-point sigmoid evaluations, not randomized synthetic societies.

## Results

| Policy | P_C(first actions [1,0,1,0]) | P_C(B0) | P_C(B1) | TV lower bound vs **any** L H4 |
|---|---:|---:|---:|---:|
| A | 0.060056840862198896 | 0.001345981012039281 | 0.002502368369258287 | **0.0005781936786095031** |
| A0 | 0.055226744748529843 | 0.004950919743435842 | 0.009204457458088306 | **0.002126768857326232** |

Here B0/B1 include first actions E, random t2 regime, matching μ0/μ1 respectively, actor-0 dyad fine zero, and actor-0 t2 cooperation; all other events are integrated. The selected **C** probabilities are unequal, while **every L** must have `P_L(B0)=P_L(B1)` because, under the independently assigned random regime, the actor's stored L belief does not depend on the *current* partner. The numerical correlated two-atom L test produced event contrast 0 under each policy, as predicted.

**Why this constitutes an analytic all-H4 nonmembership witness:** For every L law in the entire ≤4-atom family, take the bounded observable function `1_{B0}−1_{B1}`. Its expectation is zero under L, and nonzero under the selected C law. Therefore the full law equality is impossible, and the TV lower bound follows from `|E_C h−E_L h| ≤ 2 TV(C,L)`. The all-H4 statement comes from the *random-matching independence proof*, **not** from numerically enumerating all unknown H4 mixtures.

## Evidence

- Merge baseline: PR #62, squash SHA `74d25d1bd3ee208a714da5f598457bbc7042d33b`.
- Execution branch `ops/exp003-cl-invariant-once-20261027` at `e1e15f268b1d50a27504267573b45d8e890a0352`.
- Actions artifact `exp003-cl-h4-random-regime-certificate`, ID **12721576499**, digest SHA-256 `15ad845e805b1a82828083c0d13db8ade525841e0404d494e1fa6079b087825a`, expiry **2026-11-26 08:53:03 UTC**. Archive includes the raw output, source checksum, pinned Cargo.lock and source commit.
- The JSON report adds provenance but is not byte-for-byte identical to the raw artifact output. Raw evidence is preserved in the Actions artifact.
- The one-off workflows remain only on operational branches; **no permanent CI workflow on main**. All new research executable artifacts are proposed through this PR.
- No data in this report may be used as a blinded classifier sample; it contains truth-family labels and mathematical witness parameters.

## Independent acceptance questions

Have an isolated mathematical reviewer verify (a) the full posterior conditioning on E and public random z2, (b) the t2 μ2 independence from latent type even under correlated H4 joint atoms, (c) correct dyad fine indexing and marginalization, (d) the `P_C^A(E)` two-matching-type formula, and (e) the full-likelihood versus conditional-marginal TV bound. Ask the reviewer to seek a counterexample from a legal L H4 representation. Only after this source-separated review should the candidate direction **C⊄L for T≥2** be added as independently accepted to the issue's 15-pair register. The reverse **L⊄C** question and all 95%-coverage acceptance remain open.

This is a narrow **synthetic identifiability result**, not a human sociology empirical finding.
