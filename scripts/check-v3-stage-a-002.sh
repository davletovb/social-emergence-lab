#!/usr/bin/env bash
# Derived-results-only V3 Stage A audit; no simulation. Ties every committed
# Stage A file to its cell's SHA256SUMS seal, re-derives each frozen decision
# from the committed cell summaries, checks stage-a-summary.json, and
# re-derives the post hoc Amendment 1 analysis (--write regenerates it). It
# asserts nothing about which way the results came out.
set -euo pipefail
[[ $# -eq 0 || ( $# -eq 1 && "$1" == --write ) ]] || {
  echo "usage: bash scripts/check-v3-stage-a-002.sh [--write]" >&2
  exit 2
}
fail() { echo "$1" >&2; exit 2; }
cd "$(dirname "$0")/.."
folder=experiments/002-reputation-lock-in/results/v3-stage-a-20261022
post_hoc="$folder/stage-a-amendment-1-post-hoc.json"
canonical='walk(if type == "number" then . + 0 else . end)'
same() { cmp -s <(jq -cS "$canonical" "$1") <(jq -cS "$canonical"); }
# <cell dir> <committed file> <path in SHA256SUMS>
sealed() { grep -qx "$(sha256sum "$1/$2" | cut -d' ' -f1)  ./$3" "$1/SHA256SUMS"; }
# Stage B's design supplies Amendment 1's constants.
stage_b="$(cargo run --quiet --locked --bin reputation-v3 -- plan 0)"
read -r rule cv_t < <(jq -r '[.decision_rule, .control_variate_critical_t] | @tsv' <<< "$stage_b")
[[ "$rule" == v3-decision-002-amendment-1.jq ]] || fail "Stage B plan names no Amendment 1 rule"

cells=()
for cell in 2 5 8; do
  dir="$folder/cell-$cell"
  plan="$dir/batch-plan.json"
  sealed "$dir" batch-plan.json batch-plan.json && sealed "$dir" decision.json decision.json ||
    fail "Cell $cell plan or decision is not the sealed copy"
  jq -e --argjson cell "$cell" --argjson b "$stage_b" '.design.cell == $cell
    and .design.stage == "A" and .design.smoke == false
    and (.design | has("decision_rule") | not)
    and .design.critical_t == $b.critical_t and .design.margin == $b.margin' "$plan" > /dev/null ||
    fail "Cell $cell plan is not a Stage A main-grid design"
  mapfile -t seeds < <(jq -r '.design.seeds[]' "$plan")
  [[ "$(cd "$dir/cell-summaries" && LC_ALL=C ls)" == "$(printf 'parent-%s.json\n' "${seeds[@]}")" ]] ||
    fail "Cell $cell summaries are not exactly the planned parents"
  for seed in "${seeds[@]}"; do
    sealed "$dir" "cell-summaries/parent-$seed.json" "parent-$seed/cell-summary.json" ||
      fail "Cell $cell parent $seed summary is not the sealed copy"
  done
  read -r t margin < <(jq -r '[.design.critical_t, .design.margin] | @tsv' "$plan")
  jq -s --argjson t "$t" --argjson margin "$margin" -f scripts/v3-decision-002.jq \
    "$dir"/cell-summaries/parent-*.json | same "$dir/decision.json" ||
    fail "Cell $cell frozen decision differs from its summaries"
  cells+=("$(jq -s -c --argjson t "$t" --argjson margin "$margin" --argjson cv_t "$cv_t" \
    -f "scripts/$rule" "$dir"/cell-summaries/parent-*.json | jq -c --argjson cell "$cell" '{cell: $cell} + .')")
done

jq -e --slurpfile p2 "$folder/cell-2/batch-plan.json" --slurpfile p5 "$folder/cell-5/batch-plan.json" \
   --slurpfile p8 "$folder/cell-8/batch-plan.json" --slurpfile d2 "$folder/cell-2/decision.json" \
   --slurpfile d5 "$folder/cell-5/decision.json" --slurpfile d8 "$folder/cell-8/decision.json" \
   --arg s2 "$(sha256sum "$folder/cell-2/SHA256SUMS" | cut -d' ' -f1)" \
   --arg s5 "$(sha256sum "$folder/cell-5/SHA256SUMS" | cut -d' ' -f1)" \
   --arg s8 "$(sha256sum "$folder/cell-8/SHA256SUMS" | cut -d' ' -f1)" '
  . as $summary
  | [[$p2[0], $d2[0], $s2], [$p5[0], $d5[0], $s5], [$p8[0], $d8[0], $s8]] as $batches
  | .stage == "A" and (.cells | length) == 3
  and all(range(0; 3); $summary.cells[.] as $c | $batches[.] as [$p, $d, $s]
    | $c.cell == $p.design.cell and $c.rewire_every == $p.design.rewire_every
      and $c.classification == $d.classification
      and $c.new_link_plus_selection == $d.new_link_plus_selection.interval_95
      and $c.structural_excess == $d.structural_excess.interval_95
      and $c.null_symmetry_flag == $d.null_control.symmetry_flag
      and $c.sha256sums_sha256 == $s
      and $summary.source_revision == $p.source_revision
      and $summary.hiyal_revision == $p.hiyal_revision)
' "$folder/stage-a-summary.json" > /dev/null || fail "stage-a-summary.json does not match the sealed cells"

analysis="$(printf '%s\n' "${cells[@]}" | jq -s --arg rule "$rule" --argjson cv_t "$cv_t" '{
  status: "POST_HOC_DESCRIPTIVE_NOT_A_STAGE_A_DECISION",
  note: "Amendment 1 was adopted after these Stage A outcomes were seen. The Stage A decisions are the frozen rule'"'"'s, in cell-N/decision.json; this file applies the amended rule to the same committed cell summaries to describe them.",
  rule: $rule, control_variate_critical_t: $cv_t,
  cells: .}')"
if [[ $# -eq 1 ]]; then
  jq . <<< "$analysis" > "$post_hoc"
else
  same "$post_hoc" <<< "$analysis" || fail "Post hoc Amendment 1 analysis differs from the cell summaries"
fi
jq -c '{status, cells: [.cells[] | {cell, rewire_every, classification, median_classification: .robustness.median_classification,
  null_symmetry_flag: .null_control.symmetry_flag}]}' "$post_hoc"
