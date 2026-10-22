# Experiment 002 V3 — independent scientific-review packet

**Prepared:** 2026-10-22. **Purpose:** invite critical *human* review of the completed exploratory synthetic study; this document is **not** a peer review, independent endorsement, preregistration or claim of novelty. Tracking: [Issue #22](https://github.com/davletovb/social-emergence-lab/issues/22).

## Two-page decision brief

**Core question.** After a completely arbitrary false reputation advantage is corrected, does post-correction inequality persist because the *opportunity graph* carries historical ties forward, or because active reputation feedback creates a fresh advantage?

**Prior-art constraint.** The broad claim "early arbitrary reputations can generate unequal exchange opportunities" was established experimentally by [Frey & van de Rijt (2016)](https://doi.org/10.1038/srep38304). [van de Rijt (2019)](https://doi.org/10.1086/702899) makes self-correction a serious rival account. [Tsvetkova (2021)](https://doi.org/10.1098/rstb.2020.0299) shows reputation/rewiring may sometimes **reduce**, not amplify, inequality. [Papamichalis et al. (2026)](https://doi.org/10.1038/s41586-026-11154-7) already documents long-lived information-driven network changes. **Do not describe any of these broad mechanisms as a novel discovery.**

**Model.** N=20 equally capable synthetic individuals, directed choice graph with fixed outdegree d=4, randomly boosted/unboosted group labels, false prior ±δ=0.2, softmax selection β=4, reputation-preferred addition κ=0 or 6, correction at τ=200 and observation through T=600. Every cell uses 48 *independent parent seeds*; matched false-prior and δ=0 parents each branch into four interventions. Unit of analysis: parent seed, *not* number of events or forks.

**Nine-condition factorial.** For each network rewire period m ∈ {2,10,20}, compare (a) uniform removal plus random addition, (b) remove lowest-belief neighbor plus random addition, and (c) remove lowest-belief neighbor plus reputation-biased addition. The model is *specified synthetic behavior* rather than an identified model of humans.

**Pre-event exposure identity**, averaged over eligible allocators: signed expected access = continuously inherited checkpoint-edge exposure + postcheckpoint new/re-added-link exposure + conditional choice-selection exposure. The *primary* posttruth active-regeneration quantity is new-link exposure + selection. The realized access gap also contains stochastic innovation. The components are **accounting, not causally identified independent mediator effects**. Comparing rules is an intervention, but changes to removal and addition interact with historical network evolution.

**Primary status from [PR #20](https://github.com/davletovb/social-emergence-lab/pull/20):** 9/9 cells computed (432 parents); 0/9 positive regeneration decisions. The full-feedback m=10 and m=20 cells follow slow structural relaxation. The m=2 full-feedback cell was **inconclusive under the frozen Stage A rule** (negative rather than positive new-link-plus-selection estimate); under *post hoc* Amendment 1 it looks compensatory but that is not a frozen Stage A finding. At fast *belief-deletion-only* (m=2), Stage B's prespecified amended mean and median rules both classified **compensation** (new opportunities favor the initially disadvantaged group). Random-turnover cells give *identical* post-correction false-prior and δ=0 fork histories and should not be mistaken for independently replicated treatment effects.

**Sequencing warning.** The original Stage A three full-feedback cells were run first, using the original frozen decision rule. **Amendment 1 was written after their outcomes, before the other six Stage B cells.** It introduced compensation, median/sign checks, and null-adjusted estimates. Respect the difference: do not retroactively count Stage A's amended labels as predeclared, or claim all three Stage A cells met slow relaxation.

**Scientific interpretation.** The evidence supports a plausible *within-model* slow structural-memory account over active regeneration at these fixed parameters. It does not prove absence of regeneration elsewhere; it does not estimate a human causal effect, prove model realism, or establish a novel sociological law. A positive total gap by itself is not active lock-in.

## Questions for an independent reviewer

1. Is "structural memory vs active reputation regeneration" theoretically informative after Frey & van de Rijt's 2016 experiment/theory, van de Rijt's 2019 correction work, Tsvetkova's 2021 network cooperation study and Papamichalis's 2026 information-network trial? Please inspect supplements and forward citations, and say precisely what is **not** new.
2. Does conditioning on continuously surviving edges answer the hypothesis, or risk conditioning on a posttreatment mediator / survivor selection? Which *causal intervention contrasts* could actually distinguish inherited opportunities from regenerated opportunities?
3. Is the new-link-plus-selection quantity an adequate operational definition of active lock-in when the graph's incoming degree and the selective deletion process interact? What negative control or intervention would falsify that choice?
4. Given Stage A m=2 negative compensation, the Stage A-to-B protocol amendment, multiple cells/windows, and selected ±0.05 equivalence margin, which inferences deserve any uncertainty language? Would randomization inference / hierarchical posterior predictive diagnostics be preferable?
5. Is the zero-prior symmetry condition a valid negative control under these model rules? How should we interpret two originally flagged Stage A null intervals, and the exact identity of random-turnover posttruth treatment and control forks?
6. Is the apparent compensation at m=2 belief deletion simply a mechanical side effect of deleting the lowest-belief partner and candidate restrictions? Does earlier sociology/network dynamics literature already predict that sign?
7. Which graph families (degree heterogeneity, undirected reciprocity, transitivity, community structure), heterogeneous qualities and learning rates, and policy alternatives are the *minimum* generality tests—and which would be scientifically motivated rather than parameter fishing?
8. Is the strongest contribution a negative-result benchmark / computational-methods note rather than a new substantive sociology paper? What independent human or behavioral comparison would change that verdict?

**Requested deliverable from the reviewer:** a dated written assessment distinguishing *major threats to validity*, originality/prior-art conflicts, tests that might overturn the interpretation, and a GO/NO-GO decision on additional simulation and publication framing. Retain critical and negative feedback unedited.

## Empirical feasibility: verified availability versus unverified variables

| Study / record | Externally confirmed availability | Plausible test | What is **not** established |
|---|---|---|---|
| [Tsvetkova 2021](https://doi.org/10.1098/rstb.2020.0299); [processed data and model/scripts on Figshare](https://doi.org/10.6084/m9.figshare.15097107) | Paper states processed experimental datasets and scripts are deposited; paper analyses nine network-cooperation experiments | Sign/magnitude of reputational information effects on choice and inequality; network process if time-stamped dyads exist | **No exact files/columns, edge timestamps, applicable record-specific license, or rights were inspected yet.** No evidence of an exogenous false-label truth-reset × graph-repair intervention |
| [Frey & van de Rijt 2016](https://doi.org/10.1038/srep38304) and its supplementary information | Full paper/supplement accessible; 336 laboratory subjects in trust games | Choice-volume concentration under accumulated reputation | No verified public participant-level event dataset or matching truth-correction intervention |
| [Papamichalis et al. 2026](https://doi.org/10.1038/s41586-026-11154-7); [analysis-code DOI](https://doi.org/10.5281/zenodo.22004783) | Nature states code is deposited and reports edge dynamics from 110 Honduran villages | Tie formation, dissolution, information exposure and persistence | **Individual-level and edge-list data are controlled-access:** Yale data-use agreement and IRB approval, no redistribution. No false-reputation correction manipulation. Only aggregate published results are accessible without approval |
| [Princeton MusicLab archive](https://www.princeton.edu/~mjs3/data.shtml) | Public experimental data described; download conditions vary | Self-fulfilling initial popularity after manipulated information | Not a graph-repair or directed tie-turnover design |

**Next empirical action, not yet completed:** inspect the exact Tsvetkova Figshare manifest, licenses and column dictionary (possibly via published files and a local read-only audit); verify pre-choice belief, eligibility graph, event timestamps, and tie creation/deletion *separately*. A compatible benchmark must actually contain the observable it is claimed to calibrate. Do not equate a public metadata page with authorized access to participant-level private network data.

## Evidence and source inventory

- [Original falsification and novelty gate](../experiments/002-reputation-lock-in/h1-falsification-novelty-gate-20261022.md) and [V2 36-seed paired results](../experiments/002-reputation-lock-in/results/v2-36-paired-20261021/README.md).
- [Frozen and amended V3 protocol](../experiments/002-reputation-lock-in/v3-protocol-20261022.md), [Stage A](../experiments/002-reputation-lock-in/results/v3-stage-a-20261022/README.md), [Stage B](../experiments/002-reputation-lock-in/results/v3-stage-b-20261022/README.md).
- [Issue #19](https://github.com/davletovb/social-emergence-lab/issues/19) tracks scientific gate, [#21](https://github.com/davletovb/social-emergence-lab/issues/21) requires durable ~9.5 GB raw V3 journals, [#22](https://github.com/davletovb/social-emergence-lab/issues/22) requires independent review/data rights, and [#12](https://github.com/davletovb/social-emergence-lab/issues/12) tracks the original V0 archive expiring 2026-11-04.
- Committed cell summaries, `decision.json`, and `SHA256SUMS` make the derived result auditable but **do not preserve the original raw Hiyal journals**, and regeneration is not custody of original bytes. No further large simulation should precede preservation planning.

**Until an external human review and one variable-level empirical audit are complete: GO for interpretation/archival work; NO-GO for another broad simulation grid, policy generalization or novel-law language.**
