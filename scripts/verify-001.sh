#!/usr/bin/env bash
# Verify an Experiment 001 Hiyal study directory. Expected counts come from
# its study-plan.json; every parent and arm is ledger-, fork- and
# reference-replayed by the `validate` binary; and, unless --before-hash is
# given, SHA256SUMS must match and cover exactly the files present.
set -euo pipefail
usage() {
  echo "Usage: bash scripts/verify-001.sh <grid-output-directory> [--before-hash]" >&2
  exit 2
}
[[ $# -eq 1 || ( $# -eq 2 && "$2" == --before-hash ) ]] || usage
fail() { echo "$1" >&2; exit 2; }
[[ -d "$1" ]] || fail "No such evidence directory: $1"
output="$(cd "$1" && pwd)"
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
check_hashes=$([[ $# -eq 1 ]] && echo yes || echo no)
seal_excludes=(! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock)

for f in study-plan.json hiyal-summary.csv hiyal-trajectories.csv hiyal-benchmarks.csv hiyal-study-report.json reproducibility-manifest.json; do
  [[ -s "$output/$f" ]] || fail "Missing evidence: $f"
done
plan="$output/study-plan.json"
jq -e '.schema == 2' "$plan" >/dev/null || fail "Unsupported study plan schema"
jq -e --slurpfile plan "$plan" '
  .schema == 2 and .scientific_status == "exploratory_not_preregistered"
  and .first_seed == $plan[0].first_seed
  and .paired_societies_per_cell == $plan[0].pairs
  and .hiyal_git_sha == $plan[0].hiyal_revision
' "$output/reproducibility-manifest.json" >/dev/null || fail "Reproducibility manifest disagrees with study plan"

# Each (actors, initial, horizon) group has one parent and one shared control
# per seed, plus one treatment per offset cell.
read -r pairs cells groups cell_steps group_steps < <(jq -r '
  ([.cells[] | {actors, initial, horizon}] | unique) as $groups
  | [.pairs, (.cells | length), ($groups | length), ([.cells[].horizon] | add), ([$groups[].horizon] | add)]
  | @tsv' "$plan")
arms=$((pairs * (groups + cells)))
parents=$((pairs * groups))
events=$((pairs * (cell_steps + group_steps)))

cd "$output"
if [[ "$check_hashes" == yes ]]; then
  [[ -s SHA256SUMS ]] || fail "No SHA256SUMS file"
  sha256sum --check --status --strict SHA256SUMS || fail "Evidence hash mismatch"
  diff -q <(sed -E 's/^[0-9a-f]{64}  //' SHA256SUMS) \
    <(find . -type f "${seal_excludes[@]}" -print | LC_ALL=C sort) >/dev/null \
    || fail "SHA256SUMS does not cover exactly the files present"
fi

planned_cells="$(mktemp)"
planned_trials="$(mktemp)"
trap 'rm -f "$planned_cells" "$planned_trials"' EXIT
# The exact trial directories the plan implies: per group and seed, one
# parent, one shared control and one treatment per offset cell.
first_seed="$(jq -r .first_seed "$plan")"
[[ "$first_seed" =~ ^[0-9]+$ ]] || fail "Unsupported first_seed in study plan"
while IFS=$'\t' read -r actors initial horizon offsets; do
  for ((i = 0; i < pairs; i++)); do
    seed_dir="./actors-$actors-initial-$initial-steps-$horizon/seed-$(printf '%020d' $((first_seed + i)))"
    echo "$seed_dir/parent"
    echo "$seed_dir/arms/control"
    for offset in $offsets; do
      echo "$seed_dir/arms/treatment-offset-$offset"
    done
  done
done < <(jq -r '.cells | group_by([.actors, .initial, .horizon])[]
  | [.[0].actors, .[0].initial, .[0].horizon, ([.[].offset | tostring] | join(" "))] | @tsv' "$plan") \
  | LC_ALL=C sort > "$planned_trials"
[[ "$(wc -l < "$planned_trials")" -eq $((arms + parents)) ]] || fail "Could not derive the planned trial set"
# Search relative to the output directory so its own path cannot match.
find . -mindepth 3 -maxdepth 4 -type d \( -path './*/seed-*/parent' -o -path './*/seed-*/arms/*' \) ! -name '.*' \
  | LC_ALL=C sort | diff -q - "$planned_trials" >/dev/null \
  || fail "Trial directories do not match the study plan's groups, seeds and arms"
mapfile -t arm_dirs < <(grep '/arms/' "$planned_trials")
mapfile -t parent_dirs < <(grep '/parent$' "$planned_trials")
for dir in "${arm_dirs[@]}"; do
  for f in manifest.json events.jsonl snapshot.json fork.json; do
    [[ -s "$dir/$f" ]] || fail "Incomplete Hiyal arm bundle at $dir (missing $f)"
  done
  for f in effects.jsonl ancestry/manifest.json ancestry/events.jsonl; do
    [[ -f "$dir/$f" ]] || fail "Incomplete Hiyal arm bundle at $dir (missing $f)"
  done
done
for dir in "${parent_dirs[@]}"; do
  for f in manifest.json snapshot.json; do
    [[ -s "$dir/$f" ]] || fail "Incomplete Hiyal parent at $dir (missing $f)"
  done
  [[ -f "$dir/events.jsonl" ]] || fail "Incomplete Hiyal parent at $dir (missing events.jsonl)"
done

# Derived outputs must match the plan's shape.
jq -r '.cells[] | [.actors, .initial, .offset, .horizon] | join(",")' "$plan" | LC_ALL=C sort > "$planned_cells"
awk -F, -v pairs="$pairs" '
  NR==1 { if ($0 != "actors,initial,offset,horizon,pairs,control_mean_gini,treatment_mean_gini,paired_delta,bootstrap_95_lo,bootstrap_95_hi") exit 1; next }
  NF != 10 || $5 != pairs || (pairs >= 10) != ($9 != "" && $10 != "") { exit 1 }
  { print $1 FS $2 FS $3 FS $4 }
' hiyal-summary.csv | LC_ALL=C sort | diff -q - "$planned_cells" >/dev/null \
  || fail "hiyal-summary.csv does not match the study plan's cells"
awk -F, -v rows=$((3 * arms)) '
  NR==1 { if ($0 != "actors,initial,offset,horizon,seed,arm,step,gini,top_share,rank_persistence,mean_rank_movement") exit 1; next }
  NF != 11 { exit 1 }
  { n++ } END { if (n != rows) exit 1 }
' hiyal-trajectories.csv || fail "hiyal-trajectories.csv should have $((3 * arms)) rows"
awk -F, -v arms="$arms" -v events="$events" '
  NR==1 { if ($0 != "actors,initial,offset,horizon,seed,arm,start_step,new_accepted_steps,run_ms,verification_ms,journal_bytes,snapshot_bytes,state_json_bytes") exit 1; next }
  NF != 13 { exit 1 }
  { n++; steps += $8 } END { if (n < arms || steps != events) exit 1 }
' hiyal-benchmarks.csv || fail "hiyal-benchmarks.csv should cover $events accepted events across $arms arms"

# Independent Hiyal ledger, fork attribution and reference replay of every trial.
cargo build --quiet --locked --release --bin validate --manifest-path "$repo/Cargo.toml"
target="$(cargo metadata --locked --format-version 1 --no-deps --manifest-path "$repo/Cargo.toml" | jq -r .target_directory)"
"$target/release/validate" "${parent_dirs[@]}" "${arm_dirs[@]}" >/dev/null

echo "Verified $cells cells x $pairs pairs: $arms arms and $parents parents ledger-, fork- and reference-replayed, $events accepted events; SHA-256 $([[ "$check_hashes" == yes ]] && echo checked || echo pending)"
