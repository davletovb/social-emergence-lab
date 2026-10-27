//! Experiment 003: random-regime observational invariant / H4 C-not-L witness.
//!
//! Certificate logic is an analytic law argument documented alongside source.
//! This binary numerically checks the claim on a selected C truth and a
//! heterogeneous correlated two-atom L mixture, without simulating or fitting.
//! Does NOT establish L-not-C, or global C/L disjointness, nor statistical power.
use hiyal::core::{AgentId, World};
use serde_json::{Value, json};
use social_emergence_lab::experiment003 as e;

const E_ACTIONS: [u8;4] = [1,0,1,0];
const TOL: f64 = 1e-12;
fn sig(x:f64)->f64 { 1.0/(1.0+(-x).exp()) }
fn policy_z(p:e::Policy,z:e::Regime)->f64 {
    match (p,z) {
        (e::Policy::A,_)=>0.5,
        (e::Policy::A0,e::Regime::Random)=>1.0,
        _=>0.0
    }
}
fn fine_possibilities(p:e::Policy)->Vec<([u8;2],f64)> {
    match p {
        e::Policy::A=>vec![([0,0],0.25),([1,0],0.25),([0,1],0.25),([1,1],0.25)],
        e::Policy::A0=>vec![([0,0],1.0)]
    }
}
fn globals()->e::Globals{
    e::Globals{beta:1.0,epsilon:0.0,rho:1.0,delta:0.0,omega:0.0}
}
fn c_truth(p:e::Policy)->e::Design {
    let mut d=e::pilot_design(e::Family::C,p);
    d.globals=globals();
    d.atoms=vec![e::Atom{weight:1.0,tuple:e::Tuple::C{eta:0.0,kappa:1.0}}];
    d.validate().unwrap();
    d
}
fn l_mixture(p:e::Policy)->e::Design {
    let mut d=e::pilot_design(e::Family::L,p);
    d.globals=e::Globals{beta:1.7,epsilon:0.08,rho:2.3,delta:0.0,omega:0.0};
    d.atoms=vec![
        e::Atom{weight:0.4,tuple:e::Tuple::L{
            eta:0.1,kappa:1.75,alpha:0.8,zeta:0.75}},
        e::Atom{weight:0.6,tuple:e::Tuple::L{
            eta:-0.2,kappa:-1.25,alpha:0.3,zeta:0.1}},
    ];
    d.validate().unwrap();
    d
}
// Selected heterogeneous two-atom *conformance* examples for all five
// partner-independent families; the universal result is analytic, not tested
// by the choice of these two atoms.
fn other_mixture(fam:e::Family,p:e::Policy)->e::Design {
    if fam==e::Family::L{return l_mixture(p);}
    assert!(fam!=e::Family::C);
    let mut d=e::pilot_design(fam,p);
    d.globals=e::Globals{beta:1.7,epsilon:0.08,rho:2.3,delta:0.7,omega:0.4};
    d.atoms=vec![
        e::Atom{weight:0.4,tuple:match fam {
            e::Family::M=>e::Tuple::M{eta:0.1},
            e::Family::N=>e::Tuple::N{eta:0.1,nu:0.7},
            e::Family::S=>e::Tuple::S{eta:0.1,tau:1},
            e::Family::R=>e::Tuple::R{eta:0.1,alpha:0.8,lambda:0.5},
            _=>unreachable!(),
        }},
        e::Atom{weight:0.6,tuple:match fam {
            e::Family::M=>e::Tuple::M{eta:-0.2},
            e::Family::N=>e::Tuple::N{eta:-0.2,nu:-0.5},
            e::Family::S=>e::Tuple::S{eta:-0.2,tau:-1},
            e::Family::R=>e::Tuple::R{eta:-0.2,alpha:0.3,lambda:0.2},
            _=>unreachable!(),
        }},
    ];
    d.validate().unwrap();
    d
}
#[derive(Clone,Copy,Debug)]
struct EventResult{
    first_actions_probability:f64,
    matching0_joint_probability:f64,
    matching1_joint_probability:f64,
}
impl EventResult{
    fn contrast(&self)->f64 {self.matching0_joint_probability-self.matching1_joint_probability}
    fn tv_lower_bound(&self)->f64 {self.contrast().abs()/2.0}
}
// B_m = {first actions equal E, second regime random, second matching mu=m,
//        actor0's t2 dyad fine = 0, actor0 cooperates at t2}.
// All other first- and second-period observations are marginalized.
// The 1/2 event indicator contrast is a valid full-public-history TV bound.
fn observable_event(d:&e::Design)->EventResult {
    observable_event_for(d,E_ACTIONS)
}
fn observable_event_for(d:&e::Design,actions:[u8;4])->EventResult {
    d.validate().unwrap();
    assert!(d.atoms.len()<=2,"bounded numerical diagnostic only");
    let mut first=0.0;
    let mut b=[0.0;2];
    let count=d.atoms.len().pow(4);
    let k=policy_z(d.policy,e::Regime::Random)/3.0*
        (if d.policy==e::Policy::A {0.5}else{1.0});
    for label in 0..count {
        let mut indices=label;
        let mut weight=1.0;
        let mut actors=[d.atoms[0].tuple;4];
        for t in &mut actors {
            let idx=indices%d.atoms.len();
            indices/=d.atoms.len();
            *t=d.atoms[idx].tuple;
            weight*=d.atoms[idx].weight;
        }
        if weight==0.0 {continue;}
        let pristine=e::Society::new_fixed(d.clone(),actors).unwrap();
        for z in [e::Regime::Random,e::Regime::Choice]{
            let regime_mass=policy_z(d.policy,z);
            if regime_mass==0.0 {continue;}
            for mu in 0..3{
                let match_prob=pristine.broker(z)[mu];
                for (bits,fine_mass) in fine_possibilities(d.policy){
                    let prior=regime_mass*match_prob*fine_mass
                        *pristine.joint_action_probability(mu,bits,actions);
                    let mass=weight*prior;
                    first+=mass;
                    let pr=e::PublicRecord{
                        period:1,regime:z,matching:mu,fine_bits:bits,actions,
                        payoffs:e::Society::payoff_for(mu,bits,actions)
                    };
                    let mut after=pristine.clone();
                    after.apply(AgentId(0),pr).unwrap();
                    for (out,mu2) in b.iter_mut().zip([0,1]){
                        // Both matchings 0 and 1 put actor0 in dyad 0, with
                        // fine bit 0. The other dyad's fine is marginalized.
                        *out+=mass*k*after.cooperation(0,mu2,0);
                    }
                }
            }
        }
    }
    EventResult{
        first_actions_probability:first,
        matching0_joint_probability:b[0],
        matching1_joint_probability:b[1],
    }
}
fn analytic_c(p:e::Policy)->EventResult {
    let q=sig(-0.5);
    // For A: under mu0/mu2, E is two mixed-action dyads.
    // Under mu1, E is a (1,1) dyad and a (0,0) dyad.
    // At t1, choice matching is uniform and regimes sum to 1.
    let prob_e=if p==e::Policy::A{
        let r=0.5;
        let mixed=(q*(1.0-q)+r*(1.0-r))/2.0;
        let both_one=(q*q+r*r)/2.0;
        let both_zero=((1.0-q).powi(2)+(1.0-r).powi(2))/2.0;
        (2.0*mixed.powi(2)+both_one*both_zero)/3.0
    }else{q.powi(2)*(1.0-q).powi(2)};
    // H1 fixes actions [1,0,1,0]. Current t2 mu0 partner of actor0
    // is actor1 (previously defected), but mu1 partner is actor2
    // (previously cooperated). Current fine is zero in both cells.
    let coop0=sig(-1.0);
    let coop1=sig(0.0);
    let factor=policy_z(p,e::Regime::Random)/3.0*
        (if p==e::Policy::A {0.5}else{1.0});
    EventResult{
        first_actions_probability:prob_e,
        matching0_joint_probability:prob_e*factor*coop0,
        matching1_joint_probability:prob_e*factor*coop1,
    }
}
// Combine ALL 16 disjoint first-period action patterns for actor0, instead
// of fixing only E=[1,0,1,0]. Let h=sum_{a: a1!=a2}
// (a1-a2) [1_{B0,a}-1_{B1,a}]. Since the first-period action vectors
// partition the sample and mu0/mu1 are disjoint, |h|<=1. All M,N,S,L,R
// still satisfy E[h]=0; the chosen C has a positive signed contrast.
fn pooled_c(p:e::Policy)->(f64,f64,f64){
    let mut computed=0.0;
    let mut different=0.0;
    let source=c_truth(p);
    for mask in 0..16 {
        let action=std::array::from_fn(|i|((mask>>i)&1) as u8);
        if action[1]==action[2] {continue;}
        let v=observable_event_for(&source,action);
        different+=v.first_actions_probability;
        computed+=(action[1] as f64-action[2] as f64)*v.contrast();
    }
    let q=sig(-0.5);
    let r=0.5;
    // At t1, C acts with q at zero fine and r at half fine.
    // Matchings 0 and 1 place actors 1/2 in separate dyads, so their
    // averaged fines give independent Bernoulli with pbar=(q+r)/2.
    // Matching 2 pairs actors 1/2 in the SAME dyad, with common fine.
    let exact_different=if p==e::Policy::A{
        let pbar=(q+r)/2.0;
        (2.0/3.0)*(2.0*pbar*(1.0-pbar))
            +(1.0/3.0)*(q*(1.0-q)+r*(1.0-r))
    }else{2.0*q*(1.0-q)};
    let k=policy_z(p,e::Regime::Random)/3.0*
        (if p==e::Policy::A {0.5}else{1.0});
    let exact_signed=k*(sig(0.0)-sig(-1.0))*exact_different;
    assert!((different-exact_different).abs()<TOL);
    assert!((computed-exact_signed).abs()<TOL);
    (different,computed,computed.abs()/2.0)
}
fn evaluate()->Value {
    let mut policies=Vec::new();
    for p in e::Policy::ALL{
        let analytic=analytic_c(p);
        let enumerated=observable_event(&c_truth(p));
        let lm=observable_event(&l_mixture(p));
        let mut other_contrasts=serde_json::Map::new();
        for fam in [e::Family::M,e::Family::N,e::Family::S,e::Family::L,e::Family::R]{
            let sample=observable_event(&other_mixture(fam,p));
            assert!(sample.contrast().abs()<TOL,"family {} violated invariant",fam.name());
            other_contrasts.insert(fam.name().into(),json!(sample.contrast()));
        }
        let (p_different,pooled_signed,pooled_bound)=pooled_c(p);
        assert!((analytic.first_actions_probability-enumerated.first_actions_probability).abs()<TOL);
        assert!((analytic.matching0_joint_probability-enumerated.matching0_joint_probability).abs()<TOL);
        assert!((analytic.matching1_joint_probability-enumerated.matching1_joint_probability).abs()<TOL);
        assert!(analytic.tv_lower_bound()>0.0);
        assert!(lm.contrast().abs()<TOL);
        policies.push(json!({
            "policy":p.name(),
            "public_first_action_event":"a1=[1,0,1,0], marginalizing all other period-1 variables",
            "t2_conditioning":"z2=random, actor0 fine=0, a0_2=1, comparing mu2=0 (partner1) vs mu2=1 (partner2)",
            "selected_C_truth":{"beta":1.0,"epsilon":0.0,"kappa":1.0,"G":"one atom, identical eta=0 for four actors"},
            "C_first_action_event_probability":analytic.first_actions_probability,
            "C_joint_B0_probability":analytic.matching0_joint_probability,
            "C_joint_B1_probability":analytic.matching1_joint_probability,
            "C_signed_event_contrast":analytic.contrast(),
            "all_H4_L_exact_event_contrast":"0 by random-broker conditional-independence invariant, not by numerical optimization",
            "TV_lower_bound_C_truth_vs_any_L_H4":analytic.tv_lower_bound(),
            "numerical_C_event_contrast":enumerated.contrast(),
            "sample_correlated_two_atom_L_contrast":lm.contrast(),
            "sample_correlated_two_atom_L_first_action_event_probability":lm.first_actions_probability,
            "C_probability_first_period_actor1_actor2_differ":p_different,
            "C_aggregated_first_action_signed_contrast":pooled_signed,
            "TV_lower_bound_C_truth_vs_any_M_N_S_L_R_H4_aggregated":pooled_bound,
            "example_other_family_two_atom_contrasts":other_contrasts,
            "scope":"candidate one-truth nonmembership C outside each of M,N,S,L,R; reverse inclusion, overall pair class relation and statistical inference remain open"
        }));
    }
    json!({
        "model":"Experiment003-A2-v0.2.2",
        "version":"cl-random-rematching-invariant-certificate-v0.2-review-extension",
        "status":"ANALYTIC_MODEL_LAW_WITNESS_NUMERICALLY_CHECKED_NOT_INDEPENDENT_P0",
        "analytic_identity":"Under z2=random and equal actor0 fine, mu2 is independent of all latent parameters/history. Every M,N,S,L,R current action offset is independent of the newly assigned partner, so these families all satisfy P(B0)=P(B1) for arbitrary H4 mixtures.",
        "analytic_exclusion":"The one-atom C truth with kappa=beta=1,epsilon=0 responds differently to partners with previous actions 0 and 1. For E=[1,0,1,0], P_C(B0)-P_C(B1)=P_C(E)*k*[sigmoid(-1)-sigmoid(0)] != 0. Therefore C truth is not representable by ANY L H4 law under A or A0 for T>=2.",
        "claim_limits":"Does not establish reverse direction L not subset C, or any other reverse inclusion, family disjointness, complete 15-pair classification, finite-sample calibration or human validity.",
        "policies":policies
    })
}
fn main(){println!("{}",serde_json::to_string_pretty(&evaluate()).unwrap());}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn c_truth_analytic_vs_complete_t1_marginalization(){
        for p in e::Policy::ALL {
            let a=analytic_c(p);
            let b=observable_event(&c_truth(p));
            assert!((a.first_actions_probability-b.first_actions_probability).abs()<TOL);
            assert!((a.matching0_joint_probability-b.matching0_joint_probability).abs()<TOL);
            assert!((a.matching1_joint_probability-b.matching1_joint_probability).abs()<TOL);
            assert!(a.tv_lower_bound()>0.0);
        }
    }
    #[test]
    fn heterogeneous_correlated_l_mixture_respects_invariant(){
        for p in e::Policy::ALL {
            let d=l_mixture(p);
            assert_eq!(d.atoms.len(),2);
            let r=observable_event(&d);
            assert!(r.first_actions_probability>0.);
            assert!(r.contrast().abs()<TOL);
        }
    }
    #[test]
    fn all_other_partner_independent_families_obey_random_regime_invariant(){
        for p in e::Policy::ALL{
            for fam in [e::Family::M,e::Family::N,e::Family::S,e::Family::L,e::Family::R]{
                let d=other_mixture(fam,p);
                assert_eq!(d.atoms.len(),2);
                let event=observable_event(&d);
                assert!(event.first_actions_probability>0.0);
                assert!(event.contrast().abs()<TOL,"{}",fam.name());
            }
        }
    }
    #[test]
    fn pooled_all_first_action_patterns_improve_bound(){
        for (p,expected) in [
            (e::Policy::A,0.004717469806363266),
            (e::Policy::A0,0.018099874571358386),
        ]{
            let (p_different,signed,bound)=pooled_c(p);
            assert!(p_different>0.45 && p_different<0.51);
            assert!(signed>0.0);
            assert!((bound-expected).abs()<1e-13);
            assert!(bound>analytic_c(p).tv_lower_bound()*5.0);
        }
    }
    #[test]
    fn numeric_bound_regressions(){
        let a=analytic_c(e::Policy::A);
        let a0=analytic_c(e::Policy::A0);
        assert!((a.tv_lower_bound()-0.0005781936786095032).abs()<1e-13);
        assert!((a0.tv_lower_bound()-0.0021267688573262324).abs()<1e-13);
    }
}
