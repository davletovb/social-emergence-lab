# Documentation reconciliation audit — 28 October 2026

**Audit basis:** `social-emergence-lab` main at `2cc0c225b5b0954cd6c79f127c2012431b6d4c5c`, corresponding Rust model code, committed results and original custody reports, and the statuses of [scientific gate #65](https://github.com/davletovb/social-emergence-lab/issues/65), [historical R-06 #50](https://github.com/davletovb/social-emergence-lab/issues/50) and [R-05 decision](research/r05/decision.md). This audit corrects **current-facing status/navigation**, not original results or the chronological meaning of protocols. A complete line-by-line scientific re-verification of all historical studies is **not** represented by these documentation changes.

## Reconciliation decisions

| Affected documents | Previous risk | Disposition |
| --- | --- | --- |
| [Main README](../README.md) | Primarily described 001 and 002, omitting implemented 003 and current caveats | Rewritten around a broad social-emergence research mission, 001–003 portfolio, actual software, executed experiments, open methods and reproducibility |
| [Theoretical framework](theoretical-framework.md) | Core question and mechanism list appeared inequality-only; network role labelled future | Scope expanded to networks, reputation, cooperation/learning, identification and explicitly provisional institutions/gaming |
| [Research roadmap](research-roadmap.md), [current status](current-research-status.md), [methodology](methodology.md) | “A2 selected only, no pilot” and vague independent unit could be read as current; P0 gates underreported | Dated 2026-10-28 status, explicit 24-society pilot, 384 actor periods, independent society/K/J and H4 versus fixed-F, separate implementation/research/coverage |
| [R-05 landing](research/r05/README.md), [owner selection](research/r05/decision.md) | “No topic selected” was true at initial screening, but now contradicted subsequent owner choice | Clearly marked historical initial status, linked 2026-10-25 owner A2 selection and incomplete screening exceptions |
| [R-06 landing](research/r06/README.md), [old implementation note](../experiments/003-cooperation-mechanism-recovery/exploratory-implementation-20261026.md), [old framework](scientific-validation-and-research-discovery-plan.md) | “No implementation/pilot/test” at date of drafting now appeared to deny merged Rust/model diagnostics | Added dated status changes and links without editing original historical run claims; scientific P0 still open [#65](https://github.com/davletovb/social-emergence-lab/issues/65) |
| [Experiment 003 landing](../experiments/003-cooperation-mechanism-recovery/README.md) | No canonical per-experiment navigation from current law to code, pilot and author-side witnesses | Added model-version hierarchy, implemented/ran evidence, public-only boundary and full unsolved scientific gates |
| [Experiment 002 research question](../experiments/002-reputation-lock-in/research-question.md), [V0 formal model](../experiments/002-reputation-lock-in/formal-model.md) | “Not implemented/run” remains in originally proposed docs, while Rust V0–V3 were executed | Original design retained as historical; prominent links to V2/V3 results and replay-verified evidence, including post hoc and negative-outcome caveats |

## Evidence-to-claim classification

- **Implemented:** Rust code and tests, six A2 mechanisms, manually invoked bounded Hiyal pilot, per-period public export, independent replay checks, categorical finite-support tool.
- **Executed exploratory:** 001 Hiyal paired grid, 002 V0–V3 (not a new discovery; V3 active-regeneration negative), 003 24-society pilot and bounded exact/law diagnostics.
- **Analytically argued (author-side):** constructive H4 embeddings and public-event noninclusion witnesses; reviewed and numerically checked with preserved versions, but **not** source-isolated signed P0 proof, free-parameter global identification or a valid 95%-coverage test. The **N⊄S** cross-actor moment proof was rejected as inconsistent with H4 actor-iid sampling; **R⊄L under A0** remains UNKNOWN.
- **Not authorized/complete:** an externally preregistered confirmatory Experiment 003, empirical human causal validation, accepted complete H4 15-pair classification, calibrated composite-null simultaneous identified-set coverage, out-of-family adequacy, frozen K/T/J power/budget, and durable custody of every original exploratory artifact.

## Deliberately unchanged historical evidence

Original v0.1/v0.2/v0.2.1 protocol drafts, dated V2/V3 decisions, raw-results JSON, execution hashes, source pins and null/post hoc findings remain intact. A dated header that said “not implemented” **before execution** is not itself evidence fabrication and should be accompanied by a supersession note, not silently altered into a past-tense claim that the run already existed. The old #50 closure does **not** check scientific acceptance; [#65](https://github.com/davletovb/social-emergence-lab/issues/65) remains open.

## Review checks

- Reconcile factual claims against existing Rust paths, design documents, executed result and custody reports rather than infer progress from a closed issue or merged PR alone.
- Validate relative Markdown links for updated entry points against the repository tree; record the result in the documentation PR. Documentation-only changes do **not** justify another heavy GitHub Actions run or claiming model tests have newly passed.
- Preserve model source and results as-is. There is no change to experiment parameters, simulator runtime, original results, evidence bytes or Hiyal dependency.
