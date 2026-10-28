#!/usr/bin/env python3
"""Experiment 003: selected H4 law witnesses from time/fine invariants.

AUTHOR-side source-separate stdlib enumeration, not blinded independent P0.
Enumerates *every* T1 public matching/fine/action cell with actor-level R/L
updates and 4-actor joint action probabilities; no simulator imports, no RNG.
The all-H4 rival bounds are analytic consequences of the frozen model law.
"""
import itertools
import json
import math
import unittest

PAIRS=(((0,1),(2,3)),((0,2),(1,3)),((0,3),(1,2)))
BETA=1.0
C0=.25+.5/(1+math.exp(.1))
def sig(x):
    return 1/(1+math.exp(-x))
def partner(mu,i):
    return next(j for pair in PAIRS[mu] if i in pair for j in pair if j!=i)
def fine_index(mu,i):
    return next(d for d,pair in enumerate(PAIRS[mu]) if i in pair)
def fine_amount(mu,bits,i):
    return bits[fine_index(mu,i)]/2
def p_action(fine,offset):
    return sig(fine-1+offset)
def p_tuple(probabilities,acts):
    return math.prod(p if a else 1-p for p,a in zip(probabilities,acts))
def cells(policy):
    fines=((0,0),) if policy=="A0" else tuple(itertools.product((0,1),repeat=2))
    for mu in range(3):
        for bits in fines:
            for acts in itertools.product((0,1),repeat=4):
                yield mu,bits,acts,1/(3*len(fines))
def reward(mu,bits,acts,i):
    a=acts[i];other=acts[partner(mu,i)]
    return 2*other-a-fine_amount(mu,bits,i)*(1-a)
def new_R():
    return {"q":[[[0.,0.],[0.,0.]] for _ in range(4)],
            "asp":[0.]*4}
def advance_R(state,mu,bits,acts):
    out={"q":[[[x for x in row] for row in block] for block in state["q"]],
         "asp":state["asp"][:]}
    for i in range(4):
        fine=bits[fine_index(mu,i)]
        a=acts[i]
        payoff=reward(mu,bits,acts,i)
        old_asp=state["asp"][i]
        out["q"][i][a][fine]=min(10,max(-10,state["q"][i][a][fine]+(payoff-old_asp)))
        # lambda_A=0, aspiration remains as was
        out["asp"][i]=old_asp
    return out
def p_R_actor0(state,finebit):
    return p_action(finebit/2,state["q"][0][1][finebit]-state["q"][0][0][finebit])
def advance_L(beliefs,mu,bits,acts):
    # selected L alpha=1 zeta=0 -> q_i,t+1 = previous partner's act
    return [float(acts[partner(mu,i)]) for i in range(4)]
def p_L_actor0(belief,finebit):
    return p_action(finebit/2,3*belief)
def t1_enumerated(policy,family):
    assert family in ("R","L")
    pi0=0.
    pi2=0.
    p_first=0.
    for mu,bits,acts,allocation in cells(policy):
        if family=="R":
            state=new_R()
            actor_probs=[p_action(fine_amount(mu,bits,i),0) for i in range(4)]
        else:
            state=[.5]*4
            actor_probs=[p_action(fine_amount(mu,bits,i),3*.5) for i in range(4)]
        mass=allocation*p_tuple(actor_probs,acts)
        p_first+=mass
        pi0+=mass*p_action(0,0 if family=="R" else 1.5)
        if family=="R":
            after=advance_R(state,mu,bits,acts)
            pi2+=mass*p_R_actor0(after,0)
        else:
            after=advance_L(state,mu,bits,acts)
            pi2+=mass*p_L_actor0(after[0],0)
    assert abs(p_first-1)<1e-12
    return pi0,pi2

def analytic_R_rates(policy):
    q=sig(-1)
    next_zero=(
        q*q*sig(0)
        +q*(1-q)*sig(-2)
        +(1-q)*q*sig(-3)
        +(1-q)**2*q
    )
    p2=(q+next_zero)/2 if policy=="A" else next_zero
    return q,p2

def analytic_L_rates(policy):
    p0=sig(.5)
    prior_coop=(sig(.5)+sig(1))/2 if policy=="A" else sig(.5)
    p2=prior_coop*sig(2)+(1-prior_coop)*sig(-1)
    return p0,p2

def time_event_weight(policy):
    # P(actor0 fine zero), marginalizing both current matching regimes.
    # Action offset is partner-independent for M/N/S/R/L, and fines are
    # assigned independently AFTER matching, so no z restriction is needed.
    return .5 if policy=="A" else 1.

def selected_N_outside_R(policy):
    # Every R truth at t1 has zero Q: p(no fine) <= c*.
    # The selected N/S/L mapped truth has p=sig(.5) under no fine.
    p=sig(.5)
    return time_event_weight(policy)*(p-C0)

def selected_R_outside_N_and_S(policy):
    # For every N/S H4 rival, P(f0t=0, a0t=1) across both
    # current regimes is identical at t1 and t2.
    t1,t2=t1_enumerated(policy,"R")
    ref1,ref2=analytic_R_rates(policy)
    assert abs(t1-ref1)<1e-12 and abs(t2-ref2)<1e-12
    assert t2<t1
    # h=1_{A1}-1_{A2} in [-1,1], oscillation <=2.
    return time_event_weight(policy)*(t1-t2)/2

def selected_L_outside_N_and_S(policy):
    t1,t2=t1_enumerated(policy,"L")
    ref1,ref2=analytic_L_rates(policy)
    assert abs(t1-ref1)<1e-12 and abs(t2-ref2)<1e-12
    assert t2>t1
    return time_event_weight(policy)*(t2-t1)/2

def selected_R_outside_L_under_A():
    # H=public t1 random/mu0/fines00/actions1100;
    # marginalize BOTH t2 matching regimes and all matchings while comparing
    # independently assigned fine actor0=0 versus fine actor0=1/2.
    # Every L H4 has nondecreasing conditional p(f), since L belief
    # is fixed before t2 fine; current fine is independent of latent.
    # R with contextual Q after reward 1 has reversed fine effect.
    q=sig(-1)
    first_public_mass=(1/2)*(1/3)*(1/4)*q*q*(1-q)*(1-q)
    state=advance_R(new_R(),0,(0,0),(1,1,0,0))
    p0=p_R_actor0(state,0)
    ph=p_R_actor0(state,1)
    assert abs(p0-.5)<1e-14 and ph<p0
    # Current broker probabilities sum to one in either regime; conditional
    # actor response uses only its OWN prior Q or L belief and current fine.
    signed=first_public_mass*.5*(p0-ph)
    # two disjoint fine0/fine1 events, h=1_Bzero-1_Bhalf ∈[-1,1]
    return {"P_first_public_H":first_public_mass,
            "R_t2_zero_fine":p0,"R_t2_half_fine":ph,
            "signed_fine_reversal_contrast":signed,
            "TV_lower_bound_vs_any_L_H4":signed/2}


def independent_actor_moment_audit():
    """The suggested N/S t1 Hankel test confuses iid actors and shared type."""
    nus=(-1.5,0.0,1.5)
    ps=[sig(nu-1) for nu in nus]
    weights=[1/3]*3
    latent=[sum(w*p**k for w,p in zip(weights,ps)) for k in range(5)]
    observed=[]
    for k in range(5):
        # Each of k separate actors has an independent H4 latent draw.
        observed.append(sum(math.prod(weights[i]*ps[i] for i in labels)
                            for labels in itertools.product(range(3),repeat=k)))
    def det3(ms):
        a,b,c=([ms[i+j] for j in range(3)] for i in range(3))
        return a[0]*(b[1]*c[2]-b[2]*c[1])-a[1]*(b[0]*c[2]-b[2]*c[0])+a[2]*(b[0]*c[1]-b[1]*c[0])
    avg=observed[1]
    beta_s=-math.log(avg/(1-avg))
    assert .1<=beta_s<=10
    return {
        "truth":"N three equal iid actor atoms with eta=0, nu=-1.5,0,1.5; beta=1 eps=0",
        "sampling":"independent actor draws from deterministic G, NOT a common latent type",
        "latent_E_p_powers_unobserved":latent,
        "observable_first_k_cooperators_no_fines":observed,
        "latent_hankel_determinant":det3(latent),
        "observable_hankel_determinant":det3(observed),
        "A0_T1_exact_S_alias":"one S atom, delta=omega=eta=epsilon=0",
        "A0_T1_exact_S_alias_beta":beta_s,
        "scope":"the proposed cross-actor moment witness is invalid, no global N subset S claim"
    }

def evaluate():
    policies=[]
    for policy in ("A","A0"):
        R1,R2=t1_enumerated(policy,"R")
        L1,L2=t1_enumerated(policy,"L")
        policies.append({
            "policy":policy,
            "t1_and_t2_zero_fine_event_base_all_regimes":time_event_weight(policy),
            "selected_N_vs_any_R_T1_TV_lower_bound":selected_N_outside_R(policy),
            "selected_R_zero_fine_t1_cooperation":R1,
            "selected_R_zero_fine_t2_cooperation":R2,
            "selected_R_vs_any_N_or_S_T2_TV_lower_bound":selected_R_outside_N_and_S(policy),
            "selected_L_zero_fine_t1_cooperation":L1,
            "selected_L_zero_fine_t2_cooperation":L2,
            "selected_L_vs_any_N_or_S_T2_TV_lower_bound":selected_L_outside_N_and_S(policy)
        })
    return {
        "status":"AUTHOR_SIDE_H4_LAW_INVARIANT_NOT_INDEPENDENT_P0",
        "model":"Experiment003-A2-v0.2.2 J/H4",
        "families":"R not subset N/S by temporal instability; N/S/L not subset R by T1 ceiling; L not subset N/S by learning nonstationarity; R not subset L under A by fine reversal",
        "frozen_family_classes":["M","N","C","S","L","R"],
        "universal_R_t1_zero_fine_cooperation_ceiling":C0,
        "policies":policies,
        "R_vs_L_A_only":selected_R_outside_L_under_A(),
        "unknown":["R not subset L under A0","N not subset S: the proposed T1 Hankel argument is INVALID for actor-iid H4","full all-15 inclusion map","statistical H4 coverage"],
        "N_three_type_T1_moment_audit":independent_actor_moment_audit(),
    }


def stationary_full_two_period_mass(policy, family):
    """Numerically integrate iid H4, T1 J law, t2 choice broker, fine0 event.

    Target N/S/M mechanisms have fixed actor offsets and an actor-level
    matching-appraisal nuisance that may correlate with action preference.
    """
    atoms={
        "M":[(.4,-.9,0.),(.6,.4,0.)],
        "N":[(.25,-.8,-1.1),(.75,.7,1.5)],
        "S":[(.4,-.6,1.),(.6,.4,-1.)],
    }[family]
    beta,eps,rho,delta,omega=1.7,.08,2.3,.8,.7
    fine_states=((0,0),) if policy=="A0" else tuple(itertools.product((0,1),repeat=2))
    regimes=(("random",1.),) if policy=="A0" else (("random",.5),("choice",.5))
    def offset(a):
        return 0 if family=="M" else (a[2] if family=="N" else delta*a[2])
    def chance(f,o):
        return eps/2+(1-eps)*sig(beta*(f-1+o))
    def matching_broker(actor_types,prior_actions):
        weights=[]
        for pairs in PAIRS:
            w=0.
            for i,j in pairs:
                ci=1/3+actor_types[i][1]+(omega*actor_types[i][2] if family=="S" else 0)
                cj=1/3+actor_types[j][1]+(omega*actor_types[j][2] if family=="S" else 0)
                w+=min(sig(ci*(2*prior_actions[j]-1)),sig(cj*(2*prior_actions[i]-1)))
            weights.append(math.exp(rho*w))
        z=sum(weights)
        return [v/z for v in weights]
    first=second=mass_check=0.
    for types in itertools.product(range(len(atoms)),repeat=4):
        actor_types=[atoms[j] for j in types]
        latent_mass=math.prod(a[0] for a in actor_types)
        chance0=chance(0,offset(actor_types[0]))
        for mu in range(3):
            for fines in fine_states:
                probabilities=[chance(fine_amount(mu,fines,i),offset(actor_types[i])) for i in range(4)]
                for actions in itertools.product((0,1),repeat=4):
                    mass=latent_mass*p_tuple(probabilities,actions)/(3*len(fine_states))
                    mass_check+=mass
                    if fines[0]==0 and actions[0]==1:
                        first+=mass
                    for z,z_mass in regimes:
                        broker=[1/3]*3 if z=="random" else matching_broker(actor_types,actions)
                        for mu2,p_mu in enumerate(broker):
                            # New fine draw after matching. Actor0's current
                            # zero-fine cooperation uses fixed type, not mu2.
                            second+=mass*z_mass*p_mu*(.5 if policy=="A" else 1.)*chance0
    assert abs(mass_check-1)<1e-11
    return first,second

class ObservableTests(unittest.TestCase):
    def test_complete_t1_law_enumeration_and_analytic_close(self):
        for policy in ("A","A0"):
            for fam,analytic in (("R",analytic_R_rates),("L",analytic_L_rates)):
                with self.subTest(policy=policy,family=fam):
                    enumerated=t1_enumerated(policy,fam)
                    derived=analytic(policy)
                    for a,b in zip(enumerated,derived):
                        self.assertAlmostEqual(a,b,places=13)
    def test_N_S_L_noninclusion_in_R_at_t1(self):
        for policy in ("A","A0"):
            lower=selected_N_outside_R(policy)
            self.assertGreater(lower,0)
            self.assertAlmostEqual(lower,{"A":.0674744624706623,
                                          "A0":.1349489249413246}[policy],places=13)
    def test_R_noninclusion_in_N_S_after_one_update(self):
        for policy in ("A","A0"):
            lower=selected_R_outside_N_and_S(policy)
            self.assertGreater(lower,0)
            self.assertAlmostEqual(lower,{"A":.007035078411326806,
                                          "A0":.028140313645307224}[policy],places=13)
    def test_L_noninclusion_in_N_S_after_learning(self):
        for policy in ("A","A0"):
            lower=selected_L_outside_N_and_S(policy)
            self.assertGreater(lower,0)
            self.assertAlmostEqual(lower,{"A":.015140221223373579,
                                          "A0":.01366867648617881}[policy],places=13)
    def test_R_current_fine_reversal_refutes_any_L_under_A(self):
        w=selected_R_outside_L_under_A()
        self.assertGreater(w["R_t2_zero_fine"],w["R_t2_half_fine"])
        self.assertAlmostEqual(w["TV_lower_bound_vs_any_L_H4"],
                               4.9310612526720534e-5,places=16)
    def test_three_family_two_period_H4_stationarity_by_full_law(self):
        for policy in ("A","A0"):
            for family in ("M","N","S"):
                with self.subTest(policy=policy,family=family):
                    first,second=stationary_full_two_period_mass(policy,family)
                    self.assertGreater(first,0)
                    self.assertAlmostEqual(first,second,places=12)

    def test_reviewer_Hankel_proposal_invalid_under_actor_iid_H4(self):
        v=independent_actor_moment_audit()
        self.assertGreater(v["latent_hankel_determinant"],5e-5)
        self.assertAlmostEqual(v["observable_hankel_determinant"],0,places=14)
        pbar=v["observable_first_k_cooperators_no_fines"][1]
        self.assertAlmostEqual(v["observable_first_k_cooperators_no_fines"][2],pbar*pbar,places=14)
        # With A0 at T1, neutral broker and no fines, this specific N
        # three-type marginal law equals a lawful one-atom S(delta=0)
        # with a different allowed beta, for EVERY 4-action vector.
        beta_s=v["A0_T1_exact_S_alias_beta"]
        for actions in itertools.product((0,1),repeat=4):
            self.assertAlmostEqual(p_tuple([pbar]*4,actions),
                                   p_tuple([sig(-beta_s)]*4,actions),places=14)

    def test_dual_contrasts_as_public_disjoint_event_masses(self):
        w=selected_R_outside_L_under_A()
        m=w["P_first_public_H"]
        e0=m*.5*w["R_t2_zero_fine"]
        ehalf=m*.5*w["R_t2_half_fine"]
        self.assertAlmostEqual(e0-ehalf,w["signed_fine_reversal_contrast"],places=15)
        self.assertAlmostEqual((e0-ehalf)/2,w["TV_lower_bound_vs_any_L_H4"],places=15)
        for policy in ("A","A0"):
            r1,r2=t1_enumerated(policy,"R")
            l1,l2=t1_enumerated(policy,"L")
            self.assertAlmostEqual(time_event_weight(policy)*(r1-r2)/2,
                                   selected_R_outside_N_and_S(policy),places=14)
            self.assertAlmostEqual(time_event_weight(policy)*(l2-l1)/2,
                                   selected_L_outside_N_and_S(policy),places=14)

if __name__=="__main__":
    import sys
    if "--json" in sys.argv:
        print(json.dumps(evaluate(),indent=2,sort_keys=True))
    else:
        unittest.main(verbosity=2)
