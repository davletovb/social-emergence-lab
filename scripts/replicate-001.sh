#!/usr/bin/env bash
# Produce an Experiment 001 Hiyal study with a reproducibility manifest and a
# sealed SHA256SUMS. Defaults to the 10-pair 18-cell grid; `smoke <pairs>`
# runs the cheap two-cell plan through the same pipeline (used by PR CI).
set -euo pipefail
if [[ $# -ne 2 && $# -ne 4 ]]; then
  echo "Usage: bash scripts/replicate-001.sh <new-absolute-output-dir> <first-seed> [grid|smoke <pairs>]" >&2
  exit 2
fi
output="$1"
seed="$2"
mode="${3:-grid}"
pairs="${4:-10}"
[[ "$output" = /* ]] || { echo "Output must be an absolute path" >&2; exit 2; }
[[ "$seed" =~ ^[0-9]+$ ]] || { echo "Seed must be an unsigned integer" >&2; exit 2; }
[[ "$mode" == grid || "$mode" == smoke ]] || { echo "Mode must be grid or smoke" >&2; exit 2; }
[[ "$pairs" =~ ^[0-9]+$ ]] || { echo "Pairs must be an unsigned integer" >&2; exit 2; }
if [[ -e "$output" ]]; then
  echo "Refusing existing experiment directory: $output" >&2
  exit 2
fi
for tool in jq sha256sum cargo git; do
  command -v "$tool" >/dev/null || { echo "Missing required tool: $tool" >&2; exit 2; }
done
cd "$(dirname "${BASH_SOURCE[0]}")/.."
# Untracked files (a build.rs, .cargo/config.toml) can change the build too.
[[ -z "$(git status --porcelain)" ]] || {
  echo "Source changes, including untracked files, must be committed before evidence generation" >&2
  exit 2
}
revision="$(git rev-parse HEAD)"
# A pull_request checkout is a synthetic merge that no ref keeps alive;
# record its second parent, the authored PR head (needs fetch-depth >= 2).
if [[ "${GITHUB_EVENT_NAME:-}" == pull_request ]]; then
  source_revision="$(git rev-parse --verify -q 'HEAD^2')" || {
    echo "Pull-request merge checkout needs fetch-depth >= 2 to record the PR head" >&2
    exit 2
  }
else
  source_revision="$revision"
fi

# Build before timing, so the recorded window covers only the experiment.
cargo build --locked --release --bin hiyal-grid --bin validate
bin="$(cargo metadata --locked --format-version 1 --no-deps | jq -r .target_directory)/release"
# hiyal-grid validates the plan before creating the output directory, so a
# rejected seed or pair count leaves nothing behind.
start="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"
"$bin/hiyal-grid" "$mode" "$output" "$pairs" "$seed"
finish="$(date -u '+%Y-%m-%dT%H:%M:%SZ')"

# The revision actually compiled in, as recorded by the runner itself.
hiyal="$(jq -r .hiyal_revision "$output/study-plan.json")"
[[ "$hiyal" =~ ^[0-9a-f]{40}$ ]] || { echo "Study plan lacks an exact Hiyal SHA" >&2; exit 2; }
if [[ "$mode" == grid ]]; then
  experiment="experiment-001-exploratory-replicated-v1"
else
  experiment="experiment-001-pipeline-smoke"
fi
cpu="$( (lscpu 2>/dev/null | grep -m1 '^Model name:' | cut -d: -f2- | sed 's/^ *//') || true)"
os="$(uname -srm)"
rustc="$(rustc --version --verbose)"
jq -n \
  --arg experiment "$experiment" --arg mode "$mode" \
  --arg start "$start" --arg finish "$finish" \
  --arg lab_revision "$revision" --arg lab_source_revision "$source_revision" --arg hiyal_revision "$hiyal" \
  --arg cargo_lock_sha256 "$(sha256sum Cargo.lock | cut -d' ' -f1)" \
  --arg rustc "$rustc" --arg os "$os" --arg cpu "$cpu" \
  --argjson first_seed "$seed" --argjson pairs "$pairs" \
  '{
    schema: 2, scientific_status: "exploratory_not_preregistered",
    experiment: $experiment, plan_mode: $mode, started_utc: $start, completed_utc: $finish,
    lab_git_sha: $lab_revision, lab_source_sha: $lab_source_revision, hiyal_git_sha: $hiyal_revision,
    cargo_lock_sha256: $cargo_lock_sha256, rust_toolchain: $rustc,
    os: $os, cpu_model: $cpu, first_seed: $first_seed, paired_societies_per_cell: $pairs,
    note: "Original journals, fork ancestry, RNG state, configuration and SHA256SUMS are archived. No external empirical validation."
  }' > "$output/reproducibility-manifest.json"

bash scripts/verify-001.sh "$output" --before-hash
(
  cd "$output"
  # Hiyal's empty writer-lock markers are not evidence and archivers often
  # drop hidden files, so they are excluded from the seal.
  find . -type f ! -name SHA256SUMS ! -name .writer.lock ! -name .effects-writer.lock -print0 \
    | LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS
)
echo "Evidence manifest sealed: $output/SHA256SUMS (sha256 $(sha256sum "$output/SHA256SUMS" | cut -d' ' -f1))"
bash scripts/verify-001.sh "$output"
