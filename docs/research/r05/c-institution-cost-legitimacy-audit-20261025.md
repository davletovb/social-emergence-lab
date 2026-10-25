# R-05 / Candidate C — monitoring-cost, legitimacy and membership-selection audit

**Date:** 2026-10-25 · **Tier-2 targeted original-methods and formal identification review** · **Status:** original **single cost-shift operator is NOT generally sufficient to identify legitimacy**; source-level and restricted mathematical findings only. **No experiment, code, data ingestion or model freeze**.

## 1. Narrow candidate and physical intervention

Original [institution-formation dossier](dossiers/institution-formation.md): groups with identical initial institution adoption can reflect (M) instrumental/material benefit, (L) preference for a legitimate/chosen institution, or (S) sorting of cooperative people into joining. Proposed operator `do(c←c+Δ)`: **externally imposed increase in total *institution-wide monitoring expenditure***, with **payer, number of payers, distribution/receipts, membership opportunity, enforcement detection probability, fine schedule, and whether the institution exists** held constant. In a two-member toy, each member who joins an implemented institution pays exactly `c/2`; nonmembers pay 0. Fees are paid only if the institution is actually implemented.

**Do not silently replace** this with a higher *per-punishment cost to peers*, lower enforcement accuracy, altered punishment severity, different public-good marginal return, fairness framing or a change in group size. These are **different treatments**. Likewise, an exogenously *assigned* fixed membership condition is a **new distinct operator** (a possible future amendment), not an actual component of the original cost-only candidate.

The claim under review is **synthetic structural identification** first, *not* that institutional legitimacy has not been studied, nor that the following toy was tested in any historical dataset.

## 2. Closest primary studies — actual treatment/measurement comparison

### Kosfeld, Okada & Riedl (2009), American Economic Review 99(4):1335–1355

[Publisher DOI](https://doi.org/10.1257/aer.99.4.1335) · [publisher-version PDF hosted by Maastricht University](https://cris.maastrichtuniversity.nl/ws/files/72449645/riedl_2009_institution_formation_in_public_goods.pdf). **Original published pp. 1339–1340 and 1346 inspected**, including original stages, payoffs and treatments:

- Subjects play a **three-stage institution formation game**: willingness to join a sanction institution, unanimous vote of prospective participants to implement, then public-good contributions. The sanctioning institution governs only members; outsiders may free-ride; institution expenses **shared equally among members**, payable when created.
- In the **two institution-formation treatments** the institution cost was **fixed at c=2**; the **public good's marginal per-capita return was varied**: IF40 (`a=0.4`, minimum efficient institution size 3) versus IF65 (`a=0.65`, minimum size 2). Two no-institution PG40/PG65 control arms. Subjects played **20 rounds with fixed partner groups**, 164 subjects across all treatments (44 in each institution arm, 40 and 36 in controls).
- Their work **does not randomize our proposed c increase** in the published institution arms, but establishes material incentives, endogenous group composition, fairness objections and costly membership as central and already studied. Specifically, grand organizations frequently formed despite classical partial-institution equilibria; the original paper interprets aversion to outsider free-riding as fairness/social-preference evidence.

**Important non-equivalence:** The paper's `a` treatment is a public-good payoff return change, not an exogenous price of institution monitoring. Reproduce its `c/s` member cost term only as a *structural reference*, never call our cost response an empirical replication of the IF40/IF65 comparison.

### Dickson, Gordon & Huber (2022), Science Advances 8(7):eabj7377

[Original full-text Methods](https://pmc.ncbi.nlm.nih.gov/articles/PMC8856613/) reviewed, especially protocol, accuracy, enforcement and authority compensation. This study already tries to **identify legitimacy-related intrinsic compliance versus extrinsic sanction incentives**, using a more sophisticated design than a one-dimensional institution fee:

- Randomly reassign subjects in groups of **five** each period: **four citizens and one authority**, over **20 rounds**. Authority chooses **small or big investment** in the accuracy of enforcement information. **Realized enforcement accuracy is then randomized conditional on the selected investment**, allowing comparisons with the *same realized accuracy* but different authority effort. The authority subsequently chooses among enforcement rules, usually **punish-according-to-signal (PATS)**; citizens observe the choices/accuracy and contribute or keep their endowment.
- Experiment 1 assigned **90 subjects**, Experiment 2 **110 subjects**; Experiment 2 also randomized the authority's compensation structure (benefit from public pot vs flat payment) to test alternative reciprocity explanations.
- The published design and article report a positive intrinsic legitimacy channel when authority effort differs but realized accuracy is matched, and material accuracy effects when accuracy differs with effort held fixed.
- **NOT our treatment:** accuracy-investment signaling is *not institution-wide cost increase paid by members*, and the study does not require endogenous institutional entry/exit and selection into membership of Kosfeld type. It is substantial, verified **prior art for causal legitimacy identification**, so broad “identify legitimacy for the first time” would be indefensible.

### Anderson & Putterman (2006), Games and Economic Behavior 54(1):1–24

[Publisher original abstract](https://doi.org/10.1016/j.geb.2004.08.007) explicitly says their experimental treatments **randomly varied the marginal price a player pays to punish another player** after contributions; random rematching removed strategic repeat encounters. They find demand for punishment responds to price and contribution shortfall. **Peer cost per sanction is not the fixed member share of institution-wide monitoring cost.** Related price-sensitivity predecessor, not an identical operator. Full text was not needed to establish this non-equivalence; do not claim it has settled institution-level cost effects.

### Source status / data rights

Kosfeld's AEA article identifies an official supplemental appendix and replication package; the Maastricht institutional-host copy is publicly readable but has **Taverne/end-user redistribution limitations**, **not a blanket license to mirror it**. Dickson's original open-access article and Yale resources identify replication archives, but **raw file variables, codebooks, deposit-specific terms and partner-session clustering have not been audited for this lab**. We accessed academic text and metadata only, not participant data.

## 3. Independent minimal mathematical reference — what a cost-only shift cannot identify

For two candidate members, one institution, and one simultaneous private joining/implementation decision at an already known composition, let `g_i` be the actor's **unobserved expected net material benefit of joining**, derived from beliefs about compliance, participation, enforcement reliability and future receipts. Let `v_i` be an actor-specific extra utility from perceiving the institution legitimate/chosen. Under logit choice with fixed `β=ln(3)` and **the same physical fee paid by each of two members**:

```text
Delta_i(c) = g_i + v_i - c / 2
P_i(join | c, g_i, v_i) = sigmoid( ln(3) * Delta_i(c) )
sigmoid(x) = 1 / (1 + exp(-x))
```

Consider two distinct hypothesized *mechanisms* with equal **total expected join advantage**:

- **M**: `g_i=1, v_i=0`: all preference attributable to expected material gain.
- **L**: `g_i=0, v_i=1`: all extra utility attributable to legitimacy preference.

Both imply the **same exact probabilities for every finite cost c**, not merely for two cost values, because both have `g_i+v_i=1`. Example:

| Externally fixed institution cost `c` | Per-member fee `c/2` | `P_M(join)` | `P_L(join)` |
| --- | --- | --- | --- |
| `c=0` | 0 | **3/4** | **3/4** |
| `c=2` | 1 | **1/2** | **1/2** |

If two actors have independent joining probabilities and the institution requires **unanimity**, the toy probability both approve is **9/16 at c=0** and **1/4 at c=2**, **under both generators**. A randomly varying *fixed-payer* fee cannot identify the decomposition `g` versus `v` if `g` is free and unmeasured. The cost coefficient is potentially identified; the **intercept is only the sum**, so `v` remains set-identified. This is an exact, **restricted nonidentification construction** and independent of numerical fitting/pilot outcomes.

**What would break this particular equivalence?** Independently fixing or tightly observing `g` (via correctly measured payoffs *and relevant expected behavior/beliefs*) plus a valid exclusion restriction on `v` could permit evidence of extra joining preference from the intercept. But **material payoffs written on a screen are not the actor's perceived net gain** when others' membership and cooperation are uncertain. Alternatively, introduce a *separate* genuine, ethical legitimacy cue `ℓ` that changes `v` without changing material beliefs/payoffs—**a new treatment** requiring independent falsification of its exclusion assumption, not guaranteed by calling it a “fairness cue.”

**Composition is a second confound:** if only altruistic/optimistic agents volunteer, `P(join|c)` also changes through the type distribution of entrants. A fee shift could affect which agents enter; conditioning only on observed members creates post-treatment selection bias. Before attributing an aggregate adoption curve to legitimacy or material price, measure **every eligible person's join decision**, baseline covariates and believed future contributions, *not just members who entered*; randomize or hold composition only in an explicitly new designed comparison.

## 4. Observable schema / causal interpretation

| Construct | Needed to claim difference | What published studies actually support | Critical caveat |
| --- | --- | --- | --- |
| Assigned institution-level total cost and **fixed payer share** | Record `do(c)`, fee due and number of eligible/actual paying members | Kosfeld fee fixed 2; Dickson varies authority effort/accuracy; Anderson varies peer punishment price | **Our main c-perturbation not observed in these comparators** |
| Material expected net gain `g` | Payoffs, belief forecasts, sanctioned behavior, expected future group composition | Kosfeld predicted participant counts incentivized; Dickson material accuracy contrasts | Unknown subjective net gains still permit `g+v` confounding |
| Legitimacy `v` | Separate validated indicator/variation and exclusion restrictions | Dickson causal random-accuracy/authority effort design speaks to intrinsic compliance | A fee shift alone is not legitimacy-specific |
| Endogenous membership and selection | All eligible invitations, join votes, final membership, actual sanction regime, dropout and group IDs | Kosfeld participation/implementation separate; Dickson rematched citizens/authority | Different randomization/repetition design; do not merge rows across original studies |
| Compliance after adoption | Contribution per person, fine assignment, visibility, realized payoffs | Both studies have observed game behavior | Selecting on adopted groups creates selection bias |

## 5. Scientific verdict and first-stage stopping decision

1. **Cost shift alone**: **FAIL as a general discriminating intervention** between material expected-benefit and legitimacy preference, due exact `g+v` equivalence. This is **not a general claim that legitimacy is unidentifiable in all experiments**—Dickson et al. established causal evidence with a distinct stochastic mediation design.
2. **Scientific negative contribution**: an explicit observation/assignment-conditional *identification-limit* theorem for institution choice could still be worthwhile **if it says more than the elementary decomposition above** and is shown not already implied by institutional-choice theory (Kosfeld and later studies). This two-parameter intercept aliasing **alone is too elementary to establish novelty**.
3. **Material vs legitimacy empirical correspondence**: **NOT READY** using the named existing studies for `do(c)`; no rights-verified cost-randomization dataset or independently costed human study. Dickson is substantial *different intervention prior art*; do not mislabel that data as our cost-arm observation.
4. **Necessary redesign to continue C**: specify a **second independently assigned, plausibly exclusion-valid manipulation** of authority legitimacy/quality *or* measured payoff beliefs held constant, plus baseline eligible cohort and (if relevant) pre-assigned membership. This would be a **new candidate variant** needing the same R-05 scrutiny as A2/B/D; do **not** retroactively include it in original C's score.
5. **Decision:** **C's current original single-cost operator cannot advance as a uniquely identifying mechanism experiment**. Retain in Tier-2 as a documented negative-result/novelty investigation *only*; **conditionally replace its Tier-2 work slot with resource-scarcity D** if owner seeks an immediately more promising discriminating design. Do not automatically declare D empirically ready or select an Experiment 003 from this inference.

## 6. Source/access and open checks

- [x] Verify original published **Kosfeld three-stage payoffs**, institution cost fixed c=2 and treatment variation MPCR `0.4/0.65`; verify 20 rounds and 164 participants; no inference original c is randomized.
- [x] Verify **Dickson realized accuracy randomization**, authority investment, PATS and second-study compensation randomization from original article Methods.
- [x] Verify Anderson–Putterman randomized *peer punishment price*, not the proposed institution membership cost, from publisher abstract.
- [x] Provide an **independent-of-Hiyal exact cost-only non-identification fixture** with two distinct `g,v` mechanisms and explicit identical observables.
- [ ] Source-independent expert criticism of `g/v` restrictions, equilibrium selection and whether this small negative theorem is already routine.
- [ ] Raw archived data/permissions and codebook inventory if the empirical route is revived. No raw participant records handled.
- [ ] If considering a new legitimacy/selection operator, **version the candidate and audit it as a new treatment** rather than adding it after seeing results.
- [ ] Decide whether to pause C and evaluate D as an alternative for Tier-2 **after** documented source-aware comparison; not an automatic winner.

**No source-supported sociological discovery, pilot data, CI cost or R-06 freeze produced by this review.**
