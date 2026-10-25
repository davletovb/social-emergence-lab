# R-06 / Experiment 003 — v0.2.2 common independent-verifier fixture contract

**Date 2026-10-25 · VERSIONED TEST INPUTS ONLY · source-neutral verification handoff, NOT proof, simulation, P0 completion, or scientific result.**

Read **only** the new authoritative [v0.2.2 mathematical joint-law/hierarchy contract](a2-v022-joint-law-hierarchy-20261025.md) together with the unchanged v0.2.1 substantive [six action/matching rules](a2-v021-six-kernel-design-20261025.md). Use the exact same machine-readable [fixture input JSON](a2-v022-fixture-inputs-20261025.json). It is **not** permissible to replace missing values silently, adopt a report's previously tested values, or copy the other implementation. Both verifiers must reconstruct **from the specification**, sign and hash their own code/output, and submit **before** exchanging results.

## 1. Required verifier roles and inputs

**B — Python**: run a new independent v0.2.2 conformance check in stdlib Python; supply `python_reference.py`, `test_python_reference.py`, `python_results.json` (or a new versioned equivalent) and every failure/explanation. The previously self-reported 73/73 and 14/14 mutant results apply only to the reviewer's **conditional v0.2.1 completion**. Their executable source/results were **not** in the four original user-uploaded reports; report original hashes if reusing their own source and identify every v0.2.2 change. No Hiyal or author's desk-check code.

**C — Rust**: update an independent zero-dependency Rust reference from the attached standalone model contract and the **same exact fixture JSON**, with source, tests and results JSON. The original Rust report included full source in an appendix, but that was **not independently rerun during first reconciliation**. No dependence on the Python implementation or project Hiyal. State changes relative to v0.2.1, compile/test output and code/results hashes.

**A — mathematical**: derive the J conditional product, H4 marginalized law and inclusion maps from the v0.2.2 contract without access to either submitted implementation; disprove incorrect generalizations and indicate what the H4 atom cap changes.

**D — statistical**: audit whether a family-wise model-admission method with unknown H4 atom locations/weights and overlapping families can actually achieve valid composite-null coverage; **do not assert that a 0.025/0.025 split empirically reaches 95%**. State calibration blockers and whether an even smaller initial inferential target is needed.

Do **not** supply the owner's previous reconciliation or expected verdicts as input to a fresh isolated verifier. Verification reports may cite the **current contract** and the neutral fixture inputs, but no other model's answers.

## 2. Canonical physical event representation

One public period record has exactly these fields:

`{"period":1,"z":"choice","matching":"01|23","dyad_fines":[0,0.5],"actions":[1,0,1,1]}`.

- Actor order is `0,1,2,3`; bit 0 represents **actor 0**, not the most-significant bit of a string. If encoding action masks, define `mask=∑ a_i·2^i` and document it.
- Matching order is `01|23, 02|13, 03|12`; dyad 0 **contains actor 0**. Fines index those **two** dyads in canonical order. No independently drawn fine for each actor.
- `z` is assigned before partner selection; the dyad fines follow selection, and actions follow fine notification.
- Released payoffs are **not** an extra random draw. Recompute from public records and optionally expose `[u0,u1,u2,u3]` for diagnosis; do not use private Q/τ/η as classifier inputs.
- The `first`, `next_rematch`, and `next_repeat` examples in the shared JSON are **candidate physically possible public records**, not trajectories with guaranteed or equal likelihood. `first` has zero probability under A0 since its regime/fine assignments are impossible there: report exact zero, not a conditional A0 density of a disallowed sequence.
- For a fixed state and parameters, conditional on current assignments and pairing, the probability of action vector is the **product** of four Bernoulli factors. Under H4, integrate latent actor tuples over the specified **joint actor-tuple** measure. Cross-field correlations within an actor are retained, although tuples are iid across actors.

## 3. Required output comparison (identical inputs)

All verifiers should output results tagged `experiment003-v022`, `model_sha`, `fixture_sha`, `language`, `implementation_sha`, `completion=J_independent/H4_or_F`, and `case_id`. Numeric floats are compared **by value**, not byte-identical JSON formatting.

| Test class | Quantity to reproduce independently | Expected resolution |
| --- | --- | --- |
| Stage payoff | Four action-pair payoff tuples at f=0 and f=1/2 | exact half-integers |
| t1 matching | all 3 matchings under random and choice for each F model | numerical ±1e-12 |
| t1 action | each actor's p and full 16-vector joint action probabilities at F inputs | numerical ±1e-12 |
| Two-period physical path | P(first), P(first,next_rematch), P(first,next_repeat) under A; disallowed A0 paths zero | numerical ±1e-12 abs/1e-9 rel |
| Choice after first | three t2 broker probabilities, all six families, both matching modes | numerical ±1e-12 |
| L and R updates | q_i, Q_i(a,f), A_i after first; compare t2 action kernel and terminal-update visibility | numerical ±1e-12 |
| H4 mixtures | t1 model-averaged law; H4 S/N/frozen-L mapped equality of **joint** two-period law | mass difference ≤1e-10; TV ≤1e-10 |
| M embeddings | all six H4 representations of M produce same t1/t2 joint law | TV ≤1e-10 |
| Alternative coupling | shared-uniform J-shock vs J-independent with identical p_i | **positive** TV generally; separate diagnostic only |
| Permutation | relabel actors, canonicalize matching/fines; match relabelled full law | ≤1e-10 |

**Tolerances are numerical comparison conventions, not theorem assumptions.** For extreme logarithmic tails, compare log probabilities directly; no underflow-derived false zero. A correct proof of exactly equal kernels has higher standing than a finite floating-point match.

An output artifact may be NDJSON sorted by `case_id`; include a plain-text test run log (passing and failing) and a ledger of any *new completion*, numerical corner or parameter-domain deviation. For exhaustive T=2 cases, publish a machine-readable matching-order-aligned per-record vector (or stable full table with a clear schema); **hash the final table as integrity metadata**, but don't require byte-identical floats across languages. Quantitative comparisons must actually join on identical public `case_id` and event values.

## 4. Required independent failure injection and validity checks

- Flip the fine's payoff sign, switch actor vs partner signal in C, invert L mask, use updated rather than prior aspiration in R, exchange fines across dyads, and couple two actors through one shared uniform. Every incorrect kernel should be detected by an appropriate check; mutation numbers are not a scientific accuracy score.
- Check normalization conditional on **all** broker choices/fines/histories, not only one F parameter vector.
- Test S→N→L embeddings with correlated `η,ν`: **do not** impose matching-taste independence in primary H4. Test zero-effect and mixed-sign families.
- **Important epistemic rule**: "an F-level conditional law agrees in two languages" does not certify H4 identifiable sets; a finite H4 example does **not** certify all admissible distributions; a 15-pair table with single parameter representatives does **not** establish global separation.
- Preserve a three-column discrepancy log: `case_id | B versus C numeric/exact difference | root cause and independently confirmed resolution`. Never resolve disagreement through a majority vote; both must independently recalculate any corrected case.

## 5. Why first-pass reports cannot yet count as cross-language confirmation

The user supplied **four Markdown verifier reports**, not B's stated `python_reference.py`, `test_python_reference.py` and `python_results.json` as executable attachments. C's Rust source is embedded inside a large Markdown appendix but no extracted standalone independent crate was supplied in the four uploads. The two original verifiers used different illustrative parameters and named alternative couplings. Reported 73/73 and 49/49 passing tests **are not** paired cross-language test results.

**Submissions needed before P0 can be marked complete**: (1) both executable source/test/result artifacts with hashes, (2) the complete cross-language common input fixture results, (3) independent mathematical J/H4/pairwise audit, (4) an explicit complete discrepancy register and limitations, and (5) a separate source/methods judgment on calibration. A code-review approval or an AI-only narrative consensus is not a substitute.

**Execution status at publication:** test inputs and instructions prepared; **no new external B/C independent session has executed the v0.2.2 packet; no simulations, Hiyal changes, routine CI or human data usage**.
