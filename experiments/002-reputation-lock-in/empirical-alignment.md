# Experiment 002 — empirical alignment and data-access audit

**Checked:** 2026-10-21. **Scope:** feasibility, not data acquisition, participant recruitment or institutional approval.

A calibrated model should reproduce at least one **observed choice or rewiring statistic** rather than assuming that agent rules are a faithful description of human reputation systems. None of the listed datasets has been downloaded, accessed under an account, or validated against our simulation as part of this review.

| Resource | Verified availability | What can be checked | Crucial limitation / access |
| --- | --- | --- | --- |
| [Princeton MusicLab](https://www.princeton.edu/~mjs3/data.shtml); [Princeton archive](https://oprdata.princeton.edu/Archive/CM/) | Public dataset description for **4 experiments, 27,267 participants, 167 CSVs**; archive currently displays **registration required** for downloads | False popularity, choice responses, behavioral response curves | No directed rewiring or exogenously repaired opportunity network. Do not claim to calibrate graph dynamics from MusicLab alone |
| [Tsvetkova 2021](https://doi.org/10.1098/rstb.2020.0299), [data/code Figshare](https://doi.org/10.6084/m9.figshare.15097107) | Article identifies public model scripts and processed data from **nine** network social-dilemma experiments | Choice/reputation/inequality relation, network dynamics when time-resolved edges actually provided | Need inspect the downloaded code/data licenses and exact edge timestamps before claiming reanalysis |
| [Simpson, Montgomery & Melamed 2023](https://doi.org/10.1038/s41467-023-43486-7) | Published experiment with **1,073** participants in cooperative networks | Disclosure format, partner selection, segregation | Raw time-resolved participant network data availability and permissions require verification; no experimental truth-correction × edge-repair factorial |
| [Barnes et al. 2025](https://arxiv.org/abs/2506.04435) | Describes a published author research dataset and open collaboration-network resource (reviewed here as **preprint**) | Graph edge intervention and shifts in centrality; test degree-only vs centrality | Sensitive, inferred demographic labels and limited causal identification; don't repurpose individual-level attributes without rights and privacy assessment |
| [Papamichalis et al. 2026](https://doi.org/10.1038/s41586-026-11154-7), [analysis code](https://doi.org/10.5281/zenodo.22004783) | 110-village randomized trial with **8,331** participants; code openly deposited | Information exposure × graph formation/persistence; network rewiring rates, degree and link turnover | **Human microdata and network edge lists are controlled-access**: request with data-use agreement and ethical approval; **no unrestricted raw-data access**. Study manipulated health education, not false reputation correction |
| [Holtz, Dow & Aral 2026](https://doi.org/10.1609/icwsm.v20i1.42680) | Publisher article describes randomized marketplace ratings rollout | Negative prior and external plausibility of reputation-treatment magnitude | No evidence yet of participant-level raw data availability; added ratings, not repair of historical misratings |

## Empirical operationalization candidates

A *minimal* credible mapping needs:
1. Opportunity network: actual eligibility or consideration sets, not merely observed successful ties.
2. Reputation: public/private quality perceptions measured **before** selection, not inferred solely from a person's later opportunities.
3. Postcorrection timing: known exposure to changed information and subsequent partner/selection events.
4. Intervention identity: which link-creation choices were random/exogenous, rather than simply predicted correlations.
5. Outcomes: new opportunities, not accumulated status or wealth from before the correction.

**None** of the reviewed datasets is currently confirmed to contain *all five*. Human-subject experimental testing would require ethical review, participant protections and potentially consent/debriefing for any misleading signal manipulation. It is not authorized by this document.

## Best practical sequence

- Begin V0 with **synthetic-only calibration** against mathematical degree/selection identities and known nulls, not an artificial claim of empirical validation.
- Verify the use/reuse rights and accessible variables in the Tsvetkova Figshare materials and MusicLab registration archive; write an explicit column/variable audit, with licenses.
- Evaluate whether Simpson's behavioral-network materials include adjacency over rounds and precise disclosure variants before requesting access.
- Investigate Papamichalis 2026's public **analysis code only**; accessing participant network edges requires formal application/permissions. Use aggregate reported persistence as *context*, not as if it directly fitted false-reputation parameters.
- If neither dataset supports correction × graph repair, frame V0 as **mechanism exploration** rather than externally validated sociology. Prepare an ethically reviewed experimental design if seeking human causal claims.

**Go/no-go:** scientific publication claims about real society stay blocked until at least one independent construct-valid comparison is established. A successful Hiyal simulation cannot satisfy this requirement.
