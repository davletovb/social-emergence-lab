#!/usr/bin/env bash
# Derived-results-only V3 Stage B audit; no simulation. Ties every committed
# Stage B file to its cell's SHA256SUMS seal, checks each plan is the frozen
# design from `reputation-v3 plan`, re-derives each decision under the rule
# the plan names (Amendment 1), and re-derives stage-b-summary.json from the
# sealed files (--write regenerates it). It asserts nothing about which way
# the results came out.
set -euo pipefail
[[ $# -eq 0 || ( $# -eq 1 && "$1" == --write ) ]] || {
  echo "usage: bash scripts/check-v3-stage-b-002.sh [--write]" >&2
  exit 2
}
fail() { echo "$1" >&2; exit 2; }
cd "$(dirname "$0")/.."
folder=experiments/002-reputation-lock-in/results/v3-stage-b-20261022
summary="$folder/stage-b-summary.json"
canonical='walk(if type == "number" then . + 0 else . end)'
same() { cmp -s <(jq -cS "$canonical" "$1") <(jq -cS "$canonical"); }
# <cell dir> <committed file> <path in SHA256SUMS>
sealed() { grep -qx "$(sha256sum "$1/$2" | cut -d' ' -f1)  ./$3" "$1/SHA256SUMS"; }
cargo build --quiet --locked --bin reputation-v3
binary="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)/debug/reputation-v3"

cells=()
for cell in 0 1 3 4 6 7; do
  dir="$folder/cell-$cell"
  plan="$dir/batch-plan.json"
  sealed "$dir" batch-plan.json batch-plan.json && sealed "$dir" decision.json decision.json ||
    fail "Cell $cell plan or decision is not the sealed copy"
  design="$("$binary" plan "$cell")"
  jq -e --argjson design "$design" '.design == $design and .design.stage == "B"
    and (.source_revision | test("^[0-9a-f]{40}$"))
    and (.hiyal_revision | test("^[0-9a-f]{40}$"))' "$plan" > /dev/null ||
    fail "Cell $cell plan is not the frozen Stage B design"
  read -r rule t margin cv_t < <(jq -r '[.decision_rule, .critical_t, .margin,
    .control_variate_critical_t] | @tsv' <<< "$design")
  [[ "$rule" == v3-decision-002-amendment-1.jq ]] || fail "Cell $cell plan names no Amendment 1 rule"
  mapfile -t seeds < <(jq -r '.seeds[]' <<< "$design")
  [[ "$(cd "$dir/cell-summaries" && LC_ALL=C ls)" == "$(printf 'parent-%s.json\n' "${seeds[@]}")" ]] ||
    fail "Cell $cell summaries are not exactly the planned parents"
  for seed in "${seeds[@]}"; do
    sealed "$dir" "cell-summaries/parent-$seed.json" "parent-$seed/cell-summary.json" ||
      fail "Cell $cell parent $seed summary is not the sealed copy"
  done
  jq -s --argjson t "$t" --argjson margin "$margin" --argjson cv_t "$cv_t" -f "scripts/$rule" \
    "$dir"/cell-summaries/parent-*.json | same "$dir/decision.json" ||
    fail "Cell $cell decision differs from its summaries"
  cells+=("$(jq -n -c --slurpfile p "$plan" --slurpfile d "$dir/decision.json" \
    --arg seal "$(sha256sum "$dir/SHA256SUMS" | cut -d' ' -f1)" '$p[0] as $p | $d[0] as $d | {
      cell: $p.design.cell, rewire_every: $d.rewire_every, mechanism: $d.mechanism,
      classification: $d.classification,
      frozen_rule_classification: $d.frozen_rule_classification,
      median_classification: $d.robustness.median_classification,
      robust: $d.robustness.agrees,
      new_link_plus_selection: $d.new_link_plus_selection.interval_95,
      structural_excess: $d.structural_excess.interval_95,
      regeneration_exceeds_margin: $d.regeneration_exceeds_margin,
      compensation_exceeds_margin: $d.compensation_exceeds_margin,
      null_symmetry_flag: $d.null_control.symmetry_flag,
      null_symmetry_flag_t: $d.null_control.symmetry_flag_t,
      source_revision: $p.source_revision, hiyal_revision: $p.hiyal_revision,
      sha256sums_sha256: $seal}')")
done

derived="$(printf '%s\n' "${cells[@]}" | jq -s '
  if (map(.source_revision) | unique | length) != 1 or (map(.hiyal_revision) | unique | length) != 1
  then error("Stage B cells disagree on source or Hiyal revision") else . end
  | {protocol: "experiment-002-v3-20261022", stage: "B",
     rule: "v3-decision-002-amendment-1.jq",
     source_revision: .[0].source_revision, hiyal_revision: .[0].hiyal_revision,
     no_regeneration_under_either_rule:
       all(.[]; .classification != "regeneration" and .median_classification != "regeneration"),
     cells: map(del(.source_revision, .hiyal_revision))}')"
if [[ $# -eq 1 ]]; then
  jq . <<< "$derived" > "$summary"
else
  same "$summary" <<< "$derived" || fail "stage-b-summary.json differs from the sealed cells"
fi
jq -c '{stage, no_regeneration_under_either_rule, cells: [.cells[] | {cell, classification,
  median_classification, null_symmetry_flag}]}' "$summary"
