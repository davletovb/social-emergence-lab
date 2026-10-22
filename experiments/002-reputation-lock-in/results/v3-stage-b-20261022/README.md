# Experiment 002 V3 Stage B — results (22 October 2026)

**Status:** exploratory synthetic-model evidence, decided by the rule of [Amendment 1](../../v3-protocol-20261022.md#amendment-1--adopted-2026-10-22-after-stage-a-and-before-any-stage-b-run). That rule was adopted after Stage A and committed before any Stage B parent ran. Not externally preregistered, not human data, not a sociological law.

## Design

Stage B is the six cells without reputation-preferred addition (κ=0):
- **random turnover** (uniform link removal): cell 0 (m=2), cell 3 (m=10), cell 6 (m=20);
- **belief deletion** (the lowest-belief neighbour is removed): cell 1 (m=2), cell 4 (m=10), cell 7 (m=20).

Each cell has 48 parents on the frozen seeds 130000 + 100·cell + 0…47. Each parent is a matched δ=0.2 / δ=0 pair with four intervention forks per side, at N=20, d=4, τ=200, T=600.

The runs were made on 2026-10-22 in a review container (Linux x86-64, Rust 1.89.0, 4 cores), from source `defb96ee73f4fe8a5258fc7178166e7bc7961b49` with Hiyal `e38419f857afbb53428284a312b0eb926a2320e2`. They used `scripts/v3-grid-002.sh <dir> <cell> --jobs 4`, one cell after another, 07:36–09:26 UTC. Each cell took 18–19 minutes and passed `scripts/verify-v3-grid-002.sh`. No seed failed or was replaced.

**One aborted launch.** A first launch at 07:35 UTC was stopped about 30 seconds in, to detach the run from a session time limit. At that point cell 0 had started 4 parents and none had finished or written a report. That partial directory was set aside unread, and Stage B was then run in full from the start on the same frozen seeds.

## Decisions under Amendment 1

δ=0.2 information-only fork, 95% t intervals over 48 parents (t = 2.011741), margin ±0.05. The median rule uses the exact order-statistic interval [x₍₁₇₎, x₍₃₂₎] of the 48 parents (coverage 97.1%).

| Cell | Mechanism | m | Late gap | New links + selection (`nps`) | Structural excess | t rule | Median rule |
| --- | --- | ---: | --- | --- | --- | --- | --- |
| 0 | random turnover | 2 | +0.010 (−0.028, +0.049) | −0.012 (−0.039, +0.015) | −0.018 (−0.046, +0.010) | slow relaxation | inconclusive |
| 3 | random turnover | 10 | +0.000 (−0.039, +0.039) | +0.004 (−0.018, +0.026) | −0.002 (−0.031, +0.027) | slow relaxation | inconclusive |
| 6 | random turnover | 20 | +0.017 (−0.016, +0.050) | +0.003 (−0.014, +0.020) | −0.001 (−0.019, +0.017) | slow relaxation | slow relaxation |
| 1 | belief deletion | 2 | +0.113 (+0.074, +0.151) | −0.046 (−0.069, −0.023) | −0.049 (−0.075, −0.024) | **compensation** | **compensation** |
| 4 | belief deletion | 10 | +0.124 (+0.084, +0.165) | −0.017 (−0.036, +0.002) | −0.024 (−0.047, −0.001) | slow relaxation | slow relaxation |
| 7 | belief deletion | 20 | +0.080 (+0.047, +0.112) | −0.001 (−0.016, +0.015) | +0.001 (−0.019, +0.021) | slow relaxation | slow relaxation |

Robustness checks (`nps` unless stated):

| Cell | Median (interval) | Signs, sign-test p | Control-variate `nps` | δ=0 signs, sign-test p |
| --- | --- | --- | --- | --- |
| 0 | −0.013 (−0.065, +0.013) | 17+ / 31−, p = 0.06 | degenerate (forks identical) | 17+ / 31−, p = 0.06 |
| 3 | +0.001 (−0.031, +0.028); excess (−0.0503, +0.027) | 25+ / 23−, p = 0.89 | degenerate (forks identical) | 25+ / 23−, p = 0.89 |
| 6 | +0.021 (−0.020, +0.035) | 27+ / 21−, p = 0.47 | degenerate (forks identical) | 27+ / 21−, p = 0.47 |
| 1 | −0.062 (−0.095, −0.016) | 13+ / 35−, p = 0.002 | −0.037 (−0.054, −0.021) | 19+ / 29−, p = 0.19 |
| 4 | −0.013 (−0.039, +0.009) | 18+ / 30−, p = 0.11 | −0.018 (−0.029, −0.008) | 26+ / 22−, p = 0.67 |
| 7 | −0.001 (−0.018, +0.031) | 23+ / 25−, p = 0.89 | −0.007 (−0.016, +0.002) | 31+ / 17−, p = 0.06 |

**Stage B outcome under Amendment 1:**
- **No regeneration.** No cell is regeneration under either rule, so Stage B shows no active reputation lock-in.
- **Belief deletion:** m=10 and m=20 are robust slow relaxation. m=2 is **robust compensation**: both rules agree, the sign test gives p = 0.002, and the `nps` t interval is below zero in all four windows. Its mean, −0.046, is not beyond the −0.05 margin.
- **Random turnover:** m=20 is robust slow relaxation. At m=2 and m=10 the t rule gives slow relaxation but the median rule does not: median intervals reach past −0.05: both `nps` and `excess` at m=2, and `excess` at m=10, by 0.0003. As the next section shows, these cells carry no false-prior effect at all.
- **Null controls:** no control is flagged by either the sign test or the t interval.

## Random turnover: the correction removes the effect completely

In all 144 random-turnover parents, the δ=0.2 information-only fork's summary is **identical** to the δ=0 one, in every window and every quantity, including the gap. Per parent, the late false-prior-minus-null gap is exactly 0.

- **Why.** Random turnover removes and adds links without reading beliefs, so the false prior never shapes the graph. At the checkpoint the two sides have the same graph and the same random-stream positions; only beliefs and wealth differ, and wealth does not affect the dynamics. The information reset sets every belief to 0.5 on both sides, so from τ onwards the two forks run identically.
- **What it means.** In these cells the "δ=0.2" quantities are null quantities, and the δ=0 control is the same data, not an independent check. Their slow-relaxation label means there is no effect left to relax. The protocol's description of the two sides as "independently evolving" histories does not hold for these forks after the reset.
- **The control variate is degenerate there.** The fit is exact (coefficient 1, zero residual), so its interval [0, 0] is not informative.
- **Margin calibration.** Under the median rule, two of these pure-null cells miss the ±0.05 margin, at m=2 and m=10, so the margin is tight for the spread of an exact null at fast turnover.

## Per-window decomposition

Means over 48 parents, δ=0.2 information-only fork. "Surviving fraction" is the share of checkpoint links never removed since τ. The uniform-removal prediction is (1 − 1/80)^⌊t/m⌋ averaged over the window, with t = event − 200. "Exposure per surviving link" is surviving-link exposure ÷ surviving fraction.

| Mechanism | m | Window | Gap | Surviving-link exposure | Surviving fraction | Uniform prediction | Exposure per surviving link | New links + selection |
| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| random turnover | 2 | 201–300 | −0.006 | −0.015 | 0.743 | 0.742 | −0.020 | −0.008 |
| random turnover | 2 | 301–400 | −0.016 | −0.014 | 0.393 | 0.396 | −0.036 | −0.004 |
| random turnover | 2 | 401–500 | −0.023 | −0.008 | 0.206 | 0.211 | −0.038 | −0.013 |
| random turnover | 2 | 501–600 | +0.010 | −0.007 | 0.111 | 0.112 | −0.066 | −0.012 |
| random turnover | 10 | 201–300 | +0.011 | +0.002 | 0.945 | 0.944 | +0.002 | +0.002 |
| random turnover | 10 | 301–400 | −0.021 | +0.003 | 0.834 | 0.833 | +0.003 | +0.001 |
| random turnover | 10 | 401–500 | −0.024 | −0.002 | 0.732 | 0.734 | −0.003 | +0.004 |
| random turnover | 10 | 501–600 | +0.000 | −0.005 | 0.647 | 0.648 | −0.008 | +0.004 |
| random turnover | 20 | 201–300 | +0.024 | +0.012 | 0.975 | 0.975 | +0.012 | +0.000 |
| random turnover | 20 | 301–400 | +0.005 | +0.009 | 0.915 | 0.915 | +0.010 | −0.000 |
| random turnover | 20 | 401–500 | −0.006 | +0.006 | 0.860 | 0.859 | +0.007 | +0.000 |
| random turnover | 20 | 501–600 | +0.017 | +0.006 | 0.808 | 0.807 | +0.007 | +0.003 |
| belief deletion | 2 | 201–300 | 0.416 | 0.449 | 0.748 | 0.742 | 0.600 | −0.022 |
| belief deletion | 2 | 301–400 | 0.228 | 0.262 | 0.436 | 0.396 | 0.600 | −0.045 |
| belief deletion | 2 | 401–500 | 0.128 | 0.169 | 0.286 | 0.211 | 0.591 | −0.045 |
| belief deletion | 2 | 501–600 | 0.113 | 0.125 | 0.213 | 0.112 | 0.585 | −0.046 |
| belief deletion | 10 | 201–300 | 0.200 | 0.181 | 0.945 | 0.944 | 0.192 | −0.004 |
| belief deletion | 10 | 301–400 | 0.117 | 0.161 | 0.833 | 0.833 | 0.193 | −0.009 |
| belief deletion | 10 | 401–500 | 0.128 | 0.142 | 0.738 | 0.734 | 0.193 | −0.012 |
| belief deletion | 10 | 501–600 | 0.124 | 0.119 | 0.658 | 0.648 | 0.182 | −0.017 |
| belief deletion | 20 | 201–300 | 0.076 | 0.105 | 0.975 | 0.975 | 0.107 | +0.000 |
| belief deletion | 20 | 301–400 | 0.087 | 0.096 | 0.914 | 0.915 | 0.105 | +0.002 |
| belief deletion | 20 | 401–500 | 0.082 | 0.090 | 0.859 | 0.859 | 0.104 | +0.001 |
| belief deletion | 20 | 501–600 | 0.080 | 0.088 | 0.808 | 0.807 | 0.109 | −0.001 |

At random turnover, exposure per surviving link divides a near-zero exposure by the surviving fraction, so it is noise, largest where few links survive.

## Interpretation

- **Survival matches the uniform hazard under random turnover.** The surviving fraction is within 0.005 of the prediction in every window at every m. This is the check the protocol asked for in random-turnover cells.
- **Belief deletion shows the same structural memory as full feedback.** In each cell the exposure per surviving checkpoint link stays nearly constant across the four windows: 0.585–0.600 at m=2, 0.182–0.193 at m=10, 0.104–0.109 at m=20. The gap shrinks as those links are replaced. These per-link values are below full feedback's (0.80 / 0.33 / 0.19 in Stage A), presumably because preferred addition before τ concentrates more of the bias in the checkpoint graph.
- **Link survival under belief deletion.** At m=2, checkpoint links survive longer than uniform removal predicts (0.213 against 0.112 in the last window), as in full feedback; at m=10 slightly (0.658 against 0.648); at m=20 they match.
- **Fast turnover with belief deletion compensates.** At m=2, links formed after correction favour the unboosted group, robustly, at about −0.045 per window from 301–400 on. A plausible mechanism, not tested here: a formerly favoured actor is already a neighbour of many actors, so the links that can still be added are disproportionately to the others. Full feedback at m=2 pointed the same way in Stage A, but not robustly.
- **No regeneration without reputation-preferred addition**, as expected: nothing positive appears in any cell or window.

## Whole grid (Stages A and B)

Under the frozen rule for Stage A and Amendment 1 for Stage B, no cell of the nine shows regeneration, under the t rule or the median rule. The post-correction gap is carried by checkpoint links still present, at a near-constant exposure per surviving link, and it fades as they are replaced:

| Mechanism | m=2 | m=10 | m=20 |
| --- | --- | --- | --- |
| full feedback (Stage A) | inconclusive as frozen; compensation, not robust (post hoc) | slow relaxation | slow relaxation |
| belief deletion | compensation, robust | slow relaxation | slow relaxation |
| random turnover | no effect left after correction | no effect left after correction | no effect left after correction |

This supports **structural persistence** over **active reputation lock-in** in this model. Stage A's frozen condition ("every Stage A cell is classified slow relaxation") remains unmet as written, because of the m=2 cell. Item 3 of the protocol's review (independent theory and method review, empirical calibration) is still required before any paper.

## Files and preservation

Committed per cell:
- `batch-plan.json`: the frozen design, which names Amendment 1's rule, plus source and Hiyal revisions;
- `decision.json`: the rule's output;
- `SHA256SUMS`: the full evidence seal;
- `cell-summaries/parent-<seed>.json`: each parent's derived summary.

`stage-b-summary.json` is derived from these by `scripts/check-v3-stage-b-002.sh --write`. In CI, that script runs no simulation. It ties every committed file to its cell's `SHA256SUMS` entry, checks each plan against `reputation-v3 plan`, re-derives every decision under the plan's rule, and re-derives the stage summary.

| Cell | SHA256SUMS digest |
| --- | --- |
| 0 | `75d1eee90e6e88651580c0f4cf9e05ecf0619bff647534fd24cde7f7651e5cc0` |
| 1 | `47ec0e8c9510aa78a444b9a1240755383ad81a45cf23a651d1fe707bf1ace847` |
| 3 | `90e1b06b6b3a421487ed3dca3973c7d50261e23a8af3f3d553d8bc0d5a18aea6` |
| 4 | `498803a38dc8a82c6a3086409940b636834291f2b38f850d1e7bf68bcacb453f` |
| 6 | `4990fccad7ace0b49ed2f9d3da15b24bfdcd47ee6c988f54355ddf13cb88709b` |
| 7 | `e620eeee6927dc9fc95477ffac7efdfd63a37fbaf0ad111bd24b4d6ac77be668` |

**The raw Hiyal evidence (about 6.3 GB) is not yet in durable storage.** It was produced in an ephemeral review container. The original run directories were packed and checked on 2026-10-22, and the bundle is held by the repository owner; see the [raw-evidence custody record](../v3-raw-evidence-20261022/README.md). Stage A cell 2 regenerated byte for byte, so these cells should also regenerate from source `defb96e`, which must stay reachable from `main`:

```bash
bash scripts/v3-grid-002.sh /absolute/new/v3-grid-cell-1 1 --jobs 4
cmp /absolute/new/v3-grid-cell-1/SHA256SUMS experiments/002-reputation-lock-in/results/v3-stage-b-20261022/cell-1/SHA256SUMS
```

No Stage B cell has been regenerated yet. Durable storage of the packed originals is still outstanding (Issue #21).
