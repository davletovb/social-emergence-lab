# Experiment 003 — Cooperation mechanism identification and model recovery

**Protocol DRAFT v0.1 · 2026-10-25 · R-06 — NOT FROZEN / NOT RUN**  
**Research direction:** selected by owner 2026-10-25 in [R-05 decision](../../docs/research/r05/decision.md), with outstanding source/scoring exceptions.  
**Candidate development:** [Issue #36](https://github.com/davletovb/social-emergence-lab/issues/36) · [selection process #37](https://github.com/davletovb/social-emergence-lab/issues/37)  
**Strict claim ceiling:** structural **synthetic model-identification and limits** within explicit competing generative families. No assertion about which mechanisms govern people, populations or institutions.

## 1. Problem, falsification and predecessor constraint

Distinct microscopic mechanisms may produce similar observed cooperation histories. The proposed research asks **whether materially different generators can be identified or excluded** with a physically admissible, *fairly budgeted* schedule of public observations and fine/matching manipulations. A useful result could be **an identifiable parameter region**, a nontrivial **intervention-sensitive equivalence boundary**, or a negative result that realistic observables cannot uniquely identify causes.

Model selection by held-out forecast error **is not** mechanism identification. Cross-family ties, nested corners and structural observational equivalence are **part of the result**.

Closest documented precedents include [Galbiati et al. (2018)](https://doi.org/10.1073/pnas.1813502115) (fine assignment in anonymously rematched dyads), [Galbiati et al. later analysis](https://doi.org/10.1007/s40881-023-00159-x) of the **same** underlying data, [Yang & Yue (2019)](https://doi.org/10.1038/s41598-019-50083-6) (matching), [Wilson & Collins (2019)](https://elifesciences.org/articles/49547) (model recovery), [Flache & Macy (2002)](https://doi.org/10.1177/002200202236167) (learning). **Full close-original-methods audit remains a pre-run gate**; model-recovery tooling or simple theoretical equivalence alone is not a novel contribution.

## 2. Shared physical world and generators — proposal, not acceptance

The detailed provisional [six-generator mathematical rules](../../docs/research/r05/a2-six-generator-formal-spec-20261025.md) are the *candidate specification* and are **not yet independently checked**:

- **M:** pure material utility and externally assigned fines.
- **N:** stable individual normative *preference* (not a full Bicchieri social norm).
- **C:** conditional action based on previously observed actual partner cooperation.
- **S:** stable heterogeneous behavioral types submitting partner rankings to the **same common** matching scheduler; other generators also have the physical right to rank.
- **L:** sanction-masked learning of partner cooperativeness with explicit belief state.
- **R:** aspiration-based payoff reinforcement updating, no explicit social preference weight.

Proposed minimal world: **four actors, two dyadic matches each period**; binary contribution, matching assignment and publicly assigned fine `f∈{0,1}`, with material payoff `u_i=a_j-a_i-f(1-a_i)`. Types/beliefs/reinforcement state remain unobservable in the primary evidence regime; exactly one *realized synthetic history* per independently randomized society is the preferred prospective-style benchmark.

**Critical v0.1 repair:** Since `a_j=u_i+a_i+f(1-a_i)`, the partner's action is **algebraically recoverable from own exact payoff**. Hence the invalid treatment that hid partner action while revealing exact payoff has been **retired**. All primary worlds use **public truthful previous actions and payoff histories**, with history visibility fixed at `r=1`; the source-proof is in [A2 information repair](../../docs/research/r05/a2-information-repair-identifiability-20261025.md). A true-hidden-history study would require a distinct, reviewed feedback/payout change affecting learning, and is **not authorized** by this version.

## 3. Candidate original interventions

**Same physical action to each generator and at the same time, with matched information and resource accounting:**

1. **Publicly assigned fines:** fine on defection `f=1` versus absence `f=0`, varying by an explicit, predeclared assignment schedule without resetting learned states.
2. **Partner assignment:** externally uniform random pairing versus an identical centralized clearing algorithm that evaluates voluntarily submitted partner rankings; publicly observed previous actions are equally visible in both.
3. **Truthfully observed partner history** is a *measured condition*, not an independent conceal/reveal treatment. A separately named **scripted synthetic partner** may display a truly generated prior action (cooperate/defect) in independent fixture worlds; no forced human action or deceptive signal.
4. **Optional checkpoint forks** are a *synthetic-only* diagnostic. A parent and its forks are dependent and all arm costs count; no classifier in the primary one-history-per-society arm receives cloned counterfactual outputs.

**Still to freeze:** whether fine and partner-assignment schedules are simple factorial assignment or checkpoint transitions; time horizon and recruitment/independent world units; handling of switching partners with unequal true previous actions; payoff and matching timing; algorithmic accessibility of choice rankings. History visibility is **not** a free treatment variable in this draft. Preserve [Issue #36's](https://github.com/davletovb/social-emergence-lab/issues/36) synthetic A versus prospective-style B versus historical 2018-compatible C versus latent-oracle D **distinct** regimes; do not confuse the protocol's generator **C** with regime C.

## 4. Prior hand-derived fixture results — NOT trial results

| Example | Condition | Exact reference / boundary |
| --- | --- | --- |
| N vs C | True previous-partner cooperation always 1 and ν=κ=1 | Equal P(cooperate) for all f: **3/4 at f=1**, **1/2 at f=0** |
| N vs C | True prior partner defected; f=1, fixed parameters | N **3/4**, C **1/2** |
| C vs L | Fixed partner; α_L=1, ζ=0, q initialized to true partner previous action and κ matched | **Identical full observed action laws** over arbitrary fine histories |
| C vs L | Reachable switch to a different partner who cooperated, old partner defected; f=1 | C **3/4**, L **1/2** in defined special corner |
| S assorting | Two publicly known H and two L previous actions, ω=1 | Scheduler scores **+4** for H/H–L/L, **−4** for each mixed matching; S-choice assort probability 1, random matching **1/3** |
| Fixed-type composition | Same four types, exogenous H/H–L/L vs H/L–H/L | Aggregate mean **1/2** in both, pair-both-cooperate probability **5/16** vs **3/16** |
| Early M/R | First decision, randomized fine, R fixed initial values | M P(cooperate) **1/4 vs 1/2** as f changes 0→1; R initial probability unaffected by current fine (later reinforcement may respond) |

Fixtures: [original 23](../../docs/research/r05/a2-oracle-fixture-20261025.csv) and [21 added checks](../../docs/research/r05/a2-information-checks-20261025.csv). They are **restricted mathematical references**, no empirical results, no six-family recoverability proof and not independently reimplemented in a source-separate codebase.

## 5. Primary endpoints — decisions pending freeze

**Candidate primary target:** minimum over all **six true generators** of *unique mechanism-family recovery rate* using only held-out **independent synthetic society histories** at a fixed total resource budget. Define the admissible family set from held-out predictive performance and independently calibrated uncertainty. **Singleton set** = uniquely recovered; **two or more** = **undetermined**, *even if an individual forecasting representative wins a regularization/parsimony criterion*. Count boundary, mixed and structurally overlapping true cases in the unconditional denominator; report interior/overlap strata and undetermined rates separately.

**Observation policy:** public action, fine, matched partner ID, genuine previous actions and released payoff after each decision, assigned matching regime and dates/period. No private norm/type, latent learner state, counterfactual twin response, generator-ID flag, or **raw exact partner-ranking scores** in the primary classifier (the S model alone using nonzero ranking weights otherwise makes it trivially fingerprintable). Both matching protocols are available to every generator. Report any exclusion/same-rule ambiguity.

**Resource neutrality:** equal number of independent societies and total actor-periods/observed matches/fit attempts between intervention and sham-only assignments; no free synthetic fork cloning. Exogenous assignment probabilities and independent units documented in advance; group/world clustering not event-row independence.

**Needed before any run:** explicit and defensible disjoint train/calibration/test parent-world domains; **probability of identification** is not the same as statistical failure to reject alternatives. Freeze loss, admissible-set tolerance, uncertainty intervals, six-family prevalence, parameter/noise/misspecification distributions, minimal scientifically meaningful effect, multiplicity and stopping. An exploratory pilot may be used **only to determine variance and feasibility**, followed by a separate untouched confirmatory seed region if authorized. Pilot results may not be relabeled preregistered findings.

## 6. Independent evidence ladder and hard-stop criteria

1. **Specification audit:** independently reconstruct reward, information and timing constraints, the full matching optimizer and all six transition probabilities. Identify latent leakage (especially rank scores), synchronized RNG streams, and true fine-withdrawal effects.
2. **Exact independent oracle:** a separate implementation (no Hiyal internals or code-copying) reproduces restricted two/four-agent fixtures and exact C/L fixed-match same-kernel claim; log deviations and independent reviewer assumptions.
3. **Adversarial equivalence:** test parameter interiors, near-overlaps, boundary nesting (M inside N/C/S/L), misspecified/mixed models, false identifiability caused by trivial model-specific observation features. A null or non-unique identified set is a valid outcome.
4. **Explicit schedule/fixed total budget:** randomization unit, interventions and sham controls, seed/RNG namespaces, observation/assignment cost, held-out decisions and uncertainty are frozen before running.
5. **Original-source gap & empirical boundary:** source-audit Galbiati fine per **match** and stochastic match termination/rematching; do not claim sorting, disclosure manipulations or lab realism from its actual design. Human trials or dataset downloads require separate data rights, ethical and feasibility review.
6. **Written GO / REDESIGN / STOP:** if the only success requires unrealistically observed latent state, copying paired forks into primary outcomes, choosing obviously separated parameter corners, outcome-dependent tuning or an ill-defined intervention ⇒ **REDESIGN/STOP**, not a bigger simulation.

**Independent reviewers / R-06 gate:** none has verified the final v0.1 six-family design; this file is the start of R-06 **drafting**, not a completed R-06 acceptance. No costed tiny pilot, CI run, Hiyal implementation, raw subject access, source-complete novelty check or experiment freeze exists.

## 7. Work slices (no execution authorization)

- **P0:** separate independent check of math, physical observation/assignment and 2–4-agent fixture/learning variants.
- **P1:** publish exact observation budget, mechanism-family recovery decision, identifiable-set tolerance, parameters/true labels, prior-art audit and failure cases in a protocol revision.
- **P2:** determine tiny **manual** non-CI pilot feasibility and cost, only **after** P0/P1 are approved.
- **P3:** only after GO and resource approval, consider minimal Hiyal implementation and event journal test. Do not activate large GitHub Actions or parameter grids.

**Status at creation:** selected **Experiment 003 research question**; R-06 proposal **open/draft**. No experiment has started.
