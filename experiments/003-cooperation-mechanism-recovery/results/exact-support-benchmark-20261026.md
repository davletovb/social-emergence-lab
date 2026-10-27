# Experiment 003 / R-06 — bounded exact-support identification benchmark

**2026-10-26. Scientific status: EXPLORATORY numerical/model-analysis evidence, NOT confirmatory identification, independent P0 verification, full 15-pair H4 proof, or calibrated statistical test.**

## 1. Executed source and strict scope

A small Rust crate [`tools/exp003-exact`](../../../tools/exp003-exact) enumerates **every one of the 384 publicly possible categorical records** at one period under policy A (2 regime values × 3 matchings × 4 dyad-fine vectors × 16 simultaneous action vectors). Under A0, **48** of the 384 records have positive mass: 3 matchings × 16 action vectors, with random matching and zero fines only. Physical payoffs are checked/determined by the existing Rust simulator.

For a **declared first-period history**, it also enumerates the entire 384-category second-period suffix law. It does **not** enumerate **all** 384² possible two-period histories or fit any free-rival H4 model. For the selected two-atom H4 example, it exactly *sums the finite latent-type choices* (2⁴ = 16 actor tuple assignments, drawn **once per actor per society**), then calculates event probabilities with standard **IEEE-754 f64** sigmoid/softmax arithmetic. “Exact support” or “exhaustive” refers to **categorical enumeration**, not to exact symbolic transcendental probabilities, interval-certified bounds or exhaustive parameter-domain search.

The benchmark calls functions from **the same merged Rust simulator** in [PR #59](https://github.com/davletovb/social-emergence-lab/pull/59): it is an **internal executable oracle/diagnostic**, **not independently authored mathematical or Python/Rust source-separated verification**. Positive results therefore support consistency of this implementation on the enumerated examples, not independent correctness of all shared equations.

**Execution:** [one-off GitHub Actions run #40376577546](https://github.com/davletovb/social-emergence-lab/actions/runs/40376577546), on operational branch `ops/exp003-exact-once-20261026` at SHA `2293b57e943cc826635d9804f5c12c8217b57ab2`. All **3 targeted Rust tests passed**, including normalization and A0 support, public payoff/dyad fine indexing, and evaluation of all three identified benchmark contrasts under A and A0. The Rust comparison test now also **pins the reported numeric TV distances and H4 prefix masses** to a 1e-12 tolerance, rather than checking only their signs. The original successful run predates these added assertions; they were separately executed successfully in review run [#40437345779](https://github.com/davletovb/social-emergence-lab/actions/runs/40437345779) on 2026-10-27 with `cargo test --locked --manifest-path tools/exp003-exact/Cargo.toml` (3 passed, 0 failed). It makes **zero stochastic draws** and runs without changing the v0.2.2 model or any scientific acceptance checkbox. No default-branch CI workflow is added.

The JSON output with machine precision and exact selected parameters is retained as [`exact-support-benchmark-20261026.json`](exact-support-benchmark-20261026.json). The run's evidence ZIP artifact is **`exp003-v022-exact-support-once`**, ID `12675655142`, SHA-256 `127193ef3ce8ab4a0de9cc4d51a555e3301f02948e798fd59c2dbcb19396960a`, expires **2026-11-25 18:19:03 UTC**. Its contents include results, run-specific Cargo.lock and source hashes. The **exact same 16,195-byte `Cargo.lock`** has now also been committed at [`tools/exp003-exact/Cargo.lock`](../../../tools/exp003-exact/Cargo.lock): its Git blob hash `ef8ef42aed743945a3ac62047dacea89d5d62bad` matches the original artifact file. SHA-256 of that lockfile is `589f2e2743d2fe5d51e151475cd3376573c601137fdefc4e536ec943ddb500dc`. The Rust dependency graph is reproducible even if the artifact expires. Preserve the original numerical evidence ZIP externally if required beyond retention.

## 2. Results: clear divisions between different logical questions

| Question | Policy A | Policy A0 | Valid conclusion |
|---|---:|---:|---|
| Mapped H4 `S→N→L` first-period full 384-record law, max absolute difference | 0 | 0 | Selected two-atom source/pushforwards match numerically |
| Same mapped H4 example, joint chosen period-1 prefix + **all** 384 period-2 suffixes, max difference | 0 | 0 | Selected T2 continuation also matches; no global H4 search |
| Chosen C versus frozen-partner L **after a shared t1 public record**: T2 conditional full-record TV distance | 0.13021565 | 0.11552929 | `alpha_L=1,zeta=0,kappa_L=kappa_C` mapping fails with rematching |
| Selected N versus selected M full first-period public law TV | 0.33386583 | 0 | Extra A fine cell distinguishes these two parameterizations |
| First-period selected C/N/L and M/R mapped collapse, maximum error | 0 | 0 | Consistent with known T1 family-law equalities |

All zeros above are numerically reported **0.0 for the specifically enumerated cases**; they are not certified real-number zero bounds against every possible parameterization.

### 2.1 Preservation of S⊆N⊆L

The concrete H4 mixture follows the common v0.2.2 example with two **joint** atoms (`w=0.4,0.6`), each containing correlated `η,τ`. For any actor from the source S tuple:

- N pushforward: `ν = δτ` and `η_N = η_S + ωτ` (preserves the choice-broker appraisal);
- L pushforward: `κ_L = 2ν`, `α_L = 0` (frozen belief `q=1/2`) and `ζ=0`.

The actor IID 2-atom joint distribution remains joint and 2-atom under both transformations; its correlation is **not split** into independent marginals. Beta, epsilon, rho and policy are held fixed. Each of the 16 latent assignments was retained across both periods.

Under A and A0, the first-period entire law and the chosen first-prefix/second-suffix joint probabilities agree to the computed tolerance. The **mathematical inclusion** for all legal H4 mixtures requires the general parameter pushforward argument and is **not a new numerical discovery**. In particular, L's `α=0` update keeps its belief at `1/2` by construction; thus the chosen S→N→L numerical match is an implementation **regression guard**, not independent scientific verification of the hierarchy.

### 2.2 C versus L: conditional rematching witness only

The shared t1 history has matching `01|23`, zero fines, actions `[1,0,1,0]`. At t2, consider actor 0 paired with actor 2 by rematching `02|13`: **current** partner 2 cooperated last period, whereas **previous** partner 1 defected. Use homogeneous legal `β=κ=1, ε=0`, shared matching appraisal, `α_L=1, ζ=0, κ_L=κ_C`.

`p_C(a_0=1 | μ_2=02|13,f_2=0,H_1)=σ(0)=0.5`.

`p_L(a_0=1 | μ_2=02|13,f_2=0,H_1)=σ(-1)=0.2689414214…`.

The first public record has positive probability under A and A0, and under the selected map the two models agree on its first-period probability. After that record, numerical enumeration of **all** period-2 records gives conditional TV **0.1302** (A) or **0.1155** (A0). This refutes **this particular fixed mapping** when partners can change. It does not prove that no alternative *free-parameter* L law, including legal H4 mixtures, can represent the chosen C **unconditional full history law**. Whether `P_C^{d,T,H4} ⊆ P_L^{d,T,H4}` via another mapping remains **UNKNOWN**.

### 2.3 M versus N: a real conditional cell exclusion for a chosen N truth

Let N be a **homogeneous one-atom H4** truth with `ν=0.75, β=4, ε=0, η=0, ρ=1` and M be homogeneous with `β=1, ε=0, η=0, ρ=1`. Both satisfy the numeric v0.2.2 domains.

At zero fine, `p_N = σ(4(-1+0.75)) = σ(-1) = p_M = 0.2689414214...`. Under A0 the same action/matching law therefore repeats at all horizons for these chosen fixed parameters, not merely t1.

At fine `1/2`, however, `p_N=σ(1)=0.7310585786...` whereas this specific M parameterization gives `p_M=σ(-1/2)=0.3775406688...`. More strongly, **every** allowed M has
`p_M(1|f=1/2)=ε/2+(1-ε)σ(-β/2) < 1/2`
because `β≥0.1>0` and `0≤ε≤0.5<1`. This inequality persists under any permitted H4 mixture and matching selection because M action probabilities have no latent action preference. Hence **no M H4 representation** can recover this particular N truth's full A observable law. That last conditional-cell exclusion is a valid **analytic** argument for this witness, unlike treating selected TV alone as a global identification theorem.

The first-period selected M/N full-record TV is **0 under A0 and 0.33386583 under A**. That is an illustrative numerical difference, not a lower bound on every pair of family distributions nor power at K societies.

### 2.4 Separately expressed Python cross-check (author-side only)

A stdlib Python reference, [`tools/exp003-exact/check_cross_language.py`](../../../tools/exp003-exact/check_cross_language.py), re-derives all 384 record masses for the **fixed M/N T1** and **C/L conditional T2** witnesses without importing the Rust model or source. It reconstructs dyadic fines, mutual-choice broker softmax and the simultaneous Bernoulli action product; for C/L, it **derives the L post-period-1 belief from the original matched partner's action and fine context**, instead of injecting a hard-coded belief vector. Its additional unit checks exercise the general actor-dependent matching appraisals `η`, nontrivial `ρ`, trembling-hand `ε`, and belief update with `α, ζ`. These are **limited unit checks**, not a full numerical cross-verification of the nonzero-nuisance model or its entire history law.

**Execution:** the **exact committed Python script itself** was executed in [targeted review Actions run #40437345779](https://github.com/davletovb/social-emergence-lab/actions/runs/40437345779) on 2026-10-27 using `python3 -m unittest discover -s tools/exp003-exact -p check_cross_language.py -v`: **4 tests passed**. Fixed-witness values matched the Rust run to floating precision: M/N `A≈0.33386582682695076, A0=0`; C/L `A≈0.13021565405157565, A0≈0.11552928931500243`. The reviewed Rust command also succeeded with `--locked` using the committed lockfile and reproduced a valid JSON result (SHA-256 `e2ab71988922ddf53cd19ec67ec13818a5f47f1357d749d4f72d63f6772dba06`). Because the checks were prepared by the **same author/session** and cover **only two fixed witness families**, they are **corroboration, not the required blinded independent B Python P0 or whole-H4 cross-language replay**. They do not independently rerun the correlated S→N→L mixture integral.

For reproducibility, run `python3 tools/exp003-exact/check_cross_language.py` with Python 3.10+; it uses no third-party packages.

## 3. Interpretation and unresolved questions

The first **research takeaway** is a distinction among three levels of distinguishability:

1. **Structural nesting despite richer intervention:** no amount of A data breaks **S→N→L** under the admitted correlated H4 nuisance hierarchy. Test design alone cannot turn an inclusion into disjoint families.
2. **An invalid familiar mechanism map:** C vs L can diverge after rematching; global C/L family inclusion still requires free-rival proof, not a selected kernel mismatch.
3. **An intervention-revealed exclusion for one truth:** M and N can agree under A0 yet differ under A. Thus information from an intervention can shrink one truth's identified set even though the **family-class nesting diagram** still exists.

**Not claimed:** a complete H4 population two-period law search, all 15 pairwise proofs, uniformly valid H4 likelihood optimization, simultaneous full compatible-family confidence-set coverage, causal decomposition of fines versus matching, finite-data classification rates, statistical uncertainty calibration, or evidence about real human social behavior.

**Suggested next technical slice:** independently rederive a small subset of the same full public-history masses without calling `social-emergence-lab::experiment003`, then implement a genuinely bounded/rationally certified or interval-enclosed **free-rival** search for a narrow H4 case (starting with T1, one-atom or restricted two-atom). Report computational feasibility and optimization gaps rather than declaring global separation. Separately budget fine-only/matching-only intervention extensions rather than changing the current A/A0 design silently.

**Scientific gate:** R-06 P0 remains open. The separate first stochastic-pilot archival PR [#60](https://github.com/davletovb/social-emergence-lab/pull/60) remains independent of this code/math benchmark and is not silently merged by this PR. Both PRs link back to one another for provenance, but their `results/` JSON files remain **audit summaries, never classifier or held-out validation inputs**; only the six-field `public-only/` output of a genuinely separate observational run may be considered for future classifier ingestion.
