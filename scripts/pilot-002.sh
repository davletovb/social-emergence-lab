#!/usr/bin/env bash
# One bounded, NON-CONFIRMATORY model-development batch for Experiment 002.
# Run once on a CI branch; do not select favorable seeds or discard failures.
set -euo pipefail
[[ $# -eq 1 && "$1" == /* ]] || { echo "Usage: bash scripts/pilot-002.sh <NEW-ABSOLUTE-DIR>" >&2; exit 2; }
root="$1"
[[ ! -e "$root" ]] || { echo "Refusing existing evidence directory $root" >&2; exit 2; }
for command in jq sha256sum cargo git; do command -v "$command" >/dev/null || { echo "Missing $command" >&2; exit 2; }; done
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo"
[[ -z "$(git status --porcelain)" ]] || { echo "Source must be clean before evidence run" >&2; exit 2; }
source_sha="$(git rev-parse HEAD)"
if [[ "${GITHUB_EVENT_NAME:-}" == pull_request ]]; then
  source_sha="$(git rev-parse --verify HEAD^2)" || { echo "PR checkout must have fetch-depth >=2" >&2; exit 2; }
fi
mkdir -p "$root"
cargo build --quiet --locked --bin reputation-pilot
target="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)"
pilot="$target/debug/reputation-pilot"

# Explicit public exploratory seeds. No adaptive selection; each parent seed is
# unique across groups/controls. Model after a fixed, completed prior review.
for seed in 81000 81001 81002 81003; do
  "$pilot" "$root/n6-default-$seed" "$seed" 6 2 20 60 default > /dev/null
done
for seed in 82000 82001 82002 82003; do
  "$pilot" "$root/n10-default-$seed" "$seed" 10 3 20 60 default > /dev/null
done
for entry in "83000 no-false-prior" "83001 no-selection-bias" "83002 fixed-graph" "83003 random-churn"; do
  read -r seed scenario <<< "$entry"
  "$pilot" "$root/n6-control-$scenario-$seed" "$seed" 6 2 20 60 "$scenario" > /dev/null
done

# The engine records its own revision in every genesis; the runs must agree.
hiyal="$(jq -r .hiyal_revision "$root"/n*/report.json | sort -u)"
[[ "$hiyal" =~ ^[0-9a-f]{40}$ ]] || { echo "Runs disagree on, or lack, the Hiyal revision" >&2; exit 2; }
jq -n --arg source "$source_sha" --arg hiyal "$hiyal" '
  {scientific_status:"exploratory_model_development_NOT_preregistered",
   source_commit:$source,hiyal_commit:$hiyal,
   purpose:"Testing simulation behavior, not evidence about real societies",
   started_from_fixed_seeds:true,
   specification:"n6,d2 and n10,d3 each four independent parents (tau20,total60), plus one parent for each named n6 negative control",
   limitations:"Four societies per configuration; do not report confidence intervals, significance, reproducibility across settings, or novel causal laws"}' > "$root/batch-plan.json"
jq -s -f "$repo/scripts/batch-summary-002.jq" "$root"/n*/report.json > "$root/batch-summary.json"
jq -s -f "$repo/scripts/arm-summary-002.jq" "$root"/n*/report.json > "$root/arm-summary.json"
(
  cd "$root"
  find . -type f ! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock -print0 |
    LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS
  sha256sum --check --status --strict SHA256SUMS
)
echo "BEGIN-002-BATCH-SUMMARY"
cat "$root/batch-summary.json"
echo "END-002-BATCH-SUMMARY"
echo "BEGIN-002-ARM-SUMMARY"
cat "$root/arm-summary.json"
echo "END-002-ARM-SUMMARY"
echo "BEGIN-002-BATCH-PLAN"
cat "$root/batch-plan.json"
echo "END-002-BATCH-PLAN"
echo "BEGIN-002-EVIDENCE-MANIFEST-DIGEST"
sha256sum "$root/SHA256SUMS"
echo "END-002-EVIDENCE-MANIFEST-DIGEST"
