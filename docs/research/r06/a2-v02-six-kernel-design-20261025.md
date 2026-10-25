# R-06 — A2 v0.2 kernel and physical-world redesign

> **Historical v0.2 design — superseded by [v0.2.1 six-kernel specification](a2-v021-six-kernel-design-20261025.md) for independent P0.** Keep this file for audit. The reviewer [PR #53](https://github.com/davletovb/social-emergence-lab/pull/53) checks v0.2, not the revised v0.2.1 model.


**Status:** design decisions proposed for Experiment 003; **NOT independently verified, frozen, implemented or run**. Dated 2026-10-25. Supersedes the **candidate equations** of [v0.1](../r05/a2-six-generator-formal-spec-20261025.md) for future verification, but does **not** rewrite v0.1's 23 + 21 old-oracle values or claim they validate v0.2. Source impetus: [R-06 Claude review](https://github.com/davletovb/social-emergence-lab/issues/50#issuecomment-6394749043), [PR #51](https://github.com/davletovb/social-emergence-lab/pull/51), and [owner selection](../r05/decision.md).

**Contribution:** synthetic structural/model-set identification and explicitly allowed non-identification; no claim about actual human mechanisms. The six family *labels* still denote restricted generators, **not six disjoint truths**. All remain subject to a 15-pair adversarial free-rival-parameter review before P0 completion.

## 1. Common four-actor stage game: real cooperation surplus

Four agents, two contemporaneous pairs, binary action `a_i ∈ {0,1}` (1 = contribute/cooperate). For each pair `(i,j)`, the same realized material payoff for **every** generator is

`u_i(a_i,a_j;f) = 2 a_j - a_i - f (1-a_i)`.

Cost of contributing 1, partner benefit 2, publicly assigned **fine on defection** `f ≥ 0`. With `f=0` the payoffs are:

| Actor i \\ partner j | j defects (0) | j contributes (1) |
| --- | --- | --- |
| **i defects (0)** | `P=0` | `T=2` |
| **i contributes (1)** | `S=-1` | `R=1` |

Therefore `T>R>P>S`, and `2R>T+S`: this **is** a prisoner's-dilemma stage game with cooperation surplus (joint CC = 2; joint DD = 0). A partner's contribution raises own payoff by 2; the material difference between **own** contributing and defecting remains

`Δu_mat ≡ u_i(1,a_j;f)-u_i(0,a_j;f)=f-1`.

**Primary fine levels selected at design level:** `f ∈ {0, 1/2}`. `f=1` makes own action materially indifferent at fixed partner action and is **excluded from primary fine arms**; it may be reported only as a separately labeled boundary fixture if independently justified. **All old exact fixture values that used f=1 and the v0.1 aspiration/Q trace refer to the OLD game/model.** M/N/C/S/L *action logits* at f=0 or 1 remain algebraically the same as old because `Δu_mat` did not change; **R reinforcement traces and payoff observables do not**. At `f=1/2`, exact `σ(ln 3 × x)` is generally irrational; old rational-vector tests are not a substitute for new numeric/analytic checks.

**Information-leak guard still valid:** knowing `(u_i,a_i,f)` reveals the partner's action: `a_j=(u_i+a_i+f(1-a_i))/2`. This is **not** a problem in v0.2 because all past actions and earned payoffs are publicly revealed to every agent; a hidden-partner-action arm remains **retired**.

## 2. One experiment/period ordering and truthful t=1 initialization

One **experimental dataset for classification** consists of **K independent four-actor societies** under the *same true family and predeclared family-level parameter-generating process*, each with one realized `T`-period history. Agents and societies never see counterfactual cloned futures. K and T are not yet selected; a separate **Monte Carlo repetition** generates multiple independently drawn such datasets to estimate statistical operating characteristics. Do not treat `K×4×T` actor-decisions as `K×4×T` independent sampling units.

Period ordering shared by all six:
1. At `t=1`, no previous action exists. Every agent and classifier knows a **neutral initialization signal** `s_{j,1}=1/2` labelled `initial_prior_not_realized_action` (not falsely described as a truthful earlier action). For `t≥2`, `s_{j,t}=a_{j,t-1}` is an **actual, public previous action** for every candidate partner. Exact last-round payoffs also remain public.
2. Publicly assign matching regime `z_t∈{random, choice}` at society-period level. Everyone receives the same available partner history and **is allowed to express cardinal pair preferences**. In the random arm the broker discards these and chooses a uniformly random matching; in choice arm it applies the **identical** stochastic mutual-acceptability rule below.
3. Draw one matching over the three perfect matchings of four actors. Then publicly assign `f_{mu,t}∈{0,1/2}` **per matched dyad**, independent of all latent states, and announce it before actions. A fine cannot influence the matching that was already selected in that period; it can influence future behavior/matching through public history.
4. Every actor chooses `a_i,t` simultaneously, only using currently authorized observations and private state. Payoffs `u_i,t` are computed and **actions and exact payoffs are publicly revealed after both choices**.
5. Update family-specific learning states (L and R) **once after** realized payoffs and actions; no anticipatory knowledge, no unobserved update after T counted as a distinguishing outcome. Advance t.

**Randomness namespaces** are a prospective requirement for verifier: `(domain, experiment_id, world_id, period, actor_id, purpose)` for action draw/choice, matching choice, fine assignment and parameter generation. Counter-based or equivalent event-stable mapping; **no RNG/code is implemented or tested in this document**. Primary classifier receives pair IDs, assigned regime/fine, genuine past actions/current actions, released payoff, period; **not** latent family, private utility coefficients, cardinal matching scores, R values, or simulated counterfactual futures.

## 3. Shared contribution-choice observation link

Every generator uses the **same** noise and lapse specification (to remove R's old noise fingerprint):

`P(a_i,t=1 | H_t,x_i,t,f) = ε/2 + (1-ε) σ(β Δ_i,t)`, with `σ(x)=1/(1+e^{-x})` and `β>0`, `0≤ε<1`.

Here `β,ε` are **common nuisance parameters** with the **same prior, admissible domain and estimation access** for all six families; do not fix them for five models and fit them only for R. Their actual values, prior and across-world hierarchy must be frozen before any simulation; `β=ln 3, ε=0` is solely a legacy **reference-fixture corner**, not a winning parameter.

Let `d(f)=f-1`. All six incorporate the **same contemporaneous material term `d(f)`**; differences are substantive response/learning structures, not one family's ability to notice a fine. Mechanism-specific latent variables are **not** primary classifier features.

| Family | `Δ_i,t` action utility difference | State update after realized choice | Scientific caveat / overlap |
| --- | --- | --- | --- |
| **M: material response** | `d(f)` | none | Nested as null-effect/zero-learning corner in several richer families |
| **N: stable private preference** | `d(f)+ν_i`, `ν_i∈[-ν_max,ν_max]` | `ν_i` fixed | **Signed** baseline allowed; S's negative type is no longer a special sign-only label; no assertion this is a Bicchieri norm |
| **C: conditional cooperation** | `d(f)+κ_i s_{j,t}`, `κ_i∈[-κ_max,κ_max]` | no persistent hidden learning, response to **actual matched partner's** past action; `t=1` neutral prior `1/2` | N/C coincide on restricted all-1 histories at `ν=κ`; with κ=0, M corner |
| **S: fixed types + sorting** | `d(f)+δτ_i`, `τ_i∈{-1,+1}` persistent, `δ≥0` | `τ` fixed, does not acquire changing cooperative beliefs; partner choices have **additional type-linked affinity** (below) | Its action law is a heterogeneous special case of **signed** N; identification must use **joint action+matching** and restrictions actually tested, not sign |
| **L: fine-masked partner learning** | `d(f)+κ_L q_i,t`, `q_i,1∈[0,1]`; `κ_L∈[-κ_max,κ_max]` | after observing current partner `a_{j,t}`: `w(f)=1-ζ 1[f>0]`; `q_{i,t+1}=(1-α_L w)q_i,t+α_L w a_{j,t}`, `α_L,ζ∈[0,1]` | **N nests in L when `α_L=0`** and `ν=κ_L q_1`; C/L may coincide on fixed matches at `α_L=1, ζ=0`; f>0 changes q updating |
| **R: contextual aspiration reinforcement** | `d(f)+Q_i,t(1,f)-Q_i,t(0,f)`, with **two explicit fine contexts** `f=0` and `1/2` | initialize **all** `Q_{i,1}(a,f)=0`. After realized action `a` under context `f`, `Q_{i,t+1}(a,f)=clip(Q_i,t(a,f)+α_R (u_i,t-A_i,t), -Qmax,Qmax)`; leave other 3 Q entries unchanged. `A_{i,t+1}=(1-λ_A)A_i,t+λ_A u_i,t`; `α_R,λ_A∈[0,1]`. | Immediate fine effect shares the common `d(f)`, so R does **not** have a built-in fine-blind signature. With `α_R=0, Q=0` it nests M. **Payoff-based adaptation** remains possible and is not assumed to identify R |

**Update-time check:** a fine change in the **last observed period** may update L's q or R's Q but has no subsequent action in which that learning can be observed. v0.1 PR #51's terminal-vs-earlier-withdrawal result remains an important negative control *in spirit*, but its **old L(ζ=1) exact values and all R payoff traces require regeneration at the chosen v0.2 fine schedule**.

**Overlap is intentional**. At α_L=0, L may implement an N law even with κ_L nonzero; null/boundary cases remain in the unconditional denominator. There is no magic relabeling of these as separate “truths.” Signed N and common matching tastes introduce **new** possible S/N equivalences; that risk is a required 15-pair analysis, not a flaw to hide by prohibiting rival parameters.

## 4. Partner preference offered to all six — no S-only right to match

At each period, every actor may submit a cardinal bounded valuation for each other actor's partnership. **All** six families receive the same physically observable `s_{j,t}`, common public one-step estimate `π_{j,t}` and the same permitted fixed idiosyncratic *matching nuisance* `η_i`:

- At t=1 `π_{j,1}=1/2, x_{j,1}=0`.
- At t≥2 `π_{j,t}=(1+s_{j,t})/3` (a declared smoothed predictive *heuristic*, not empirically calibrated Bayesian truth) and `x_{j,t}=2s_{j,t}-1`.
- `η_i∈[-η_max,η_max]` is an **actor-fixed, potentially signed partner-appraisal bias**, allowed for **ALL SIX**. It affects **matching only**, never directly enters a material contribution payoff or action choice. The admissible distribution and possible correlations of η with stable action preferences are to be reviewed, **not silently set independent to artificially identify S**.
- Common valuation `v_i(j)=σ(γ [,2(π_{j,t}-1/2)+η_i x_{j,t}+mathbf1_{{S}} ω τ_i x_{j,t},])` for j≠i, `γ>0, ω≥0`. **Only S has the additional `τ_i`-linked affinity**, but every other family already has variable nonzero matching valuations via η. An S mechanism must be distinguished by the **testable correlation between persistent behavioral type and pairing pattern**, not by access to preference submission or a sign-restricted contribution coefficient. For all `t=1`, `v_i(j)=1/2` under all six (neutral prior).
- `γ`, `η` domain/hierarchy and the common matching temperature `ρ>0` are **shared model-neutral nuisance rules**. The unknown `ω` is a hypothesis parameter only within S, **not** uniquely observable.

**One broker rule for everyone (no guaranteed stable roommate solution assumed):** for each of the three perfect matchings `μ`, compute **mutual acceptability** `W(μ)=∑_{(i,j)∈μ} min(v_i(j),v_j(i))`. In the **choice** arm assign `P(μ|v,z=choice) = exp(ρ W(μ))/∑_{μ'} exp(ρ W(μ'))`. In the **random** arm assign `P(μ|z=random)=1/3`, ignoring the same submitted valuations. This is a **stochastic centralized mutual-consent approximation**, **not** a proof of pairwise stable matching or real-world institutional feasibility. Its full support prevents deterministic label-by-pair assignment; common η means all families can exhibit heterogeneous sorting. It also avoids the v0.1 `ω>0` scale invariance from deterministic argmax: **the distribution**, not just the best matching, can vary with ω. Whether it does for a given history and nuisance value is an *open mathematical test*.

**Essential identification caveat:** `η_i + mathbf1_{{S}} ωτ_i` may be observationally absorbed into a rival's η distribution. Signed N can match S's fixed-type **contribution kernel**. A sophisticated non-S model may therefore mimic S even in joint match+action observations. We **do not assert** the new matching rule successfully identifies S; if it is observationally equivalent under unrestricted relevant nuisance laws, **report that non-identification**, or request a new owner-reviewed hypothesis.

**No perfect-stability claim:** softmax matching yields some pairings that may be blocked by other pairs; it is an explicitly stochastic broker rather than mutual bilateral commitment. This is a deliberate design choice requiring methodological review before freeze.

## 5. Exclusions, source inheritance and required future verification

- No hidden-history arm, model-specific matching privilege, raw cardinal `v_i(j)` or η/τ/Q/q/ν/κ access in the primary classifier; the broker uses valuation submissions **internally**, then discloses only the actual pairing and regime. This is equal observability *across* mechanisms. No arbitrary fake earlier partner action: t1 signals explicitly labeled neutral priors.
- Historical [v0.1 action/payoff checks](../r05/a2-analytical-oracle-20261025.md) and [PR #51 exact-arithmetic script](a2_kernel_desk_check.py) are **legacy verification for v0.1**, not v0.2. Revise fixtures only in P0; retain historical references and explain changed R/payout predictions.
- **P0 must first challenge the newly chosen model, not merely confirm its printed values:** verify payoff matrix, all information flows, time-indexed actor Q updates, actual stochastic matching weights/ties and ν/κ signed domains. Test `t=1`, end-period learning visibility, actual match-and-action joint kernels, degenerate `η`, θ and ω, and whether the listed class pairs are non-identifiable in the continuous rival parameter spaces.
- No stochastic trial, Hiyal migration/implementation, new GitHub workflow, code generation of artificial humans, empirical correspondence claim or R-06 scientific freeze has been approved. The review's proposed repairs were used as hypotheses; v0.2 is a **new designed model**, not a retroactive result.

**Source limitations:** Galbiati's actual PD payoffs and matching protocol still require original-methods confirmation before claiming experimental docking; the v0.2 payoff matrix is **our** stage game, not one imported or attributed to Galbiati. Likewise the stochastic broker is **ours**, not Yang & Yue's original matcher.
