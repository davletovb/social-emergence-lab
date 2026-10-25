# R-05 D — minimal two-patch scarcity oracle and source/novelty audit
**Date:** 2026-10-25 · **Status:** exact restricted synthetic model, negative identification result and prior-art check; **NOT** an Experiment 003 selection, independent reproduction of human data, four-generator empirical calibration, ecological forecast or approved R-06 protocol.

**Original narrow question** from [the D dossier](dossiers/resource-scarcity.md): holding pre-shock resource use constant, does an identical exogenous scarcity disturbance distinguish risk pooling, reciprocity, rule enforcement and selective exclusion? This oracle tests **only the originally nominated shock** and observes where it fails; it does not quietly add randomized access rights or reputation disclosure as a second treatment.

## 1. Common physical world and exact transition

Two actors A and B each have a **local renewable resource patch**. This is the smallest viable non-global case; a single stock would erase meaningful downstream/local-access differences. Per patch integer stock x_A,x_B∈{0,1,2}; capacity K=2, collapse floor 0 (once empty, local regeneration stops). Both actors have the same initial property access: can harvest their own patch; transfers of at most one resource unit from B to A are **physically permitted for all behavioral generators**, conditional on availability and room. A public eligibility marker for A can affect *voluntary choices* but is **not an extra physical prohibition** in one model. Every actor is able to observe the same stock, event and public prior-contribution records when they exist.

**One fixed decision cycle:**
1. Pre-shock state (2,2), observed initial extraction rule e_A=e_B=1, which is a *fixed control* and **not itself evidence of cooperation**.
2. Assigned **physical shock** d∈{(0,0),(1,0)} subtracts one from A's stock, respecting nonnegative stock: x^s_i=max(0,x_i-d_i).
3. B can choose h∈{0,1} (unit transfer to A) if x^s_B≥e_B+h and x^s_A+h≤2, otherwise h=0. B's transfer and A's receipt are distinct event records, never fabricated.
4. Allocation x^a_A=x^s_A+h, x^a_B=x^s_B-h. Extract e_A=e_B=1 only if each assigned patch has enough resource after transfer; in the tabulated cases both do. Post-extraction y_i=x^a_i-e_i.
5. Each local patch independently regenerates g(y)=min(2,y+1) if y≥1; g(0)=0. **Post-period stocks** x'_i=g(y_i). Total material conservation before regeneration: sum(y)=sum(x^s)-h+h-e_A-e_B. Regeneration is an explicit exogenous productive inflow of at most one per viable patch, **not conserved**.

Pre-committed previous A contribution k_A∈{0,1} and eligibility tag E_A∈{0,1} can be observed and used in policy decisions. **For the original one-shock experiment they are fixed across arms at k_A=1 and E_A=0**, while a preexisting rule-compliance incentive is fixed and available to all. A hypothetical *change* to k_A or E_A is a **new comparison/design amendment**, not another consequence of shock alone.

### Four behavioral policies in the EXACT SAME physical action set

These are deterministic **restricted policy corners** for identifiability analysis, not realistic calibrated human strategies or formal utility equilibria:

- **I (need-based risk pooling):** transfer h=1 if A suffered a resource shock and B can afford it; otherwise h=0.
- **R (reciprocal obligation):** transfer h=1 if A was shocked and its **true prior contribution k_A=1**, subject to feasibility; otherwise h=0.
- **E (enforcement-led sharing):** transfer h=1 when A is shocked and a **preexisting publicly enforceable sharing rule** demands h=1 and B can comply; otherwise h=0. The enforcement rule/penalty exists under the **same physical institution for every policy**; E is the behavioral type that obeys it.
- **X (selective exclusion):** transfer h=1 only if A was shocked and its public eligibility tag E_A=1, subject to feasibility; otherwise h=0. Eligibility is a preference/choice input, not a unilateral physical access-rights difference across generators.

Common physical shocks, possible donor actions, extraction/regeneration, and observable history are identical across mechanisms. Label E here means “enforcement” (not the R-05 empirical track); label R here means “reciprocity” (not the A2 aspiration learner).

## 2. Exact one-period outcomes: what the original shock can identify

With k_A=1 and E_A=0 in every world:

| Assigned shock to A | Generator | B→A transfer h | after shock x^s | after extraction y | after regen x' | total after regen |
| --- | --- | --- | --- | --- | --- | --- |
| none | I, R, E, X | 0 (A patch full) | (2,2) | (1,1) | (2,2) | 4 |
| 1 | I | 1 | (1,2) | (1,0) | **(2,0)** | 2 |
| 1 | R | 1 | (1,2) | (1,0) | **(2,0)** | 2 |
| 1 | E | 1 | (1,2) | (1,0) | **(2,0)** | 2 |
| 1 | X | 0 | (1,2) | (0,1) | **(0,2)** | 2 |

**Primary exact null:** The post-shock *aggregate* resource total is **2 under all four policies**; without local patches or transfer records, even the exclusion versus sharing policies cannot be recovered from total stock alone. With local patches and observed transfer, X separates (h=0) from I/R/E (h=1), **but I, R and E remain fully observationally equivalent** across the tested no-shock and shock histories, even though their off-support policy rules differ. Repeating the *same* history/shock alone gives more data but **no mechanism identification** of I vs R vs E. This is a concrete support-dependent structural observational-equivalence construction, not noisy optimizer failure.

**Conservation/regeneration caution:** The equal total after regeneration is specific to K=2, the zero collapse floor, e_A=e_B=1 and this simultaneous stock-transfer rule. A different feasible resource system might lose/gain total stock. The transfer case **rescues A's patch but collapses B's**; therefore do not call “sharing causes higher resilience” without specifying *whose* survival and what welfare function. Pre-shock extraction equality is not evidence that the mechanisms all exhibit equal cooperation.

**Off-support differences, *not a result of the original shock-only intervention*:** If A's publicly recorded prior contribution k_A were **0** while all else stayed the same, reciprocal policy R would choose h=0 while need-based policy I and enforcement-led E still choose h=1. If sharing enforcement were disabled, E might differ from I; if eligibility were changed, X might switch. Those are **new physical/history conditions** that must be versioned and compared fairly as separate interventions; this cannot be silently scored as an original D identifying success. All four policy corners are chosen for a pedagogical counterexample, not a scientific novelty claim.

## 3. Source-level novelty and observational compatibility (read original sources)

- **[Howe, Murphy, Gerkey & West (2016), PLOS ONE](https://doi.org/10.1371/journal.pone.0158940)** [original text, treatments T1–T4](https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0158940): real field sessions with subsistence users manipulated **need/shock**, revealed **current cooperative effort**, and kept **stable identities** in an additional arm. Help to people facing shocks was often conditioned on contribution history, without significantly increasing public-good contributions. This is **direct precedent** for separating need and reputation-related sharing. They did **not** use our two-patch K=2 regeneration/collapse toy, nor is our toy a model of their financial earnings.
- **[Cherry, Howe & Murphy (2015), Ecology and Society](https://doi.org/10.5751/ES-07390-200168)** and its original [Indiana University metadata](https://hdl.handle.net/10535/9804): lab test of voluntary pooling under idiosyncratic risk, which reduced income variance **without raising cooperation** in that game. This is also direct prior art on “risk pooling promotes cooperation” and its possible null.
- **[Anderies, Janssen, Lee & Wasserman (2013), Ecological Economics](https://doi.org/10.1016/j.ecolecon.2013.04.010)**: experiments in asymmetric irrigation systems with uncertain water supply/infrastructure, already covering resource-position inequity and exogenous environmental variation.
- **[Janssen & Anderies (2007)](https://www.jstor.org/stable/26522981)**: robustness depends on disturbances and trade-offs across objectives; beware declaring a global-best outcome based on one stock.
- Literature treatment *details* were inspected in the primary HTML of Howe; the Cherry/Anderies experimental insights here are supported by original published abstracts and repository descriptors, **not full protocols or raw data**. No article in these checks was a source for our exact K=2 transition values.

**Novelty assessment:** A generic result “shocks alter aid depending on needs and reputation” or “risk pooling helps maintain aggregate cooperation” is **not new** and the latter may fail. The mathematical example illustrates why the original limited observation/shock operator fails to distinguish three mechanisms; before claiming a novel identification-limit theorem, check whether this is merely a textbook support/observability argument.

## 4. Checklist / possible next narrow proposal (not approved)

- [x] Minimum **two actors, two local stocks**, exact integer feasibility/conservation and collapse/regeneration specified.
- [x] Same single physical shock applied to all four candidate policies; deterministic expected outcomes calculated for no-shock/shock, with aggregate-vs-local observability compared.
- [x] Primary published field comparator has directly documented shock vs effort vs reputation treatments; a transfer/need result is established prior art.
- [ ] Independently implement/validate exact state transition in different code or by an external analyst; the present source-independent arithmetic check is not a separate reviewer.
- [ ] Find a **non-tautological untested theoretical identification boundary or distinct physical operator** and audit it against the Howe/Cherry/Anderies originals.
- [ ] If amending to assign actual access rights or public contribution histories, log a **new treatment/version** with equal claims, costs and source checks; do not grant this design an automatic R-05 ranking advantage.
- [ ] Decide relevant estimand (patch-specific survival vs aggregate stock, individual aid/transfer, behavior) and likely sample units; K=2 deterministic settings are **not** human ecological calibration.

**R-05 disposition of D:** The exact study succeeds as a **tiny independent-of-Hiyal counterexample** showing original shock-only structural ambiguity; **D is NOT promoted automatically to replace C**. Scientific novelty, broader mechanism discrimination, empirical compatibility, source-reviewed scoring and owner authorization remain open. A valid outcome is to hold D and focus effort on A2 rather than expand D into another complicated, post hoc factorial.
