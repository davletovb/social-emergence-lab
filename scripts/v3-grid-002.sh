#!/usr/bin/env bash
# Manual-only V3 main-grid runner for ONE cell; NEVER run it in routine CI.
# Stage A is cells 2, 5 and 8 (full feedback at m = 2, 10, 20). Runs the
# cell's frozen parents from `reputation-v3 plan`, derives each parent's cell
# summary and the cell's frozen decision, seals the batch and verifies it.
# --smoke runs three parents on non-frozen seeds: an engineering dry run, not
# evidence.
set -euo pipefail
usage() {
  echo "usage: bash scripts/v3-grid-002.sh <NEW-ABSOLUTE-DIR> <cell:0..8> [--smoke] [--jobs N]" >&2
  exit 2
}
[[ $# -ge 2 && "$1" == /* && "$2" =~ ^[0-8]$ ]] || usage
root="$1"
cell="$2"
shift 2
smoke=()
jobs=1
while (( $# )); do
  case "$1" in
    --smoke) smoke=(--smoke); shift ;;
    --jobs) [[ $# -ge 2 && "$2" =~ ^[1-9][0-9]*$ ]] || usage; jobs="$2"; shift 2 ;;
    *) usage ;;
  esac
done
[[ ! -e "$root" ]] || { echo "Refusing to overwrite V3 evidence: $root" >&2; exit 2; }
for cmd in git jq cargo sha256sum xargs; do command -v "$cmd" >/dev/null || { echo "Missing $cmd" >&2; exit 2; }; done
cd "$(dirname "$0")/.."
[[ -z "$(git status --porcelain)" ]] || { echo "Refusing dirty research checkout" >&2; exit 2; }
source_sha="$(git rev-parse HEAD)"
if [[ "${GITHUB_EVENT_NAME:-}" == pull_request ]]; then source_sha="$(git rev-parse HEAD^2)"; fi
# Release build: about 11x faster than debug, with byte-identical output.
cargo build --quiet --locked --release --bin reputation-v3
binary="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)/release/reputation-v3"
design="$("$binary" plan "$cell" "${smoke[@]}")"
read -r period mechanism actors degree checkpoint total t margin < <(
  jq -r '[.rewire_every, .mechanism, .actors, .degree, .checkpoint, .total, .critical_t, .margin] | @tsv' <<< "$design")
mapfile -t seeds < <(jq -r '.seeds[]' <<< "$design")

mkdir -p "$root"
trap 'status=$?; echo "V3 grid cell $cell failed (exit=$status); planned seeds are not replaceable" > "$root/FAILURE.txt"; exit "$status"' ERR
echo "V3 grid cell $cell: m=$period $mechanism, ${#seeds[@]} parents, $jobs jobs" >&2
# Each run re-derives its paired report from both sides' journals before writing it.
printf '%s\n' "${seeds[@]}" |
  xargs -P "$jobs" -I{} "$binary" run "$root/parent-{}" {} "$period" "$mechanism" \
    "$actors" "$degree" "$checkpoint" "$total" > /dev/null
for seed in "${seeds[@]}"; do
  path="$root/parent-$seed"
  jq -n --slurpfile prior "$path/paired-report.json" \
     --slurpfile with "$path/false-prior/report.json" \
     --slurpfile null "$path/no-false-prior/report.json" \
     --arg mode "$mechanism" --argjson seed "$seed" --argjson period "$period" \
     -f scripts/v3-cell-summary-002.jq > "$path/cell-summary.json"
done
jq -s --argjson t "$t" --argjson margin "$margin" -f scripts/v3-decision-002.jq \
  "$root"/parent-*/cell-summary.json > "$root/decision.json"
# The engine records its own revision in every genesis; the runs must agree.
hiyal="$(jq -r .hiyal_revision "$root"/parent-*/paired-report.json | sort -u)"
[[ "$hiyal" =~ ^[0-9a-f]{40}$ ]] || { echo "Runs disagree on, or lack, the Hiyal revision" >&2; exit 2; }
jq -n --argjson design "$design" --arg source "$source_sha" --arg hiyal "$hiyal" \
  '{design: $design, source_revision: $source, hiyal_revision: $hiyal}' > "$root/batch-plan.json"
(
  cd "$root"
  find . -type f ! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock -print0 |
    LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS
  sha256sum --check --strict --status SHA256SUMS
)
# Check the sealed batch exactly as a preserved copy would be checked.
bash scripts/verify-v3-grid-002.sh "$root" --jobs "$jobs" >&2
trap - ERR
jq -c '{cell: '"$cell"', rewire_every, mechanism, parents, classification,
  new_link_plus_selection: .new_link_plus_selection.interval_95,
  structural_excess: .structural_excess.interval_95}' "$root/decision.json"
echo "V3 GRID SEAL (record outside the evidence directory):" >&2
sha256sum "$root/SHA256SUMS"
