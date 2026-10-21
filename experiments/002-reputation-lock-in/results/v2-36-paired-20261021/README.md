# Experiment 002 V2 — 36 independently verified parent-seed pairs

**Scientific status: exploratory synthetic model development; not preregistered, not empirical sociology, and not evidence of an original law.**

A completely predeclared independent seed domain **98000–98035** was run on three disjoint twelve-pair Hiyal shards ([original workflow run 39683450918](https://github.com/davletovb/social-emergence-lab/actions/runs/39683450918)). Each shard passed deterministic parent/fork replay, independent reference checks, report derivation, exact seed inventory, and internal SHA-256 evidence manifest verification. The original workflow's **aggregation-only** step failed because it wrote jq `sqrt(x)` rather than `x|sqrt`; **all simulation shards succeeded**. An [independent cheap aggregation-only run 39684883492](https://github.com/davletovb/social-emergence-lab/actions/runs/39684883492) downloaded those *exact existing signed artifacts* and recomputed the 36-pair summary without rerunning or selecting any seeds. Its checksum and summary verification passed; it uploaded its own sealed derived-summary artifact.

The [per-pair detailed dataset](paired-summary-with-uncertainty.json) is preserved in this repository alongside [separate aggregate statistics](paired-summary.json), [original archive metadata](artifacts.json), and [measured V1/V2 benchmark](performance-benchmark.json). **These derived files cannot substitute for the original Hiyal event journals.**

## Descriptive results (N=20,d=4; checkpoint 200, horizon 600)

| Late signed opportunity-gap estimand | Independent seeds | Mean | Sample SD | Approximate unadjusted 95% t interval |
| --- | ---: | ---: | ---: | --- |
| δ=0.2 initial false-prior information-only arm | 36 | +0.21833 | 0.14808 | +0.16823 to +0.26844 |
| δ=0 initial false-prior null information-only arm | 36 | +0.02778 | 0.15655 | −0.02519 to +0.08075 |
| Same-seed δ=0.2 minus δ=0 | 36 | +0.19056 | 0.16764 | +0.13383 to +0.24728 |

Signed opportunity gap is the share of new opportunities accruing to randomized boosted labels minus that to unboosted labels, measured in each fork's **last 100 postintervention events**. Interval calculation: `mean ± t[35,0.975] × sd/√36` with t≈2.030108, assuming independent parent seeds. These are model-conditional exploratory intervals with no preregistered claims, multiple-testing correction, participant population or construct-validation evidence. Within-model δ=0 is an exact zero **in expectation** by symmetry, not necessarily in individual runs. Source revision: `057dbec5fc79c98dc745556c74410776b0bc036e`; Hiyal pinned to `e38419f857afbb53428284a312b0eb926a2320e2`.

The δ=0.2 and δ=0 gaps had sample correlation **0.3954**; the paired difference SD **0.16764** was **larger** than the δ=0.2-only SD **0.14808**. Pairing does not improve precision here. The positive model result follows from a system deliberately capable of false-prior feedback; it cannot establish substantive sociological novelty.

**A useful model warning:** the paired `repair_after_truth` difference duplicates the paired information-only gap difference for every seed to floating-point tolerance (confirmed by all 36 source-derived records). Both truth+network repair forks converge to the same signed late result under this model's reset, repair graph, and RNG coupling. **Do not count this as an independent replication or separate mechanism discovery.**

## Archival and reproducibility

The three original independently replayed archives, plus the separately recomputed aggregate, are named and SHA-256-identified in [artifacts.json](artifacts.json). The aggregate internal manifest's SHA256SUMS hash is `4e3c1bcf16a59cc242b6cdc64998f3de39423d40f056a4565363028002628c81`. Original artifacts are GitHub Actions archives retained until **January 19, 2027**, not durable research storage. Download the three original ZIPs and separately re-verify each `SHA256SUMS` before their expiration. Then run:

```bash
bash scripts/aggregate-paired-002.sh /absolute/directory/with/three/extracted-shard-folders /absolute/new-output-folder
```

This recomputes per-seed means and uncertainty from the original **checksum-covered** report files. For full verification, use `scripts/verify-002.sh` on *each shard* with its exact source revision and Hiyal dependency; unlike aggregation, that performs independent Hiyal parent+fork journal replay. A user-controlled durable archive is still outstanding.

## Performance finding

On the same GitHub runner with separately built release binaries, V1 vs V2 run time was **609 vs 620 ms** for N10,d3,40/120 and **2624 vs 2630 ms** for N20,d4,80/240. Parent + all four fork event journals and reports were **byte-identical** between versions; verification timing and evidence bytes are in [performance-benchmark.json](performance-benchmark.json). These single-run measurements show **no meaningful speed benefit** from the compact policy observation. See [Hiyal issue #117](https://github.com/davletovb/hiyal/issues/117) for per-event canonical serialization/hashing performance hardening that does not weaken historical replay.

## Next scientific gate

Independently critique whether the observed positive effect is nontrivial beyond mechanisms already explicitly programmed into the model. Expand controlled graph-family/turnover/learning sensitivity and potential empirical calibration before a held-out protocol. Do not recycle exploratory seeds for subsequent confirmation.
