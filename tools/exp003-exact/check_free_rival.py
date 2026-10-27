#!/usr/bin/env python3
"""Same-author stdlib check of Exp003 selected N vs M T1 analytic and grid results.

Independently expresses *homogeneous* one-atom action-vector probabilities.
No Rust source import. No C/L cross-language optimization or full-H4 verifier.
"""
import math
import unittest

def logistic(x):
    return 1.0 / (1.0 + math.exp(-x))

def p_m(fine, beta, epsilon):
    return epsilon / 2 + (1 - epsilon) * logistic(beta * (fine - 1))

def p_n(fine):
    return logistic(4 * (fine - 1 + .75))

def law(fines, action_mask, prob):
    result = 1.
    for i in range(4):
        p = prob(.5 * fines[0 if i < 2 else 1])
        result *= p if action_mask & (1 << i) else 1 - p
    return result

def full_t1_tv(policy, beta, epsilon):
    fine_states = ((0, 0),) if policy == 'A0' else (
        (0, 0), (1, 0), (0, 1), (1, 1)
    )
    # Homogeneous actor-iid one-atom: the three matchings are relabelings,
    # all with 1/3 and both regimes under A have equal T1 broker probabilities.
    # Hence sum over matching/regime cancels and averaging fine states is exact.
    difference = 0.
    for fines in fine_states:
        for mask in range(16):
            difference += abs(
                law(fines, mask, p_n) -
                law(fines, mask, lambda f: p_m(f, beta, epsilon))
            )
    return difference / (2 * len(fine_states))

class SelectedWitness(unittest.TestCase):
    def test_analytic_unrestricted_m_event_bound(self):
        m_ceiling=.25 + .5 * logistic(-.05)
        n_half=p_n(.5)
        tv_bound=.5 * (n_half - m_ceiling)
        self.assertAlmostEqual(m_ceiling,.49375130175789483,places=14)
        self.assertAlmostEqual(tv_bound,.11865363843605503,places=14)
        self.assertGreater(tv_bound,0)
        # The bound depends on the permitted beta and epsilon endpoints,
        # NOT on homogeneous fixed rivals or the finite grid.
        for beta in (.1,1.,10.):
            for eps in (0.,.25,.5):
                self.assertLessEqual(p_m(.5,beta,eps),m_ceiling+1e-15)

    def test_full_grid_under_a_and_a0(self):
        for policy,expected in (('A',.33386582682695076),('A0',0.)):
            best=(float('inf'),None,None)
            evaluated=0
            for bi in range(1,101):
                for ei in range(21):
                    beta=bi / 10
                    eps=ei / 40
                    residual=full_t1_tv(policy,beta,eps)
                    evaluated+=1
                    if residual < best[0]:
                        best=(residual,beta,eps)
            with self.subTest(policy=policy):
                self.assertEqual(evaluated,2100)
                self.assertAlmostEqual(best[0],expected,places=13)
                self.assertEqual(best[1:],(1.0,0.0))

if __name__ == '__main__':
    unittest.main(verbosity=2)
