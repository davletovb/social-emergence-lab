# Experiment 002 V3 — raw Hiyal evidence (Stages A and B, 22 October 2026)

This bundle preserves the raw evidence behind the V3 main-grid results in [davletovb/social-emergence-lab](https://github.com/davletovb/social-emergence-lab), merged to `main` in `1b59d24`:
- `experiments/002-reputation-lock-in/results/v3-stage-a-20261022/` (Stage A: cells 2, 5, 8; source `3b6ddb4`);
- `experiments/002-reputation-lock-in/results/v3-stage-b-20261022/` (Stage B: cells 0, 1, 3, 4, 6, 7; source `defb96e`).

The repository commits only derived files: each cell's plan, decision, `SHA256SUMS` seal and per-parent summaries. This bundle holds everything else: every parent's Hiyal journals, snapshots and reports.

Status: exploratory synthetic-model evidence, not human data.

## Contents

- **Nine archives**, one per sealed cell batch, each `experiment-002-v3-stage-<a|b>-cell-<n>.tar.xz`. Each unpacks to one directory, exactly as `scripts/v3-grid-002.sh` wrote it:
  - `batch-plan.json`, `decision.json` and `SHA256SUMS`;
  - one `parent-<seed>` directory for each of its 48 parents.
  
  Every archive is about 25 MB and unpacks to 1.0–1.2 GB (3,266 sealed files).
- **`MANIFEST.json`** lists, per archive: stage, cell, mechanism, rewire period, seeds, decision rule and classification, source and Hiyal revisions, archive size and SHA-256, and the SHA-256 of the cell's `SHA256SUMS`.
- **`SHA256SUMS`** covers the archives, `MANIFEST.json` and this README.

## Verify

```bash
sha256sum --check SHA256SUMS                  # the bundle itself
mkdir cells && tar -C cells -xJf experiment-002-v3-stage-b-cell-1.tar.xz
(cd cells/experiment-002-v3-stage-b-cell-1 && sha256sum --check --strict --quiet SHA256SUMS)
```

**The key link to the repository** is the cell's `SHA256SUMS`. It must be byte-identical to the committed `results/v3-stage-<a|b>-20261022/cell-<n>/SHA256SUMS`, and its SHA-256 equals both `sha256sums_sha256` in `MANIFEST.json` and the digest in that stage's README.

**Full verification** re-derives every parent's Hiyal journals, cell summaries and decision. From a checkout of `main` (Rust 1.89, jq, sha256sum):

```bash
bash scripts/verify-v3-grid-002.sh /absolute/path/cells/experiment-002-v3-stage-b-cell-1 \
  --digest <sha256sums_sha256 from MANIFEST.json> --jobs 4
```

It takes about 5 minutes per cell on 4 cores. Its file count excludes the `.writer.lock` and `.effects-writer.lock` files that Hiyal leaves in each trial directory. They are packed but not sealed.

## How this bundle was checked (2026-10-22)

- **Every archive** was extracted into an empty directory. All its sealed files matched, its seal covered exactly the files present, and its `SHA256SUMS` was byte-identical to the committed one.
- **The Stage B cell 7 archive** also passed the full `verify-v3-grid-002.sh --digest` from `main`.
- **Packing is deterministic** with the recorded tools: sorted names, fixed owner, mode and mtime, `xz -6`. Re-packing a cell gave a byte-identical archive. Other tar or xz versions may produce different archive bytes; the seals inside do not depend on them.
- **Regeneration:** Stage A cell 2 has also been regenerated from source byte for byte.

## Not included

- **Engineering smoke runs** on non-frozen seeds.
- **A partial directory from an aborted first Stage B launch**: stopped about 30 seconds in, before any parent finished, and set aside unread, as the Stage B results README describes.

Neither is evidence.
