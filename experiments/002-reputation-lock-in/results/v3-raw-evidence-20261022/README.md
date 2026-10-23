# Experiment 002 V3 — raw evidence custody record

This records the packed copy of the raw Hiyal evidence behind the [Stage A](../v3-stage-a-20261022/README.md) and [Stage B](../v3-stage-b-20261022/README.md) results ([Issue #21](https://github.com/davletovb/social-emergence-lab/issues/21)).

**Status:**
- packed and checked on 2026-10-22, and downloaded by the repository owner;
- replayed in full outside the producing environment, then published as the [GitHub Release `experiment-002-v3-raw-evidence-20261022`](https://github.com/davletovb/social-emergence-lab/releases/tag/experiment-002-v3-raw-evidence-20261022) on 2026-10-23 and read back;
- retained at the repository level, like V0: no DOI yet.

`bundle/` holds byte-identical copies of the bundle's `MANIFEST.json`, `README.md` and `SHA256SUMS`, so a stored copy can be checked against this repository. The `bundle/README.md` describes the bundle and how to verify it.

## What was packed

- **Original run directories, not regenerated.** All nine cell batches, exactly as `scripts/v3-grid-002.sh` wrote and sealed them, in the review container that produced them:
  - Stage A, from source `3b6ddb4`, sealed 06:05–06:38 UTC;
  - Stage B, from source `defb96e`, sealed 07:51–09:22 UTC.
  
  Both on 2026-10-22, with Hiyal `e38419f`.
- **The one regenerated copy is not in the bundle.** Stage A cell 2 was regenerated as a reproducibility check, and that copy was deleted after its `SHA256SUMS` matched.
- **One archive per cell**, packed 09:39–09:47 UTC on Linux x86-64 with GNU tar 1.35 and xz 5.4.5, by:

  ```bash
  tar --sort=name --format=pax --owner=0 --group=0 --numeric-owner \
      --mode='u+rwX,go+rX,go-w' --mtime='2026-10-22 00:00:00Z' \
      --pax-option='exthdr.name=%d/PaxHeaders/%f,delete=atime,delete=ctime' \
      --transform="s,^v3-grid-cell-<n>,experiment-002-v3-stage-<a|b>-cell-<n>," \
      -C <stage dir> -cf - v3-grid-cell-<n> | xz -6 -T4 > experiment-002-v3-stage-<a|b>-cell-<n>.tar.xz
  ```

- **One bundle file.** The nine archives, `MANIFEST.json`, `README.md` and `SHA256SUMS` were combined into `experiment-002-v3-raw-evidence-20261022.tar`, made the same way without compression at 09:55:16 UTC:
  - 223,897,600 bytes;
  - SHA-256 `8b6ba3a1c8eeab84b25dff1e90faff82e362484592fb74974db1e80a139c1812`.
  
  It was sent to and downloaded by the repository owner on 2026-10-22.

| Stage | Cell | Mechanism | Archive SHA-256 | Cell `SHA256SUMS` SHA-256 |
| --- | ---: | --- | --- | --- |
| A | 2 | full-feedback, m=2 | `3cd3d85ebe5f258186e24dcf073c6d5dd0849b644b6e515eb429d09aeaa5e49d` | `d0037ac3e583b9597969ac7e96d5790d1897eb749f29d1180528d1fca86144d0` |
| A | 5 | full-feedback, m=10 | `3dfc34aef8ab678c014a6cafd702562720da70a8aa379e865366217ed83b7049` | `979dd9c1b96b559cf6efa14d67682b6a374710ce1058a07eeb461d4b6e106366` |
| A | 8 | full-feedback, m=20 | `1fce8c5b30e239392d84b4f1a2afd10f11005ab204f4d383365dd0fd04f22bba` | `29dcc642e29f945ceb43513da953185c50669e6131d1b5904b3bcac2391ccf25` |
| B | 0 | random-turnover, m=2 | `5f34ee3c33269dea52715e55ae8d39a625d3e84bd1211196f69092826c1f2f89` | `75d1eee90e6e88651580c0f4cf9e05ecf0619bff647534fd24cde7f7651e5cc0` |
| B | 1 | belief-deletion, m=2 | `80595fec203193933766c2cc0618933ae7a90f21f3bbd2b0b8942ab89f7d8fe6` | `47ec0e8c9510aa78a444b9a1240755383ad81a45cf23a651d1fe707bf1ace847` |
| B | 3 | random-turnover, m=10 | `8eda7884e92cdc3d8a5deda64f84de8d6ad187a246e19a9f07e1ef0de4f992f7` | `90e1b06b6b3a421487ed3dca3973c7d50261e23a8af3f3d553d8bc0d5a18aea6` |
| B | 4 | belief-deletion, m=10 | `56cbed6f09d7887f02a0c6f46c4817fb1f27b73ebc9fbd26b38258fe38170917` | `498803a38dc8a82c6a3086409940b636834291f2b38f850d1e7bf68bcacb453f` |
| B | 6 | random-turnover, m=20 | `539575942f7a7341b2987a2b1cf44e817f0762070b353a4b5359a8f4fb25a06b` | `4990fccad7ace0b49ed2f9d3da15b24bfdcd47ee6c988f54355ddf13cb88709b` |
| B | 7 | belief-deletion, m=20 | `18146ee808fae1e5311b429145e8972d38bee424e4f7820f7aa7384df171be98` | `e620eeee6927dc9fc95477ffac7efdfd63a37fbaf0ad111bd24b4d6ac77be668` |

Each cell's `SHA256SUMS` covers 3,266 files and is byte-identical to the one committed in its stage's results.

## Checks made before delivery (in the producing container)

- **Seals:** every archive was extracted into an empty directory. All its sealed files matched, its seal covered exactly the files present, and its `SHA256SUMS` was byte-identical to the committed one.
- **Full verifier:** the extracted Stage B cell 7 passed the full `verify-v3-grid-002.sh --digest` from `main`.
- **Determinism:** re-packing a cell gave a byte-identical archive.

`scripts/check-v3-raw-evidence-002.sh` checks, in CI, that `bundle/` agrees with the committed results:
- the manifest's seal digests, seeds and revisions against each cell's committed `SHA256SUMS` and `batch-plan.json`;
- its archive hashes against `bundle/SHA256SUMS`.

Given the path to an unpacked bundle, it also checks that copy:

```bash
tar -xf experiment-002-v3-raw-evidence-20261022.tar
bash scripts/check-v3-raw-evidence-002.sh /absolute/path/experiment-002-v3-raw-evidence-20261022
```

## Content, sensitivity and terms

- **Synthetic content only:** model configurations, seeds, Hiyal event journals, snapshots, random-stream positions and derived reports from a synthetic agent model.
- **No human or personal data.**
- **No secrets:** the runs made no model or tool calls, so they hold no API keys or prompts.
- **No licence stated:** the repository has no licence file, so reuse terms are not stated. Choose them before publishing the bundle.

## Release and readback (2026-10-23)

[Actions run 39901070657](https://github.com/davletovb/social-emergence-lab/actions/runs/39901070657) ran on a GitHub-hosted runner, outside the environment that produced and packed the evidence. It received the bundle in three parts through a temporary branch, `evidence-transfer/v3-raw-20261022`, which shares no history with `main`. Then, in order:
1. **Bundle check:** it reassembled the bundle and checked its SHA-256 against the value above.
2. **Seal check:** it checked every cell's `SHA256SUMS` against the copy committed on `main` at `f16b019`.
3. **Full replay:** it extracted each of the nine cells and ran `scripts/verify-v3-grid-002.sh --digest`, from that commit, on it.
4. **Publication:** only then did it publish the release, tagged on `main` commit `f16b019b52b0a4da921fba14cce8af6c4921f431`. It holds 13 assets:
   - the bundle;
   - the nine per-cell archives as separate assets;
   - `MANIFEST.json`, `README.md` and `SHA256SUMS`.
5. **Readback:** it downloaded every published asset and checked it by SHA-256 and byte for byte against its source.

The tag points at `main`, not at the transfer branch. Once that temporary branch is deleted, the transferred parts are no longer reachable.

## Still open

- [ ] **Transfer branch:** delete `evidence-transfer/v3-raw-20261022`. It is not needed after publication.
- [ ] **Local copy:** check the owner's local copy: `sha256sum` on the bundle, then `check-v3-raw-evidence-002.sh` on the unpacked copy.
- [ ] **Long-term deposit:** a Zenodo or institutional deposit with a DOI, if long-term scholarly archiving is wanted. GitHub Release assets last only as long as the release and repository.
- [ ] **Licence:** choose and record the access and reuse terms. The release is public, but the repository states no licence.
