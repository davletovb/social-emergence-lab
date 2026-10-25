# R-05 / Candidate B — empirical source and assignment feasibility audit
**Date:** 2026-10-25 · **Stage:** Tier-2 targeted **publication + archive-metadata audit**; **no raw participant data fetched** · **Decision:** existing experimental **D effect has an auditable source**, proposed **D×R factorial is NOT present/validated**

## Exact question and intervention contract

**Proposed novel estimand:** in a specified population and public-norm setting, can observed behavior after randomized **truthful disclosure of the empirical distribution of privately elicited attitudes (D)** and separately randomized **actual removal of sanctions (R)** distinguish (i) misperceived peer approval, (ii) social-coordination payoffs and (iii) sanction-avoidance, allowing belief/sanction expectation spillovers and measurement error? R must remove a genuine *sanctioning capability or liability*, not merely make the respondent anonymous or supply **information** about punishment. Private attitude *elicitation* is a baseline measurement operation shared by all four arms; it is **not** the D intervention. Neither physical two-factor randomization nor latent psychological isolation can be inferred from a one-factor disclosure study.

Original candidate: [hidden consensus dossier](dossiers/hidden-consensus.md), [two-reader reconciliation](reconciliation-a1-b-e-20261025.md). The existing 2020 experiments are *comparator evidence*, not authorized human study designs for this lab.

## Primary study: what was actually performed (source checked)

**Bursztyn, González & Yanagizawa-Drott (2020), *Misperceived Social Norms: Women Working Outside the Home in Saudi Arabia*, American Economic Review 110(10), 2997–3029**, [journal](https://doi.org/10.1257/aer.20180975), [original full text](https://home.uchicago.edu/bursztyn/Misperceived_Norms.pdf), especially main-experiment Methods pp. 3003–3007 and results pp. 3010–3013.

| Design dimension | Source-established main experiment | What our proposed D×R requires |
| --- | --- | --- |
| Setting and units | **500** married Saudi men aged 18–35 in **17 planned 30-person sessions** (510 expected, 500 completed), in Riyadh; recruited from areas with shared social contacts | A specified ethically recruited population, exposure/interference policy and genuine social consequence |
| Elicited pre-treatment | Respondent's own private opinion on women's employment, other participants' expected agreement, confidence and social contacts; guesses incentivized | Separate measured private attitude, second-order/normative expectations, perceived sanction risk, measurement error |
| Treatment operator | One-half **individually randomized** to **truthful feedback charts** showing their session's actual shares agreeing/disagreeing with two statements; control receives **no feedback**. Assignment pre-mapped to last three digits of phone numbers; respondents' full identifying details kept outside survey | Randomized D and **independently** randomized *physical removal of real sanctions R*; a validated first stage for each |
| Exposure timing | Opinions/belief-guess survey first, feedback or no-feedback in second part, incentivized decision after feedback | Preplanned factorial with common timing/observables in **four** cells; actual R assignment and spillovers |
| Immediate observed outcome | Choice between gift card and signing spouse up for a women's job-matching service; treatment **32.0%** vs control **23.5%**, p≈0.017 | Public norm compliance/contribution appropriate to specified sanction capability, plus explanatory belief/sanction measures |
| Longer outcomes | Follow-up about **3–5 months** later on job applications/interviews and persistent beliefs; session-neighborhood spillover and imperfect follow-up linkage are discussed | A prospective follow-up and interference structure, predeclared independent units, attrition and uncertainty |
| Sanction operation | **NO randomized sanction removal in the reported main 2-arm experiment**. It studies perceptions of injunctive acceptability and perceived social consequences. Survey-response anonymity and existing social sanctions are not an R treatment. | Independent R assignment; cannot estimate D×R interaction from this dataset |
| Secondary recruitment experiment | Separate employer recruitment randomized truthful-information messaging before a choice about an outside-the-home vs at-home enumerator role; **not** a sanction-removal experiment and **not** linked factorial cells with the male main experiment | Does not supply the missing R arm; do not pool its women or assignments with the men's main sessions |

The original authors explicitly **chose not to randomize observability of the job-service sign-up decision** (Methods p. 3006), as spouses' employment would be socially observable. Their survey anonymization and study ethical review **do not** imply we may recruit people or manipulate their real social sanctions without separate oversight.

The observed treatment effect is a causal comparison **for D-like factual-feedback assignment in that study**. Its mechanism interpretation is grounded in the authors' design and data, **not** direct independent identification of personal norm preference versus sanction response in our proposed 2×2.

## Archive inventory / measurement status — non-sensitive metadata only

**Repository locator:** [openICPSR, project 119404, version V1](https://doi.org/10.3886/E119404V1), dated 2020-09-23; [file-level catalog](https://www.openicpsr.org/openicpsr/project/119404/version/V1/view?path=%2Fopenicpsr%2F119404%2Ffcr%3Aversions%2FV1%2Freplication_package%2Fdata%2Fclean_data%2F01_main_exp_clean.dta&type=file) shows:
- `replication_package/data/clean_data/01_main_exp_clean.dta` — Stata file **71.5 KB, 500 cases, 36 variables** (these are *catalog field counts*, not inspected names or analysis);
- [main analysis Stata script locator](https://www.openicpsr.org/openicpsr/project/119404/version/V1/view?path=%2Fopenicpsr%2F119404%2Ffcr%3Aversions%2FV1%2Freplication_package%2Fcode%2F02_analysis%2F01_analysis_main_exp_tables.do&type=file) — `01_analysis_main_exp_tables.do`, 26.5 KB; scripts/other outputs indexed in project;
- Research publication additionally links supplement, surveys/experiment scripts, and AEA RCT registrations **0002447, 0002633 and 0005321** (original AER paper, title-page note).

**Accessible at audit:** indexed file names, type/size, case and variable counts, original paper's described treatments, outcomes, and sampling. **NOT accessed/confirmed:** actual 36 column names, a codebook-to-column crosswalk, survey scripts in package, file-specific licence and login/consent conditions, exact replicability of the published 23.5%→32.0% from raw records, cross-session covariance availability or identifiers, and whether a usable public-use subset can be lawfully passed into any external cloud/AI workflow.

**Data-use caution:** ICPSR's [current terms](https://www.icpsr.umich.edu/sites/icpsr/about/policies/terms-of-use) restrict identifying/redistributing research subjects and data. Its [redistribution policy](https://www.icpsr.umich.edu/sites/icpsr/about/policies/redistribution) warns that uploading governed ICPSR datasets to outside LLM services can amount to prohibited redistribution, **but says the policy does not apply to self-published deposits**. Historical [openICPSR FAQs](https://www.openicpsr.org/openicpsr/faqs) explain that self-published collections have individual deposit-specific licences and require compliance with them. In August 2026 openICPSR was [migrated](https://www.icpsr.umich.edu/sites/icpsr/posts/shared/openicpsr-in-transition), so legacy URL access varies. **Do not assert universal prohibition or universal CC0/CC BY rights for this exact deposit without verifying the actual collection and file terms.** Do not download, commit, mirror, share individual records with an LLM, or infer identities in this audit. Seek archive rights confirmation/approved local protected analysis first if necessary.

## Closest rival experiment — source-checked

**Smerdon, Offerman & Gneezy (2020), *'Everybody's doing it': on the persistence of bad social norms*, *Experimental Economics* 23(2), 392–420**, [DOI](https://doi.org/10.1007/s10683-019-09616-z), [author original paper](https://rady.ucsd.edu/_files/faculty-research/uri-gneezy/Everybody%20doing%20it_Experimental%20Economics.pdf). Methods and introduction specify a 50-round coordination game in which the welfare-maximizing door switches after round 25, with **fully informed versus incompletely informed preference states** and, under imperfect information, **communication** and social-value/group-size variations. The experiment supports the significance of uncertainty about others' preferences rather than purely self-sustaining coordination. It **already contrasts private-preference information against coordination-based persistence**. It does *not* on the reviewed main design establish a factorial independent **actual sanction-removal** treatment. The marginal novelty of an information-only B experiment is therefore low; only a *different, sharply defined physical R operator* with plausible measurement might justify further work.

## Observable / assignment-compatibility matrix

| Proposed variable/intervention | Bursztyn main experiment | Smerdon comparator | B 2×2 readiness |
| --- | --- | --- | --- |
| Private baseline opinion | Paper documents elicitation; **raw column unchecked** | Experimental payoffs/preference states defined; participant-level record unchecked | **POSSIBLE, not file-audited** |
| Belief about other opinions | Incentivized guesses described; raw column unchecked | Uncertainty/communication varied; raw subjective beliefs unchecked | **POSSIBLE, measurement not confirmed** |
| Random factual D treatment | **YES** (individual two-arm assignment) | Full vs imperfect information / communication conditions, not identical D | **D comparator established** |
| Random physical sanction removal R | **NO** in described main treatments | **NO** such physical factorial verified | **BLOCKED for archival factorial analysis** |
| D×R joint assignment | **NO** | **NO** in papers' designs checked | **NOT IDENTIFIED** |
| Public action/outcome | Gift-card versus wife's job service sign-up + longer reported outcomes | Repeated door choices/earnings | Different constructs and units; **do not pool** |
| Unit and assignment | Individual D, 17 sessions; session spillover possible | Group game/treatments | A future factorial must separately choose clusters and assign one history per subject |
| Rights of individual data | Collection has cataloged public metadata; **file-specific rights unknown** | Methods published, dataset rights unverified | **No records ingested** |

## Tier-2 decision / exact stopping points

**B / existing empirical D evidence:** **PASS for verified published design existence and metadata locator only**. A legitimate, bounded replication inquiry could attempt to reproduce the **D-only** published treatment on approved local data *after* rights/variable access, but a replication is not automatically novel or the proposed new experiment.

**B / existing D×R factorial:** **NO-GO** for reanalysis based on the checked records: R is **not an assigned treatment** in the source-described study, so the D×R estimand is absent, regardless of whether one of the 36 columns is a proxy for sanctions.

**B / prospective D×R:** **DESIGN GATE / NOT APPROVED**. Before even a tiny human trial: costed ethical partner/IRB review; independently manipulable noncoercive R, truthful D, credible manipulation checks and consent; protect respondent privacy; calculate minimum independent units, interference/attrition, and report a falsifying null. If R cannot be independently manipulated without changing beliefs, then mechanisms may remain set-identified and a constructive negative theoretical result could be the better contribution.

**Recommendation:** Keep B on the Tier-2 shortlist **only as an empirical feasibility and source/ethics check**, not as an empirically ready Experiment 003. Do **not** spend CI budget or treat 500-person D data as a laboratory-ready D×R sample. Cross-validate exact 36 variable names and archived terms only via a permitted non-personal, rights-aware local review if needed.

**Source claims:** Published design and outcomes are facts from the cited originals; proposed B factorial and infeasibility verdict are this project's inference from what treatment arms were actually assigned.
