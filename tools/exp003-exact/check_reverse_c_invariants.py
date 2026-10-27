#!/usr/bin/env python3
"""Experiment 003 A2 v0.2.2 author-side reverse-C analytic witnesses.

Independently reconstructs the four-actor R context Q/aspiration transitions
from publicly specified actions, pairings and payoffs. No Rust model import.
Selected analytic event inequalities cover ALL C H4 rivals, not only an
enumerated nuisance grid. Neither this file nor passing checks count as an
independently isolated P0 mathematical audit or any statistical calibration.
"""
import copy
import json
import math
import sys
import unittest

PAIRS = (((0, 1), (2, 3)), ((0, 2), (1, 3)), ((0, 3), (1, 2)))
C_CEILING = 0.25 + 0.5 / (1.0 + math.exp(0.1))


def sigma(x):
    return 1.0 / (1.0 + math.exp(-x))


def matched_partner(mu, i):
    for a, b in PAIRS[mu]:
        if i == a:
            return b
        if i == b:
            return a
    raise ValueError("actor not matched")


def dyad(mu, i):
    return next(d for d, pair in enumerate(PAIRS[mu]) if i in pair)


def payoff(mu, bits, actions, i):
    a = actions[i]
    return 2 * actions[matched_partner(mu, i)] - a - (bits[dyad(mu, i)] / 2) * (1 - a)


def fresh_R():
    return {"q": [[[0.0, 0.0], [0.0, 0.0]] for _ in range(4)],
            "aspiration": [0.0] * 4}


def R_probabilities(state, mu, bits, beta=1.0, epsilon=0.0):
    """Full simultaneous action-vector Bernoulli probabilities for four actors."""
    values = []
    for i in range(4):
        bit = bits[dyad(mu, i)]
        h = state["q"][i][1][bit] - state["q"][i][0][bit]
        values.append(epsilon / 2 + (1 - epsilon) * sigma(beta * (bit / 2 - 1 + h)))
    return values


def vector_likelihood(probabilities, actions):
    return math.prod(p if a else (1-p) for p, a in zip(probabilities, actions))


def R_transition(state, mu, bits, actions, alpha=1.0, lam=0.0):
    """All 4 actors update once, using the PRE-update aspiration and own payoff."""
    nxt = copy.deepcopy(state)
    for i in range(4):
        bit = bits[dyad(mu, i)]
        act = actions[i]
        utility = payoff(mu, bits, actions, i)
        old_asp = state["aspiration"][i]
        value = state["q"][i][act][bit] + alpha * (utility - old_asp)
        nxt["q"][i][act][bit] = max(-10.0, min(10.0, value))
        nxt["aspiration"][i] = (1 - lam) * old_asp + lam * utility
    return nxt


def assignment_mass(policy, require_both_fines):
    """Probability of a specific publicly observed random regime and matching."""
    regime = 0.5 if policy == "A" else 1.0
    random_mu = regime / 3
    if policy == "A":
        return random_mu * (0.25 if require_both_fines else 0.5)
    return random_mu


def n_witness(policy):
    """N S L selected one-atom law outside full C at t2."""
    assert policy in ("A", "A0")
    p0 = sigma(0.5)
    p_half = sigma(1.0)
    first_cooperation = (p0 + p_half) / 2 if policy == "A" else p0
    probability_D = (0.25 if policy == "A" else 1.0) * (1-first_cooperation)
    return {
        "policy": policy, "C_supremum": C_CEILING,
        "selected_N": {"beta": 1.0, "epsilon": 0.0, "nu": 1.5},
        "legal_S_embedding": {"tau": 1, "delta": 1.5, "omega": 0},
        "legal_L_embedding": {"kappa": 3.0, "alpha": 0.0, "zeta": 0.0},
        "truth_t2_cooperation_if_partner_previous_defection": p0,
        "truth_t1_actor_cooperation_probability": first_cooperation,
        "public_condition_D_probability": probability_D,
        "TV_lower_bound_truth_vs_any_C_H4": probability_D * (p0 - C_CEILING),
        "horizon_minimum": 2,
    }


def r_witness(policy):
    """Witness R outside all C at T>=2, plus historical T3 check.

    t1: (random, mu0=01|23, fine bits 00, actions 1100).
    t2 for 2-period event: (random, mu1=02|13, actor0 fine0).
    t2 complete history for legacy T3: fine bits 00, actions 1010.
    t3 legacy event: (random, mu0, actor0 fine0).
    """
    assert policy in ("A", "A0")
    state1 = fresh_R()
    a1 = (1, 1, 0, 0)
    probs1 = R_probabilities(state1, 0, (0, 0))
    t1_action_mass = vector_likelihood(probs1, a1)
    state2 = R_transition(state1, 0, (0, 0), a1)
    assert state2["q"][0][1][0] == 1
    assert state2["q"][1][1][0] == 1
    assert state2["q"][2][0][0] == 0
    assert state2["q"][3][0][0] == 0

    # t2 mu1 pairs actor0 with actor2, who previously defected at t1.
    # R actor0's Q-induced cooperation is 1/2 regardless of this new partner.
    assert matched_partner(1, 0) == 2 and a1[2] == 0
    probs2 = R_probabilities(state2, 1, (0, 0))
    p_at_t2 = probs2[0]
    first_allocation = assignment_mass(policy, require_both_fines=True)
    t2_actor0_fine0_allocation = assignment_mass(policy, require_both_fines=False)
    probability_D2 = first_allocation * t1_action_mass * t2_actor0_fine0_allocation
    bound2 = probability_D2 * (p_at_t2 - C_CEILING)

    # Preserve v0.1 T3 witness as a HISTORICAL nonminimal positive-probability
    # check. Evaluate all four actor probabilities and their state updates.
    a2 = (1, 0, 1, 0)
    t2_action_mass = vector_likelihood(probs2, a2)
    state3 = R_transition(state2, 1, (0, 0), a2)
    assert matched_partner(0, 0) == 1 and a2[1] == 0
    p_at_t3 = R_probabilities(state3, 0, (0, 0))[0]
    probability_D3 = (
        first_allocation**2 * t1_action_mass * t2_action_mass
        * t2_actor0_fine0_allocation
    )
    return {
        "policy": policy, "C_supremum": C_CEILING,
        "selected_R": {"beta": 1, "epsilon": 0, "alpha": 1, "lambda": 0},
        "t1_observed_record": {"regime": "Random", "matching": 0,
                               "fine_bits": [0,0], "actions": list(a1)},
        "t1_actor_probability_vector": probs1,
        "t1_action_probability": t1_action_mass,
        "t2_actor_probability_vector": probs2,
        "t2_conditional_actor0_cooperation": p_at_t2,
        "t2_public_condition_probability": probability_D2,
        "TV_lower_bound_truth_vs_any_C_H4": bound2,
        "horizon_minimum": 2,
        "legacy_T3_nonminimal": {
            "t2_full_actions": list(a2),
            "t2_action_probability": t2_action_mass,
            "actor0_Q_cooperate_fine0_at_t3": state3["q"][0][1][0],
            "t3_conditional_actor0_cooperation": p_at_t3,
            "t3_public_condition_probability": probability_D3,
            "TV_lower_bound_truth_vs_any_C_H4": probability_D3 * (p_at_t3-C_CEILING)
        },
    }


def audit():
    return {
        "status": "AUTHOR_SIDE_REVIEW_AMENDMENT_v02_NOT_INDEPENDENT_P0",
        "model": "Experiment003-A2-v0.2.2 J/H4",
        "scope": {
            "C_ceiling": "0.25 + 0.5 * sigmoid(-0.1), universal over any C H4 on public partner-previous-defect / current-fine0",
            "witness_directions_T_ge_2": ["N not subset C","S not subset C",
                                         "L not subset C","R not subset C"],
            "R_T3_v01": "valid historical witness, superseded in minimal horizon by T2",
            "unproved": ["independent P0 acceptance",
                         "complete 15-pair H4 identification",
                         "valid 95% H4 coverage or finite-sample power",
                         "human behavior generalization"]
        },
        "N_S_L": [n_witness(p) for p in ("A", "A0")],
        "R": [r_witness(p) for p in ("A", "A0")],
    }


class ReverseInclusionTests(unittest.TestCase):
    def test_universal_c_ceiling(self):
        self.assertAlmostEqual(C_CEILING, 0.48751040626053, places=14)
        self.assertLess(C_CEILING, 0.5)
        for beta in (.1,.2,1,5,10):
            for epsilon in (0,.25,.5):
                value=epsilon/2+(1-epsilon)*sigma(-beta)
                self.assertLessEqual(value,C_CEILING+1e-14)

    def test_n_s_l_embeddings_by_full_action_probability_cells(self):
        for policy, expected in (
            ("A",0.010905257882751379),
            ("A0",0.050948707375938415),
        ):
            r=n_witness(policy)
            self.assertAlmostEqual(r["TV_lower_bound_truth_vs_any_C_H4"],expected,places=14)
            self.assertGreater(r["public_condition_D_probability"],0)
            for fine in (0,0.5):
                for previous in (0,0.5,1):
                    # S beta=1 with delta*tau=1.5; L with alpha=0 has q=.5.
                    pN=sigma(fine-1+1.5)
                    pS=sigma(fine-1+r["legal_S_embedding"]["delta"]*r["legal_S_embedding"]["tau"])
                    pL=sigma(fine-1+r["legal_L_embedding"]["kappa"]*.5)
                    self.assertAlmostEqual(pN,pS,places=15)
                    self.assertAlmostEqual(pN,pL,places=15)

    def test_four_actor_R_state_and_two_period_noninclusion(self):
        for policy,mass,bound in (
            ("A",0.00013422309823941982,1.6763919674633255e-06),
            ("A0",0.004295139143661434,5.3644542958826415e-05),
        ):
            v=r_witness(policy)
            self.assertAlmostEqual(v["t2_public_condition_probability"],mass,places=17)
            self.assertAlmostEqual(v["TV_lower_bound_truth_vs_any_C_H4"],bound,places=17)
            self.assertEqual(v["horizon_minimum"],2)
            self.assertAlmostEqual(v["t2_conditional_actor0_cooperation"],0.5,places=15)
            self.assertEqual(v["t1_actor_probability_vector"],[sigma(-1)]*4)
            self.assertAlmostEqual(v["t2_actor_probability_vector"][0],.5)
            self.assertAlmostEqual(v["t2_actor_probability_vector"][1],.5)
            self.assertAlmostEqual(v["t2_actor_probability_vector"][2],sigma(-1))
            self.assertAlmostEqual(v["t2_actor_probability_vector"][3],sigma(-1))

    def test_T3_historical_R_path_retained_and_nonminimal(self):
        for policy,expected in (
            ("A",6.69500298067994e-08),
            ("A0",1.7139207630540646e-05),
        ):
            v=r_witness(policy)
            legacy=v["legacy_T3_nonminimal"]
            self.assertEqual(legacy["actor0_Q_cooperate_fine0_at_t3"],2)
            self.assertAlmostEqual(legacy["t3_conditional_actor0_cooperation"],sigma(1))
            self.assertAlmostEqual(legacy["TV_lower_bound_truth_vs_any_C_H4"],expected,places=17)
            self.assertGreater(v["TV_lower_bound_truth_vs_any_C_H4"],legacy["TV_lower_bound_truth_vs_any_C_H4"])

    def test_positive_history_payoffs_and_contextual_Q_timing(self):
        s=fresh_R()
        a1=(1,1,0,0)
        self.assertEqual([payoff(0,(0,0),a1,i) for i in range(4)],[1,1,0,0])
        s2=R_transition(s,0,(0,0),a1)
        self.assertEqual([s2["q"][i][1][0] for i in range(4)],[1,1,0,0])
        a2=(1,0,1,0)
        self.assertEqual([payoff(1,(0,0),a2,i) for i in range(4)],[1,0,1,0])
        s3=R_transition(s2,1,(0,0),a2)
        self.assertEqual([s3["q"][i][1][0] for i in range(4)],[2,1,1,0])
        self.assertEqual(s3["aspiration"],[0]*4)
        self.assertEqual(s3["q"][0][0],[0,0])

    def test_variational_witness_range_one_and_h4_bound(self):
        self.assertAlmostEqual((1-C_CEILING)-(-C_CEILING),1)
        for outcome in audit()["N_S_L"]+audit()["R"]:
            self.assertGreater(outcome["TV_lower_bound_truth_vs_any_C_H4"],0)


if __name__ == "__main__":
    if "--json" in sys.argv:
        print(json.dumps(audit(),indent=2,sort_keys=True))
    else:
        unittest.main(verbosity=2)
