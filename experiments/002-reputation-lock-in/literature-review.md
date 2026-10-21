# Experiment 002 — focused literature review and novelty audit

**Date:** 2026-10-21  
**Status:** targeted discovery review; not a systematic review, not a verified claim of an unsolved problem.  
**Candidate question:** When false reputations have changed *who can encounter whom*, does a later truthful reputation correction fail to restore equal access to opportunities? Does independent structural network repair recover it?

## Executive assessment

**Rejected as novel:** cumulative advantage from stochastic success; public reputations increasing inequality; false popularity becoming self-fulfilling; networks shaping inequality; and social influence sometimes self-correcting. The first draft of this experiment overlapped with substantial past work.

**Narrowed candidate (unproven novelty):** within a **coupled misinformation → network-rewiring → access** process, isolate a *post-correction opportunity gap* with identical underlying ability, and compare **corrected information alone** against **the same correction plus independent opportunity-network repair**, forked from the *same* preintervention history. This may be a useful mechanism-specific computational test, but the cited literature already explores closely related pieces. Publication-level novelty has **not** been established.

### Selected directly relevant studies (not exhaustive)

| Work | Evidence and established result | Overlap / implication | Still missing from our review |
| --- | --- | --- | --- |
| DiPrete & Eirich (2006), *Cumulative Advantage as a Mechanism for Inequality* (Annual Review of Sociology), https://doi.org/10.1146/annurev.soc.32.061604.123127 | Mechanism/review of accumulative advantages | Experiment 001 is a familiar reinforcement benchmark | No independent theoretical originality from a simple weighted urn |
| Salganik, Dodds & Watts (2006), *Experimental Study of Inequality and Unpredictability in an Artificial Cultural Market* (Science), https://doi.org/10.1126/science.1121066 | Social influence increased inequality and unpredictability in multiple music-market worlds | Reputation/social-proof feedback is not new | Our network-access counterfactual differs in unit and mechanism, not established as novel |
| Salganik & Watts (2008), *Leading the Herd Astray* (Social Psychology Quarterly), https://doi.org/10.1177/019027250807100404 | Deliberately inverted popularity of songs; many false successes became self-reinforcing, although quality recovered in part | **Direct prior art** for false reputation becoming real; we must not claim this discovery | Did not implement our proposed *later truthful correction crossed with independent network repair*, to the extent examined here |
| van de Rijt et al. (2014), *Field Experiments of Success-Breeds-Success Dynamics* (PNAS), https://doi.org/10.1073/pnas.1316836111 | Random early money/status/ratings/endorsements caused later success; returns were limited | Early arbitrary advantages are empirically established | Does not justify blanket irreversibility or unlimited amplification |
| van de Rijt (2019), *Self-Correcting Dynamics in Social Influence Processes* (American Journal of Sociology), https://doi.org/10.1086/702899 | Experiments and reanalysis show initially advantaged inferior choices can lose dominance over time | A **serious competing hypothesis**: reputation/quality dynamics can recover | We must allow restoration without repair and should vary signal reliability |
| *The Emergence of Inequality in Social Groups: Network Structure and Institutions Affect the Distribution of Earnings in Cooperation Games* (2018), https://pubmed.ncbi.nlm.nih.gov/30028884/ | 33 experiments/96 conditions report systematic effects of clustering, link fluidity and punishment on earnings inequality | Structural networks already implicated in inequality | Not a direct correction-after-endogenous-rewiring test |
| *The Effects of Reputation on Inequality in Network Cooperation Games* (2021), https://doi.org/10.1098/rstb.2020.0299 | Heuristic model plus data from nine social-dilemma experiments; network process conditions whether reputation reduces dispersion; may improve behavior-payoff alignment | Reputation does **not always** increase inequality, contradicting simple monotone hypotheses | Our conjecture needs outcomes about *access after correction*, not only aggregate Gini |
| Chu et al. (2020), *Stuck in Place? A Field Experiment on the Effects of Reputational Information on Student Evaluations* (Social Forces), https://doi.org/10.1093/sf/soz097 | Randomly assigned reputation and essay quality to teacher ratings; prior negative information can crystallize after one behavioral confirmation | Direct evidence about false/true judgments and reputational persistence | Does not independently manipulate opportunity-network repair |
| Simpson, Montgomery & Melamed (2023), *Reputations for Treatment of Outgroup Members Can Prevent the Emergence of Political Segregation in Cooperative Networks* (Nature Communications), https://doi.org/10.1038/s41467-023-43486-7 | Lab-network experiment with 1,073 people; disaggregated reputational information mitigated segregation in a politically divided setting | Very close intersection of information disclosure and evolving graph | Need direct comparison to differentiated information / tie restoration; our mechanism may overlap substantially |
| Kas (2022), *The Effect of Online Reputation Systems on Intergroup Inequality*, https://doi.org/10.1016/j.socec.2021.101800 | Experimental reputation systems failed to eliminate intergroup inequality; early disparities mattered later | “Better reputation systems fix inequality” is not guaranteed | Intergroup inequity and our randomly assigned false priors are not equivalent |

The scope and “not studied here” descriptions above reflect **what we verified in these sources**, not a claim that no other publication performed the same test. The 2021 network-cooperation paper is cited as Tsvetkova (2021) in the design review; confirm its authorship when it is next reviewed.

### Leads from the earlier concept note, not yet reviewed

The first version of this experiment's research question listed two further sources. They have **not** been reviewed here, and each must be reviewed or excluded with a recorded reason before the novelty audit is complete:

- *Agent-based computational models of reputation and status* (Oxford Handbook chapter): https://doi.org/10.1093/oxfordhb/9780190494087.013.13
- *Co-evolution of social classes, mobility, wealth, and learning*: https://www.sciencedirect.com/science/article/pii/S016726812300210X

### What the literature forces us to change

1. **Do not define the main discovery as false information causing social inequality.** Salganik & Watts (2008) already directly demonstrated manipulation of apparent success.
2. **Do not assume reinforcement implies permanent lock-in.** van de Rijt (2019) presents self-correcting dynamics and should be used to construct tests expected to fail for some parameters.
3. **Do not measure only terminal resource Gini.** Correcting beliefs cannot erase already allocated resources without redistribution; that confounds irreversibility. Instead measure the *flow of postintervention opportunities* to actors randomly labeled “high reputation” versus “low reputation.”
4. **Do not equate reputational correction with network repair.** Split the two mechanisms in a crossed design while holding preintervention histories identical.
5. **Do not assume network topology effects are novel.** Clustered/fixed/evolving networks have been studied extensively; the aim is isolating how correction effectiveness interacts with **endogenously formed access links**.
6. **Do not conflate ability, past realized opportunity, and reputation.** Primary model uses equal intrinsic performance probability for every agent, explicit noisy observations, separate belief state and graph state.
7. **Do not rely on a simulator alone to claim real-world external validity.** Mechanism evidence inside an artificial society requires independent comparison to human behavior or empirical network data.

## Honest novelty assessment

| Claim | Verdict |
| --- | --- |
| Random early success can beget more success | Established; not novel |
| False public popularity may become real | Established; not novel |
| Reputations and dynamic networks affect inequality | Established; not novel |
| Reputation distortions always persist | **Contradicted** by self-correction research |
| Correcting everyone's beliefs but retaining previous *reputation-selected* ties can leave a future access disparity, compared to correcting beliefs and repairing ties | **Candidate mechanism-specific contrast; scientific novelty unverified** |
| There exists a sharp timing/connectivity phase boundary for this contrast, robust to choice algorithms and signal quality | Hypothesis to investigate; **no evidence yet** |
| The simulated disparity operates in actual labor markets, education or online platforms | Unsupported until empirical design/validation |

## Scope of this review and next scholarly work

This was a targeted search of representative primary research from 2006–2023, not a PRISMA review, a citation graph, or a systematic forward-citation search through October 2026. In particular, we have **not** ruled out post-2023 studies on correction of misinformation combined with network access. Before positioning a paper as novel: run backward/forward citation tracing from Salganik & Watts (2008), van de Rijt (2019), Chu et al. (2020), Tsvetkova (2021) and Simpson et al. (2023); search adjacent algorithmic fairness, network formation, recommender systems, labor-market signaling, and belief updating literatures; record exclusion criteria and update this matrix.

## Potential empirical alignment

- **MusicLab (Salganik)** openly releases anonymized song choices for four experiments (27,267 participants in total): https://www.princeton.edu/~mjs3/data.shtml . Useful for social influence / incorrect-popularity mechanisms and measurement checks; **does not directly validate the proposed network repair counterfactual**, as it lacks friendship-tie manipulation.
- **Network experiments** analyzed in the 2018 inequality paper and the 2021 reputation paper may provide graph/earnings diagnostics where raw and ethically reusable data are available. Verify actual licensing, identities and access before claiming reproduction.
- A prospective **incentivized experiment with consenting human participants**, where partner recommendations and correction/rewiring are randomized, is a more credible ultimate test of the specific claim. If deception about popularity or reputations is proposed, secure applicable ethical approval and debriefing first. No human-subject recruitment or manipulation is authorized by this computational plan.

**Research gate:** literature differentiation + clear new mechanism + falsification checks + plausible empirical alignment must precede confirmatory simulation or any claim of scientific novelty.
