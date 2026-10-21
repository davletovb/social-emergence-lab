# Experiment 001 — Emergence of inequality

**Status:** design draft. No simulations or statistical findings yet.

## Research question
Can cumulative advantage amplify small stochastic opportunities into persistent inequality even when agents begin with identical resources and have identical intrinsic abilities?

## Main hypothesis
H1 (confirmatory candidate): with positive reinforcement, the mean **terminal Gini coefficient of nonnegative wealth** across independent societies is greater than under an otherwise identical non-reinforcing allocation process. Operational details and statistical thresholds must be frozen before confirmatory runs.

H0: no difference between arms in the model-defined terminal Gini estimand.

Secondary predictions (exploratory until frozen): reinforcement increases rank persistence and decreases mobility; increasing feedback produces nonlinear changes. The prediction that reducing reputation memory weakens lock-in requires a reputation/memory variant and is out of scope for V0.

## Model skeleton (to implement)
- N = 20 agents, each initialized with the same positive integer wealth W0; intrinsic abilities fixed and equal.
- At each discrete step one opportunity token enters the society. Draw a recipient by weighted choice. Every draw increases that actor's wealth by one integer token. Therefore total wealth rises by exactly one per accepted opportunity.
- Every agent is eligible at every step and every draw is accepted, so in V0 attempts equal accepted opportunities.
- Control: all agents have equal recipient probabilities regardless of history.
- Treatment: weight for actor i equals (wealth_i + offset)^alpha, where offset > 0 and alpha > 0. Alpha = 0 recovers the uniform model. Explicit finite-integer or otherwise numerically stable normalization is required.
- No endogenous production, debt, redistribution, mutation or agent entry/exit in V0. This simplified mechanism is **not** a realistic wealth economy.
- Use dedicated, named RNG streams for opportunities; determine whether draw coupling remains comparable across arms. Treatment draws may diverge after initial opportunities.
- Record wealth per actor periodically and at termination; terminal Gini is zero for perfect equality and undefined for zero-total wealth (excluded by positive W0).

## Illustrative pilot design — not preregistered
- 2 arms: alpha = 0 (control), alpha = 1 (treatment); same N, W0 and number of accepted opportunities.
- 100–500 independent society seeds per arm, paired by base seed; 1,000 opportunities per trial for initial feasibility.
- Pilot benchmarks first; consider more seeds and longer horizons based on effective variance, model stability and computational cost.
- Reserve independent seeds for subsequent confirmatory runs. Do not select only favorable horizons or feedback parameters after looking at outcomes.

## Outcome measures
**Primary:** per-society terminal Gini of wealth at a prespecified horizon; contrast mean treatment minus mean control across independent societies. Choose and freeze the estimator (paired by base seed or two-sample) and its interval method before confirmatory runs.

**Secondary:** rank persistence (Spearman correlation between a prespecified early ranking and final ranking, accounting for ties); top-quintile wealth share; mean absolute rank change; full inequality trajectories.

Include an exact equality baseline at initial time; avoid defining rank persistence at time zero when all agents are tied.

## Threats and limitations
Increasing total resources mechanically affects attainable inequality as initial endowments are diluted; compare arms only at identical accepted-event counts. Feedback here is defined by wealth rather than social status, and empirical stratification requires external calibration. Distinct draw consumption may weaken variance reduction from paired seeds. Outcomes are dependent within each society.

## Acceptance conditions
- All tests in ../../docs/methodology.md represented in executable validation.
- Verified per-run manifests and agent-level data exports with hashes.
- Held-out confirmatory seed list and frozen hypothesis/metric implementation.
- Re-run subset on restart yields byte-identical model outputs under pinned environment.
- Write-up includes uncertainty intervals, sensitivity analyses, adverse/null cases and scope limits.
