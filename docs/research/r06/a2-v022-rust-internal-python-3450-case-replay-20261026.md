# R-06 v0.2.2 — 3,450-case Rust versus INTERNAL Python replay

**Date:** 2026-10-26. **Status:** bounded author-side numerical corroboration of independently reported Rust results; **NOT a replacement for an independent B Python submission, full P0 or statistical calibration**.

## Inputs, provenance and local execution

**Rust verifier:** Claude Code role C, source `docs/research/r06/a2-v022-rust-verifier-20261026/src/model.rs` and complete result table `docs/research/r06/a2-v022-rust-verifier-20261026/results/experiment003-v022-rust-cases.ndjson` on branch `claude/sharp-franklin-gjyxye`. The GitHub **blob SHA** of the 13,036-row table is `a47ebbecfe5ba550626c3e92510437e7e8ce8a85`; C's report says its **SHA-256** is `dc744c6d721b29437ec3df5c21a22b3b421da10e771cace9fcacb2e64ebea777`. The connector successfully retrieved the **actual 2,646,258-character 13,036-row contents** for analysis, not merely the reporter's Markdown. Rust's full test suite was **not independently compiled or rerun** here.

**Python comparator:** the previous author-side stdlib reference `python_reference.py` (originally included in the user-visible internal ZIP in the research conversation), extended with a deterministic `replay_rust_cases.py` script. The Python reference uses the **locally transcribed numeric subset** of the public neutral fixture (matching the six F and nine H4 example parameter sets) but not the byte-identical full neutral JSON; it has the correct version J/H4 mechanisms and public event convention. Both Python scripts are written/reused by the **same research assistant** as prior author-side smoke tests; they are **not** the missing isolated B Python verifier.

**Local run:** Python 3.13.5, `python -m unittest test_python_reference.py test_case_join.py`, **10 tests passed**; replay generated **3,450 distinct case-ID numeric values** with no duplicates, nonfinite probabilities or NaNs. The join-script mutation test shows it detects an intentionally corrupted row. Python NDJSON SHA-256 `58eb16919f325f5fbda3ccf602d55864f5541699070f76ee9bb4991689c95a39` at 327,744 bytes. This generated file and the full source/tests/manifest/logs are supplied separately in a **downloadable user-visible archive**, not copied into this PR to avoid contaminating independent verifier packets.

## Actual matched-case result (not merely the old 30-row comparison)

The published Rust blob was parsed into its constituent NDJSON rows. The same **3,450 numerical public-event case IDs** were selected according to this completely predeclared grammar:

- **Fixed F**: `v022/F/<M,N,C,S,L,R>/T1/P/...`, `T1/ACT/...`, `T2/ACT/after=first/...`, `PATH/<A,A0>/first[+next_*]`.
- **H4 selected mixtures**: nine named neutral two-atom distributions `S_distribution`, `N_pushforward_of_S`, `L_pushforward_of_N`, `M_distribution` and five `<family>_M_corner` under `T1/ACT` and `PATH`.

**Exact case-ID cardinality:** **3,450 Python and 3,450 Rust**. For every selected `case_id`, a portable 64-bit FNV-1a fingerprint was computed over sorted `case_id|quantized_numeric_value\n` records, using two independent 10-significant-digit and 12-significant-digit decimal-mantissa quantizations (portable mantissa/exponent representation). Results:

| Resolution | Python aggregate fingerprint | Rust aggregate fingerprint | Outcome |
| --- | --- | --- | --- |
| 10 significant digits | `1f5c17cefb3a676e` | `1f5c17cefb3a676e` | **MATCH** |
| 12 significant digits | `67d34ad9a4b908df` | `67d34ad9a4b908df` | **MATCH** |

The Rust/Python data were both traversed in canonical sorted case-ID order; the two *separately implemented* JS (Rust-data) and Python (local-output) fingerprint functions yielded the same result. The aggregate hash is **not** a mathematical proof against hash collision or a complete per-row floating-point error bound. Since values were deliberately rounded to 12 significant digits first, **do not claim an exact 1e-12 absolute tolerance for all 3,450 rows** from this check alone. The earlier direct **30-value** comparison showed a maximum absolute error of `2.17e-19`; that narrower checked subset has direct residual data.

**Interpretation:** substantial numerical consistency on the prescribed **chosen fixture cases**, including action vectors, state-dependent period-2 choices and H4 mixture examples. It is neither a fresh blind B-vs-C result nor a certification of all 13,036 Rust rows, the 60 entire T=2 law tables, H4 whole-family noninclusions, a passing 15-pair matrix or confidence-set calibration.

## Reproducible next step with direct per-case tolerances

The attached source package includes `compare_case_tables.py`, which accepts a downloaded copy of C's **full** NDJSON and locally generated Python NDJSON and performs a **literal join** by `case_id`, reporting exact per-row absolute/relative deviations, missing IDs and failures against `atol=1e-12,rtol=1e-9`. This comparator was locally tested on an exact match and injected mismatch; it was **not executed against a locally available Rust NDJSON file** in this workspace because that file was retrieved through the connected GitHub service, which does not materialize it to the local Python container.

**Remaining R-06 gate:** independent B must still submit **new runnable Python** source, logs, fixture SHA, true full case ID results and isolation registration. Use C's branch-backed case table for genuine independent B-vs-C numerical replay. Keep the 15-pair identifiability and statistical full-set calibration gates open even if a later full numerical join passes.

**No Hiyal integration, stochastic simulator, pilot, GitHub Actions run, human evidence or empirical scientific claim occurred.**
