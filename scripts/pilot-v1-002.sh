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
cargo build --locked --quiet --bin reputation-pilot
binary="$(cargo metadata --locked --no-deps --format-version 1 | jq -r .target_directory)/debug/reputation-pilot"
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
jq -s '
def m: add/length;
def sd: if length<=1 then null else (m) as $mean | (map((.-$mean)*(.-$mean))|add/(length-1)|sqrt) end;
[ .[]|{seed, config,primary_information_late_gap,repair_after_truth_late,repair_without_truth_full,
    information_late:(.per_arm[]|select(.arm=="information")|.late),
    combined_late:(.per_arm[]|select(.arm=="both")|.late)
} ] |
group_by([.config.actors,.config.degree,.config.annealed,.config.removal,.config.beta,.config.kappa,.config.rewire_every,.config.delta]) |
{scientific_status:"exploratory_only_no_inferential_claim",groups:map({
 config:.[0].config,
 parents:length,seeds:map(.seed),
 info_gap_mean:(map(.primary_information_late_gap)|m),
 info_gap_sd:(map(.primary_information_late_gap)|sd),
 repair_after_truth_mean:(map(.repair_after_truth_late)|m),
 repair_after_truth_sd:(map(.repair_after_truth_late)|sd),
 repair_without_truth_mean:(map(.repair_without_truth_full)|m),
 info_degree_mean:(map(.information_late.degree)|m),
 info_selection_mean:(map(.information_late.selection)|m),
 info_inherited_mean:(map(.information_late.inherited)|m),
 info_added_mean:(map(.information_late.added)|m),
 info_survival_mean:(map(.information_late.mean_surviving_edge_fraction)|m),
 combined_gap_mean:(map(.combined_late.opportunity_gap)|m)
})}' "$root"/*/report.json > "$root/batch-summary.json"
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
echo "BEGIN-V1-SEAL-DIGEST"
sha256sum "$root/SHA256SUMS"
echo "END-V1-SEAL-DIGEST"
