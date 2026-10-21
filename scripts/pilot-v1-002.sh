#!/usr/bin/env bash
# Fixed-seed exploratory V1 at the protocol's N20,tau200,T600 schedule.
# One-time development experiment, never enabled in ordinary PR CI.
set -euo pipefail
[[ $# -eq 1 && "$1" == /* ]] || { echo "usage: pilot-v1-002.sh <NEW-absolute-output-dir>" >&2; exit 2; }
root="$1"
[[ ! -e "$root" ]] || { echo "Output already exists: $root" >&2; exit 2; }
for c in jq sha256sum cargo git; do command -v "$c" >/dev/null || exit 2; done
cd "$(dirname "$0")/.."
[[ -z "$(git status --porcelain)" ]] || { echo "Dirty checkout: refusing scientific batch" >&2; exit 2; }
source_sha="$(git rev-parse HEAD)"
if [[ "${GITHUB_EVENT_NAME:-}" == "pull_request" ]]; then
  source_sha="$(git rev-parse HEAD^2)"
fi
mkdir -p "$root"
# Release build: about 11x faster than debug, with byte-identical output.
cargo build --locked --quiet --release --bin reputation-pilot
binary="$(cargo metadata --locked --no-deps --format-version 1 | jq -r .target_directory)/release/reputation-pilot"
# Predeclared independent domains per *configuration*. Fail on ANY missing world,
# do not replace any failed seed or omit an unfavorable result.
cases=(
 "91000 default" "91001 default" "91002 default"
 "91100 no-false-prior" "91101 no-false-prior"
 "91200 random-churn" "91201 random-churn"
 "91300 annealed-mixing" "91301 annealed-mixing"
 "91400 full-mixing" "91401 full-mixing"
 "91500 rewire-every-2" "91501 rewire-every-2"
 "91600 rewire-every-20" "91601 rewire-every-20"
)
for case in "${cases[@]}"; do
  read -r seed scenario <<< "$case"
  path="$root/${scenario}-${seed}"
  echo "EXPLORATORY-TRIAL scenario=$scenario seed=$seed" >&2
  "$binary" "$path" "$seed" 20 4 200 600 "$scenario" >/dev/null
  "$binary" verify "$path" >/dev/null
done
hiyal="$(jq -r .hiyal_revision "$root"/*/report.json | sort -u)"
[[ "$hiyal" =~ ^[0-9a-f]{40}$ ]] || { echo "Non-uniform/unrecorded Hiyal revisions" >&2; exit 2; }
jq -n --arg source "$source_sha" --arg hiyal "$hiyal" '{
  scientific_status:"EXPLORATORY_UNREGISTERED_MODEL_VALIDATION",
  source_revision:$source,hiyal_revision:$hiyal,
  actors:20,checkpoint:200,total_steps:600,
  controls:["default","no-false-prior","random-churn","annealed-mixing","full-mixing","rewire-every-2","rewire-every-20"],
  seed_policy:"Fixed publicly listed, disjoint development seeds; never replace failed seeds",
  measurement:"Last 100 accepted post-checkpoint opportunities",
  warning:"Three independent parents in default; only two in each control. No significant differences, power, human effects or novel scientific discovery can be inferred."
}' > "$root/batch-plan.json"
jq -s -f scripts/batch-summary-v1-002.jq "$root"/*/report.json > "$root/batch-summary.json"
jq -s -f scripts/arm-summary-002.jq "$root"/*/report.json > "$root/arm-summary.json"
(
  cd "$root"
  find . -type f ! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS
  sha256sum --check --strict --status SHA256SUMS
)
echo "BEGIN-V1-BATCH-PLAN"
cat "$root/batch-plan.json"
echo "END-V1-BATCH-PLAN"
echo "BEGIN-V1-BATCH-SUMMARY"
cat "$root/batch-summary.json"
echo "END-V1-BATCH-SUMMARY"
echo "BEGIN-V1-ARM-SUMMARY"
cat "$root/arm-summary.json"
echo "END-V1-ARM-SUMMARY"
echo "BEGIN-V1-SEAL-DIGEST"
sha256sum "$root/SHA256SUMS"
echo "END-V1-SEAL-DIGEST"
