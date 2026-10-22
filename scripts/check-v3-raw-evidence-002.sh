#!/usr/bin/env bash
# Checks the committed V3 raw-evidence bundle record against the committed
# Stage A and Stage B results; no simulation. With the path to an unpacked
# bundle, also checks that copy's archives, manifest and README against the
# record. Extract each archive and run verify-v3-grid-002.sh for full checks.
set -euo pipefail
usage() {
  echo "usage: bash scripts/check-v3-raw-evidence-002.sh [/absolute/path/to/unpacked-bundle]" >&2
  exit 2
}
fail() { echo "$1" >&2; exit 2; }
[[ $# -eq 0 || ( $# -eq 1 && "$1" == /* ) ]] || usage
copy="${1:-}"
cd "$(dirname "$0")/.."
results=experiments/002-reputation-lock-in/results
record="$results/v3-raw-evidence-20261022/bundle"
manifest="$record/MANIFEST.json"
# The record's own files are listed in the bundle's SHA256SUMS.
(cd "$record" && grep -E '  (MANIFEST\.json|README\.md)$' SHA256SUMS | sha256sum --check --strict --quiet) ||
  fail "bundle/ files differ from the bundle's SHA256SUMS"
[[ "$(jq -r '[.archives[] | "\(.stage):\(.cell)"] | join(" ")' "$manifest")" == "A:2 A:5 A:8 B:0 B:1 B:3 B:4 B:6 B:7" ]] ||
  fail "Manifest does not list exactly the nine V3 cells"
while read -r file stage cell archive_sha seal_sha; do
  dir="$results/v3-stage-${stage,,}-20261022/cell-$cell"
  [[ "$(sha256sum "$dir/SHA256SUMS" | cut -d' ' -f1)" == "$seal_sha" ]] ||
    fail "Manifest seal digest for $file differs from the committed SHA256SUMS"
  jq -e --slurpfile p "$dir/batch-plan.json" --arg file "$file" \
     --argjson files "$(wc -l < "$dir/SHA256SUMS")" '.archives[] | select(.file == $file)
    | . as $a | $p[0] as $p
    | $a.seeds == [$p.design.seeds[0], $p.design.seeds[-1]] and $a.parents == $p.design.parents
      and $a.mechanism == $p.design.mechanism and $a.rewire_every == $p.design.rewire_every
      and $a.source_revision == $p.source_revision and $a.hiyal_revision == $p.hiyal_revision
      and $a.sealed_files == $files' "$manifest" > /dev/null ||
    fail "Manifest entry for $file differs from the committed plan or seal"
  grep -qx "$archive_sha  $file" "$record/SHA256SUMS" || fail "Archive hash for $file differs from SHA256SUMS"
done < <(jq -r '.archives[] | [.file, .stage, .cell, .archive_sha256, .sha256sums_sha256] | @tsv' "$manifest")
if [[ -n "$copy" ]]; then
  for f in MANIFEST.json README.md SHA256SUMS; do
    cmp -s "$copy/$f" "$record/$f" || fail "$copy/$f differs from the committed record"
  done
  (cd "$copy" && sha256sum --check --strict --quiet SHA256SUMS) || fail "Bundle copy fails its SHA256SUMS"
  echo "Bundle copy at $copy matches the record; extract each archive and run verify-v3-grid-002.sh"
fi
echo "V3 raw-evidence record agrees with the committed Stage A and Stage B results"
