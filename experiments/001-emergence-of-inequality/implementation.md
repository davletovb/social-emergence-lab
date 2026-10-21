# Experiment 001: initial implementation

**Status:** exploratory Rust model and runner. The confirmatory design is **not** frozen and no scientific conclusion is claimed.

## Method

This implementation is an integer-only, closed-population opportunity allocation model. Twenty equal-ability actors start with ten wealth tokens each. Each accepted event supplies exactly one new token to exactly one actor:

- Control (uniform): each agent's weight is 1.
- Treatment (linear reinforcement): agent weight equals current wealth + 1.

The model draws a fair bounded integer ticket with Hiyal's unbiased `Streams::uniform_index` rejection sampler. Decisions draw from `agent/<scheduled-actor>/opportunity` in Hiyal's serialized named streams, and the resulting recipient is stored as a replayable action. Scheduled actors are simply proposal opportunities; their identities do **not** force them to receive tokens.

Paired trials are created by `hiyal::research::paired_forks` from the same zero-event parent. Both arms share initial state and the source RNG domain but may use different bounded-ticket rejection patterns as their weights diverge. Do not claim perfect matched draws or independence of agents within the same society.

The primary descriptive metric is Gini across the complete agent wealth vector after a predeclared number of **accepted** opportunities. An additional top-quintile share is exported. Rank mobility, trajectories, parameter sweeps, credible uncertainty intervals and real-world calibration are **not implemented** yet.

## Run

Requires Rust 1.89+, Git access to the pinned Hiyal revision and Cargo dependencies:

```sh
cargo test --locked
cargo run --locked --release -- pilot /tmp/social-lab-pilot 100 1000 100000
cat /tmp/social-lab-pilot/results.csv
```

For a tiny integration smoke: `cargo run --locked -- pilot /tmp/social-smoke 2 15 42`.

A complete run writes a `results.csv` (one row per society/arm), `pilot-summary.json` (with the Hiyal revision read from `Cargo.lock`) and, per society, `outputs/<arm>-wealth.csv`, plus Hiyal-owned manifest, journal, snapshot, fork attribution and ancestry evidence under `parent/` and `arms/`. All these outputs are generated artifacts; none should be mistaken for raw human data.

The command restarts matching existing pairs, reconstructs summaries from verified journals and fails closed on mismatched pairs or tampered ancestry. The parent is published atomically, and a lone arm left at the fork boundary by an interrupted run is discarded and the pair re-forked; a lone arm with committed events is never replaced. Each invocation is limited to 1,000,000 pair-steps (journals use roughly 2.2 KB per step per arm). Re-running with a larger horizon appends additional accepted events; attempting a shorter horizon fails. **Never** reuse a directory for another scientific protocol or change the core model silently.

## Remaining work / gating

- [x] Standalone Rust consumer crate and fixed Hiyal Git revision.
- [x] Conservation/overflow, exact ticket allocation, Gini baseline tests in source.
- [x] Integration test exercising paired forks, uninterrupted-vs-resumed history, and tampered ancestry rejection.
- [x] GitHub Actions compilation/tests and a 2-pair smoke run; [verified run](https://github.com/davletovb/social-emergence-lab/actions/runs/39536247553).
- [x] Commit resolved `Cargo.lock` for the pinned Hiyal Git revision.
- [ ] Record the lab Git SHA and exact protocol/config hash in each exported run manifest.
- [ ] Add independent reference model, labeled permutation/symmetry tests, empirical sampler sanity and trajectory-level metrics.
- [ ] Benchmark 20-actor histories including memory, event/snapshot storage and execution cost.
- [ ] Freeze the confirmatory protocol, held-out seeds, horizons and analysis rules.
- [ ] Compute confidence intervals and sensitivity results across independent **societies**, not across agents or time steps.

## First exploratory run (non-confirmatory)

20 paired societies, 200 accepted opportunities per arm, starting at seed 2000 (`pilot <dir> 20 200 2000`, the same batch as the manual workflow). Descriptive terminal mean Gini was **0.086188** for uniform allocation and **0.110688** for linear reinforcement (paired mean difference **+0.024500**).

An earlier [GitHub Actions execution](https://github.com/davletovb/social-emergence-lab/actions/runs/39536537008) of the same batch used the crate's previous multiply-high sampler, which maps draws to tickets differently; its values (0.087475 vs 0.119425, +0.031950) are superseded and not reproducible with the current code. This run has no predeclared inference threshold, confidence interval, or validation against human data. These results are *exploratory software output*, not evidence of a real-world causal process.

No paid model use; no live provider credentials required.
