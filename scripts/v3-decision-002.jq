# Frozen V3 decision rule for ONE cell (`jq -s --argjson t <t> --argjson margin <m>`
# over that cell's per-parent cell-summary.json files). Frozen 2026-10-22,
# before any V3 outcome; see v3-protocol-20261022.md.
#
# Per parent, at delta=0.2, information-only fork:
#   nps    = late new-link exposure + conditional selection
#   c      = first-window inherited exposure / first-window surviving fraction
#   excess = late expected gap (inherited + new + selection) - c * late surviving fraction
# Slow relaxation predicts both are near zero: the gap is carried by surviving
# checkpoint links at a constant exposure per link.
def mean: add / length;
def sd: mean as $m | map((. - $m) * (. - $m)) | add / (length - 1) | sqrt;
def describe: {mean: mean, sd: sd,
  interval_95: [mean - $t * sd / (length | sqrt), mean + $t * sd / (length | sqrt)]};
def within: .interval_95[0] >= -$margin and .interval_95[1] <= $margin;
def contains_zero: .interval_95[0] <= 0 and .interval_95[1] >= 0;
def excess:
  .windows[0] as $first | .late as $late
  | ($late.inherited_exposure + $late.new_link_exposure + $late.conditional_selection)
    - ($first.inherited_exposure / $first.surviving_fraction) * $late.surviving_fraction;
if length < 2
   or (map(.mechanism) | unique | length) != 1
   or (map(.rewire_every) | unique | length) != 1
   or (map(.parent_seed) | unique | length) != length
then error("a V3 decision needs at least two distinct parents from one cell")
else
  (map(.information_only.delta_0_2.late.new_link_plus_selection) | describe) as $nps
  | (map(.information_only.delta_0_2 | excess) | describe) as $excess
  | (map(.information_only.delta_0.late.new_link_plus_selection) | describe) as $null_nps
  | {status: "EXPLORATORY_V3_FROZEN_DECISION_RULE",
     mechanism: .[0].mechanism, rewire_every: .[0].rewire_every,
     parents: length, seeds: map(.parent_seed), critical_t: $t, margin: $margin,
     late_gap: (map(.information_only.delta_0_2.late.signed_gap) | describe),
     new_link_plus_selection: $nps,
     structural_excess: $excess,
     classification: (
       if ($nps | contains_zero) and ($nps | within) and ($excess | within) then "slow_relaxation"
       elif $nps.interval_95[0] > 0 then "regeneration"
       else "inconclusive" end),
     regeneration_exceeds_margin: ($nps.interval_95[0] > 0 and $nps.mean > $margin),
     null_control: {new_link_plus_selection: $null_nps,
       symmetry_flag: (($null_nps | contains_zero) | not)}}
end
