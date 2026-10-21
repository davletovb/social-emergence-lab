# Experiment 001: exact analytical baseline (known Pólya urn)

**Classification:** established mathematical mechanism, not a new sociological discovery. The purpose of this model is to validate Hiyal event sourcing, policy sampling, paired trials, rank/inequality metrics, and reproducibility.

## Mathematical equivalence

There are N agents with identical wealth W0 and a positive fixed offset b. Under treatment, the probability of receiving the next unit of wealth is:

P(i next | state) = (wealth_i + b) / sum_j(wealth_j + b).

Every draw increments the recipient's wealth by exactly one; b remains constant. With initial concentration a = W0 + b, this is precisely a symmetric Pólya urn, equivalent to Dirichlet–multinomial allocations after a fixed T opportunities. The uniform arm is a multinomial control.

The expected *additional* wealth per agent is T/N in both arms. A known exact result for each agent's additional wealth X_i is:

- **Uniform:** Var(X_i) = T × (1/N) × (1 − 1/N).
- **Linear reinforcement:** Var(X_i) = [uniform variance] × (T + N × a)/(1 + N × a).
- **Exact variance ratio:** (T + N × a)/(1 + N × a).

This is a calibration for second moments of *counts*, **not** an exact formula for Gini or rank movement. It predicts no amplification at T=1; the ratio increases with horizon and decreases as a grows. Randomness still generates fluctuations under uniform control.

For N=20, W0=10, offset=1, T=500: total initial concentration is 220, so the exact count-variance ratio is **720/221 ≈ 3.2579**; control variance **23.75**; treatment variance **approximately 77.38**. Our observed Gini differences are a distinct, nonlinear statistic, so this result does not mathematically prove the reported Gini interval.

The Rust library implementation is in [src/analytic.rs](../../src/analytic.rs); unit tests cover exact two-actor enumeration, zero amplification after the first draw, horizon/offset direction, and overflow/boundary validation. A further test simulates 3,000 societies per arm with both the independent reference simulator and the production `OpportunityPolicy` drawing from Hiyal's named streams, and requires both empirical per-agent variances to match the closed form within 8%; a weighting or ticket-selection bug in the policy fails it.

## Interpretation for scientific publication

Confirming a Pólya urn amplifies allocation variance is not novel. It is a known property, not a discovered universal law of inequality. Any resulting paper should explicitly characterize Experiment 001 as a **benchmark and model-validation stage**, rather than as the central sociology contribution.

Cumulative advantage has a long research history:
- DiPrete & Eirich (2006), *Cumulative Advantage as a Mechanism for Inequality* — https://doi.org/10.1146/annurev.soc.32.061604.123127
- Research on reputation and inequality in cooperative network interactions — https://pmc.ncbi.nlm.nih.gov/articles/PMC8487746/
- Experimental/network inequality research with institutions, reputation, rewiring — https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0200965
- Agent-based models of reputation/status differentiation — https://doi.org/10.1093/oxfordhb/9780190494087.013.13

These illustrate prior work and **do not establish** that a proposed successor study is new. A targeted systematic literature search remains necessary.

## Next model-design requirement

A scientifically interesting successor must make a falsifiable prediction that cannot be inferred immediately from the symmetric urn formula, for example by introducing feedback between *noisy beliefs about other actors* and *endogenous access to social opportunities*, together with an intervention that alters beliefs without directly redistributing wealth.

Do not introduce flexible agents, networks and institutions simultaneously without controls; vary one mechanism at a time, preserve nested baselines, and distinguish structural isolation from genuine empirical claims.
