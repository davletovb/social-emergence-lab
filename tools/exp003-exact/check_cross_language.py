#!/usr/bin/env python3
"""Same-author, separately expressed Python fixed-witness check for Exp003 v0.2.2.

Does not load the Rust simulator. Does NOT certify full H4 law-set inference.
"""
import math
import unittest

PAIRS = (((0, 1), (2, 3)), ((0, 2), (1, 3)), ((0, 3), (1, 2)))
PREVIOUS = (1, 0, 1, 0)
LEARNED = (0, 1, 0, 1)

def logistic(x):
    return 1 / (1 + math.exp(-x))

def choice_broker(previous):
    if previous is None:
        return (1/3,)*3
    scores = []
    for pairs in PAIRS:
        scores.append(sum(min(logistic((2*previous[j]-1)/3),
                              logistic((2*previous[i]-1)/3)) for i, j in pairs))
    best = max(scores)
    exps = [math.exp(score-best) for score in scores]
    return tuple(x/sum(exps) for x in exps)

def probability_vector(policy, model, previous=None):
    """Every one of 384 regime/matching/fine/action categories, A0 nulls included."""
    result = []
    for regime in ('random','choice'):
        broker = (1/3,)*3 if regime=='random' else choice_broker(previous)
        for mu,pairs in enumerate(PAIRS):
            partner,dyad={},{}
            for index,(left,right) in enumerate(pairs):
                partner[left],partner[right]=right,left
                dyad[left]=dyad[right]=index
            for fine_id in range(4):
                fine=(fine_id&1,(fine_id>>1)&1)
                for mask in range(16):
                    if policy=='A0' and (regime=='choice' or fine_id):
                        result.append(0.)
                        continue
                    mass=broker[mu]*(.125 if policy=='A' else 1.)
                    for actor in range(4):
                        assigned_fine=.5*fine[dyad[actor]]
                        if model=='C':
                            offset=.5 if previous is None else previous[partner[actor]]
                            beta=1
                        elif model=='L':
                            offset=.5 if previous is None else LEARNED[actor]
                            beta=1
                        elif model=='M':
                            offset=0
                            beta=1
                        elif model=='N':
                            offset=.75
                            beta=4
                        else:
                            raise ValueError(model)
                        p=logistic(beta*(assigned_fine-1+offset))
                        mass*=p if (mask>>actor)&1 else 1-p
                    result.append(mass)
    return result

def tv(a,b):
    return sum(abs(x-y) for x,y in zip(a,b))/2

class ReferenceChecks(unittest.TestCase):
    def test_c_l_rematching(self):
        for policy,expected in [('A',0.13021565405157567),('A0',0.11552928931500243)]:
            with self.subTest(policy=policy):
                c=probability_vector(policy,'C',PREVIOUS)
                l=probability_vector(policy,'L',PREVIOUS)
                self.assertAlmostEqual(sum(c),1)
                self.assertAlmostEqual(sum(l),1)
                self.assertAlmostEqual(tv(c,l),expected,places=14)

    def test_m_n_first_period(self):
        for policy,expected in [('A',0.33386582682695076),('A0',0.)]:
            with self.subTest(policy=policy):
                m=probability_vector(policy,'M')
                n=probability_vector(policy,'N')
                self.assertAlmostEqual(sum(m),1)
                self.assertAlmostEqual(sum(n),1)
                self.assertAlmostEqual(tv(m,n),expected,places=14)

    def test_broker_and_marginals(self):
        self.assertAlmostEqual(sum(choice_broker(PREVIOUS)),1)
        self.assertAlmostEqual(logistic(0),.5)
        self.assertAlmostEqual(logistic(-1),.2689414213699951)
        self.assertAlmostEqual(logistic(1),.7310585786300049)
        self.assertLess(.25+.5*logistic(-.05),.5)

if __name__=='__main__':
    unittest.main(verbosity=2)
