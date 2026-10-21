# Experiment 002 per-arm means over parents (`jq -s` over report.json files),
# grouped by configuration and schedule. Every parent runs all four arms, so a
# linear contrast between arms (D_U-D_I, D_I-D_IS, D_U-D_I-D_S+D_IS) equals the
# same contrast of these means; H2 compares abs() of the sham and structure
# full-window means. Exploratory description only, no inference.
def avg(xs): (xs | add / length);
def mean($runs; $arm; f): avg($runs | map(.per_arm[] | select(.arm == $arm) | f));
{scientific_status: "exploratory_only_no_inferential_claim",
 groups: (group_by([.config.actors, .config.degree, .config.delta, .config.beta,
                    .config.removal, .config.kappa, .checkpoint, .total_steps])
  | map(. as $runs | .[0] as $first | {
      actors: $first.config.actors, degree: $first.config.degree,
      removal: $first.config.removal, delta: $first.config.delta,
      beta: $first.config.beta, kappa: $first.config.kappa,
      checkpoint: $first.checkpoint, total_steps: $first.total_steps,
      full_window: [$first.per_arm[0].full.from, $first.per_arm[0].full.through],
      late_window: [$first.per_arm[0].late.from, $first.per_arm[0].late.through],
      parents: length, seeds: map(.seed),
      arms: (["sham", "information", "structure", "both"] | map(. as $arm | {key: $arm, value: {
        full_gap: mean($runs; $arm; .full.opportunity_gap),
        late_gap: mean($runs; $arm; .late.opportunity_gap),
        late_degree: mean($runs; $arm; .late.degree),
        late_inherited: mean($runs; $arm; .late.inherited),
        late_added: mean($runs; $arm; .late.added),
        late_selection: mean($runs; $arm; .late.selection),
        late_innovation: mean($runs; $arm; .late.innovation),
        full_rewire_events: mean($runs; $arm; .full.rewire_events)}}) | from_entries)}))}
