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
            for index, current_matching in enumerate((0, 1)):
                # Evaluate each current matching as a distinct physical
                # t2 matching event; the L offset itself must not read the
                # freshly selected partner, but dyad indexing still uses it.
                current_pairs = PAIRS[current_matching]
                current_dyad = next(k for k, pair in enumerate(current_pairs) if 0 in pair)
                second_fines = (0, 0)
                fine_bit = second_fines[current_dyad]
                p2 = action_prob(fine_bit, actors[0]["kappa"], q2)
                joint[index] += e_prob * factor * p2
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

def brute_force_c(policy, first_actions):
    """Enumerate *all* t1 pairings/fines; no reference to analytic_c."""
    first_mass = 0.
    b = [0., 0.]
    second_regime_probability = .5 if policy == "A" else 1.
    second_actor_fine_zero = .5 if policy == "A" else 1.
    second_factor = second_regime_probability * (1/3) * second_actor_fine_zero
    for _, first_pairs, first_fines, weight in allocations(policy):
        # C's initial signal is fixed 1/2, but the two dyad fines can differ.
        t1prob = math.prod(
            sigmoid(.5* fine_of(first_fines, first_pairs, actor) - .5)
            if first_actions[actor] else
            1 - sigmoid(.5 * fine_of(first_fines, first_pairs, actor) - .5)
            for actor in range(4)
        )
        event_mass = weight*t1prob
        first_mass += event_mass
        for k, second_mu in enumerate((0, 1)):
            # C looks up the CURRENT partner's public PREVIOUS action.
            second_partner = next(j for pair in PAIRS[second_mu] if 0 in pair
                                  for j in pair if j != 0)
            second_cooperate = sigmoid(-1 + first_actions[second_partner])
            b[k] += event_mass * second_factor * second_cooperate
    return first_mass,b[0],b[1]

def pooled_c_bound(policy):
    """Use all 16 disjoint t1 action vectors; h is always in [-1,1]."""
    signed = 0.
    total_differing = 0.
    for action in itertools.product((0,1), repeat=4):
        if action[1] == action[2]:
            continue
        event,b0,b1=brute_force_c(policy,action)
        total_differing+=event
        signed+=(action[1]-action[2])*(b0-b1)
    return total_differing, abs(signed)/2

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

    def test_independently_enumerated_c_and_all_histories(self):
        for policy, p_different, improved in (
            ("A",.49000247480106296,.004717469806363266),
            ("A0",.470007424403189,.018099874571358386)
        ):
            with self.subTest(policy=policy):
                brute=brute_force_c(policy,FIRST)
                analytic=analytic_c(policy)
                for a,b in zip(brute,analytic):
                    self.assertAlmostEqual(a,b,places=14)
                p,bound=pooled_c_bound(policy)
                self.assertAlmostEqual(p,p_different,places=14)
                self.assertAlmostEqual(bound,improved,places=14)
                self.assertGreater(bound,abs(analytic[1]-analytic[2])/2*5)

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
