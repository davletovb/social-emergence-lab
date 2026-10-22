#!/usr/bin/env bash
# Deterministic and CPU-cheap descriptive aggregation of three independently
# verified original Hiyal evidence artifacts; DOES NOT simulate or re-run seeds.
set -euo pipefail
[[ $# -eq 2 && "$1" == /* && "$2" == /* ]] || { echo "usage: aggregate-paired-002.sh <EXTRACTED-SHARDS-PARENT> <NEW-ABSOLUTE-OUTDIR>" >&2; exit 2; }
source_root="$1"
out="$2"
[[ -d "$source_root" && ! -e "$out" ]] || { echo "Missing shards or output already exists" >&2; exit 2; }
for cmd in jq sha256sum cmp git; do command -v "$cmd" >/dev/null || exit 2; done
cd "$(dirname "$0")/.."
mkdir -p "$out"
# Compare values, not jq's number spelling: jq versions differ on whether an
# unchanged 0.0 prints as 0.0 or 0.
canonical='walk(if type == "number" then . + 0 else . end)'
plans=()
reports=()
runs=()
: > "$out/shard-manifests.txt"
for shard in 0 1 2; do
    folder="$source_root/experiment-002-v2-paired-shard-$shard"
    [[ -s "$folder/batch-plan.json" && -s "$folder/SHA256SUMS" ]] || { echo "Missing sealed shard $shard" >&2; exit 2; }
    (cd "$folder" && sha256sum --check --strict --status SHA256SUMS) || { echo "Shard $shard failed evidence seal" >&2; exit 2; }
    jq -e --argjson shard "$shard" \
      '.layout=="paired-prior" and .shard==$shard and .actors==20 and .degree==4 and .checkpoint==200 and .total==600 and .seeds==[range(98000+12*$shard;98012+12*$shard)]' \
      "$folder/batch-plan.json" > /dev/null || { echo "Shard $shard has wrong prereported design" >&2; exit 2; }
    files=("$folder"/paired-*/paired-report.json)
    [[ "${#files[@]}" -eq 12 && -s "$folder/paired-summary.json" ]] || { echo "Missing matched pairs in $shard" >&2; exit 2; }
    cmp -s <(jq -cS "$canonical" "$folder/paired-summary.json") \
      <(jq -s -f scripts/paired-summary-002.jq "${files[@]}" | jq -cS "$canonical") || { echo "Shard $shard summary not derivable from reports" >&2; exit 2; }
    plans+=("$folder/batch-plan.json")
    reports+=("${files[@]}")
    runs+=("$folder"/paired-*/false-prior/report.json "$folder"/paired-*/no-false-prior/report.json)
    sha256sum "$folder/SHA256SUMS" >> "$out/shard-manifests.txt"
done
jq -s -e '[.[].seeds[]]|sort == [range(98000;98036)]' "${plans[@]}" >/dev/null
jq -s -e '(map(.source_revision)|unique|length)==1 and (map(.hiyal_revision)|unique|length)==1' "${plans[@]}" >/dev/null
jq -s -f scripts/paired-summary-002.jq "${reports[@]}" > "$out/paired-summary.json"
jq -e '.parent_pairs==36 and (.seeds|sort)==[range(98000;98036)]' "$out/paired-summary.json" >/dev/null
jq --argjson t 2.030108 '. + {
 precision_display:{
  meaning:"Exploratory unadjusted approximate 95% t-intervals for independent synthetic societies; not preregistered or evidence about humans",
  critical_t_df35:$t,
  prior_gap_interval:[
   (.prior_gap.mean - $t*.prior_gap.sd/(.parent_pairs|sqrt)),
   (.prior_gap.mean + $t*.prior_gap.sd/(.parent_pairs|sqrt))
  ],
  matched_difference_interval:[
   (.matched_difference.mean - $t*.matched_difference.sd/(.parent_pairs|sqrt)),
   (.matched_difference.mean + $t*.matched_difference.sd/(.parent_pairs|sqrt))
  ],
  null_symmetry_interval:[
   (.no_prior_gap.mean - $t*.no_prior_gap.sd/(.parent_pairs|sqrt)),
   (.no_prior_gap.mean + $t*.no_prior_gap.sd/(.parent_pairs|sqrt))
  ]
 }}' "$out/paired-summary.json" > "$out/paired-summary-with-uncertainty.json"
# Per-arm means and H1's late new-link plus selection term, from the 72 run
# reports the seals cover.
(( ${#runs[@]} == 72 )) || { echo "Expected 72 run reports" >&2; exit 2; }
jq -s -f scripts/arm-summary-002.jq "${runs[@]}" > "$out/arm-summary.json"
jq -s --argjson t 2.030108 -f scripts/h1-decomposition-002.jq "${runs[@]}" > "$out/h1-decomposition.json"
jq -s --argjson t 2.030108 -f scripts/h1-quarters-002.jq "${runs[@]}" > "$out/h1-quarters.json"
jq -s '{
 scientific_status:"EXPLORATORY_ONLY_ALL_36_SHARDS_VERIFIED",
 source_commit:.[0].source_revision,hiyal_revision:.[0].hiyal_revision,
 seeds:[range(98000;98036)],parent_pairs:36,
 provenance:"3 separate exact-seed 12-pair Hiyal shards: sealed in shards and replayed there; aggregated here from checksum-verified original artifacts",
 caution:"This aggregate verifies SHA256 seals and derivation of summaries; expensive parent/fork journal replays were performed by each original shard CI job and must be repeatable from artifacts"
}' "${plans[@]}" > "$out/aggregation-plan.json"
(cd "$out"; find . -type f ! -name SHA256SUMS -print0 | LC_ALL=C sort -z | xargs -0 sha256sum > SHA256SUMS; sha256sum --check --strict --status SHA256SUMS)
echo "BEGIN-V2-36-PAIR-SUMMARY"
cat "$out/paired-summary-with-uncertainty.json"
echo "END-V2-36-PAIR-SUMMARY"
echo "BEGIN-V2-36-PAIR-H1"
cat "$out/h1-decomposition.json"
echo "END-V2-36-PAIR-H1"
echo "BEGIN-V2-36-PAIR-SEAL"
sha256sum "$out/SHA256SUMS"
echo "END-V2-36-PAIR-SEAL"
