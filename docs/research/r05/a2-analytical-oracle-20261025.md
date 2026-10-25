# R-05 / Candidate A2 — independent analytical small-world oracle and mechanism limits
**Date:** 2026-10-25 · **Status:** **mathematical reference specification / exact toy cases**, not an implemented six-generator world, model recovery run, Hiyal formal-model freeze, R-06 approval or human result

## Scope and independence claim

This document develops a **standalone hand-derivable reference** that does *not rely on Hiyal implementation or replay*. It independently specifies toy probabilities and negative controls against which a future Rust model might be checked. It is **not independent human review or a second computational implementation**, and it does **not** establish that six distinct generators are recoverable. It tests two small partial boundaries: (i) personal norm versus conditional response and (ii) fixed-type assortment versus population-level mean cooperation.

**Source motivation:** [candidate #36](https://github.com/davletovb/social-emergence-lab/issues/36); [Galbiati et al. (2018)](https://doi.org/10.1073/pnas.1813502115), randomized fine **per dyadic match**, random partner matching (not selective matching); [Yang & Yue (2019)](https://doi.org/10.1038/s41598-019-50083-6), real lab random-vs-weighted-history partner assignment; [Wilson & Collins (2019)](https://doi.org/10.7554/eLife.49547), recovery/model-comparison methodology. **Our toy is not the actual payoff design of either cited laboratory experiment.**

## Oracle 1 — exact two-player contribution game

### Fixed physical world and observable assignments

One focal actor interacts with one publicly identified **computer-controlled scripted partner**. Actions are binary `a∈{0,1}` (0 defect, 1 contribute), in one decision period after an exogenous public *history* signal `m∈{0,1}`. The truthful signal reports **the partner's real action in a previously staged observation period**, not a deceptive message or a forced human choice. Fine `f∈{0,1}` applies only to the focal actor's current defection. If two focal actors are used to generate a joint observable event, each is placed in **a separate independent copy** of this two-actor world; they are not secretly matched with one another.

Let the period payoff be `u(a_i,a_j;f) = a_i + a_j - 2a_i - f(1-a_i)`. Thus choosing cooperate rather than defect changes *material utility* by exactly `Δu_mat = f-1`; the scripted partner's current action changes payoff levels but **not** the difference (linear public-goods assumption). This special payoff is chosen for an analytically checkable oracle, not claimed as universal.

Three toy behavioral families share **the identical assigned world and fine/matching operator**. Action choice uses a logit response with `β=ln(3)`, `σ(z)=1/(1+exp(-z))` and independence across separately generated focal actors:

| Family | Decision utility difference `Δu` | Model-specific ingredient |
| --- | --- | --- |
| **M** pure material incentive | `f-1` | only physical fine |
| **N** intrinsic personal contribution preference | `f-1+ν`, `ν=1` | fixed private preference; does not depend on `m` |
| **C** conditional response to observed partner contribution | `f-1+κm`, `κ=1` | uses truthful public prior-partner action `m` |

The policy is `P(a_i=1 | f,m,family)=σ(β Δu)`. With fixed parameters, **all probabilities are exact rational numbers**:

| Fine `f` | Factual history `m` | Material M | Norm N | Conditional C |
| --- | --- | --- | --- | --- |
| 1 | 1 | 1/2 | **3/4** | **3/4** |
| 0 | 1 | 1/4 | **1/2** | **1/2** |
| 1 | 0 | 1/2 | **3/4** | **1/2** |
| 0 | 0 | 1/4 | **1/2** | **1/4** |

**Exact non-identification:** In a design that only ever exposes participants to `m=1`, families N and C have **identical action laws under every available `f∈{0,1}`** in this restricted toy (`ν=κ`). No number of additional independently generated observations *at those same histories* can identify whether the constant +1 term arose from a personal norm or conditional response. This is a concrete parameter/historical-support overlap, not an optimizer failure.

**Separating physical intervention:** change *the scripted partner's real, publicly observed previous action* from 1 to 0, holding current fine `f=1`, endowment, choice set, information delivery, focal training state and other payoff primitives fixed. This is the **computer-player defection pulse** in the synthetic oracle; it is not presented as an intervention previously administered to humans. Norm N stays `3/4` whereas conditional C drops to `1/2`. These are **specified toy predictions**, not real human responses.

A diagnostic with **two independently generated focal participants** assigned the same `f=1,m=0` world predicts `P(both focal decisions cooperate)=9/16` for N versus `1/4` for C; at `m=1` both equal `9/16`. These are analytical products of *independent draws*, not matched counterfactual responses for the same human participants. Comparing D=0 and D=1 groups in a future human design would require random assignment and separate independent cohorts.

**Crucial limitations:** If behavioral parameters are unobserved or free to vary with treatment, or if a conditional observer's `m` does not refer to the scripted partner, this difference may vanish; mixture/learning alternatives can potentially imitate both models. Sanction withdrawal **alone** at constant `m=1` does **not** separate N and C, as shown. These cases neither prove the full six-family identification claim nor permit a claim about source experiment behavioral constructs.

### Independent exact fixture requirements

A future implementation in any language must reproduce the four-row probability table **exactly to numeric tolerance**, the overlap under `m=1`, the `3/4 vs 1/2` separating action probability at `f=1,m=0`, and the `9/16 vs 1/4` two-copy joint event. Fixture data are separately stored in [`a2-oracle-fixture-20261025.csv`](a2-oracle-fixture-20261025.csv). Code is **not** being added or CI executed here; exact fractions are the normative reference, not a simulation estimate.

## Oracle 2 — four actors reveal sorting without an aggregate mean change

Consider **four** fixed-type agents: two H with independent propensity `p_H=3/4` to contribute and two L with `p_L=1/4`. Propensities are *held fixed*: no behavioral response to the group assignment or a different sanction price, which is an explicit restricted null/assortment corner. Pair them into two groups of two, under the **same physical payoff/sanction rules for every actor**. The matching operator is either **exogenously assigned assortative** `(H,H),(L,L)` or **exogenously assigned mixed** `(H,L),(H,L)` (three equally likely perfect matchings yield one assortative and two mixed). The fixed H/L labels are available to the *reference oracle*, not automatically to a human experimenter.

Exactly:
- Population mean cooperation in either matching: `(2×3/4+2×1/4)/4 = 1/2`. Thus “reshuffling reduces population-wide cooperation” is **false as a general signature** even when group composition changes dramatically.
- Mean probability that **both agents of a group cooperate**, across its two groups: assortative `[(3/4)^2+(1/4)^2]/2=5/16`; mixed `(3/4)(1/4)=3/16`; assortative minus mixed = `1/8`.
- Under **uniform random matching over the three perfect matchings**, mean group-level both-cooperate probability is `(1/3)(5/16)+(2/3)(3/16)=11/48`. Assortative minus random = `1/12`. This is a change in **group joint composition/correlation**, not any within-agent change.
- If H/L types are unobserved and partners have changing propensities, group-level covariance could equally come from conditional learning or latent common shocks. Group-level match data alone are therefore **not proof** of an endogenous assortative choice mechanism.

**Important structural limit:** the exogenously assigned H/H–L/L fixture checks a *matching and observational statistic*. It does **not** implement agents' endogenous choice to join/select partners, which is the real roster's sorting mechanism. That choice function, its ability to deviate and common access/operator definition remain future R-06 work; do **not** call the four-agent oracle a full sorting generator.

## Six-family roster mapping and missing formal rules

The current candidate calls for **six** generators:
1. M material enforcement (the simple pure corner above);
2. N personal normative preference (a **personal preference**, not a fully specified Bicchieri social norm);
3. C conditional cooperation (the `m`-responsive toy above);
4. S fixed types with assortative **endogenous** membership (only the exogenous matching *oracle* above exists now);
5. L joint enforcement→social-expectation learning (requires belief-update state, parameter domain, and physical fine-feedback first stage);
6. R genuinely out-of-family aspiration-based reinforcement learning (requires aspiration state and action-value updating and payoff/noise calibration).

**Candidate formal transitions to decide next, NOT frozen:** common actor state `(a_t,public_history,assigned_group,f_t)`; M/N/C use the restricted logit map above; S adds an endogenous partner-choice policy under a group capacity/transfer rule; L can use an explicit predeclared belief state `b_{t+1} = (1-λ)b_t + λ z(a_partner,t,f_t)` with `z` justified and bounded rather than set after seeing outcomes; R can use two action values `Q_t(0),Q_t(1)`, an aspiration `θ_t` and a predeclared satisfaction-update rule **without an explicit social-preference parameter**. L and R must not be merely renamed nested logits tuned to guarantee recovery. Their exact transition laws, observation schema, decision-noise distributions and overlap with N/C **remain unimplemented**.

## Recovery policy, analytical tests and stopping gates

**Primary candidate endpoint remains the Issue #36 proposal**, not a numerical result here: worst-family identifiable recovery from **one realized assigned history per independent experimental unit**, with cross-family ties marked **undetermined** and counted as failures for exact-label recovery. Synthetic matched-fork comparisons, a full-latent-state upper bound and Galbiati historical fine-per-match reanalysis are *separately labeled regimes* with different observation/assignment structures and full budget costs.

Prescribe future negative controls: (i) N=C when `m=1`, (ii) N/C remain indistinguishable under fine-only intervention at `m=1`, (iii) fixed-type population mean invariant to sorting, (iv) zero behavioral noise limiting cases, (v) all actor payoffs and action distributions normalized, (vi) an independent implementation verifies every exact fraction. **No statistical uncertainty interval is claimed for these mathematical probabilities.** Sampling noise requires separate simulated/empirical budgets.

**First rigorous gate that can be answered without costly simulation:** an independently checked four-state fixture for M/N/C and four-actor sorting fixture. **Second, still open:** one non-tautological six-family discriminating/intervention-or-set-identification theorem under *actual limited observables*, with independent transition implementation, model misspecification challenge, and adversarial review. **STOP/REDESIGN** if the nominated six models can be made to produce the exact same policy on all physically feasible interventions while one claims “identified winner” by complexity alone.

**Outcome of this note:** exact small-world mathematical checks **specified**, not full independent scientific verification or implementation. No Hiyal runtime, raw E002 journal, seed calibration, human data or CI jobs touched.
