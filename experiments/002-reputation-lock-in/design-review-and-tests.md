# Experiment 002 — pre-implementation design review and falsification matrix

**Gate state: DESIGN REVIEW, not preregistration.** This document specifies what would count as support, a refutation, or an invalid model. No outcomes have been produced.

## Risk register — how this design could mislead us

| Risk | Failure mode | Countermeasure and measurable check |
| --- | --- | --- |
| Prior-art novelty | Calling self-fulfilling false popularity or network inequality a first discovery | Use [literature review](literature-review.md) with explicit overlapping published work; require independent citation search before scholarly novelty claim |
| Trivial network effect | Perfect information correction leaves access skew by graph in-degree, tautologically | Distinguish expected exposure from realized allocations; report pre/post *in-degree* and the residual beyond a simple degree-only counterfactual |
| Historical-resource contamination | Old winners retain stock of wealth even if opportunities are corrected | **Primary outcome is the new 400-step opportunity flow**, not cumulative terminal wealth Gini |
| Endogenous selection / noisy observations | Low-exposure agents generate fewer performance signals; this might look like lower ability | Fix all true qualities at 0.5; log whether and when signals are sampled; report observation counts |
| Selection bias in labeling | Designating “winners” after seeing outcomes creates regression-to-the-mean / collider bias | Use balanced **randomly assigned** false-boost B at time zero, prior to graph and outcome randomness |
| Structural repair is mechanically effective | Degree-balancing directly forces equal expected access, may make intervention advantage unsurprising | Use matched balanced rewiring baseline; quantify how much advantage is explained solely by indegree; do not claim a novel theory from this alone |
| RNG artefacts | A draw stream changes because a branch takes a different decision path | Named RNG substreams, replay tests and independent reference implementation; pairing justified by common prehistory, not identical postintervention draws |
| Four-arm pseudoreplication | Treating four interventions and 20 agents as independent greatly understates uncertainty | One **preintervention world/seed** = one independent observational unit; paired within-seed comparisons |
| Confirmation leakage | Tuning κ, β, τ, measurement window or outcomes after inspecting significant results | Explicit exploratory grid and held-out confirmatory split; freeze code/plan/inference and archive before looking at held-out results |
| Coverage/missing data | Simulated failures/dropouts are excluded based on observed effect | Strict fail-closed replay/manifest rules; rerun from same committed seed only after documented infrastructure fix |
| Unrealistic perfect correction | Assuming everyone believes a globally broadcast fact | Treat 100% credibility as an ideal upper bound; vary partial coverage and misinformation persistence in **later, separately specified** versions |
| External validity | Artificial rewiring resembles social exclusion but may not match real partner selection | Confront observational network and choice data where available; propose prospective consenting human experiment to identify effect |

## Independent-review checklist

- [ ] Check whether another study already tests belief correction × *structural* tie repair after self-fulfilling reputations; forward-/backward-cite Salganik & Watts (2008), van de Rijt (2019), Chu et al. (2020), Tsvetkova (2021), Simpson et al. (2023).
- [ ] Confirm substantive theory: define "access" as an opportunity gate, distinguish it from performance or trust, and justify why a graph is the relevant social institution.
- [ ] Validate the conditional-independence and exclusion statements under the graph/reputation dynamic.
- [x] **Formal derived identity, implementation pending:** for arbitrary current graph and uniform allocator with d links, exposure-only signed expectation is `g_deg=(Σ_i B_i in_degree(i))/(N*d)`; belief-selection residual is `g_sel=(1/N)Σ_jΣ_{i∈E_j} B_i (p_ji−1/d)`; `g=g_deg+g_sel`, and `β=0` forces `g_sel=0` in every pre-event state. A code-level independent reference test is still required; see [critical review](scientific-review-2026.md).
- [ ] Document whether independent graph repair is intrinsically desirable, whether it removes useful connections, and what social costs are omitted.
- [ ] Validate whether time τ and opportunity horizon produce enough rewiring to create *meaningfully varying* graphs without forcing a desired outcome.
- [ ] Decide how to handle attempts whose updater has no valid alternative candidate (finite complete neighborhoods, repeated links, overflows).
- [ ] Ensure the repair graph is independent of randomized B/performance and identical across repaired arms; build **explicit, reproducible balanced graphs** using independently named setup streams rather than claiming an unimplemented uniform sampler over all regular graphs.
- [ ] Confirm inferential method, null and minimum meaningful **postopportunity flow** difference; **do not reuse Gini variance/power estimates from Experiment 001**. Pilot precision must be estimated on per-parent `D_I^late−D_IS^late` and assessed for sensitivity to graph families and signal update rules.
- [ ] Match at least one model prediction to externally observable behavioral or graph quantities, with a source and scope of validation; the 2026 Honduras village microdata is **controlled-access**, not freely usable. See [data audit](empirical-alignment.md).

## Candidate falsification tests, *all* must be reportable

| Design test | Predicted/allowed result | How model would be falsified or criticized |
| --- | --- | --- |
| Identical priors δ=0 + balanced fixed graph | Boost label must have zero association with expected reward flow | Systematic nonzero label dependence suggests RNG/order/graph bug |
| No rewiring + full correction | **Expected symmetry** over B-randomized independent societies: `E[D_I]=0` if graph/transition is B-independent after correction; finite-seed deviations are allowed | A stable gap calls for tests of assumptions, RNG setup, hidden state and statistical precision |
| No preferential allocation (β=0) + regular fixed graph | Uniform choice within eligible neighbors | Any state-dependence fails a probability invariant |
| Correction + independent repaired regular graph | **Exchangeability null:** beliefs and public reputations reset and `A*` is independently generated from B; expected signed gap `E[D_IS]=0` over randomized societies **if** future transitions do not access B/legacy wealth and RNG is exchangeable. Individual seed effects need not be zero | A stable gap first triggers tests of exchangeability, RNG setup and Monte Carlo precision; it is not evidence of network lock-in |
| Original false reputation + no correction | May favor boosted candidates, **not guaranteed** in finite paths | Null/negative outcome is a genuine negative result, not a reason to filter |
| Repair without correction (U vs S, same prehistory) | `E[D_U − D_S]` may be positive, zero or negative; quantify uncertainty | Interpret by magnitude, not sign: repair helps only if `abs(E[D_S]) < abs(E[D_U])`. A negative `D_U` (boosted actors under-served) that repair removes is a repair benefit |
| Correction with continued rewiring (arm I) | `D_I` by post-τ sub-window may decay toward 0 (self-correction) or persist | Decay to zero within the window rejects H1 for those parameters; report the full profile and the degree-only residual |
| Degree-only counterfactual | Mean label gap across many independent runs equals the graph-based algebraic prediction when β=0 | A difference beyond a **Monte Carlo** tolerance (for example 4 standard errors of the mean; one run's `D` has SD ≈0.05 over 400 post-τ draws) signals incorrect graph accounting. A per-run numerical tolerance would fail correct runs |
| 0 opportunity postwindow | Mechanism yields undefined postflow gap; runner refuses contrast | A “valid” inferred effect at no postintervention events is a bug |
| Full replay and checkpoint restoration | Bytewise model states/journals deterministic under pinned environment | Divergence invalidates any claimed result until fixed |

## Proposed controlled pilot phases

**Phase A — reproducibility/invariants:** 6, 10 and 20 actors (N ≥ d+2, so every rewire has a non-neighbor to add and cannot only re-add the removed link), with τ, T and m scaled so the fork is reached (for example τ=40, T=100, m=5), deterministic seeds from a *development-only* domain. Validate no self-edges, stable outdegree, boost balance, graph repair invariance, control-event replay, identical world across forks at τ, valid opportunity accounting.

**Phase B — mechanism isolation:** small exploratory parameter grid with `β∈{0,2,4}`, `κ∈{0,3,6}`, `removal∈{lowest_belief, uniform, none}`, `λ∈{0,0.1}`, and `δ∈{0,0.2}`. `κ` weights only link additions and `removal` sets the removal rule, so the two channels vary separately: `κ=0` with `removal=uniform` is random rewiring at matched churn, and `removal=none` disables rewiring. Vary one factor at a time first, then a clearly bounded factorial. Report the **entire grid**, including negative values, and compare to analytic degree-only projection.

**Phase C — time/structure sensitivity:** repeat with correction early/middle/late in *separate preregistered or exploratory configurations*, and with rewiring stopped at τ as a declared alternative model (the earlier frozen-graph design). Never pick timing based on the pilot's largest effect when interpreting confirmatory runs.

**Phase D — scientific registration:** only if a literature-based novelty or replication justification survives, and only after reliable effect variance estimates across **independent parent societies**, agree on estimand and sample size, immutable seed domain, analysis/exclusion procedure, code and raw-archive retention. Even then, a model-based confirmatory result is not a causal estimate for actual people.

## Public data and ethical considerations

- **MusicLab:** https://www.princeton.edu/~mjs3/data.shtml — published decision/popularity histories are useful for behavioral-choice calibration and demonstrating social influence, but contain no experimental structural network repair as currently identified.
- **Tsvetkova 2021:** https://doi.org/10.6084/m9.figshare.15097107 — author identifies model scripts and processed experimental data. Check full release license and whether graph/decision timestamps are sufficiently detailed for the proposed access outcome.
- **Simpson et al. 2023:** https://doi.org/10.1038/s41467-023-43486-7 — behavior/network rewiring data may support external parameter constraints; permission and exact public-data availability still to verify.

Do not represent a person with a low noisy rating as less intrinsically able. In a future incentivized experiment, false label manipulation and consent/debriefing may require independent ethics review; the present task has not recruited or intervened on any human participants.

## Design decisions (adopted)

The first draft left five choices open. Each was resolved as below and the [formal model](formal-model.md) updated to match; the alternatives remain available as declared variants.

- **D1 — rewiring continues after τ in every arm.** Freezing the graph at τ made `E[D_I]>0` follow from the in-degree inherited at τ, ruled out the self-correction alternative, and was itself an intervention on all arms. With rewiring continuing, lock-in must survive ongoing rewiring and U is a true sham. The frozen-graph design survives only as a Phase C alternative.
- **D2 — amended after 2026 critical review: paired repair-after-truth contrast.** Primary *candidate* `θ = E_parent[D_I^late − D_IS^late]` in steps 501–600. The information-only `E[D_I]` describes persistence, while the paired I–IS contrast directly estimates the within-model structural intervention when truth is already corrected. The previous assertion that pairing has “about twice the variance” was false: `Var(I−IS)=Var(I)+Var(IS)−2Cov(I,IS)`. `E[D_U−D_S]` remains a distinct secondary repair-without-correction contrast. All outcomes/inference are unregistered drafts.
- **D3 — a public reputation drives link additions.** With private beliefs only, beliefs about non-neighbors never updated, so additions preferred boosted actors about 11:1 regardless of evidence. Every observed signal now updates a shared public reputation `P_i`, which weights additions; private beliefs still drive selection and removal.
- **D4 — a separate removal rule.** `removal∈{lowest_belief, uniform, none}` is independent of κ, which weights only additions, so the removal and addition channels can be varied separately; `removal=none` is the no-rewiring baseline.
- **D5 — a d-matched no-network baseline.** The primary no-network baseline resamples d distinct non-self eligible candidates uniformly each step (same choice-set size d and ε floor, no persistent edges; this is not a globally d-regular graph); the complete graph is kept as a secondary baseline with its three-factor confound stated.


### Additional review-derived invariants (before implementation)

- [ ] **Slow rewiring null:** at m=10, τ=200 and T=600 there are only 20 pre-τ and 40 post-τ rewiring events (about 1 and 2 per allocator respectively). Log realized per-allocator replacements, original-edge survival and bias/exposure recovery per rewire; vary m and compare unbiased random churn before attributing persistence to reputation.

- [ ] **Genetic construction equivalent?** No: the V0 initial/repair graphs are independently permuted directed circulant graphs, *not* uniform over all simple d-regular graphs. Validate in/outdegree, reproducibility, permutation invariance, and sample diversity. Document topology-family sensitivity before confirmatory claims.
- [ ] **Recorded setup RNG:** because `Trial::create` creates new streams after receiving a constructed world, pregenesis setup must use a custom Hiyal `Genesis` and persist consumed setup positions through `Trial::create_from_genesis`. Verify fork/resume preservation and forbid world reducer RNG use.
- [ ] **Exact selection/exposure decomposition:** at every accepted event, independently recompute `g_deg` and `g_sel` from the *pre-event* adjacency/belief state; test `β=0 → g_sel=0` within floating-point tolerance, and `E[b_recipient−g]=0` across independent societies.
- [ ] **Late-window fairness contrast:** recompute `D_I^late−D_IS^late` in a separate reference analyzer on steps 501–600, after a shared τ=200 parent; older full-window statistics remain exploratory.
- [ ] **Policy tradeoff gate:** V0 assumes free graph rewiring and equal latent ability. Do not interpret an improvement in simulated fairness as an optimal real intervention.

## Decision before code implementation

Only implement a small, **rule-based and fully replayable** V0 after independent critique of the core graph+belief model. There is no need for LLM agents. Add science-specific code to this repository; keep generic Hiyal simulation abstractions in Hiyal itself. Add only cheap unit/integration CI, not full factorial automatic PR sweeps.
