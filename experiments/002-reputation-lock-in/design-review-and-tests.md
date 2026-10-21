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
- [ ] Derive analytically the **degree-only** expected postallocation advantage when β=0 and the graph is frozen: for uniformly sampled allocator j with d outgoing links, expected share for recipient i is `in_degree(i)/(N*d)`. Therefore `E[D]` can be written in terms of randomized-boost group in-degree sums. Use it as a **known mathematical structural benchmark**, not a novel social law.
- [ ] Document whether independent graph repair is intrinsically desirable, whether it removes useful connections, and what social costs are omitted.
- [ ] Validate whether time τ and opportunity horizon produce enough rewiring to create *meaningfully varying* graphs without forcing a desired outcome.
- [ ] Decide how to handle attempts whose updater has no valid alternative candidate (finite complete neighborhoods, repeated links, overflows).
- [ ] Ensure the repair graph is genuinely independent of randomized B and performance, and identical across repaired arms for the same parent seed.
- [ ] Confirm inferential method, null and minimum meaningful **postopportunity flow** difference; **do not reuse Gini variance/power estimates from Experiment 001**, because it is a different estimand/model.
- [ ] Match at least one model prediction to an externally observable quantity, with a source and scope of empirical validation.

## Candidate falsification tests, *all* must be reportable

| Design test | Predicted/allowed result | How model would be falsified or criticized |
| --- | --- | --- |
| Identical priors δ=0 + balanced fixed graph | Boost label must have zero association with expected reward flow | Systematic nonzero label dependence suggests RNG/order/graph bug |
| No rewiring + full correction | **Exact identity:** the post-τ process is independent of B, so `E[D]=0`; check the mean across seeds within a Monte Carlo tolerance | A stable gap is an implementation error: the model has no other persistence channel |
| No preferential allocation (β=0) + regular fixed graph | Uniform choice within eligible neighbors | Any state-dependence fails a probability invariant |
| Correction + independent repaired regular graph | **Exact identity:** beliefs reset and `A*` independent of B give `E[D_IS]=0` with no mixing assumption; Monte Carlo tolerance as above | A stable gap is an implementation error (for example `A*` depending on B) |
| Original false reputation + no correction | May favor boosted candidates, **not guaranteed** in finite paths | Null/negative outcome is a genuine negative result, not a reason to filter |
| Corrected vs repaired within same prehistory | θ may be positive, zero or negative; quantify uncertainty | Interpret by magnitude, not sign: repair helps only if `abs(E[D_IS]) < abs(E[D_I])`. A negative `D_I` (boosted actors under-served) that repair removes is a repair benefit. Because `E[D_IS]=0` (open decision D2), this reduces to whether `E[D_I] ≠ 0` |
| Rewiring disabled or exposures identical | θ should approach 0 across independent worlds | Strong effects imply intervention changes more than access |
| Degree-only counterfactual | Mean label gap across many independent runs equals the graph-based algebraic prediction when β=0 | A difference beyond a **Monte Carlo** tolerance (for example 4 standard errors of the mean; one run's `D` has SD ≈0.05 over 400 post-τ draws) signals incorrect graph accounting. A per-run numerical tolerance would fail correct runs |
| 0 opportunity postwindow | Mechanism yields undefined postflow gap; runner refuses contrast | A “valid” inferred effect at no postintervention events is a bug |
| Full replay and checkpoint restoration | Bytewise model states/journals deterministic under pinned environment | Divergence invalidates any claimed result until fixed |

## Proposed controlled pilot phases

**Phase A — reproducibility/invariants:** 6, 10 and 20 actors (N ≥ d+2, so every rewire has a non-neighbor to add and cannot only re-add the removed link), with τ, T and m scaled so the fork is reached (for example τ=40, T=100, m=5), deterministic seeds from a *development-only* domain. Validate no self-edges, stable outdegree, boost balance, graph repair invariance, control-event replay, identical world across forks at τ, valid opportunity accounting.

**Phase B — mechanism isolation:** small exploratory parameter grid with `β∈{0,2,4}`, `κ∈{0,3,6}`, `λ∈{0,0.1}`, and `δ∈{0,0.2}`; `κ=0` only flattens link *additions* (formal-model step 6); removal by lowest belief continues, so the no-rewiring baseline must also disable removals (formal-model baseline 3). Whether to add a separate removal parameter is [open decision D4](#open-design-decisions). Vary one factor at a time first, then a clearly bounded factorial. Report the **entire grid**, including negative values, and compare to analytic degree-only projection.

**Phase C — time/structure sensitivity:** repeat with correction early/middle/late in *separate preregistered or exploratory configurations*, and with evolving postintervention networks in a declared alternative model. Never pick timing based on the pilot's largest θ when interpreting confirmatory runs.

**Phase D — scientific registration:** only if a literature-based novelty or replication justification survives, and only after reliable effect variance estimates across **independent parent societies**, agree on estimand and sample size, immutable seed domain, analysis/exclusion procedure, code and raw-archive retention. Even then, a model-based confirmatory result is not a causal estimate for actual people.

## Public data and ethical considerations

- **MusicLab:** https://www.princeton.edu/~mjs3/data.shtml — published decision/popularity histories are useful for behavioral-choice calibration and demonstrating social influence, but contain no experimental structural network repair as currently identified.
- **Tsvetkova 2021:** https://doi.org/10.6084/m9.figshare.15097107 — author identifies model scripts and processed experimental data. Check full release license and whether graph/decision timestamps are sufficiently detailed for the proposed access outcome.
- **Simpson et al. 2023:** https://doi.org/10.1038/s41467-023-43486-7 — behavior/network rewiring data may support external parameter constraints; permission and exact public-data availability still to verify.

Do not represent a person with a low noisy rating as less intrinsically able. In a future incentivized experiment, false label manipulation and consent/debriefing may require independent ethics review; the present task has not recruited or intervened on any human participants.

## Open design decisions

These choices change what the experiment can show, so they need the author's decision before code. Each lists options and a recommendation.

- **D1 — rewiring after τ.** The draft freezes the graph at τ in every arm. Any in-degree skew toward boosted actors built before τ (about 20 rewires at m=10) then persists unchanged in arm I, so `E[D_I]>0` follows from the degree-only formula rather than from a finding, and the competing self-correction explanation ("a network may recover if rewiring continues") cannot occur. Freezing is also itself an intervention applied to all four arms, so U is not a pure sham. Options: (a) keep the freeze and present H1 as a degree benchmark, not a test; (b) continue rewiring after τ in every arm, so lock-in must survive ongoing rewiring and U becomes a true sham; (c) run both as separately declared models. *Recommendation:* (b).
- **D2 — primary estimand.** `E[D_IS]=0` exactly by label symmetry, so `θ = E[D_I − D_IS] = E[D_I]` and H2 restates H1 at about twice the variance. This also holds under D1(b), because in IS every input to the post-τ process is independent of B. Options: (a) make `E[D_I]` the primary estimand and keep IS as an exact-zero implementation control; (b) make repair non-degenerate (for example repair a fraction of links, or repair without resetting beliefs) so a repair contrast measures something; (c) both. *Recommendation:* (c), with (a) primary.
- **D3 — beliefs about non-neighbors.** Only the allocator who observed a candidate updates its belief, so beliefs about non-neighbors keep their false priors forever. With κ=6, every link addition then prefers a boosted actor about exp(6×0.4)≈11:1 whatever evidence has arrived, and the self-correction channel (van de Rijt, 2019) never reaches rewiring. Options: (a) keep private beliefs and state that additions are driven by priors; (b) a public reputation updated by every observed signal drives additions; (c) decay unobserved beliefs toward a population estimate. *Recommendation:* (b).
- **D4 — separate removal rule.** κ weights only additions; removal is always by lowest belief. Options: (a) keep, and treat "no rewiring" as κ=0 plus removals disabled (as baseline 3 does); (b) add a parameter `removal ∈ {lowest_belief, uniform, none}` so the removal and addition channels can be varied separately. *Recommendation:* (b).
- **D5 — no-network baseline.** The complete-graph baseline changes d, the ε floor and rewiring together. Options: (a) keep it and report it as a three-factor change; (b) a d-matched annealed baseline that resamples a uniform d-regular graph every step, keeping d=4 and the choice-set size while removing persistent structure. *Recommendation:* (b) as the primary no-network baseline, keeping (a) as a secondary.

## Decision before code implementation

Only implement a small, **rule-based and fully replayable** V0 after independent critique of the core graph+belief model. There is no need for LLM agents. Add science-specific code to this repository; keep generic Hiyal simulation abstractions in Hiyal itself. Add only cheap unit/integration CI, not full factorial automatic PR sweeps.
