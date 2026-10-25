# R-06 / Experiment 003 — v0.2.2 full stochastic law and admissible family hierarchy

**2026-10-25 · PROPOSED MODEL CONTRACT for next P0 independent verification. NOT a completed P0, calibrated inference procedure, frozen confirmatory protocol, pilot or simulation authorization.**

This narrowly completes two missing rules in the historical [v0.2.1 six-kernel candidate](a2-v021-six-kernel-design-20261025.md), prompted by four independent first-pass reports ([Issue #50 reconciliation](https://github.com/davletovb/social-emergence-lab/issues/50#issuecomment-6401214504)): **(J)** the four actors' simultaneous joint decision distribution; **(H)** the family-level admissible parameter/hierarchy class. The original six action offsets, physical payoff, matching broker, periods, policy A/A0, L/R updates and set-valued ambition are **unchanged** unless explicitly stated here. All prior reports, their model completions and failures retain their original meanings. The newly chosen bounds and finite-mixture limit are **design choices for a reproducible synthetic benchmark**, not empirical facts or scientific evidence.

## 1. Exact shared game and time order (inherited)

Four labeled actors `0,1,2,3` form two dyads per period. Contributions `a_i∈{0,1}`. Same material payoff under all families:
`u_i=2a_j-a_i-f_{ij}(1-a_i)`; per-dyad publicly assigned `f∈{0,1/2}`. `d(f)=f-1`.

- `t=1` history signal `s_{j,1}=1/2` is a **declared neutral initial prior, not an observed past action**; for `t≥2`, `s_{j,t}=a_{j,t-1}` is public.
- History disclosure → assign society-level regime `z_t` → receive private valuations (all six may rank) → broker selects matching `μ_t` → assign independent fine to each realized dyad → actors decide simultaneously → release true actions and computed payoffs → update L/R private state once.
- Canonical perfect matchings in order: `01|23,02|13,03|12`. Dyad index 0 **contains actor 0**; fine vector `[f0,f1]` is indexed by this pair order, **not by actor index**.
- A: `P(z=random)=P(z=choice)=1/2`, then `f0,f1` independently `0/1/2` with probability `1/2` each, **after** pairing. A0: `z=random`, `f0=f1=0`; no other changes. No external action, information or agent transfer arm.
- All private matching scores are passed only to the broker. The scientific classifier sees `t,z,μ,f0,f1,a0..a3` and deterministic payoffs; **not** private scores, private coefficients, hidden states, chosen random uniforms, generator ID, pre-t forks or counterfactuals.

## 2. J — primary FULL joint action and innovation rule (NEW)

Let `H_t` be full public past before period t, `X_t` all *fixed* latent actor parameters and recursively updated latent L/R states, `μ_t` realized pairing and `F_t` realized two-dyad fine vector. Each actor's conditional cooperation probability is **exactly** the v0.2.1 common link:

`p_{i,t} = ε/2 + (1-ε) σ(β [f_{i,t}-1 + H^m_{i,t}])`.

**Freeze for v0.2.2 primary verification:** conditional on `(H_t,X_t,z_t,μ_t,F_t)`, draw **four independent fresh uniforms `U_{i,t}∼Unif(0,1)`**, with `a_{i,t}=1[U_{i,t}<p_{i,t}]`. Thus the complete vector kernel is

`K_m( a_0,a_1,a_2,a_3 | H_t,X_t,z_t,μ_t,F_t ) = ∏_{i=0}^3 p_{i,t}^{a_{i,t}}(1-p_{i,t})^{1-a_{i,t}}`.

The four actor innovations are **mutually independent given those conditions**, independent of the fresh broker random draw, assignment draws, initial latent parameter draws and **fresh at every period**. Current actions are simultaneous and cannot condition on contemporaneous partner actions. Past actions may be statistically dependent through histories and latent belief/aspiration updates. Marginalizing over correlated latent actor attributes, common matching, same-dyad fines or shared histories **does not** make the observable actions unconditionally independent; never fit them as independent actor-period replicates. This choice interprets `ε` as a **per-actor marginal lapse mixture**, not an undocumented common period lapse/shock.

Broker: for t=1, `v_i(j)=1/2`. For t≥2, `x_{j,t}=2a_{j,t-1}-1`; `c_i=1/3+η_i+1_{{m=S}}ωτ_i`, `v_i(j)=σ(c_i x_{j,t})`. For each perfect matching `μ`, `W(μ)=∑_{(i,j)∈μ}min(v_i(j),v_j(i))`; `P(μ|choice,H_t,X_t)=softmax_{μ}(ρ W(μ))`; random selects 1/3. Broker innovation is fresh and independent of subsequent action and fine innovations conditional on current state.

**Do not silently equate a shared-uniform/period-shock or partner-comonotone coupling to this model.** Such rules meet the old marginals but change the full history law and identification results. If studied, label as **J-shock, separate sensitivity/falsification design version**, with its own fully specified copula/time dependence. Reusing one uniform across actors, broker and time in the primary implementation is a conformance error.

For fixed parameters the per-period publicly observable record conditional on public past has mass (after integrating unobserved parameters/states if necessary)
`P(z_t) P(μ_t|z_t,H_t,X_t) P(f0_t|z_t,μ_t)P(f1_t|z_t,μ_t) K_m(a_t|H_t,X_t,z_t,μ_t,F_t)`.
Payoffs contribute an indicator that they exactly match their deterministic formula, not a further stochastic factor. `P^A_{m,θ}` is the induced recursively marginalized **joint history** law, not a product of public-action marginals.

## 3. H — two explicitly different verification targets (NEW)

**F: fixed-vector conformance oracle ONLY.** Input is one explicit actor-indexed parameter vector for each named mechanism with `β,ε,ρ` and any S global scalars; actor vectors do **not** vary between hypothetical repeats of the same fixed test case. The output is the conditional law of a tiny society over T periods. This is for deterministic, reproducible code/math comparison. It is **not** the scientifically intended population hierarchy and statements about F do not automatically hold for population mixtures.

**H4: primary proposed population-law family class.** At each **independent four-actor society**, before t=1, sample each actor's latent parameter tuple **independently and identically** from a *family-specific joint probability measure `G_m` with ≤4 support atoms*. The joint tuple's coordinates **may be arbitrarily correlated** (especially N's `η,ν` and S's `η,τ`); point masses, zero-effect atoms and all allowed boundaries are included. The same **deterministic and unknown** `G_m` and the same family/global scalar constants apply to all K societies in **one classified dataset**. No dataset-level random hyperprior is sampled once and then incorrectly described as unconditional independence of its K societies. Societies are independent **unconditionally under the specified deterministic `(G_m, globals)`**; they become coupled within each society through matching, histories, and fine assignments as written. Under an outer evaluation across J datasets, authors may vary the truth/hyperlaw using an explicit predeclared `G_{evaluation}`; they must report conditional and pooled operating characteristics separately.

**Why ≤4 atoms?** A transparent compact finite-mixture benchmark makes the nuisance law class and its pushforward closures precise and implementable without introducing covert type-independence restrictions. It is a **new modeling restriction**. Any unbounded-support or arbitrary-measure population question is an **out-of-scope extension**, not an implicit fact about H4. Test sensitivity to the atom cap in a separately budgeted new version; do not tune it after seeing outcome accuracy.

The actor tuple by family (every tuple has matching appraisal `η`):
- **M:** `(η)`, action offset 0;
- **N:** `(η,ν)`, `ν∈[-2,2]`;
- **C:** `(η,κ_C)`, `κ_C∈[-4,4]`;
- **S:** `(η,τ)`, `τ∈{-1,+1}`, with fixed global `δ∈[0,2],ω∈[0,2]`;
- **L:** `(η,κ_L,α_L,ζ)`, `κ_L∈[-4,4]`, `α_L,ζ∈[0,1]`, `q_1=1/2`;
- **R:** `(η,α_R,λ_A)`, `α_R,λ_A∈[0,1]`, `A_1=0`, all `Q_1(a,f)=0`, **fixed `Qmax=10`**.

All six use **one common numeric nuisance domain per candidate representation**: `β∈[1/10,10]`, `ε∈[0,1/2]`, `ρ∈[1/10,10]`, and **effective appraisal** `c_i=1/3+η_i+1_Sωτ_i ∈ [-3,3]`. η is otherwise unconstrained independently of c, to prevent S-to-N pushforward closure from failing because of an arbitrary η clipping boundary. The actor-level distribution G may put mass at any valid tuple and any correlation; its support atoms/weights (nonnegative, sum to 1) are **unknown model parameters**, and candidate families get equal stated access to fitting common nuisance β/ε/ρ. For S, δ/ω are unknown common scalars across K societies (not re-sampled per actor) but τ/η follow G_S. All H4 parameter hierarchies use actor-iid tuples and **independent societies**; common scalars are fixed parameters within a dataset, not random effects drawn once from an unreported shared prior. Non-independent actor sampling or society-varying β/ε/ρ is a **different** family definition, not silently in H4.

**H4 closure intentionally preserves non-identification:**
- Every M law embeds in N (`ν=0`), C (`κ_C=0`), S (`δ=ω=0`), L (`κ_L=0`), and R (`α_R=0`) using matching c-preserving G transformations and the **same joint action rule**.
- **Every S law embeds in N**, using actor mapping `ν=δτ`, `η^N=η^S+ωτ`. The effective appraisal c is unchanged; `ν∈[-2,2]` and a pushforward of ≤4 atoms stays ≤4. **Do not impose `η⊥ν`** in primary H4.
- **Every N law embeds in L**, using `κ_L=2ν`, `α_L=0` (and e.g. `ζ=0`). The fixed `q=1/2`, c and joint atom correlations map exactly and 2ν stays in [-4,4]. Thus S⊆N⊆L in the actual defined H4 class under A and A0.
- All zero, nested and potentially weak-effect cases remain in the **unconditional truth and identification denominator**; atom and parameter restrictions are not legitimate retroactive tools for forcing a unique label.

Fixed scalar c offset 1/3 is a **model convention**; η can absorb that constant, so it is not an independently identified sociological feature. `γ=1` remains the historical normalization. `ρ` and c may be unidentifiable in homogeneous appraisal subdomains.

## 4. Target estimand and resource boundaries

For any fixed finite `T≥1`, H4 defines **each family law set** `𝓟_m^{A,T,H4}`. The actual structural identified set for true `P★` is
`I_{A,T,H4}(P★)={m : ∃ allowed H4 (G_m, globals) with P_{m}^{A,T}=P★}`.
Likewise `I_{A0,T,H4}`. The full sets may remain **UNKNOWN** even after a mathematical completion; defining a question is not solving it. Retain old v0.2.1 coverage-protected finite-data set `C_α(D)` as an **unverified statistical proposal** with an absolute + calibrated max-over-five rival gate, not a passed 95% claim. A positive difference in admitted-set sizes cannot be called structural identification if a compatible rival was improperly excluded.

**A vs A0** is a bundled randomized-fine/matching policy comparison; attributing gains specifically to fines or matching needs separately budgeted factorial contrasts. Do not assert their fixed-vector law-set inclusion lattices imply equal identified sets for each truth: the Math verifier's homogeneous-N counterexample shows a specific true law can have `I_A` smaller than `I_A0` when the extra fine cell is informative. That construction must be **rechecked against the new bounds/H4 closure** before claiming it as a v0.2.2 population result.

**No K, T, J, test-calibration statistic, likelihood fitting algorithm, uniform-size theorem or confirmatory seed domain is set by this document.** F reference proofs may be checked at explicitly small T without choosing the science study's T. H4 defines the model family sufficiently to pose identification and likelihood questions; computational/statistical feasibility remains a separate requirement.

## 5. Required P0 re-verification before approval

1. Independently derive **J** and **H4**, including limits: match probabilities, action-vector product, correct public payoffs/terminal updates, legal parameter/correlation/atom sets and per-society sampling. A fixed-vector test is not H4 identification.
2. Verify the constructive embeddings M→all, S→N→L under **all** legal H4 parameter values, and inspect at least one H4 actor-mixture and an explicit alternative J-shock coupling that violates the primary joint law.
3. Compare at least two independently authored implementations with the [common fixture contract](a2-v022-common-fixtures-20261025.md). Use matching parameters/record IDs, not merely each author's different test vectors; record tolerances, hashes and every failed row.
4. Reassess all **15 family pairs** with free H4 rival parameters, P0 overlap proofs, witnessed conditional noninclusions and UNKNOWN; do not import fixed-vector complete lattice to H4 unproved.
5. Have a separate methods reviewer audit the possible cost/nonregularity of fitting H4 mixtures and the composite coverage/entire-identified-set retention rule before setting K/T/J or requesting any stochastic run.

**Verdict:** v0.2.2 makes the mathematical *model class* and conditional action-vector kernel definite **by explicit new choices**. It is neither independent confirmation of those choices nor a claim of sociological validity. No simulation, Hiyal implementation, GitHub workflow or experimental GO is authorized.
