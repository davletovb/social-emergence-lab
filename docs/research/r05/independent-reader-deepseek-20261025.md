**Registration (written before any search)**

- **Reader:** DeepSeek V4.1 Flash, running as an isolated subagent with no prior context.
- **Session identifier:** not visible to reader.
- **Tools and access limits:** WebSearch (results are US-only; scholarly coverage may be thin) and WebFetch (some publisher sites are blocked; log every failure). No shell, file, git or GitHub tool. No code repository, issue tracker or PR opened.
- **UTC date:** 2026-10-25.
- **Materials received:** the host message and one attachment only — `attachment.md` (R-05 standalone packet v2 + search protocol, 11,884 chars). No other files.
- **Declaration:** I have read no previous verdicts, rankings, research ledgers or reviewer comments about these candidates.

Proceeding to the four logged query slots per formulation.

# Independent R-05 prior-art screening report

**Reader registration — written before any search**

- **Reader:** DeepSeek V4.1 Flash, running as an isolated subagent with no prior context.
- **Session identifier:** not visible to reader.
- **Tools and access limits:** WebSearch (results are US-only; scholarly coverage may be thin) and WebFetch. No shell, file, git or GitHub tool. No code repository, issue tracker or pull request opened. No memory/past-conversation tool. Citation chasing was therefore done from reference lists inside fetched full texts (backwards) and by keyword search for citing works (forwards), **not** from a citation index — this is a real limitation and is flagged per row.
- **UTC date:** 2026-10-25.
- **Materials received:** the host message and exactly one attachment, `attachment.md` (R-05 standalone packet v2 + search protocol, 11,884 chars).
- **Declaration:** I have read **no** previous verdicts, rankings, research ledgers or reviewer comments about these candidates. I did not recognise any earlier screening of these candidates in the results I read.

**Method notes and honest bookkeeping**

- All 28 query slots (7 formulations × 4) were issued before any full text was fetched. The per-formulation "pre-search records" below were therefore fixed before any source was inspected, but were **not literally written to the transcript before the queries were issued** — recorded as such.
- Every `web_search` returned `total_results: 10`. The protocol's cap of **five first non-duplicate scholarly results per query** was applied by taking the first five non-duplicate scholarly items in display order. Where a query returned fewer than five scholarly items (general-methodology pages, encyclopaedia entries, vendor pages), the shortfall is logged rather than back-filled.
- Citation chasing: for each formulation I nominated two closest predecessors **from the query results only**, then inspected up to three works they cite and up to three works that cite them. Where forward chasing was not completed to the cap, it is marked **incomplete**, not filled.
- Conditional verdicts are given for S and E separately. **No 0–3 scores and no shortlist.**
- Absence of a hit is never read as global novelty.

---

## A1 — Inequality and opportunity access

### Pre-search record (before any full text was inspected)

- **Proposed operator:** exogenous randomization of a *fixed-size early-opportunity token* to a random subset at a fixed date, crossed with randomization of the *posthistory graph* (who is exposed to whom afterwards), plus an independent null model (degree-preserving graph rewiring that equalises opportunity while holding the token fixed).
- **Actual observable unit:** individual success events over time, and their position in the opportunity graph.
- **S route:** synthetic only — simulate talent, cumulative-advantage and opportunity-network generators, fit all three to the same synthetic longitudinal record, with an oracle that knows the generator.
- **E route:** field or lab assignment of early success + graph; needs individual-level longitudinal records, assignment timing, and rights.
- **Possible null/overlap:** the operator is already the standard cumulative-advantage field design; the novel component is only the graph arm.

### Logged queries (engine: WebSearch, US-only; date 2026-10-25; 10 results each)

| Slot | Exact query | Scholarly yield |
|---|---|---|
| S1 | `inequality of opportunity cumulative advantage opportunity network observational equivalence structural identifiability theory` | 5 scholarly (MPG working paper; Roemer–Trannoy; Annual Reviews DiPrete–Eirich; JRC report; Synthese causal-bias paper) |
| S2 | `cumulative advantage inequality rival mechanisms model recovery simulation generative model comparison` | 5 scholarly (arXiv 2310.01096; arXiv 1402.4536; *Sociological Theory and Method* generative income model; npj Complexity 2026; AJS 2007) |
| E1 | `inequality early opportunity randomized intervention network experiment treatment assignment` | 3 scholarly + 4 methodology pages (Frontiers network-experiment designs; PNAS interference; Biostatistics) |
| E2 | `opportunity inequality cumulative advantage measurement causal experiment evidence` | 4 scholarly (arXiv 2508.14770; ESR Oxford; Synthese; AJS) |

### Closest independently discovered predecessors

1. **Gelastopoulos, Sage & van de Rijt, *Inferring cumulative advantage from longitudinal records*** (arXiv:2310.01096). **Intervention: none — pure theory.** Inspected the formal section: defines "twins" as models with the same joint law, proves Theorem 2 (every talent model has a purely path-dependent twin), Theorem 4 (talent-model twin exists iff the process is exchangeable), Theorem 5 (every path-dependent model has a time-varying-talent twin), and Proposition 3 gives the explicit twin of the Sinatra et al. citation model. **What it does not show:** it does not construct or evaluate the *intervention* that breaks the equivalence; it asserts that "(quasi-)experimental designs" are the way out without giving an operator, an assignment rule, or a null model. **Redundancy:** for the *S* route on pure longitudinal data this is the already-answered contrast; it is not redundant for the graph arm.
2. **van de Rijt, Kang, Restivo & Patil, *Field experiments of success-breeds-success dynamics***, PNAS 111(19):6934–6939 (2014). **Verified intervention:** randomly bestowed early success in four live systems (Kickstarter funding, Epinions ratings, Wikipedia awards, Change.org signatures); randomisation of recipients; controls ex ante equal. **Observed units:** recipient project/review/editor/petition and subsequent success events. **What it does not show:** it does not randomise the posthistory *network*, and its own conclusion is decreasing marginal returns — so the amplification prediction of pure cumulative advantage is only weakly supported. **Redundancy:** a straight "randomise early opportunity" E-claim is already answered; the residual is the graph.
3. **Jiang, Sun, Figueiredo, Ribeiro & Towsley, *Duration and Intensity of Cumulative Advantage Competitions*** (arXiv:1402.4536). **Intervention: none.** Gives *analytical* duration/intensity tails that **do** separate cumulative advantage from random-walk ($$\sim t^{-1/2}$$ vs exponential tails). **What it does not show:** the separation is between two stylised urn/walk models, not between talent and path dependence, and it requires observing the competition to completion.

### Citation chasing (bounded)

- **Backwards, from Gelastopoulos et al.:** Merton 1968; Salganik, Dodds & Watts 2006 (MusicLab, used as the "independent condition" null); Sinatra et al. 2016 (QQ model). Classified: *conceptual/methodological predecessor* (all three).
- **Backwards, from van de Rijt et al.:** Muchnik, Aral & Taylor 2013; Restivo & van de Rijt 2012; Centola & Macy 2007. Classified: *related mechanism* (manipulated-success designs).
- **Forwards:** a keyword search for the twins paper returned only the arXiv/ResearchGate/Semantic Scholar records of the paper itself plus one Semantic Scholar summary referencing an HHMI-appointment citation-boost study — **no independent citing work inspected**. Marked **incomplete** (no citation index).
- **Deduplicated leads:** 1 (Merton 1968 appears in both lists). Inaccessible: none logged.

### Shared operator or exact identification limit

The informative non-identifiability proposition **exists and is exact**: the twin construction is a bijection between exchangeable talent laws and path-dependent laws, so *no* amount of longitudinal record separates them. Consequently the operator must be *exogenous assignment*, and the operator that could separate all three nominated mechanisms is: randomise the token **and** randomise the graph. Under that operator the predictions diverge — talent: token effect independent of graph position and no amplification; cumulative advantage: superproportional amplification in current position; opportunity network: token effect concentrated on high-degree nodes and vanishing under degree-preserving rewiring. Structural equivalence (twins) is cleanly separable from noisy finite-sample failure here, because the twins result is a distributional identity, not an estimation error.

### S route

- **Requirements:** a declared oracle; simulation of three generators; a *pre-registered* decision rule on the intervention contrast; and an explicit statement that the twins theorem already covers the no-intervention case.
- **Feasible observables:** synthetic only — full generator knowledge.
- **Source/permission unknowns:** none (open arXiv preprints).
- **Verdict: DEFER.** Reason: the S claim as stated ("is there a common intervention that separates them?") is partly pre-empted — the exact non-identifiability half is published, and the field intervention half is published by van de Rijt. Only the *graph* arm is untouched by what I found, and I did not find a source that tests it.
- **Reversal evidence:** a paper randomising both an early-success token and the subsequent exposure graph with a degree-preserving null would move this to advance; conversely, a proof that any token+graph intervention has twins would move it to unknown/defer with a stronger reason.

### E route

- **Requirements:** legally usable individual longitudinal records; actual assignment with timing; independent sampling; rights to the platform records.
- **Feasible observables:** the van de Rijt template (four live systems) plus a graph manipulation.
- **Source/permission unknowns:** platform data rights and re-identification risk; the van de Rijt systems are third-party platforms.
- **Verdict: ADVANCE (conditional on a graph arm).** Without the graph arm it is largely a replication.
- **Reversal evidence:** if the graph cannot be manipulated without changing the mechanism being measured (e.g. rewiring destroys the very opportunity the token creates), E degrades to unknown.

---

## A2 — Cooperation, incentives and sorting

### Pre-search record

- **Proposed operator:** randomized partner-assignment rule (assortative vs random) crossed with (i) a fine payoff perturbation too small to change best responses and (ii) randomized revelation of partner history, in a repeated social dilemma.
- **Actual observable unit:** action sequences plus the assignment rule actually received.
- **S route:** synthetic recovery of six mechanism classes from action+assignment histories.
- **E route:** human panel with real assignment; must not treat counterfactual forks as one real group's observations.
- **Possible null/overlap:** heavy prior modelling literature; the sorting arm is the least-covered component.

### Logged queries (WebSearch, 2026-10-25, 10 results each)

| Slot | Exact query | Scholarly yield |
|---|---|---|
| S1 | `repeated social dilemma cooperation conditional cooperation assortative matching observational equivalence structural identifiability theory` | 5 scholarly (Sci Rep 2019 AM-PD; PMC10510442; Frontiers AI 2026; Duffy working paper; arXiv 2308.11150) |
| S2 | `cooperation social dilemma strategies model recovery simulation generative model comparison` | 4 scholarly (npj Science of Learning 2026; arXiv 2402.17270; PLOS ONE 0072427; PLOS Comp Biol 1004779) |
| E1 | `public goods cooperation randomized matching treatment assignment field laboratory experiment` | 4 scholarly (MDPI *Behavioral Sciences* 16/81333; Cambridge *Exp Econ* survey; Springer JESA; Wiley *Economic Inquiry*) |
| E2 | `repeated prisoner's dilemma cooperation types measurement causal experiment evidence` | 5 scholarly (Embrey–Fréchette–Yuksel QJE/SSRN; Cooper et al. GEB 1996; Sci Rep srep04615; *Economic Theory*; Princeton PD analysis) |

### Closest independently discovered predecessors

1. **Bell, Mieth & Buchner, *Separating conditional and unconditional cooperation in a sequential Prisoner's Dilemma***, PLOS ONE 12(11):e0187952. **Intervention: lab manipulation** of partner facial expression and an extinction instruction. **Verified:** the paper builds a multinomial processing tree with parameters for positive reciprocity, negative reciprocity and cooperation bias, and states that the model "shares the property of being identifiable" with the source-monitoring model. **What it does not show:** it reports **no formal model-recovery or parameter-recovery simulation** — validation is by sensitivity of parameters to manipulations. So the "recovery" leg is not covered here. **Redundancy:** substantial for the conditional/unconditional split; not for the six-way class.
2. **Yang & Yue, *Cooperation in an Assortative Matching Prisoners Dilemma Experiment with Pro-Social Dummies***, Sci Rep 9:14850. **Verified intervention:** three treatments, with the *matching rule itself* manipulated — random matching vs a weighted-history rank-matching protocol vs that protocol plus always-cooperating dummies. **Observed units:** 210 subjects, cooperation per round, payoff–cooperation correlation, CC/DD frequencies. **What it does not show:** no competing-model recovery; the "mechanism" claims (assortativity, push–pull) are inferred from aggregate frequency shifts, not from identification of individual strategy types. **Redundancy:** the *assignment-rule* operator is already implemented in the lab.
3. **Model-recovery predecessor: *Learning to adaptively cooperate…***, npj Science of Learning (2026). **Verified:** explicit model-recovery simulation (generate from each model, refit all), parameter recovery over 30 iterations, posterior predictive checks, and an honest report that PSIS-LOO was "not uniformly ideal" for the more complex variants. **What it does not show:** four model families only, no sorting/assortative-matching class, no assignment-history conditioning.

### Citation chasing (bounded)

- **Backwards, from Bell et al.:** Batchelder & Riefer 1999; Erdfelder et al. 2009; Bayen, Murnane & Erdfelder 1996 (identifiable source-monitoring model). Classified: *conceptual/methodological predecessor*.
- **Backwards, from Yang & Yue:** Bergstrom (AM models); Eshel & Cavalli-Sforza 1982; Alger & Weibull 2013; Wang, Suri & Watts 2012 (dynamic partner updating). Classified: *related mechanism*.
- **Forwards:** not chased to cap for either — **incomplete**.
- **Dedup:** 0. Inaccessible: none logged.

### Shared operator or exact identification limit

The identification limit is precise: only the **conditional action distribution given the public history** is identified, not the latent (type, belief, sensitivity) decomposition. Two mechanisms are equivalent whenever they induce the same conditional action law — e.g. a conditional cooperator with a high sensitivity and a belief-learner with a matching learning rate are observationally the same on-path. This is exactly the finite-mixture/equilibrium-selection problem in structural repeated-game econometrics; the *set-identification* treatment is the right formalism (Cozzolino, Gualdani, Gufler, Lomys & Magnolfi, *Robust Identification in Repeated Games*, NET Institute WP 25-04: weak optimality restrictions "yield set identification of the primitives"). The operator that separates: randomise the assignment rule (breaks sorting), randomise a sub-threshold payoff perturbation (breaks aspiration vs fine sensitivity), randomise history revelation (breaks conditional response vs belief learning).

### S route

- **Requirements:** oracle; six declared generators; recovery matrices for model *and* parameter; explicit statement of what is only set-identified.
- **Feasible observables:** synthetic.
- **Unknowns:** none.
- **Verdict: ADVANCE.** The three ingredients (identifiable MPT-style separation, lab manipulation of matching, working model-recovery pipelines) all exist separately; a joint recovery study over the six named mechanisms, with the set-identification caveat declared, is not what I found.
- **Reversal evidence:** a recovery study already covering assortative matching plus belief learning plus aspiration in one design would flip this to defer.

### E route

- **Requirements:** real assignment of matching rule and of history revelation; individual action+assignment histories; ethics and rights.
- **Feasible observables:** lab public-goods/PD with partner-matching arm — demonstrably feasible (Yang & Yue; Embrey et al.).
- **Unknowns:** whether a single design can carry all three randomised arms without contaminating the dilemma structure.
- **Verdict: ADVANCE (with the counterfactual-fork caveat).** The packet's warning is well founded: the Embrey–Fréchette–Yuksel threshold-strategy account shows how easily one real group's path is over-read.
- **Reversal evidence:** if the payoff perturbation arm is too small to survive noise in a finite session, E drops to defer.

---

## B — Hidden private disagreement

### Pre-search record

- **Proposed operator:** a *standardised incentivised private-attitude elicitation* (truth-telling device) crossed with a *separately defined removal of sanctions*, with private attitude, public compliance and intervention exposure separately observed.
- **Observable unit:** individual (private report, public action) pairs plus exposure assignment.
- **S route:** synthetic — can a private/public/belief triple be recovered from (report, action, exposure)?
- **E route:** human panel with incentivised elicitation and a sanction-removal arm.
- **Null/overlap:** the review literature already documents that correcting misperception is "necessary but not sufficient"; overlap risk is high.

### Logged queries (WebSearch, 2026-10-25, 10 results each)

| Slot | Exact query | Scholarly yield |
|---|---|---|
| S1 | `pluralistic ignorance private attitudes public compliance observational equivalence structural identifiability theory` | 3 scholarly + encyclopaedia/blog items (Frontiers 2023 review; JASSS 17/3/8; arXiv 2403.00195) |
| S2 | `pluralistic ignorance hidden disagreement model recovery simulation generative model comparison` | 4 scholarly (arXiv 2608.02758; JASSS 17/3/8; Springer LNCS ×2; Physica A) |
| E1 | `private attitude disclosure randomized sanctions removal field experiment treatment assignment` | 4 scholarly (J. Accounting Research; Applied Economics; Behavioural Public Policy; *Games* 9/24) |
| E2 | `pluralistic ignorance misperception peer attitudes causal experiment evidence` | 4 scholarly (Frontiers 2023; Nature Communications 13:4779; PNAS 2522998123; arXiv 2403.00195) |

**Shortfall noted:** E1's top results are accounting/finance disclosure experiments rather than attitude-disclosure experiments — the search service does not surface the social-psychology sanction-removal literature for this string. Noted, not reallocated.

### Closest independently discovered predecessors

1. **Miller, *A century of pluralistic ignorance***, Frontiers in Social Psychology (2023). **Intervention: none — review.** Inspected the substantive sections. It explicitly separates **low** and **high mutual observability** variants, notes the disciplinary split, and states that in the high-observability case pluralistic ignorance arises *because of* observability. It also states interventions that publish the true distribution "are not guaranteed" to work (Mildenberger & Tingley; Bursztyn & Yang). **What it does not show:** no formal identifiability statement, no operator. **Redundancy:** the *diagnosis* is settled; the *separation* is not.
2. **Lütz & Wardil, *The Evolution of Pluralistic Ignorance*** (arXiv:2403.00195). **Intervention: none — formal model.** Internal state $$s_i$$ and external state $$s_e$$ are separate binary variables with imitation probability $$p_{cp}$$, memory $$T_M$$, and an initial-minority parameter $$\gamma$$. States plainly: "In situations where the internal states of the others are readily observable, pluralistic ignorance cannot be present." **What it does not show:** no observational-equivalence theorem, no recovery, no operator.
3. **PNAS, *The cultural evolution of pluralistic ignorance*** (2025). **Intervention: none — formal model.** Utility with action cost, dissonance and social-influence terms; attitude and second-order-belief dynamics; explicit cultural-tightness parameter. **Honest finding for us:** the paper states **no** observational-equivalence result, and explicitly notes that cross-sectional surveys along the trajectory can yield opposite inferences. **What it does not show:** the separate identifiability of the dissonance channel from the social-influence channel.

### Citation chasing (bounded)

- **Backwards, from Miller 2023:** Katz & Schanck 1938; Miller & McFarland 1987; Prentice & Miller 1993; Noelle-Neumann 1974 (spiral of silence); Kuran 1995. Classified: *conceptual/methodological predecessor*.
- **Backwards, from Lütz & Wardil:** Seeme et al. (explicit pluralistic-ignorance models); Gastner et al. 2018/2019 (concealed opinions, hypocrites); Clifford & Sudbury 1973 (voter model). Classified: *related mechanism*.
- **Forwards:** not chased to cap — **incomplete**.
- **Dedup:** Miller & McFarland and Prentice & Miller appear in both lists.

### Shared operator or exact identification limit

The informative non-identifiability is exact and is the packet's own clause: the accounts **remain equivalent whenever the sanction-removal operator also shifts second-order beliefs**, because silence is itself the informative cue (Miller's "spiral of silence"; Lütz–Wardil's absorbing inaction state). Formally, with public action $$a = f(y, \tilde{y}, \sigma)$$ where $$\sigma$$ is the sanction regime, the coordination-payoff and misestimation accounts are separated iff $$\partial f/\partial\sigma \neq 0$$ at fixed $$\tilde{y}$$ **and** the exposure operator moves $$\tilde{y}$$ at fixed $$\sigma$$. If removing sanctions moves $$\tilde{y}$$ too, the two gradients are confounded and only the composite is identified. The separating operator is therefore a 2×2: {sanctions present, removed} × {aggregate belief disclosed, not disclosed}, with private attitudes elicited under a truth-telling device in all four cells.

### S route

- **Requirements:** oracle; a declared elicitation error model (because the "private" report is itself measured with error); an explicit equivalence condition; a demonstration that the 2×2 breaks the composite.
- **Feasible observables:** synthetic.
- **Unknowns:** whether a credible synthetic oracle for "misestimated peer attitude" can be specified without assuming the answer.
- **Verdict: DEFER.** Reason: the equivalence condition is easy to state and may already be implied by the existing models; the S contribution is real but thin, and the review literature has already named the confound.
- **Reversal evidence:** a source showing the 2×2 operator does not break the composite would make the S claim stronger (a genuine negative proposition); a source already running the 2×2 synthetically would make it redundant.

### E route

- **Requirements:** incentivised private elicitation; real sanction-removal assignment; exposure assignment; individual linkage; ethics.
- **Feasible observables:** plausible — Nature Communications 13:4779 shows the misperception magnitude is measurable at scale; the missing piece is the *randomised sanction removal* paired with elicitation.
- **Unknowns:** whether incentivised elicitation changes the very attitude it measures (reactivity); rights over panel linkage.
- **Verdict: ADVANCE (conditional).** The empirical contrast is not obviously answered by what surfaced.
- **Reversal evidence:** an existing field experiment crossing sanction removal with attitude elicitation would move E to defer.

---

## C — Institution adoption and persistence

### Pre-search record

- **Proposed operator:** one externally imposed increase in **institution-level monitoring cost**, with the payer and the accounting rules fixed identically across mechanisms.
- **Observable unit:** group-level adoption decision and persistence, with membership composition recorded.
- **S route:** synthetic — can material-cost and legitimacy accounts be separated by a common cost shift at fixed composition?
- **E route:** lab institution-formation game with an exogenous cost shift.
- **Null/overlap:** high — institution-formation experiments already vary cost endogenously and exogenously.

### Logged queries (WebSearch, 2026-10-25, 10 results each)

| Slot | Exact query | Scholarly yield |
|---|---|---|
| S1 | `institution adoption persistence legitimacy versus material incentives observational equivalence structural identifiability theory` | 4 scholarly (Science Advances abj7377; WIDER WP 2010/85; *Water International*; AMR 1995) |
| S2 | `institution choice legitimacy model recovery simulation generative model comparison` | 3 scholarly (Springer *Comput Brain Behav*; JASSS 27/4/4; arXiv 2608.04020) |
| E1 | `monitoring cost randomized institution choice experiment treatment assignment` | 2 scholarly (Springer *Exp Econ* s10683-017-9532-5; PMC "social cost of randomization") + methodology pages |
| E2 | `institution adoption enforcement legitimacy causal experiment evidence` | 4 scholarly (Science Advances abj7377; PMC8522977; *Public Administration Review*; DellaVigna working paper) |

### Closest independently discovered predecessors

1. **Dickson, Gordon & Huber, *Identifying legitimacy***, Science Advances 8(7) (2022). **Verified intervention:** a design-based stochastic mediation in a 20-period public-goods game with an authority; randomises the authority's *investment* in enforcement accuracy (A) and, in Experiment 2, also the authority's compensation. **Observed units:** individual contributions, 360 then 440 group-periods. **What it does not show:** it varies **enforcement accuracy**, not monitoring *cost with the payer fixed*, and it separates legitimacy from material incentives, not legitimacy from preference-for-institutional-choice. **Redundancy:** this is the strongest near-miss found; the residual is the cost-shift operator with an identical payer across mechanisms.
2. **Camera & Casari, *Monitoring institutions in indefinitely repeated games***, Exp Econ 21:673–691. **Verified intervention:** participants could pay a cost to obtain information or to create a public record; compared to a no-monitoring baseline. **Observed units:** group cooperation and institution use. **Honest finding:** the abstract reports that monitoring institutions were actively used **yet groups were "unable to attain higher levels of cooperation"** — a null on the cooperation side. **What it does not show:** cost is not varied as the identifying instrument, and legitimacy/preference is not a named mechanism.
3. **Kosfeld, Okada & Riedl, *Institution Formation in Public Goods Games*** (AER). **Verified:** three-stage game with a participation stage, unanimity implementation and a shared cost $$c$$; propositions on minimum profitable size $$s^*$$ and on grand organisation under Fehr–Schmidt inequity aversion. **What it does not show:** the *cost* is a fixed parameter ($$c=2$$), not shifted as an operator, and the fairness/legitimacy channel is tested only through the model's preference parameters.

### Citation chasing (bounded)

- **Backwards, from Dickson et al.:** Imai et al. 2013 and Pirlott & MacKinnon 2016 (mediation designs); Fischbacher et al. 2001 (conditional cooperation); Akerlof 1982. Classified: *methodological predecessor*.
- **Backwards, from Kosfeld et al.:** Fehr & Schmidt 1999; Ostrom, Walker & Gardner 1992 (covenants with and without a sword); Gürerk, Irlenbusch & Rockenbach 2006; Sutter, Haigner & Kocher 2006. Classified: *direct test* of endogenous institutional choice without participation free-riding.
- **Forwards:** a keyword search returned a legitimately citing cluster (ifree "Endogenous Institution Formation"; Angelovski et al. 2019 *Evolut Inst Econ Rev*; a reputational-incentive model; "Endogenous and costly institutional deterrence" *J. Behav. Exp. Econ.*). Classified: *related mechanism*. Inspection of these forward works was via abstracts/previews only for two of them — marked **partial**.
- **Dedup:** 0. Inaccessible: ScienceDirect abstracts behind paywall (preview only).

### Shared operator or exact identification limit

The exact limit is arithmetic, and the packet names it correctly: with the institution cost $$c$$ shared equally, the material model's adoption is a step function of the minimum profitable size $$s^*(c)$$; a legitimacy/preference term $$v$$ enters additively. Unless $$v$$ is separately varied, an increase $$\Delta c$$ is absorbed into $$s^*$$ and the two accounts predict **identical** adoption changes. Composition is the second confound: if joiners self-select on prosociality, the legitimacy account and the material account are indistinguishable without randomising who is exposed to the choice. The operator that works is therefore a 2×2: {baseline cost, cost + Δc} × {composition held fixed by random assignment, composition self-selected}, with the payer fixed.

### S route

- **Requirements:** oracle; a formal demonstration that the cost shift alone is *not* identifying; the composition-randomisation arm; explicit separation of structural equivalence from finite-sample failure.
- **Feasible observables:** synthetic.
- **Unknowns:** whether the legitimacy term can be parameterised without circularity.
- **Verdict: ADVANCE (conditional on including the composition arm).** Without the composition arm it is close to redundant with Dickson et al. plus Kosfeld et al.
- **Reversal evidence:** a formal result that a fixed-payer cost shift *does* identify $$v$$ under shared-cost accounting would remove the need for the composition arm.

### E route

- **Requirements:** real cost shift with fixed payer and accounting; composition randomisation; individual choice records; ethics.
- **Feasible observables:** all pieces exist in the lab.
- **Unknowns:** none material beyond standard lab ethics.
- **Verdict: ADVANCE.** No source found performs the fixed-payer cost-shift contrast.
- **Reversal evidence:** a paper shifting monitoring cost with an identical payer across a legitimacy-salient and a legitimacy-neutral institution would move this to defer.

---

## D — Cooperation after scarcity

### Pre-search record

- **Proposed operator:** one identical, externally introduced resource disturbance, with explicit conservation and access rights held constant across mechanisms.
- **Observable unit:** postshock extraction, transfers to the shocked unit, and stock recovery.
- **S route:** synthetic recovery of reciprocal-obligation, enforcement, risk-pooling and selective-access generators from shock responses.
- **E route:** field or lab with an exogenous shock and declared access rights.
- **Null/overlap:** high; the shock operator is standard.

### Logged queries (WebSearch, 2026-10-25, 10 results each)

| Slot | Exact query | Scholarly yield |
|---|---|---|
| S1 | `cooperation after scarcity reciprocal obligations risk pooling observational equivalence structural identifiability theory` | 3 scholarly (Sci Rep s41598-023-43918-w; JASSS 9/1/12; *Ecol. Econ.* 2026) + book chapters |
| S2 | `sharing risk pooling resource shock model recovery simulation generative model comparison` | 4 scholarly (MDPI *Mathematics* 12/2083; *Risks* 14/226; Human Generosity Project Hao et al. 2014; PLOS ONE 0248757) |
| E1 | `resource shock randomized sharing cooperation experiment treatment assignment` | 4 scholarly (PLOS ONE 0158940; *Ecology and Society* 22/4/10; Charness risk-sharing experiment; PLOS ONE 0240212) |
| E2 | `scarcity cooperation sharing causal experiment evidence resource disturbance` | 4 scholarly (*Ecol. Econ.* S0921800926000480 and S0921800924002854; *J. Econ. Behav. Organ.* seasonal scarcity; *Cerebral Cortex* 32/5330) |

### Closest independently discovered predecessors

1. **Howe, Murphy, Gerkey & West, *Indirect Reciprocity, Resource Sharing, and Environmental Risk***, PLOS ONE 11(7):e0158940. **Verified intervention:** four sharing treatments that vary *what is revealed before the sharing decision* — Reward, Risk (randomly selected shock victim announced), Risk/Reward, Risk/Reward/Reputation. **Observed units:** 136 subsistence users, rubles received, group allocation. **What it does not show:** it does not randomise *access rights*, and it explicitly rules out direct reciprocity by rotating letters. **Redundancy:** the shock operator and the reciprocity-vs-risk-pooling contrast are already implemented; the residual is the access-rights arm.
2. **Cherry, Howe & Murphy, *Sharing as risk pooling in a social dilemma experiment***, *Ecology and Society* 20(1):68. **Verified:** a Shock treatment "introduc[ing] idiosyncratic risk by randomly selecting one group member to receive a negative shock after all allocation decisions." **Honest finding:** a Full Information treatment was "statistically indistinguishable" from No Information, and average earnings were "no greater than without sharing" — i.e. a partial null. **What it does not show:** no separate enforcement or selective-access treatment.
3. **Charness & Genicot, *Informal Risk Sharing in an Infinite-horizon Experiment***. **Verified:** pairs, a randomly drawn recipient of $$h$$ each period, continuation probability 80% vs 90%, one random period paid. **Observed units:** 4,112 transfers, 94 participants. **Honest finding:** the risk-sharing model fails on one of four predictions (*ex ante* inequality). **What it does not show:** no common-pool stock and therefore no recovery dynamic.

### Citation chasing (bounded)

- **Backwards, from Howe et al.:** Cherry, Howe & Murphy 2015; Panchanathan & Boyd 2004; Barr & Genicot 2008; Aktipis et al. 2016. Classified: *direct test* / *related mechanism*.
- **Backwards, from Cherry et al. 2015:** Ostrom 1992; Milinski et al. 2008; Rapoport et al. 1992. Classified: *conceptual predecessor*.
- **Forwards:** not chased to cap — **incomplete**.
- **Dedup:** Cherry/Howe/Murphy appears in both lists; Ostrom 1990/1992 recurs.

### Shared operator or exact identification limit

Exact limit: with a **common** shock and free access, transfers toward the shocked unit are consistent with risk pooling, reciprocal obligation *and* enforcement simultaneously — the shock does not separate them. Separation requires the shock to be (i) idiosyncratic, (ii) announced before the transfer decision (Cherry et al.; Howe et al. T2), and (iii) crossed with randomised access rights. Under that three-part operator the predictions diverge: risk pooling predicts transfers increasing in shock severity and *decreasing* in the number of insured units (adverse-selection signature); reciprocal obligation predicts transfers increasing in the victim's prior contribution; enforcement predicts postshock punishment of non-contributors; selective access predicts exclusion of non-rights-holders irrespective of contribution.

### S route

- **Requirements:** oracle; four generators; explicit statement that the common-shock case is non-identifying; recovery under the three-part operator.
- **Feasible observables:** synthetic.
- **Unknowns:** whether "reciprocal obligation" and "enforcement" can be parameterised to be non-nested.
- **Verdict: ADVANCE.** The non-identifiability half is demonstrable and the operator is concrete; the residual over Cherry/Howe et al. is the access-rights arm.
- **Reversal evidence:** a formal proof that idiosyncratic-shock + access-rights still admits twins would strengthen the S claim but push it towards a pure negative result.

### E route

- **Requirements:** exogenous idiosyncratic shock with assignment; conservation rule declared; access rights randomisable; individual records; rights (and, for a field site, community permissions).
- **Feasible observables:** lab version fully feasible; field version needs site cooperation.
- **Unknowns:** field-site access rights and consent; whether "conservation rule" can be imposed credibly outside the lab.
- **Verdict: DEFER (field) / ADVANCE (lab).** The shock-plus-information operator is already well covered; the added arm is access rights.
- **Reversal evidence:** an existing design randomising access rights jointly with the shock would move E to defer in the lab too.

---

## E — Polarization and cross-group contact

### Pre-search record

- **Proposed operator:** common randomised cross-group exposure with independently recorded pre/post attitudes, conditional on randomised content.
- **Observable unit:** individual pre/post attitude, exposure assignment, content assignment, identity strength.
- **S route:** synthetic recovery of persuasion, defensive-identity and selective-exposure generators.
- **E route:** RCT with delayed post-measurement; the sorting problem is the central threat.
- **Null/overlap:** very high; contact RCTs are abundant and a meta-analysis already reports a file-drawer problem.

### Logged queries (WebSearch, 2026-10-25, 10 results each)

| Slot | Exact query | Scholarly yield |
|---|---|---|
| S1 | `polarization cross-group contact selective exposure observational equivalence structural identifiability theory` | 4 scholarly (arXiv 2605.28284; arXiv 2609.37521; Sci Rep 024-56868-8; *Computers in Human Behavior* abstract) |
| S2 | `polarization intergroup contact identity model recovery simulation generative model comparison` | 4 scholarly (JASSS 26/3/2; Oxford *J. Complex Networks*; PLOS ONE 0338486; ACL COLING 2025) |
| E1 | `intergroup contact randomized exposure attitudes experiment treatment assignment` | 5 scholarly (PNAS 1317670111 Enos; *Behavioural Public Policy* re-evaluation; *PSRM* Israel youth programme; *Communications Psychology* 2026; Monash China study) |
| E2 | `polarization cross-group exposure measurement causal experiment evidence` | 5 scholarly (AER Levy 2021; Science adu5584; PNAS 2112552118; *Political Behavior* 2026; UMD working paper) |

### Closest independently discovered predecessors

1. **Enos, *Causal effect of intergroup contact on exclusionary attitudes***, PNAS 111(10):3699–3704. **Verified intervention:** Spanish-speaking confederates inserted into the daily commute; randomisation at the *train-pair* level, 18 matched pairs; pre-survey five days before, post-survey after 3 days or 2 weeks. **Observed units:** commuters; three exclusionary-policy items. **What it does not show:** it randomises exposure but not content, and it cannot separate defensive identity from persuasion because no content position is assigned.
2. **Levy, *Social Media, News Consumption, and Polarization***, AER 111(3):831–870. **Verified intervention:** random offers of subscriptions to conservative or liberal outlets on Facebook; the causal chain measured is subscription → exposure → visits → shares → opinions/attitudes. **Honest finding, and important:** exposure to counter-attitudinal news *decreased* negative attitudes, but there was **"no evidence that the political leanings of news outlets affect political opinions"** — i.e. an affective/opinion dissociation. **What it does not show:** no pre-registered content-position assignment to individuals; sorting into visits is itself endogenous.
3. **Paluck, Green & Green, *The contact hypothesis re-evaluated***, *Behavioural Public Policy* 3(2):129–158. **Verified:** meta-analysis of the 27 experiments meeting random assignment + delayed measurement. **Honest findings:** pooled 0.39 but falling to ~0.20–0.25 excluding disability studies; strong heterogeneity; and a **pre-analysis-plan effect** — planned studies pooled at **0.016** versus **0.451** for unplanned. **What it does not show:** mechanism separation; the meta-analysis is about effect size, not about distinguishing persuasion from identity defence from sorting.

### Citation chasing (bounded)

- **Backwards, from Enos:** Pettigrew & Tropp 2006; Allport 1954; Gerber & Green 2012; Imai, King & Nall 2009. Classified: *methodological predecessor*.
- **Backwards, from Paluck et al.:** Broockman & Kalla 2016; Scacco & Warren 2018; Finseraas & Kotsadam 2017; Pettigrew & Tropp 2006. Classified: *direct test* of the RCT-with-delayed-measurement design.
- **Forwards:** not chased to cap — **incomplete**.
- **Dedup:** Pettigrew & Tropp appears in both.

### Shared operator or exact identification limit

Exact limit: aggregate opinion is identical under all three mechanisms, so aggregate data cannot separate them; only the joint distribution of (assignment, pre-attitude, identity strength, post-attitude) can. Worse, **selective exposure is endogenous to assignment unless the assignment is exogenous** — which is why cluster- or geography-level assignment (Enos) works and voluntary platform exposure (Levy) does not fully. The separating operator is a three-arm randomisation: assigned content direction × assigned similarity of the counterpart × forced vs free choice of exposure. Persuasion predicts movement toward assigned content irrespective of similarity; defensive identity predicts backlash concentrated in high-identifiers and larger for counter-attitudinal content; selective exposure predicts the *difference between forced and free arms*.

### S route

- **Requirements:** oracle; three generators; explicit treatment of the free-vs-forced arm; recovery of the identity parameter.
- **Feasible observables:** synthetic.
- **Unknowns:** whether "defensive identity response" is separable from a heterogeneous persuasion parameter without an auxiliary identity measure.
- **Verdict: ADVANCE.** The three-way separation is not what the existing model-recovery work covers (the JASSS 26/3/2 model is derived from experimental data but does not do recovery).
- **Reversal evidence:** a recovery study over exactly these three generators would move it to defer.

### E route

- **Requirements:** real assignment with delayed measurement; pre/post attitudes; content assignment; rights and ethics.
- **Feasible observables:** all demonstrated (Enos; Paluck et al. catalogue).
- **Unknowns:** the file-drawer problem is a genuine threat to any *new* effect-size claim — Paluck et al.'s pre-registration contrast (0.016 vs 0.451) should be treated as the baseline expectation.
- **Verdict: ADVANCE, but the E claim must be about *separation*, not about a new effect size.**
- **Reversal evidence:** if the three-way assignment proves infeasible without deception, E degrades to defer.

---

## F — Referral and selection

### Pre-search record

- **Proposed operator:** exogenous opportunity-to-refer (who is allowed to refer) plus direct observation of *who could have applied*.
- **Observable unit:** the eligible pool, the referral decision, and the hire.
- **S route:** synthetic recovery of access, employer screening-preference and qualification generators.
- **E route:** field experiment randomising referral *opportunity*, with the applicant pool enumerated.
- **Null/overlap:** high; referral RCTs exist, and the "who could have applied" arm is the distinctive requirement.

### Logged queries (WebSearch, 2026-10-25, 10 results each)

| Slot | Exact query | Scholarly yield |
|---|---|---|
| S1 | `referral hiring network selection observational equivalence structural identifiability theory` | 5 scholarly (Beaman & Magruder working paper; NBER w25920; *REStud* referral-based job search networks; Philadelphia Fed WP 21-34; Springer *J. Evol. Econ.*) |
| S2 | `employee referral hiring model recovery simulation generative model comparison` | 2 scholarly (NBER w25920; *J. Labor Economics* abstract) + vendor pages |
| E1 | `referral randomized opportunity to refer field experiment treatment assignment hiring` | 5 scholarly (Pallais & Sands; NBER w25920; Srivastava; two AEA RCT registry entries) |
| E2 | `referral hiring disparity measurement causal experiment evidence` | 4 scholarly (NY Fed SR568; IRLE Brazil; *J. Labor Economics*; *J. Management* abstract) |

### Closest independently discovered predecessors

1. **Beaman & Magruder, *Who gets the job referral?*** **Verified intervention:** original participants randomised across seven contract treatments (fixed finder's fee vs performance pay); the design feature is that both parties know the maximum is paid, eliminating side-contracts. **Observed units:** ~831 OPs; referral task scores, trust-game transfers, relationship type. **Honest limitation stated by the authors themselves:** they "cannot distinguish whether low-ability OPs lack information vs have lower-quality networks." **Redundancy:** the *incentive-to-refer* operator is implemented; the *opportunity-to-refer* operator (who is allowed to refer at all) is the residual.
2. **Pallais & Sands, *Why the Referential Treatment?*** **Verified intervention:** individual experiment randomises referrer monitoring; team experiment randomises team type (own referral / someone else's / non-referred); supplemental experiment at a new firm with no referrer affiliation. **Observed units:** referred and non-referred *applicants* — importantly not only hires. **What it does not show:** it identifies selection vs peer influence, not access vs screening; and it cannot observe who *could* have been referred from a given network.
3. **Friebel, Heinz et al., *What Do Employee Referral Programs Do?*** **Verified intervention:** 238 stores randomised to control or four arms (information only; €50/€90/€120 bonuses), pre-registered. **Observed units:** worker-month and store-month panels. **Honest and relevant findings:** information alone produced **zero** referrals; even at €120 only 5% of hires were referrals; and **mediation attributed only ~5% of the attrition effect to referrals**, with most of the effect in stores where no referrals occurred. **What it does not show:** no enumeration of the eligible pool, so access vs screening is not identified.

### Citation chasing (bounded)

- **Backwards, from Beaman & Magruder:** Montgomery 1991; Munshi 2003; Calvo-Armengol & Jackson 2004; Fernandez, Castilla & Moore 2000; Heckman 1976 (selection correction). Classified: *conceptual predecessor*.
- **Backwards, from Pallais & Sands:** Beaman & Magruder 2012; Burks et al. 2013; Montgomery 1991. Classified: *direct test*.
- **Forwards:** not chased to cap for either — **incomplete**.
- **Dedup:** Beaman & Magruder and Montgomery appear in both.

### Shared operator or exact identification limit

Exact limit: without observing the eligible pool, an observed hiring disparity is consistent with (i) unequal referral *access*, (ii) employer screening preference, and (iii) applicant qualification differences, because referral status is endogenous to both the referrer's choice and the employer's belief that referral signals quality. The separating operator is therefore a two-part randomisation: randomly allocate the *opportunity* to refer to incumbents (breaking access), and independently enumerate and (where possible) randomly advance the *eligible* pool (breaking qualification and screening). Under that operator, access predicts composition change at a fixed employer rule; screening predicts a change in the hired/eligible ratio conditional on access; qualification predicts the change is fully explained by observables.

### S route

- **Requirements:** oracle; three generators; explicit demonstration that the incentive-to-refer operator (already implemented by Beaman & Magruder) does **not** identify access.
- **Feasible observables:** synthetic.
- **Unknowns:** whether the employer screening-preference generator is distinguishable from a referral-signal equilibrium without equilibrium selection assumptions.
- **Verdict: ADVANCE.** The non-identifiability half is easy and the residual is well defined.
- **Reversal evidence:** a formal result that referral-incentive variation already identifies access would make the S claim redundant.

### E route

- **Requirements:** randomised referral opportunity; enumerated eligible pool; hire outcomes; firm cooperation; data rights (personnel records).
- **Feasible observables:** all demonstrated in isolation (Srivastava's lower-caste referral-opportunity allocation is the closest; Friebel et al. is the scale precedent). Srivastava's PDF timed out on fetch, so its design is recorded from the search snippet only — **partially inspected**.
- **Unknowns:** firm data rights; whether enumerating the eligible pool is feasible without observing all applications.
- **Verdict: ADVANCE.** The opportunity-to-refer randomisation plus pool enumeration is the least-covered combination found.
- **Reversal evidence:** a field study randomising referral opportunity *and* enumerating the eligible pool would move E to defer.

---

## Search failures and shortfalls

| # | Item | Nature | Handling |
|---|---|---|---|
| 1 | `xlab.berkeley.edu/.../yildirim-feehan-paper-2020-07-08.pdf` (A1 E1) | Timeout on **two** separate attempts | Substituted van de Rijt PNAS 2014 as the A1 E predecessor; the Yildirim–Feehan networked-inequality experiment remains **uninspected** |
| 2 | `nber.org/system/files/working_papers/w25920/w25920.pdf` (F) | Timeout | Substituted the ECONtribute working-paper version, successfully fetched |
| 3 | `uibk.ac.at/.../sfb_seminar_20_heinz_paper.pdf` (F) | Timeout | Superseded by the ECONtribute fetch |
| 4 | `kartiksrivastava94.github.io/.../Srivastava_JobReferrals.pdf` (F) | Timeout | Design recorded from search snippet only; **partially inspected** |
| 5 | `accessecon.com/.../EB-21-V41-I3-P102.pdf` (A2 S) | Timeout | Partially substituted by Cozzolino et al. (NET WP 25-04) via RePEc |
| 6 | `pnas.org/doi/10.1073/pnas.1406936111` | **404** — wrong DOI | Corrected to `10.1073/pnas.1316836111`; fetched successfully |
| 7 | ScienceDirect article pages (C, D) | Paywall preview only | Abstract/preview recorded; Methods not read for those items |
| 8 | A2 E1 and B E1 query strings | Returned general-methodology and accounting-disclosure pages rather than the target social-psychology literature | Shortfall logged, **not** reallocated to the favoured track |
| 9 | Forward citation chasing, all seven formulations | No citation index available; forward chasing done by keyword search only, and not completed to the 6-lead cap anywhere | Marked **incomplete** per formulation |
| 10 | A1 forwards | Only self-citations/records of the twins paper surfaced | Marked **incomplete** |

**Overall quota status:** the **28 query slots are complete** (7 × 4). The **citation-expansion cap of 12 leads per formulation is complete for A1, A2, B, C, D, E and F on the backwards side only; forwards is incomplete for all seven.**

---

## Consolidated verdict table (conditional, no scores)

| # | Shared operator / identification limit | S verdict | E verdict | Single most decisive reversal evidence |
|---|---|---|---|---|
| A1 | Twin construction is a distributional identity; only exogenous token **+ graph** randomisation separates | **defer** | **advance** (needs graph arm) | A token+graph randomisation with degree-preserving null, or a proof that even that has twins |
| A2 | Only the conditional action law given public history is identified; set-identification applies | **advance** | **advance** (with counterfactual-fork caveat) | An existing joint recovery over assortative matching + belief learning + aspiration |
| B | Accounts are equivalent exactly when sanction removal also moves second-order beliefs | **defer** | **advance** (conditional) | A field study crossing sanction removal with incentivised attitude elicitation |
| C | Shared-cost accounting absorbs the legitimacy term into $$s^*(c)$$; composition is the second confound | **advance** (needs composition arm) | **advance** | A formal result that a fixed-payer cost shift alone identifies the legitimacy term |
| D | Common shock + free access is non-identifying; needs idiosyncratic, pre-announced shock **+ randomised access rights** | **advance** | **defer** (field) / **advance** (lab) | A design randomising access rights jointly with the shock |
| E | Aggregate opinion is invariant; needs forced-vs-free exposure arm and identity measure | **advance** | **advance** (claim separation, not effect size) | A three-arm assignment that is infeasible without deception |
| F | Without the eligible pool, access, screening and qualification are confounded | **advance** | **advance** | A field study randomising referral opportunity *and* enumerating the eligible pool |

**Unknowns recorded rather than resolved:** A1 graph-arm feasibility outside platforms; B reactivity of incentivised elicitation; C parameterisation of the legitimacy term without circularity; D field-site rights and consent; E file-drawer baseline for any new effect size; F firm data rights and pool enumeration.

**No shortlist has been forced. A1 and A2 have not been ranked against each other.**

---

## Registration trailer

- **Reviewer model / provider / version:** DeepSeek V4.1 Flash, running as an isolated subagent with no prior context, on the host platform.
- **Session identifier:** not visible to reader.
- **Tools used:** WebSearch (US-only results; 28 queries, each returning 10 results) and WebFetch (26 fetch attempts: 20 successes, 5 timeouts, 1 HTTP 404). No shell, file, git, GitHub, memory or chat-history tool. No repository, issue tracker or PR opened.
- **Access date:** 2026-10-25 (UTC).
- **Prompts:** the host message plus the four fixed query templates `S1`, `S2`, `E1`, `E2` applied verbatim with the candidate's own mechanisms substituted, exactly as logged above.
- **Tool result timestamps:** all within the single session on 2026-10-25 UTC.
- **Materials received:** host message + one attachment (`attachment.md`, 11,884 chars). Nothing else.
- **Declaration:** no previous verdicts, rankings, research ledgers or reviewer comments about these candidates were read before this report was written.
- **Independence limit acknowledged:** this is internal independent screening *within models* — not external human scientific peer review.

*End of report.*