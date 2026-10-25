#!/usr/bin/env python3
"""
A2 v0.2 design desk check — review artifact for Issue #50 (second review).

Written from the TEXT of docs/research/r06/a2-v02-six-kernel-design-20261025.md.
Equality claims compare exact rational ARGUMENTS of the logit / valuation maps:
sigma, the lapse map p = eps/2 + (1-eps) sigma(beta*x) and the broker softmax are
fixed functions shared by all families, so equal arguments give equal
probabilities. Floats are used only to report strict inequalities between
sigma values. No random numbers, no simulation. AI-assisted; not P0.
Where v0.2 is silent (R's initial aspiration A_1 and Qmax), the R check uses
placeholders A_1=0 and Qmax=10 and says so. Stdlib only.

Usage: python3 docs/research/r06/a2_v02_design_check.py   (exits non-zero on any FAIL)
"""
from fractions import Fraction as F
from itertools import product
import math

OUT = []


def rec(tag, msg, ok):
    st = "PASS" if ok is True else ("FAIL" if ok is False else ok)
    OUT.append((st, tag, msg))


sig = lambda z: 1.0 / (1.0 + math.exp(-z))
MATCHINGS = [((0, 1), (2, 3)), ((0, 2), (1, 3)), ((0, 3), (1, 2))]  # index 0 = (0,1)(2,3)


# --------------------------------------------------------------------------- payoff
def u(ai, aj, f, b=2):
    return F(b * aj - ai) - F(f) * (1 - ai)


def payoff_checks():
    for f in (F(0), F(1, 2)):
        T, R, P, S = u(0, 1, f), u(1, 1, f), u(0, 0, f), u(1, 0, f)
        pd = T > R > P > S and 2 * R > T + S
        rec("payoff", f"f={f}: T={T} R={R} P={P} S={S}; prisoner's dilemma", pd)
    leak = all((u(ai, aj, f) + ai + f * (1 - ai)) / 2 == aj
               for ai, aj in product((0, 1), repeat=2) for f in (F(0), F(1, 2)))
    rec("payoff", "leak identity a_j = (u_i + a_i + f(1-a_i))/2 in every cell", leak)
    diff = all(u(1, aj, f) - u(0, aj, f) == f - 1 for aj in (0, 1) for f in (F(0), F(1, 2)))
    rec("payoff", "own C-minus-D material difference is f-1 at both fine levels", diff)


# --------------------------------------------------------------------------- valuations
def val_arg(gamma, s_j, eta_i, is_S=False, omega=0, tau_i=0, t1=False):
    """Argument of sigma in v_i(j) (kernel doc section 4)."""
    if t1:
        pi, x = F(1, 2), F(0)
    else:
        pi, x = F(1 + s_j, 3), F(2 * s_j - 1)
    return F(gamma) * (2 * (pi - F(1, 2)) + F(eta_i) * x + (F(omega) * tau_i * x if is_S else 0))


def valuation_checks():
    ok = all(2 * (F(1 + s, 3) - F(1, 2)) == F(2 * s - 1, 3) for s in (0, 1))
    rec("valuation", "2(pi_j - 1/2) == x_j/3 exactly for t>=2, so v_i(j) = sigma(gamma * x_j * (1/3 + eta_i + 1_S*omega*tau_i))",
        "FINDING" if ok else False)
    # scale redundancy: (gamma, eta) -> (lam*gamma, (1/3+eta)/lam - 1/3)
    red = True
    for gamma, eta, lam in product((F(1, 2), F(1), F(3)), (F(-1), F(0), F(1, 4), F(2)), (F(1, 2), F(2), F(3))):
        eta2 = (F(1, 3) + eta) / lam - F(1, 3)
        red &= all(val_arg(gamma, s, eta) == val_arg(lam * gamma, s, eta2) for s in (0, 1))
    rec("valuation", "gamma is not separately identified: (gamma, eta_i) and (lam*gamma, (1/3+eta_i)/lam - 1/3) give identical valuations",
        "FINDING" if red else False)
    t1 = all(val_arg(g, 0, e, True, w, tau, t1=True) == 0 for g in (F(1), F(2)) for e in (F(-1), F(1)) for w in (F(0), F(1)) for tau in (-1, 1))
    rec("valuation", "t=1: every valuation is sigma(0)=1/2 for all six families (uniform broker)", t1)


# --------------------------------------------------------------------------- broker
def W(matching, v):
    return sum(min(v[i][j], v[j][i]) for i, j in matching)


def broker_probs(v, rho):
    ws = [W(mu, v) for mu in MATCHINGS]
    m = max(ws)
    e = [math.exp(rho * (w - m)) for w in ws]
    z = sum(e)
    return [x / z for x in e], ws


def broker_checks():
    s = (1, 1, 0, 0)
    for gamma, rho in ((F(1), 1.0), (F(1), 5.0), (F(3), 5.0)):
        v = [[sig(float(val_arg(gamma, s[j], 0))) if i != j else 0.0 for j in range(4)] for i in range(4)]
        p, ws = broker_probs(v, rho)
        rec("broker", f"common tastes (eta=0, non-S), last actions 1,1,0,0, gamma={gamma}, rho={rho}: "
                      f"W = {ws[0]:.4f}/{ws[1]:.4f}/{ws[2]:.4f}, P(cooperators paired) = {p[0]:.4f} > 1/3",
            "FINDING" if (ws[0] > ws[1] and ws[0] > ws[2] and p[0] > 1 / 3) else False)
    # identical histories, different latent tastes -> broker still sorts on eta
    s = (1, 1, 1, 1)
    eta = (F(2, 3), F(2, 3), F(-1, 6), F(-1, 6))
    v = [[sig(float(val_arg(1, s[j], eta[i]))) if i != j else 0.0 for j in range(4)] for i in range(4)]
    p, ws = broker_probs(v, 5.0)
    rec("broker", f"all four cooperated last round, eta=(2/3,2/3,-1/6,-1/6): P(similar tastes paired) = {p[0]:.4f} > 1/3 "
                  "(realized pairs carry information about latent eta)", "FINDING" if p[0] > 1 / 3 else False)


# --------------------------------------------------------------------------- S vs N
def s_vs_n_checks():
    # Constructive equality: N with nu_i = delta*tau_i and eta_i^N = eta_i + omega*tau_i
    eq = True
    n = 0
    grid = (F(-1, 2), F(0), F(1, 3))
    for tau in product((-1, 1), repeat=4):
        for eta in product(grid, repeat=2):  # actors 0,1 vary; 2,3 reuse
            etas = (eta[0], eta[1], eta[0], eta[1])
            for delta, omega, gamma in product((F(1, 4), F(1)), (F(1, 2), F(2)), (F(1), F(2))):
                x_S = lambda f, i: f - 1 + delta * tau[i]          # S action argument
                x_N = lambda f, nu_i: f - 1 + nu_i                  # N action argument
                for f in (F(0), F(1, 2)):
                    eq &= all(x_S(f, i) == x_N(f, delta * tau[i]) for i in range(4))
                for s in product((0, 1), repeat=4):
                    for i, j in product(range(4), repeat=2):
                        if i == j:
                            continue
                        a = val_arg(gamma, s[j], etas[i], True, omega, tau[i])
                        b = val_arg(gamma, s[j], etas[i] + omega * tau[i])
                        eq &= a == b
                        n += 1
    rec("S/N", f"S(delta, omega, eta) == N(nu_i=delta*tau_i, eta_i^N=eta_i+omega*tau_i): identical action and valuation "
               f"arguments on {n} (config, pair) cases -> identical joint law whenever N may correlate eta with nu "
               "and eta_max >= |eta_i + omega*tau_i|",
        "FINDING" if eq else False)
    rec("S/N", "the random arm discards valuations by rule, so S's omega has no observable effect there: "
               "S == N with two-point nu in that arm", "FINDING")


# --------------------------------------------------------------------------- L sign
def l_sign_checks(T=4):
    # (i) q stays in [0,1]: each update is a convex combination of q and a partner action in {0,1}
    #     with weight alpha*w in [0,1]. Checked exhaustively for short histories as a regression test.
    in_unit = True
    for q1, alpha, zeta in product((F(0), F(1, 3), F(1)), (F(0), F(1, 2), F(1)), (F(0), F(1, 2), F(1))):
        for fs in product((F(0), F(1, 2)), repeat=T):
            for ph in product((0, 1), repeat=T):
                for t in range(T + 1):
                    q = L_q(t, ph, fs, q1, alpha, zeta)
                    in_unit &= 0 <= q <= 1
    rec("L", "q_i,t stays in [0,1] on every enumerated history (convex-combination update)", in_unit)
    # (ii) Algebra, not a grid: for any real kappa_L and q_a, q_b in [0,1],
    #      (kappa_L q_a)(kappa_L q_b) = kappa_L^2 q_a q_b >= 0, so two L offsets never have opposite strict signs.
    target = (F(1, 2), F(-1, 2))
    rec("L", "with a common kappa_L, all L offsets kappa_L*q_i,t share one sign at every t (kappa_L^2 q_a q_b >= 0); "
             f"N with nu = {tuple(str(x) for x in target)} has offset product {target[0] * target[1]} < 0, so no L "
             "parameter value represents it at any alpha_L: mixed-sign heterogeneity separates N and S from L by domain",
        "FINDING" if (in_unit and target[0] * target[1] < 0) else False)


# --------------------------------------------------------------------------- C/L fixed match, R/M
def L_q(t, partner_hist, fines, q1, alpha, zeta):
    q = F(q1)
    for k in range(t):
        w = 1 - F(zeta) * (1 if fines[k] > 0 else 0)
        q = (1 - F(alpha) * w) * q + F(alpha) * w * partner_hist[k]
    return q


def cl_and_rm_checks(T=4):
    fine_sched = list(product((F(0), F(1, 2)), repeat=T))
    hists = list(product((0, 1), repeat=T - 1))
    # C/L under fixed matching with v0.2 t=1 neutral prior s=1/2: equal for homogeneous kappa
    ok = True
    for kappa in (F(-1), F(1, 2), F(3, 2)):
        for fs in fine_sched:
            for ph in hists:
                for t in range(T):
                    s_t = F(1, 2) if t == 0 else F(ph[t - 1])
                    xc = fs[t] - 1 + kappa * s_t
                    xl = fs[t] - 1 + kappa * L_q(t, ph, fs, F(1, 2), 1, 0)
                    ok &= xc == xl
    rec("C/L", f"fixed matching, alpha_L=1, zeta=0, q1 = neutral prior 1/2: C == L for a COMMON kappa "
               f"({len(fine_sched) * len(hists) * T * 3} nodes)", ok)
    # Heterogeneous C: at t=1 (s = 1/2) actor i's argument d + kappa_i/2 equals L's d + kappa_L/2 only if
    # kappa_L = kappa_i, so the matching kappa_L is unique per actor; two actors need two different values.
    kappas = (F(1, 2), F(3, 2))
    needed = {2 * ((F(0) - 1 + k * F(1, 2)) - (F(0) - 1)) for k in kappas}  # solve d + kappa_L/2 = d + kappa_i/2
    rec("C/L", f"C actors with kappa = {tuple(str(k) for k in kappas)} each force a different kappa_L "
               f"({sorted(str(k) for k in needed)}); a common kappa_L matches at most one -> the fixed-match "
               "equivalence covers only homogeneous-kappa C", "FINDING" if len(needed) == len(kappas) else False)
    # R (v0.2): contextual Q(a,f), all Q_1 = 0. A_1 and Qmax are unspecified in v0.2; placeholders A_1=0, Qmax=10.
    def R_args(alpha, own, par, fs, A1=F(0), lam=F(1, 2), Qmax=F(10)):
        Q = {(a, f): F(0) for a in (0, 1) for f in (F(0), F(1, 2))}
        A, args = A1, []
        for t in range(len(fs)):
            f = fs[t]
            args.append(f - 1 + Q[(1, f)] - Q[(0, f)])
            if t < len(own):
                uu = u(own[t], par[t], f)
                Q[(own[t], f)] = max(-Qmax, min(Qmax, Q[(own[t], f)] + alpha * (uu - A)))
                A = (1 - lam) * A + lam * uu
        return args
    n_nodes, eq0, moved = 0, True, False
    for fs in fine_sched:
        for own in product((0, 1), repeat=T - 1):
            for par in product((0, 1), repeat=T - 1):
                m_args = [f - 1 for f in fs]
                eq0 &= R_args(F(0), own, par, fs) == m_args
                moved |= R_args(F(1, 2), own, par, fs) != m_args
                n_nodes += T
    rec("R/M", f"R with Q_1=0 equals M at t=1 and, for alpha_R=0, at every t on {n_nodes} enumerated nodes; "
               "with alpha_R=1/2 it departs once a fine context recurs", eq0 and moved)


def cost_note():
    J, designs, fam, B = 400, 2, 6, 200
    fits = J * designs * fam * fam * (B + 1)
    rec("cost", f"illustration: J={J} datasets per (design, true family), 2 designs, 6x6 fits, B={B} bootstrap refits "
                f"-> {fits:,} model fits", "NOTE")


def main():
    payoff_checks()
    valuation_checks()
    broker_checks()
    s_vs_n_checks()
    l_sign_checks()
    cl_and_rm_checks()
    cost_note()
    for st, tag, msg in OUT:
        print(f"[{st:7}] {tag:9} {msg}")
    nf = sum(1 for o in OUT if o[0] == "FAIL")
    print(f"\n{len(OUT)} checks: {nf} FAIL, {sum(1 for o in OUT if o[0] == 'FINDING')} FINDING, "
          f"{sum(1 for o in OUT if o[0] == 'NOTE')} NOTE, rest PASS")
    return 1 if nf else 0


if __name__ == "__main__":
    raise SystemExit(main())
