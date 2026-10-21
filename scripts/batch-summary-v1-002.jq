# Experiment 002 V1 batch summary over per-run report.json files (`jq -s`).
# Kept byte-compatible with the V1 development batch; per-arm means are in
# arm-summary-002.jq.
def m: add/length;
def sd: if length<=1 then null else (m) as $mean | (map((.-$mean)*(.-$mean))|add/(length-1)|sqrt) end;
[ .[]|{seed, config,primary_information_late_gap,repair_after_truth_late,repair_without_truth_full,
    information_late:(.per_arm[]|select(.arm=="information")|.late),
    combined_late:(.per_arm[]|select(.arm=="both")|.late)
} ] |
group_by([.config.actors,.config.degree,.config.annealed,.config.removal,.config.beta,.config.kappa,.config.rewire_every,.config.delta]) |
{scientific_status:"exploratory_only_no_inferential_claim",groups:map({
 config:.[0].config,
 parents:length,seeds:map(.seed),
 info_gap_mean:(map(.primary_information_late_gap)|m),
 info_gap_sd:(map(.primary_information_late_gap)|sd),
 repair_after_truth_mean:(map(.repair_after_truth_late)|m),
 repair_after_truth_sd:(map(.repair_after_truth_late)|sd),
 repair_without_truth_mean:(map(.repair_without_truth_full)|m),
 info_degree_mean:(map(.information_late.degree)|m),
 info_selection_mean:(map(.information_late.selection)|m),
 info_inherited_mean:(map(.information_late.inherited)|m),
 info_added_mean:(map(.information_late.added)|m),
 info_survival_mean:(map(.information_late.mean_surviving_edge_fraction)|m),
 combined_gap_mean:(map(.combined_late.opportunity_gap)|m)
})}