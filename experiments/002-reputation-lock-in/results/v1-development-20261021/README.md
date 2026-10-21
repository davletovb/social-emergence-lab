# Experiment 002 V1 — graph sampling and large-batch resource limit

**No confirmatory result.** The exact, source-pinned [GitHub Actions run 39619698742](https://github.com/davletovb/social-emergence-lab/actions/runs/39619698742) passed the Rust correctness tests, independent reference/annealed controls and four-seed N20,d4 graph benchmark, then its **15-parent 20-agent 200/600 exploratory suite was canceled** at the 30-minute workflow limit. All original Hiyal worlds for that incomplete suite were lost when its temporary runner ended, and the evidence-upload step never ran. **No complete 15-parent summary exists.** Do not infer results or substitute the missing cases.

The graph benchmark's complete [machine-readable result](graph-benchmark-n20-d4.json) records 8 successes out of 8 attempted graphs, from four fixed seeds 94000–94003 (each initial and repair) under a **200,000 attempt cap**. Accepted attempt counts: **2,955; 8,983; 708; 14,454; 15,979; 7,299; 383; 2,845** (maximum 15,979). This tiny fixed-seed run is **not a confidence bound** on the generator's population failure probability or a controlled CPU performance measurement.

The resource-bounded fallback script uses three default parent seeds and two no-false-prior parent seeds at the same N20,d4,200/600 specification. This is an engineering feasibility probe, not an independent confirmatory sample or a substitute for all the planned network controls. Other V1 controls remain defined and unit/smoke tested; future sufficiently sized studies must run those conditions independently.

**Performance limitation:** The stored world retains a growing step-level trace. Hiyal hashes and replays full world state, which may create substantial simulation overhead as the event horizon grows. Before scaling, benchmark and consider reconstructing analysis trajectories from actions/event logs instead of embedding a growing trace in every world state. Do not optimize by discarding provenance or changing source mid-study.

**Historical replay:** this aborted 15-parent attempt ran at source commit `60a3ccd4f48cab611f96325ee84045f88c2e9bbe` before the later explicit V1 serialization version gate. Do not pretend that newer code verified its unpreserved evidence.

## Five-parent resource-bounded fallback — completed

The fallback [run 39624548746](https://github.com/davletovb/social-emergence-lab/actions/runs/39624548746) completed after the original larger job was cancelled. Its code source was `8728ebde897768eed55a17c1e82e78f4a300cfe0`, Hiyal revision `e38419f857afbb53428284a312b0eb926a2320e2`. The five parents have 20 agents, 4 eligible connections, checkpoint 200, total 600, and four paired postcheckpoint forks each. Three default seeds **91000, 91001, 91002** and two no-false-prior seeds **91100, 91101** were completed. All journal and fork verifications, the internal SHA-256 manifest, and the GitHub artifact upload passed. The complete machine-readable [batch plan](batch-plan.json) and [batch summary](batch-summary.json) are committed in this directory.

The complete raw archive (not committed to Git) is Actions artifact **`experiment-002-v1-five-parent-exploratory`**, ID **12356838566**, 2,637,762 compressed bytes, [download page](https://github.com/davletovb/social-emergence-lab/actions/runs/39624548746/artifacts/12356838566), **expires 2027-01-19 15:19 UTC**. GitHub-reported outer ZIP digest: SHA-256 **`805c7a0aaf595655f1c1eaef597bfa7fa005c2626df7d5af068bb7393483e0fd`**. Internal `SHA256SUMS` digest: SHA-256 **`0f0f37bd5b2a2d8b8c42668b8528e2b1f524a8be74860c089de034ae6789be5e`**. The archive must be preserved in durable storage and reverified before expiration. The committed JSON summaries are not substitutes for raw Hiyal journals.

| Configuration | Parents | Mean late information-only gap | Mean late repair-after-truth contrast | Mean late degree exposure | Mean late selection component | Mean fraction of uninterrupted checkpoint edges |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Default, δ=0.2 | 3 | +0.2267 | +0.2933 | +0.1817 | +0.0054 | 0.6465 |
| No-false-prior, δ=0 | 2 | +0.2100 | +0.0500 | +0.1250 | +0.0312 | 0.6743 |

**Crucial interpretation:** Even in the no-false-prior control, the signed opportunity gap can be positive by chance and through endogenous network selection unrelated to the randomized false priors. These n=2–3 means, from nonidentical parent seeds, do not establish effect size, significance, equivalence or causal reputation lock-in. The large relative contribution of graph exposure is expected from the structure of the specified model and is not a newly discovered social mechanism.

**Capacity note:** Scaling this engine currently incurs substantial journal/replay cost. The incomplete original 15-parent attempt left no verified raw bundle. Avoid re-running the full batch in automatic PR checks. [V1 validation protocol](../../validation-v1.md) documents the next research and reproducibility gates.
