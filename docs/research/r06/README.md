# R-06 — Experiment 003 (A2) working files

**Date:** 2026-10-25 · **Status:** AI-assisted desk check of the [v0.1 six-kernel specification](../r05/a2-six-generator-formal-spec-20261025.md), ahead of P0 in [Issue #50](https://github.com/davletovb/social-emergence-lab/issues/50). It is **not** the source-independent verification P0 requires, not external review, and it freezes or approves nothing. No simulation, CI, Hiyal code or human data.

## Files

- [`a2_kernel_desk_check.py`](a2_kernel_desk_check.py) — stdlib-only script written from the text of the v0.1 specification and the [information-repair note](../r05/a2-information-repair-identifiability-20261025.md). Exact rational arithmetic and exhaustive enumeration over short histories; no random numbers.
- [`a2-kernel-desk-check-output-20261025.txt`](a2-kernel-desk-check-output-20261025.txt) — recorded output at `0239ddc`.

Run from anywhere with `python3 docs/research/r06/a2_kernel_desk_check.py`. It infers the repository root and exits non-zero if any check fails.

## Result

27 checks: **0 FAIL**, 13 FINDING. The Codex review's terminal-withdrawal counterexample is now covered: an unfined final round leaves L and N observationally identical within the study; an unfined earlier round **may**, but need not, separate their next observed action. A FINDING is true as stated but flags a design issue for the v0.2 decisions discussed in the review comment on #50.

**Confirmed:**
- All 23 [oracle-fixture](../r05/a2-oracle-fixture-20261025.csv) rows and the 19 numeric [information-check](../r05/a2-information-checks-20261025.csv) rows reproduce exactly from the kernel text.
- C ≡ L under fixed matching (`α_L=1, ζ=0, q₁=m₁, κ_L=κ`) on every node of an exhaustive T=4 enumeration, for κ ∈ {½, 1, 3/2}.
- M ≡ N(ν=0) ≡ C(κ=0) ≡ S(δ=0) ≡ L(κ_L=0), as specification §3.1 states.

**Findings:**
- At f=0 the stage game has R=P=0 and is constant-sum, so it is not a prisoner's dilemma.
- N is nested in L at `α_L=0`, exactly, for every fine schedule; specification §3 does not list this.
- L with ζ=1 equals N on an always-fined schedule **or a schedule whose only unfined round is terminal**; some earlier unfined rounds can create divergent future choices, provided observation continues.
- Among the logit families, only S can cooperate below the material baseline.
- Argmax clearing gives the same matching for every ω>0. Under sum-of-scores clearing, preferences that weight partner cooperation equally cannot sort; a stable-matching rule sorts on them.
- At f=1 the R learner's reinforcement depends only on the partner's action.

The output ends with proposed exact multi-period fixtures for L, R and the Q clip. They are candidates for P0, not frozen references.
