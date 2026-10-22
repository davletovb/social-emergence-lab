#!/usr/bin/env bash
# Manual-only V3 nine-cell feasibility pilot. NEVER execute in routine CI.
# Frozen exploratory seeds before any V3 results: 120200..120208, one per cell.
set -euo pipefail
[[ $# -eq 1 && "$1" == /* ]] || { echo "usage: bash scripts/v3-feasibility-002.sh <NEW-ABSOLUTE-DIR>" >&2; exit 2; }
root="$1"
[[ ! -e "$root" ]] || { echo "Refusing to overwrite V3 evidence" >&2; exit 2; }
for cmd in git jq cargo sha256sum; do command -v "$cmd" >/dev/null || exit 2; done
cd "$(dirname "$0")/.."
[[ -z "$(git status --porcelain)" ]] || { echo "Refusing dirty research checkout" >&2; exit 2; }
source_sha="$(git rev-parse HEAD)"
# These nine parent seeds are disjoint from all earlier V2 evidence.
mkdir -p "$root"
active_cell="preflight"
trap 'status=$?; printf "V3 feasibility failed at %s (exit=%s). Planned seeds are not replaceable.\\n" "$active_cell" "$status" > "$root/FAILURE.txt"; exit "$status"' ERR
jq -n --arg source "$source_sha" '{
  scientific_status: "EXPLORATORY_V3_FEASIBILITY_ONLY",
  protocol: "experiment-002-v3-20261022",
  source_revision: $source,
  cells: [range(0;9) as $i | {
    seed: (120200 + $i),
    rewire_every: ([2,10,20][($i/3 | floor)]),
    mechanism: (["random-turnover","belief-deletion","full-feedback"][$i%3])
  }],
  actors: 20, degree: 4, checkpoint: 200, total: 600,
  primary: "late new/readded-link exposure + conditional selection after truthful correction",
  caution: "Pilot is one independent parent seed per cell. No uncertainty estimate or lock-in assertion. All missing/failed cells reported, never replaced."
}' > "$root/batch-plan.json"
cargo build --quiet --locked --release --bin reputation-v3
target="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)"
binary="$target/release/reputation-v3"
cell=0
for period in 2 10 20; do
  for mode in random-turnover belief-deletion full-feedback; do
    seed=$((120200 + cell))
    path="$root/cell-$cell-$mode-m$period"
    active_cell="cell=$cell seed=$seed m=$period mode=$mode"
    echo "V3 FEASIBILITY $active_cell" >&2
    "$binary" run "$path" "$seed" "$period" "$mode" 20 4 200 600 >/dev/null
    "$binary" verify "$path" >/dev/null
    jq -n --slurpfile prior "$path/paired-report.json" \
       --slurpfile with "$path/false-prior/report.json" \
       --slurpfile null "$path/no-false-prior/report.json" \
       --arg mode "$mode" --argjson seed "$seed" --argjson period "$period" \
       -f scripts/v3-cell-summary-002.jq > "$path/cell-summary.json"
    cell=$((cell+1))
  done
done
[[ "$cell" == 9 ]] || { echo "Incorrect cell count" >&2; exit 2; }
jq -s '{scientific_status:"EXPLORATORY_V3_FEASIBILITY_NOT_CONFIRMATORY",
  independent_parent_societies:length, cells:.,
  no_go_rule:"A positive total gap alone cannot establish ongoing reputation feedback; compare new-link+selection with structural survival and delta-zero control."
}' "$root"/cell-*/cell-summary.json > "$root/summary.json"
(
  cd "$root"
  find . -type f ! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock -print0 |
    LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS
  sha256sum --check --strict --status SHA256SUMS
)
bash scripts/verify-v3-002.sh "$root" >/dev/null
trap - ERR
echo "V3 MANUAL FEASIBILITY ARCHIVE (retain raw journals outside GitHub Actions)" >&2
sha256sum "$root/SHA256SUMS"
echo "V3 summary: $root/summary.json" >&2
