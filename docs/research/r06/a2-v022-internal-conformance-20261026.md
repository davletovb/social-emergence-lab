# Experiment 003 / R-06 — v0.2.2 **internal** deterministic conformance check

**Date:** 2026-10-26 (Asia/Tashkent). **Status:** internal smoke tests, **NOT independent P0**, not a protocol freeze or an authorized simulation. Research version: [v0.2.2 J/H4](a2-v022-joint-law-hierarchy-20261025.md), inheriting the [v0.2.1 six substantive kernels](a2-v021-six-kernel-design-20261025.md). Official source-neutral [fixture inputs](a2-v022-fixture-inputs-20261025.json).

## 1. What was actually executed

One assistant authored and executed two separate deterministic reference implementations **in the same work session**:

- Python 3.13 / standard library, no stochastic draws; **9 test groups passed after correcting initially mistaken hand-entered expectations**. Those corrections are documented below; do not count initial failures as “not observed”.
- C++17 compiled using `g++ -std=c++17 -O2 -Wall -Wextra -Werror`; the **second available executable language is C++**, **not Rust**. It independently encodes the fixed-vector F path/actor updates and prescribed two-atom H4 examples, but was authored by the same assistant and therefore **not a source-isolated independent verifier**.
- **45 common F/H4 output values** from both implementations (six F families × three fixed public paths, plus nine H4 configurations × three paths): **zero out-of-tolerance mismatches**, worst absolute difference `2.168404344971009e-19` under `abs ≤ 1e-12 OR rel ≤ 1e-9` declared comparison conventions.
- A separate Python exhaustive finite-event check enumerated **384 possible public one-period records per design** (`2 matching regimes × 3 pairs × 4 dyadic fine vectors × 16 action vectors`) and 384 second-period suffixes after the first prescribed record. It performed **8,064 H4 equivalence comparisons** on the specifically supplied two-atom `S→N→L` and `M→N/C/S/L/R` mapped examples, with **zero mismatches**, plus **30 conditional/marginal normalization results** with maximum absolute mass deviation `6.661338147750939e-16`. Enumeration is over these **specified examples**, not all legal H4 measures and not all possible two-period prefixes.
- Both implementations treat fine assignment at the **dyad** level after matching and action innovations as conditionally independent given the full state; conditional actor-vector law is a Bernoulli product. A separate internal diagnostic shows a common shared uniform could preserve actor marginals but produce a different joint event probability.

### Test correction ledger

**First Python invocation failed 2 groups from wrong manually entered expected values**, not hidden implementation exceptions. For the `first` record (`matching=01|23, f=[0,0.5], actions=[1,0,1,1]`), correct payoffs are `[-1,2,1,1]`, not the initial expected `[-1,1,1,0.5]`. Correct first L-q update is `[0.2,0.7,1,0.525]` rather than `[0.2,0.5,1,0.6]`. R initial aspiration and action-value check expectations were likewise set directly from the payoff/update equations before rerun. After corrections, all nine Python test groups passed. First C++ compilation also required ordinary C++ syntax/typing fixes; strict warnings-as-errors compilation then succeeded. Keep these corrections visible rather than reporting an unqualified first-pass success.

## 2. Evidence boundaries

This is **author-side numerical checking**, not a genuinely blinded second implementation, peer review, separate mathematical proof, full Rust replay or complete independent Python/Rust data submission. The workspace had Python and a C++ compiler, **no `rustc` or Cargo**, and no tool to launch isolated external reviewer contexts. The locally transcribed input contains the F and H4 numerical parameters and public records from the canonical JSON, but it is a **subset**, not a byte-identical download of the full official input file. The 45 cross-language matched values and 8,064 equalities confirm **only selected finite cases and the tested code**, not the continuous/finitely-mixed whole-family containment theorem, full H4 identifiability, or a statistical `C_α(D)` coverage guarantee.

Original four first-pass independent AI reports pertained to the **incomplete v0.2.1 version**, and the Python verifier's original source/test/result artifacts were not provided among those Markdown submissions. Neither their reported tests nor this internal C++ check may be relabeled **P0 independently verified**.

### Provenance of complete executable artifacts

A versioned, user-visible ZIP was produced in the related ChatGPT conversation on 2026-10-26 containing: local fixture subset, Python source/tests, C++ source, numerical outputs, strict-compile and test-run results, exhaustive-enumeration result, cross-language comparison JSON, README correction ledger and SHA-256 manifest. The ZIP is **not included or mirrored in the public GitHub repository** here to avoid presenting internally authored numerical answers as inputs to fresh isolated verifiers. An owner may archive it explicitly for repeatability. This Markdown entry records honest findings and bounded provenance; it does **not** certify that separate B/C reviewers have received or signed it.

## 3. Remaining mandatory R-06 evidence

- [ ] Seal **fresh genuinely independent A math, B Python, C Rust and D statistics** v0.2.2 reports **before** exchanging their solutions; request executable Python/Rust source/tests/results, not just narrative summaries.
- [ ] Use the **unchanged neutral official fixtures**, same event case IDs, whole-joint-vector probabilities and H4 actor-tuple integration. Cross-compare independent submissions, root-cause every discrepancy, show hashes and limitations. Author-side answers must remain hidden until reports are sealed.
- [ ] Full **15 unordered pair** structural law inclusion/separation analysis with **free rival H4 parameters**; finite two-atom examples and fixed F code do not certify unrestricted H4 family-law claims. In particular, report S⊆N⊆L as **constructive, parameter-qualified** and distinguish A0 versus A particular identified-set behavior.
- [ ] Independent statistical review of composite-model absolute and max-rival test calibration, preservation of all structurally compatible families, open-world empty-set adequacy and plausible fitting complexity; no achieved 95%-coverage claim yet.
- [ ] Freeze K/T/J, nuisance laws/truth weights, independent seeds and study costs **only after** actual verification; separate GO/REDESIGN/STOP and owner run approval.

**Disposition:** meaningful internal smoke checking completed. **P0 remains UNCHECKED; Experiment 003 v0.2.2 remains UNFROZEN; no Hiyal code, GitHub Actions, stochastic simulation, field study or empirical human inference was run.**
