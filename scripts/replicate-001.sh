#!/usr/bin/env bash
# Produce the 18-cell exploratory Hiyal study and reproducibility manifest.
set -euo pipefail
if [[ $# -ne 2 ]]; then
  echo "Usage: bash scripts/replicate-001.sh <new-absolute-output-dir> <first-seed>" >&2
  exit 2
fi
output="$1"
seed="$2"
[[ "$output" = /* ]] || { echo "Output must be an absolute path" >&2; exit 2; }
[[ "$seed" =~ ^[0-9]+$ ]] || { echo "Seed must be an unsigned integer" >&2; exit 2; }
if [[ -e "$output" ]]; then
  echo "Refusing existing experiment directory: $output" >&2
  exit 2
fi
command -v jq >/dev/null
command -v sha256sum >/dev/null
command -v cargo >/dev/null
command -v git >/dev/null
[[ -z "$(git status --porcelain --untracked-files=no)" ]] || {
  echo "Tracked source changes must be committed before evidence generation" >&2
  exit 2
}
revision="$(git rev-parse HEAD)"
# PR checkout may be a synthetic merge; pin the authored source separately.
source_revision="$(git rev-parse HEAD^2 2>/dev/null || git rev-parse HEAD)"
hiyal="$(awk '/^\[\[package\]\]/{in_hiyal=0} /^name = "hiyal"$/{in_hiyal=1} in_hiyal && /^source =/{print;exit}' Cargo.lock | sed -E 's/.*#([0-9a-f]{40})".*/\1/')"
[[ "$hiyal" =~ ^[0-9a-f]{40}$ ]] || {
  echo "Could not determine exact Hiyal SHA from Cargo.lock" >&2
  exit 2
}
mkdir -p "$(dirname "$output")"
mkdir "$output"
start="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
cargo run --locked --release --bin hiyal-grid -- grid "$output" 10 "$seed"
finish="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"

cpu="$( (lscpu 2>/dev/null | grep -m1 '^Model name:' | cut -d: -f2- | sed 's/^ *//') || true)"
os="$(uname -srm)"
rustc="$(rustc --version --verbose)"
jq -n \
  --arg protocol "experiment-001-exploratory-replicated-v1" \
  --arg start "$start" --arg finish "$finish" \
  --arg lab_revision "$revision" --arg lab_source_revision "$source_revision" --arg hiyal_revision "$hiyal" \
  --arg cargo_lock_sha256 "$(sha256sum Cargo.lock | cut -d' ' -f1)" \
  --arg rustc "$rustc" --arg os "$os" --arg cpu "$cpu" \
  --argjson first_seed "$seed" \
  '{
    schema: 1, scientific_status: "exploratory_not_preregistered",
    experiment: $protocol, started_utc: $start, completed_utc: $finish,
    lab_git_sha: $lab_revision, lab_source_sha: $lab_source_revision, hiyal_git_sha: $hiyal_revision,
    cargo_lock_sha256: $cargo_lock_sha256, rust_toolchain: $rustc,
    os: $os, cpu_model: $cpu, first_seed: $first_seed, paired_societies_per_cell: 10,
    note: "Original journals, fork ancestry, RNG state, configuration and SHA256SUMS are archived. The archive is only retained for the GitHub Actions artifact retention period; no external empirical validation."
  }' > "$output/reproducibility-manifest.json"

bash scripts/verify-001.sh "$output" --before-hash
(
  cd "$output"
  find . -type f ! -name SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS
  sha256sum --check --status SHA256SUMS
)
echo "Evidence manifest sealed: $output/SHA256SUMS"
bash scripts/verify-001.sh "$output"
