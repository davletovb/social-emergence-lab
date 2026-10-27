# Experiment 003 — reverse-C H4 witness, v0.2 review correction

**Date:** 2026-10-27 UTC · **Status:** author-derived synthetic model-law mathematics and executed author-side checks, **NOT independently accepted R-06 P0**.

## Reviewer finding and correction

The review of [PR #64](https://github.com/davletovb/social-emergence-lab/pull/64#pullrequestreview-5666829989) verified the earlier N/S/L and R(T≥3) arguments and observed that R needs **only one reward update**, not two, to violate the all-H4 C ceiling. The prior T3 witness and its [original machine-readable evidence](reverse-C-H4-witnesses-20261027.json) are retained as **historical but nonminimal**; no past run is relabelled as a T2 test. The full updated derivation is in [the research report](../reverse-C-H4-witnesses-20261027.md#2a-review-correction-r--c-already-at-t2).

## New R not subset C witness at minimum T=2

- Selected truth: legal homogeneous one-atom R, `β=1, ε=0, α_R=1, λ_A=0, η=0`, all initial Q/aspirations zero.
- Positive-probability public T1 record: `z=random, μ0=01|23, fine bits 00, actions [1,1,0,0]`.
- Actor 0 cooperates with a cooperator for payoff 1, updating `Q_{0,2}(1,0): 0→1` after T1; it does not update `Q_{0,2}(0,0)`; aspiration remains 0.
- The public T2 condition `D_2` requires `z=random, μ1=02|13, actor0 fine zero`. Partner actor 2 **defected** at T1, so under any C H4 rival, even after arbitrary latent mixture posterior selection, `p_C(a_{0,2}=1|D_2)≤c*=0.25+0.5σ(−0.1)=0.48751040626053`.
- Under selected R, the same conditional actor0 cooperation is `σ(−1+1)=0.5>c*`. All other T2 fines/actions are marginalized.
- Thus the **public** dual function `h=1_{D_2∩{a_{0,2}=1}}−c*1_{D_2}` has oscillation 1, satisfies `E_C[h]≤0` for **all** C H4 rival laws, and yields `TV(P_R,P_C)≥P_R(D_2)(0.5−c*)` without free-parameter optimization.

| Policy | P_R(D₂) | TV lower bound versus every legal C H4 |
| --- | ---: | ---: |
| A | 0.00013422309823941982 | **0.0000016763919674633255** |
| A0 | 0.004295139143661434 | **0.000053644542958826415** |

The original two-update T3 witness is still valid but strictly weaker: bounds `6.69500298e−8` under A and `1.71392076e−5` under A0. **The minimum horizon now matches N/S/L: T≥2.** Combined with merged PR #63's converse C-not-subset results, the candidate C/R family relationship is bidirectional noninclusion at T≥2, **not disjointness**, since overlapping parameterized laws remain.

## Strengthened source checks

The revised stdlib [Python file](../../../tools/exp003-exact/check_reverse_c_invariants.py) models **all four actors**, derives the entire first-period and second-period action-vector probabilities, computes every actor's matching-specific physical payoff, updates contextual Q and aspiration using the *previous* aspiration, and checks fine/dyad/action indexing. It no longer simply types in the second-period action mass. It checks the N/S/L one-atom embeddings across multiple fine/history cells instead of tautological numerical identities.

A new [Rust conformance binary](../../../tools/exp003-exact/src/bin/reverse_c.rs) constructs the same legal one-atom R world in the existing v0.2.2 simulator, applies both public records through the actual model transition, checks the actor0 probabilities at T2 and T3, and reproduces the public-event probability/bound separately from the Python algebra.

**Execution:** an initial tiny CI run [#40503052644](https://github.com/davletovb/social-emergence-lab/actions/runs/40503052644) failed at Rust compilation due to a test syntax typo (`.25` rather than `0.25`), which was corrected. The subsequent targeted [run #40503316441](https://github.com/davletovb/social-emergence-lab/actions/runs/40503316441) **passed 3 Rust and 6 Python tests**, independently ran both languages, and programmatically compared all selected R witness masses/bounds. Rust and Python output SHA-256 respectively:

- `40e4126844ff6250ba7ab5424c54701ad9b03f94a701ed00e3d1033b622e5cf9`
- `c5467b3688b976e2402b0317ed4b924dec556541601f852d4c7e8e6fed7816`

The compact [Actions artifact](https://github.com/davletovb/social-emergence-lab/actions/runs/40503316441) has ID `12732939351` and ZIP SHA-256 `294a1e8833e7b1646bed5ebfd693fc6cf98b13ea494180ff25afaeb0377fba4b`, expiring **2026-11-26 12:23:34 UTC**. The [v0.2 JSON](reverse-C-H4-witnesses-v02-20261027.json) preserves the corrected values and test provenance; the *raw* outputs remain in the evidence bundle.

## Scientific disposition

The neutral [review checklist](../../../docs/research/r06/reverse-C-review-checklist-20261027.md) now asks an isolated verifier to **derive the minimum legal horizon** rather than anchoring them on the author's superseded T≥3 statement. The general `TV≥|E_P h−E_Q h|/2` remains mathematically true but loose for oscillation-one `h`; the sharper oscillation-one bound is correctly used.

**The five review points are addressed in the PR proposal.** Source-isolated independent mathematics review, unrestricted rival-family proof acceptance, H4 confidence-set/95%-coverage calibration and human-behavior interpretation remain separate and unfinished. No new full CI workflow is added to `main`, and no merge is requested in this PR.
