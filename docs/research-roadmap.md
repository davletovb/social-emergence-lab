# Research roadmap

Status: Experiment 001 completed as an exploratory **Pólya-urn baseline**; Experiment 002 has a literature-grounded, falsifiable design proposal. No experiment is registered as confirmatory.

## Mission
Investigate how macro-level social regularities emerge from individual interactions using explicit mechanisms, falsifiable hypotheses, and reproducible computational experiments. Hiyal is the simulation engine; this repository owns scientific models and interpretations.

## Research sequence
1. **001 — Emergence of inequality**: determine whether stochastic differences persist under reinforcement, relative to a non-reinforcing control. Status: exploratory benchmark completed (a known Pólya urn); confirmatory protocol in draft, not registered.
2. **002 — Posttruth recovery and structural repair**: test whether a late opportunity advantage survives truthful reputation correction under ongoing rewiring, decompose it into inherited-link exposure, post-correction-link exposure and selection, and estimate the repair-without-correction contrast. Status: [research question](../experiments/002-reputation-lock-in/research-question.md), [literature/novelty audit](../experiments/002-reputation-lock-in/literature-review.md), [formal causal model](../experiments/002-reputation-lock-in/formal-model.md), [falsification checklist](../experiments/002-reputation-lock-in/design-review-and-tests.md), [2026 critical review](../experiments/002-reputation-lock-in/scientific-review-2026.md) and [data access audit](../experiments/002-reputation-lock-in/empirical-alignment.md); **small exploratory pilot permitted after design fixes; not implemented or preregistered**.
3. **003 — Emergence of cooperation**: explore reciprocity, enforcement, group composition, and cooperation under controlled incentives. Status: candidate.
4. **004 — Social norms and institutional persistence**: examine how conventions stabilize and survive actor turnover. Status: candidate.
5. **005 — Polarization and information networks**: test how topology and selective exposure affect opinion distributions. Status: candidate.

Sequence after 001 depends on evidence and feasibility, not a fixed delivery promise.

## Priority before Experiment 002 implementation

Critical desk review identified 2025–2026 prior art, an unjustified variance statement, and a graph sampling/RNG genesis specification gap, and amended the candidate estimand accordingly. **True independent academic review is still missing.** Before a larger research study, external literature review must establish the exact overlap with prior false-popularity experiments, self-correction research, reputation network models and opportunity-access repair. Technical design review must also show that the proposed contrast is not merely a trivial restatement of uneven in-degree. External validation (available behavioral/network measurements or a feasible ethically reviewed prospective experiment) remains a major gate.

## Stage gates
- G0: theory, estimand, interventions, parameter space and primary metric specified before confirmatory runs.
- G1: executable model passes conservation (closed models) or accounting-identity (open models), symmetry, boundary, replay and restart tests.
- G2: pilot determines time/memory, numerical integrity and plausible effect variability.
- G3: protocol and code/config hashes frozen; confirmatory seeds held apart from exploratory seeds.
- G4: sensitivity checks, negative controls, full outcome reporting and replication package.
- G5: discuss scope, competing explanations and limits of real-world generalization.

## Research integrity
Keep exploratory and confirmatory runs distinguishable. No post hoc hypothesis rewriting. Store raw outputs outside Git when large, together with checksums, versioned configurations and an immutable manifest. Publish negative or null results as well as positive outcomes.
