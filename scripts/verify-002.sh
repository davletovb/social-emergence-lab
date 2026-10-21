#!/usr/bin/env bash
# Verify an Experiment 002 batch written by pilot-002.sh: SHA256SUMS matches
# and covers exactly the files present (and, if given, has the recorded
# digest); every run is re-derived from its Hiyal evidence by
# `reputation-pilot verify`; and the batch plan and summaries agree with the
# verified reports.
set -euo pipefail
usage() {
  echo "Usage: bash scripts/verify-002.sh <batch-directory> [expected-SHA256SUMS-sha256]" >&2
  exit 2
}
[[ $# -eq 1 || $# -eq 2 ]] || usage
fail() { echo "$1" >&2; exit 2; }
[[ -d "$1" ]] || fail "No such evidence directory: $1"
batch="$(cd "$1" && pwd)"
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
for command in jq sha256sum cargo; do command -v "$command" >/dev/null || fail "Missing $command"; done

[[ -s "$batch/SHA256SUMS" ]] || fail "Missing SHA256SUMS"
if [[ $# -eq 2 ]]; then
  [[ "$(sha256sum "$batch/SHA256SUMS" | cut -d' ' -f1)" == "$2" ]] || fail "SHA256SUMS digest differs from $2"
fi
(cd "$batch" && sha256sum --check --strict --quiet SHA256SUMS) || fail "SHA256SUMS check failed"
listed="$(sed 's/^[0-9a-f]\{64\}  //' "$batch/SHA256SUMS" | LC_ALL=C sort)"
present="$(cd "$batch" && find . -type f ! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock | LC_ALL=C sort)"
[[ "$listed" == "$present" ]] || fail "SHA256SUMS does not cover exactly the files present"

shopt -s nullglob
runs=("$batch"/n*/)
(( ${#runs[@]} > 0 )) || fail "No run directories"
for entry in "$batch"/* "$batch"/.[!.]*; do
  case "$(basename "$entry")" in
    n*) [[ -d "$entry" ]] || fail "Unexpected file: $entry" ;;
    SHA256SUMS | batch-plan.json | batch-summary.json | arm-summary.json) ;;
    *) fail "Unexpected entry: $entry" ;;
  esac
done
for f in batch-plan.json batch-summary.json; do [[ -s "$batch/$f" ]] || fail "Missing $f"; done

cd "$repo"
cargo build --quiet --locked --bin reputation-pilot
target="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)"
for run in "${runs[@]}"; do
  "$target/debug/reputation-pilot" verify "${run%/}" > /dev/null || fail "Run failed verification: $run"
done

reports=("$batch"/n*/report.json)
[[ "$(jq -r .seed "${reports[@]}" | sort | uniq -d)" == "" ]] || fail "Runs reuse a parent seed"
hiyal="$(jq -r .hiyal_revision "${reports[@]}" | sort -u)"
jq -e --arg hiyal "$hiyal" '.hiyal_commit == $hiyal' "$batch/batch-plan.json" >/dev/null ||
  fail "Batch plan names a different Hiyal revision than its runs"
same() { cmp -s <(jq -cS . "$1") <(jq -s -f "$2" "${reports[@]}" | jq -cS .); }
same "$batch/batch-summary.json" scripts/batch-summary-002.jq || fail "batch-summary.json disagrees with the runs"
if [[ -e "$batch/arm-summary.json" ]]; then
  same "$batch/arm-summary.json" scripts/arm-summary-002.jq || fail "arm-summary.json disagrees with the runs"
fi
echo "Verified ${#runs[@]} runs: seal, Hiyal setup/replay/fork ancestry, interventions, reports and summaries"
echo "Source commit $(jq -r .source_commit "$batch/batch-plan.json"), Hiyal $hiyal"
