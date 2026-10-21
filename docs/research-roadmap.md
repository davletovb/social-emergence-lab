# Research roadmap

Status: proposed, not executed.

## Mission
Investigate how macro-level social regularities emerge from individual interactions using explicit mechanisms, falsifiable hypotheses, and reproducible computational experiments. Hiyal is the simulation engine; this repository owns scientific models and interpretations.

## Research sequence
1. **001 — Emergence of inequality**: determine whether stochastic differences persist under reinforcement, relative to a non-reinforcing control. Status: protocol design.
2. **002 — Emergence of cooperation**: explore reciprocity, enforcement, group composition, and cooperation under controlled incentives. Status: candidate.
3. **003 — Social norms and institutional persistence**: examine how conventions stabilize and survive actor turnover. Status: candidate.
4. **004 — Polarization and information networks**: test how topology and selective exposure affect opinion distributions. Status: candidate.

Sequence after 001 depends on evidence and feasibility, not a fixed delivery promise.

## Stage gates
- G0: theory, estimand, interventions, parameter space and primary metric specified before confirmatory runs.
- G1: executable model passes conservation (closed models) or accounting-identity (open models), symmetry, boundary, replay and restart tests.
- G2: pilot determines time/memory, numerical integrity and plausible effect variability.
- G3: protocol and code/config hashes frozen; confirmatory seeds held apart from exploratory seeds.
- G4: sensitivity checks, negative controls, full outcome reporting and replication package.
- G5: discuss scope, competing explanations and limits of real-world generalization.

## Research integrity
Keep exploratory and confirmatory runs distinguishable. No post hoc hypothesis rewriting. Store raw outputs outside Git when large, together with checksums, versioned configurations and an immutable manifest. Publish negative or null results as well as positive outcomes.
