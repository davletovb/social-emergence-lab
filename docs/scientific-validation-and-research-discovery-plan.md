# Scientific Validation and Research Discovery Framework

**Version:** 0.1 — discussion and planning draft  
**Date:** 2026-10-23  
**Status:** Proposed framework; no new experiment authorized  
**Scope:** Social Emergence Lab research methods and future sociological experiments  
**Companions:** [Methodology](methodology.md) · [Theoretical framework](theoretical-framework.md) · [Research roadmap](research-roadmap.md) · [Experiment 002 V3 critical reanalysis](review/experiment-002-v3-critical-reanalysis-20261022.md)

> **Organizing question:** What can simulation teach us that is not already implicit in its rules, and which observation or intervention would falsify our explanation?

## 1. Purpose and boundaries

Social Emergence Lab investigates how explicit microscopic interaction rules generate population-level social outcomes. Its immediate objective is **not** to maximize the number or scale of simulations. It is to produce *discriminating, auditable explanations* while resisting false novelty, post hoc story-making, and unjustified generalization to humans.

This document has two workstreams:

1. **Scientific verification and validation:** test implementations, predictions, model identification, generality, and empirical correspondence independently of the original model/analysis code when possible.
2. **Future research discovery:** evaluate multiple sociological questions and select experiments for their ability to distinguish competing accounts, not just produce visually interesting behavior.

Hiyal remains the experimental execution and reproducibility library, not a source of sociological validity by itself. Work here is primarily deterministic Rust model specification, neutral data exports, independent verification in any suitable language, and carefully bounded scientific analysis.

**Current decisions:** Treat Experiment 002 V0–V3 as *exploratory synthetic evidence*, not a newly established sociological mechanism. The V3 raw-journal package is reported downloaded and retained on the researcher's local computer; external archival work is deferred by owner choice, **not independently checksum-verified**. External human academic review is also deferred until there is a clearer scientific contribution. Neither item blocks low-cost internal methodological work. No broad V4 grid or new Experiment 003 is approved by this plan.

## 2. Scientific claims and evidence levels

Never treat these standards as interchangeable:

| Level | Question answered | Evidence and expected deliverable | Current interpretation of Experiment 002 |
| --- | --- | --- | --- |
| V1 — Implementation verification | Did our program implement the intended transition rules? | Event-journal replay; invariants; independent reference code; exact small-world checks | Strong replay/reference work, but the program and its journal verifier share assumptions |
| V2 — Within-model causal identification | Does an intervention **on the model** change the outcome, compared with a well-defined counterfactual? | Matched checkpoint forks; intervention estimands; common-world comparisons; alternative explanations | Partial: direct fork contrasts are meaningful; inherited/new-link/selection accounting components are not independently identified mediators |
| V3 — Model robustness and identification | Do competing microscopic models fit the same macro-pattern? Which experiment distinguishes them? | Negative controls; competing-model tournament; adversarial tests; alternate graph rules and seed domains | Limited: nine factorial cells constrain one specified model family |
| V4 — Empirical correspondence | Do model variables and multi-pattern predictions correspond to measurements of people and organizations? | Source-level variable/rights matrix; matched construct definitions; out-of-sample pattern checks | Not established for the structural-memory mechanism; public processed cooperation data lack required dyad histories |
| V5 — External scientific assessment | Does a qualified outsider find the explanation, tests and novelty persuasive? | Human methods/theory review, checked prior art, candid response to criticism | Deferred; automated code review is **not** external scientific review |

A model may be useful at V1–V3 without claiming V4–V5. Success at V1 does **not** imply empirical realism, and one attractive macro-pattern does **not** identify its generating mechanism.

## 3. Verification toolkit

Each new model must declare which checks are applicable and report the results **including failures**.

### A. Independent implementation / model docking

- Create a deliberately minimal reference model from the *written mathematical specification*, using a separate implementation (for example Python, Julia, or a tiny Rust reference that does not import Hiyal internals).
- Compare initial conditions, conditional transition probabilities, one-step expected outcomes, and short trajectories under fixed inputs. Do not require byte-for-byte equality when RNG libraries or floating-point order differ; compare exact probabilities and output distributions as appropriate.
- Detect **shared-assumption errors**: an independently written model that copies a flawed specification can reproduce the flaw. Have one reviewer independently derive the rules and propose boundary cases.
- Store an interface-neutral fixture format (JSONL/CSV): parameters, initialized world, forced event sequence, predicted one-step outcomes, tolerances, and versions. Run only small fixtures in routine CI.

### B. Exact and analytical benchmarks

- Enumerate complete transition possibilities for 2–6 agent *toy worlds* when the real model allows a tractable restricted setting; otherwise construct a small mathematically explicit surrogate.
- Derive exact expectations for zero-feedback, randomized label assignment, equal-performance symmetry, frozen networks, and uniform edge-removal hazards.
- For survival, compare an exact or independently derived **uniform hazard** benchmark to uniform deletion only. Never import that curve as an exact null for lowest-belief deletion.
- Check conservation, exposure-accounting identities, permutation symmetry, event order, replacement candidates, and tie handling. Independently verify analysis metrics from exported states.
- Report whether agreement is algebraic, computational, or statistical; an accounting identity is a check, not discovery.

### C. Direct counterfactual interventions

- Create exact matched checkpoints, with the same initial society and information-correction rule. Manipulate a **specific causal lever**, holding other postcheckpoint rules fixed where logically possible.
- Define the *model-level* estimand before running: outcome under intervention minus outcome under the comparator, averaged across independent parent societies.
- Keep the original four arms (sham, information, structure, both) intact. If adding interventions, write a new protocol revision and explicit assumptions; never modify archived V0–V3 decision rules.
- Prefer outcome trajectories and mechanisms that differ between arms over one endpoint statistic. Describe contamination from intervention-induced changes in eligibility, learning, and subsequent RNG draw order.
- Do not call a contrast in posttreatment surviving edges an unbiased natural mediation effect: selective survival conditions on a variable affected by treatment.

### D. Rival-model and identifiability tests

- For one chosen macro target, build **at least three substantively different** generators: a null stochastic mechanism, the favored mechanism, and a plausible rival mechanism.
- Calibrate each to the *same training patterns*, using equal tuning budgets and transparent loss functions. Reserve an untouched diagnostic pattern or intervention for comparison.
- Ask whether outputs are **observationally equivalent** at the available aggregation; document indistinguishable models rather than asserting a unique explanation.
- Propose interventions that maximize disagreement between model predictions (not observed treatment effects) at reasonable computational and ethical cost.
- Blind the mechanism label during first-pass classification, where feasible, and test whether analysts can identify a mechanism from withheld outcomes.

### E. Adversarial falsification and generality

- Ask in advance: *what smallest manipulation or outcome would disconfirm our preferred explanation?* Record it in an experiment-specific falsification matrix.
- Test parameter boundaries and nuisance assumptions (network topology, degree distributions, clustering, quality heterogeneity, initial status, information noise, turnover, outcome window, observation and learning rules).
- Separate **stress tests** intended to find counterexamples from **robustness statements** requiring predeclared ranges and inference rules.
- Use held-out seed families and independent parent societies; forks, events, and windows within one society are dependent observations. Account for multiple comparisons or label them exploratory.
- Publish discovered failure regions as scientific outputs, not bugs to be tuned away.

### F. Multiple-pattern empirical comparison

- Require more than matching one aggregate indicator. Choose independent measured regularities such as distribution shape, trajectories, conditional response to interventions, heterogeneity, and network changes.
- Audit exact fields, time granularity, treatment definitions, population, missingness, measurement reliability, license and reuse rights **before** designing any calibration.
- **No inferred edge dynamics from aggregate player-round tables.** Tsvetkova processed cooperation files permit at most partial action/payoff comparisons; the V3 structural-memory estimand requires genuine dyad-level histories and correction timing. See the [empirical data audit](experiment-002-empirical-variable-audit-20261022.md).
- Do not claim that agreement with outside data identifies a mechanism: compare alternative explanations and independent holdout patterns.
- When a suitable dataset is unavailable, explicitly report “no valid empirical mechanism benchmark identified.” Do not manufacture proxy validity.

## 4. Experiment 002 — next small diagnostic, not V4

**Proposed work package E002-D1: counterfactual diagnostic at the correction checkpoint.** This is a limited *review design*; it has not been run or approved as a new outcome-generating study.

**Question.** After false information is corrected, how much of the remaining signed opportunity effect depends causally on retained opportunities versus a specified posttruth feedback policy?

**Compare at the same preintervention checkpoint and the same information correction:**

| Branch | Checkpoint graph | Postcorrection rule | Identified contrast |
| --- | --- | --- | --- |
| A — keep | Keep inherited graph | Existing feedback policy | Reference information-correction outcome |
| B — repair | Replace with an independently sampled, explicitly specified graph | Same feedback policy | Effect of **that repair intervention** against A |
| C — policy neutralization | Keep inherited graph | Random removal/addition and/or remove reputation dependence under a defined policy | Effect of changing **that feedback policy** against A |
| D — combined | Repair graph | Neutralized feedback | Factorial interaction between specified repair and policy change |

D is informative only if all four branches are feasible and well-defined, and **not all interventions can be held fixed** when a rule changes candidate sets. Existing information/both branches cover part of this logic; verify what is already recorded before introducing new machinery. Exact counterfactual coupling is not guaranteed merely by sharing a numeric seed.

**Measurements:**

- Primary: late **false-prior-minus-no-prior signed opportunity gap**; this is the model-level effect of the arbitrary initial prior.
- Mechanism diagnostics (secondary): continuously surviving checkpoint-edge count and conditional exposure, newly/re-added-link exposure, conditional selection, their *paired* false-prior-minus-null differences, and realized innovation.
- Graph-repair policy contrast: paired late gap B − A, and the analogous C − A and interaction (D − C) − (B − A) if D is run.
- Additional: trajectory by window, presence of a negative compensatory contribution, and diagnostics for intervention-induced distribution shifts in exposure eligibility.

**Pre-run work:** write equations and estimands, verify B is not simply reusing a historical “both” statistic with a different name, choose an exact checkpoint fixture, and show analytically how the branches differ. Re-run only a tiny predeclared fixed-seed pilot if a fixture cannot settle the question.

**Decision:** advance only if a new contrast distinguishes competing causal explanations rather than merely restating inherited + added + selection = total. A negative or observationally indistinguishable result is a valid stop.

## 5. Future research areas — candidate portfolio, not approved experiments

These are **topics with substantial existing literatures**, not assertions that they are unsolved. Novelty must be argued at the level of a narrow mechanism, conflicting predictions, and evidence that could distinguish them.

| Candidate | Discriminating research question | Competing microscopic explanations | Potential feasible evidence |
| --- | --- | --- | --- |
| **Mechanism identifiability / inverse social modeling** | Can multiple generators reproduce the same inequality/polarization trajectory, and which minimal intervention distinguishes them? | Independent luck, cumulative reinforcement, selective ties, heterogeneous quality | Synthetic blind prediction tournaments first; empirical outcome panel only if matched observables exist |
| **Hidden consensus and norm collapse** | Under what conditions does a publicly observed norm survive private opposition, then collapse abruptly? | Coordination incentives, punishment avoidance, misperceived private support | Behavioral norm/coordination experiments, treatment-specific published outcomes and prior art |
| **Institution formation and breakdown** | When do decentralized groups create enforcement and authority, and when do the resulting rules become brittle or extractive? | Repeated-game reputation, enforcement economies, leadership/centralization | Institutional/public-good experiments where actor and institutional variables are observable |
| **Cooperation under resource scarcity** | Why does scarcity increase cooperation in some networks and fracture others? | Reciprocity, punishment, selection/exit, resource-risk pooling | Public-goods/cooperation experimental data; distinguish aggregate from actor-level access |
| **Polarization and bridging relationships** | When do cross-group ties reduce ideological distance versus intensify defensive belief updating? | Homophily, persuasion, social identity, asymmetric credibility | Panel and network experiments if a matched, lawful dataset is found |
| **Mobility and organizational gatekeeping** | Can formally neutral selection produce persistent exclusion through legacy connections and incumbent sponsorship? | Information constraints, network access, prior advantage, heterogeneous productivity | Recruitment/promotion experiments or simulated counterfactual benchmarks |

No new experiment number or title is assigned by this document. In particular, “Hidden Consensus” is a **candidate**, not a rename of the existing roadmap's provisional cooperation-focused Experiment 003.

### First-pass selection rubric

Rate each candidate explicitly from 0 (weak) to 3 (strong), justifying every rating:

1. **Mechanism discrimination:** at least two meaningful rivals predict different intervention responses.
2. **Analytical tractability:** simple nulls, small exact-world predictions and independently implementable rules exist.
3. **Evidence feasibility:** suitable, ethically available observable variables or a well-defined negative-result contribution.
4. **Theoretical relevance and prior-art gap:** a specific unresolved distinction, not a broad “no one studied this” claim.
5. **Execution cost and complexity:** can a bounded pilot answer the first scientific question without a large grid?
6. **Generality/robustness:** a failure region can be mapped meaningfully, not hidden by tuning.

Apply the rubric only after a **targeted novelty/source scan** for each top contender. Do not generate precise ranked totals until actual papers, constructs, and available data have been checked.

## 6. Candidate protocol template

No implementation begins without a short versioned protocol containing:

1. **Problem and prior art:** what is established, what remains ambiguous, and citations/supplements checked.
2. **Three rival generative stories:** clear primitive rules and an explicit null; state whether they are truly different mechanisms.
3. **Observables/estimands:** event-level and aggregate variables, temporal units, interventions, and how labels are randomized.
4. **Discriminating intervention:** a particular manipulation with rival directional predictions and at least one possible negative outcome.
5. **Validation ladder:** analytical toy case, independent implementation fixture, replay invariant, counterfactual identification check, and empirical construct mapping if available.
6. **Predeclared decision and uncertainty:** effect size/margin justification, unit of independent replication, number of societies from a pilot variance estimate, sequential stopping, multiplicity family.
7. **Failure and provenance:** seed domains, raw-event manifest, version/commit, negative controls, missing/failed seeds, and interpretation limits.
8. **Scientific decision:** GO, REDESIGN or STOP with reason; a null result need not trigger a larger run.

## 7. Implementation roadmap (small, independently reviewable slices)

| Slice | Concrete deliverable | Acceptance gate | Cost/CI posture |
| --- | --- | --- | --- |
| **R-01 — Specification audit** | Annotated Experiment 002 intervention graph, estimands and ambiguity ledger; link existing Hiyal tests to each claim | Clearly distinguish intervention effect, conditional expectation identity, and posttreatment survivor composition; identify unresolved questions | Read-only, no simulation |
| **R-02 — Independent tiny-world oracle** | Reference fixture suite (small-state mathematics + external minimal implementation) | Independently derived edge/sampling probabilities and symmetry/null results match Rust within declared precision; record disagreements | Only small fixtures in CI |
| **R-03 — E002 discriminating forks** | Written E002-D1 protocol, existing-fork reuse assessment, then **optional** bounded feasibility test | At least one nontrivial rival prediction; arms causally interpretable; negative controls and costs specified before run | Manual tiny pilot only if justified |
| **R-04 — Rival-model tournament design** | Minimal alternative generators, held-out diagnostic targets, blind scoring plan | Same fit budget, documented train/test split, explicit possible observational equivalence | Design first; small CPU-only pilot after approval |
| **R-05 — Research-topic dossiers** | One-page candidate brief for each of six areas, with prior art, rival hypotheses, available evidence, and feasibility | Top 2–3 compare on the rubric; one candidate chosen or “none ready” | Research/design only |
| **R-06 — New experiment proposal** | Frozen Experiment 003 proposal **only after R-05 choice** | Hypotheses separable, toy null derived, minimum evidence and power/cost plan written; no output seen before freeze | No grid authorized by this plan |

**Order:** R-01 → R-02 → decision on R-03. R-04 and R-05 may proceed in parallel as inexpensive design work. R-06 is explicitly gated on candidate comparison, not calendar pressure. Preserve the option to stop after any slice.

## 8. Research operating principles and anti-patterns

**Do:**
- Treat *the society/independent parent seed* as the experimental unit; reuse matched branches for variance reduction while making no false independence assumptions.
- Freeze a versioned protocol, include failed seeds, retain original decision rules and date subsequent amendments.
- Use exact/null predictions and intervention contrasts before costly Monte Carlo replications.
- Store independent analysis code and neutral exports; prefer small mathematical oracles over copying the production model.
- Separate internal reproducibility from model realism and from external scientific review.
- Allow **“explanation not identifiable”** and **“no novel effect detected”** as successful research outcomes.

**Avoid:**
- Raising simulation counts when the available contrast cannot distinguish rival mechanisms.
- Interpreting “no statistically significant regeneration” as proof that regeneration is impossible.
- Declaring an active mechanism from a large total inequality gap when inherited exposure explains most of it.
- Interpreting mediator accounting identities or selectively surviving ties as causal mediation estimates.
- Selecting only favorable windows, metrics, graph families, or parameter values after seeing outcomes.
- Claiming that published study-level statistics validate agent-level behaviors, or inventing unobserved ties in processed datasets.
- Treating automated/LLM review, deterministic replay or passing CI as external independent sociological confirmation.

## 9. Near-term decisions for discussion

1. **Priority choice:** do we first derive a tiny-world analytical/reference implementation (R-02) or formalize the existing correction-versus-repair checkpoint contrasts (R-01/03)? Default: R-01 first, because it prevents implementing a duplicated or unidentified effect.
2. **What counts as progress:** a validated model transition, a discriminating counterexample, an explicitly indistinguishable pair of mechanisms, or a credible empirical construct mapping. The absence of a positive finding does not make a project unsuccessful.
3. **Next substantive direction:** shortlist by the rubric, not by attachment to “003” or an existing codepath. Candidate suggestion for initial discussion: **mechanism identifiability**, **hidden consensus/norm change**, and **scarcity/cooperation**.
4. **External validity:** remain explicit that human review and realistic behavioral correspondence are still needed for human-population or publication claims, but they are **not prerequisites** for internal exploratory methodological research.

### Record of decisions

| Date | Decision | Status |
| --- | --- | --- |
| 2026-10-22 | Treat V3's negative/compensatory results as exploratory and do not launch another large unmotivated grid | Agreed research direction |
| 2026-10-22 | Accept the owner's local retention of raw V3 evidence for the current milestone; external upload deferred | Owner reported; no independent local verification |
| 2026-10-22 | Defer cold-contact external academic review until research question and contribution are stronger | Agreed direction |
| 2026-10-23 | Use this framework to debate further validation and research candidates before approving Experiment 003 | Proposed; planning only |

This document does **not** approve new experiments, modify historical evidence, or supersede the existing [methodology policy](methodology.md).
