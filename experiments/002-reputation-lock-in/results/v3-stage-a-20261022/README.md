# Experiment 002 V3 Stage A — results (22 October 2026)

**Status:** exploratory synthetic-model evidence under the protocol and decision rule frozen in [v3-protocol-20261022.md](../../v3-protocol-20261022.md) before any V3 outcome. Not externally preregistered, not human data, not a sociological law.

## Design

Stage A is the three full-feedback turnover cells: cell 2 (m=2), cell 5 (m=10) and cell 8 (m=20). Each has 48 parents on the frozen seeds 130200–130247, 130500–130547 and 130800–130847. Each parent is a matched δ=0.2 / δ=0 pair with four intervention forks per side, at N=20, d=4, τ=200, T=600.

The runs were made on 2026-10-22 in a review container (Linux x86-64, Rust 1.89.0, 4 cores), from source `3b6ddb43bc451f7b708fca6202eb2b087675e7fe` with Hiyal `e38419f857afbb53428284a312b0eb926a2320e2`. They used `scripts/v3-grid-002.sh <dir> <cell> --jobs 4`. Each cell took 16–18 minutes and passed `scripts/verify-v3-grid-002.sh`. No seed failed or was replaced.

## Decisions under the frozen rule

δ=0.2 information-only fork, 95% t intervals over 48 parents (t = 2.011741), margin ±0.05:

| Cell | m | Late gap | New links + selection | Structural excess | Decision | δ=0 new links + selection |
| --- | ---: | --- | --- | --- | --- | --- |
| 2 | 2 | +0.155 (+0.104, +0.207) | −0.037 (−0.072, −0.003) | −0.029 (−0.065, +0.006) | **inconclusive** | +0.037 (+0.007, +0.068), flagged |
| 5 | 10 | +0.229 (+0.194, +0.264) | −0.014 (−0.034, +0.006) | −0.013 (−0.038, +0.011) | **slow relaxation** | −0.025 (−0.047, −0.002), flagged |
| 8 | 20 | +0.136 (+0.101, +0.172) | −0.007 (−0.023, +0.010) | −0.002 (−0.022, +0.017) | **slow relaxation** | −0.011 (−0.027, +0.005) |

**Stage A outcome, as frozen:**
- No cell shows regeneration; no new-link-plus-selection interval lies above zero.
- Cells 5 and 8 are slow relaxation.
- Cell 2 is inconclusive because its new-link-plus-selection interval lies entirely *below* zero. So the protocol's condition "every Stage A cell is classified slow relaxation" is **not met as written**, though nothing points to regeneration.

## Per-window decomposition

Means over 48 parents, δ=0.2 information-only fork. "Surviving fraction" is the share of checkpoint links never removed since τ; the uniform-removal prediction is (1 − 1/80)^⌊t/m⌋ averaged over the window.

| m | Window | Gap | Surviving-link exposure | Surviving fraction | Uniform prediction | Exposure per surviving link | New links + selection |
| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 2 | 201–300 | 0.556 | 0.596 | 0.748 | 0.742 | 0.797 | −0.032 |
| 2 | 301–400 | 0.314 | 0.352 | 0.437 | 0.396 | 0.804 | −0.044 |
| 2 | 401–500 | 0.208 | 0.240 | 0.296 | 0.211 | 0.812 | −0.036 |
| 2 | 501–600 | 0.155 | 0.188 | 0.225 | 0.112 | 0.834 | −0.037 |
| 10 | 201–300 | 0.326 | 0.314 | 0.944 | 0.944 | 0.333 | −0.006 |
| 10 | 301–400 | 0.234 | 0.273 | 0.834 | 0.833 | 0.327 | −0.008 |
| 10 | 401–500 | 0.218 | 0.246 | 0.741 | 0.734 | 0.332 | −0.006 |
| 10 | 501–600 | 0.229 | 0.222 | 0.664 | 0.648 | 0.334 | −0.014 |
| 20 | 201–300 | 0.179 | 0.179 | 0.974 | 0.975 | 0.184 | −0.006 |
| 20 | 301–400 | 0.183 | 0.166 | 0.914 | 0.915 | 0.182 | −0.007 |
| 20 | 401–500 | 0.166 | 0.159 | 0.858 | 0.859 | 0.185 | −0.002 |
| 20 | 501–600 | 0.136 | 0.152 | 0.805 | 0.807 | 0.189 | −0.007 |

## Interpretation

- **Structural memory at every turnover rate.** In each cell, the exposure carried by each surviving checkpoint link stays nearly constant across the four windows:
  - m=10: 0.327–0.334;
  - m=20: 0.182–0.189;
  - m=2: 0.797–0.834, rising about 5%.
  
  So the post-correction gap is carried by checkpoint links still present, and it shrinks as they are replaced.
- **Link survival.** At m=10 and m=20, surviving fractions match uniform removal closely. At m=2, checkpoint links survive longer than uniform removal predicts (0.225 against 0.112 in the last window); lowest-belief removal is not uniform when rewiring is frequent.
- **No regeneration.** New links plus selection is never positive in any window or cell.
- **Fast turnover goes slightly the other way.** At m=2, links formed after correction favour the unboosted group: new-link exposure is about −0.06 per window, partly offset by selection at about +0.02. That compensates for the inherited advantage rather than rebuilding it. Its size (about 0.04) is comparable to the null-control deviations below, so treat it cautiously.
- **Per-link exposure depends on turnover.** It is larger the faster the turnover: 0.80 at m=2, 0.33 at m=10, 0.19 at m=20. More rewiring before τ lets the false prior reshape more of the checkpoint graph, so each surviving link carries more of the bias. This is why the protocol requires each cell's own constant.
- **Null controls.**
  - With δ=0 nothing reads the labels, and labels are balanced and uniformly assigned. So the expected value is exactly zero, and its distribution is symmetric about zero.
  - Two of three controls were nonetheless flagged, with opposite signs. At m=2 the flag is driven by a right tail (one parent at +0.345; signs 27 positive, 21 negative). At m=10 the signs are 16 positive, 32 negative (sign test p ≈ 0.03).
  - The chance of two or more false flags among three independent controls is about 0.7%. So the nominal t intervals look somewhat optimistic, especially at m=2, where the null sd is 0.105.
  - As frozen, these flags call for investigation, not exclusion. The exact sign test below flags only m=10.

## Post hoc robustness under Amendment 1

[Amendment 1](../../v3-protocol-20261022.md#amendment-1--adopted-2026-10-22-after-stage-a-and-before-any-stage-b-run) was adopted after these outcomes were seen, and before any Stage B run. It adds a **compensation** category (`nps` interval entirely below 0) and distribution-free checks, and it decides Stage B. Applied here to the same committed cell summaries, it is **descriptive only**: the Stage A decisions above stay the frozen rule's.

| Cell | m | t rule | Median rule: `nps` median (95% order-statistic interval) | `nps` signs, sign-test p | Control-variate `nps` | δ=0 signs, sign-test p |
| --- | ---: | --- | --- | --- | --- | --- |
| 2 | 2 | compensation; mean −0.037, not beyond −0.05 | inconclusive: −0.064 (−0.092, +0.006) | 18+ / 30−, p = 0.11 | −0.040 (−0.077, −0.003) | 27+ / 21−, p = 0.47 |
| 5 | 10 | slow relaxation | slow relaxation: −0.006 (−0.038, +0.015) | 20+ / 28−, p = 0.31 | −0.007 (−0.027, +0.014) | 16+ / 32−, p = 0.029, **flagged** |
| 8 | 20 | slow relaxation | slow relaxation: −0.001 (−0.022, +0.015) | 23+ / 25−, p = 0.89 | −0.000 (−0.015, +0.014) | 22+ / 26−, p = 0.67 |

The median intervals are [x₍₁₇₎, x₍₃₂₎] of the 48 parents, with 97.1% coverage. The control variate is the intercept of the least-squares fit of `nps` on the same parent's δ=0 `nps`, with a t interval at 46 df. At m=2 the `nps` t intervals lie below zero in all four windows: (−0.043, −0.021), (−0.063, −0.024), (−0.068, −0.005) and (−0.072, −0.003). These windows share parents, so this shows stability, not replication.

Reading, post hoc:
- **No regeneration.** No cell is regeneration under either rule, so Stage A shows no active reputation lock-in.
- **m=10 and m=20 are robust slow relaxation.** The control variate brings their `nps` closer to zero: from −0.014 to −0.007, and from −0.007 to −0.000.
- **The m=2 compensation is not robust.** Its median interval reaches above 0, and its sign test is not significant: 18 positive against 30 negative parents is not distinguishable from an even split. Its median, −0.064, is further from zero than its mean, so the t-rule verdict is not driven by negative outliers.
- **One null flag under the sign test.** Only m=10's null control is flagged. At δ=0 every quantity is exactly symmetric about 0, so the sign test is exact there. The m=2 t flag came from its right tail, and the sign test does not reproduce it. With three exact tests of size 2.9%, at least one flag happens by chance about 8.5% of the time.
- **The frozen Stage A outcome stands:** not every cell is slow relaxation, and none shows regeneration.

## Files and preservation

Committed per cell:
- `batch-plan.json`: the frozen design plus source and Hiyal revisions;
- `decision.json`: the frozen rule's output;
- `SHA256SUMS`: the full evidence seal;
- `cell-summaries/parent-<seed>.json`: each parent's derived summary.

`stage-a-summary.json` collects the three decisions and the seal digests. `stage-a-amendment-1-post-hoc.json` holds the post hoc Amendment 1 analysis above. CI runs `scripts/check-v3-stage-a-002.sh`, which ties every committed file to its cell's `SHA256SUMS` entry, re-derives each frozen decision and the post hoc analysis from the committed cell summaries, and checks `stage-a-summary.json`. It runs no simulation.

| Cell | SHA256SUMS digest |
| --- | --- |
| 2 | `d0037ac3e583b9597969ac7e96d5790d1897eb749f29d1180528d1fca86144d0` |
| 5 | `979dd9c1b96b559cf6efa14d67682b6a374710ce1058a07eeb461d4b6e106366` |
| 8 | `29dcc642e29f945ceb43513da953185c50669e6131d1b5904b3bcac2391ccf25` |

**The original raw Hiyal evidence (about 3.2 GB) is preserved, compressed, in the [GitHub Release `experiment-002-v3-raw-evidence-20261022`](https://github.com/davletovb/social-emergence-lab/releases/tag/experiment-002-v3-raw-evidence-20261022).** The original run directories were packed on 2026-10-22, and all nine cells were replayed with `verify-v3-grid-002.sh` before publication; see the [raw-evidence custody record](../v3-raw-evidence-20261022/README.md). The evidence also regenerates from source `3b6ddb4`, which must stay reachable from `main`:

```bash
bash scripts/v3-grid-002.sh /absolute/new/v3-grid-cell-5 5 --jobs 4
cmp /absolute/new/v3-grid-cell-5/SHA256SUMS experiments/002-reputation-lock-in/results/v3-stage-a-20261022/cell-5/SHA256SUMS
```

- **Cell 2 regenerated byte for byte.** On 2026-10-22 it was regenerated this way from a clean checkout of `3b6ddb4`, in a fresh directory in the same container. All 3,266 sealed files matched: its `SHA256SUMS` is byte-identical to the committed one (digest `d0037ac3…`).
- **Cells 5 and 8** have not been regenerated.
- **Amended verifier.** The original cell 2 batch also passes `verify-v3-grid-002.sh --digest` from the Amendment 1 commit, which applies the frozen rule to Stage A.
- **Storage:** the packed originals are in the release above, at the repository-retention level; a DOI deposit is still desirable.
