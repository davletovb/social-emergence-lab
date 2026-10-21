# Experiment 001 protocol checklist

**Status: draft / not preregistered / not executed.**

## Before implementation
- [ ] Choose W0, weight offset, numeric normalization and observation checkpoints.
- [ ] Specify treatment versions, feedback range and primary analysis horizon.
- [ ] Define terminal-Gini implementation and tie policy; independently verify against hand-computed cases.
- [ ] Define whether an opportunity counts as an attempt or accepted event (use accepted opportunities for analysis).
- [ ] Document identification, unit of analysis, confirmatory estimand and excluded-run rule.
- [ ] Predefine seeds for development, exploratory pilot and held-out confirmation (non-overlapping sets).

## Engineering validation
- [ ] Standalone Rust consumer crate builds against a pinned Hiyal revision.
- [ ] Unit tests and independent reference implementation pass.
- [ ] State accounting: sum(wealth) = N * W0 + accepted_opportunities.
- [ ] Zero-feedback recipient distribution sanity and label-symmetry tests pass.
- [ ] Checkpoint/resume and independently validated event replay agree.
- [ ] CPU time, memory, I/O, data size and disk behavior measured on real 20-agent histories.
- [ ] No paid provider calls in V0.

## Pilot then preregistration
- [ ] Run exploratory pilot; report all conditions and measured variability.
- [ ] Decide replication count using a design-based precision/power argument.
- [ ] Freeze code, protocol, seeds, hashes, and analysis script before confirmation.
- [ ] Execute confirmation without adapting hypothesis or exclusions.
- [ ] Publish run-level estimates, uncertainty, robustness checks and limitations.

## Artifact schema (planned)
Each run should be identifiable by study ID, protocol version, lab commit SHA, Hiyal commit SHA, toolchain, configuration hash, seed, arm, status, attempts, accepted steps, event-log digest, checkpoint digest, metric digest and output path.

Raw data: append-only actor wealth observations keyed by trial, accepted step and actor ID. Derived data: one row per society with terminal Gini and secondary metrics. Keep raw and derived separated.
