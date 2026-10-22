# Experiment 002 — empirical data and rights audit (2026-10-22)

**Status:** source/column inspection complete for the accessible **GitHub processed CSVs**, NOT full calibration, participant-level edge audit, external review or license clearance for all underlying studies. Tracks [#22](https://github.com/davletovb/social-emergence-lab/issues/22). No human-subject data were requested from a controlled-access repository.

## Actual replication sources and versions

- Milena Tsvetkova (2021), *The effects of reputation on inequality in network cooperation games*, [article](https://doi.org/10.1098/rstb.2020.0299), [Figshare dataset DOI](https://doi.org/10.6084/m9.figshare.15097107). The publisher states processed experimental data, NetLogo model and Python analysis scripts are deposited for **nine experiments**.
- [Figshare landing page](https://figshare.com/articles/dataset/The_effects_of_reputation_on_inequality_in_network_cooperation_games/15097107) shows **version 2, 2025-04-25**, “Download all” **1.22 GB**, dataset-level **CC BY 4.0** license, and links the [source repository](https://github.com/social-research/reputation-inequality).
- The linked GitHub repository declares a **GPL-3.0 license**. Treat the two repositories/deposits as distinct licensing surfaces: dataset-level CC BY does **not** automatically override GitHub software copyleft terms or the permissions of each underlying experiment. Do not copy the code into Hiyal without separately evaluating GPL obligations.
- **Audit method:** inspected GitHub's public recursive file inventory, exact `data/*_data.csv` headers and first rows on branch `main` via the GitHub API. We did **not** download the 1.22 GB Figshare package, verify its archive identity, inspect every study's original raw participant data or check data-use terms of all original authors.

## Observed processed CSV column inventory

Ten files are present in GitHub `data/`. All columns below were read from the file itself, not inferred from a publication. Some cells are blank, and `reputation` in the displayed rows is **a condition label** such as `reputation = 0`, not evidence that a continuous pre-selection reputation score was measured.

| File | Observed columns |
| --- | --- |
| `bayer_data.csv` | version, reputation, group, player, round, action, avg_action, wealth |
| `bolton04_data.csv` | reputation, group, player, round, action, payoff, avg_action, wealth |
| `bolton05_data.csv` | version, reputation, group, player, round, action, payoff, avg_action, wealth |
| `cuesta_data.csv` | reputation, group, player, round, action, avg_action, wealth |
| `harrell_data.csv` | version, reputation, group, player, round, action, payoff, avg_action, wealth |
| `kamei_data.csv` | version, reputation, group, player, round, action, payoff, avg_action, wealth |
| `melamed2_data.csv` | version, reputation, group, player, round, action, payoff, avg_action, wealth |
| `melamed_data.csv` | version, reputation, group, player, round, action, payoff, avg_action, wealth |
| `seinen_data.csv` | action, group, payoff, player, round, reputation, avg_action, wealth |
| `stahl_data.csv` | reputation, group, player, round, action, payoff, avg_action, wealth |

All ten files are aggregated to the **player–round** grain indicated by their headers; the values may encode varying games and conditions. **No CSV header has** `source_actor`/`target_actor` (directed ties), eligible-partner lists, newly formed/removed edges, a correction timestamp, or randomized independent graph repair. We have not verified the participant keys are globally unique or that `wealth` is measured consistently across games. A source code notebook could contain additional transformed constructs, but it does not make the missing raw edge IDs appear in these CSVs.

## Variable-level correspondence to the Hiyal V3 experiment

| Construct needed for V3 | Observed candidate from inspected files | Audit status |
| --- | --- | --- |
| Reputation information/treatment | `reputation` condition, often `reputation = 0` / other condition | **Partial:** manipulation arm only; no confirmed per-dyad pre-choice belief |
| Action/cooperation propensity | `action`, `avg_action` | **Present as processed player-round fields**, game-specific semantics not resolved |
| Economic payoff inequality | `payoff` (8/10 files), `wealth` (10/10) | **Potential benchmark**, provided outcome/unit normalization and game differences are modeled |
| Group/person/time ID | `group`, `player`, `round`, sometimes `version` | **Present**, but study-specific key uniqueness remains untested |
| Directed opportunity edge j → i | no source-target pair or adjacency column | **Not observable in these processed CSVs** |
| Tie formation/deletion and continuous τ survival | no edge IDs/time of addition or deletion | **Not observable in these processed CSVs** |
| Choice among an explicit eligible set | no eligible-set or selected partner ID | **Not observable in these processed CSVs** |
| Truth correction versus graph repair | no known independent corrected-belief × network-repair factorial | **Absent/not established** |

**Decision:** These public processed CSVs may support *external plausibility checks* on cooperation, payoff dispersion and heterogeneous reputation-condition associations. **They are not sufficient to identify or calibrate the V3 structural-memory versus post-correction edge-regeneration quantity**. Specifically, computing link survival/new-link exposure from `group,player,round` alone would invent unobserved edges. Do not present such an analysis as empirical validation.

## Additional empirical constraints

- [Frey and van de Rijt (2016)](https://pmc.ncbi.nlm.nih.gov/articles/PMC5171920/): direct experimental prior art for arbitrary reputational inequality, with supplementary information. No public, permission-checked event-level edge/correction dataset has been confirmed by this audit.
- [Papamichalis et al. (2026)](https://www.nature.com/articles/s41586-026-11154-7): relevant information-induced tie formation/dissolution across 110 Honduran villages. The authors expressly require a **Yale data-use agreement and IRB approval** for individual-level and network edge data; public [Zenodo code](https://doi.org/10.5281/zenodo.22004783) is not an unrestricted release of the edge lists. Do not attempt to bypass those conditions.

## Next validation gate

1. If a **partial** benchmark is scientifically useful, document a prereviewed mapping of `wealth`/`action` and the `reputation` condition for each particular experiment, check missing data and permissions, and replicate a published inequality measure **without claiming model calibration of link survival**.
2. For the intended *structural-memory* estimand, identify a dataset with genuine pre/post tie identities, intervention timing, objective reputation information and opportunity eligibility. If none is lawfully available, report **no valid complete empirical benchmark** and consider an ethically reviewed prospective design.
3. Obtain a human sociological methods/theory assessment before any new large-scale simulation or originality/policy claim.

**Overall classification:** **PARTIAL BENCHMARK POSSIBLE for payoff and cooperation; NO-GO for full V3 empirical mechanism calibration on the inspected public files.**
