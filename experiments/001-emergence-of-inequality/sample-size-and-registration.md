# Experiment 001: exploratory precision design and registration gate

**Status: PRE-REGISTRATION DRAFT ONLY. No confirmatory seed draws, experiments, or preregistered claims.**

## Actual pilot data

Source: [verified Hiyal exploratory run](results/replicated-74000/README.md), \`hiyal-trajectories.csv\` (10 paired society seeds 74000–74009). For N=20, W0=10, offset=1, T=500 accepted opportunities, the paired terminal Gini differences are:

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

Approximate normal-theory calculations with z_0.975=1.959964 and z_0.90=1.281552:

- n for target two-sided 95% CI half-width 0.01: ceil((1.959964 × 0.0448353/0.01)²) = **78 paired societies**.
- n for approximate 90% two-sided 5%-alpha power for true delta 0.02: ceil(((1.959964+1.281552) × 0.0448353/0.02)²) = **53 paired societies**.
- **Candidate n=100** paired societies would imply approximate 95% half-width ~**0.00879** under the assumed SD. This is a *design heuristic*, not a guarantee or a verified simulation-based power analysis.

No calculation provides evidence of sociological generalization. The candidate 100-pair specification uses the **observed pilot horizon T=500**, rather than improperly carrying its variance estimate to the original draft's untested T=1000 horizon. If T=1000 is scientifically preferred, estimate variability independently at that horizon first.

The pure Rust \`design\` tool recomputes these values directly from terminal trajectory rows, pairs the actual shared uniform control by seed, detects duplicates, and emits machine-readable JSON.

\`\`\`sh
cargo run --locked --bin design -- \
  experiments/001-emergence-of-inequality/results/replicated-74000/hiyal-trajectories.csv
\`\`\`

## Confirmation *must* be withheld

Before conducting any confirmatory trial:
1. Decide whether the known urn is sufficiently interesting as a **calibration experiment**, not a novelty claim. Allocate scientific attention to a successor model with novel, falsifiable predictions.
2. Freeze exact code/runner, primary specification N=20/W0=10/b=1/T=500 or justified alternative, n, estimand, two-sided inference rule, distribution assumptions, missing-data policy, and failure handling.
3. Define an immutable independent seed generation method and held-out seed list, with no overlap with previous exploratory sets. Avoid unplanned peeking or adaptive stopping.
4. Verify full original-journal archival, exact trial identities, Hiyal/fork integrity, model hash, Cargo.lock, Rust version, analysis script checksum, and preservation policy.
5. Obtain independent design/analysis review and register a timestamped immutable protocol **before** executing any confirmatory data.
6. Conduct a prospective power simulation when considering more complex outcomes or non-Gaussian effects. Different society-level mechanisms may have substantially different variance.

A successful code test or positive bootstrap interval does not satisfy these gates.
