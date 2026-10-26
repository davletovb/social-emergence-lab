# R-06 Experiment 003 — four independent-context v0.2.2 reports, critical reconciliation

**Date:** 2026-10-26. **Decision: PARTIAL ANALYTIC AND NUMERICAL SUPPORT; P0 NOT COMPLETE; full 15-pair H4 classification NOT ACCEPTED; statistical admission NOT READY.**

This is **owner-side reconciliation of four supplied Markdown reports**, not a new independent A/B/C/D verification and not a claim about human behavior. **Do not edit the original reports or silently fill absent executable artifacts.** The input report SHA-256 values below were computed from the exact uploaded bytes. The local [internal Python/C++ check](a2-v022-internal-conformance-20261026.md) predates this independent-context round and is comparison context only.

## 1. Exact source provenance and evidential weight

| Role | Submitted attachment / SHA-256 (hex) | Actual evidence | Limits / decision |
| --- | --- | --- | --- |
| **A math — Venice DeepSeek V4.1 Flash** | `2-v022-math-independent-verification-20261026.md` / `afbbd6bd46329e4782b83fc1813ae58e1c6e09e10b1bbe93a04b9e7b1f87496e` | Mathematical work on J, H4, bounded embeddings, fine-arm counterexample; declares 15/15 classified, mathematical GO | **Accept specified constructive maps and fine-arm witness; REJECT claimed zero-UNKNOWN 15-pair proof**, including the C→L rematching argument. Its fixed-F-to-H4 characterization also needs qualification. |
| **B "Python" — Venice** | `2-v022-python-independent-verification-20261026.md` / `f249968884a8805e9ec0ff4509e13378c0639cd536a5b86d07b7676016069d2b` | Narrative of in-browser **JavaScript** harness, some approximate numeric checks | **Not a reproducible Python submission.** No Python source/results/test log or readable hashes; inherited v0.2.1 kernels were not supplied, and reporter explicitly improvised C's action offset. Contains inconsistent A0 record probability. Treat as **partial, nonconformant**. |
| **C Rust — Claude Code cloud agent** | `a2-v022-rust-verifier-report-20261026.md` / `8e67c24b749cd9cc35fe8cdfe2361571bd779fa9c408003a47f1feceadcad952` | Reports std-only Rust, **51/51 tests**, **26/26 checks**, **19/19 mutations**, 13,036 NDJSON rows and 60 exact-enumeration law tables, zero-dependency build under rustc 1.89/1.97 | **Stronger implementation evidence.** Verified that `src/model.rs` and `results/experiment003-v022-rust-results.json` exist in branch `claude/sharp-franklin-gjyxye`; did **not** independently rebuild/rerun its full suite. Source is branch-backed, not imported into `main` or a passing cross-verifier Python/Rust reconciliation. |
| **D statistics — OpenAI GPT-6 Codex assistant** | `a2-v022-statistical-identification-review-20261026.md` / `415950e172ea0a9c98ef083d31512eb3832c6dfa9dbdd1c483dc527952d4df42` | Detailed H4 likelihood, structural inclusions, first-period equivalences, A vs A0 ordering, composite size and simultaneous-set coverage audit | **Substantive methodological review; REDESIGN statistical admission before GO.** No executed calibration, coverage/power or fitting result. Input-restricted rather than fully context-free. |

**Isolation caution:** Each agent independently *reported* its access constraints. Self-attestation is not a cryptographic seal. B/C mathematical computations may share fixture expectations; code/tests written by one agent can share an interpretation error. The original user attachments remain immutable in the project conversation; this file records their hashes instead of rewriting their text.

## 2. Findings supported by multiple reviews or direct independent derivation

1. **J is a coherent model *by stipulation***: fresh independent four-actor Bernoulli draws conditional on full latent state and current physical assignments. H4 with ≤4 **joint** actor-tuple atoms and iid actor sampling at fixed globals is mathematically definable, not a statement about an unrestricted real population.
2. **Exact constructive family embeddings:** `M⊆N,C,S,L,R`, `S⊆N` by `ν=δτ, η^N=η^S+ωτ`, and `N⊆L` by `κ_L=2ν, α_L=0`, with matching-appraisal `c` and joint actor-tuple atom weights preserved. These maps hold for arbitrary finite T and both policies under specified admissible domains; zero/boundary cases stay in the target. An M truth is compatible with **all six** family labels.
3. **T=1 exact law-class collapses:** `𝓟_C=𝓟_N=𝓟_L` and `𝓟_R=𝓟_M` at finite horizon **T=1** for either A/A0, because C/L begin from 1/2 and R begins with all Q zero; public broker is uniform at t1. The family relation matrix is **horizon-dependent**; a blanket “no equal pairs” cannot hold over all T.
4. **A vs A0 structural information ordering:** let E = every A period be random matching with both fines zero. `P_A(E)=8^{-T}>0`, and `P^A_θ(·|E)=P^{A0}_θ`. Thus equality of full A laws implies equality of A0 laws for any two admissible θ, hence for the corresponding common underlying truth **`I_A⊆I_A0`**. This does NOT mean finite-data estimation always improves with the intervention or that A's rare E history yields many A0 observations.
5. **Strict narrowing witness is valid:** one-atom homogeneous N `ν=0.75, β=4, ε=0` gives `p(f=0)=σ(-1)=0.26894142137` and `p(f=1/2)=σ(1)=0.73105857863`. M with `β=1, ε=0` matches the entire A0 no-fine law, but **every** admissible M has `p(f=1/2)<0.5` (maximum about `0.4937513`). Thus this N truth admits M under A0 but **not** A. Under A0, all six families may express that M law; under A, M is excluded. This is a population model example, NOT a finite-sample detection-rate result.
6. **Statistical admission not calibrated:** the proposed 0.025 absolute plus 0.025 **joint max-over-five** rival gate would provide 95% *single nominated true family* coverage only if each component's rejection size is **uniformly** valid over its whole H4 composite null. The gates have not been implemented or calibrated. Crucially **single-family coverage does not imply simultaneous retention of every observationally compatible family**. At an M truth the correct population set has all six labels; treating reductions caused by excluding other compatible labels as identification is scientifically invalid.

## 3. Blocking reviewer discrepancies and corrections (do NOT take majority vote)

### A mathematical: overclaims 15-pair classification

- **C→L as given is FALSE under rematching.** At `α_L=1,ζ=0`, L stores the *previous period's actual partner's* contribution; C observes the *current period's partner's previous* contribution. For a positive-probability history in which actor 0's previous partner 1 defected while its newly assigned partner 2 contributed, with `κ=1,β=1,ε=0,f=0`, C predicts `p=σ(0)=0.5`, L predicts `p=σ(-1)=0.26894142`. The same construction can occur under randomized matching A and A0. **This falsifies the particular mapping, not all possible free-parameter H4 L representations.** The mathematical report's phrase “broker choices identical, therefore C→L” ignores this state-history mismatch. Rust independently reports TV `0.266` (A) / `0.235` (A0) for that particular fixture map, consistent in direction.
- **Its claimed 9 inclusion + 6 incomparable + 0 UNKNOWN + 0 equal matrix is not certified.** The above C→L map fails; its other separation arguments compare displayed micro-state rules, not an optimized rival **observable H4 joint law** after actor-mixture marginalization and latent matching selection. A single distinct parameter choice or failed parameter map cannot prove that no alternative rival parameterization represents a law. At T=1 C/N/L and R/M are equal, contradicting the blanket “no equality” description without a declared horizon restriction. Use only **verified inclusions** in the matrix; label other global properness/incomparability results **UNKNOWN pending a formal free-rival proof**.
- **Its claim “F is the single-atom H4 case” is too broad.** F allows arbitrary actor-indexed heterogeneous parameter vectors. A single atom in H4 draws the **same tuple for all actors**, so an arbitrary heterogeneous F law is not automatically an H4 single-atom law; some fixed F vectors can be represented via mixtures, but that is a new proof task. Conversely H4 population mixtures cannot be replaced by deterministic F inputs. Preserve F vs H4 as distinct targets.
- **R's claimed “own-action dependence implies incomparable with all other families” is not a global observable-law proof.** It also skips the fact that Q uses `(a,f)` contexts: an action-dependent update to Q at t1 need not change the t2 action offset if the t2 fine is in the *other* context. Conditional private-state distinctions must not be promoted to whole H4 law separation without accounting for parameter mixing, assignments and alternative representations.
- Accept A's valid compact-domain embeddings and A0/N/M fine-arm counterexample; **do not mark all 15 pairs proven or approve P0 because A said GO**.

### B "Python" verifier: incomplete and conflicting numerical claims

- The report says its “Python” model was **executed in a JavaScript in-browser harness**, with no executable Python source or test logs. It also states v0.2.1 mechanism definitions were **not supplied** and explicitly flags **C implemented as a constant offset** rather than `κ_C×s_current_partner,t`; this is not v0.2.2 kernel conformance.
- The provided `first` record has `z=choice` and `f=[0,1/2]`; its **full A0 probability is exactly zero**. B lists `P_A0=0.0073948416` in the same paragraph as its “first-record joint”, apparently mixing an action-conditional calculation with the full public-record mass. This must be relabeled or corrected by B before the number is usable.
- B states “at t=1 only C differs from M”, inconsistent with the supplied fixed F fixture's **nonzero N and S action offsets, and nonzero L κ**, while M/R are equal and C/L coincide at t1 when κ parameters match. **Reject this diagnosis** without source-backed correction. Its claimed all-family H4 fixture proof is unsupported by accessible replay.
- Do not count B as a completed independent Python verifier until the **actual stdlib Python implementation, all six inherited equations, matched JSON results and hashes** are supplied or the role is rerun in a properly equipped isolated session.

### C Rust: independent code exists but report isn't a proof of whole H4 lattice

- The role C report declares 51 passing tests, 26 specification checks, 19 injected faults, 13,036 NDJSON rows, 60 whole-law tables, Rust 1.89/1.97. Source in `claude/sharp-franklin-gjyxye` includes `docs/research/r06/a2-v022-rust-verifier-20261026/src/model.rs`, and the reported `results/experiment003-v022-rust-results.json` is retrievable; **the whole Rust suite was not independently rerun in this reconciliation**. Its typed expectations/golden tests are also same-author.
- Unlike A, C correctly **refuses** to infer C⊆L from the failed rematching parameter map, or all 15 unrestricted-H4 noninclusions from finite checks. Its strong reported H4 S/N/L and M embeddings are **conformance** witnesses and corroborate the constructive proofs, not achieved finite-data coverage.
- **Limited third comparison performed here:** parsing C's **Markdown table** of six fixed-family × three A paths and four H4 example × three paths, compared 30 values against the earlier (NOT independent) internal Python archive `experiment003_v022_internal_verification_20261026.zip`: **30/30 within `1e-12`; maximum absolute difference `2.168404344971009e-19`**. See user-visible JSON reconciliation artifact. This does **not** compare C's complete 13,036-row NDJSON with an actual independent B Python result.

### D statistical: primary inferential design must change before GO

- D correctly formulates the **whole observed-society trajectory likelihood** as `sum_{actor atom assignments} (∏atom weights) × (∏matching broker factors × actor-vector action factors)`; draw four actor tuples **once** per society, not every period, and do not omit type-dependent broker selection.
- The conditional 0.025+0.025 union bound protects one true label only **if** both composite nulls meet the specified size. A finite-grid or naïve fitted bootstrap does not certify worst-case H4 size.
- For scientific mechanism identification, **simultaneous compatible-family retention `Pr(I(P★)⊆C(D))≥1−α_set`** is more pertinent than simply retaining the named true generator. A law-level confidence region intersected with each family is one proposed conservative approach; D suggests a bounded-feature/Hoeffding-region baseline. No uniform calibration or power is established and feature-based methods may have low power.
- D's exact T1 collapse and `I_A⊆I_A0` theorem are persuasive and can be verified algebraically; its verdict **REDESIGN statistical admission** is appropriate. No fitting feasibility, cost, K/T/J or earned 95% coverage was demonstrated.

## 4. Auditable cross-check performed during this reconciliation

An auxiliary machine-readable table of **30** values was generated by directly parsing the current C Rust *uploaded report* (§6.2/§6.3) and comparing to the **previous internal** Python ZIP values. The original attached C report SHA is pinned above. Summary: `30 rows / 0 outside tolerance / max abs 2.168404344971009e-19`. The full JSON with provenance and limitations is attached in the conversation artifact; it is NOT a B/C blinded common-case sign-off and does not certify the Rust report's NDJSON hashes or running compilers.

Independently recalculated elementary numeric examples:
- N/M A0: `σ(-1)=0.2689414213699951`.
- N fine half: `σ(1)=0.7310585786300049`; highest admissible M fine-half `≈0.49375130175789483`.
- C/L rematch particular map: `σ(0)=0.5` vs `σ(-1)=0.2689414213699951`.

## 5. Gate decision and minimum actionable follow-up

| Gate | Disposition |
| --- | --- |
| J conditional-action law / H4 hierarchy definiteness | **Supported as coherent model definitions**; the choice of independence/H4 remains a *synthetic modeling assumption*, not human evidence |
| M embeddings, S⊆N⊆L | **Constructively supported** for admitted parameter/domain maps; Rust finite checks support conformance |
| Full 15 free-parameter H4 pair identification | **NOT VERIFIED**. C→L map is invalid under rematching; A's 0-UNKNOWN classification rejected |
| Python↔Rust same-neutral full artifact replay | **NOT COMPLETED**. C code exists in a separate branch; B ran JavaScript and did not submit Python code/results |
| Uniform null calibration / full compatible-family retention | **NOT ESTABLISHED**. D requests inferential redesign before pilot |
| Empirical sociology discovery / human correspondence | **NOT CLAIMED** |
| P0 overall and experimental GO | **KEEP OPEN / NO GO** |

**Recommended smallest next actions (no new stochastic simulation or CI):**
1. Preserve original role A/B/C/D report bytes/hashes as a single archive. Do not revise reports retrospectively; request a **targeted corrigendum** from A: remove unproved `C⊆L`, horizon-free 15-pair claims and F=single-atom-H4 assertion; correct R-context argument.
2. Recover B's executable Python code if possible; otherwise rerun B with **both** v0.2.2 controlling law, inherited v0.2.1 equations and identical JSON fixture, requiring actual Python source/tests/NDJSON with case IDs. Demand B fix impossible A0 first-record probability and C time-history rule.
3. Retrieve/review C source and cases from its branch and perform **real same-case join** with B's new Python output. A 30-case prior-author comparator is a useful checksum, not the independent pair requested.
4. Keep matrix **PROVED INCLUSION / PROVED NONINCLUSION (with free-rival proof) / WITNESS-ONLY / UNKNOWN**, separately at T1, T≥2, A, A0 and H4. Prioritize C/L rematching and R contextual own-action histories. Do not assume the full matrix can be conclusively filled without substantive new theory.
5. Adopt **full structurally compatible-family set retention** as the proposed *primary* error guarantee for D's next methodological design revision, distinguishing it from one-family coverage. First establish a conservative law/feature confidence region or another uniformly valid finite-sample method, then address optimization certificates, out-of-family empties, finite-sample power and K/T/J costs.
6. Maintain **R-06 P0 unchecked**, no owner pilot approval, no Hiyal code, no CI, no new human data or claimed scientific law.

**Bottom line:** v0.2.2 resolved an important earlier mathematical specification gap. The independent-role Rust and statistical submissions are substantively useful. The mathematical role's blanket 15-pair GO and the incomplete JS-based "Python" role cannot be accepted. This round supports a **targeted verification/inference correction**, not another unfocused mechanism rewrite.
