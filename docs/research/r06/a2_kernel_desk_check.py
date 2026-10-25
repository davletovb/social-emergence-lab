#!/usr/bin/env python3
"""
A2 v0.1 six-kernel desk check (R-06 / Issue #50, pre-P0).

Written from the TEXT of
  docs/research/r05/a2-six-generator-formal-spec-20261025.md
  docs/research/r05/a2-information-repair-identifiability-20261025.md
only: no Hiyal code, no earlier scripts, no fixture values copied into the
kernels. Exact rational arithmetic (fractions.Fraction). Nothing here is a
simulation: every check is a closed-form value or an exhaustive enumeration
over short histories / small configuration spaces. No random numbers.

Status: AI-assisted desk check. It is NOT the source-independent
verification required by R-06 P0, and it is not external review.
Stdlib only. Exits non-zero if any check FAILs; FINDING lines are true as
stated but flag a design issue for the v0.2 decisions.

Usage: python3 docs/research/r06/a2_kernel_desk_check.py [repo-root]
"""
from fractions import Fraction as F
from itertools import product
import csv
import pathlib
import sys

RESULTS = []  # (section, check, status, detail)


def record(section, check, ok, detail=""):
    status = "PASS" if ok is True else ("FAIL" if ok is False else ok)
    RESULTS.append((section, check, status, detail))


# ---------------------------------------------------------------------------
# Shared world (spec section 1)
# ---------------------------------------------------------------------------
def payoff(ai, aj, f):
    """u_i = a_i + a_j - 2 a_i - f (1 - a_i)."""
    return F(ai + aj - 2 * ai) - F(f) * (1 - ai)


def sigma_ln3(x):
    """sigma(ln3 * x) = 3^x / (1 + 3^x); exact for integer x, else None."""
    x = F(x)
    if x.denominator != 1:
        return None
    n = int(x)
    t = F(3) ** n
    return t / (1 + t)


# ---------------------------------------------------------------------------
# Six kernels as logit arguments (beta = ln 3 for M/N/C/S/L).
# Two kernels with the same beta give identical probabilities iff their
# logit arguments are equal, so equivalence checks compare arguments exactly.
# ---------------------------------------------------------------------------
def x_M(f):
    return F(f) - 1


def x_N(f, nu):
    return F(f) - 1 + F(nu)


def x_C(f, kappa, m):
    return F(f) - 1 + F(kappa) * m


def x_S(f, delta, tau):
    return F(f) - 1 + F(delta) * tau


def x_L(f, kappaL, q):
    return F(f) - 1 + F(kappaL) * q


def L_update(q, f, o, alpha, zeta):
    """w = 1 - zeta*1[f>0];  q' = (1 - alpha w) q + alpha w o."""
    w = 1 - F(zeta) * (1 if f > 0 else 0)
    return (1 - F(alpha) * w) * q + F(alpha) * w * o


def R_update(Q, A, a, u, alphaR, lam, Qmax=10):
    """Q'(a) = clip(Q(a) + alphaR (u - A)); unchosen unchanged; A' = (1-lam)A + lam u."""
    Q = dict(Q)
    Q[a] = max(F(-Qmax), min(F(Qmax), Q[a] + F(alphaR) * (u - A)))
    A = (1 - F(lam)) * A + F(lam) * u
    return Q, A


def S_score(omega, tau_i, s_j):
    return F(omega) * tau_i * (2 * s_j - 1)


MATCHINGS = [((0, 1), (2, 3)), ((0, 2), (1, 3)), ((0, 3), (1, 2))]


def partner_map(matching):
    p = {}
    for i, j in matching:
        p[i], p[j] = j, i
    return p


def total_score(matching, score):
    """Sum over all four actors of score[i][partner(i)]."""
    p = partner_map(matching)
    return sum(score[i][p[i]] for i in range(4))


def argmax_set(score):
    totals = [total_score(mu, score) for mu in MATCHINGS]
    best = max(totals)
    return frozenset(k for k, t in enumerate(totals) if t == best), totals


def weakly_stable(matching, score):
    """No unmatched pair that STRICTLY prefers each other to current partners."""
    p = partner_map(matching)
    for i in range(4):
        for j in range(i + 1, 4):
            if p[i] == j:
                continue
            if score[i][j] > score[i][p[i]] and score[j][i] > score[j][p[j]]:
                return False
    return True


# ---------------------------------------------------------------------------
# 1. Reproduce the stored fixtures from the kernels
# ---------------------------------------------------------------------------
def check_oracle_fixture(path):
    nu = kappa = 1
    pH, pL = F(3, 4), F(1, 4)
    assort = (pH * pH + pL * pL) / 2
    mixed = pH * pL
    random_ = F(1, 3) * assort + F(2, 3) * mixed
    n_ok = 0
    rows = list(csv.DictReader(open(path)))
    for r in rows:
        exp = F(r["expected_exact"])
        fam, cond, oracle = r["family"], r["condition"], r["oracle"]
        if oracle == "A2-two-actor":
            f, m = int(r["f"]), int(r["m"])
            x = {"M": x_M(f), "N": x_N(f, nu), "C": x_C(f, kappa, m)}[fam]
            got = sigma_ln3(x)
        elif oracle == "A2-independent-two-copies":
            f, m = int(r["f"]), int(r["m"])
            x = {"N": x_N(f, nu), "C": x_C(f, kappa, m)}[fam]
            got = sigma_ln3(x) ** 2
        else:  # four-actor fixed H/L
            got = {
                ("assortative", "population-mean-cooperation"): (2 * pH + 2 * pL) / 4,
                ("mixed", "population-mean-cooperation"): (2 * pH + 2 * pL) / 4,
                ("assortative", "mean-per-pair-P(both-cooperate)"): assort,
                ("mixed", "mean-per-pair-P(both-cooperate)"): mixed,
                ("uniform-random", "mean-per-pair-P(both-cooperate)"): random_,
                ("assort-minus-mixed", "difference-mean-per-pair-P(both-cooperate)"): assort - mixed,
                ("assort-minus-random", "difference-mean-per-pair-P(both-cooperate)"): assort - random_,
            }[(cond, r["quantity"])]
        ok = got == exp
        n_ok += ok
        if not ok:
            record("fixtures", f"oracle {oracle}/{cond}/{fam}", False, f"got {got}, stored {exp}")
    record("fixtures", f"oracle fixture CSV: {n_ok}/{len(rows)} rows reproduced exactly", n_ok == len(rows))


def check_information_csv(path):
    rows = list(csv.DictReader(open(path)))
    n_ok, n_rows = 0, len(rows)
    tau = (1, 1, -1, -1)
    s = (1, 1, 0, 0)
    score = [[S_score(1, tau[i], s[j]) for j in range(4)] for i in range(4)]
    best, totals = argmax_set(score)
    for r in rows:
        cid, model, exp = r["check_id"], r["model"], r["expected"]
        if cid == "leak":
            ai, aj, f = 1, 0, 1
            u = payoff(ai, aj, f)
            got = u + ai + f * (1 - ai)
            ok = (u == -1) and (got == F(exp))
        elif cid.startswith("NC_"):
            f, m = {"NC_fine1": (1, 1), "NC_fine0": (0, 1), "NC_defect": (1, 0)}[cid]
            x = x_N(f, 1) if model == "N" else x_C(f, 1, m)
            ok = sigma_ln3(x) == F(exp)
        elif cid == "CL_fixed_match":
            ok = None  # verified by enumeration in section 3
            n_rows -= 1
            continue
        elif cid == "CL_rematch":
            # period 1: partner j (old) acted 0; period 2: new partner k whose last action was 1; f=1
            q2 = L_update(F(0), 0, 0, 1, 0)  # alpha=1, zeta=0 -> q2 = old partner action
            x = x_C(1, 1, 1) if model == "C" else x_L(1, 1, q2)
            ok = sigma_ln3(x) == F(exp)
        elif cid.startswith("MR_"):
            f = 0 if cid == "MR_fine0" else 1
            got = sigma_ln3(x_M(f)) if model == "M" else F(1, 2)  # R: sigma(beta_R*(Q1-Q0)), Q1=Q0
            ok = got == F(exp)
        elif cid.startswith("S_rank"):
            k = {"S_rank_assort": 0, "S_rank_mixedA": 1, "S_rank_mixedB": 2}[cid]
            ok = totals[k] == F(exp)
        elif cid == "S_choice":
            ok = (best == frozenset({0})) and F(exp) == 1
        elif cid == "S_random":
            ok = F(exp) == F(1, 3)
        elif cid == "Zero_scores_choice":
            zero = [[F(0)] * 4 for _ in range(4)]
            b, _ = argmax_set(zero)
            ok = len(b) == 3 and F(exp) == F(1, 3)
        else:
            ok = False
        n_ok += bool(ok)
        if not ok:
            record("fixtures", f"info-check {cid}/{model}", False, f"expected {exp}")
    record("fixtures", f"information-check CSV: {n_ok}/{n_rows} numeric rows reproduced (CL_fixed_match rows checked by enumeration below)", n_ok == n_rows)


# ---------------------------------------------------------------------------
# 2. Payoff structure
# ---------------------------------------------------------------------------
def check_payoff():
    ok_leak = all(
        payoff(ai, aj, f) + ai + f * (1 - ai) == aj
        for ai, aj, f in product((0, 1), repeat=3)
    )
    record("payoff", "leak identity a_j = u_i + a_i + f(1-a_i) holds for all 8 cells", ok_leak)
    ok_diff = all(payoff(1, aj, f) - payoff(0, aj, f) == f - 1 for aj, f in product((0, 1), repeat=2))
    record("payoff", "material C-minus-D difference equals f-1 for every partner action", ok_diff)
    # Stage-game classification at f = 0
    R, S, T, P = payoff(1, 1, 0), payoff(1, 0, 0), payoff(0, 1, 0), payoff(0, 0, 0)
    pd = (T > R > P > S) and (2 * R > T + S)
    record("payoff", f"f=0 stage game: R={R}, S={S}, T={T}, P={P}; prisoner's dilemma (T>R>P>S, 2R>T+S)?",
           "FINDING" if not pd else True,
           "R == P and 2R == T+S: mutual cooperation pays no more than mutual defection")
    const_sum = all(payoff(ai, aj, 0) + payoff(aj, ai, 0) == 0 for ai, aj in product((0, 1), repeat=2))
    record("payoff", "without a fine the dyad's total payoff is 0 in every cell (constant-sum)",
           "FINDING" if const_sum else False, "cooperation creates no surplus; it only transfers 1 unit")
    # Fixtures for M/N/C/S/L depend on payoff only through f-1, so u = b*a_j - a_i - f(1-a_i), b>1,
    # would leave them unchanged.
    b = 2
    alt = lambda ai, aj, f: F(b * aj - ai) - F(f) * (1 - ai)
    same_diff = all(alt(1, aj, f) - alt(0, aj, f) == f - 1 for aj, f in product((0, 1), repeat=2))
    Rb, Sb, Tb, Pb = alt(1, 1, 0), alt(1, 0, 0), alt(0, 1, 0), alt(0, 0, 0)
    record("payoff", f"repair check: u=2a_j-a_i-f(1-a_i) keeps C-D difference f-1 and is a PD (R={Rb},S={Sb},T={Tb},P={Pb})",
           same_diff and (Tb > Rb > Pb > Sb) and (2 * Rb > Tb + Sb))


# ---------------------------------------------------------------------------
# 3. Nesting / equivalence by exhaustive enumeration (fixed pair, T periods)
# ---------------------------------------------------------------------------
def enumerate_fixed_pair(T, kernel_a, kernel_b, schedules=None, m1_values=(0, 1)):
    """
    Fixed dyad (actors 0 and 1), every fine schedule in {0,1}^T and every joint
    action history. kernel_*: (t, f, own_hist, partner_hist, m1) -> logit argument.
    Returns (n_nodes, first_counterexample or None).
    """
    if schedules is None:
        schedules = list(product((0, 1), repeat=T))
    n = 0
    for m1 in m1_values:
        for sched in schedules:
            for hist in product(product((0, 1), repeat=2), repeat=T - 1):
                own = [h[0] for h in hist]
                par = [h[1] for h in hist]
                for t in range(T):
                    f = sched[t]
                    xa = kernel_a(t, f, own[:t], par[:t], m1, sched)
                    xb = kernel_b(t, f, own[:t], par[:t], m1, sched)
                    n += 1
                    if xa != xb:
                        return n, (m1, sched, hist[:t], t, xa, xb)
    return n, None


def L_state(t, par_hist, sched, q1, alpha, zeta):
    q = F(q1)
    for k in range(t):
        q = L_update(q, sched[k], par_hist[k], alpha, zeta)
    return q


def check_nesting(T=4):
    # M nested in N, C, S, L at their zero corners
    for name, kb in [
        ("N(nu=0)", lambda t, f, o, p, m1, s: x_N(f, 0)),
        ("C(kappa=0)", lambda t, f, o, p, m1, s: x_C(f, 0, m1 if t == 0 else p[-1])),
        ("S(delta=0, either type)", lambda t, f, o, p, m1, s: x_S(f, 0, -1)),
        ("L(kappaL=0)", lambda t, f, o, p, m1, s: x_L(f, 0, L_state(t, p, s, F(1, 2), F(1, 2), F(1, 2)))),
    ]:
        n, cx = enumerate_fixed_pair(T, lambda t, f, o, p, m1, s: x_M(f), kb)
        record("nesting", f"M == {name}: identical action law on all {n} history nodes (T={T})", cx is None, str(cx or ""))

    # C == L(alpha=1, zeta=0, q1=m1, kappaL=kappa) under fixed matching (documented Check B)
    for kappa in (F(1, 2), F(1), F(3, 2)):
        n, cx = enumerate_fixed_pair(
            T,
            lambda t, f, o, p, m1, s, k=kappa: x_C(f, k, m1 if t == 0 else p[-1]),
            lambda t, f, o, p, m1, s, k=kappa: x_L(f, k, L_state(t, p, s, m1, 1, 0)),
        )
        record("nesting", f"C(kappa={kappa}) == L(alpha=1, zeta=0, q1=m1) under fixed matching: {n} nodes", cx is None, str(cx or ""))

    # NEW: N nested in L at alpha_L = 0 (not listed in spec section 3)
    for nu, kL in [(F(0), F(1)), (F(1, 2), F(3, 2)), (F(1), F(1)), (F(3, 2), F(3, 2))]:
        q1 = nu / kL
        n, cx = enumerate_fixed_pair(
            T,
            lambda t, f, o, p, m1, s, nu=nu: x_N(f, nu),
            lambda t, f, o, p, m1, s, kL=kL, q1=q1: x_L(f, kL, L_state(t, p, s, q1, 0, F(1, 2))),
        )
        record("nesting", f"N(nu={nu}) == L(alpha_L=0, kappa_L={kL}, q1={q1}) for every fine schedule and history: {n} nodes",
               "FINDING" if cx is None else False, "exact nesting for all T; spec section 3 lists only M-in-L")

    # NEW: L with zeta = 1 is N on always-fined schedules, but not otherwise
    allfine = [tuple([1] * T)]
    n, cx = enumerate_fixed_pair(
        T,
        lambda t, f, o, p, m1, s: x_N(f, F(1, 2)),
        lambda t, f, o, p, m1, s: x_L(f, 1, L_state(t, p, s, F(1, 2), F(1, 2), 1)),
        schedules=allfine,
    )
    n2, cx2 = enumerate_fixed_pair(
        T,
        lambda t, f, o, p, m1, s: x_N(f, F(1, 2)),
        lambda t, f, o, p, m1, s: x_L(f, 1, L_state(t, p, s, F(1, 2), F(1, 2), 1)),
    )
    record("nesting", f"L(zeta=1) == N on the always-fined schedule ({n} nodes) but differs once any f=0 period occurs",
           "FINDING" if (cx is None and cx2 is not None) else False,
           "fine schedules with long fined stretches make L look like N")

    # S action kernel with all tau=+1 equals N(nu=delta)
    ok = all(x_S(f, d, 1) == x_N(f, d) for f in (0, 1) for d in (F(0), F(1, 2), F(1)))
    record("nesting", "S action kernel with all types tau=+1 equals N(nu=delta)", ok)

    # Sign restriction: N, C, L logits are never below the material baseline f-1 on the stated domains
    floor_ok = True
    for f in (0, 1):
        for v in (F(0), F(3, 4), F(3, 2)):
            for m in (0, 1):
                for q in (F(0), F(1, 2), F(1)):
                    floor_ok &= x_N(f, v) >= x_M(f) and x_C(f, v, m) >= x_M(f) and x_L(f, v, q) >= x_M(f)
    below = x_S(1, F(1, 2), -1) < x_M(1)
    record("nesting", "only S (tau=-1) among the beta-logit families can cooperate BELOW the material baseline",
           "FINDING" if (floor_ok and below) else False,
           "under random matching, S vs N is separated by the domain restriction nu>=0, not by a mechanism")


# ---------------------------------------------------------------------------
# 4. Matching scheduler
# ---------------------------------------------------------------------------
def check_scheduler():
    omegas = (F(1, 2), F(1), F(2), F(3))
    invariant = True
    n_cfg = 0
    for tau in product((-1, 1), repeat=4):
        for s in product((0, 1), repeat=4):
            sets = []
            for om in omegas:
                sc = [[S_score(om, tau[i], s[j]) if i != j else F(0) for j in range(4)] for i in range(4)]
                sets.append(argmax_set(sc)[0])
            invariant &= all(x == sets[0] for x in sets)
            n_cfg += 1
    record("scheduler", f"argmax matching is identical for omega in {{1/2,1,2,3}} across all {n_cfg} (tau, s) configurations",
           "FINDING" if invariant else False, "with a common omega and argmax clearing, only omega=0 vs omega>0 matters")

    # Homogeneous types (or any common weight on partner cooperation) -> all three matchings tie
    tie_all = True
    for t0 in (-1, 1):
        for s in product((0, 1), repeat=4):
            sc = [[S_score(1, t0, s[j]) if i != j else F(0) for j in range(4)] for i in range(4)]
            tie_all &= len(argmax_set(sc)[0]) == 3
    record("scheduler", "any profile where all actors weight partner cooperation equally gives all three matchings the same total",
           "FINDING" if tie_all else False, "sum-of-scores clearing sorts only on preference HETEROGENEITY")

    # Contrast: common 'prefer last-round cooperators' preferences under weak stability
    s = (1, 1, 0, 0)
    sc = [[F(2 * s[j] - 1) if i != j else F(-99) for j in range(4)] for i in range(4)]
    stable = [k for k, mu in enumerate(MATCHINGS) if weakly_stable(mu, sc)]
    record("scheduler", "same common preferences under a mutual-consent (stable-matching) rule select only the assortative matching",
           "FINDING" if stable == [0] else False, f"stable matchings: {stable} (0 = cooperators together)")


# ---------------------------------------------------------------------------
# 5. R learner under the A2 payoff
# ---------------------------------------------------------------------------
def check_R():
    # Under f=1 the reinforcement u - A depends only on the partner's action
    ok = True
    for A in (F(-1, 2), F(0), F(1, 2)):
        for aj in (0, 1):
            ok &= (payoff(1, aj, 1) - A) == (payoff(0, aj, 1) - A)
    record("R", "with f=1 the R learner's reinforcement u-A is identical for cooperating and defecting (depends only on a_j)",
           "FINDING" if ok else False, "stay after a cooperative partner, shift after a defector when -1 < A < 0")
    same = payoff(1, 1, 0) == payoff(0, 0, 0)
    record("R", "mutual cooperation and mutual defection pay the same (0) at f=0, so no aspiration makes CC self-reinforcing while DD is not",
           "FINDING" if same else False)


# ---------------------------------------------------------------------------
# 6. Proposed multi-period exact fixtures for L and R (not in the stored CSVs)
# ---------------------------------------------------------------------------
def proposed_fixtures():
    rows = []
    # L: alpha=1/2, zeta=1/2, q1=1/2, kappa_L=1; fines (1,0,1); partner actions (1,0,1)
    q = F(1, 2)
    fines, partner = (1, 0, 1), (1, 0, 1)
    for t in range(3):
        rows.append(("L", t + 1, f"f={fines[t]}", f"q_t={q}", f"logit_arg=f-1+q={x_L(fines[t], 1, q)}"))
        q = L_update(q, fines[t], partner[t], F(1, 2), F(1, 2))
    rows.append(("L", 4, "-", f"q_4={q}", "after three updates"))
    # R: alphaR=1/2, lam=1/2, A1=-1/2, Q1=(0,0); own (1,0,1), partner (1,0,0), fines (1,0,1)
    Q, A = {0: F(0), 1: F(0)}, F(-1, 2)
    own, par, fines = (1, 0, 1), (1, 0, 0), (1, 0, 1)
    for t in range(3):
        u = payoff(own[t], par[t], fines[t])
        rows.append(("R", t + 1, f"f={fines[t]} a_i={own[t]} a_j={par[t]} u={u}",
                     f"Q(0)={Q[0]} Q(1)={Q[1]} A={A}", f"Q(1)-Q(0)={Q[1]-Q[0]}"))
        Q, A = R_update(Q, A, own[t], u, F(1, 2), F(1, 2))
    rows.append(("R", 4, "-", f"Q(0)={Q[0]} Q(1)={Q[1]} A={A}", f"Q(1)-Q(0)={Q[1]-Q[0]}"))
    # R clip: alphaR=1, lam=0 (A fixed at -1), defect against a cooperator at f=0 (u=1) six times
    Q, A = {0: F(0), 1: F(0)}, F(-1)
    trace = []
    for _ in range(6):
        Q, A = R_update(Q, A, 0, payoff(0, 1, 0), 1, 0)
        trace.append(str(Q[0]))
    rows.append(("R-clip", "1..6", "alphaR=1 lam=0 A=-1 a_i=0 a_j=1 f=0 (u=1)", "Q(0) after each update: " + ",".join(trace), "clip at Qmax=10 binds on update 6"))
    return rows


def main():
    default_root = pathlib.Path(__file__).resolve().parents[3]
    repo = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else default_root
    d = repo / "docs" / "research" / "r05"
    check_oracle_fixture(d / "a2-oracle-fixture-20261025.csv")
    check_information_csv(d / "a2-information-checks-20261025.csv")
    check_payoff()
    check_nesting(T=4)
    check_scheduler()
    check_R()

    for sec, chk, st, det in RESULTS:
        print(f"[{st:7}] {sec:9} {chk}" + (f"\n{'':19}-> {det}" if det else ""))
    print("\nProposed exact multi-period fixtures (rational parameters):")
    for r in proposed_fixtures():
        print("  " + " | ".join(str(c) for c in r))
    n_fail = sum(1 for r in RESULTS if r[2] == "FAIL")
    n_find = sum(1 for r in RESULTS if r[2] == "FINDING")
    print(f"\n{len(RESULTS)} checks: {n_fail} FAIL, {n_find} FINDING (true as stated, but a design issue), rest PASS")
    return 1 if n_fail else 0


if __name__ == "__main__":
    sys.exit(main())
