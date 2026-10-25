# R-05 A2 — physical-information repair and analytic identifiability checks (v0.1)
**Date:** 2026-10-25. **Status:** proposed design repair with exact mathematical checks, NOT approved R-06 protocol, independently reviewed implementation, full six-family identification, simulation or human study.

**Context:** [A2 six-generator v0](a2-six-generator-formal-spec-20261025.md), [exact earlier M/N/C fixture](a2-analytical-oracle-20261025.md), [Issue #36](https://github.com/davletovb/social-emergence-lab/issues/36). This file *supersedes* v0's contradictory concealed-history idea for the prospective-design reference; the original v0 is preserved. Changes remain candidate design decisions, not retrospectively applied to real Galbiati participant data.

## 1. Formal diagnosis of information leakage

The common material reward is U_i = a_j - a_i - f(1-a_i), with a_i,a_j∈{0,1}, assigned f∈{0,1}. If an actor knows its own current action, fine, and *exact* payoff, it can reconstruct the partner's action deterministically:

**a_j = U_i + a_i + f(1-a_i).**

Thus a purported no-history/no-partner-action condition cannot pay out and reveal this exact private payoff *before the next decision*, while claiming that the partner's action was unknown. This is not stochastic noise, but a structural logical inconsistency. Hiding only the partner-history UI leaves the information exposed in the monetary reward; obfuscating the display while keeping a real-time payout may also disclose it. Delaying feedback changes aspiration reinforcement and belief learning, confounding a pure information treatment.

**Design disposition:** RETIRE the independent "history hidden, full payoff shown" arm from the primary prospective one-realized-history comparison, rather than assert an inadequate masking patch. Under v0.1, **everyone receives true previous actions and earned payoffs openly on the same schedule**. The history visibility flag r is fixed to **1** in primary comparable worlds, not randomized; updates of the six families use the same legitimate action/payoff information. No missing-history placeholder m=1/2 is used in primary comparisons. This preserves reward learning and truthful conditional responses, at the cost of removing the earlier information-reveal factorial. It is an **explicit narrowing/amendment**, not completion of the prior design.

**One-round schedule:** (1) record genuine previous-period candidate actions visible to all, (2) assign the public fine f and random/choice matching regime z, (3) agents submit choices/rankings if enabled using only that public history, (4) clear pairs with the common scheduler, (5) agents choose actions simultaneously based on actual public history of current partner and their own internal state, (6) reveal current outcomes and *exact* payoffs to all, (7) update L belief state and R Q/aspiration state, (8) proceed to next period. For the initial reference period a scripted computer player's true actual prehistory is committed before choices. Do not claim human participants observed a fictional previous act.

**Physical arms retained for a candidate design:** fine presence versus withdrawal (f=1 vs f=0), exogenously assigned random matching versus centralized clearing of agent-submitted rankings, and *only in a separately labeled synthetic scripted-partner fixture* a truthful previously realized partner-cooperation versus defection signal. All assigned arms apply identically to all six generators. These arms are not a replication of Galbiati's fine-per-match random rematching, nor of a real prospective human trial.

**Cross-world fairness:** randomize one arm per independent prospective-style unit, no paired counterfactual fork response columns. If all agents observe history publicly, partner actions also become known and there is no valid "anonymous hidden-history" condition. Synthetic forks can still exist *as a separately named synthetic diagnostic*, counted as dependent on their parents and consuming budget.

## 2. Additional analytical checks: where model labels genuinely overlap

Let beta = ln 3 and sigma(x)=1/(1+exp(-x)). These cases hold under the v0 candidate kernels and are mathematical *assumption-conditional* predictions, not human results.

**Check A — N versus C at cooperative-only historical support.** N has nu=1 and C has kappa=1. If every *observed previous-partner action* is 1, the action probabilities are identical for any public fine f: sigma(beta(f-1+1)). At f=1 both are 3/4, at f=0 both are 1/2. Fine randomization alone cannot identify the personal preference vs responsive preference on that support. If the partner *actually defected* in the previous period, with f=1 and all else fixed, N remains 3/4 while C becomes 1/2. This is a valid two-policy restricted contrast, not an all-six identification theorem.

**Check B — C and L can have the SAME FULL stochastic action law under fixed matching.** Under the candidate L learning kernel, choose alpha_L=1, zeta=0, q_i,1=a_{j,0}, kappa_L=kappa_C and r=1. If each agent is matched with the **same partner** j for every period and both observe partner actions, then inductively q_i,t=a_{j,t-1}=m_i,t. Therefore at *every round* both families use the identical conditional action probability sigma(beta(f_t-1+kappa*m_i,t)). Because this equality is valid conditional on every public history and uses the same matching/action transition law, the **joint observed action law is equal for any fine schedule and any finite T** under this fixed-match arm. This is an explicit observational-equivalence subset, not the tautology "identical observable distributions cannot be distinguished": it states the *primitive parameter and physical matching conditions* that force equivalence.

If matching can change, L learns the previous *matched* partner, while C conditions on the current partner's previous action. On a genuinely reachable two-period history where last partner defected and the new partner previously cooperated, at f=1,kappa=1, L predicts 1/2 and C predicts 3/4. The difference is a mathematically checkable **intervention-sensitive prediction**, not a claim that all parameter values separate, nor that an identical human history can be forked. The history is only valid if both partner actions were truly recorded, as all-public v0.1 requires.

**Check C — current-fine sensitivity at initial state.** M with beta=ln 3 predicts P(a=1|f=0)=1/4 and P(a=1|f=1)=1/2. R at a fixed pre-outcome Q1,Q0 cannot respond to the *current, not-yet-received fine* within its prescribed action rule, so P_R(a=1|f=0,Q)=P_R(a=1|f=1,Q). With randomized f orthogonal to Q/history, this is a restricted first-decision contrast. It does not identify R uniquely against every learning/social family, and R's *future* actions can respond to fines through payoffs and its Q/A updates.

**Check D — matching and sorting.** The original four-person fixed-type oracle shows a possible pair-composition effect without an aggregate cooperation effect. But the v0 S model alone submits nonconstant ranking scores while the other five submit zero scores by assumption. Giving a model-recovery classifier *raw, exact preference scores* would be a potentially **trivial family fingerprint by construction**. Hence primary recovery should use **realized matched pairs and action/fine histories only**; report ranking availability/accuracy in a separate upper-bound/sensitivity diagnostic. Even pairs alone can reflect unequal action propensities, fixed-scheduler tie rules or endogenous selection; do not infer actual personal preference or motives from an assortative pair alone. Need a nontrivial observable comparison when rankings are **not** observed.

**Check E — S endogenous matching under the *actual proposed choice scheduler*.** Take four actors H1,H2,L1,L2 with true fixed S types tau=(+1,+1,-1,-1), and **genuinely recorded** public previous actions s=(1,1,0,0). Under the same score_i(j)=omega*tau_i*(2*s_j-1), omega=1, the clearing scheduler has exactly three perfect matchings. The total bid score of (H1,H2)+(L1,L2) is **+4**, whereas either H–L/H–L matching has total **-4**. Thus the **S-specific ranking rule** deterministically yields assortative pairs under the *choice* arm, while an exogenous uniform random matching produces them with probability **1/3**. Families M/N/C/L/R whose candidate ranking utilities are hard-coded zero yield probability **1/3 even under choice clearing**. This is a real **restricted endogenous matching** result about the written scheduler, unlike the previous externally assigned H/H–L/L pair-statistic fixture. It is **not universal**: past actions need not equal hidden types; omega may be zero; and hard-coding zero preference in the other families can artificially boost classifiability. Consequently pair outcomes (not raw ranking scores) may be a secondary identifying statistic, but it requires adversarial model alternatives in which more than one generator can prefer partners.

## 3. Versioned changes required in the v0 generator note

- Make primary r=1 public actions/payoffs an unconditional regime and mark r=0/no-history as **retired pending complete redesign**. The v0 description of neutral placeholder under r=0, conditional update with r, and ranking visibility must **not** be read as a valid experimental treatment.
- L q update in the proposed primary arm becomes w_t=(1-zeta*1[f_t>0]) since r=1; q updates from a partner's *actually observed action*. R receives true payoff each period and updates immediately. Public rankings of candidate prior actions must precede ranking submissions, and the matched partner's history is only selected from those same public records.
- The informational defection pulse must be based on a **real controlled previous-period action**; otherwise treat it as a synthetic intervention, not evidence about a human participant's action.
- Full seven- or six-class claims require model-recovery evidence across **all six** (M,N,C,S,L,R), not just a separation between chosen corners or a classifier using generator-specific debugging fields.

## 4. Exact check table

| Test | Setup | Expected |
| --- | --- | --- |
| Payoff leak | own action a_i=1, partner a_j=0, fine f=1, payoff U=-1 | reconstructed a_j=0 |
| N/C overlap | cooperative partner prehistory m=1, fine f=1 | both 3/4 |
| N/C overlap fine withdrawn | m=1, f=0 | both 1/2 |
| N/C separation | truthful m=0, f=1 | N=3/4; C=1/2 |
| C/L equivalence fixed match | alpha_L=1,zeta=0,q_1=partner prehistory, kappa equal | identical action transition kernels for all f,T |
| C/L rematch split | previous partner m_old=0, new partner prehistory m_new=1, f=1,kappa=1 | L=1/2; C=3/4 |
| M/R first-fine contrast | M beta=ln3; R fixed Q; randomized pre-choice f | M difference 1/4 between f=1 and f=0; R difference 0 |
| S aggregation bound | exogenous H/H+L/L vs H/L+H/L; original restricted fixed types | population mean=1/2 in both; both-cooperate per pair 5/16 vs 3/16 |
| S endogenous clearing | H1/H2 last action 1, L1/L2 last action 0, tau H=+1 L=-1, omega=1 | assortative matching total score +4, two mixed matchings -4 each; choice picks assortative with probability 1 vs uniform matching 1/3 |

Check vectors are in [a2-information-checks-20261025.csv](a2-information-checks-20261025.csv). Source-independent arithmetic may verify numbers; **independent implementation and scientific acceptance are not yet accomplished**.

## 5. Scientific stop and review conditions

**Leakage resolved as a *candidate protocol choice*:** no primary hidden-history arm exists to be contradicted by payoff displays. This does **not** establish that the six families can be recovered. The full six-model identified set, parameter uncertainty, exogenous matching histories, choice visibility, finite budget and mis-specified rivals remain OPEN. Stop if the only apparent recovery depends on the forbidden hidden-history arm, access to raw preferences/latent values, a generator label, paired fork responses from a single unit, or post hoc nonoverlapping parameter choices.

**Independent review still required:** have a context-independent verifier check history timing, eligibility and observables; implement exact transition/likelihood checks without reading a Hiyal output; adversarially search for more C/L/S/R observational equivalences and near-overlap regimes. This document is **our own analytic review**, not independent external validation.

**R-05 status:** A2 remains a *conditional synthetic identification candidate* with two concrete contrast families and an explicitly retired invalid arm. No R-06 approval or simulation budget granted.
