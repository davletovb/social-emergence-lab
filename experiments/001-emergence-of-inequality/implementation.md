# Experiment 001: initial implementation

**Status:** exploratory Rust model and runner. The confirmatory design is **not** frozen and no scientific conclusion is claimed.

## Method

This implementation is an integer-only, closed-population opportunity allocation model. Twenty equal-ability actors start with ten wealth tokens each. Each accepted event supplies exactly one new token to exactly one actor:

- Control (uniform): each agent's weight is 1.
- Treatment (linear reinforcement): agent weight equals current wealth + 1.

The model uses exact multiply-high rejection sampling for a fair bounded integer ticket. Decisions draw from `agent/<scheduled-actor>/opportunity` in Hiyal's serialized named streams, and the resulting recipient is stored as a replayable action. Scheduled actors are simply proposal opportunities; their identities do **not** force them to receive tokens.

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

A complete run writes a `results.csv` (one row per society/arm), `pilot-summary.json` and, per arm, `wealth.csv`, plus Hiyal-owned manifest, journal, snapshot, fork attribution and ancestry evidence. All these outputs are generated artifacts; none should be mistaken for raw human data.

The command restarts matching existing pairs, reconstructs summaries from verified journals and fails closed on partial/mismatched pairs or tampered ancestry. Re-running with a larger horizon appends additional accepted events; attempting a shorter horizon fails. **Never** reuse a directory for another scientific protocol or change the core model silently.

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

[GitHub Actions execution](https://github.com/davletovb/social-emergence-lab/actions/runs/39536537008) successfully ran 20 paired societies, 200 accepted opportunities per arm, starting at seed 2000. Descriptive terminal mean Gini was **0.087475** for uniform allocation and **0.119425** for linear reinforcement (paired mean difference **+0.031950**). This run has no predeclared inference threshold, confidence interval, or validation against human data. These results are *exploratory software output*, not evidence of a real-world causal process.

No paid model use; no live provider credentials required.
