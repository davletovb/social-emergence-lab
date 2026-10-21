#!/usr/bin/env bash
set -euo pipefail
if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "Usage: bash scripts/verify-001.sh <grid-output-directory> [--before-hash]" >&2
  exit 2
fi
output="$1"
for f in study-plan.json hiyal-summary.csv hiyal-trajectories.csv hiyal-benchmarks.csv hiyal-study-report.json reproducibility-manifest.json; do
  [[ -s "$output/$f" ]] || { echo "Missing evidence: $f" >&2; exit 2; }
done
jq -e '.schema == 2 and .pairs == 10 and (.cells | length) == 18' "$output/study-plan.json" >/dev/null
jq -e '.scientific_status == "exploratory_not_preregistered" and .paired_societies_per_cell == 10' "$output/reproducibility-manifest.json" >/dev/null
count="$(find "$output" -type f -path '*/arms/*/events.jsonl' | wc -l)"
[[ "$count" -eq 240 ]] || { echo "Expected 240 Hiyal arm journals, found $count" >&2; exit 2; }
awk -F, '
  NR==1 { if ($0 != "actors,initial,offset,horizon,pairs,control_mean_gini,treatment_mean_gini,paired_delta,bootstrap_95_lo,bootstrap_95_hi") exit 1; next }
  NF != 10 || $5 != 10 || $9 == "" || $10 == "" { exit 1 }
  {key=$1 FS $2 FS $3 FS $4; if(seen[key]++) exit 1; n++}
  END { if (n != 18) exit 1 }
' "$output/hiyal-summary.csv"
while IFS= read -r -d '' events; do
  dir="$(dirname "$events")"
  [[ -s "$dir/manifest.json" && -s "$dir/snapshot.json" && -s "$dir/fork.json" ]] || {
    echo "Incomplete Hiyal trial bundle at: $dir" >&2
    exit 2
  }
done < <(find "$output" -type f -path '*/arms/*/events.jsonl' -print0)
if [[ "$#" -eq 1 ]]; then
  [[ -s "$output/SHA256SUMS" ]] || { echo "No SHA256SUMS file" >&2; exit 2; }
  (cd "$output" && sha256sum --check --status SHA256SUMS) || { echo "Evidence hash mismatch" >&2; exit 2; }
fi
echo "Verified 18 cells × 10 pairs, 240 complete original Hiyal arm journals; SHA-256 $(if [[ "$#" -eq 1 ]]; then echo checked; else echo pending; fi)"
