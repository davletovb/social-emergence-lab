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

- [ ] Check whether another study already tests belief correction × *structural* tie repair after self-fulfilling reputations; forward-/backward-cite Salganik & Watts (2008), van de Rijt (2019), Tsvetkova (2021), Simpson et al. (2023).
- [ ] Confirm substantive theory: define "access" as an opportunity gate, distinguish it from performance or trust, and justify why a graph is the relevant social institution.
- [ ] Validate the conditional-independence and exclusion statements under the graph/reputation dynamic.
- [ ] Derive analytically the **degree-only** expected postallocation advantage when β=0 and the graph is frozen: for uniformly sampled allocator j with d outgoing links, expected share for recipient i is \`in_degree(i)/(N*d)\`. Therefore \`E[D]\` can be written in terms of randomized-boost group in-degree sums. Use it as a **known mathematical structural benchmark**, not a novel social law.
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
| No rewiring + full correction | Postcorrection randomized-boost advantage should approach zero in expectation | Stable gap after many seeds indicates another persistence channel, contrary to simple structural-only hypothesis |
| No preferential allocation (β=0) + regular fixed graph | Uniform choice within eligible neighbors | Any state-dependence fails a probability invariant |
| Correction + independent repaired regular graph | New opportunity gap should approach zero, assuming sufficiently mixed after signal updates | Stable positive gap requires additional mechanism or biased graph selection |
| Original false reputation + no correction | May favor boosted candidates, **not guaranteed** in finite paths | Null/negative outcome is a genuine negative result, not a reason to filter |
| Corrected vs repaired within same prehistory | θ may be positive, zero or negative; quantify uncertainty | A near-zero or negative θ rejects proposed repair benefit for studied parameters |
| Rewiring disabled or exposures identical | θ should approach 0 across independent worlds | Strong effects imply intervention changes more than access |
| Degree-only counterfactual | Expected label gap equals graph-based algebraic prediction when β=0 | Difference above numerical tolerance signals incorrect graph accounting |
| 0 opportunity postwindow | Mechanism yields undefined postflow gap; runner refuses contrast | A “valid” inferred effect at no postintervention events is a bug |
| Full replay and checkpoint restoration | Bytewise model states/journals deterministic under pinned environment | Divergence invalidates any claimed result until fixed |

## Proposed controlled pilot phases

**Phase A — reproducibility/invariants:** 4, 10 and 20 actors, up to 100 accepted steps, deterministic seeds from a *development-only* domain. Validate no self-edges, stable outdegree, boost balance, graph repair invariance, control-event replay, identical world across forks at τ, valid opportunity accounting.

**Phase B — mechanism isolation:** small exploratory parameter grid with \`β∈{0,2,4}\`, \`κ∈{0,3,6}\`, \`λ∈{0,0.1}\`, and \`δ∈{0,0.2}\`; explicitly document that \`κ=0\` must *disable adaptive removals entirely* rather than secretly rewiring at uniform randomness. Vary one factor at a time first, then a clearly bounded factorial. Report the **entire grid**, including negative values, and compare to analytic degree-only projection.

**Phase C — time/structure sensitivity:** repeat with correction early/middle/late in *separate preregistered or exploratory configurations*, and with evolving postintervention networks in a declared alternative model. Never pick timing based on the pilot's largest θ when interpreting confirmatory runs.

**Phase D — scientific registration:** only if a literature-based novelty or replication justification survives, and only after reliable effect variance estimates across **independent parent societies**, agree on estimand and sample size, immutable seed domain, analysis/exclusion procedure, code and raw-archive retention. Even then, a model-based confirmatory result is not a causal estimate for actual people.

## Public data and ethical considerations

- **MusicLab:** https://www.princeton.edu/~mjs3/data.shtml — published decision/popularity histories are useful for behavioral-choice calibration and demonstrating social influence, but contain no experimental structural network repair as currently identified.
- **Tsvetkova 2021:** https://doi.org/10.6084/m9.figshare.15097107 — author identifies model scripts and processed experimental data. Check full release license and whether graph/decision timestamps are sufficiently detailed for the proposed access outcome.
- **Simpson et al. 2023:** https://doi.org/10.1038/s41467-023-43486-7 — behavior/network rewiring data may support external parameter constraints; permission and exact public-data availability still to verify.

Do not represent a person with a low noisy rating as less intrinsically able. In a future incentivized experiment, false label manipulation and consent/debriefing may require independent ethics review; the present task has not recruited or intervened on any human participants.

## Decision before code implementation

Only implement a small, **rule-based and fully replayable** V0 after independent critique of the core graph+belief model. There is no need for LLM agents. Add science-specific code to this repository; keep generic Hiyal simulation abstractions in Hiyal itself. Add only cheap unit/integration CI, not full factorial automatic PR sweeps.
