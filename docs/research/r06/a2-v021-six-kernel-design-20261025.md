# R-06 — A2 six-generator world and kernels, candidate v0.2.1

> **Historical six action/matching rules, still inherited by v0.2.2 unless overridden.** For P0 use the [controlling v0.2.2 complete joint stochastic law and H4 hierarchy](a2-v022-joint-law-hierarchy-20261025.md), which resolves the previously open coupling/parameter-class choices. No independent v0.2.2 implementation result yet.


**2026-10-25 · DESIGN AMENDMENT, NOT INDEPENDENT P0 VERIFICATION OR FROZEN PROTOCOL**

This is the **current, self-contained** proposed v0.2.1 specification for Experiment 003. It supersedes [v0.2](a2-v02-six-kernel-design-20261025.md), which remains archived unmodified. Scientific impetus: [Claude's second review](https://github.com/davletovb/social-emergence-lab/issues/50#issuecomment-6395811896) and the [internal, exact-arithmetic desk check PR #53](https://github.com/davletovb/social-emergence-lab/pull/53). The check is AI-assisted **author-side evidence, not an independent P0 oracle**. Six family labels are potentially overlapping parameterized hypotheses, never assumed to be six mutually exclusive kinds of real people.

## 1. Same physical world, payoff and observation constraints

- **Actors**: four actors, two simultaneous undirected pairs per society-period; each independent society provides one realized history of length T. A classified *dataset* contains K independent societies from one declared model family; J independently generated datasets measure design operating characteristics. K, T, J **not chosen**.
- **Actions** `a_i,t∈{0,1}`; own cooperation costs 1, creates a benefit of 2 for the matched other actor; defecting costs a publicly assigned fine `f∈{0,1/2}` applied **per matched dyad** after matching but before actions. **All families** receive exactly `u_i=2*a_j-a_i-f*(1-a_i)`. At f=0, `(T,R,P,S)=(2,1,0,-1)`; at f=1/2, `(3/2,1,-1/2,-1)`, both prisoner's dilemmas; own marginal material incentive `d(f)=f-1`. No primary `f=1` knife-edge.
- **Public history**: at t=1, every candidate actor is assigned the **explicit neutral prior** `s_j,1=1/2`, *not a fabricated previous action*. At t≥2, `s_j,t=a_j,t-1` is the truthful public previous action. Every actor observes last publicly released payoffs and histories; exact own payoff reveals partner's action, so there is **no hidden-partner-history treatment**. The classifier receives only matching regime, realized pairs, assigned fine, timing, actions, and released payoffs; no private scores, rankings, model labels, latent states or paired counterfactual futures.
- **Period ordering**: publish public history; assign matching regime; every actor may submit private cardinal valuations of all eligible peers; choose a matching; assign and announce each dyad fine; simultaneous actions; publicly release actions/payoffs; update learning for the next decision only. End-of-horizon learning does not create an observable action.
- **Random matching arm**: uniformly select from the three perfect matchings. **Choice arm**: use the *same* stochastic broker, valuation rule and feasible partner set for **every family**, with no S-only privilege.
- **Counter-based RNG contract for future P0**: use event-stable, separate domains keyed by experiment, independent dataset, society, actor/pair, period and purpose. This is not an implemented RNG or an approval to run a pilot.

## 2. Shared contribution-response and actor-level heterogeneity

The **same decision noise and material fine response for all families**:

`p_i,t = eps/2 + (1-eps)*sigmoid(beta * Delta_i,t)`, `Delta_i,t=d(f)+H_i,t`;

`sigmoid(x)=1/(1+exp(-x))`; `beta>0`, `0<=eps<1`. For any comparison, **beta and eps have exactly the same parameter access, estimation rules and generating prior** across all six families. A shared beta/eps does **not** imply six disjoint output distributions.

**Explicit actor-level parameter map (proposals, with numerical ranges not yet frozen):**

| Family | Actor-level latent state/parameters | World/family-level parameters | Contribution offset `H_i,t` and update |
| --- | --- | --- | --- |
| **M material** | common matching nuisance `eta_i`; no action-specific preference beyond `d(f)` | shared noise `beta,eps` | `H=0`; no action-state update |
| **N stable preference** | signed fixed `nu_i`, matching `eta_i` | common nuisance | `H=nu_i`; fixed across rounds |
| **C conditional behavior** | **signed `kappa_C,i`**, matching `eta_i` | common nuisance | `H=kappa_C,i*s_partner,t`; current matched partner's actual previous action (t1 neutral prior); no private learning state |
| **S type-linked assorting** | persistent binary `tau_i∈{-1,+1}`, matching `eta_i` | contribution effect `delta>=0`, extra sorting effect `omega>=0`, common nuisance | `H=delta*tau_i`; matching has *additional type-correlated term* (section 3). If delta=omega=0, equals M's response/matching laws (subject to common eta prior) |
| **L fine-masked social learning** | **signed actor-specific `kappa_L,i`**, `q_i,1=1/2`, actor-specific learning `alpha_L,i∈[0,1]`, masking `zeta_i∈[0,1]`, matching `eta_i` | shared nuisance | `H=kappa_L,i*q_i,t`. After observing current partner's actual action `a_j,t`, `w_i,t=1-zeta_i*1[f>0]`, `q_i,t+1=(1-alpha_L,i*w_i,t)*q_i,t+alpha_L,i*w_i,t*a_j,t` |
| **R contextual reinforcement** | four `Q_i(a,f)` values initially 0, actor aspiration `A_i,1=0`, **actor-specific** `alpha_R,i,lambda_A,i∈[0,1]`, matching `eta_i` | shared `Qmax=10` **as a provisional fixed modeling constant**, shared nuisance | `H=Q_i,t(1,f)-Q_i,t(0,f)`. After action a and payoff u, update only `Q_i,t+1(a,f)=clip(Q_i,t(a,f)+alpha_R,i*(u-A_i,t), -10,10)`; other context/action entries unchanged. `A_i,t+1=(1-lambda_A,i)*A_i,t+lambda_A,i*u` |

**Heterogeneity constraints**: signed N, C, L offsets can all take positive or negative signs across actors. This removes the previous **common `kappa_L` sign-only fingerprint**, while preserving mechanisms' genuinely different update restrictions. Since `q_i,t∈[0,1]` and `kappa_L,i` may be signed, L remains a meaningful learning family. **C ≡ L in fixed matching** at `alpha_L,i=1, zeta_i=0, kappa_L,i=kappa_C,i`, `q_i,1=1/2` for arbitrary actor-specific kappa; unlike v0.2 the construction need not assume homogeneous kappa. At `alpha_L,i=0`, L can mimic N if `nu_i=kappa_L,i/2` and both sets permit the corresponding parameter magnitudes. Boundary overlap is **preserved**.
  
**Not falsely symmetric:** M has no stable action effect by *definition*, R learns action values by *definition*, S has binary persistent types, and N is fixed. Adding free individual intercepts to M would change M's scientific meaning into N; this asymmetry is a **mechanism-definition assumption to challenge for robustness**, not silently hidden as equal heterogeneity. Before P0, independent adversarial testing must include actor-specific material costs/fine sensitivities as an *out-of-spec alternative*, not mislabel them an M truth.

**R aspiration/Q initialization is now explicit** (A1=0, Qmax=10), so exact P0 fixtures can be defined without guessing parameters. The numerical bounds on beta, eps, eta, omega, delta, signed kappas and other substantive parameter distributions are **unfrozen**; an independent reviewer may recommend changing A1/Qmax via another versioned design amendment.

## 3. Matching tastes and the honest S-versus-N equivalence

**Fix `gamma=1`** rather than jointly fit gamma and all eta. The prior `gamma` scale was not identifiable because the argument reduced to a single coefficient per actor. This is a **normalization**, not evidence identifying underlying preferences.

At t=1 every valuation `v_i(j)=1/2`. At t≥2 let `x_{j,t} = 2*s_{j,t} - 1 ∈ {-1,+1}`, where `s_{j,t}=a_{j,t-1}`; fix `pi_{j,t}=(1+s_{j,t})/3`. For any actor i and possible partner j, j≠i:

`v_i(j)=sigmoid(x_j,t*(1/3+eta_i+indicator[S]*omega*tau_i))`.

All six may have nonzero, signed **actor-specific `eta_i`**. In primary **unrestricted-nuisance** analysis its joint generating distribution/correlation with contribution preferences or learning parameters is **not artificially forced to independence**. The family S has an extra explicitly linked type term `omega*tau_i`, but this term can be **absorbed into N's eta**:

`nu_i^N=delta*tau_i` and `eta_i^N=eta_i^S+omega*tau_i`.

If those mapped parameters lie within N's admissible family/nuisance domain and its eta-to-nu joint distribution is permitted, **S and N have identical action and matching valuation kernels**, and therefore identical **full joint match/action laws** under either random or choice broker. **This equivalence is a PRIMARY scientific result/negative control to preserve**; do not assert S is uniquely recoverable from the same observables. At random matching omega is unobserved regardless, and S reduces to a two-point signed-N special case even under restrictive matching priors.

**Optional, separately labeled structural restriction sensitivity:** `eta_i ⟂ nu_i` in N (and analogous eta independence of action-family states elsewhere), with explicitly stated distributional/hierarchical assumptions. Only *under that extra assumption* might the correlation of persistent contribution type with partner preference reveal S; the restriction is **not adopted for primary analysis and is not proved from observed data**. Independent reviewers should examine whether it is sociologically credible or simply a convenient identifying assumption.

**Broker** (same stochastic mutual-acceptability rule for every family): for each of three complete dyad matchings `mu`, `W(mu)=sum_{(i,j) in mu} min(v_i(j),v_j(i))`. With `rho>0` common, `P(mu|choice)=exp(rho*W(mu))/sum_mu' exp(rho*W(mu'))`; with z=random, `P(mu)=1/3`. This is a stochastic centralized matching protocol, **not** a demonstrated stable-roommate solution. Valuations are submitted privately and used by the broker, but private scores remain forbidden classifier input.

**Identifiability cautions**: gamma fixed does not automatically identify eta and rho separately; actor-level nuisance distributions remain unfrozen and must not be tuned ex post. Because S/N equivalence can survive even perfect observation, **more data alone will not solve that aliasing**.

## 4. Prospective intervention policy and equal-budget sham definition

**Candidate intervention policy A (not yet frozen)**: at each society-period independently assign `z_t=random` or `choice` with probability 1/2 each (same assigned z for every actor in that society-period). Following matching, assign each dyad fine `f_mu,t=0` or `1/2` with probability 1/2 each, independently of observed history and latent states, and announce before decisions. The assignment rules and probabilities are **identical across the six generators**; no actor can use a future fine when ranking a partner.

**Candidate sham A0**: for all periods of all societies assign `z_t=random` and `f_mu,t=0`. Participants/agents have identical public recording, payoff/action structure, number of actors, time horizons, and independent societies. Their preference submissions are simply ignored by uniform matching as declared. The interventions are not literally identical across policies by design: A offers fine/matching variation, A0 is the **natural untreated world**, at exactly the same actor-period/matching count. Report also, as separately budgeted future diagnostics, fine-only and matching-only ablations; **do not silently add those to primary A vs A0 or count free forks**.

**Scientific implication of sham choice:** at A0 S's omega has no observable matching effect and remains a two-point N model. Therefore some structurally observational equivalence can be larger under A0 than under other controls—but S/N under unrestricted nuisance may remain equivalent even under A. **Sham definition is part of the estimand**; changing it later changes the scientific question and requires versioning.

## 5. Valid inference guards and future verification boundary

**Only admissible public input**: z assignments, realized pair IDs, f per dyad, public actions and released payoffs/time, with t1 explicitly neutral. Latent nu/kappa/tau/eta/Q/q/A, private cardinal valuations, simulator family labels, future clones and RNG-state tokens remain outside the classifier. A supervised evaluator may see true label solely to score coverage, not to fit or select a family.

**Mandatory structural overlap**: M/nested boundaries, N↔L frozen learning, C↔L fixed matching, S↔N under unrestricted eta correlation, and potentially M↔R when alpha_R=0, are all in the reporting target. They are not discarded to guarantee singleton model labels. Given the changed L/R/valuation rules, **v0.2 and v0.1 desk-check outputs are historical**; no v0.2.1 oracle has been run.

**Next P0 (NOT done here)**: two source-independent implementations of the complete v0.2.1 payoff, common valuation + stochastic broker, 6 contribution kernels and learning updates; exact t1/terminal update, matched-action law, actor-specific L signed heterogeneity, full S↔N mapping at valid bounds, and separate out-of-family challenges; explicitly record any failed fixtures. Then 15 free-rival pairwise population identification questions and statistical calibration with absolute adequacy must be reviewed before K/T/J budget or any pilot authorization.

**State:** This is a self-contained **design draft**; no tests, RNG runs, simulations, model fitting, Hiyal modifications, data downloads, CI triggers or experimental result. The R-06 scientific protocol is **NOT frozen**.
