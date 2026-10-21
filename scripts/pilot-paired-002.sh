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
# Release build: about 11x faster than debug, with byte-identical output.
cargo build --quiet --locked --release --bin reputation-paired-prior
target="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)"
binary="$target/release/reputation-paired-prior"
mkdir -p "$root"
# Exact 20-agent model, 200/600 event schedule, development seeds only.
# DO NOT replace seeds that fail or adapt the sample based on effect signs.
for seed in 92000 92001 92002 92003; do
    path="$root/paired-$seed"
    # run re-derives paired-report.json from both runs' journals before writing it.
    "$binary" run "$path" "$seed" 20 4 200 600 >/dev/null
done
# The engine records its own revision in every genesis; the runs must agree.
hiyal="$(jq -r .hiyal_revision "$root"/paired-*/paired-report.json | sort -u)"
[[ "$hiyal" =~ ^[0-9a-f]{40}$ ]] || { echo "Runs disagree on, or lack, the Hiyal revision" >&2; exit 2; }
jq -n --arg source "$source_sha" --arg hiyal "$hiyal" '{
 scientific_status:"UNREGISTERED_SYNTHETIC_PAIRED_DEVELOPMENT",
 layout:"paired-prior",
 source_revision:$source,
 hiyal_revision:$hiyal,
 seeds:[92000,92001,92002,92003],
 agents:20,degree:4,checkpoint:200,total:600,
 design:"Each seed is repeated with delta=0.2 and delta=0.0, separately journaled and verified; no missing seeds replaced",
 inference:"Four matched parent seeds are insufficient for hypothesis tests, precise intervals or novelty claims"
}' > "$root/batch-plan.json"
jq -s -f scripts/paired-summary-002.jq "$root"/paired-*/paired-report.json > "$root/paired-summary.json"
(cd "$root"; find . -type f ! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock -print0 |
  LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS
 sha256sum --check --strict --status SHA256SUMS)
# Check the sealed batch as a whole, exactly as a preserved copy would be.
bash scripts/verify-002.sh "$root" >&2
cat "$root/paired-summary.json"
echo "SHA256SUMS-DIGEST"
sha256sum "$root/SHA256SUMS"
