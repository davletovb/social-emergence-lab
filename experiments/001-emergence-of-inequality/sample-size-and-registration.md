# Experiment 001: exploratory precision design and registration gate

**Status: PRE-REGISTRATION DRAFT ONLY. No confirmatory seed draws, experiments, or preregistered claims.**

## Actual pilot data

Source: [verified Hiyal exploratory run](results/replicated-74000/README.md), `hiyal-trajectories.csv` (10 paired society seeds 74000–74009). For N=20, W0=10, offset=1, T=500 accepted opportunities, the paired terminal Gini differences are:

| Society seed | Gini treatment − uniform control |
|:--|--:|
| 74000 | +0.063857142858 |
| 74001 | +0.034000000000 |
| 74002 | +0.010571428572 |
| 74003 | +0.070857142857 |
| 74004 | +0.075142857143 |
| 74005 | +0.092000000000 |
| 74006 | +0.059714285715 |
| 74007 | +0.058857142858 |
| 74008 | +0.053428571428 |
| 74009 | +0.062857142857 |

Mean paired difference **+0.058128571429**; sample SD of independent paired differences **0.022417644694**; sample SE ~0.007089. This pilot SD estimate is from **only ten societies** and is highly uncertain; it must not be treated as ground truth.

## Conservative planning exercise (not a promise of power)

Double the observed **standard deviation** (factor 2.0) for stress-testing: assumed SD=**0.044835289389**. This quadruples the variance; it is a deliberately conservative planning scenario, not a formal upper confidence bound. Let a minimally interesting within-model difference be **0.02 Gini units** and target 95% confidence interval half-width **0.01**.

The design is **paired**: each seed's control and treatment are forked from the same parent, and the estimand is the mean of the per-society differences, whose pilot SD is used above. Sizes therefore come from the one-sample t distribution of those differences with n − 1 degrees of freedom (the SD is estimated, so normal z values understate n):

- n for target two-sided 95% CI half-width 0.01: smallest n with t_0.975,n−1 × 0.0448353/√n ≤ 0.01 → **80 paired societies** (half-width 0.00998; normal theory would give 78, whose t half-width is 0.0101).
- n for 90% power of a two-sided 5%-alpha paired t-test when the true delta is 0.02: exact noncentral-t power → **55 paired societies** (power 0.901; normal theory would give 53, whose power is 0.890).
- **Candidate n=100** paired societies would imply a 95% half-width of ~**0.00890** under the assumed SD. This is a *design heuristic*, not a guarantee or a verified simulation-based power analysis. The draft protocol's candidate analysis is a paired percentile bootstrap, which at these sizes behaves like the normal interval and tends to be slightly narrower than t, so planning with t is the safer choice.

No calculation provides evidence of sociological generalization. The candidate 100-pair specification uses the **observed pilot horizon T=500**, rather than improperly carrying its variance estimate to the original draft's untested T=1000 horizon. If T=1000 is scientifically preferred, estimate variability independently at that horizon first.

The pure Rust `design` tool recomputes these values directly from terminal trajectory rows, pairs the actual shared uniform control by seed, detects duplicates, and emits machine-readable JSON. A unit test pins the values above against the archived trajectories, and CI checks the tool's sample sizes. Flags (`--actors`, `--initial`, `--offset`, `--horizon`, `--sd-multiplier`, `--half-width`, `--mde`, `--illustrative-n`) re-plan for another pilot specification without editing source.

```sh
cargo run --locked --bin design -- \
  experiments/001-emergence-of-inequality/results/replicated-74000/hiyal-trajectories.csv
```

## Confirmation *must* be withheld

Before conducting any confirmatory trial:
1. Decide whether the known urn is sufficiently interesting as a **calibration experiment**, not a novelty claim. Allocate scientific attention to a successor model with novel, falsifiable predictions.
2. Freeze exact code/runner, primary specification N=20/W0=10/b=1/T=500 or justified alternative, n, estimand, two-sided inference rule, distribution assumptions, missing-data policy, and failure handling.
3. Define an immutable independent seed generation method and held-out seed list, with no overlap with previous exploratory sets. Avoid unplanned peeking or adaptive stopping.
4. Verify full original-journal archival, exact trial identities, Hiyal/fork integrity, model hash, Cargo.lock, Rust version, analysis script checksum, and preservation policy.
5. Obtain independent design/analysis review and register a timestamped immutable protocol **before** executing any confirmatory data.
6. Conduct a prospective power simulation when considering more complex outcomes or non-Gaussian effects. Different society-level mechanisms may have substantially different variance.

A successful code test or positive bootstrap interval does not satisfy these gates.
