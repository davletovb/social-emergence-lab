# H1 decomposition of the late information-only gap (`jq -s` over per-run
# report.json files, with --argjson t <critical t>). The formal model calls a
# late gap carried only by checkpoint links still present "slow relaxation";
# lock-in needs the late new-link plus selection term to be positive.
def mean: add / length;
def sd: mean as $m | map((. - $m) * (. - $m)) | add / (length - 1) | sqrt;
def describe: {mean: mean, sd: sd,
  interval_95: [mean - $t * sd / (length | sqrt), mean + $t * sd / (length | sqrt)]};
group_by(.config.delta)
| map(map(.per_arm[] | select(.arm == "information") | .late) as $late
  | {delta: .[0].config.delta, parents: length,
     late_window: [$late[0].from, $late[0].through],
     gap: ($late | map(.opportunity_gap) | describe),
     surviving_link_exposure: ($late | map(.inherited) | describe),
     new_link_exposure: ($late | map(.added) | describe),
     selection: ($late | map(.selection) | describe),
     new_link_plus_selection: ($late | map(.added + .selection) | describe),
     surviving_edge_fraction: ($late | map(.mean_surviving_edge_fraction) | mean)})
| {scientific_status: "EXPLORATORY_NOT_CONFIRMATORY",
   rule: "Formal-model H1: a late gap carried only by surviving checkpoint links is slow relaxation; lock-in needs new_link_plus_selection > 0",
   critical_t: $t, groups: .}
