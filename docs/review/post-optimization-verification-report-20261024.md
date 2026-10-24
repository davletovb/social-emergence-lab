# Internal post-optimization verification report — 24 October 2026

**Status:** Engineering: **PASS within tested coverage**. Independent archived-output arithmetic: **PASS**. Scientific model identification: **PARTIAL / OPEN**. Empirical and external-human validation: **NOT ESTABLISHED**.

**Project:** Social Emergence Lab. [Verification gate #32](https://github.com/davletovb/social-emergence-lab/issues/32).

## Source, scope and provenance

- Consumer baseline: `social-emergence-lab` `c671814d1d087ea798211a5d53e11f02f013afc8`, pinned Hiyal `050d1de094f5d0813817e2020ec5c5da97d5c42e`. Independent arithmetic and replay tooling merged in [PR #33](https://github.com/davletovb/social-emergence-lab/pull/33), then one-time compatibility harness merged in [PR #34](https://github.com/davletovb/social-emergence-lab/pull/34).
- Original Experiment 001 source: lab `9848f0983d16b334a064046dcd29d5800da3e26a`, Hiyal `c6f3d5d650457077eaff4e26a0060bc6cee5dbdd`. Original full archived evidence unchanged.
- Original Experiment 002 V0 source: lab `bc870fe5ca75ad627421838b24fb1375eceaffdb`, Hiyal `4bb53aca5cffa6bc01b7558efbd690af7dbed79f`. Original public GitHub Release ZIP unchanged.
- Original Experiment 002 V3 Stage A/B: lab `3b6ddb43bc451f7b708fca6202eb2b087675e7fe` / `defb96ee73f4fe8a5258fc7178166e7bc7961b49`, Hiyal `e38419f857afbb53428284a312b0eb926a2320e2`. The original 432-parent journals and historical frozen/amended decision rules were **not modified**.

These are **retrospective engineering/scientific audits of exploratory synthetic studies**. The experimental unit is the independently seeded *society*; repeated events/windows and paired forks are not independent replicates.

## Execution evidence

| Gate | Execution | Observed result | Boundary |
| --- | --- | --- | --- |
| Current optimized Rust integration | [PR #33 final CI, run 40082979364](https://github.com/davletovb/social-emergence-lab/actions/runs/40082979364) | **PASS**: `cargo test --locked`, V0/V1 small pilot replays, current Hiyal observer/sensitivity and 001 evidence generation/seal/replay smoke, plus V3 committed Stage A/B decisions | Newly generated tiny smokes, not complete reruns of historical studies |
| Independent archival arithmetic | Same [run 40082979364](https://github.com/davletovb/social-emergence-lab/actions/runs/40082979364), `scripts/post-optimization-arithmetic-audit.py` | **PASS**: 001 720 trajectory rows/18 cells; 002 V2 36 parent pairs; V3 all nine cells/432 independent parent summaries, including post hoc paired NPS | No Rust call by audit; committed derived summaries are not original raw journals |
| V0 original journal compatibility | [One-time run 40083325558](https://github.com/davletovb/social-emergence-lab/actions/runs/40083325558) | **PASS**: original V0 ZIP outer SHA `ec8b1b15…12fab`; internal seal SHA `80666d6f…d8ec`; 12 archived runs Hiyal + independent reference replay under optimized pin | V0 exploratory development scale |
| Experiment 001 original journal compatibility | Same [run 40083325558](https://github.com/davletovb/social-emergence-lab/actions/runs/40083325558) | **PASS**: original archived GitHub Actions artifact outer SHA `1eb1160f…5fea5`; original internal seal SHA `5e585b12…a487b81`; original full grid 60 parents and 240 arm histories checked against optimized Hiyal | Artifact has limited Actions retention to 2027-01-19; keep archival issue open |
| V3 original journal compatibility | Same [run 40083325558](https://github.com/davletovb/social-emergence-lab/actions/runs/40083325558) | **PASS**: original Stage B cell 1 archive digest `80595fec…e6`; every cell file checked against committed seal; selected archived parent `130100` fully verified under optimized pin | **Only one of 432 parents** rerun against optimized verifier, not full new-pin V3 replay |
| V3 original full raw evidence | Previously passed [original historical-pin release verification, run 39901070657](https://github.com/davletovb/social-emergence-lab/actions/runs/39901070657) | **PASS on historical source/Hiyal pin** for all nine cells/432 parents, release readback | Do not relabel this as 432 post-optimization replays |

The first PR #33 smoke run was initially red because the new Python reference import wrote `__pycache__`; evidence generation correctly refused a dirty source tree. Disabling Python bytecode writes in the audit resolved it. The subsequent complete CI succeeded, **without weakening this fail-closed guard or changing evidence digests**.

### Independent numeric checks

- Experiment 001: across 18 cells, recomputing treatment/control mean Gini and paired difference from committed endpoint trajectories agreed with published decimal CSVs within **5.00003e−13** maximum. Three reported cells have negative observed paired means; they were not excluded. The count-variance benchmark `(500+20×11)/(1+20×11) = 720/221 ≈ 3.25791855` is the exact symmetric Pólya-urn prediction for count allocations, **not** a proof about Gini.
- Experiment 002 V2: recomputed paired seed differences and sample SD from all 36 committed per-parent records; checked H1 exposure decomposition against both prior and null groups. The observed late signed gap under information correction alone is `+0.218333` at false prior `δ=0.2`; new/re-added link plus conditional selection is `+0.001595` with historical exploratory interval `[−0.021438,+0.024627]`. A positive gap alone did not establish feedback regeneration.
- Experiment 002 V3: independent standard-library analysis of all 432 parent summaries preserves the `δ=0` comparison, complete nine-condition design, negative findings and post hoc sensitivity labels. **No cell satisfies the recorded positive active-regeneration classification**; fast `m=2` deletion-based settings exhibit exploratory compensation. Stage A's frozen decisions were **not** retroactively amended.

## Direct intervention already present in Experiment 002 V2

The pre-existing four-arm design provides part of the proposed E002-D1 graph-repair diagnostic. Using only committed 36-parent arm means:

| False prior | Information correction alone, late gap | Information + structural repair, late gap | Descriptive `both - information` |
| --- | ---: | ---: | ---: |
| `δ=0.2` | +0.218333 | +0.027222 | **−0.191111** |
| `δ=0` | +0.027778 | +0.027222 | **−0.000556** |

This is the effect of **the specific model-defined graph-repair intervention** in these synthetic branches, not a natural mediation estimand. The arm mean difference is consistent with substantial inherited-opportunity persistence; it is not proof that no other mechanism could generate the observed macro gap. The still-proposed C/D *policy-neutralization* intervention would be a genuinely distinct extension only after an exact protocol and causal estimands are written. Do not spend another grid on a B-versus-A contrast already observed.

## Analytical adversary: macro-level observational equivalence

An independent six-agent **one-step exact-probability counterexample** establishes that the same aggregate expected signed opportunity gap can arise from different micro-mechanisms. This is *not* a Hiyal trajectory, not a population model calibration, and not an empirically observed result.

Let agents `0,1,2` be boosted and `3,4,5` unboosted; schedule each allocator uniformly, two directed eligible edges per allocator, exploration probability `ε=0.1`. The recipient probability in a row is
`p(i|j)=ε/2+(1−ε) exp(β R_ji)/Σ_{k in row_j} exp(β R_jk)`.

**Model S (structural advantage, unbiased choice):** `β=0`. Directed adjacency by allocator `j=0,…,5`:
`[[1,2],[0,2],[0,1],[0,4],[1,5],[3,4]]`.
Eight of twelve directed opportunity edges enter boosted labels, so the expected boosted share is `8/12 = 2/3`, and signed excess `2(2/3)-1 = 1/3`. This is valid fixed outdegree-two, nonself adjacency but not balanced indegree; it represents a reachable-style unequal network, not the original **regular** graph initialization.

**Model C (choice advantage, balanced graph):** `[[1,3],[2,4],[0,5],[0,4],[1,5],[2,3]]`.
Every allocator sees exactly one boosted and one unboosted candidate, every candidate has indegree two. Set private beliefs to `0.7` for boosted and `0.3` for unboosted candidates, with
`β = log(((2/3−ε/2)/(1−ε))/(1−(2/3−ε/2)/(1−ε)))/0.4 ≈ 1.94426142147`.
Then each allocator chooses a boosted target with probability exactly `2/3`, yielding the **same signed gap `1/3`** although the opportunity network has no degree asymmetry.

Under the **discriminating intervention** `β ← 0`: Model S still predicts gap `1/3`; Model C predicts gap **zero**. Identical macro expected gap does not identify whether structure or preference generated it. Even matching this one-step intervention does not guarantee dynamic observational equivalence; trajectories and additional diagnostics are necessary.

This independently computed six-agent construction sharpens the reason we must compare rival microscopic models rather than infer a unique mechanism from a single aggregate statistic. It does **not** validate either model against real social behavior or test Hiyal's numerical implementation; the separate Rust reference/negative-control tests address that narrower software question.

## Decision table

| Evidence level | Status | What this licenses |
| --- | --- | --- |
| Current-pin software and deterministic replay | **PASS within coverage** | Reuse optimized Hiyal for future *exploratory* controlled simulations; keep old hashes and manifests fixed |
| Published historical numeric aggregation | **PASS** | Cite 001 and 002 within-model descriptive results with original design/uncertainty labels |
| Specific within-model intervention in V2 | **PARTIAL** | Discuss effects of the recorded correction/repair branches; not posttreatment natural mediation |
| Identification against rival models | **OPEN** | Exact toy counterexample proves one aggregate is insufficient; docking and rival-model prediction tournament remain |
| Multi-pattern empirical comparison | **NOT ESTABLISHED** | No extrapolation to human populations |
| External human methods/theory review | **DEFERRED** | Do not describe automated audits as external peer review |
| Novelty/confirmatory claim | **NO-GO** | Do not turn repeated reproduction or retrospective tests into a discovery claim |

### Next research gate

**GO to design** for a distinct small, exploratory next experiment, but **NOT GO** for strong scientific claims, publication novelty or a large new simulation grid. Freeze the next research question, competing mechanisms, intervention contrasts, observed summary **and trajectories**, and disconfirming outcomes **before execution**. A natural candidate is cooperation/institution formation under resource scarcity (also suitable for later simulation gaming); avoid simply swapping labels while recapitulating Experiment 002. Keep Hiyal deterministic scientific transitions separate from a potentially interactive game renderer. Preserve unresolved full new-pin V3 batch verification as research-integrity debt rather than implying completion.
