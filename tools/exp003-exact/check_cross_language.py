#!/usr/bin/env python3
"""Same-author, separately expressed Python fixed-witness check for Exp003 v0.2.2.

Does not load the Rust simulator. Does NOT certify full H4 law-set inference.
"""
import math
import unittest

PAIRS = (((0, 1), (2, 3)), ((0, 2), (1, 3)), ((0, 3), (1, 2)))
PREVIOUS = (1, 0, 1, 0)

def logistic(x):
    return 1 / (1 + math.exp(-x))

def belief_after_first_matching(matching, actions, fines, *, alpha, zeta):
    """Compute L's POST-period-1 actor-specific belief, not a fixed lookup."""
    learned = [0.5] * 4
    for dyad, (i, j) in enumerate(PAIRS[matching]):
        effective_learning = alpha * (1 - zeta * fines[dyad])
        learned[i] = (1 - effective_learning) * learned[i] + effective_learning * actions[j]
        learned[j] = (1 - effective_learning) * learned[j] + effective_learning * actions[i]
    return tuple(learned)

def choice_broker(previous, eta=(0., 0., 0., 0.), rho=1.):
    if previous is None:
        return (1/3,)*3
    scores = []
    for pairs in PAIRS:
        scores.append(rho * sum(min(
            logistic((1/3 + eta[i]) * (2*previous[j]-1)),
            logistic((1/3 + eta[j]) * (2*previous[i]-1))
        ) for i, j in pairs))
    best = max(scores)
    exps = [math.exp(score-best) for score in scores]
    return tuple(x/sum(exps) for x in exps)

def probability_vector(policy, model, previous=None, *, learned=None,
                       epsilon=0., eta=(0., 0., 0., 0.), rho=1.):
    if model == 'L' and previous is not None and learned is None:
        raise ValueError('L post-period belief must be derived from a known matching history')
    """Every one of 384 regime/matching/fine/action categories, A0 nulls included."""
    result = []
    for regime in ('random','choice'):
        broker = (1/3,)*3 if regime=='random' else choice_broker(previous, eta, rho)
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
                            offset=.5 if previous is None else learned[actor]
                            beta=1
                        elif model=='M':
                            offset=0
                            beta=1
                        elif model=='N':
                            offset=.75
                            beta=4
                        else:
                            raise ValueError(model)
                        p=epsilon/2 + (1-epsilon)*logistic(beta*(assigned_fine-1+offset))
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
                learned = belief_after_first_matching(0, PREVIOUS, (0, 0),
                                                      alpha=1., zeta=0.)
                self.assertEqual(learned, (0., 1., 0., 1.))
                l=probability_vector(policy,'L',PREVIOUS, learned=learned)
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
        eta=(.2,-.1,.3,-.2)
        broker=choice_broker(PREVIOUS,eta=eta,rho=2.3)
        self.assertAlmostEqual(sum(broker),1)
        self.assertGreater(max(broker)-min(broker),0.01)
        with_epsilon=probability_vector(
            'A','C',PREVIOUS,eta=eta,rho=2.3,epsilon=.18
        )
        self.assertEqual(len(with_epsilon),384)
        self.assertAlmostEqual(sum(with_epsilon),1,places=14)
        self.assertAlmostEqual(logistic(0),.5)
        self.assertAlmostEqual(logistic(-1),.2689414213699951)
        self.assertAlmostEqual(logistic(1),.7310585786300049)
        # An M response with positive beta and epsilon<=0.5 is below 0.5:
        self.assertLess(.25+.5*logistic(-.05),.5)

    def test_belief_update_uses_previous_partner_and_fine_context(self):
        state=belief_after_first_matching(
            0,PREVIOUS,(1,0),alpha=.6,zeta=.5
        )
        for observed,expected in zip(state,(.35,.65,.2,.8)):
            self.assertAlmostEqual(observed,expected)
        self.assertEqual(belief_after_first_matching(
            0,PREVIOUS,(1,1),alpha=0.,zeta=1.),(.5,)*4)
        with self.assertRaisesRegex(ValueError,'derived'):
            probability_vector('A','L',PREVIOUS)

if __name__=='__main__':
    unittest.main(verbosity=2)
