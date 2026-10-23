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

**V0 preservation gate:** complete at the repository-retention level, but a Zenodo/institutional DOI deposit remains desirable for long-term scholarly archiving. The unrelated V1, V2 and Experiment 001 archive gates are still open; V3's is below.

## V3 — original raw evidence packed, replay-verified and released

The [merged PR #20](https://github.com/davletovb/social-emergence-lab/pull/20) commits per-parent derived summaries, frozen/amended decisions and `SHA256SUMS` for **nine cells** (432 independent parent seeds). It does not commit the ~9.5 GB of original Hiyal event and fork journals:

- Stage A full-feedback cells **2, 5, 8**: source `3b6ddb43bc451f7b708fca6202eb2b087675e7fe`; approximately **3.2 GB**.
- Stage B other cells **0, 1, 3, 4, 6, 7**: source `defb96ee73f4fe8a5258fc7178166e7bc7961b49`; approximately **6.3 GB**.
- Both stages pinned Hiyal `e38419f857afbb53428284a312b0eb926a2320e2`.

**The journals are preserved as originals, not regenerated.**
- **Packing.** On 2026-10-22, all nine original run directories were still on disk in the review container that produced them. They were packed as one deterministic `tar.xz` per cell, inside a single bundle:
  - `experiment-002-v3-raw-evidence-20261022.tar`;
  - **223,897,600 bytes**, SHA-256 **`8b6ba3a1c8eeab84b25dff1e90faff82e362484592fb74974db1e80a139c1812`**.
- **Checks before delivery.** Every archive was extracted and checked against its seal, and each cell's `SHA256SUMS` is byte-identical to the one committed on `main`.
- **The record.** The [V3 custody record](../experiments/002-reputation-lock-in/results/v3-raw-evidence-20261022/README.md) holds every archive and seal hash, the pack command, tool versions and UTC times. The owner downloaded the bundle the same day.
- **The one regenerated copy is not in the bundle.** Stage A cell 2 was regenerated as a reproducibility check; its `SHA256SUMS` matched byte for byte, and that copy was deleted.

### Release replay and readback

- **The run.** [GitHub Actions run 39901070657](https://github.com/davletovb/social-emergence-lab/actions/runs/39901070657) ran on 2026-10-23 on a GitHub-hosted runner, outside the producing environment. It received the bundle in parts through a temporary branch shared with no other history.
- **Checks before publishing.** It reassembled the bundle and checked its SHA-256. It checked every cell's seal against the seals committed on `main`. It then replayed all nine cells with `scripts/verify-v3-grid-002.sh --digest`, re-deriving every parent's Hiyal journals, cell summaries and decision.
- **Publishing and readback.** Only after that did it publish the [public release `experiment-002-v3-raw-evidence-20261022`](https://github.com/davletovb/social-emergence-lab/releases/tag/experiment-002-v3-raw-evidence-20261022), tagged on `main` commit `f16b019`. The release holds the bundle, the nine per-cell archives as separate assets, and `MANIFEST.json`, `README.md` and `SHA256SUMS`. The job then downloaded every published asset and checked it by SHA-256 and byte for byte against its source.
- **Cleanup.** The release tag points at `main`, not at the transfer branch. Once the temporary branch `evidence-transfer/v3-raw-20261022` is deleted, the transferred parts are no longer reachable.

**Checking the owner's local copy.** `sha256sum experiment-002-v3-raw-evidence-20261022.tar` should print the digest above. After `tar -xf`, run `bash scripts/check-v3-raw-evidence-002.sh /ABSOLUTE/experiment-002-v3-raw-evidence-20261022` from a checkout that includes the custody record. It checks the copy against that record.

**V3 preservation gate:** complete at the repository-retention level, as for V0.
- The release is retained while the release and repository exist; it is not DOI-backed.
- A Zenodo or institutional deposit remains desirable.
- The repository states no licence, so reuse terms are the owner's to set.

**Do not call regenerated journals "the originals."** Regenerating a cell on its exact source revision is a reproducibility check, not custody. Label any such copy `regenerated-verified-copy`, and compare its `SHA256SUMS` byte for byte with the committed one. Make it with `scripts/v3-grid-002.sh /NEW/ABSOLUTE/DIR <cell> --jobs 4`, then check it with `scripts/verify-v3-grid-002.sh`.

**The nine-cell grid must not be rerun casually or in routine CI.**

## Scientific decision

Reproducibility is not empirical validity. V3 finds no positive active-regeneration criterion in the studied synthetic setting; fast belief-dependent deletion shows compensation. The remaining next gate is **independent human methods review and empirical construct validation**, tracked in [#22](https://github.com/davletovb/social-emergence-lab/issues/22). See the [variable-level audit](experiment-002-empirical-variable-audit-20261022.md).
