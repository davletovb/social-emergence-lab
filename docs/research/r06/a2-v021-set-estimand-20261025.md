# R-06 — A2 v0.2.1 set-valued inference and design comparison

**2026-10-25 · DRAFT METHOD DECISION, NOT INDEPENDENTLY CALIBRATED OR FROZEN**

This is the **current proposed statistical/inferential contract**, superseding the [v0.2 draft](a2-v02-set-identification-estimand-20261025.md) for future independent P0/P1. Read alongside the [v0.2.1 six-generator kernels](a2-v021-six-kernel-design-20261025.md). It specifically addresses points 4–5 and 8–9 of the [Issue #50 new review](https://github.com/davletovb/social-emergence-lab/issues/50#issuecomment-6395811896). No numerical coverage, test power, budget or result has been established.

## 1. Truth, identifiable set, and statistical admitted set are not the same object

One **classified dataset** `D` consists of **K independent four-actor societies** drawn under one predeclared generating family `m★∈{M,N,C,S,L,R}` and a common declared family-level parameter-generating process with actor-level heterogeneity. Each world provides one T-period trajectory; **K×4×T action records are clustered observations, not K×4×T independent studies**. J fully independently generated datasets per family and design quantify statistical operating characteristics. `K,T,J` and their computation budget remain unknown, pending identification and calibration review.

Let `P★^A` be the **joint population law of physically observed matchings, fines, actions, public histories and realized payoffs** under assigned regime A. For each family `m`, let `θ_m` include all allowed family and shared-nuisance parameters/heterogeneity laws. Define the structural **identified family set**:

`I_A(P★) = {m : some permitted θ_m yields P_m,θ_m^A = P★^A}`.

This is a **population structural property**, not calculated by retaining models on one sample. It can be empty for genuinely out-of-family laws, singleton or multi-family. If S and N meet the parameter-mapping conditions of [v0.2.1](a2-v021-six-kernel-design-20261025.md), **both belong to the same structural identified set** even with unlimited matched-action observations. A valid design may narrow `I` on some truths and not others. An independence restriction on eta/nu is **not** included in primary `I`, and becomes a separately labeled sensitivity target if studied at all.

A **finite-data admitted set** `C_α(D)` consists of the families not excluded by properly **calibrated composite-model tests** of the full joint law, using exactly the same observables/fit opportunities. Valid outputs:
- **Empty:** no candidate survived absolute model adequacy or properly calibrated rival comparison. This is an admissible result, not an implementation exception.
- **Singleton:** one surviving model family. This is a finite-data statistical claim about the nominated models, **not proof of any human mechanism**.
- **Multiple:** unresolved alternatives, either truly structurally equivalent or not excluded at available statistical power.

All six families must be fitted against the same public histories; no access to latent mechanism labels, submitted valuation scores, future counterfactual forks or withheld private actor states.

## 2. Explicit coverage-protected two-gate candidate admission

A purely relative likelihood winner is not adequate. A purely omnibus absolute test might have poor power against particular close rivals. The **proposed** two-gate rule for each candidate family `m` is:

1. **Absolute fit gate** `A_m(D)`: retain `m` only if its predeclared held-out **joint-law goodness-of-fit** statistic does **not** reject it at a calibrated test level `α_abs`, including parameter/refit uncertainty.
2. **Rival-specific gate** `R_m(D)`: retain `m` only if no alternative family `n≠m` significantly **outperforms `m`** under the same held-out joint data and a predeclared **max-over-five competitor**/simultaneous-within-m calibration at level `α_rel`. Include the possibility that several rivals tie or that all fail absolute fit.

`C_α(D) = {m: A_m(D) AND R_m(D)}`.

**Tentative error split only**, `α_abs=0.025`, `α_rel=0.025`, total `α=0.05`. The exact loss, held-out domain, constrained nuisance fitting, competitive statistic, bootstrap/worst-case thresholds and error split **are NOT frozen**. For a fixed in-family true `m★`, if *both* component tests are uniformly calibrated at their stated sizes across the true family's full composite parameter domain, a union bound provides:

`Pr(m★ not in C_α) <= α_abs + α_rel = 0.05`.

**This is conditional mathematics, NOT evidence the tests satisfy those bounds.** Neither asymptotic naive χ² nor raw one-sided rival comparisons guarantee it at parameter boundaries, constrained nuisance or misspecification. The five within-family rival comparisons need *their own* max-statistic / simultaneous correction as part of controlling the level `α_rel`; it is **not valid to run five independent unadjusted 0.025 tests and still claim ≤0.025 family exclusion probability**. By contrast, if the claim is only retention of the **single unknown true family**, an additional Bonferroni correction across all **six candidate families** is not automatically required. Strong **simultaneous** retention of *every structurally equivalent family*, or a simultaneous coverage assertion about the entire structural set, is a different target and may demand further calibration.

**OOF/empty gate:** For explicitly prespecified generators outside the union of all six allowed law families, all six may be rejected and `C=∅`. For in-family truths, an empty set counts against the true-family coverage guard. Absolute and relative testing under source-unknown or underidentified nuisance still require an independent statistician's review. OOF correctness is not guaranteed by writing `C=∅` in the protocol.

## 3. Physical treatment A and sham A0, with fixed comparison intent

**A (intervention candidate)**: per society-period, independently select matching mode `z=random/choice` with probability 1/2 each; after matching, independently assign each dyad fine `f=0` or `1/2` with probability 1/2 each. History/outputs publicly observed under the same time ordering in both designs. No model receives a private fine before matching; assignment probabilities do not vary with true family or past response.

**A0 (specific untreated comparator)**: `z=random` and `f=0` in every society-period and dyad. All other rules, actor-periods, independent worlds, action/observation logging, fitting attempts and candidate family domain definitions unchanged. This is *not* a hypothetical “no information collected” or different-resource control: it is the fixed physical no-fine/uniform-matching baseline.

- Compare A and A0 at **identical K, T, observed actor-periods, observed realized matchings and total fit/calibration effort** (after a fair common computation-budget protocol is frozen).
- A0 is an intentionally weaker **physical policy comparator**, so differences in structural identified sets may be real; do not interpret them only as statistical power differences. Under random matching, S's omega cannot alter matching, so S and signed two-point N can overlap. The primary *unrestricted-nuisance* S/N equivalence may persist under A too.
- Fine-only, matching-only, and other alternate shams are **separate sensitivity designs to budget explicitly**, not undisclosed primary variations.
- Run-generation design is one realized history per society. Shared random-number pairing of different societies across A/A0, if ever used for variance reduction, must be counted and documented and **not exposed** to the primary classifier.

## 4. Primary design estimand: ambiguity with coverage gate, then structural decomposition

For a known synthetic truth `m★` (truth label used **for evaluation only**), define the number of *other labels* retained:

`B(D,m★) = |C_α(D) \ {m★}|`.

Do **not** call every other retained family “false” or “wrong”: in an overlapping family region, retaining another mathematically compatible family is **correct**. B is a **nontrue-label ambiguity burden**, not necessarily a false belief.

**Primary proposed comparison**, evaluated after coverage is calibrated:

`Δ_B = E[B(D^A0,m★)] - E[B(D^A,m★)]`.

Report separately for each family and declared boundary/interior/near-overlap stratum, and only optionally a preweighted macro average. A positive Δ_B means treatment decreases admitted alternative labels under adequate truth-family coverage; it does **not** imply global structural identification or psychological causality. Model construction may generate exactly equivalent families, so Δ_B may be zero at M truths or at S/N aliasing truths, no matter K/T.

**Decompose whenever population structural identified sets can be certified**:
- **Structurally warranted alternatives** `B_struct(A,m★)=|I_A(P★) \ {m★}|`, a theoretical property of design A and underlying true joint law; this is a **lower bound on population ambiguity**, not a statistical error or a goal to suppress.
- **Extra finite-data ambiguity** `B_extra(D,A)=|C_α(D) \ I_A(P★)|`, when `I` is constructively verified (nonnegative and 0 if only structural explanations retained). Its expectation estimates avoidable residual uncertainty conditional on certified I; if I is **UNKNOWN**, mark B_extra **NOT IDENTIFIED**, not zero.
- **Missing structural explanations** `B_miss(D,A)=|I_A(P★) \ C_α(D)|`, an error/sensitivity quantity; if a structurally equivalent model is lost statistically, report it instead of claiming successful discrimination. Strong simultaneous control over B_miss needs its own error policy.
- Structural effects of A: `B_struct(A0) - B_struct(A)` are valuable **only if supported by a real joint-kernel proof or certified separation**; not a finite-grid artifact.

The decomposition prevents an apparent decrease in B caused by **incorrectly excluding a truly compatible rival** from being advertised as scientific identification. Until sufficient structural sets are proved, Δ_B and coverage are descriptive finite-data results, not a claim that ambiguity is removable in principle.

## 5. Reporting, adverse outcomes and budgets still open

Mandatory outcomes **per independent generated dataset and true-family stratum**: true-family coverage, true-family false exclusion, `C=∅` false-empty/in OOF correct-empty, `|C|` histogram, B, incorrect singleton, correct singleton as secondary, structural equivalence where proved, missing structural alternatives, held-out prediction **separately from family inference**, plus J-dataset uncertainty intervals. **Do not treat action rows as independent observations or suppress near-overlap/boundary truths.**

**15 pairwise free-parameter questions** remain `M/N, M/C, M/S, M/L, M/R, N/C, N/S, N/L, N/R, C/S, C/L, C/R, S/L, S/R, L/R`. For each A0/A and accessible history, compare the **joint stochastic match/action law**, search for constructive equalities over allowable rival nuisance domains, certify positive divergence via valid analytic restrictions/bounds when possible, otherwise write **UNKNOWN**. The known S/N mapping is a serious existing candidate structural equality under unrestricted η/ν correlation. Finite rational grids are only diagnostic, not a global certification of positive KL/TV divergence in continuous parameter spaces.

**Unfrozen**: absolute-fit statistic; rival-improvement statistic; acceptance level/domain calibration; η correlations and actor-level hyperpriors; conservative fit/bootstrapping algorithm; practical equivalence margins; uncertainty and multiplicity objective; K, T, J; nuisance ranges and distribution; pilot cost; undisclosed confirmatory region. These all require P0/P1 independent scrutiny and owner approval before any experimental data generation. A nominal 95% *goal* is not achieved coverage.

**Verification boundary:** PR #53 is an internal design desk check for v0.2, not a P0 validation of v0.2.1. No stochastic simulation, human-data download, Hiyal implementation, CI run, independent six-kernel verification or selection of K/T/J has been performed for this document.
