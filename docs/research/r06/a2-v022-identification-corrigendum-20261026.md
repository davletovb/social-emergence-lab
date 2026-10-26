# R-06 / Experiment 003 — targeted mathematical corrigendum and identification matrix

**2026-10-26 — issue-level scientific correction, not an amendment to the signed original A report.**
Based on the four **original** v0.2.2 submissions and their [reconciliation](a2-v022-four-report-reconciliation-20261026.md). Current mechanism definitions remain [v0.2.2 J/H4](a2-v022-joint-law-hierarchy-20261025.md) inheriting the [v0.2.1 offset/state definitions](a2-v021-six-kernel-design-20261025.md). Neither a changed sociology claim nor a P0 global-completion assertion is authorized.

## Critical corrections

**1. C→L with rematching was not proved and the offered map is false.** At `α_L=1, ζ=0, κ_L=κ_C`, actor i's L state for period t is its **period t−1 matched partner's** realized action; C responds to the **period t matched partner's** actual previous action. When those partners differ, the states are not generally equal. A reachable period-2 rematch can produce `p_C=σ(0)=1/2` versus `p_L=σ(−1)=0.26894…` for `β=κ=1,ε=0,f=0`. This directly **refutes that kernel mapping** under A/A0 randomized rematching. The alternative stronger proposition `𝓟_C^{d,T,H4}⊆𝓟_L^{d,T,H4}` for *some entirely different free rival representation* remains **UNKNOWN**, not disproved by one failed parameter map. Under a separately declared fixed-partner world, the mapping is valid; that is not A0, which rematches randomly.

**2. The 15-pair map must be conditional on horizon and free-rival H4 representation.** In particular, at `T=1`, the **observable law classes** satisfy `𝓟_C=𝓟_N=𝓟_L` and `𝓟_R=𝓟_M`, under either A or A0, because C/L start at half-prior and R's Q starts zero. A global table asserting zero equal family pairs without stating T≥2 is false.

**3. Distinct private-state update formulas do not establish distinct observable H4 distributions.** The role-A proposed "own-action R versus partner-action L" argument compares micro-state kernels at chosen parameters, not the family-law infimum over all admissible H4 rival actor mixtures and nuisance parameters. Different formulas, a positive finite-grid TV value, or a failed selected map **do not prove global H4 noninclusion**.

**4. Fixed F ≠ one-atom H4.** F allows four arbitrary **actor-indexed** parameter vectors; H4 with one atom gives all four iid sampled actors the **same** tuple. The F class cannot simply be declared a subset of H4's one-atom slice. Proving an F law is H4-representable requires an explicit legal population distribution and equality of its *marginalized full observable law*, not merely a guess that atoms can encode four fixed actors.

**5. Preserve structural treatment results.** For common underlying true parameter configuration θ, the A event `E` that every period has uniform matching and both zero fines has `Pr_A(E)=8^{−T}`; conditional on E, the public law is A0. Therefore `I_A(P_θ^A)⊆I_{A0}(P_θ^{A0})`. A one-atom homogeneous N with ν=3/4, β=4, ε=0 is matched under A0 by M at β=1, ε=0, both `σ(-1)=0.26894…`; under A with fine 1/2 N gives `σ(1)=0.73106…`, while every admissible M has action probability below 1/2. Hence **strict narrowing for this witness is valid**, without claiming finite-data power or strict narrowing for all true laws.

## Conservative 15-pair *family-law* register

**Legend:** `⊆` means an admissible atom-level, c-preserving pushforward **proves family inclusion for all finite T, under A and A0** (not necessarily strict). `?` means the full H4 inclusion/properness/incomparability is **not established**. This is a **corrected lower-bound map**, not a final classification, and no unproved reverse noninclusion is implied.

| Pair | What is constructively established | Global remaining question |
| --- | --- | --- |
| M / N | M⊆N (ν=0) | Strictness/complete relationship at chosen T |
| M / C | M⊆C (κ_C=0) | Strictness at chosen T |
| M / S | M⊆S (δ=ω=0) | Strictness at chosen T |
| M / L | M⊆L (κ_L=0) | Strictness at chosen T |
| M / R | M⊆R (α_R=0) | Strictness at chosen T |
| N / C | ? | Both inclusion directions / possible intersection |
| N / S | S⊆N (ν=δτ; η_N=η_S+ωτ) | Whether reverse inclusion/properness at T |
| N / L | N⊆L (κ_L=2ν; α_L=0) | Whether reverse inclusion/properness at T |
| N / R | ? | Family-law comparison |
| C / S | ? | Family-law comparison |
| C / L | ?; **specific α_L=1 fixed-partner map fails on rematching** | Any alternative full H4 inclusion |
| C / R | ? | Family-law comparison |
| S / L | S⊆L by S⊆N⊆L | Whether reverse inclusion/properness at T |
| S / R | ? | Family-law comparison |
| L / R | ? | Family-law comparison |

**Count:** **eight pairs with proved inclusion direction** (five M embeddings, S⊆N, N⊆L, S⊆L); **seven additional pairs** have no proved family-level inclusion direction. Even the eight inclusion pairs generally lack a reverse-noninclusion certificate at arbitrary T. At T=1, explicit class equalities apply irrespective of this conservative overall map.

**Do not attach "9 proved inclusions, 6 incomparable, no unknown" to R-06 acceptance**. The parent A report remains an original unedited submission. This corrigendum is an owner/reconciler assessment; request a source-separated role-A correction separately if the provenance gate requires it.

## Independent scientific gate

Before claiming a complete 15-pair classification, require for each policy/horizon and unordered family pair either:
- a constructive full *observable H4 law* embedding/equality for **all** admitted source-family mixtures, respecting atom caps and domains;
- a valid **free-rival nonmembership/separation proof** (or certified optimizer/domain bound that actually ranges over every admissible rival H4 representation), including latent matching selection;
- or **UNKNOWN**.

The particular tested Rust TV difference for the proposed C/L mapping is useful evidence that that map is wrong, **not** a proof that C and L law classes are disjoint. No pilot, CI, Hiyal implementation or human-subject inference is authorized by this corrigendum.
