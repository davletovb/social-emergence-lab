# V3 decision rule, Amendment 1, for ONE cell (`jq -s --argjson t <t> --argjson
# margin <m> --argjson cv_t <t>` over that cell's per-parent cell-summary.json
# files; cv_t is the 95% t critical value at parents - 2 df). Adopted
# 2026-10-22 after Stage A and before any Stage B run; see
# v3-protocol-20261022.md. Stage A keeps the frozen v3-decision-002.jq.
#
# Same quantities as the frozen rule, with two changes:
# - a fourth category, compensation, for an nps interval entirely below 0;
# - distribution-free checks: exact order-statistic median intervals, exact
#   sign tests, the same classification on medians, and per-window intervals.
#   At delta=0 the sign test is exact, since label symmetry makes every null
#   quantity symmetric about 0; it is the null control's primary flag.
# It also reports nps adjusted by its own parent's delta=0 nps as a control
# variate: the null mean is exactly 0, so the regression intercept estimates
# the same mean with whatever variance the two sides share removed.
def mean: add / length;
def sd: mean as $m | map((. - $m) * (. - $m)) | add / (length - 1) | sqrt;
def describe: {mean: mean, sd: sd,
  interval_95: [mean - $t * sd / (length | sqrt), mean + $t * sd / (length | sqrt)]};
def within: .interval_95[0] >= -$margin and .interval_95[1] <= $margin;
def contains_zero: .interval_95[0] <= 0 and .interval_95[1] >= 0;
def classify($nps; $excess):
  if $nps.interval_95 == null or $excess.interval_95 == null then "inconclusive"
  elif ($nps | contains_zero) and ($nps | within) and ($excess | within) then "slow_relaxation"
  elif $nps.interval_95[0] > 0 then "regeneration"
  elif $nps.interval_95[1] < 0 then "compensation"
  else "inconclusive" end;
def excess:
  .windows[0] as $first | .late as $late
  | ($late.inherited_exposure + $late.new_link_exposure + $late.conditional_selection)
    - ($first.inherited_exposure / $first.surviving_fraction) * $late.surviving_fraction;
# P(X <= i) for X ~ Binomial($n, 1/2), i = 0..$n.
def half_binomial_cdf($n):
  reduce range(0; $n) as $i ({pmf: pow(0.5; $n), cdf: [pow(0.5; $n)]};
    .pmf = .pmf * ($n - $i) / ($i + 1) | .cdf += [.cdf[-1] + .pmf]) | .cdf;
# Median and its exact distribution-free 95% interval [x_(k), x_(n+1-k)], with
# the largest k whose coverage 1 - 2 P(X <= k-1) is at least 0.95. Below six
# parents no such k exists and the interval is null.
def median_summary:
  sort as $x | length as $n | half_binomial_cdf($n) as $cdf
  | ([range(1; ($n / 2 | floor) + 1) | select($cdf[. - 1] <= 0.025)] | last) as $k
  | {median: (if $n % 2 == 1 then $x[($n - 1) / 2] else ($x[$n / 2 - 1] + $x[$n / 2]) / 2 end)}
    + if $k == null then {interval_95: null, order_statistics: null, coverage: null}
      else {interval_95: [$x[$k - 1], $x[$n - $k]], order_statistics: [$k, $n + 1 - $k],
            coverage: (1 - 2 * $cdf[$k - 1])} end;
# Intercept of the least-squares fit of $y on $x, with a t interval at n - 2 df.
def control_variate($y; $x):
  ($x | length) as $n | ($x | mean) as $mx | ($y | mean) as $my
  | ([range(0; $n) | ($x[.] - $mx) * ($x[.] - $mx)] | add) as $sxx
  | ([range(0; $n) | ($x[.] - $mx) * ($y[.] - $my)] | add) as $sxy
  | ($sxy / $sxx) as $b | ($my - $b * $mx) as $a
  | ([range(0; $n) | ($y[.] - $a - $b * $x[.]) | . * .] | add / ($n - 2)) as $s2
  | ($s2 * (1 / $n + $mx * $mx / $sxx) | sqrt) as $se
  | {coefficient: $b, mean: $a, se: $se, interval_95: [$a - $cv_t * $se, $a + $cv_t * $se]};
# Exact two-sided sign test of a zero median; exact zeros are dropped.
def sign_test:
  (map(select(. > 0)) | length) as $positive | (map(select(. < 0)) | length) as $negative
  | {positive: $positive, negative: $negative,
     p_two_sided: (if $positive + $negative == 0 then 1
       else [1, 2 * half_binomial_cdf($positive + $negative)[[$positive, $negative] | min]] | min end)};
if length < 3
   or (map(.mechanism) | unique | length) != 1
   or (map(.rewire_every) | unique | length) != 1
   or (map(.parent_seed) | unique | length) != length
then error("an amended V3 decision needs at least three distinct parents from one cell")
else
  map(.information_only.delta_0_2.late.new_link_plus_selection) as $nps_values
  | map(.information_only.delta_0_2 | excess) as $excess_values
  | map(.information_only.delta_0.late.new_link_plus_selection) as $null_values
  | ($nps_values | describe) as $nps
  | ($excess_values | describe) as $excess
  | ($nps_values | median_summary) as $nps_median
  | ($excess_values | median_summary) as $excess_median
  | ($null_values | describe) as $null_nps
  | ($null_values | sign_test) as $null_sign
  | classify($nps; $excess) as $classification
  | classify($nps_median; $excess_median) as $median_classification
  | {status: "EXPLORATORY_V3_DECISION_RULE_AMENDMENT_1",
     mechanism: .[0].mechanism, rewire_every: .[0].rewire_every,
     parents: length, seeds: map(.parent_seed), critical_t: $t, margin: $margin,
     late_gap: (map(.information_only.delta_0_2.late.signed_gap) | describe),
     new_link_plus_selection: $nps,
     structural_excess: $excess,
     classification: $classification,
     frozen_rule_classification: (if $classification == "compensation" then "inconclusive" else $classification end),
     regeneration_exceeds_margin: ($nps.interval_95[0] > 0 and $nps.mean > $margin),
     compensation_exceeds_margin: ($nps.interval_95[1] < 0 and $nps.mean < -$margin),
     robustness: {
       new_link_plus_selection: ($nps_median + {sign_test: ($nps_values | sign_test)}),
       structural_excess: $excess_median,
       median_classification: $median_classification,
       agrees: ($classification == $median_classification),
       null_adjusted_new_link_plus_selection: control_variate($nps_values; $null_values),
       windows: [range(0; 4) as $w
         | {from: .[0].information_only.delta_0_2.windows[$w].from,
            through: .[0].information_only.delta_0_2.windows[$w].through,
            new_link_plus_selection: (map(.information_only.delta_0_2.windows[$w].new_link_plus_selection) | describe)}]},
     null_control: {new_link_plus_selection: $null_nps,
       median: ($null_values | median_summary),
       sign_test: $null_sign,
       symmetry_flag: ($null_sign.p_two_sided < 0.05),
       symmetry_flag_t: (($null_nps | contains_zero) | not)}}
end
