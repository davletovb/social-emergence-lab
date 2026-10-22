#!/usr/bin/env bash
# Verify one sealed V3 main-grid cell written by v3-grid-002.sh: the seal and
# its exact coverage (and, if given, the recorded SHA256SUMS digest), the
# frozen design from `reputation-v3 plan`, exactly the planned parents, every
# parent's Hiyal journals, and the re-derived cell summaries and decision
# (under the rule the plan names: frozen for Stage A, Amendment 1 for Stage B).
set -euo pipefail
usage() {
  echo "usage: bash scripts/verify-v3-grid-002.sh <ABSOLUTE-DIR> [--digest SHA256] [--jobs N]" >&2
  exit 2
}
fail() { echo "$1" >&2; exit 2; }
[[ $# -ge 1 && "$1" == /* ]] || usage
root="$1"
shift
digest=""
jobs=1
while (( $# )); do
  case "$1" in
    --digest) [[ $# -ge 2 ]] || usage; digest="$2"; shift 2 ;;
    --jobs) [[ $# -ge 2 && "$2" =~ ^[1-9][0-9]*$ ]] || usage; jobs="$2"; shift 2 ;;
    *) usage ;;
  esac
done
[[ -f "$root/SHA256SUMS" && -f "$root/batch-plan.json" ]] || fail "Missing sealed V3 grid evidence"
for cmd in jq cargo sha256sum cmp xargs; do command -v "$cmd" >/dev/null || fail "Missing $cmd"; done
cd "$(dirname "$0")/.."
if [[ -n "$digest" ]]; then
  [[ "$(sha256sum "$root/SHA256SUMS" | cut -d' ' -f1)" == "$digest" ]] || fail "SHA256SUMS digest differs from $digest"
fi
(cd "$root" && sha256sum --check --strict --quiet SHA256SUMS) || fail "SHA256SUMS check failed"
listed="$(sed 's/^[0-9a-f]\{64\}  //' "$root/SHA256SUMS" | LC_ALL=C sort)"
present="$(cd "$root" && find . -type f ! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock | LC_ALL=C sort)"
[[ "$listed" == "$present" ]] || fail "SHA256SUMS does not cover exactly the files present"

cargo build --quiet --locked --release --bin reputation-v3
binary="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)/release/reputation-v3"
plan="$root/batch-plan.json"
cell="$(jq -r '.design.cell' "$plan")"
[[ "$cell" =~ ^[0-8]$ ]] || fail "Batch plan names no V3 cell"
smoke=()
[[ "$(jq -r '.design.smoke' "$plan")" == true ]] && smoke=(--smoke)
design="$("$binary" plan "$cell" "${smoke[@]}")"
jq -e --argjson design "$design" '.design == $design
  and (.source_revision | test("^[0-9a-f]{40}$"))
  and (.hiyal_revision | test("^[0-9a-f]{40}$"))' "$plan" >/dev/null ||
  fail "Batch plan is not the frozen design of cell $cell"
read -r period mechanism t margin < <(jq -r '[.rewire_every, .mechanism, .critical_t, .margin] | @tsv' <<< "$design")
mapfile -t seeds < <(jq -r '.seeds[]' <<< "$design")
# Stage A keeps the frozen rule; Stage B plans name Amendment 1's rule.
rule="$(jq -r '.decision_rule // "v3-decision-002.jq"' <<< "$design")"
case "$rule" in
  v3-decision-002.jq | v3-decision-002-amendment-1.jq) ;;
  *) fail "Unknown V3 decision rule: $rule" ;;
esac
cv_t="$(jq -c '.control_variate_critical_t // null' <<< "$design")"

# Exactly the planned parents plus the plan, decision and seal.
shopt -s nullglob
entries=("$root"/* "$root"/.[!.]*)
(( ${#entries[@]} == ${#seeds[@]} + 3 )) || fail "Unexpected entries in V3 grid evidence"
[[ -f "$root/decision.json" ]] || fail "Missing decision.json"
for seed in "${seeds[@]}"; do
  path="$root/parent-$seed"
  [[ -d "$path" ]] || fail "Missing planned parent: $path"
  inside=("$path"/* "$path"/.[!.]*)
  (( ${#inside[@]} == 4 )) || fail "Unexpected entries in $path"
  for entry in false-prior no-false-prior paired-report.json cell-summary.json; do
    [[ -e "$path/$entry" ]] || fail "Missing $path/$entry"
  done
done

printf '%s\n' "${seeds[@]}" |
  xargs -P "$jobs" -I{} "$binary" verify "$root/parent-{}" > /dev/null ||
  fail "A parent failed Hiyal verification"
# Compare values, not jq's number spelling: jq versions differ on whether an
# unchanged 0.0 prints as 0.0 or 0.
canonical='walk(if type == "number" then . + 0 else . end)'
same() { cmp -s <(jq -cS "$canonical" "$1") <(jq -cS "$canonical"); }
for seed in "${seeds[@]}"; do
  path="$root/parent-$seed"
  jq -n --slurpfile prior "$path/paired-report.json" \
     --slurpfile with "$path/false-prior/report.json" \
     --slurpfile null "$path/no-false-prior/report.json" \
     --arg mode "$mechanism" --argjson seed "$seed" --argjson period "$period" \
     -f scripts/v3-cell-summary-002.jq | same "$path/cell-summary.json" ||
    fail "Cell summary differs from journals: $path"
done
jq -s --argjson t "$t" --argjson margin "$margin" --argjson cv_t "$cv_t" -f "scripts/$rule" \
  "$root"/parent-*/cell-summary.json | same "$root/decision.json" ||
  fail "decision.json differs from the cell summaries"
hiyal="$(jq -r .hiyal_revision "$root"/parent-*/paired-report.json | sort -u)"
jq -e --arg hiyal "$hiyal" '.hiyal_revision == $hiyal' "$plan" >/dev/null ||
  fail "Batch plan names a different Hiyal revision than its runs"
echo "V3 grid cell $cell (stage $(jq -r .stage <<< "$design"), m=$period, $mechanism, ${#seeds[@]} parents$([[ ${#smoke[@]} -gt 0 ]] && echo ', SMOKE')): verified; classification $(jq -r '.classification + (.robustness.median_classification | if . then " (median rule: \(.))" else "" end)' "$root/decision.json")"
echo "Source commit $(jq -r .source_revision "$plan"), Hiyal $hiyal"
