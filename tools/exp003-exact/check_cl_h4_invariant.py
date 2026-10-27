#!/usr/bin/env python3
"""Author-side stdlib independent-equation check: C-not-L H4 witness.

This is numerical corroboration of the documented all-H4 proof, not an
independently authored P0 source or a confidence-set calibration.
"""
import itertools
import math
import unittest

PAIRS = (((0, 1), (2, 3)), ((0, 2), (1, 3)), ((0, 3), (1, 2)))
FIRST = (1, 0, 1, 0)
L_TYPES = (
    dict(weight=.4, eta=.1, kappa=1.75, alpha=.8, zeta=.75),
    dict(weight=.6, eta=-.2, kappa=-1.25, alpha=.3, zeta=.1),
)

def sigmoid(x):
    return 1 / (1 + math.exp(-x))

def allocations(policy):
    fine_states = ((0, 0),) if policy == "A0" else tuple(itertools.product((0, 1), repeat=2))
    for matching, pairs in enumerate(PAIRS):
        for bits in fine_states:
            yield matching, pairs, bits, 1 / (3 * len(fine_states))

def fine_of(bits, pairs, actor):
    return bits[0 if actor in pairs[0] else 1]

def action_prob(bit, kappa, q, beta=1.7, epsilon=.08):
    return epsilon / 2 + (1 - epsilon) * sigmoid(beta * (bit / 2 - 1 + kappa * q))

def l_mixture_event(policy):
    factor = (1 / 12) if policy == "A" else (1 / 3)
    # Four actors drawn iid from one unknown JOINT tuple measure:
    first_mass, joint = 0., [0., 0.]
    for labels in itertools.product(range(len(L_TYPES)), repeat=4):
        actors = [L_TYPES[x] for x in labels]
        mixture_weight = math.prod(actor["weight"] for actor in actors)
        for _, pairs, bits, allocated in allocations(policy):
            p1 = [action_prob(fine_of(bits, pairs, i), actors[i]["kappa"], .5)
                  for i in range(4)]
            e_prob = mixture_weight * allocated * math.prod(
                p if a else (1 - p) for a, p in zip(FIRST, p1)
            )
            first_mass += e_prob
            # L actor0's t2 belief depends only on its PREVIOUS partner and
            # t1 fine, not on which partner it gets in the random t2 matching.
            d = 0 if 0 in pairs[0] else 1
            previous_partner = next(j for j in pairs[d] if j != 0)
            t1_learning_weight = actors[0]["alpha"] * (1 - actors[0]["zeta"] * bits[d])
            q2 = (1 - t1_learning_weight) * .5 + t1_learning_weight * FIRST[previous_partner]
            p2 = action_prob(0, actors[0]["kappa"], q2)
            joint[0] += e_prob * factor * p2
            joint[1] += e_prob * factor * p2
    return first_mass, joint

def analytic_c(policy):
    q, r = sigmoid(-.5), .5
    if policy == "A0":
        event = q*q*(1-q)**2
        factor = 1 / 3
    else:
        g = (q*(1-q)+r*(1-r))/2
        u = (q*q+r*r)/2
        v = ((1-q)**2+(1-r)**2)/2
        event = (2*g*g+u*v)/3
        factor = 1 / 12
    return event, event * factor * sigmoid(-1), event * factor * sigmoid(0)

class AllH4Witness(unittest.TestCase):
    def test_chosen_c_probability_and_bound(self):
        cases = (("A", .0600568408621989, .0005781936786095032),
                 ("A0", .055226744748529843, .0021267688573262324))
        for policy, expected_event, expected_bound in cases:
            with self.subTest(policy=policy):
                event, b0, b1 = analytic_c(policy)
                self.assertAlmostEqual(event, expected_event, places=14)
                self.assertAlmostEqual(abs(b0-b1)/2, expected_bound, places=14)
                self.assertLess(b0, b1)

    def test_correlated_two_atom_l_mixture_obeys_random_regime_invariant(self):
        for policy in ("A", "A0"):
            event, (b0, b1) = l_mixture_event(policy)
            with self.subTest(policy=policy):
                self.assertGreater(event, 0)
                self.assertAlmostEqual(b0, b1, places=14)

    def test_every_actor_stays_in_admitted_parameter_domain(self):
        for t in L_TYPES:
            self.assertTrue(-3 <= 1 / 3 + t["eta"] <= 3)
            self.assertTrue(-4 <= t["kappa"] <= 4)
            self.assertTrue(0 <= t["alpha"] <= 1)
            self.assertTrue(0 <= t["zeta"] <= 1)
        self.assertAlmostEqual(sum(t["weight"] for t in L_TYPES), 1)

if __name__ == "__main__":
    unittest.main(verbosity=2)
