# Experiment 002 — independent critical desk review and updated novelty assessment

**Reviewed:** 2026-10-21 · **Status:** critical research desk review, *not* external peer review or a systematic evidence synthesis. **Decision:** GO for a *small, explicitly exploratory, rule-based pilot after fixing design issues*; NO-GO for confirmatory study, policy recommendations or a claimed original sociological law.

## 1. Literature search, boundaries and decisions

This update targeted empirical field/lab studies on reputation, manipulated popularity, endogenous network formation, structural interventions, and misinformation correction. Search themes included *reputation network rewiring correction*, *information intervention durable network*, *reputation systems field experiment 2026*, *edge interventions inequality*, and *network misinformation correction*. Sources were checked at publishers, bibliographic indexes and available author manuscripts as of 2026-10-21. The earlier literature matrix in [literature-review.md](literature-review.md) remains part of the record.

**Not a systematic review:** no comprehensive citation export, citation-graph closure, registration, exhaustive screening, negative-result database search or 2026 bibliometric completeness claim. No human researcher other than this automated desk review has yet endorsed the design.

| Source / publication date | What it established | Novelty impact on Experiment 002 | Direct match? |
| --- | --- | --- | --- |
| Salganik & Watts, *Leading the Herd Astray*, Social Psychology Quarterly **2008**, https://doi.org/10.1177/019027250807100404 | False initial popularity can produce self-fulfilling rankings in a controlled online cultural market; superior choices may regain popularity | False reputation → success is **not** novel | No randomized network repair after truth correction |
| van de Rijt, *Self-Correcting Dynamics in Social Influence Processes*, AJS **2019**, https://doi.org/10.1086/702899 | Positive feedback is compatible with eventual recovery of superior options; correction is a serious rival mechanism | Blanket irreversibility is **not** defensible | No comparable opportunity graph intervention in abstract |
| Tsvetkova, *The Effects of Reputation on Inequality in Network Cooperation Games*, Phil. Trans. R. Soc. B **2021**, https://doi.org/10.1098/rstb.2020.0299 | Model plus nine experimental datasets; reputation can decrease payoff dispersion in randomly rewired networks | Reputation need not exacerbate inequality; network dynamics are central prior art | Not the exact false-label correction × repair experiment |
| Simpson, Montgomery & Melamed, *Reputations for Treatment of Outgroup Members Can Prevent the Emergence of Political Segregation*, Nature Communications **2023**, https://doi.org/10.1038/s41467-023-43486-7 | Large (N=1,073) cooperative network experiment: reputation disclosure format changed network segregation | Information and endogenous ties jointly affecting outcomes is **not** novel | Different reputation manipulation, causal estimand and population |
| Barnes et al., *Edge interventions can mitigate demographic and prestige disparities in the Computer Science coauthorship network*, **2025 preprint**, https://arxiv.org/abs/2506.04435 | Simulated edge additions using an observed collaboration network can alter centrality and predicted status disparities | Network *repair* as an intervention is **not** novel | Does not cross perfect correction of false reputations with repair |
| Qiao et al., *Dynamic Simulation Framework for Disinformation Dissemination and Correction With Social Bots*, Findings of EMNLP **November 2025**, https://doi.org/10.18653/v1/2025.findings-emnlp.252 | Agent-based diffusion/correction strategies with realistic social-network topology | Computational misinformation correction is an established adjacent area | Outcome is misinformation propagation, not subsequent opportunity flow |
| Liu et al., *Efficient Edge Rewiring Strategies for Enhancing PageRank Fairness*, **January 2026 preprint**, https://arxiv.org/abs/2602.02512 | Optimize structural rewiring of existing networks to reduce group-level unequal PageRank/exposure | An even closer example of explicit fairness-oriented edge repair, but not the false-belief correction × recovery process | Graph ranking/exposure optimization rather than randomized-history reputation correction |
| Holtz, Dow & Aral, *Can Social Networks Substitute for Reputation Systems?*, ICWSM **May 2026**, https://doi.org/10.1609/icwsm.v20i1.42680 | Randomized rollout of ratings in a large online marketplace found **no average effect** on activity, user behavior or marketplace structure over six months | The reputational effect may be weak or absent; a **negative empirical prior** is mandatory | Ratings introduction is not correction of false reputations |
| Papamichalis, Forastiere, Airoldi & Christakis, *Educational intervention can rewire village social networks*, Nature **14 October 2026**, https://doi.org/10.1038/s41586-026-11154-7 | Randomized health-information intervention among **8,331 people in 110 Honduran villages** changed ties, including persistent friendship/financial network effects; degree distribution alone can miss tie changes | Information-induced persistent network rewiring is **already supported empirically**. Strongest newly identified challenge to a broad originality claim | Different context; not a false-reputation correction × post-access repair factorial |

### Verdict by candidate claim

| Candidate scientific claim | Verdict |
| --- | --- |
| Misinformation can become self-fulfilling | **Known** |
| Network rewiring can create lasting differences in access or segregation | **Known/strongly anticipated** |
| Information interventions can change social network links durably | **Demonstrated (including October 2026 publication)** |
| Structural edge interventions can improve simulated inequality metrics | **Already studied** |
| Truth-correction alone necessarily fails | **Too strong; may contradict self-correction evidence** |
| A joint correction + independent repair intervention changes *posttruth opportunity recovery time*, beyond an analytic graph-only prediction, for a specific rule family | **Mechanism-specific candidate; originality unestablished** |
| The model gives an actionable estimate for humans | **No**, absent behavioral/graph calibration and, potentially, new controlled data |

**Do not describe the future pilot as a discovery or a test of a universal sociological law.** If we cannot identify a difference from the analytic degree-only explanation and existing network interventions, the scientific payoff will be a useful *negative result* and a reproducible simulation benchmark.

## 2. Critical design issues found

### A. The primary estimand must compare counterfactual repairs, not only one arm

The merged protocol defines the primary quantity as `E[D_I]` (posttruth false-boost opportunity gap in information-only arm) and calls `E[D_I - D_IS]` degenerate, claiming it has “about twice the variance”. That variance statement is unjustified.

Mathematically `E[D_I-D_IS]=E[D_I]-E[D_IS]`, and under an explicit exchangeability null `E[D_IS]=0`. **Yet** the paired variance equals `Var(D_I)+Var(D_IS)-2Cov(D_I,D_IS)`, which can be *larger or smaller* than `Var(D_I)`. Further, the within-parent contrast is the direct **causal effect of graph repair conditional on truthful correction**, while `E[D_I]` alone describes persistence.

**Recommendation:** candidate primary intervention contrast `θ_repair|truth = E_parent[D_I^late - D_IS^late]` over a predeclared *late* post-τ window; full-window `E[D_I]` and its components remain co-primary diagnostics/secondary estimands until preregistration. **Do not fix a null-testing procedure or decision rule yet.** If one wants only to know whether any gap survives information correction, `E[D_I]` is an appropriate descriptive target, not an independent policy effect.

### B. Separate exact graph exposure from belief-mediated selection at every step

Let `b_i∈{-1,+1}` be the randomized initial false-boost indicator; `A_{ji,t}` the current directed opportunity adjacency; d the fixed allocator outdegree; and let `p_{ji,t}` be the exact policy probability for eligible i given allocator j.

For uniformly randomly scheduled allocators, define pre-event conditional expectation of the boost group's **signed** next opportunity:

```text
g_t       = (1/N) Σ_j Σ_{i:A_ji,t=1} b_i p_ji,t
g_t^deg   = (1/(N*d)) Σ_i b_i in_degree_i(A_t)
g_t^sel   = (1/N) Σ_j Σ_{i:A_ji,t=1} b_i [p_ji,t - 1/d]

IDENTITY:   g_t = g_t^deg + g_t^sel
UNDER β=0: g_t^sel = 0 exactly, hence g_t = g_t^deg.
```

These are **known mathematical accounting identities, not a discovered sociology law**. The observed signed winner indicator is `b_recipient`; its innovation noise `b_recipient - g_t` should have zero conditional mean if sampling is implemented correctly.

This decomposition is essential: a positive `D_I` could be *nothing more than inherited in-degree*, or could reflect belief re-formation after correction. Report time-averaged `g^deg`, `g^sel`, observed signed flow and the difference between the observed flow and its conditional expected flow. For structural explanation, compare a separate **β=0** allocation with the same graph history where feasible, and an independent graph generator.

### C. Graph initialization claims unjustified uniformity

The current formal model claims a **uniform draw over all simple directed d-in/d-out-regular graphs** “for example configuration pairing with rejection,” but does not provide a sampler and correctness argument. Naive pairings generate multi-edges/self-edges; rejection may be astronomically inefficient, and generic rewiring chains have mixing uncertainty.

**Recommendation for V0:** define and test an **exact balanced, explicitly nonuniform permutation-of-circulant generator** (independently seeded node permutation, directed offsets `1..d`). It guarantees no self-loops and in/outdegree d, and makes no unproved uniform-distribution claim. Label assignments, initial graph and pre-drawn repair graph use separate setup substreams. Add graph-family robustness later (degree-preserving edge swaps with stated mixing limitations), not as a silent model change.

### D. Correct Hiyal genesis/setup RNG ownership

Current `Trial::create` creates `initial_streams = Streams::new(trial,seed)` **after** the supplied world is constructed. Consequently “draw B, graph and A* from trial-owned `setup` streams at genesis” cannot be implemented by drawing from an unrelated temporary RNG and then calling `Trial::create`. We must explicitly construct a serializable `Genesis` with the *advanced, audited setup stream positions* and call Hiyal `Trial::create_from_genesis`, or define an independently specified deterministic world-construction algorithm with fully recorded seed derivation and no claim to the trial's setup-stream replay. `scheduler` must remain owned by Hiyal and each event action must materialize the noisy performance and rewiring results; `World::apply` must consume **no RNG**.

No agent ability variation is allowed in V0, so selecting and observing performance are separable as specified.

### E. Exact-zero claims require stated symmetry assumptions

`E[D_IS]=0` is an **expectation under exchangeability**, not a promise that any seed yields exactly zero realized gap. The claim assumes the repaired graph and future transition/RNG law are B-independent, equal quality, no wealth→selection path, no hidden label-index-based tie-breaks, and proper symmetric randomization. Validate both exact per-state identities (when appropriate) and Monte Carlo tests with prechosen tolerances.

Likewise `E[D_I]=0` under no rewiring and corrected belief is a model-conditional expectation, not a finite-run identity. Unequal realized rewards are normal.

### F. Sparse rewiring creates a built-in slow-recovery confound

The draft's `m=10, τ=200, T=600, N=20` entails **20 total pre-τ rewiring events** and **40 post-τ events**, assuming exactly one rewire per multiple of 10. Rewiring allocators are uniformly chosen, so each allocator undergoes **about one pre-τ and two post-τ rewires on average**. That is sparse relative to each allocator's four outgoing links. Even without any false reputation, turnover of an inherited biased graph may therefore be slow.

**Required test:** measure replacement events per allocator, surviving pre-τ edge fraction, boost-signed in-degree advantage over time, and recovery as a function of **cumulative rewiring opportunities** rather than solely elapsed reward events. Compare matched **β=0** graph dynamics and random churn (`removal=uniform, κ=0`), and vary rewiring frequency m in a clearly exploratory sensitivity check. A long-lived gap that disappears at faster *unbiased* turnover is a basic structural relaxation finding, not evidence of unique reputational persistence.

### G. Mechanism/policy scope

With identical quality, zero opportunity costs, one fixed reward token per allocation and freely replaceable links, network repair carries **no welfare or information tradeoff**. A positive fairness effect could be a near-tautological result of equalizing exposure. It is not evidence that reparative recommendations are optimal in human settings.

After V0 model verification, separate extensions should include *costly link creation*, imperfect correction credibility, heterogeneous but observable performance and competing efficiency measures. Do not add these while first validating the mechanism.

## 3. Provisional scientific contribution after this review

> **The model is a testbed for the recovery time and composition of a randomized, false-reputation advantage after a truthful information reset, under ongoing endogenous rewiring.** It asks whether any late-window opportunity gap is completely attributable to observable graph exposure, or whether reputation dynamics regenerate selection bias; and how paired structural graph repair changes that recovery trajectory.

This is narrower and more falsifiable than “reputation lock-in” or “network repair works.” It could still turn out to be obvious or fully explained by a simple Markov model; **the no-novelty outcome is acceptable**.

## 4. Decisions for the project

- **GO:** implement only the small deterministic Rust+Hiyal **V0 correctness and mechanism-isolation pilot**, after applying the precise graph/RNG/estimand clarifications. Cost target: local CPU, no LLM, no full grid in PR CI.
- **HOLD:** claims of first discovery, confirmatory preregistration, human-policy extrapolation and larger runs.
- **OPEN:** more formal backward/forward citation screening through 2026 and external academic reviewer; simulation cannot substitute for human-independent assessment.
- **OPEN:** empirical measurement/construct match; see [dataset feasibility](empirical-alignment.md).
- **OPEN:** original Experiment 001 evidence permanence in [issue #6](https://github.com/davletovb/social-emergence-lab/issues/6).

## Verified sources and provenance notes

Citations above are to publisher proceedings, publisher abstracts or accessible paper text. Dates are publication dates, not dates those experiments happened. 2025 Barnes is a **preprint** as checked here, not automatically a published peer-reviewed result. The October 2026 Nature article used **controlled-access** human network microdata; its accompanying analytical code is public. See [empirical-alignment.md](empirical-alignment.md).

This review is **not independent external peer review**. It is an independent *critical pass on the previous draft*, performed with a broader set of sources and specific mechanism-level objections.
