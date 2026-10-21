# Experiment 002 — Can truthful reputation correction restore opportunity access?

**Status:** scoped, falsifiable computational study **proposed**. Not implemented, simulated or preregistered; literature differentiation is preliminary. No claim of a discovered sociological law.

## Core problem

An arbitrary, false early reputational ranking can affect who receives opportunities, but also *who becomes connected to whom*. After that structural history, broadcasting the truth may fix everyone's beliefs while leaving unequal opportunities for future interaction. Does correcting the beliefs alone restore access, or is additional independent restoration of the network needed?

The **candidate claim** is about the postcorrection flow of *new* opportunities, not accumulated wealth, ability, a generic Matthew effect, or simply whether false signals influence decisions. The original version is too close to established work by Salganik & Watts (2008), van de Rijt (2019) and network reputation experiments.

## Primary causal test

All agents have **identical true quality**. Randomly give half artificially high and half artificially low starting reputation labels. Let reputations (private beliefs and a shared public reputation) affect selection and endogenous outgoing social links, before and after checkpoint τ. From an identical preintervention history, fork four interventions:

| Arm | Correct false beliefs? | Independently repair opportunity links? |
| --- | --- | --- |
| Sham | No | No |
| Information-only | Yes | No |
| Network-only | No | Yes |
| Both | Yes | Yes |

For each fork measure `D`, the share of **post-τ** opportunity tokens reaching the randomly boosted half minus the share reaching the other half. The **candidate primary estimand** is `E[D_information-only]` across independently seeded preintervention societies: the gap in new opportunities that survives complete belief correction while rewiring continues, judged against its degree-only prediction and its late-window value. `D_both` is an exact-zero control (`E[D_both]=0` by label symmetry), and `E[D_sham − D_network-only]` is the repair contrast ([design decisions](design-review-and-tests.md#design-decisions-adopted)). Do not count actors or fork branches as independent samples.

**Falsification is essential:** if correction alone removes the future opportunity gap, or structural repair does not help, the proposed mechanism is not supported for the tested parameter range. Negative controls must distinguish graph-induced exclusion from noisy belief updating and arbitrary allocation. Every parameter/outcome and inference rule remains a draft.

## Working documents

- [Targeted prior literature and explicit novelty veto](literature-review.md)
- [Formal mechanism, causal graph, state transitions and four-arm intervention](formal-model.md)
- [Design-review risks, reference checks, falsification matrix and implementation gates](design-review-and-tests.md)

## Scientific boundary

False popularity becoming real, cumulative advantage, reputation changing inequality and network-driven access asymmetries are **all established ideas**. The proposed intervention contrast may be a useful mechanism-isolation or replication target, but newness has **not** been established. The present model is entirely agent based; no human participation, empirical calibration or prior ethical approval is implied.

Only after independent review should we implement V0 as transparent deterministic Rust worlds using Hiyal's replay, fork and RNG facilities. Avoid LLM agents and heavyweight PR CI until there is a reason to add them.
