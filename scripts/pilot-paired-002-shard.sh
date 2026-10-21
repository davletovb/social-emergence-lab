#!/usr/bin/env bash
# One-time, fixed-design exploratory batch SHARD. Exactly 12 distinct parent
# seeds; no dropping/replacing failed seeds or selecting based on outcomes.
set -euo pipefail
[[ $# -eq 2 && "$1" == /* && "$2" =~ ^[012]$ ]] || { echo "usage: bash scripts/pilot-paired-002-shard.sh <NEW-ABSOLUTE-DIR> <shard:0..2>" >&2; exit 2; }
root="$1"
shard="$2"
[[ ! -e "$root" ]] || { echo "Refusing existing evidence dir" >&2; exit 2; }
for cmd in git jq cargo sha256sum; do command -v "$cmd" >/dev/null || exit 2; done
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo"
[[ -z "$(git status --porcelain)" ]] || { echo "Dirty research checkout" >&2; exit 2; }
source_sha="$(git rev-parse HEAD)"
if [[ "${GITHUB_EVENT_NAME:-}" == pull_request ]]; then source_sha="$(git rev-parse HEAD^2)"; fi
first=$((98000 + shard * 12))
last=$((first + 12))
mkdir -p "$root"
cargo build --quiet --locked --release --bin reputation-paired-prior
target="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)"
binary="$target/release/reputation-paired-prior"
for ((seed=first; seed<last; seed++)); do
  echo "PAIRED-V2 seed=$seed shard=$shard" >&2
  "$binary" run "$root/paired-$seed" "$seed" 20 4 200 600 >/dev/null
done
hiyal="$(jq -r .hiyal_revision "$root"/paired-*/paired-report.json | sort -u)"
[[ "$hiyal" =~ ^[a-f0-9]{40}$ ]] || { echo "Missing/nonuniform Hiyal provenance" >&2; exit 2; }
jq -n --arg source "$source_sha" --arg hiyal "$hiyal" --argjson first "$first" --argjson last "$last" --argjson shard "$shard" '{
 scientific_status:"EXPLORATORY_NOT_PREREGISTERED",
 layout:"paired-prior",
 source_revision:$source,hiyal_revision:$hiyal,
 shard:$shard, seeds:[range($first;$last)],
 actors:20,degree:4,checkpoint:200,total:600,
 design:"12 fixed disjoint matched-seed parent pairs; only δ changes; parent is unit of analysis",
 primary:"mean information-only last-100-event signed gap at δ=0.2, compared to symmetry-defined zero",
 secondary:"matched late signed-gap difference and δ=0 symmetry control",
 caveat:"No human data; not confirmatory; report every failed or missing seed and do not replace it"
}' > "$root/batch-plan.json"
jq -s -f scripts/paired-summary-002.jq "$root"/paired-*/paired-report.json > "$root/paired-summary.json"
(
 cd "$root"
 find . -type f ! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock -print0 |
   LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS
 sha256sum --check --strict --status SHA256SUMS
)
bash scripts/verify-002.sh "$root" >&2
echo "BEGIN-V2-SHARD-SUMMARY"
cat "$root/paired-summary.json"
echo "END-V2-SHARD-SUMMARY"
echo "BEGIN-V2-SHARD-SEAL"
sha256sum "$root/SHA256SUMS"
echo "END-V2-SHARD-SEAL"
