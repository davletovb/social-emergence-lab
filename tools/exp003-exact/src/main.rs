//! Deterministic finite-support enumeration for Experiment 003 A2 v0.2.2.
//! Not Monte Carlo, not symbolic exact arithmetic, not H4 rival optimization.
//! Enumerates ALL 384 physical public records per period, with 0 mass for A0
//! forbidden allocations. Conditional action vectors use the primary J product.
use hiyal::core::{AgentId, World};
use serde_json::{Value, json};
use social_emergence_lab::experiment003 as e;

const N_RECORDS: usize = 384;
const TOL: f64 = 1e-12;

fn record(n: usize, period: usize) -> e::PublicRecord {
    assert!(n < N_RECORDS);
    let mask = n % 16;
    let fidx = (n / 16) % 4;
    let mu = (n / 64) % 3;
    let regime = if n / 192 == 0 { e::Regime::Random } else { e::Regime::Choice };
    let fines = [(fidx & 1) as u8, ((fidx >> 1) & 1) as u8];
    let actions = std::array::from_fn(|i| ((mask >> i) & 1) as u8);
    e::PublicRecord {
        period,
        regime,
        matching: mu,
        fine_bits: fines,
        actions,
        payoffs: e::Society::payoff_for(mu, fines, actions),
    }
}
fn regime_mass(p: e::Policy, z: e::Regime) -> f64 {
    match (p, z) {
        (e::Policy::A, _) => 0.5,
        (e::Policy::A0, e::Regime::Random) => 1.0,
        (e::Policy::A0, e::Regime::Choice) => 0.0,
    }
}
fn fine_mass(p: e::Policy, bits: [u8; 2]) -> f64 {
    match p {
        e::Policy::A => 0.25,
        e::Policy::A0 => if bits == [0, 0] { 1.0 } else { 0.0 },
    }
}

/// Full joint public record mass, including regime and dyad-fine probabilities.
/// The four current actor actions are correlated marginally under mixtures;
/// this method is applied per fixed latent configuration, before H4 integration.
fn probability(s: &e::Society, rec: &e::PublicRecord) -> f64 {
    let z = regime_mass(s.design.policy, rec.regime);
    let f = fine_mass(s.design.policy, rec.fine_bits);
    if z == 0.0 || f == 0.0 { return 0.0; }
    z * s.broker(rec.regime)[rec.matching] * f
        * s.joint_action_probability(rec.matching, rec.fine_bits, rec.actions)
}
fn all_next(s: &e::Society, period: usize) -> Vec<f64> {
    (0..N_RECORDS).map(|r| probability(s, &record(r, period))).collect()
}
fn total(xs: &[f64]) -> f64 { xs.iter().sum() }
fn max_diff(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(x,y)| (x-y).abs()).fold(0.0, f64::max)
}
fn tv(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());
    0.5*a.iter().zip(b).map(|(x,y)| (x-y).abs()).sum::<f64>()
}
fn fixed(family:e::Family, policy:e::Policy, g:e::Globals, t:e::Tuple) -> e::Society {
    let mut d=e::pilot_design(family,policy);
    d.globals=g;
    d.atoms=vec![e::Atom{weight:1.0,tuple:t}];
    e::Society::new_fixed(d,[t;4]).expect("legal finite witness")
}

/// Correct H4 integration: sample the JOINT atom index ONCE per actor per
/// society. For two atoms, all 2^4 = 16 assignments are enumerated.
/// The same assignment is retained across t=1 and t=2.
fn integrate_h4(d: &e::Design, prefix: Option<e::PublicRecord>) -> (Vec<f64>, f64) {
    d.validate().expect("valid H4 specification");
    assert!(d.atoms.len() <= 2, "bounded two-atom benchmark, not all H4");
    let combos = d.atoms.len().pow(4);
    let mut mass = vec![0.0; N_RECORDS];
    let mut prefix_mass = 0.0;
    for mut combo in 0..combos {
        let mut actors = [d.atoms[0].tuple; 4];
        let mut weight = 1.0;
        for actor in &mut actors {
            let idx=combo % d.atoms.len();
            combo/=d.atoms.len();
            *actor = d.atoms[idx].tuple;
            weight*=d.atoms[idx].weight;
        }
        if weight == 0.0 {continue;}
        let mut s=e::Society::new_fixed(d.clone(),actors).expect("legal atom combination");
        let likelihood=if let Some(pr)=prefix {
            let mass=probability(&s,&pr);
            if mass != 0.0 {s.apply(AgentId(0),pr).expect("valid prescribed public path");}
            mass
        } else {1.0};
        prefix_mass+=weight*likelihood;
        if likelihood == 0.0 {continue;}
        let next_period=if prefix.is_some(){2}else{1};
        let dist=all_next(&s,next_period);
        for (acc,p) in mass.iter_mut().zip(dist) {*acc+=weight*likelihood*p;}
    }
    (mass, prefix_mass)
}
fn mapped_s_n_l(p: e::Policy) -> [e::Design; 3] {
    let source=e::pilot_design(e::Family::S,p);
    let mut n=e::pilot_design(e::Family::N,p);
    let mut l=e::pilot_design(e::Family::L,p);
    n.globals=source.globals;
    l.globals=source.globals;
    n.atoms=source.atoms.iter().map(|a|{
        let e::Tuple::S{eta,tau}=a.tuple else {panic!("source must be S")};
        e::Atom{weight:a.weight,tuple:e::Tuple::N{
            eta:eta+source.globals.omega*tau as f64,
            nu:source.globals.delta*tau as f64,
        }}
    }).collect();
    l.atoms=n.atoms.iter().map(|a|{
        let e::Tuple::N{eta,nu}=a.tuple else {panic!("source must map into N")};
        e::Atom{weight:a.weight,tuple:e::Tuple::L{
            eta,kappa:2.0*nu,alpha:0.0,zeta:0.0,
        }}
    }).collect();
    source.validate().unwrap();n.validate().unwrap();l.validate().unwrap();
    [source,n,l]
}
fn prefix_for(p:e::Policy) -> e::PublicRecord {
    let regime=if p==e::Policy::A { e::Regime::Choice } else { e::Regime::Random };
    let bits=if p==e::Policy::A {[0,1]}else{[0,0]};
    let actions=[1,0,1,1];
    e::PublicRecord{
        period:1,regime,matching:0,fine_bits:bits,actions,
        payoffs:e::Society::payoff_for(0,bits,actions)
    }
}
fn m_n_fine_witness(p:e::Policy)->Value {
    let base=e::Globals{beta:1.0,epsilon:0.0,rho:1.0,delta:0.0,omega:0.0};
    let m=fixed(e::Family::M,p,base,e::Tuple::M{eta:0.0});
    let mut ng=base;ng.beta=4.0;
    let n=fixed(e::Family::N,p,ng,e::Tuple::N{eta:0.0,nu:0.75});
    let dm=all_next(&m,1);
    let dn=all_next(&n,1);
    let p_m0=m.cooperation(0,0,0);
    let p_n0=n.cooperation(0,0,0);
    let p_mhalf=m.cooperation(0,0,1);
    let p_nhalf=n.cooperation(0,0,1);
    assert!((p_m0-p_n0).abs()<TOL);
    assert!((total(&dm)-1.0).abs()<TOL && (total(&dn)-1.0).abs()<TOL);
    if p==e::Policy::A0 {assert!(max_diff(&dm,&dn)<TOL);}else{assert!(tv(&dm,&dn)>0.01);}
    // Every allowed M has p(cooperate|f=1/2) < 1/2 because
    // beta>0 and epsilon<=1/2. N here has p>1/2.
    assert!(p_mhalf<0.5 && p_nhalf>0.5);
    json!({"policy":p.name(),"t1_joint_tv_for_selected_M_vs_N":tv(&dm,&dn),
        "zero_fine_M_cooperate":p_m0,"zero_fine_N_cooperate":p_n0,
        "half_fine_M_cooperate":p_mhalf,"half_fine_N_cooperate":p_nhalf,
        "logical_scope":"witness M is compatible with selected N at A0 T1; conditional half-fine refutes ALL M representations under A for this N truth; no finite-data power claim"})
}
fn c_l_rematching(p:e::Policy)->Value {
    let globals=e::Globals{beta:1.0,epsilon:0.0,rho:1.0,delta:0.0,omega:0.0};
    let mut c=fixed(e::Family::C,p,globals,e::Tuple::C{eta:0.0,kappa:1.0});
    let mut l=fixed(e::Family::L,p,globals,e::Tuple::L{eta:0.0,kappa:1.0,alpha:1.0,zeta:0.0});
    // Realized prior matching 01|23; actor 0 saw last partner actor 1 defect.
    // After rematching to 02|13, current partner actor 2 had cooperated.
    let acts=[1,0,1,0];
    let r=e::PublicRecord{period:1,regime:e::Regime::Random,
        matching:0,fine_bits:[0,0],actions:acts,
        payoffs:e::Society::payoff_for(0,[0,0],acts)};
    let pcmass=probability(&c,&r);let plmass=probability(&l,&r);
    assert!((pcmass-plmass).abs()<TOL && pcmass>0.0);
    c.apply(AgentId(0),r).unwrap();
    l.apply(AgentId(0),r).unwrap();
    let pc=c.cooperation(0,1,0);let pl=l.cooperation(0,1,0);
    assert!(pc>pl);
    let dc=all_next(&c,2);let dl=all_next(&l,2);
    assert!((total(&dc)-1.0).abs()<TOL&&(total(&dl)-1.0).abs()<TOL);
    assert!(tv(&dc,&dl)>0.0);
    json!({"policy":p.name(),"first_record_public_probability":pcmass,
       "t2_actor0_cooperation_given_rematch_C":pc,
       "t2_actor0_cooperation_given_rematch_L":pl,
       "t2_conditional_complete_record_tv_under_fixed_mapping":tv(&dc,&dl),
       "t2_conditional_max_record_difference":max_diff(&dc,&dl),
       "scope":"counterexample to alphaL=1, zeta=0, kappaL=kappaC parameter mapping, NOT a free-rival H4 family noninclusion proof"})
}
fn initial_collapses(p:e::Policy)->Value {
    let g=e::Globals{beta:1.7,epsilon:0.08,rho:2.3,delta:0.0,omega:0.0};
    let c=fixed(e::Family::C,p,g,e::Tuple::C{eta:0.15,kappa:1.2});
    let n=fixed(e::Family::N,p,g,e::Tuple::N{eta:0.15,nu:0.6});
    let l=fixed(e::Family::L,p,g,e::Tuple::L{eta:0.15,kappa:1.2,alpha:0.7,zeta:0.8});
    let m=fixed(e::Family::M,p,g,e::Tuple::M{eta:0.15});
    let r=fixed(e::Family::R,p,g,e::Tuple::R{eta:0.15,alpha:0.7,lambda:0.5});
    let dc=all_next(&c,1);
    let dn=all_next(&n,1);
    let dl=all_next(&l,1);
    let dm=all_next(&m,1);
    let dr=all_next(&r,1);
    let d=max_diff(&dc,&dn).max(max_diff(&dn,&dl));
    let mr=max_diff(&dm,&dr);
    assert!(d<TOL && mr<TOL);
    json!({"policy":p.name(),"t1_C_N_L_max_absolute_difference":d,
        "t1_M_R_max_absolute_difference":mr,
        "scope":"chosen legal homogeneous witnesses; full H4 T1 class equalities follow separately from legal parameter maps"})
}
fn nesting(p:e::Policy)->Value {
    let [s,n,l]=mapped_s_n_l(p);
    let (one_s,ms)=integrate_h4(&s,None);
    let (one_n,mn)=integrate_h4(&n,None);
    let (one_l,ml)=integrate_h4(&l,None);
    assert!((ms-1.0).abs()<TOL && (mn-1.0).abs()<TOL && (ml-1.0).abs()<TOL);
    assert!((total(&one_s)-1.0).abs()<TOL);
    let t1_diff=max_diff(&one_s,&one_n).max(max_diff(&one_n,&one_l));
    assert!(t1_diff<TOL);
    let prefix=prefix_for(p);
    let (two_s,ps)=integrate_h4(&s,Some(prefix));
    let (two_n,pn)=integrate_h4(&n,Some(prefix));
    let (two_l,pl)=integrate_h4(&l,Some(prefix));
    let diff=max_diff(&two_s,&two_n).max(max_diff(&two_n,&two_l));
    assert!(ps>0.0 && (ps-pn).abs()<TOL && (pn-pl).abs()<TOL);
    assert!((total(&two_s)-ps).abs()<TOL);
    assert!(diff<TOL);
    json!({"policy":p.name(),"H4_joint_atoms":s.atoms.len(),
        "actor_assignments_enumerated":s.atoms.len().pow(4),
        "t1_public_events":N_RECORDS,
        "t1_max_absolute_difference_S_N_L":t1_diff,
        "selected_t1_prefix_probability":ps,
        "t2_public_suffixes_enumerated":N_RECORDS,
        "t1_t2_joint_prefix_suffix_max_absolute_difference_S_N_L":diff,
        "scope":"complete finite-support numerical integration of one declared two-atom H4 example and all 384 possible next records; not the full unrestricted H4 law class"})
}
fn evaluate()->Value {
    let mut results=Vec::new();
    for p in e::Policy::ALL {
        results.push(json!({"policy":p.name(),
            "t1_class_collapse":initial_collapses(p),
            "nested_S_N_L_two_atom_H4":nesting(p),
            "C_L_rematching":c_l_rematching(p),
            "M_N_fine":m_n_fine_witness(p)}));
    }
    json!({"model":"Experiment003-A2-v0.2.2",
        "status":"DETERMINISTIC_FINITE_SUPPORT_NUMERICAL_BENCHMARK_NOT_CONFIRMATORY",
        "arithmetic":"exhaustive 384-category event enumeration with IEEE-754 f64 sigmoid/softmax; not exact symbolic/rational arithmetic",
        "no_stochastic_draws":true,
        "tested_policies":results,
        "inference_limits":"No fitting of unconstrained competing H4 nuisances, no statistical calibration, no global pair lattice or achieved coverage claims"})
}
fn main(){println!("{}",serde_json::to_string_pretty(&evaluate()).unwrap());}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn supports_are_complete_and_normalize(){
        for policy in e::Policy::ALL{
            for family in e::Family::ALL {
                let d=e::pilot_design(family,policy);
                let s=e::Society::new_fixed(d.clone(),[d.atoms[0].tuple;4]).unwrap();
                let masses=all_next(&s,1);
                let nonzero=masses.iter().filter(|&&x|x>0.0).count();
                assert_eq!(nonzero,if policy==e::Policy::A {384}else{48});
                assert!((total(&masses)-1.0).abs()<TOL);
            }
        }
    }
    #[test]
    fn complete_bounded_comparisons(){
        let v=evaluate();
        let policies=v["tested_policies"].as_array().unwrap();
        assert_eq!(policies.len(),2);
        // Pin the exact reported witness values, not just their signs or that
        // both policies are present. Changes to the underlying model must be
        // reviewed alongside changes to the archived result report.
        for (index,expected_cl,expected_mn,expected_prefix) in [
            (0usize, 0.13021565405157567, 0.33386582682695076, 0.0006941543970427322),
            (1usize, 0.11552928931500243, 0.0, 0.0023487305203184652),
        ] {
            let p=&policies[index];
            let name=if index==0 {"A"}else{"A0"};
            assert_eq!(p["policy"].as_str(),Some(name));
            for (field,expected) in [
                ("C_L_rematching",expected_cl),
                ("M_N_fine",expected_mn),
            ] {
                let value_key=if field=="C_L_rematching"{
                    "t2_conditional_complete_record_tv_under_fixed_mapping"
                }else{"t1_joint_tv_for_selected_M_vs_N"};
                let actual=p[field][value_key].as_f64().unwrap();
                assert!((actual-expected).abs()<TOL,
                    "{name} {field}/{value_key}: actual {actual}, expected {expected}");
            }
            let h4=&p["nested_S_N_L_two_atom_H4"];
            let actual_prefix=h4["selected_t1_prefix_probability"].as_f64().unwrap();
            assert!((actual_prefix-expected_prefix).abs()<TOL);
            assert!(h4["t1_max_absolute_difference_S_N_L"].as_f64().unwrap()<TOL);
            assert!(h4["t1_t2_joint_prefix_suffix_max_absolute_difference_S_N_L"].as_f64().unwrap()<TOL);
            let alias=&p["t1_class_collapse"];
            assert!(alias["t1_C_N_L_max_absolute_difference"].as_f64().unwrap()<TOL);
            assert!(alias["t1_M_R_max_absolute_difference"].as_f64().unwrap()<TOL);
        }
    }
    #[test]
    fn correct_public_dyad_fine_indexing(){
        let r=record(1+16*(2+4*(2+3*0)),1);
        assert_eq!(r.matching,2);
        assert_eq!(r.fine_bits,[0,1]);
        assert_eq!(r.actions,[1,0,0,0]);
        assert_eq!(r.payoffs,e::Society::payoff_for(2,[0,1],[1,0,0,0]));
    }
}
