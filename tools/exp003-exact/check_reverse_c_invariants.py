#!/usr/bin/env python3
"""Exp 003 A2 v0.2.2: source-separated scalar checks of reverse-C witnesses.

INPUTS: Frozen J/H4 + v0.2.1 offsets/updating formula, not Rust executable.
This is author-side mathematical corroboration, NOT an isolated P0 reviewer,
human data, a 15-pair completion, or a confidence set.

Each reported lower bound follows from an event B subset D and a universal
conditional upper bound on C cooperation:
 h = 1_B - Cmax * 1_D, range(h) of length 1.
 E_C[h] <= 0; TV(P_truth,P_C) >= E_truth[h].
"""
import json
import math
import sys
import unittest


def sigma(x):
    return 1.0 / (1.0 + math.exp(-x))


# All legal C H4 mixtures and any public history / matching at t >= 2:
# when the current partner's previous public action is 0 and fine_i=0,
# C offset kappa_i * 0 = 0 independent of every latent parameter.
# p_C = eps/2+(1-eps)*sigma(-beta), with 0.1<=beta<=10, 0<=eps<=0.5.
# Monotone decreasing in beta, increasing in eps; attainable supremum:
C_CEILING = 0.25 + 0.5 * sigma(-0.1)
Q_R_T1 = sigma(-1.0)


def n_witness(policy):
    """All-H4 C separation via conditional partner-previous-defection cell at t2."""
    assert policy in ("A", "A0")
    beta, nu, epsilon = 1.0, 1.5, 0.0
    p_no_fine = epsilon / 2 + (1-epsilon)*sigma(beta*(-1+nu))
    p_half_fine = epsilon / 2 + (1-epsilon)*sigma(beta*(-0.5+nu))
    # t1 broker is uniform under the neutral-prior design. Exogenous
    # fine has marginal 1/2 under A, no fine under A0; all actors equal.
    p_actor1_t1 = (p_no_fine+p_half_fine)/2 if policy == "A" else p_no_fine
    # D={z2=random, actor0 fine 0, current partner previous action=0}.
    # Under random matching partner is uniformly drawn from actors 1,2,3;
    # exchangeability of this particular homogeneous N truth suffices.
    regime_and_fine = 0.25 if policy == "A" else 1.0
    probability_D = regime_and_fine*(1-p_actor1_t1)
    bound = probability_D*(p_no_fine-C_CEILING)
    return dict(
        policy=policy,
        C_supremum=C_CEILING,
        model="selected homogeneous N(beta=1,eps=0,nu=1.5), identically embedded in S and L",
        family_witnesses={"N": {"nu": 1.5},
                          "S": {"tau": 1, "delta": 1.5, "omega": 0},
                          "L": {"kappa": 3, "alpha": 0, "zeta": 0}},
        truth_t2_cooperation_if_partner_previous_defection=p_no_fine,
        truth_t1_actor_cooperation_probability=p_actor1_t1,
        public_condition_D_probability=probability_D,
        TV_lower_bound_truth_vs_any_C_H4=bound,
        horizon_minimum=2,
    )


def payoff_coop_partner_coop(fine=0):
    return 2*1 - 1 - .5*fine*(1-1)


def r_witness(policy):
    """Strict R outside all C H4 at T>=3 by a positive-probability path.

    Path:
      t1 random mu0=01|23, fines [0,0], actions [1,1,0,0]
      t2 random mu1=02|13, fines [0,0], actions [1,0,1,0]
      t3 random mu0, actor0 fine 0; t2 actor1 (new partner) defected.
    Homogeneous R beta=1 eps=0 alpha_R=1 lambda_A=0, A1=Q1=0.
    q_actor0(1,0) evolves 0 -> 1 -> 2 after two successful
    cooperation-with-cooperator actions, so t3 p_R = sigma(1)>Cmax.
    """
    assert policy in ("A", "A0")
    q = Q_R_T1
    t1_action_mass = q**2 * (1-q)**2
    # t2 actor0 and actor1 each have Q(1,0)=1, so p=.5;
    # actors 2,3 previously defected for payoff 0 (Q(0,0) unchanged),
    # thus both have p=q. t2 public actions [1,0,1,0].
    t2_action_mass = 0.5*(1-0.5)*q*(1-q)
    mu_random_fines_zero = 1/24 if policy == "A" else 1/3
    t3_random_mu0_actor0_no_fine = 1/12 if policy == "A" else 1/3
    probability_D = (
        mu_random_fines_zero**2
        * t1_action_mass*t2_action_mass
        * t3_random_mu0_actor0_no_fine
    )
    aspiration = 0.0
    q_coop_zero_fine = 0.0
    for t in (1, 2):
        utility = payoff_coop_partner_coop()
        old_aspiration = aspiration
        q_coop_zero_fine = min(10, max(-10, q_coop_zero_fine+(utility-old_aspiration)))
        aspiration = old_aspiration  # lambda_A = 0
        assert abs(q_coop_zero_fine-t) < 1e-14
    p_truth = sigma(-1+q_coop_zero_fine)
    return dict(
        policy=policy,
        C_supremum=C_CEILING,
        model="selected homogeneous R(beta=1,eps=0,alpha_R=1,lambda_A=0)",
        public_path="t1 random mu0 fines00 actions1100; t2 random mu1 fines00 actions1010; t3 random mu0 actor0 fine0 (a1,t2=0)",
        t1_action_mass=t1_action_mass,
        t2_action_mass=t2_action_mass,
        public_condition_D_probability=probability_D,
        Q_actor0_coop_fine0_at_t3=q_coop_zero_fine,
        truth_t3_cooperation_if_current_partner_previous_defection=p_truth,
        TV_lower_bound_truth_vs_any_C_H4=probability_D*(p_truth-C_CEILING),
        horizon_minimum=3,
    )


def audit():
    return {
        "status": "AUTHOR_SIDE_NORMATIVE_EQUATION_AUDIT_NOT_INDEPENDENT_P0",
        "model": "Experiment003-A2-v0.2.2 J/H4",
        "proof_scope": {
            "C_ceiling": "0.25+0.5*sigmoid(-0.1), all H4 C at zero current fine and previous action of currently matched partner zero",
            "witness_noninclusions": [
                "N not subset C, A and A0, T>=2",
                "S not subset C, via legal N law embedding, A and A0, T>=2",
                "L not subset C, via legal N law embedding, A and A0, T>=2",
                "R not subset C, via explicit R history, A and A0, T>=3",
            ],
            "unproved": [
                "R not subset C at T=2",
                "independent P0 approval or full 15-pair classification",
                "95% H4 confidence-set coverage and finite-sample power",
                "human sociological generalization",
            ],
        },
        "N_S_L": [n_witness(p) for p in ("A", "A0")],
        "R": [r_witness(p) for p in ("A", "A0")],
    }


class ReverseInclusionTests(unittest.TestCase):
    def test_universal_C_ceiling_and_parameter_bounds(self):
        self.assertAlmostEqual(C_CEILING, 0.48751040626053, places=14)
        self.assertLess(C_CEILING, 0.5)
        for beta in (0.1, 0.2, 1, 5, 10):
            for eps in (0, 0.25, 0.5):
                p = eps/2 + (1-eps)*sigma(-beta)
                self.assertLessEqual(p, C_CEILING+1e-14)

    def test_selected_N_S_L_witnesses(self):
        for policy, expected in (
            ("A", 0.010905257882751379),
            ("A0", 0.050948707375938415),
        ):
            d = n_witness(policy)
            self.assertGreater(d["truth_t2_cooperation_if_partner_previous_defection"], 0.5)
            self.assertAlmostEqual(d["TV_lower_bound_truth_vs_any_C_H4"], expected, places=14)
            self.assertEqual(d["family_witnesses"]["S"]["delta"], 1.5)
            # S uses δτ=ν; L with α=0 retains q=1/2 so κq=ν.
            self.assertEqual(1.5*1, 1.5)
            self.assertEqual(3*0.5, 1.5)

    def test_R_public_history_is_realizable_and_updates_timing(self):
        self.assertEqual(payoff_coop_partner_coop(), 1)
        for policy, expected in (
            ("A", 6.69500298067994e-08),
            ("A0", 1.7139207630540646e-05),
        ):
            d = r_witness(policy)
            self.assertEqual(d["Q_actor0_coop_fine0_at_t3"], 2)
            self.assertGreater(d["truth_t3_cooperation_if_current_partner_previous_defection"], 0.5)
            self.assertGreater(d["public_condition_D_probability"], 0)
            self.assertAlmostEqual(d["TV_lower_bound_truth_vs_any_C_H4"], expected, places=18)

    def test_variational_witness_normalization(self):
        # h=1_B-Cmax*1_D takes only these three values.
        possible = [0, -C_CEILING, 1-C_CEILING]
        self.assertAlmostEqual(max(possible)-min(possible), 1)
        for report in audit()["N_S_L"]+audit()["R"]:
            self.assertGreater(report["TV_lower_bound_truth_vs_any_C_H4"], 0)


if __name__ == "__main__":
    if "--json" in sys.argv:
        print(json.dumps(audit(), indent=2, sort_keys=True))
    else:
        unittest.main(verbosity=2)
