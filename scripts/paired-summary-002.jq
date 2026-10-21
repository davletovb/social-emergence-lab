# Experiment 002 matched false-prior batch summary over paired-report.json
# files (`jq -s`). With delta=0 the expected gap is exactly zero, so the
# false-prior gap is the primary estimate and the no-prior gap a symmetry
# check; the matched difference only helps if the two correlate strongly.
def mean: add / length;
def sd: if length < 2 then null
  else mean as $m | map((. - $m) * (. - $m)) | add / (length - 1) | sqrt end;
def corr($x; $y):
  if ($x | length) < 3 then null else
    ($x | mean) as $mx | ($y | mean) as $my
    | ([range(0; $x | length) as $i | ($x[$i] - $mx) * ($y[$i] - $my)] | add) as $sxy
    | ($x | map((. - $mx) * (. - $mx)) | add) as $sxx
    | ($y | map((. - $my) * (. - $my)) | add) as $syy
    | if $sxx == 0 or $syy == 0 then null else $sxy / (($sxx * $syy) | sqrt) end
  end;
sort_by(.seed)
| map(.prior_information_late_gap) as $prior
| map(.no_prior_information_late_gap) as $null
| map(.matched_seed_prior_effect) as $diff
| {scientific_status: "EXPLORATORY_NOT_CONFIRMATORY",
   parent_pairs: length, seeds: map(.seed),
   prior_gap: {mean: ($prior | mean), sd: ($prior | sd)},
   no_prior_gap: {mean: ($null | mean), sd: ($null | sd)},
   matched_difference: {mean: ($diff | mean), sd: ($diff | sd)},
   prior_no_prior_correlation: corr($prior; $null),
   per_pair: map({seed, prior_information_late_gap, no_prior_information_late_gap,
                  matched_seed_prior_effect, paired_repair_difference})}
