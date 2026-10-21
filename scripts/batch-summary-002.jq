# Experiment 002 batch summary over per-run report.json files (`jq -s`).
# Kept byte-compatible with the first development batch; add new fields to
# arm-summary-002.jq instead.
def avg(xs): (xs | add/length);
{scientific_status:"exploratory_only_no_inferential_claim",
 groups:(group_by([.config.actors,.config.degree,.config.delta,.config.beta,.config.removal,.config.kappa])
 | map({actors:.[0].config.actors,degree:.[0].config.degree,
   removal:.[0].config.removal,delta:.[0].config.delta,beta:.[0].config.beta,
   kappa:.[0].config.kappa,parents:length,seeds:map(.seed),
   mean_information_late_gap:avg(map(.primary_information_late_gap)),
   mean_repair_after_truth_late:avg(map(.repair_after_truth_late)),
   mean_repair_without_truth_full:avg(map(.repair_without_truth_full))}))}
