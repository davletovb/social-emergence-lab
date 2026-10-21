# Experiment 001: candidate confirmatory protocol (DRAFT — NOT REGISTERED)

**Status:** candidate only. All existing runs and the 10-pair replicated Hiyal grid are exploratory; they must not be relabeled as confirmatory. The experiment is a toy mechanism simulation, not a study of human participants.

## Question and estimand

Can wealth-weighted allocation of otherwise identical opportunities create greater inequality than uniform random allocation among identical artificial actors? The experimental unit is an **entire independently seeded society**. The primary estimand is the across-society average difference in terminal wealth Gini, treatment minus control, when the two arms start from the same initial parent state.

Candidate exact specification to freeze **before** any held-out testing:
- Twenty fixed artificial actors, each with initial wealth 10 integer tokens.
- One accepted event allocates exactly one new wealth token.
- Uniform control: each recipient weight 1.
- Treatment: recipient weight current wealth + 1 (linear feedback with offset 1).
- Candidate primary horizon: **500 accepted opportunities** in each arm, matching the pilot sample that supports our variance estimate; 1,000 steps remains a separate, unpowered alternative requiring its own pilot.
- Candidate **100 paired societies**, based on a deliberately conservative pilot-based approximation, **not a finalized power result**. Doubling the observed 10-pair SD gives a planning SD of ~0.04484; estimated N=78 for a ±0.01 half-width and N=53 for an approximately 90%-power test of a 0.02 Gini-unit effect. See [sample-size analysis](sample-size-and-registration.md). A separately reviewed confirmatory runner is required; the exploratory grid is capped at ten pairs.
- Within each pair, Hiyal paired-fork identities share initial world and RNG seed-domain. Different arm weights can alter rejection counts and RNG consumption: they do **not** guarantee common identical random recipients.

## Inferential analysis — still to freeze

Calculate one terminal Gini difference per paired society; estimate the sample mean and its uncertainty **across independent societies**. Candidate method: 95% paired bootstrap percentile interval (with bootstrap resamples and seed fixed in advance), with a separately defined paired randomization/permutation analysis if its exchangeability assumptions are justified. Predefine directional/two-sided hypothesis, alpha, method of treating zero, finite precision, stopping and decision rule. Secondary trajectory Gini, top share, and tie-aware rank persistence/movement are explicitly descriptive and will be labeled for multiplicity. Never treat agents or time points as independent observations.

## Negative controls and exclusions — to freeze

Use exact equality, no-op and probability-symmetry tests, an equal-weight/zero-feedback negative control and conservation identity \`sum(wealth) = actors × starting_wealth + accepted_events\`. Any failed Hiyal replay/ancestry check, unexpected rejection or checksum mismatch aborts the complete batch; do **not** drop inconvenient society outcomes. Retain failure logs.

Random seeds and complete generator algorithm, source revisions, model parameters, analysis hashes, sampling plan and precommitted exclusions must be frozen in a public, immutable revision **before** confirmatory runs. Seeds used in previous pilot, reference sensitivity, and the replicated Hiyal study cannot be reused as held-out confirmation.

## Reproducibility and long-term evidence

Archive original Hiyal event journals, genesis, RNG streams, snapshot, fork attribution and ancestry along with derived data. Include exact source commit SHA, Cargo.lock SHA-256, Hiyal revision, toolchain, CPU/OS, complete study plan, UTC time, and SHA256SUMS. GitHub Actions artifacts are retained temporarily and **do not** provide permanent scientific storage. Copy replication archives into a durable user-controlled repository/object store with an explicit retention policy before artifact expiration.

## Scientific contribution check

The proposed model is a deliberately simple form of cumulative advantage, closely related to familiar preferential-attachment/urn mechanisms. Positive inequality under reinforcement should not be framed as discovering a novel universal law. A publishable sociology project needs stronger alternative mechanisms, theoretical differentiation, and empirical calibration/operationalization.

## Registration gate

- [x] Measure exploratory paired-difference variance and produce a conservative *approximate* N calculation in [sample-size analysis](sample-size-and-registration.md).
- [ ] Independently review variance inflation, minimum important effect, confidence interval/power assumptions and final N.
- [ ] Decide whether the chosen mechanism offers a nontrivial falsifiable prediction rather than only verifying code.
- [ ] Freeze confirmatory design/seed list, effect estimator, inference algorithm, stopping/exclusion rules and all dependent commits.
- [ ] Arrange independent replication and durable archival.
- [ ] Register publicly **before** running any confirmatory dataset.

Until these are complete, every computation is exploratory.
