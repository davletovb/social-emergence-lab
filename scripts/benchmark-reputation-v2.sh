#!/usr/bin/env bash
# Same-runner benchmark and deterministic action-journal equivalence gate.
# Baseline: V1 merged commit. Optimized: checked-out V2 research head.
# NOT a sociological experiment; no cherry-picked seeds or speed claims without runs.
set -euo pipefail
[[ $# -eq 1 && "$1" == /* ]] || { echo "usage: benchmark-reputation-v2.sh <NEW-ABSOLUTE-OUTPUT-DIR>" >&2; exit 2; }
output="$1"
[[ ! -e "$output" ]] || { echo "Refusing existing results directory" >&2; exit 2; }
for cmd in git cargo jq date du cmp sha256sum; do command -v "$cmd" >/dev/null || exit 2; done
cd "$(dirname "$0")/.."
[[ -z "$(git status --porcelain)" ]] || { echo "Benchmark requires clean source checkout" >&2; exit 2; }
baseline="fb716dee97b2bfdb0e234c4c747e5c890cc1ea01"
optimized="$(git rev-parse HEAD)"
if [[ "${GITHUB_EVENT_NAME:-}" == "pull_request" ]]; then optimized="$(git rev-parse HEAD^2)"; fi
mkdir -p "$output"
workspace="$(mktemp -d /tmp/reputation-v2-same-runner.XXXXXXXX)"
trap 'git worktree remove -f "$workspace/base" >/dev/null 2>&1 || :; git worktree remove -f "$workspace/new" >/dev/null 2>&1 || :; rm -rf "$workspace"' EXIT
git fetch --no-tags --quiet --depth=1 origin "$baseline" "$optimized"
git worktree add --quiet --detach "$workspace/base" "$baseline"
git worktree add --quiet --detach "$workspace/new" "$optimized"
# Build costs excluded, but both binaries are separately compiled on the same
# runner with the same installed Rust toolchain and locked Hiyal revision.
for mode in base new; do
  (
    cd "$workspace/$mode"
    CARGO_TARGET_DIR="$workspace/target-$mode" cargo build --quiet --locked --release --bin reputation-pilot
  )
done
binary_base="$workspace/target-base/release/reputation-pilot"
binary_new="$workspace/target-new/release/reputation-pilot"
printf '%s\n' '{"scientific_status":"ENGINEERING_EQUIVALENCE_BENCHMARK","baseline_commit":"fb716dee97b2bfdb0e234c4c747e5c890cc1ea01","optimized_commit":"'"$optimized"'","measurement":"monotonic wall-time approximated by date +%s%N; same runner; release binaries; time excludes compile; 1 seed per size and no uncertainty interval","limitations":"single runner and tiny cases, I/O and cache effects; do not generalize speed ratio"}' > "$output/method.json"
: > "$output/measurements.jsonl"
for spec in "10 3 40 120 92550" "20 4 80 240 92551"; do
    read -r n d checkpoint total seed <<< "$spec"
    label="n${n}-d${d}-${seed}"
    # Separate file roots for each mode, never overwrite. Both run+verify
    # independently in a deterministic parent and 4-fork Hiyal layout.
    for mode in base new; do
        trial="$workspace/$label-$mode"
        binary="$binary_base"
        if [[ "$mode" == new ]]; then binary="$binary_new"; fi
        start="$(date +%s%N)"
        "$binary" "$trial" "$seed" "$n" "$d" "$checkpoint" "$total" default >/dev/null
        stop="$(date +%s%N)"
        run_ms="$(((stop-start)/1000000))"
        start="$(date +%s%N)"
        "$binary" verify "$trial" >/dev/null
        stop="$(date +%s%N)"
        verify_ms="$(((stop-start)/1000000))"
        bytes="$(du -sb "$trial" | cut -f1)"
        jq -n --arg label "$label" --arg mode "$mode" --argjson n "$n" --argjson seed "$seed" \
          --argjson run_ms "$run_ms" --argjson verify_ms "$verify_ms" --argjson bytes "$bytes" \
          '{case:$label,mode:$mode,actors:$n,seed:$seed,run_ms:$run_ms,verify_ms:$verify_ms,evidence_bytes:$bytes}' >> "$output/measurements.jsonl"
    done
    # Exact journal equivalence is stronger than merely equal mean effects.
    for segment in parent sham information structure both; do
        cmp "$workspace/$label-base/$segment/events.jsonl" "$workspace/$label-new/$segment/events.jsonl" || {
            echo "EVENT JOURNAL DIFFERENCE for $label $segment" >&2; exit 1;
        }
    done
    cmp "$workspace/$label-base/report.json" "$workspace/$label-new/report.json" || {
        echo "REPORT DIFFERENCE for $label" >&2; exit 1;
    }
    cp "$workspace/$label-new/report.json" "$output/$label.report.json"
done
jq -s --arg optimized "$optimized" '
{scientific_status:"ENGINEERING_ONLY_NO_SCIENTIFIC_INFERENCE",
 baseline:"fb716dee97b2bfdb0e234c4c747e5c890cc1ea01",optimized:$optimized,
 same_runner:true,release:true,byte_identical_parent_and_four_fork_event_journals:true,
 cases:(group_by(.case)|map({
 case:.[0].case,actors:.[0].actors,seed:.[0].seed,
 baseline_run_ms:(map(select(.mode=="base"))|.[0].run_ms),
 optimized_run_ms:(map(select(.mode=="new"))|.[0].run_ms),
 baseline_verify_ms:(map(select(.mode=="base"))|.[0].verify_ms),
 optimized_verify_ms:(map(select(.mode=="new"))|.[0].verify_ms),
 baseline_evidence_bytes:(map(select(.mode=="base"))|.[0].evidence_bytes),
 optimized_evidence_bytes:(map(select(.mode=="new"))|.[0].evidence_bytes)
 }))}
' "$output/measurements.jsonl" > "$output/benchmark-summary.json"
(cd "$output"; find . -type f ! -name SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS; sha256sum --check --status --strict SHA256SUMS)
echo "BEGIN-V2-BENCHMARK-SUMMARY"
cat "$output/benchmark-summary.json"
echo "END-V2-BENCHMARK-SUMMARY"
echo "BEGIN-V2-BENCHMARK-SEAL"
sha256sum "$output/SHA256SUMS"
echo "END-V2-BENCHMARK-SEAL"
