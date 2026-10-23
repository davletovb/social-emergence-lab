# Experiment 002 — journal-trace world kind

**Status:** engineering change, opt-in. Not a new experimental protocol and not
evidence. The frozen V3 scripts and every archived run keep using the V0 kind.

## Why

`ReputationWorld.trace` gains one entry per opportunity. Hiyal hashes the whole
world at every committed event, so a run of T events hashes O(T²) trace bytes.
The [V2 notes](performance-and-paired-null-v2.md) found the trace is 97% of the
final world's JSON. Hiyal's fixed per-event overhead
(davletovb/hiyal#118 and the canonical-writer follow-up) has since been
reduced, so this growth is now the main cost of long runs.

## What it is

- World kind `social-emergence-lab.reputation.journal-trace.v1`, selected by
  `Config.journal_trace` (`--journal-trace` on `reputation-pilot` and
  `reputation-paired-prior`).
- Same model, policy and actions as V0. The only difference is that the world
  keeps no `trace`, so state hashes cover O(N²) state.
- The trajectory is rebuilt from the Hiyal-verified journal by the
  independent reference replay (`reputation_reference::replay_journal_trace`),
  which also re-checks every pre-event state.
- One rule builds each trajectory entry: `StepTrace::after`, used by V0's
  `apply` and by the journal replay alike.
- V0 is unchanged. The new config field is omitted when false, so V0 configs
  serialize to the same bytes, and `KIND` and every V0/V1 hash are unchanged.
- `verify` rejects a run whose world kind does not match its configuration.
  For V0 runs it now also checks that each fork's stored trace equals the
  trajectory rebuilt from its journal.

## Equivalence checks

- **Seed 72100, N=20, d=4, τ=80, T=240, paired prior.** The V0 and
  journal-trace runs commit identical actors, actions and RNG positions in all
  2,080 events across both parents and all eight forks. The per-arm reports
  are identical, and `paired-report.json` differs only in `config.journal_trace`.
- **`tests/reputation_journal_trace.rs`** checks:
  - the same identity at N=10;
  - that the rebuilt trajectory equals the V0 stored trace;
  - that windows computed from either trajectory are equal;
  - that a journal-trace world rejects a stored trace;
  - that an edited report fails `verify`;
  - that V0 configs serialize without the new field.
- **V0 raw-evidence archive** (`80666d6f…d8ec`): still passes
  `scripts/verify-002.sh`, including the new stored-trace check. The V3 evidence
  checks still pass.

## Measurements

`reputation-paired-prior run` then `verify`, seed 72100, N=20, d=4. Same
machine: 4-core Xeon, no SHA-NI, Rust 1.89.0, release build. Median of 3
rotated repetitions, in seconds.

- "Hiyal `main`" is `55d30ea`, the current pin.
- "Hiyal + canonical writer" is the unmerged Hiyal branch with the
  direct canonical JSON writer.
- Each pair compares V0 and journal-trace within the same binary.

| τ, T | Step | V0 on `main` | Journal-trace on `main` | Speedup | V0 + canonical writer | Journal-trace + canonical writer | Speedup |
|---|---|---|---|---|---|---|---|
| 80, 240 | run | 9.50 | 6.65 | 1.43× | 7.95 | 6.42 | 1.24× |
| | verify | 3.22 | 2.15 | 1.50× | 2.55 | 2.09 | 1.22× |
| 200, 600 | run | 24.26 | 11.44 | 2.12× | 19.34 | 10.06 | 1.92× |
| | verify | 9.86 | 3.41 | 2.89× | 6.94 | 3.18 | 2.18× |
| 200, 1000 | run | 60.03 | 16.87 | 3.56× | 45.36 | 16.09 | 2.82× |
| | verify | 22.78 | 4.99 | 4.57× | 16.41 | 5.05 | 3.25× |

The speedup grows with T, as expected once per-event cost no longer grows with
the trace. With the journal-trace kind, the canonical writer adds little,
because world serialization is now small. Total bytes on disk drop only 6%,
because the event journal dominates disk use. The final fork snapshot shrinks
about 10× (62 KB to 6 KB at T=240).

## Using it

```sh
cargo run --locked --release --bin reputation-paired-prior -- run /tmp/jt-demo 72100 20 4 200 600 --journal-trace
cargo run --locked --release --bin reputation-paired-prior -- verify /tmp/jt-demo
```

`verify` needs no flag, because it reads the kind from the evidence. Any new
protocol that adopts this kind must say so in its plan. Its runs are not
byte-comparable with V0 runs, though their actions and measured windows are
identical for the same seed and configuration.
