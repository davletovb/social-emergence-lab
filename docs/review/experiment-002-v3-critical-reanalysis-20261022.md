# Experiment 002 V3 — adversarial computational reanalysis and external-review request

**2026-10-22 | POST HOC, EXPLORATORY, SYNTHETIC ONLY.** This is an automated critical audit, **not an independent human academic review**, empirical replication, preregistration, or proof of novelty. Submitted for independent challenge under [Issue #22](https://github.com/davletovb/social-emergence-lab/issues/22). Underlying V3 [Stage A](../../experiments/002-reputation-lock-in/results/v3-stage-a-20261022/README.md) and [Stage B](../../experiments/002-reputation-lock-in/results/v3-stage-b-20261022/README.md) results and decision rules are unchanged.

## Specific question and numerical method

The original V3 rule classifies **treated-arm** mean posttruth new/re-added link exposure + conditional-selection exposure (NPS). A synthetic society with `delta=0` may, by chance, have nonzero NPS even though its *population expectation* is zero. The model already saved a matched `delta=0` parent for each false-prior `delta=0.2` parent.

Define a different, arguably more informative **paired false-prior effect** on the accounting quantity:

`paired_NPS(seed,cell) = (added + selection)_(delta=0.2,information) - (added + selection)_(delta=0,information)`, averaged over events 501–600.

This contrasts treatment histories while holding the original random seed, graph genesis and label assignment matched; subsequent stochastic histories may diverge. Importantly, this still **does not causally separate new-link and selection mechanisms** or identify a natural direct/indirect effect. The direct intervention on the simulation is the initial false prior, not posttruth mediator values. Use this alongside the signed opportunity-gap difference (a model-level treatment effect), not as a substitute for it.

**Source and check:** separately retrieved **all 48 saved parent summaries** for each of the six reputation-sensitive cells from the merged `main` repo (288 distinct independent parents). Computed means, across-parent sample SDs and t 95% intervals with df=47 (t=2.011741), never treating repeated events or matched branches as independent observations. The three random-turnover cells (144 other parents) were already shown in [Stage B](../../experiments/002-reputation-lock-in/results/v3-stage-b-20261022/README.md) to have *identical* `delta=0.2` versus `delta=0` postcorrection forks (exact paired difference zero); the accompanying **stdlib-only independent script validates all 432 files** upon a complete checkout: [`docs/review/v3_independent_paired_audit.py`](v3_independent_paired_audit.py). It does not call the Hiyal model, original jq rules or generate new simulations.

## Recomputed late-window NPS contrast

| Rewire period | Removal/addition mechanism | Treated-arm mean NPS | Matched false-prior-minus-null NPS [unadjusted 95% t interval] | Inherited-exposure matched difference | Signed opportunity-gap matched difference |
| ---: | --- | ---: | --- | ---: | ---: |
| 2 | random / random | −0.012 | **0 exactly** | 0 | 0 |
| 2 | lowest-belief / random | −0.0462 | **−0.0326 [−0.0512, −0.0141]** | +0.1420 | +0.1238 |
| 2 | lowest-belief / reputation-preferred | −0.0374 | **−0.0747 [−0.1195, −0.0299]** | +0.1741 | +0.0954 |
| 10 | random / random | +0.004 | **0 exactly** | 0 | 0 |
| 10 | lowest-belief / random | −0.0166 | **−0.0186 [−0.0295, −0.0078]** | +0.1227 | +0.0988 |
| 10 | lowest-belief / reputation-preferred | −0.0139 | **+0.0107 [−0.0141, +0.0356]** | +0.2186 | +0.2200 |
| 20 | random / random | +0.003 | **0 exactly** | 0 | 0 |
| 20 | lowest-belief / random | −0.0006 | **−0.0077 [−0.0164, +0.0009]** | +0.0849 | +0.0721 |
| 20 | lowest-belief / reputation-preferred | −0.0067 | **+0.0045 [−0.0110, +0.0201]** | +0.1672 | +0.1733 |

The treated-arm numbers for the six reputation-sensitive cells were independently recalculated and agree with the original published means. The *paired* NPS means and intervals are new **post hoc sensitivity analyses**, not replacements for the frozen Stage A/ amended Stage B classifications.

### Multiple-comparison and distribution-free challenge

For the six reputation-sensitive cells, **exact signs of the paired contrasts** are respectively (positive/negative):
- period 2 deletion-only: 16/32; full-feedback: 14/34;
- period 10 deletion-only: 15/33; full-feedback: 24/24;
- period 20 deletion-only: 23/25; full-feedback: 27/21.

Exact two-sided sign-test p-values for the three negative treated-minus-null cells (m=2 deletion-only; m=2 full-feedback; m=10 deletion-only) are approximately 0.02930, 0.00552, 0.01328. Under a **Holm correction across all nine exploratory cells**, the corresponding adjusted exact-sign values are **0.20513, 0.04964, 0.10627**. Therefore only m=2 full-feedback *just* crosses 0.05 on this particular exploratory multiplicity family; that is **not confirmatory evidence**, especially with the *post hoc* choice of estimand, tests, and comparison family. Normal/t intervals do not establish robustness to skew, heavy tails, or researcher degrees of freedom. The original Stage A null t checks raised sensitivity concerns.

**Important nuance:** false-prior negative matched NPS at fast turnover means new links/selection tend to **offset** historical advantage in the studied model; it does not establish a human compensatory reputation law. A positive late *total* signed gap remains because inherited-edge exposure is larger; for example full-feedback m=2 has +0.1741 inherited exposure minus 0.0747 paired NPS and stochastic innovation, giving about +0.0954 realized gap.

## Adversarial scientific verdict and failure modes

1. **Strong within-model result, not original sociology:** structural carryover dominates the observed posttruth inequality in this model; no full-feedback cell has a positive matched NPS 95% interval. The overall 2016 arbitrary-reputation inequality mechanism was already known. See [Frey & van de Rijt (2016)](https://doi.org/10.1038/srep38304), [van de Rijt (2019)](https://doi.org/10.1086/702899), [Tsvetkova (2021)](https://doi.org/10.1098/rstb.2020.0299), and [Papamichalis et al. (2026)](https://doi.org/10.1038/s41586-026-11154-7).
2. **Potentially important alternative mechanism:** deletion of the lowest-belief partner (with/without reputation-preferred addition) may *mechanically* generate compensation when rewiring is frequent. Reviewer should demand a prediction from the actual candidate-set/deletion dynamics, not a retrospective story inferred from NPS.
3. **Posttreatment selection:** `inherited_per_surviving_edge` conditions on links that never died; their group composition can change through selective deletion. Nearly constant exposure per remaining edge is a descriptive pattern, **not** an unconfounded causal structural-memory estimate or an exact uniform-hazard null under belief-sensitive removal.
4. **Random-turnover control ambiguity:** identical postcorrection treated/null forks mean *no causal false-prior effect* at all. Classifying such a cell as "slow relaxation" can misleadingly suggest something persisted and then faded.
5. **Limited generality:** graph starts from regular directed N=20,d=4 sampler; homogenous actor qualities, single β, κ, learning rates, degree and correction intervention. No degree heterogeneity, community structure, differential actor quality, incomplete misinformation repair, or real-world calibration.
6. **Inference vulnerabilities:** ±0.05 equivalence margin was motivated by earlier V2 results; Stage A had a frozen rule but was not externally preregistered; Stage B's amendment was made after Stage A observations. Multiple windows share parents. All post hoc paired comparisons in this review must be labeled exploratory; the absence of positive NPS intervals cannot establish global absence of regeneration.
7. **Independent implementation versus data custody:** Rust/Hiyal replay checks are software reproducibility, but a different implementation of the structural causal model or competing rules is needed for model-robustness. The script here checks only derived input tables, not the original raw event journals.

**Verdict:** GO for **external human theory/methods critique and targeted empirical-variable feasibility**, NO-GO for stronger discovery, publication or human causal claims; no need for a larger seed grid yet. A serious reviewer may favor an **interpretable negative-result benchmark and an explicitly limited turnover-compensation hypothesis** rather than asserting new sociological law.

## Independent reviewers to invite (no contact initiated)

1. **Milena Tsvetkova**, Associate Professor of Computational Social Science, LSE ([faculty page](https://www.lse.ac.uk/people/milena-tsvetkova), publicly listed `m.tsvetkova@lse.ac.uk`): strong computational-methods and network-inequality expertise; author of the 2021 closest comparator. Ask for a genuine novelty challenge, not endorsement.
2. **David Melamed**, Professor of Sociology, Ohio State ([faculty page](https://sociology.osu.edu/people/melamed.9), `melamed.9@osu.edu`): dynamic network experiments, reputation and group processes; can assess whether dyad-level empirical data might lawfully be available (see [Simpson, Montgomery & Melamed 2023](https://doi.org/10.1038/s41467-023-43486-7)).
3. **Brent Simpson**, Professor of Sociology, University of South Carolina ([faculty page](https://www.sc.edu/study/colleges_schools/artsandsciences/sociology/our_people/faculty_staff_directory/simpson_brent.php), `BTS@mailbox.sc.edu`): networks, cooperation and experimental designs. **He coauthors with Melamed; two coauthors do not replace independent methodological perspectives from separate teams.**

Confirm willingness/conflicts of interest and *permission to publish feedback*. Favor at least one quantitative network methodologist not involved with any of the cited projects. Do not count automated desk reviews as independent academic verification.

### Ready-to-send invitation text (draft only)

**Subject:** Request for critical scientific review — exploratory reputation/network simulation (negative result)

Dear Professor,

I am developing an open-source computational sociology project investigating what remains after an arbitrary reputational advantage is corrected: historical opportunity ties, or ongoing feedback that creates new unequal opportunities.

Our 432-independent-parent, nine-condition synthetic study currently favors structural persistence over post-correction active regeneration. A separate, explicitly post hoc paired-control analysis points to possible compensation under fast selective deletion. We regard these as *exploratory model-specific findings*, not preregistered or validated effects in human populations.

I am seeking a genuinely independent and skeptical assessment, especially whether the question adds anything beyond established work on arbitrary reputational inequality, self-correcting social influence, cooperation networks, and durable information-induced rewiring. I would be grateful if you could examine our short protocol and critical-review packet, or suggest a suitably independent methodological reviewer.

Repository: https://github.com/davletovb/social-emergence-lab

Critical review packet: https://github.com/davletovb/social-emergence-lab/blob/main/docs/experiment-002-v3-independent-review-packet.md

There is no expectation of a positive recommendation; identifying reasons to reject the mechanism or novelty claim is an equally valuable result. Please let me know if an appropriate scope for an initial written critique is feasible.

Best regards,

[Researcher name]

**No invitation has been sent.** External review only counts after consent, substantive human feedback, a recorded date, and explicit treatment of conflicts and limitations.
