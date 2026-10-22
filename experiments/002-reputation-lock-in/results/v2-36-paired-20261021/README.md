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

## H1: lock-in or slow relaxation?

The formal model's H1 needs more than a positive late gap: a gap carried only by checkpoint links that are still present is classified as **slow relaxation**, and lock-in requires the late new-link plus selection term (`g^new + g^sel`) to be positive. From the 72 run reports ([h1-decomposition.json](h1-decomposition.json), [arm-summary.json](arm-summary.json)), the information-only fork's last-100-event gap decomposes as:

| Configuration | Late gap | Surviving checkpoint links | New links | Selection | New links + selection (95% t interval) | Checkpoint links still present |
| --- | ---: | ---: | ---: | ---: | --- | ---: |
| δ=0.2 | +0.2183 | +0.2239 | −0.0055 | +0.0071 | +0.0016 (−0.0214 to +0.0246) | 66% |
| δ=0 | +0.0278 | +0.0236 | +0.0155 | +0.0040 | +0.0196 (−0.0108 to +0.0499) | 66% |

At δ=0.2 the whole late gap is carried by links that survive from τ; the new-link plus selection term is indistinguishable from zero. **Under H1 this is slow relaxation, not lock-in**, in this model and parameter setting: about two thirds of the false-prior era's links are still in place 300–400 events after correction, and the advantage they carry has not yet decayed.

The δ=0.2 and δ=0 gaps had sample correlation **0.3954**; the paired difference SD **0.16764** was **larger** than the δ=0.2-only SD **0.14808**. Pairing does not improve precision here. The positive model result follows from a system deliberately capable of false-prior feedback; it cannot establish substantive sociological novelty.

**A useful model warning:** the paired `repair_after_truth` difference duplicates the paired information-only gap difference for every seed. This is structural, not algebraic: the truth+repair fork resets beliefs and public reputations and installs the same repair graph, so it erases every input to decisions that depends on δ, and the number of random draws per event does not depend on δ. In all 36 pairs the two truth+repair forks made identical allocation decisions from identical stream positions. Their gap is therefore noise that does not depend on δ, and `repair_after_truth` at δ=0.2 is the information-only gap minus that noise. **Do not count it as an independent replication or separate mechanism discovery.**

## Archival and reproducibility

The three original independently replayed archives, plus the separately recomputed aggregate, are named and SHA-256-identified in [artifacts.json](artifacts.json). The aggregate internal manifest's SHA256SUMS hash is `4e3c1bcf16a59cc242b6cdc64998f3de39423d40f056a4565363028002628c81`. Original artifacts are GitHub Actions archives retained until **January 19, 2027**, not durable research storage. Download the three original ZIPs and separately re-verify each `SHA256SUMS` before their expiration. Then run:

Extract each ZIP into a folder named after its artifact (`experiment-002-v2-paired-shard-0`, `-1`, `-2`) inside one parent directory, then run:

```bash
bash scripts/aggregate-paired-002.sh /absolute/directory/with/three/extracted-shard-folders /absolute/new-output-folder
```

This recomputes per-seed means and uncertainty, and now also `arm-summary.json` and `h1-decomposition.json`, from the original **checksum-covered** report files. Values compare exactly; the aggregate `SHA256SUMS` digest above is only reproducible with the same jq, because jq versions differ on whether an unchanged `0.0` prints as `0.0` or `0` (the verification scripts compare values, not this spelling). For full verification, use `scripts/verify-002.sh` on *each shard* with its exact source revision and Hiyal dependency; unlike aggregation, that performs independent Hiyal parent+fork journal replay. A user-controlled durable archive is still outstanding.

**Review reproduction (2026-10-22):** running `scripts/pilot-paired-002-shard.sh` for all three shards at `057dbec` (Linux x86-64, Rust 1.89.0) and aggregating gave every one of the 36 per-seed records and every summary statistic with identical values; each reproduced shard passed `verify-002.sh`. The committed `arm-summary.json` and `h1-decomposition.json` were computed from those reproduced reports.

## Performance finding

On the same GitHub runner with separately built release binaries, V1 vs V2 run time was **609 vs 620 ms** for N10,d3,40/120 and **2624 vs 2630 ms** for N20,d4,80/240. Parent + all four fork event journals and reports were **byte-identical** between versions; verification timing and evidence bytes are in [performance-benchmark.json](performance-benchmark.json). These single-run measurements show **no meaningful speed benefit** from the compact policy observation. See [Hiyal issue #117](https://github.com/davletovb/hiyal/issues/117) for per-event canonical serialization/hashing performance hardening that does not weaken historical replay.

## Next scientific gate

Independently critique whether the observed positive effect is nontrivial beyond mechanisms already explicitly programmed into the model. Expand controlled graph-family/turnover/learning sensitivity and potential empirical calibration before a held-out protocol. Do not recycle exploratory seeds for subsequent confirmation.
