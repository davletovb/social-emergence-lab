#!/usr/bin/env bash
# Manual-only synthetic paired-prior development sweep.
# Seeds are fixed before evaluation. Each parent seed has two DIFFERENT
# stochastic histories after the false-prior parameter changes.
set -euo pipefail
[[ $# -eq 1 && "$1" == /* ]] || { echo "Usage: scripts/pilot-paired-002.sh <NEW-ABSOLUTE-OUTPUT-DIR>" >&2; exit 2; }
root="$1"
[[ ! -e "$root" ]] || { echo "Refusing to overwrite existing evidence" >&2; exit 2; }
for tool in git cargo jq sha256sum; do command -v "$tool" >/dev/null || exit 2; done
cd "$(dirname "$0")/.."
[[ -z "$(git status --porcelain)" ]] || { echo "Refusing dirty research source" >&2; exit 2; }
source_sha="$(git rev-parse HEAD)"
if [[ "${GITHUB_EVENT_NAME:-}" == pull_request ]]; then
    source_sha="$(git rev-parse --verify HEAD^2)"
fi
cargo build --quiet --locked --bin reputation-paired-prior
target="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)"
binary="$target/debug/reputation-paired-prior"
mkdir -p "$root"
# Exact 20-agent model, 200/600 event schedule, development seeds only.
# DO NOT replace seeds that fail or adapt the sample based on effect signs.
for seed in 92000 92001 92002 92003; do
    path="$root/paired-$seed"
    "$binary" run "$path" "$seed" 20 4 200 600 >/dev/null
    "$binary" verify "$path" >/dev/null
done
jq -n --arg source "$source_sha" '{
 scientific_status:"UNREGISTERED_SYNTHETIC_PAIRED_DEVELOPMENT",
 source_revision:$source,
 seeds:[92000,92001,92002,92003],
 agents:20,degree:4,checkpoint:200,total:600,
 design:"Each seed is repeated with delta=0.2 and delta=0.0, separately journaled and verified; no missing seeds replaced",
 inference:"Four matched parent seeds are insufficient for hypothesis tests, precise intervals or novelty claims"
}' > "$root/batch-plan.json"
jq -s '
def mean: add/length;
{scientific_status:"EXPLORATORY_NOT_CONFIRMATORY",
parent_pairs:length,seeds:map(.seed),
mean_prior_effect:(map(.matched_seed_prior_effect)|mean),
per_pair:map({seed,prior_information_late_gap,no_prior_information_late_gap,matched_seed_prior_effect,paired_repair_difference})}
' "$root"/paired-*/paired-report.json > "$root/paired-summary.json"
(cd "$root"; find . -type f ! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock -print0 |
  LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS
 sha256sum --check --strict --status SHA256SUMS)
cat "$root/paired-summary.json"
echo "SHA256SUMS-DIGEST"
sha256sum "$root/SHA256SUMS"
