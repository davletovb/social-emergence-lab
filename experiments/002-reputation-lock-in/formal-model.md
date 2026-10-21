# Experiment 002 — formal causal model and computational specification

**Status:** falsifiable design proposal. Not implemented, run, preregistered, or demonstrated to be novel. All numeric parameters below are *illustrative V0 development defaults* and must not be used as post hoc confirmatory choices.

## Question and causal claim

A reputational system can choose who receives opportunities. Those choices may also restructure who can **see and select** whom. If beliefs are subsequently corrected, the unequal graph may preserve unequal access. We seek to isolate this **pathway through access**, rather than mistake accumulated wealth for continued discrimination.

### Proposed causal graph

```mermaid
flowchart LR
    B[Random initial false boost] --> R[Observers' beliefs]
    Q[Equal true ability] --> Y[Noisy performance signals]
    Y --> R
    R --> C[Recipient selection]
    R --> F[Endogenous link formation]
    F --> A[Eligible opportunity access]
    A --> C
    C --> W[Opportunity count / wealth]
    C --> Y
    I[Truthful belief correction] --> R
    S[Independent network repair] --> A
    S --> F
```

`B` is randomized within society; `I` and `S` are assigned **after the exact same preintervention journal**, via a four-branch fork. `Q` is fixed to the same value for all agents in the primary experiment. The graph contains dynamic feedback: selection determines whose outcomes are observed; observation changes beliefs; beliefs shape future access. There is no inherent difference in productivity, and opportunities arise at one per accepted step.

## Actors and state

- **N=20** agents `i=0..19`; each serves as both a *candidate recipient* and an *opportunity allocator/observer*. There are no true status or ability differences.
- Every candidate has true quality `q_i=q0=0.5`; `q0` means chance of a successful observable task *conditional on receiving an opportunity*, not wealth or deservingness.
- Wealth/resource count `w_i(0)=10` for all; one accepted opportunity adds one integer token to its recipient. A success indicator does **not** change token earnings in V0.
- Directed opportunity graph `A_{ji}∈{0,1}` is a record of which candidate `i` allocator `j` can select. No self-edges. Each allocator has exactly `d=4` outgoing candidates throughout V0 (the secondary complete-graph baseline 4b is the one exception, with `d=N−1`). In-degree can vary.
- Each allocator keeps a separate private belief `R_{ji,t}∈[0,1]` about candidate `i`'s latent quality, including agents outside the current neighborhood.
- Each candidate also has one **public reputation** `P_{i,t}∈[0,1]`, shared by all observers. `P_{i,0}=q0+δ B_i` (the false label is public), and every observed signal about `i` updates it (step 5). Private beliefs drive recipient selection and link removal among an allocator's own neighbors; the public reputation drives link *additions*, so the view of a candidate an allocator has never observed can still be corrected by others' observations.
- Randomly assign ten of the twenty candidate labels to `B_i=+1` (misleadingly favored) and the rest to `B_i=-1`. This assignment is **independent** of graph positions and of future performance outcomes.
- The initial graph is drawn **uniformly at random** from balanced d-regular directed graphs (in- and out-degree d, no self-loops or duplicate links; for example configuration pairing with rejection) using the `setup` stream, so topology itself varies across seeds rather than being one relabeled lattice. At genesis the repair graph `A*` used by arms S and IS is drawn the same way and stored in the world state, unused until τ (a fork intervention can only mutate world state; see below). Initial beliefs `R_{ji,0}=q0+δ B_i` with illustrative `δ=0.2`; therefore everyone is truly equally able but perceptions differ.

**Important:** initial labels `B_i` are randomized misinformation, *not* protected categories, true skill, early wealth, or post-hoc indicators chosen from outcomes.

## Timing and transition algorithm

Fix the order of operations in each accepted step `t=1..T`:

1. Sample allocator `j` uniformly from all N. In Hiyal this is the engine-owned scheduler in `Scheduling::Random` mode (one uniform draw from its `scheduler` stream), not a policy stream.
2. Define eligible set `E_j(t)={i:A_{ji,t}=1}`; it must have exactly d distinct non-self recipients.
3. Select recipient `i∈E_j(t)` with probability:
   `p(i|j,t) = ε/d + (1−ε) × exp(β R_{ji,t}) / Σ_{k∈E_j(t)} exp(β R_{jk,t})`.
   Illustrative `ε=0.1` and `β=4`. Compute softmax stably by subtracting its maximum log weight; use a deterministic bounded draw from the named `selection` stream. `β=0` is an exact equal-probability negative control.
4. Add one wealth token to the selected `i`; record that accepted opportunity and its allocation, actor IDs, graph and belief revision identifiers. `Σw_i(t)=N×10+t`.
5. Draw an independent performance signal `Y_{i,t}~Bernoulli(q0)` using a named **performance** RNG stream. Only observing allocator `j` updates the selected candidate's score: `R_{ji,t+1}=(1−λ)R_{ji,t}+λY_{i,t}`, illustrative `λ=0.1`. Every other private belief remains unchanged. The same signal updates the public reputation: `P_{i,t+1}=(1−λ_P)P_{i,t}+λ_P Y_{i,t}`, illustrative `λ_P=0.1`. Signals are unbiased; exposure is endogenously selected, so a rarely selected candidate's public reputation corrects slowly.
6. At every rewiring step `t≡0 mod m`, illustrative `m=10`, **before and after τ and in every arm**, draw a rewiring allocator `r` uniformly from all N using the `rewiring` stream (independent of step 1's allocator `j`). Apply the removal rule `removal∈{lowest_belief, uniform, none}` (default `lowest_belief`): `lowest_belief` removes `r`'s neighbor with the lowest private belief `R_{ri}` (ties broken by a `rewiring` draw); `uniform` removes a uniformly drawn neighbor; `none` disables rewiring entirely (no removal and no addition), so the graph stays fixed. After a removal, add one candidate that is not currently a neighbor of `r`, not `r` itself and not the candidate just removed, with probability proportional to `exp(κ P_i)` using the public reputation; illustrative `κ=6`. `κ` weights only the addition (`κ=0` makes additions uniform), and `removal` sets the removal channel, so the two vary separately. Outdegree d is invariant and no replacement creates a duplicate link. Because rewiring continues after τ, any lock-in must survive ongoing rewiring, and the self-correction alternative can occur.

At the fixed checkpoint `τ=200` accepted opportunities, first checkpoint Hiyal state **before** any intervention. Fork **four branches with identical preintervention state, namespaced RNG state and outcomes**; both information and structural changes occur in one atomic fork policy transition. Hiyal's `fork_at_head` passes the intervention only the world state, and `verify_fork` requires each branch's RNG stream positions to equal the parent's, so an intervention cannot draw random numbers: graph repair installs the pre-drawn `A*`. Arms differ only in this fork mutation; afterwards every arm runs the same transition rules, including rewiring. Continue each fork until `T=600` for a postintervention window of 400 accepted opportunities. Conditional signal draws may diverge across arms; do not pretend action-level random draws remain identical.

## Postintervention factorial design

| Branch | Correct beliefs? | Independently repair graph? | Semantics |
| --- | --- | --- | --- |
| U — sham | No | No | No state change; rewiring continues as in every arm |
| I — information-only | Yes | No | Set **all** private beliefs and every public reputation to the true value q0; retain existing opportunity links |
| S — structure-only | No | Yes | Install the pre-drawn balanced d-regular graph `A*`; retain beliefs and public reputations |
| IS — combined | Yes | Yes | Correct all beliefs and public reputations, and install `A*` |

For **S and IS**, install **the same** pre-drawn balanced graph `A*` (sampled at genesis from the `setup` stream, independent of `B`, quality signals and rewards). Recreate candidate exposure without using `B`, wealth or observed outcomes. Each branch's postintervention belief updates then follow the common V0 rule. For sham, also perform the same accounting/control-event framework but do not change state, so overhead differences do not silently alter the step horizon.

An ideal global truth correction is intentionally strong and not an empirically plausible rumor-debunking rate. Partial coverage, delayed correction, imperfect credibility, partial graph repair and multiple quality levels are **separate later extensions**, not hidden in V0.

## Preintervention baselines

We need multiple **distinct families**; these are not among the four postintervention branches of the same history:

1. **No misinformation:** set `δ=0`; observe whether spurious disparities arise from stochastic variation alone.
2. **No reputation effect on allocation:** `β=0`; recipients uniform within eligible neighborhoods. If rewiring still uses beliefs this is a topology-only mechanism, not an overall null.
3. **No reputation-based rewiring:** (3a) `removal=none`: the original balanced graph stays fixed throughout, so correction must eliminate expected **new** advantage without topology history; (3b) `removal=uniform`, `κ=0`: random rewiring at the same churn rate, the matched baseline for separating reputation-driven rewiring from rewiring as such.
4. **No network dependence (d-matched, primary):** at every step allocator `j`'s eligible set is a fresh uniform draw of d distinct non-self candidates from a `mixing` stream, i.e. an annealed random d-regular graph: same choice-set size and ε floor as the main model, but no persistent structure and no rewiring.
   (4b, secondary) **Complete graph** of all other actors, fixed and symmetric. This changes three things at once: `d` becomes N−1=19 (softmax and ε act over 19 candidates, so each candidate's ε/d floor falls from 0.025 to about 0.005), and rewiring becomes impossible, so its differences from the d=4 runs cannot be attributed to network restriction alone.
5. **No learning/no noise:** disable performance updates, `λ=0`; diagnostic isolation of false prior propagation. This is not the empirically preferred version.
6. **Original wealth-feedback urn:** Experiment 001 is a separate known reference model, not mathematically equated to reputation-mediated dynamics.

Negative-control conditions should not be collapsed into a single all-off case; each tests a different pathway. Always publish null and negative outcomes.

## Formal hypotheses (candidate, NOT registered)

- **H1 (persistent access gap after correction):** with reputation-driven rewiring continuing after τ, truthful correction alone can leave a positive association between randomized **false-boost labels** and the *future flow* of opportunity tokens even when all true qualities equal: `E_seed[D_I]>0`. Part of a positive full-window gap is expected mechanically from the in-degree inherited at τ, so H1 is supported only if the gap exceeds its degree-only prediction or persists into the late post-τ window; it is rejected if the gap decays to zero as rewiring with corrected reputations rebalances the graph.
- **H2 (structural remedy without correction):** after the same preintervention history and with no belief correction, installing the repair graph lowers the post-τ false-boost gap: `abs(E[D_S]) < abs(E[D_U])`, judged by magnitude rather than sign.
- **H3 → exact symmetry check, not a hypothesis:** when the graph is balanced, fixed and independent of `B` (`removal=none`), perfect correction resets every belief and public reputation, allocation does not depend on wealth, and post-τ signals are unbiased, so the whole post-τ process is independent of `B` and `E[D_I]=0` exactly; no mixing assumption is needed. Finite runs deviate randomly, but a stable nonzero gap can only be an implementation error, never evidence of another mechanism.
- **H4 (conditional, exploratory):** correction and repair may interact non-additively depending on signal reliability, rewiring rate and the fraction of links preserved. No a priori universal phase transition is assumed.

**Competing explanation:** unbiased noisy performance signals may allow reputation beliefs to self-correct without graph repair; a corrected network may recover without intervention if rewiring continues. The protocol *must* allow H1 and H2 to be rejected, including when the sign reverses.

## Estimands that avoid legacy-wealth confounding

Let `K_i^{z}=w_i^{z}(T)−w_i(τ)` be *new tokens during the postintervention window* in fork z. With equal-sized randomized boost groups:

`D_z = (Σ_{i:B_i=+1} K_i^z − Σ_{i:B_i=−1} K_i^z)/(T−τ)`.

`D_z∈[-1,1]`, equals 0 if each group receives exactly half the new opportunities, and is **not** the historical stock of wealth or a post-hoc grouping.

- **Primary:** `E_seed[D_I]`, the false-boost gap in new opportunities that survives complete belief correction while rewiring continues.
- **Persistence:** `D_I` in each 100-step post-τ sub-window, with `E_seed[D_I^late]` over the last one as the key persistence measure, and the residual `E_seed[D_I − D_I^deg]`, where `D_I^deg` is the degree-only prediction from the realised in-degrees. These separate a decaying inherited-degree gap from a persistent one.
- **Repair contrast:** `E_seed[D_U − D_S]`, paired on the same prehistory: structural repair without belief correction. It is not degenerate, because `S` keeps the false beliefs.
- **Exact-zero control:** `E_seed[D_IS]=0` exactly by label symmetry: after the fork, beliefs, public reputations and graph are all independent of `B`, including under continued rewiring. It is estimated as an implementation check with a Monte Carlo tolerance, not a hypothesis. For the same reason `E[D_I − D_IS]` equals `E[D_I]` and is not used as a separate contrast.
- **Secondary:** correction-only `E[D_U−D_I]`; factorial interaction `E[D_U−D_I−D_S+D_IS]` (equal to `E[D_U−D_S]−E[D_I]`); post-window allocation Gini, cumulative wealth Gini, graph in-degree/boost correlation, and rank persistence measured from τ.
- **Negative controls:** `E[D_z]=0` in expectation (exactly, by label symmetry) in every arm with no misinformation (`δ=0`), and `E[D_I]=0` with `removal=none`. Use tolerance/precision rules defined before confirmation, not exact equality in finite runs.

The statistical experimental unit is the **preintervention parent society**. Four fork outcomes, individual agents, and time steps are dependent within parent; cluster/pair bootstrap and inference only across independent parent seeds.

## Implementation and data integrity requirements

- Full model state comprises `(wealth, private beliefs R, public reputations P, directed graph A, pre-drawn repair graph A*, boost labels B, accepted_step)` and the `removal` rule, plus the named RNG streams: Hiyal's engine-owned `scheduler` (allocator choice, step 1), `selection` (recipient draw, step 3), `performance` (signals, step 5), `rewiring` (rewiring allocator, tie-breaks and additions, step 6) `setup` (label permutation, initial graph and `A*`, genesis only) and `mixing` (eligible sets in baseline 4a only). There is no reward stream: a token is a deterministic +1. Interventions are logged fork mutations. No hidden inference calls or nondeterministic unordered iteration.
- Emit exact seed, random-stream labels, Hiyal/lab revisions, model schema and full parameter digest; archive every fork ancestry, initial false-label permutation, preintervention event journal, intervention delta, postintervention actions and original signals.
- Verify invariant outdegree d, no self/duplicate links, q_i equal, score bounds [0,1], balanced initial boost groups, every action recipient eligible at that exact pre-event graph, one token/event, fork-identity and replay/checkpoint equivalence.
- Independent **model reference** and algebraic checks: β=0 equal-probability within a fixed eligibility set; β=0 with `removal=none` yields label symmetry; `κ=0` gives uniform addition weights; graph repair creates balanced exposure independent of boosted labels.
- Per-run provenance must not reveal pseudorandom stream values that make confirmatory seeds predictable *before* public preregistration while obscuring outcomes; commit hashed registration materials before confirmatory analysis, then release seeds and full data afterward.
- For the final human-facing research claim, construct validation and ethical approval are outside the simulator and still mandatory.

## Explicit scope of this document

**We have formalized a falsifiable computational study; we have not run it.** Parameters, inferential thresholds and even H1's effect sign are provisional. The prior literature review does **not** establish novelty. Nothing in this protocol is evidence about actual unequal treatment of people.
