# Experiment 002 — V0 development batch evidence

**Status:** 12 predeclared, synthetic, exploratory parent societies; no preregistration, no inference or human data. Run **[39602086513](https://github.com/davletovb/social-emergence-lab/actions/runs/39602086513)** on October 21, 2026. Model source revision `bc870fe5ca75ad627421838b24fb1375eceaffdb` and Hiyal revision `4bb53aca5cffa6bc01b7558efbd690af7dbed79f`.

The sealed Hiyal raw-bundle artifact `experiment-002-v0-dev-batch` is GitHub Actions artifact **12347080662**. Outer artifact digest (reported by GitHub): SHA-256 `ec8b1b15cbfaec6f3263025421f31ca3de423c533da36392f37aa20323912fab`, ZIP compressed size **862,502 bytes**. GitHub's temporary artifact **expires 2026-11-04 12:46:41 UTC**. It has *not* been archived permanently. The raw bundle contains independent parent journals, 4 forks per parent, event histories, snapshots, fork ancestry, per-arm reports, `batch-plan.json`, `batch-summary.json`, and internal `SHA256SUMS`.

The archived `SHA256SUMS` file's SHA-256 digest is `80666d6f9329b9e332078d57cdd7fddfea5619948a759e9f5d16c5a6fe12d8ec`. Writer-lock markers were excluded from the seal because they are not experimental data. All 12 parent reports include complete Hiyal fork and parent inspection/replay; the batch log recorded the seal check before upload. After extracting a preserved copy, verify with `sha256sum --check --strict SHA256SUMS`, then verify each Hiyal trial and fork with the pinned model version. These derived files alone do **not** preserve the event-level evidence.

## Design and interpretation

- Four independent parent seeds (81000–81003) at 6 actors, 2 graph links; four (82000–82003) at 10 actors, 3 links. Checkpoint 20 events, total 60 events each; all four counterfactual arms per parent. These are two *different configurations*, not one 8-parent inferential sample.
- Four further one-seed, 6-actor controls: no-false-prior 83000, no-selection-bias 83001, fixed-graph 83002, random-churn 83003.
- Full signed gap `D` = share of new opportunities received by randomized boost labels minus share received by nonboost labels, in [-1,1]. The reported repair-after-truth difference is `D_I-D_IS`, paired within the same parent. No p-values, significance claims or power estimates are justified here.
- Control branches have **one parent seed each**; departures from zero do not falsify a symmetric expected-value null.
- With a short 40-event postintervention window, the `late` statistic uses **all 40 events**, not the 100-event late window of the canonical plan. This batch checks engineering and direction only, **not the study's H1 recovery-time hypothesis**.
- Results must be followed by independent reference analysis, graph acceptance benchmarks, multiple society-scale/turnover sensitivity experiments, and genuinely held-out preregistration only if scientifically justified.

See [model source](../../../../src/reputation.rs), [pilot runner](../../../../src/reputation_pilot.rs), [protocol](../../formal-model.md), and [scientific limitations](../../implementation-v0.md).
