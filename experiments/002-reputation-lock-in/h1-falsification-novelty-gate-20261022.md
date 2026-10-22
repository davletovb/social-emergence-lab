# Experiment 002 — H1 falsification and novelty decision gate

**Assessment:** 22 October 2026. **Reviewer:** automated critical desk review; **not independent external expert review, not systematic literature review and not preregistration**.

## Decision

**NO-GO** for naming the present 36-society result "continuing reputation lock-in", "novel sociological law", or an empirically demonstrated human effect. **GO** for reproducible modeling of how a temporary false label changes the *inherited opportunity graph* and a focused sensitivity/negative-control study. The empirical and novelty bars are unmet.

**Why this changes the original interpretation:** The original `research-question.md` and `formal-model.md` state that H1 needs persistence **beyond** opportunity links inherited from the correction checkpoint. The original 36-society data instead contain a large late positive information-only signed gap, but almost no excess from new edges plus conditional reputation choice.

## Reproducing the core descriptive assessment from committed results

Source: `results/v2-36-paired-20261021/h1-decomposition.json`; N=20,d=4, uniformly scheduled allocators, intervention τ=200 and horizon T=600; last 100 postintervention opportunities, **36 independent parent seeds** 98000–98035. The labelled groups are artificial, not human populations. All intervals are approximate exploratory 95% t intervals, unadjusted, conditional on the coded model.

| Information-only fork, last 100 steps | δ=0.2 false-prior label | δ=0 initial-prior null |
| --- | ---: | ---: |
| Realized signed opportunity gap | +0.21833 (0.16823, 0.26844) | +0.02778 (−0.02519, 0.08075) |
| Continuously surviving checkpoint-edge exposure | **+0.22389** (0.20484, 0.24293) | +0.02365 (−0.00084, 0.04813) |
| Postcheckpoint new/re-added-edge exposure | **−0.00549** (−0.02769, 0.01672) | +0.01552 (−0.01240, 0.04344) |
| Belief-mediated conditional selection | **+0.00708** (0.00112, 0.01305) | +0.00404 (−0.00312, 0.01121) |
| **New-edge + belief selection (stronger H1 gate)** | **+0.00159** (−0.02144, 0.02463) | +0.01956 (−0.01081, 0.04993) |
| Mean continuously surviving checkpoint-edge fraction | 0.65843 | 0.65819 |

Algebraic exposure identity, in expectation over the next scheduled allocator: `expected_signed_access = surviving_checkpoint_links + newly_added_links + conditional_choice_bias`; the realized signed next opportunity adds finite-sample innovation. These quantities are **accounting**, not causal decompositions of separately manipulated mediators. Beware interactions between exposure and choice and dependence on a fixed β=4 / κ=6 / m=10 policy.

**Main inference:** The positive late observed gap is compatible with **structural memory** of the initially misinformed period, not an ongoing feedback-induced regeneration of unequal links after true correction. The estimate for the stronger H1 quantity is close to zero, and its interval crosses zero. This does **not prove the continuing mechanism is absent** at other parameters, horizons, networks, or even with greater precision, but the existing experiment does not establish it. Calling the full signed gap itself "novel reputation lock-in" would be misleading.

**Nuance:** The **+0.00708** belief-choice term alone has an interval slightly above zero; that still does *not* rescue the stronger H1 as specified: its combined effect with new-link exposure is near zero. Moreover, this component depends on EMA updates under identical intrinsic qualities and may reflect stochastic re-formation of beliefs rather than continuing the original false belief.

### The gap decays at the link-turnover rate

Each run's report also holds four 100-event windows after τ. For the δ=0.2 information-only fork over the same 36 parents ([h1-quarters.json](results/v2-36-paired-20261021/h1-quarters.json)):

| Window | Late gap | Surviving-link exposure | Checkpoint links still present | Exposure per surviving link | New links + selection |
| --- | ---: | ---: | ---: | ---: | ---: |
| 201–300 | +0.287 | +0.316 | 94.4% | 0.334 | −0.003 |
| 301–400 | +0.256 | +0.279 | 83.4% | 0.335 | −0.002 |
| 401–500 | +0.256 | +0.252 | 73.7% | 0.341 | +0.000 |
| 501–600 | +0.218 | +0.224 | 65.8% | 0.340 | +0.002 |

The exposure each surviving checkpoint link carries stays constant, near 0.34, and the new-link plus selection term stays near zero in every window: the gap shrinks only as checkpoint links are replaced. Checkpoint links survive about 88–89% of each 100 events, a half-life of roughly 560–610 events, close to what near-uniform removal predicts. With one rewiring every m events among N·d links, a checkpoint link survives a rewiring with probability about 1 − 1/(N·d), so after t events

```text
surviving fraction S(t) ≈ (1 − 1/(N·d))^(t/m)        (N·d = 80, m = 10: 0.882 per 100 events)
late gap(t)       ≈ c · S(t),  c ≈ 0.34 here
```

That is a quantitative **slow-relaxation** account of the whole post-correction gap. It gives Issue #19's turnover grid a prediction to test before it runs: link half-lives of about 110, 550 and 1,100 events for m = 2, 10 and 20. The constant c will differ between grid cells, because m also changes the history before τ and so the checkpoint graph's imbalance. So the testable claims are that, within each cell, the gap divided by the surviving fraction stays constant over the four windows, and that new links plus selection stays near zero. A gap that stays above c·S(t), or a positive new-link plus selection term, would be evidence of regeneration after correction.

### Structural null capable of the same qualitative persistence

The exact degree-only exposure under unbiased per-neighbor selection is

```text
g(degree) = (1 / (N·d)) Σ_i label_sign(i) · indegree(i).
```

If truthful correction resets beliefs, β=0 (or all beliefs tied), rewiring is disabled, and the checkpoint graph has nonzero group in-degree imbalance, then **the expected postcorrection signed opportunity gap remains unequal indefinitely** without any active misinformation or biased choice. This is a mathematical counterexample to an inference from `D_I^late > 0` to ongoing reputation lock-in. A focused Rust test demonstrates it at N=10,d=2 with exactly one edge shifted toward a boosted label, showing positive expected gap = 2/(N·d) and zero new-link/selection effects throughout subsequent learning.

A second small test establishes that in a balanced regular graph, independently drawn at setup, resetting beliefs and replacing the graph makes **the expected signed access gap exactly zero at intervention**, not each realized opportunity sequence. This distinction prevents misreporting stochastic arm outcomes as exact zeros.

### Four-fork redundancy warning

For all 36 observed seeds, `paired_repair_difference` equals `matched_seed_prior_effect` up to floating-point tolerance. This is **not** corroboration from an independent mechanism. Graph and belief reset makes the policy inputs coincide, and in a review reproduction of all 36 pairs the two `IS` forks also started from identical stream positions and made identical allocation decisions. The number of draws per event does not depend on δ; positions could only diverge through a rejection redraw in a tie-break draw, whose probability is of order n/2⁶⁴ per event. Keep the treatment estimands conceptually distinct, but never advertise these two observed contrasts as separate supporting findings.

## 2026 literature novelty triage

This pass explicitly adds a highly direct prior-art work absent from the old selected bibliography:

| Study / permanent source | Relevant established result | Consequence for originality |
| --- | --- | --- |
| **Frey & van de Rijt (2016), _Arbitrary Inequality in Reputation Systems_**, Scientific Reports 6:38304, [DOI 10.1038/srep38304](https://doi.org/10.1038/srep38304) | A laboratory trust-game experiment with initially comparable exchange partners found **arbitrary exchange-volume inequality** driven by early reputational differences and cumulative partner selection. It also discusses reputation cascades and a model. | **Direct and important prior art**: temporary chance/reputation advantage → unequal opportunities among equally worthy actors is emphatically **not novel**. Read its supplement and cite it centrally before writing any introduction. |
| **Salganik & Watts (2008), _Leading the Herd Astray_**, [DOI 10.1177/019027250807100404](https://doi.org/10.1177/019027250807100404) | Experimentally manipulated initial popularity, with self-fulfilling and potentially self-correcting dynamics. | False ranking → enduring popularity is **not novel**; self-correction must remain a live competitor. |
| **van de Rijt (2019), _Self-Correcting Dynamics in Social Influence Processes_**, [DOI 10.1086/702899](https://doi.org/10.1086/702899) | Feedback processes can yield recovery and correction rather than permanent dominance. | The research cannot assume lock-in from positive feedback alone. |
| **Tsvetkova (2021), _Effects of Reputation on Inequality in Network Cooperation Games_**, [DOI 10.1098/rstb.2020.0299](https://doi.org/10.1098/rstb.2020.0299) | Reputation effects on inequality can be welfare-equalizing in networks, depending on the cooperative game and network dynamics. | The sign of reputational effects is not universal. |
| **Holtz, Dow & Aral (2026), _Can Social Networks Substitute for Reputation Systems?_**, ICWSM 20, [DOI 10.1609/icwsm.v20i1.42680](https://doi.org/10.1609/icwsm.v20i1.42680) | Large randomized rating-system rollout in an online marketplace found no average effects on marketplace activity, behavior or network structure over six months. | A strong empirical **negative-prior** against claims that reputation intervention necessarily shifts actual opportunity flow. Ratings introduction is not identical to historical false-prior correction. |
| **Papamichalis et al. (14 October 2026), _Educational intervention can rewire village social networks_**, Nature, [DOI 10.1038/s41586-026-11154-7](https://doi.org/10.1038/s41586-026-11154-7) | Randomized educational intervention in 110 villages changed formation and persistence of ties, sometimes enduring several years; overall degree statistics could remain stable. | Information-driven durable network rewiring is **known**. This did not test false reputation correction or independent structural repair, so do not claim exact duplication. |

The latest specific publication dates and DOI links were checked against publisher or indexed abstracts; Frey & van de Rijt (2016, Scientific Reports; Vincenz Frey, laboratory trust-game experiment), Holtz, Dow & Aral (2026, ICWSM 20, field experiment) and Tsvetkova (2021, Phil. Trans. R. Soc. B 376:20200299) were re-checked against their PubMed Central or AAAI proceedings records in review on 22 October 2026. This is **a targeted audit, not an exhaustive citation search**. A genuinely independent human scientist should review the candidate claim and the original 2016 paper's supplemental model, alongside 2025–2026 citations and any registered nearby ongoing studies.

## Next experimental design: maximum discriminative value, minimal CI

1. **Veto obvious structural-only persistence:** report postcorrection `late(new_links + selection)` as *the mechanism-specific falsification metric* for the existing model, retaining the original gap as a descriptive context outcome. Do not silently redefine an earlier primary outcome after looking at outcomes; a new exploratory protocol must explicitly mark this as a follow-up.
2. **Change turnover while holding everything else constant:** use a **prespecified** fixed small parameter grid of `rewire_every ∈ {2,10,20}`, across a fresh disjoint seed set; compare inherited fraction, newly formed edge exposure, and late selection against the slow-relaxation prediction above (gap ÷ surviving fraction constant over the four windows, new links plus selection near zero). The intervention fork must be identical within each parent history. A faster schedule may eliminate structural inheritance without demonstrating a new reputational effect.
3. **Separate rewiring mechanisms:** uniform removal/addition (random churn), lowest-belief removal with κ=0, and current public-reputation-biased κ=6 additions. Current controls do not isolate each component cleanly.
4. **Predefine an analytic graph-only counterfactual:** for each checkpoint graph, compute `g^degree` with uniform partner choice; for frozen network, this is a time-invariant exact expected gap. Use this as baseline for all postcorrection access estimates, and document that difference from it can be due to **network evolution or choice**, not solely a lasting misinformation effect.
5. **Run a small feasibility grid manually outside routine PR CI**, include outcomes in all cells and failure counts; do not select favorable settings after inspecting results. Do not use the same 36 developmental seeds for later confirmatory testing.
6. **External review and construct validation:** inspect prior work's supplemental models and data rights; only after finding a nontrivial distinct mechanism should we consider a preregistered human study or a scientifically ambitious paper.

**Decision gate:** The present code and replay infrastructure are good enough for meaningful *falsification attempts*, but current results justify **structural persistence in this model**, not a new self-sustaining reputation law. The next useful result would be either (a) persistence driven by genuinely new access links/choice after factual correction under robust turnover controls or (b) a clear null finding that stops the project from overclaiming.
