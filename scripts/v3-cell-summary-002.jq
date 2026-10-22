# Run with jq -n --slurpfile prior paired-report.json
# --slurpfile with false-prior/report.json
# --slurpfile null no-false-prior/report.json
# --arg mode, --argjson period and --argjson seed.
def information: .per_arm | map(select(.arm == "information")) | .[0];
def measurement: {
  from, through, signed_gap: .opportunity_gap,
  inherited_exposure: .inherited, new_link_exposure: .added,
  conditional_selection: .selection,
  new_link_plus_selection: (.added + .selection),
  realized_innovation: .innovation,
  surviving_fraction: .mean_surviving_edge_fraction,
  inherited_per_surviving_edge: (
    if .mean_surviving_edge_fraction != null and .mean_surviving_edge_fraction > 0
    then .inherited / .mean_surviving_edge_fraction else null end
  ),
  rewires: .rewire_events
};
($prior[0]) as $pair |
($with[0]) as $positive |
($null[0]) as $zero |
($positive | information) as $p |
($zero | information) as $z |
if $pair.seed != $seed or $positive.seed != $seed or $zero.seed != $seed
   or $pair.config.rewire_every != $period
   or $positive.config.rewire_every != $period
   or $positive.config != $pair.config
   or $zero.config.delta != 0
   or $positive.config.delta != 0.2
   or $positive.config.beta != 4 or $positive.config.epsilon != 0.1
   or $positive.config.actors != 20 or $positive.config.degree != 4
   or $positive.checkpoint != 200 or $positive.total_steps != 600
   or ($p.quarters | length) != 4
   or ($z.quarters | length) != 4
   or $pair.matched_setup != true
   or ($mode == "random-turnover" and ($positive.config.removal != "uniform" or $positive.config.kappa != 0))
   or ($mode == "belief-deletion" and ($positive.config.removal != "lowest_belief" or $positive.config.kappa != 0))
   or ($mode == "full-feedback" and ($positive.config.removal != "lowest_belief" or $positive.config.kappa != 6))
   or (["random-turnover","belief-deletion","full-feedback"] | index($mode)) == null
then error("V3 cell failed design/provenance checks")
else {
  status: "EXPLORATORY_FIXED_SEED_FEASIBILITY_NOT_CONFIRMATORY",
  mechanism: $mode,
  rewire_every: $period,
  parent_seed: $seed,
  hiyal_revision: $positive.hiyal_revision,
  information_only: {
    delta_0_2: {late: ($p.late | measurement), windows: ($p.quarters | map(measurement))},
    delta_0: {late: ($z.late | measurement), windows: ($z.quarters | map(measurement))}
  },
  late_false_prior_minus_null: {
    signed_gap: ($p.late.opportunity_gap - $z.late.opportunity_gap),
    new_link_plus_selection: (
      ($p.late.added + $p.late.selection) -
      ($z.late.added + $z.late.selection)
    )
  },
  interpretation: "One independently seeded society per cell. No statistical inference; do not declare reputation lock-in from a positive total gap."
} end
