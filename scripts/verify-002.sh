#!/usr/bin/env bash
# Verify an Experiment 002 batch written by pilot-002.sh (V0) or
# pilot-v1-002*.sh (V1): SHA256SUMS matches
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
for entry in "$batch"/* "$batch"/.[!.]*; do
  [[ -d "$entry" ]] && continue
  case "$(basename "$entry")" in
    SHA256SUMS | batch-plan.json | batch-summary.json | arm-summary.json) ;;
    *) fail "Unexpected file: $entry" ;;
  esac
done
runs=("$batch"/*/)
(( ${#runs[@]} > 0 )) || fail "No run directories"
for f in batch-plan.json batch-summary.json; do [[ -s "$batch/$f" ]] || fail "Missing $f"; done
# The V0 and V1 batch plans name their revisions differently.
plan="$batch/batch-plan.json"
if jq -e 'has("hiyal_commit")' "$plan" >/dev/null; then
  hiyal_key=hiyal_commit source_key=source_commit summary=scripts/batch-summary-002.jq
elif jq -e 'has("hiyal_revision")' "$plan" >/dev/null; then
  hiyal_key=hiyal_revision source_key=source_revision summary=scripts/batch-summary-v1-002.jq
else
  fail "Unrecognized batch plan"
fi

cd "$repo"
cargo build --quiet --locked --release --bin reputation-pilot
target="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)"
for run in "${runs[@]}"; do
  "$target/release/reputation-pilot" verify "${run%/}" > /dev/null || fail "Run failed verification: $run"
done

reports=("$batch"/*/report.json)
(( ${#reports[@]} == ${#runs[@]} )) || fail "A run directory has no report.json"
[[ "$(jq -r .seed "${reports[@]}" | sort | uniq -d)" == "" ]] || fail "Runs reuse a parent seed"
hiyal="$(jq -r .hiyal_revision "${reports[@]}" | sort -u)"
jq -e --arg key "$hiyal_key" --arg hiyal "$hiyal" '.[$key] == $hiyal' "$plan" >/dev/null ||
  fail "Batch plan names a different Hiyal revision than its runs"
same() { cmp -s <(jq -cS . "$1") <(jq -s -f "$2" "${reports[@]}" | jq -cS .); }
same "$batch/batch-summary.json" "$summary" || fail "batch-summary.json disagrees with the runs"
if [[ -e "$batch/arm-summary.json" ]]; then
  same "$batch/arm-summary.json" scripts/arm-summary-002.jq || fail "arm-summary.json disagrees with the runs"
fi
echo "Verified ${#runs[@]} runs: seal, Hiyal setup/replay/fork ancestry, interventions, reports and summaries"
echo "Source commit $(jq -r --arg key "$source_key" '.[$key]' "$plan"), Hiyal $hiyal"
