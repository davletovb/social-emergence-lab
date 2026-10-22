#!/usr/bin/env bash
# Verify original byte seal, every Hiyal parent/fork ancestry and derived V3 summary.
# Optional second argument is the independently archived SHA256 of SHA256SUMS.
set -euo pipefail
[[ $# -ge 1 && $# -le 2 && "$1" == /* ]] || {
  echo "usage: bash scripts/verify-v3-002.sh <ABSOLUTE-DIR> [EXPECTED-MANIFEST-SHA256]" >&2; exit 2;
}
root="$1"
[[ -f "$root/SHA256SUMS" && -f "$root/batch-plan.json" ]] || { echo "Missing sealed V3 evidence" >&2; exit 2; }
cd "$(dirname "$0")/.."
for cmd in jq cargo sha256sum cmp; do command -v "$cmd" >/dev/null || exit 2; done
if [[ $# -eq 2 ]]; then
  [[ "$(sha256sum "$root/SHA256SUMS" | cut -d' ' -f1)" == "$2" ]] || {
    echo "Manifest digest differs from archived reference" >&2; exit 2;
  }
fi
(
  cd "$root"
  sha256sum --check --strict --status SHA256SUMS
  tmp="$(mktemp)"
  trap 'rm -f "$tmp"' EXIT
  find . -type f ! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock -print0 |
    LC_ALL=C sort -z | xargs -0 sha256sum > "$tmp"
  cmp -s SHA256SUMS "$tmp" || { echo "Unlisted/modified evidence files" >&2; exit 2; }
)
jq -e '.protocol == "experiment-002-v3-20261022"
  and ((.source_revision | type) == "string")
  and (.source_revision | test("^[0-9a-f]{40}$"))' "$root/batch-plan.json" >/dev/null
[[ "$(jq -r '.cells | length' "$root/batch-plan.json")" == 9 ]] || exit 2
cargo build --quiet --locked --release --bin reputation-v3
target="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)"
binary="$target/release/reputation-v3"
cell=0
for period in 2 10 20; do
  for mode in random-turnover belief-deletion full-feedback; do
    seed=$((120200 + cell))
    jq -e --argjson i "$cell" --argjson seed "$seed" --argjson period "$period" --arg mode "$mode" \
      '.cells[$i] == {seed:$seed,rewire_every:$period,mechanism:$mode}' "$root/batch-plan.json" >/dev/null
    path="$root/cell-$cell-$mode-m$period"
    "$binary" verify "$path" >/dev/null
    jq -n --slurpfile prior "$path/paired-report.json" \
       --slurpfile with "$path/false-prior/report.json" \
       --slurpfile null "$path/no-false-prior/report.json" \
       --arg mode "$mode" --argjson seed "$seed" --argjson period "$period" \
       -f scripts/v3-cell-summary-002.jq | cmp -s "$path/cell-summary.json" - || {
      echo "Derived V3 cell summary differs from journals: $cell" >&2; exit 2;
    }
    cell=$((cell+1))
  done
done
jq -s '{scientific_status:"EXPLORATORY_V3_FEASIBILITY_NOT_CONFIRMATORY",
  independent_parent_societies:length, cells:.,
  no_go_rule:"A positive total gap alone cannot establish ongoing reputation feedback; compare new-link+selection with structural survival and delta-zero control."
}' "$root"/cell-*/cell-summary.json | cmp -s "$root/summary.json" - || {
  echo "V3 aggregate summary differs from sealed individual results" >&2; exit 2;
}
echo "V3: SHA manifest, exact nine-cell design, Hiyal journals and summary verified" >&2
