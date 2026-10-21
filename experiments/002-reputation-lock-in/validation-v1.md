# Experiment 002 — V1 mechanism validation and preregistration boundary

**Status:** exploratory model-validation milestone, **NOT** a preregistered or externally validated sociological result.

## What this stage tests

The scientific problem is not whether early advantages can self-reinforce (established) but whether *after truthful correction* the remaining advantage can be distinguished between (a) checkpoint opportunity links that are still uninterrupted, (b) post-checkpoint or re-added opportunity links, and (c) reputation-dependent choice **conditional on eligibility**. At every preselection state the recorded signed expected advantage is

```text
expected = Σ_i b_i P(recipient=i)
degree   = Σ_i b_i indegree_i /(N*d)
selection= expected - degree
degree   = continuously_surviving_checkpoint_exposure + new_or_readded_exposure
observed = expected + innovation
```

The implementation computes conditional probabilities by exponential weights plus uniform exploration. The **independent reference module** independently sums recipient-specific probabilities by allocator and derives indegree, inherited/new-edge, and selection contributions. Equality of independent calculations is an *engineering invariant*, not empirical corroboration. `reputation-pilot verify` (and so `scripts/verify-002.sh`) replays every recorded event of every parent and fork through this reference, checking each pre-event state and the action's recorded expectation, and requires the replay to reach Hiyal's verified final world.

## Versioning and historical evidence

V0 observations included the model's optional `annealed` field only when active and did not write uninterrupted checkpoint-edge counts to each journaled trace. V1 **explicitly opts in** through `survival_accounting=true`; legacy Config JSON lacking that flag deserializes to `false` and is replayed without inventing previously absent trace fields. Otherwise historical Hiyal hash-chain replay would be invalid. The V1 diagnostic batch completed before the backward-compatibility repair, if any, must be replayed with its **exact pinned source commit**, and results must never be silently rewritten.

V1's Hiyal world-kind identifier is retained for V0 compatibility, but `source_revision`, exact Cargo.lock and the persisted `Config` (including control flags) are the actual study-model identity. Avoid comparing results produced by a different code revision as though all parameter controls were identical.

## Controls and comparability

| Scenario | Opportunity graph | Rewiring | Limitation |
| --- | --- | --- | --- |
| default | Directed simple regular at genesis, then endogenous edges | Remove weakest private belief, add proportional to public reputation | Hypothesized coupling |
| no-false-prior | Same construction, initially no boosted belief | Endogenous | Random labels remain, but no randomized reputational advantage |
| random-churn | Same | Uniform deletion and uniform replacement | Separates simple slow turnover from belief-driven tie choice |
| annealed-mixing | Fresh *permuted-circulant* regular graph **every event**, from `agent/<j>/annealed` | No persistent-edge rewiring | D-matched choice set, but **different graph-family distribution** from the default configuration; not a perfect counterfactual. Every in-degree is d, so the degree-exposure term is **exactly 0** by construction |
| full-mixing | All N−1 other recipients visible | None | Choice set much larger (N−1 versus d), so not degree matched |
| rewire-every-2 | Same as default | 5× faster nominal schedule | Changes expected rewires, not just beliefs |
| rewire-every-20 | Same as default | 2× slower nominal schedule | May preserve initial ties substantially longer |

The formal model's D5 baseline draws only the scheduled allocator's d candidates from `agent/<j>/mixing` and is not globally regular. Here the allocator's row of a randomly permuted circulant graph is still a uniformly random set of d others, so each event's choice distribution matches D5; the difference is the stream name and that the whole graph is regular.

Do not call the annealed or complete-graph condition a fully controlled intervention against a degree-matched default; compare trends and mechanisms, not causal effects across different baseline graphs. The four intervention forks **within** a parent are the paired model interventions. The independent statistical unit remains the parent society.

## Graph-sampling benchmark

`reputation-graph-benchmark` repeats two separately streamed, uniform configuration-model pairings per fixed seed. It records attempts until the first simple directed d-regular graph (reject self-loops and duplicate incoming/outgoing pairs), wall-clock diagnostic microseconds and every failure at a hard 200,000-attempt budget. Conditional on acceptance, it samples from the simple directed regular family. Each attempt is an independent uniform stub pairing, so the attempt counts do bound the failure rate: the benchmark reports total attempts, a one-sided 95% lower bound on the per-attempt acceptance probability (Poisson approximation) and the implied upper bound on one graph exhausting the cap. For the committed N=20, d=4 run (8 graphs, 53,606 attempts) that bound is about 3.5×10⁻⁷ per graph. It does not benchmark CI runtime or show the generator resembles real social networks.

## Predeclared larger development batch

Script: `scripts/pilot-v1-002.sh`. N=20 agents, d=4 default links, shared intervention checkpoint τ=200, accepted opportunity horizon T=600. **Only 15 parents:** three default seeds 91000–91002, and two each for no-false-prior (91100–91101), random churn (91200–91201), annealed mixing (91300–91301), full mixing (91400–91401), rewiring every two steps (91500–91501), and every twenty steps (91600–91601). All use the same four fork interventions, 400 post-fork opportunities and a final 100-step descriptive window. Every parent trial must pass replay and fork verification before it is included; any failed seed must be disclosed rather than replaced.

Run in release mode, the full plan takes about 7 minutes (one N=20 parent: 19.5 s to run and 8 s to verify, against 207 s and 99 s for the debug build the first attempt used, with byte-identical output). The cancelled run's log shows only trial-start lines, so no outcome was seen before the fallback was chosen.

These group sizes are **far too small for reliable variance estimates**, confidence intervals, equivalence tests, power calculations or a meaningful scientific claim. There is no held-out confirmatory sample or preregistration. The batch is an engineering and direction-finding probe.

## Observed engineering outcome — five-parent fallback

The original 15-parent grid did **not** finish and yielded no complete evidence bundle. Its workflow was cancelled at the 30-minute CI limit. The fixed five-parent fallback **did finish**: [Actions run 39624548746](https://github.com/davletovb/social-emergence-lab/actions/runs/39624548746) verified every Hiyal parent/fork, checked the evidence seal and uploaded the original raw bundle (artifact `12356838566`, expires **2027-01-19**). Results and exact hashes are in [V1 development results](results/v1-development-20261021/README.md).

| N=20,d=4, τ=200,T=600 | Independent parents | Late signed information-only opportunity gap | Incremental repair-after-truth contrast |
| --- | ---: | ---: | ---: |
| Default false-prior effect δ=0.2 | 3 | +0.2267 | +0.2933 |
| No-false-prior δ=0 | 2 | +0.2100 | +0.0500 |

The signed opportunity gap is measured in the final **100** post-checkpoint events. These means are highly imprecise. With δ=0 nothing in the model reads the labels, so the expected gap is **exactly zero** in every arm; the two no-false-prior parents' +0.21 is sampling noise, not a persisting gap. A review check (regenerable, not archived) of 16 further δ=0 parents, seeds 96000–96015 (`reputation-pilot <dir> <seed> 20 4 200 600 no-false-prior`), gave a late information-only gap of mean +0.009, sd 0.144, range −0.28 to +0.26; the late `D_I−D_IS` contrast had mean −0.001, sd 0.218. Per-parent noise of that size is as large as the default mean, so three default and two null parents cannot distinguish the configurations; nothing here shows the default gap is or is not caused by the false labels. If the default spreads similarly, a ±0.05 95% margin needs about 32 parents per configuration. `info_degree_mean`, `info_selection_mean`, `info_inherited_mean`, `info_added_mean` and `info_survival_mean` are also retained in the [machine-readable summary](results/v1-development-20261021/batch-summary.json), and per-arm means (including H2's sham and structure full-window gaps) in [arm-summary.json](results/v1-development-20261021/arm-summary.json). The separate groups have different random parent seeds; their difference is **not** a within-parent controlled treatment effect.

## Minimum gates after V1

- [ ] Independently review mathematical and programming assumptions, including uniform-stub conditioning and synthetic-world symmetry.
- [ ] Repeat controls with adequate *independent parent* sample sizes and matching/timing; predefine a target precision and inferential method **before** confirmatory seed draws.
- [ ] Benchmark acceptance distribution across many fixed seeds with a failure policy; no seed filtering.
- [ ] Recheck real network/perception construct validity against authorized datasets; no extrapolation to actual human opportunities.
- [ ] Preserve full original event/fork journals in durable, external evidence storage, with checksums and exact source revision.
- [ ] Independently evaluate novelty, including the 2026 literature and structural-exposure tautology warning.

**No large automatic PR CI**: only unit/replay/reference checks and a tiny 6-agent smoke should remain after a one-time evidence run.
