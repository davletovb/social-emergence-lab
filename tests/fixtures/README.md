# Test fixtures

`reputation-v0-bc870fe-n6-default-81000.tar.gz` is one run (parent and four
forks, writer-lock markers omitted) of the Experiment 002 V0 development batch,
produced by `scripts/pilot-002.sh` at commit `bc870fe` with Rust 1.89.0 on
Linux x86-64. That batch's `SHA256SUMS` digest equals the archived batch's
(`80666d6f…`, issue #12), so these files are byte-identical to the archived
run; `reputation-v0-bc870fe-n6-default-81000.sha256` holds their entries from
that seal. Check after extracting: `sha256sum --check --strict <that file>`.

`tests/reputation_v0.rs` verifies it with the current code, so a change that
can no longer replay V0 evidence fails the tests.
