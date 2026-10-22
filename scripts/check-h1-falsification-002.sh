#!/usr/bin/env bash
# Derived-results-only H1 audit: checks the committed 36-parent files agree
# and prints the decomposition; no simulation, no seed selection, no
# confirmatory claims, and no assertion about which way the result came out.
set -euo pipefail
cd "$(dirname "$0")/.."
folder=experiments/002-reputation-lock-in/results/v2-36-paired-20261021
jq -n \
 --slurpfile h "$folder/h1-decomposition.json" \
 --slurpfile a "$folder/arm-summary.json" \
 --slurpfile p "$folder/paired-summary-with-uncertainty.json" \
 --slurpfile q "$folder/h1-quarters.json" '
 ($h[0].groups | map(select(.delta==0.2)) | .[0]) as $h1 |
 ($h[0].groups | map(select(.delta==0.0)) | .[0]) as $null |
 ($a[0].groups | map(select(.delta==0.2)) | .[0]) as $arm |
 ($p[0]) as $prior |
 ($q[0].groups | map(select(.delta==0.2)) | .[0]) as $quarters |
 def close($x;$y): (($x-$y)|fabs) < 0.000000001;
 if $h1 == null or $null == null or $arm == null
    or $h1.parents != 36 or $null.parents != 36
    or $prior.parent_pairs != 36
    or (close($h1.gap.mean; $prior.prior_gap.mean) | not)
    or (close($h1.new_link_plus_selection.mean;
                ($h1.new_link_exposure.mean + $h1.selection.mean)) | not)
    or (close($h1.gap.mean; $arm.arms.information.late_gap) | not)
    or (close($h1.surviving_link_exposure.mean;
                $arm.arms.information.late_inherited) | not)
    or $quarters == null or $quarters.parents != 36
    or ($quarters.quarters[-1].window != $h1.late_window)
    or (close($quarters.quarters[-1].gap.mean; $h1.gap.mean) | not)
    or (close($quarters.quarters[-1].new_link_plus_selection.mean;
                $h1.new_link_plus_selection.mean) | not)
 then error("Existing H1 results and primary study archives do not reconcile")
 else {
  scientific_status:"EXPLORATORY_MECHANISM_FALSIFICATION_DESCRIPTIVE",
  independent_parent_seeds:36,
  cohort:"N20 d4 tau200 T600 δ0.2",
  late_signed_gap:$h1.gap,
  surviving_checkpoint_edge_exposure:$h1.surviving_link_exposure,
  new_or_readded_edge_exposure:$h1.new_link_exposure,
  belief_selection_exposure:$h1.selection,
  stronger_h1_excess:$h1.new_link_plus_selection,
  stronger_h1_interval_excludes_zero:(
    $h1.new_link_plus_selection.interval_95[0] > 0
    or $h1.new_link_plus_selection.interval_95[1] < 0
  ),
  surviving_share_of_gap:($h1.surviving_link_exposure.mean / $h1.gap.mean),
  quarters:($quarters.quarters | map({window, gap:.gap.mean,
    surviving_link_exposure:.surviving_link_exposure.mean,
    new_link_plus_selection:.new_link_plus_selection.mean,
    surviving_edge_fraction, exposure_per_surviving_link})),
  no_false_prior_excess:$null.new_link_plus_selection,
  interpretation:"The observed positive late opportunity gap does not establish a self-regenerating new-edge-or-belief mechanism; stronger H1 is not established by current interval.",
  caveat:"The 95% intervals and this threshold are retrospective exploratory checks, not a registered hypothesis test or proof the effect is zero."
 } end
' 
