#!/usr/bin/env bash
# Verify an Experiment 002 batch written by pilot-002.sh (V0),
# pilot-v1-002*.sh (V1) or pilot-paired-002.sh (matched pairs): SHA256SUMS matches
# and covers exactly the files present (and, if given, has the recorded
# digest); every run or pair is re-derived from its Hiyal evidence by
# `reputation-pilot verify` or `reputation-paired-prior verify`; and the batch
# plan and summaries agree with the verified reports.
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
plan="$batch/batch-plan.json"
[[ -s "$plan" ]] || fail "Missing batch-plan.json"
# V0 and V1 plans name their revisions differently; matched false-prior
# batches declare their layout.
if jq -e '.layout == "paired-prior"' "$plan" >/dev/null; then
  layout=paired hiyal_key=hiyal_revision source_key=source_revision
  summary_file=paired-summary.json summary=scripts/paired-summary-002.jq
  allowed="SHA256SUMS batch-plan.json paired-summary.json"
elif jq -e 'has("hiyal_commit")' "$plan" >/dev/null; then
  layout=runs hiyal_key=hiyal_commit source_key=source_commit
  summary_file=batch-summary.json summary=scripts/batch-summary-002.jq
  allowed="SHA256SUMS batch-plan.json batch-summary.json arm-summary.json"
elif jq -e 'has("hiyal_revision")' "$plan" >/dev/null; then
  layout=runs hiyal_key=hiyal_revision source_key=source_revision
  summary_file=batch-summary.json summary=scripts/batch-summary-v1-002.jq
  allowed="SHA256SUMS batch-plan.json batch-summary.json arm-summary.json"
else
  fail "Unrecognized batch plan"
fi
for entry in "$batch"/* "$batch"/.[!.]*; do
  [[ -d "$entry" ]] && continue
  [[ " $allowed " == *" $(basename "$entry") "* ]] || fail "Unexpected file: $entry"
done
runs=("$batch"/*/)
(( ${#runs[@]} > 0 )) || fail "No run directories"
[[ -s "$batch/$summary_file" ]] || fail "Missing $summary_file"

cd "$repo"
cargo build --quiet --locked --release --bin reputation-pilot --bin reputation-paired-prior
target="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)/release"
if [[ $layout == paired ]]; then
  for run in "${runs[@]}"; do
    for entry in "$run"* "$run".[!.]*; do
      case "$(basename "$entry")" in
        false-prior | no-false-prior | paired-report.json) ;;
        *) fail "Unexpected entry in pair: $entry" ;;
      esac
    done
    "$target/reputation-paired-prior" verify "${run%/}" > /dev/null || fail "Pair failed verification: $run"
  done
  reports=("$batch"/*/paired-report.json)
else
  for run in "${runs[@]}"; do
    "$target/reputation-pilot" verify "${run%/}" > /dev/null || fail "Run failed verification: $run"
  done
  reports=("$batch"/*/report.json)
fi

(( ${#reports[@]} == ${#runs[@]} )) || fail "A run directory has no report"
[[ "$(jq -r .seed "${reports[@]}" | sort | uniq -d)" == "" ]] || fail "Runs reuse a parent seed"
if jq -e 'has("seeds")' "$plan" >/dev/null; then
  [[ "$(jq -c '.seeds | sort' "$plan")" == "$(jq -s -c 'map(.seed) | sort' "${reports[@]}")" ]] ||
    fail "Runs are not exactly the planned seeds"
fi
hiyal="$(jq -r .hiyal_revision "${reports[@]}" | sort -u)"
jq -e --arg key "$hiyal_key" --arg hiyal "$hiyal" '.[$key] == $hiyal' "$plan" >/dev/null ||
  fail "Batch plan names a different Hiyal revision than its runs"
# Compare values, not jq's number spelling: jq versions differ on whether an
# unchanged 0.0 prints as 0.0 or 0.
canonical='walk(if type == "number" then . + 0 else . end)'
same() { cmp -s <(jq -cS "$canonical" "$1") <(jq -s -f "$2" "${reports[@]}" | jq -cS "$canonical"); }
same "$batch/$summary_file" "$summary" || fail "$summary_file disagrees with the runs"
if [[ -e "$batch/arm-summary.json" ]]; then
  same "$batch/arm-summary.json" scripts/arm-summary-002.jq || fail "arm-summary.json disagrees with the runs"
fi
what=$([[ $layout == paired ]] && echo "matched pairs" || echo runs)
echo "Verified ${#runs[@]} $what: seal, Hiyal setup/replay/fork ancestry, interventions, reports and summaries"
echo "Source commit $(jq -r --arg key "$source_key" '.[$key]' "$plan"), Hiyal $hiyal"
