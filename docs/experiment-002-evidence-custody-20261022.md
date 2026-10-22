# Experiment 002 — evidence custody and recovery status (2026-10-22)

**This is a provenance record, not a statement that all research archives have permanent custody.** See [V0 issue #12](https://github.com/davletovb/social-emergence-lab/issues/12), [V1 #14](https://github.com/davletovb/social-emergence-lab/issues/14), [V2 #17](https://github.com/davletovb/social-emergence-lab/issues/17), [V3 #21](https://github.com/davletovb/social-emergence-lab/issues/21) and [Experiment 001 #6](https://github.com/davletovb/social-emergence-lab/issues/6).

## V0 — original GitHub Actions ZIP archived, replay-verified and restored

- Upstream action artifact: workflow **39602086513**, artifact **12347080662**, exact original filename **`experiment-002-v0-dev-batch-original.zip`**.
- Archive size: **862,502 bytes**, outer SHA-256 **`ec8b1b15cbfaec6f3263025421f31ca3de423c533da36392f37aa20323912fab`**.
- Internal `SHA256SUMS` SHA-256 **`80666d6f9329b9e332078d57cdd7fddfea5619948a759e9f5d16c5a6fe12d8ec`**.
- Verified **398/398** listed evidence files. The original ZIP contains **519 file members**: 398 sealed evidence files, one `SHA256SUMS`, and **120** zero-byte writer-lock markers expressly excluded from the seal. Exact file inventory excluding those markers matches the signed list.
- Original lab source **`bc870fe5ca75ad627421838b24fb1375eceaffdb`**, Hiyal **`4bb53aca5cffa6bc01b7558efbd690af7dbed79f`**.
- A second copy plus a machine-readable provenance JSON were saved in the owner's **private ChatGPT Library** under `/Sociology Research/Evidence/Experiment 002/V0/`. That Library copy was separately retrieved and found **byte-for-byte identical** to the original archive, with the same SHA-256.
- **Original GitHub Actions artifact expires 2026-11-04 12:46:41 UTC.** On 2026-10-22, the original unchanged ZIP was published as a public [GitHub Release asset](https://github.com/davletovb/social-emergence-lab/releases/download/experiment-002-v0-raw-evidence-20261022/experiment-002-v0-dev-batch-original.zip), under the owner-controlled repository. The GitHub-reported release size and SHA-256 match the original archive exactly. Release assets are retained while the release and repository exist and remain accessible under GitHub's service terms; **they are not DOI-backed, externally guaranteed permanent archival storage**. A byte-identical private Library copy provides an additional backup.

### Completed original V0 replay and restored-asset verification

A dedicated [GitHub Actions recovery job, run 39794492499](https://github.com/davletovb/social-emergence-lab/actions/runs/39794492499) succeeded on **2026-10-22** using Rust **1.89.0**. It re-downloaded the *original* Actions ZIP from artifact `12347080662`, verified the outer digest before extracting it, then checked the internal `SHA256SUMS` digest and executed from clean `main`:

```bash
bash scripts/verify-002.sh /ABSOLUTE/EXTRACTED-V0 80666d6f9329b9e332078d57cdd7fddfea5619948a759e9f5d16c5a6fe12d8ec
```

Its logs confirm **12/12** runs passed setup and parent/fork Hiyal replay, ancestry, intervention attribution, per-arm reports, batch summary, SHA coverage and expected source/Hiyal revisions. **No V0 simulation was rerun** for preservation. After verification, it created the release `experiment-002-v0-raw-evidence-20261022`, then independently fetched that published asset and passed the same SHA-256 plus a byte-for-byte `cmp` with the downloaded original ZIP.

- [Public release](https://github.com/davletovb/social-emergence-lab/releases/tag/experiment-002-v0-raw-evidence-20261022)
- [Published original ZIP](https://github.com/davletovb/social-emergence-lab/releases/download/experiment-002-v0-raw-evidence-20261022/experiment-002-v0-dev-batch-original.zip)
- Archive retention: owned repository's GitHub Release without the original Actions artifact's fixed 90-day expiration; owner/GitHub may delete or discontinue it. Private Library retains a separate confirmed copy, but neither provides a DOI or an independent archival institution's retention guarantee.

**V0 preservation gate:** complete at the repository-retention level, but a Zenodo/institutional DOI deposit remains desirable for long-term scholarly archiving. The unrelated V1, V2, V3 and Experiment 001 archive gates are still open.

## V3 — raw research evidence still missing durable custody

The [merged PR #20](https://github.com/davletovb/social-emergence-lab/pull/20) commits per-parent derived summaries, frozen/amended decisions and `SHA256SUMS` for **nine cells** (432 independent parent seeds). **It does not commit ~9.5 GB of original Hiyal event and fork journals**:

- Stage A full-feedback cells **2, 5, 8**: source `3b6ddb43bc451f7b708fca6202eb2b087675e7fe`; approximately **3.2 GB**.
- Stage B other cells **0, 1, 3, 4, 6, 7**: source `defb96ee73f4fe8a5258fc7178166e7bc7961b49`; approximately **6.3 GB**.
- Both stages pinned Hiyal `e38419f857afbb53428284a312b0eb926a2320e2`.
- One source-pinned Stage A cell 2 was reported as regenerated byte-identically, but the full nine-cell raw archive is not preserved.

**Do not silently call regenerated journals “the originals.”** Unless the ephemeral originals are still physically accessible, regenerate every cell on its exact source revision and label it a `regenerated-verified-copy`. Use `scripts/v3-grid-002.sh /NEW/ABSOLUTE/DIR <cell> --jobs 4`, then `scripts/verify-v3-grid-002.sh`. Compare the newly generated `SHA256SUMS` byte-for-byte to the committed manifest for the corresponding stage/cell; on any discrepancy stop and log it. Store each original or regenerated cell in a separate checksummed archive with source revision, manifest hash, storage object version and independent restore check.

**The nine-cell grid must not be rerun casually or in routine CI.** Only run deliberate per-cell archive regeneration after confirming approximately 10 GB free output plus working space and choosing durable research storage. A DOI/archive deposit may require a separate retention and publication review.

## Scientific decision

Reproducibility is not empirical validity. V3 finds no positive active-regeneration criterion in the studied synthetic setting; fast belief-dependent deletion shows compensation. The remaining next gate is **independent human methods review and empirical construct validation**, tracked in [#22](https://github.com/davletovb/social-emergence-lab/issues/22). See the [variable-level audit](experiment-002-empirical-variable-audit-20261022.md).
