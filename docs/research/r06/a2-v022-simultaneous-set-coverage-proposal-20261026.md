# R-06 — statistical admission v0.2.2-S: protect ALL compatible families

**2026-10-26. Statistical METHOD PROPOSAL; mathematically valid finite-sample coverage construction under stated assumptions, but NO H4 fitting solver, no achieved numerical coverage/power and no experiment GO.** This is a **targeted replacement for the primary inferential protection** in [v0.2.1 two-gate proposal](a2-v021-set-estimand-20261025.md), not a change to the [v0.2.2 physical J/H4 law](a2-v022-joint-law-hierarchy-20261025.md). Motivated by role D's original [2026-10-26 statistics review](a2-v022-four-report-reconciliation-20261026.md).

## Why the old primary protection is insufficient

The old `α_abs=.025 + α_rel=.025` argument can retain the **nominated true family** with probability ≥.95 *only if both composite gates are uniformly calibrated*. That conditional union bound is correct as stated (the max-over-five rival gate is **one** gate, not five unadjusted tests). However the science concerns **all observationally compatible explanations**. For truth P, define
`I_{d,T,H4}(P)={m: P∈𝓟_{m}^{d,T,H4}}`.

A new primary validity guarantee should be
`inf_{P∈⋃_m𝓟_m} Pr_P[ I_{d,T,H4}(P) ⊆ C(D) ] ≥ 1−α_set`,
where **D contains K iid independent four-actor SOCIETY trajectories**, not 4KT independent actor-period draws. For an M truth, I contains **all six families**. One-label coverage does not protect the correct ambiguity; a smaller admitted set could mean wrong exclusion of an equivalent mechanism.

## A provably conservative starting construction — finite-feature confidence region

Fix policy `d∈{A,A0}`, a finite T, and a **predeclared finite collection** of bounded functions `φ_1,…,φ_q: public_trajectory→[0,1]`. These must depend ONLY on publicly observed period/actor/matching/fine/actions/payoffs, with **no latent types, source labels or debug scores**; use the SAME function definitions for every candidate family. They can include such features as `1[period-1 dyad fine=1/2 AND actor 0 cooperates]`, `1[period-2 choice partner pairing=02|13]` and public-history-dependent indicators. Predeclare the feature catalog before looking at confirmatory D; otherwise adaptivity needs its own selection/error adjustment. For A0, features referring to impossible fine events are degenerate, not informative.

Let `Y_1,…,Y_K` be **iid entire society trajectories under fixed family globals/G**, and `\hat μ_j=K^{-1}∑_{k=1}^K φ_j(Y_k)`. Set

`r(K,q,α_set)=sqrt(log(2q/α_set)/(2K))`.

For each feature j, Hoeffding's bound yields
`Pr(|\hat μ_j−E_Pφ_j|>r)≤2 exp(−2Kr²)`.
The union over q features gives
`Pr_P(∀j:|\hat μ_j−E_Pφ_j|≤r)≥1−α_set`
for **every** underlying iid society law P, including boundary/zero-weight H4 mixtures, without requiring Wilks asymptotics, fitted-model bootstrap or a choice of generating family.

Define the random feature-law confidence box
`Q_φ(D)={P: |E_P φ_j−\hat μ_j|≤r ∀j}`.
The **conceptual** admitted set is
`C_φ(D)={m∈{M,N,C,S,L,R}: 𝓟_m^{d,T,H4} ∩ Q_φ(D) ≠∅}`.

**Finite-sample set-retention proof:** On the simultaneous feature-coverage event, the true P is in Q_φ(D). By definition every family in `I(P)` contains that same P, hence intersects Q. Therefore `I(P)⊆C_φ(D)`. The uniform ≥1−α_set bound transfers to the **entire compatible set at once**, not only one generator label. In particular an in-family `C_φ(D)=∅` event has probability ≤α_set (for the stated iid and exact-feasibility rule). This is a **formal conditional result for the proposed construction**, not measured operating performance.

For jointly analyzing **both A and A0**, either establish a joint confidence event or allocate `α_A+α_A0≤α_joint` by union bound; separate 95% claims do not provide 95% joint coverage.

## Where the guarantee could break in practice

1. **Uncertified feasibility is the major blocker.** The conceptual membership question “does *some* H4 law have all feature expectations inside these bands?” is a **continuous, nonconvex constrained existence problem** over ≤4 unknown joint actor-tuple atoms, weights and fixed globals; rare history variables, clipping and broker minima make it difficult. **One local fit that fails to enter the box is NOT a certificate of family exclusion.** An operational algorithm must return **ADMIT / CERTIFIED-EXCLUDE / UNRESOLVED**, mapping UNRESOLVED to **admit** for coverage. A mathematically guaranteed **outer approximation** of each family’s feature expectation image can safely enlarge C, but may lose all power. Numerical inner approximations alone can destroy coverage.
2. **Sparse or weak feature sets** can preserve all six families even when complete observable laws are distinguishable. Finite-feature feasibility measures **feature compatibility**, not complete structural law equivalence. The population set I refers to **full** public histories; `C_φ\I` can remain large for any K if the features are insufficient. Do not rename this as irreducible structural ambiguity.
3. **No free data-dependent feature mining.** Choosing q or indicators after observing confirmatory Y invalidates the displayed r unless the whole selection is incorporated into a valid uniform class bound. In addition, requiring *two stages* (“Q feature region” **and** a separately calibrated rival gate) will generally destroy the above full-set guarantee unless the second stage receives its own simultaneous family-set error allocation. Keep the old max-over-five statistic **exploratory/secondary** until its rejection event is controlled for every compatible family.
4. **Study evaluation G and atom support cap:** H4 defines a compact model class, not a distribution for truth evaluation. The ≤4-atom cap is a chosen synthetic restriction; full-set validity above holds for any fixed P in the model union under correct iid societies, but extrapolation to arbitrary random-effects worlds or human populations is not justified.
5. **Out-of-family sensitivity/power:** `C_φ=∅` is allowed if no family expectation vector meets Q. There is no uniformly high empty-set power against out-of-family P that approach the union arbitrarily closely at fixed K. Establish *certified outside-the-union feature separation* and predeclared distance strata for numerical OOF testing. Failure to solve one family’s optimization is NOT scientific evidence for `C=∅`.
6. **Nonidentification is mandatory:** M⊆all and S⊆N⊆L mean many proper sets cannot be singled out at infinite K, independent of code correctness. H4 family laws can be equal across zero/boundary representations. A true M law's admitted set should **retain all six** except on the allowed joint error event.

## Reporting should separate structural from finite ambiguity

For in-family truth m★ and certified structural `I(P★)`, define:
- `B_struct=|I\{m★}|` (irreducible alternative labels),
- `B_extra=|C\I|` (extra finite-data compatible candidates),
- `B_miss=|I\C|` (wrongly excluded genuine alternatives).

Exact identity (valid **whether or not the generating family is admitted**):
`|C\{m★}| = |I\{m★}| + |C\I| − |(I\{m★})\C|`.
Only compatible **other** labels are counted in the final missing-alternative term; excluding m★ must also be reported separately via `1[m★∈C]`. Do not subtract that same event a second time—the identity remains true without conditioning on truth-label coverage. Always report `1[m★∈C]`, `1[I⊆C]`, `C=∅`, `|C|`, and features/unknown-I limitations separately, rather than using B as if all nontruth labels were demonstrably false.

## Minimal GO gate for statistical science — no pilot yet

- [ ] Approve the **primary simultaneous full-set error target**, alpha and whether joint A/A0 protection is required, before selecting a statistic.
- [ ] Freeze the **public-only feature family**, q, T and K scheme *before any evaluation*; determine if it can distinguish scientifically meaningful contrasts.
- [ ] Provide a mathematically **safe over-approximation/certificate** for H4 feature-expectation feasibility with a solver that can return UNRESOLVED, and test the numerical bounding code independently.
- [ ] Establish coverage under finite precision, absent computation, effective missingness and held-out domain; a proof for exact Q intersect is **not an operational fitted procedure guarantee** unless exclusions are certified.
- [ ] Specify and test OOF separation targets, nonuniform-power limitations and budget trade-offs; a correct-empty claim needs actual evidence of adequacy-test power.
- [ ] Only then freeze K,T,J, any secondary rival statistic and planned runtime; **obtain separate owner authorization** before stochastic calibration/pilot/CI.

**Decision:** mathematically valid conservative *proposal* with a clear full-set coverage proof under stated iid and exact-feasibility assumptions, **not** a complete executable statistical test, achieved 95% coverage, experimental GO, or proof that the six underlying causes can be recovered.
