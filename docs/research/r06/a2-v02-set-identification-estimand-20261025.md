# R-06 — A2 v0.2 set-valued mechanism identification and success criterion

> **Historical v0.2 inference draft — superseded by [v0.2.1 admitted-set criterion](a2-v021-set-estimand-20261025.md).** The latter fixes A0, proposes a calibrated rival-outperformance gate, and distinguishes structurally equivalent alternatives. No coverage has been validated.


**Date:** 2026-10-25 · **Design-level estimand adopted for drafting; thresholds, distributions, budget and validation NOT frozen.** Companion to [v0.2 shared-world/six-kernel design](a2-v02-six-kernel-design-20261025.md). This **supersedes v0.1's “minimum of six unique-family recovery rates” as the primary target**, without hiding the old target or altering v0.1's results.

## 1. Distinguish the truth generator, identifiable family set and finite-data admitted set

The six candidate families are `M,N,C,S,L,R`. Each family defines a composite **joint distribution of matching, public interventions, actions and released payoffs** for the same physically assigned intervention design `A`. Family-specific and **common shared nuisance parameters** (noise, ranking appraisal, actor composition) range over explicitly allowed sets. The inference unit is **one generated dataset D from K independent societies**, each with four actors and T decision periods, all governed by **the same generating family** but distinct actor-level state; experiment-level family hyperparameters must be common or hierarchically sampled according to a **predeclared** rule. `K×4×T` observations are not independent units. A separate Monte Carlo replication draws independent entire datasets `D_1,D_2,...` to estimate error/ambiguity rates, with its **own** precision budget.

**Population structural identified set for the true joint law P_★ under physical design A:**

`I_A(P★) = { m ∈ {M,N,C,S,L,R} : ∃ θ_m in allowed domain with P_{m,θ_m}^A = P★^A on ALL permitted joint observables }`.

This is a **population property**, not computable exactly from a finite run. It can have one, several or all six members; a true generator in nested/overlapping regions may have **non-singleton I even at infinite observation budget**. A finite parameter grid giving zero divergence at a few examples does not establish global equality, and a positive grid minimum does not establish a strictly positive **continuous-space** separation. To claim a structural nonidentification/equivalence, construct a shared law or prove equality of joint kernels. To claim global separability, prove a strictly positive lower bound on divergence after **minimizing over the competing family's full admissible parameter space** (or record unproved/unknown if not possible). Historical C/L **action law** equality does not imply S's full **joint matching-and-action** law without matching kernel equality too.

**Finite-data admitted model set** `C_α(D) ⊆ {M,N,C,S,L,R}` is constructed via the **same** nested fit/test/calibration recipe for every family. A model is retained when it is **not excluded** by its predeclared model-adequacy test at calibrated error `α`; the same observed records, treatment assignments and compute-fitting budget are available to every model. **No forced best model or “at least one winner” constraint.** A separately calibrated **absolute adequacy gate** may exclude *all six*, so `C_α(D)=∅` is a valid output. This is especially important for deliberately **out-of-family** truths and misspecification.

**Interpretation of outcomes:**
- `|C|=1`: *unique statistically admitted family within the six nominated models at this finite-data calibration*, **not proof of the true psychological or sociological cause**. Even a singleton can be wrong under statistical error/misspecification.
- `|C|>1`: an **undetermined set**, useful and reportable; may reflect exact structural aliasing or merely insufficient data. Diagnose which with theory/identification map, not merely a predictive score.
- `C=∅`: **none of these six models adequately explains the observed joint data** under the prespecified testing rule (or a Type I error if data were in-family); do not coerce an out-of-family dataset into a nominated label.
- `m★∈C`: retention of the known synthetic true family for **evaluation only**. The classifier does not see `m★`, latent preferences, generated parameter values, hypothetical forks, private scores or counterfactual histories.

## 2. Primary comparison: reduction of false-alternative ambiguity subject to coverage

Two budget-matched design policies (same number of independent societies and total actor decisions/matches/fit attempts):
- **A (intervention policy):** a predeclared randomized fine and partner-matching schedule using v0.2's physically identical arms for all six.
- **A₀ (sham/comparison):** same total observation and fitting budget, no proposed informative treatment contrast, with schedule and monitoring recorded before any outcomes. This is a design *comparator*, not a mechanism label.

Define a **false-alternative burden per in-family truth** `B(D,m★)= | C_α(D) \ {m★} |`: the number of **nontrue family labels left plausible**, as known to the synthetic evaluator only. When `I_A(P★)` has more than one member, some alternative retention is **structurally correct and irreducible**; the result must explicitly report that population aliasing separately rather than call these false sociological explanations. `B` is a **design ambiguity score**, not an error rate or evidence of unique causes. An over-aggressive test can artificially reduce it by excluding the true family too, hence the coverage guard below is mandatory.

**Primary proposed design comparison (conditional on a predeclared in-family truth distribution G):**

`Δ_B(G) = E_G[B(D^{A₀},m★)] - E_G[B(D^A,m★)]`.

Report the **six family-conditional effects separately**, relevant near-overlap/misspecification strata, and only then any **equally weighted six-family macro average** using explicitly fixed G. Prefer expected reduction of **nontrue alternatives** and resulting set-size distribution to forced true-label singleton recovery. `Δ_B > 0` means the intervention retained fewer rival family labels **subject to demonstrably calibrated truth coverage and model adequacy**, not that humans behave in one way. A nonpositive or ambiguous result is scientifically valid.

**Coverage/validity gate required before comparing ambiguity:** target `P_{m★,θ}(m★∈C_α(D^A)) ≥ 1-α` and the same under A₀, for each generating family, at **all** relevant prespecified parameter strata (nominal proposal `1-α=0.95`). **Do not assert finite-data 95% validity before calibrating/tested evidence, especially at parameter boundaries and nuisance extrema.** Calibration across families may require conservative family-specific bootstrap/supremum thresholds, simultaneous-test corrections, and an adversarial validation of nominal versus observed family-wise coverage. If a design fails its coverage standard, its apparently smaller B does **not** count as a scientific gain. At a structural overlap, maintain honest admissible alternatives under their own calibrated tests; report if an estimated set systematically excludes observationally equal classes.

**Absolute fit:** need a prespecified, held-out **absolute joint-likelihood/predictive-discrepancy test**, not only model-to-model relative ranks. A relative best-fitting model is not adequate by default. If all candidates fail their absolute calibration, output `C=∅` and separately report in-family false-empty rate (contributes to undercoverage) and out-of-family **correct empty-set detection rate**.

## 3. Mandatory companion outcomes (none can be hidden)

| Outcome | Denominator / level | Meaning |
| --- | --- | --- |
| **True-family set coverage** `1[m★∈C]` | independent **dataset**, separately by six truth families and near-overlap/boundary strata | Calibration/false-exclusion gate; not an optimization objective to manipulate |
| **Wrong-family retention** `B` and `|C|` distribution | independent dataset, all in-family truths | Partial identification/ambiguity burden; empty set and false exclusion also displayed |
| **Correct singleton** `1[C={m★}]` | independent datasets; conditional strata, **secondary only** | Not suitable as unconditional min-across-family primary due nesting |
| **Incorrect singleton** `1[|C|=1,m★∉C]` | independent in-family datasets | False mechanism-confidence risk |
| **In-family false empty** `1[C=∅]` | independent in-family datasets | Absolute-test false rejection or fit failure |
| **Out-of-family correct empty** `1[C=∅]` | independently generated, predeclared **genuinely alternative** generators | Honest detection of unknown mechanisms; no fabricated forced family |
| **Joint-law population ID class** `I_A(P★)` | theoretically specified parameters and arms | Structural equivalence/separation vs finite-data ambiguity; may remain **unknown** if unproved |
| **Predictive fit on held-out data** | dataset with independent world splits | Kept separate from mechanism confidence; a predictive winner is not unique structural recovery |
| **Estimator/interval uncertainty** | independent *datasets* (and societies clustered inside) | Monte Carlo confidence/intervals; no actor-period pseudo-replication |

**No replacement of nesting by assertion:** M is nested in several larger families; N is nested in L at α_L=0 and compatible κ_L/q₁ bounds; S may be nested in N with flexible signed ν and matching nuisance η. Endogenous matching and dynamic updates can generate additional observational equivalence. **These cases remain in the unconditional G denominators** and appear in the reported ID-class/near-boundary strata. Do **not** disqualify exact overlap truths or assign them a winner to improve a dashboard.

**Meaningful-effect margins:** a secondary *practical* question can require predeclared minimum contrasts/effect sizes (e.g., probability change across arms) and report a practical equivalence decision. Margins are **not** a proof that parameter-space models become disjoint, nor a license to prohibit an overlapping generator that matches observations. Specify any norm/learning thresholds as **conditional secondary regimes**, alongside full unconditional results. Original owner chose **identifiability/limits**, not a guaranteed classifier accuracy contest.

## 4. Admissible-set construction and calibration — decisions prior to pilot, NOT claims of validation

The following is a **design specification**, not yet an executed statistical procedure:

1. Partition **whole societies and complete experimental datasets**, not action rows, into disjoint fit/tuning/calibration/test domains. Freeze the allocation, future seed region, shared nuisance priors and observed treatment schedule **before inspecting test outcomes**.
2. Fit each family to the **same joint action-and-matching observables**, including its allowed nuisance and fixed physical assignment regime; do not give S a separately observed private score or R an extra free inverse temperature. The fit target must reflect *generated pair assignment probabilities* and actions, not action logits alone. Equalize computational optimizer effort and parameter-space search quality; report boundary optimization failures.
3. Use a **held-out absolute goodness-of-fit statistic** and calibrate rejection against the *composite model's own allowed parameter uncertainty*, including boundaries and poor-identification cases. Candidate approaches: parametric bootstrap with nuisance re-fitting, worst-case over prespecified restricted nuisance regions, and calibration that protects truth-family coverage; their technical correctness/compute cost require P0/P1 review. **Naive χ² asymptotics fail at constrained zero-effect boundaries**.
4. A candidate model is included if its adjusted adequacy test does not reject; **the empty set is permitted**. Report both unadjusted and multi-family/simultaneous-calibration sensitivity where scientifically appropriate. Calibration by itself does not magically guarantee uniform 95% coverage of all composite families; empirical validation and conservative bounds needed.
5. Predeclare **out-of-family** generators (e.g., mixtures, dynamic aspiration with unmodeled social reinforcement, stochastic matching not in the six) before tests. They must not be accidental near duplicates called novel truth; if they belong to the nominated union they are *in-family*, not OOD.
6. Independently review results including **null**, empty, full-set `C={M,N,C,S,L,R}`, and boundary outcomes. Never tune acceptance thresholds using the confirmatory region after seeing which treatment “wins.”

The exact 15 unordered family comparisons are `M/N, M/C, M/S, M/L, M/R, N/C, N/S, N/L, N/R, C/S, C/L, C/R, S/L, S/R, L/R`. For each pair under each arm, assess the infimum over **free rival parameters of divergence between the FULL joint matching-and-action kernels**. Label: **constructively equal**, **separated with analytic/certified bound**, **numeric/conditional suggestive**, or **unknown**. A 16-step history/finite grid cannot certify global positive divergence over continuous nuisance domains.

## 5. Budget, inferential unit and anti-leak rules

- **K independent societies × four agents × T action periods = 4KT action decisions per classified dataset.** A pair counts once for matching assignment and twice for actions. Pseudorandom seeds/agents are nested **within** societies; independent analysis uses society/dataset replication appropriately. T, K, independent dataset repetition count J and all arm probabilities remain **TO BE DETERMINED AFTER identification map and calibration feasibility**.
- **J independent generated datasets** estimate operating characteristics; one-world action rows are not J samples. Interventions/sham might share common random numbers only as a *variance-reduction comparison* after plan, never as paired future responses given to the primary model classifier; use joint uncertainty for such dependent design contrasts.
- **Equal budget:** same K,T, fine/match assignment accounting, actor decisions, observed histories, model fit attempts, calibration draw counts (unless justified under common cap), and reproducibility/replay requirements. Calibrating six composite models can be costly; estimate that cost before authorizing simulation or CI.
- **Shared calibration vs prior:** actual `β,ε,η,γ,ρ` priors and hyperparameter distributions, all family-specific effect ranges and actor-type allocation, boundary/overlap proportions, out-of-family mix, objective weights and minimal effect size **UNFROZEN**. They must be recorded **before** a run, not selected to make the chosen design look good.
- **No primary input leakage:** no true family label, Q/q/type, private score, per-fork counterfactual, internal family-specific RNG stream, own posterior hidden under debug data or treatment-assignment inconsistency. A reference script can inspect latent state *solely* for independent oracle verification (distinct source/tool, not the classifier).
- **Cost-sensitive stop:** if all plausible interventions have `I_A(P★)` large or identical under free rivals, report that non-identification or redesign. Do not keep raising sample size, trimming hard parameter corners, or changing model families to force a positive Δ.

## 6. Gate checklist — scientific work deliberately deferred until user authorizes verification

- [x] Primary estimand **redesigned conceptually**: set-valued admission with coverage, ambiguity, calibrated absolute adequacy and possible empty model set; **no numerical pass claimed**.
- [x] One classification unit and two distinct budgets (**K×4×T observations versus J independently replicated datasets**) defined conceptually.
- [x] Full 15-pair/free-parameter identification-map contract written.
- [ ] Independent statistician/methodologist evaluates uniform family-wise coverage assumptions, boundaries, absolute fit and feasibility of fitting all six joint likelihoods.
- [ ] Independently verify v0.2 kernels and matching distributions, including alternative family nuisance/correlation rivals and model nesting.
- [ ] Freeze all underlying distributions, calibration method, effect margins/weights, confidence level, sample budget and full P0/P1 methodology in an approved dated protocol.
- [ ] Approve a bounded run separately. No such approval or run is implied.

**Decision:** the old six-way minimum singleton rate is RETIRED as the primary endpoint; structural overlaps are first-class outcomes, not errors to delete. This specification is for **the later independent verification stage**, not evidence that the new formula has already passed it.
