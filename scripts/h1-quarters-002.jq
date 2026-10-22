# Quarterly H1 decomposition of the information-only fork (`jq -s` over
# per-run report.json files, with --argjson t <critical t>). Under slow
# relaxation the gap is carried by surviving checkpoint links, so exposure
# per surviving link stays constant while new links plus selection stay
# near zero; regeneration would push the gap above that.
def mean: add / length;
def sd: mean as $m | map((. - $m) * (. - $m)) | add / (length - 1) | sqrt;
def describe: {mean: mean, sd: sd,
  interval_95: [mean - $t * sd / (length | sqrt), mean + $t * sd / (length | sqrt)]};
group_by(.config.delta)
| map(map(.per_arm[] | select(.arm == "information") | .quarters) as $runs
  | {delta: .[0].config.delta, parents: length,
     quarters: [range(0; $runs[0] | length) as $k | ($runs | map(.[$k])) as $w
       | {window: [$w[0].from, $w[0].through],
          gap: ($w | map(.opportunity_gap) | describe),
          surviving_link_exposure: ($w | map(.inherited) | describe),
          new_link_plus_selection: ($w | map(.added + .selection) | describe),
          surviving_edge_fraction: ($w | map(.mean_surviving_edge_fraction) | mean),
          exposure_per_surviving_link:
            (($w | map(.inherited) | mean) / ($w | map(.mean_surviving_edge_fraction) | mean))}]})
| {scientific_status: "EXPLORATORY_NOT_CONFIRMATORY", critical_t: $t, groups: .}
