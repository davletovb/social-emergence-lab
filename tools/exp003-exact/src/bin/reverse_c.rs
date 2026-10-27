//! Model-implementation conformance check for author-derived R-not-C T2 witness.
//! This is NOT a P0 independent mathematical verification or an all-H4 fit.
use hiyal::core::{AgentId, World};
use serde_json::json;
use social_emergence_lab::experiment003 as e;

fn sigma(x: f64) -> f64 { 1.0/(1.0+(-x).exp()) }
fn c_ceiling() -> f64 { 0.25+0.5*sigma(-0.1) }

fn selected_r(policy: e::Policy) -> e::Society {
    let mut d=e::pilot_design(e::Family::R,policy);
    d.globals=e::Globals{beta:1.0,epsilon:0.0,rho:1.0,delta:0.0,omega:0.0};
    let tuple=e::Tuple::R{eta:0.0,alpha:1.0,lambda:0.0};
    d.atoms=vec![e::Atom{weight:1.0,tuple}];
    d.periods=3;
    d.validate().expect("legal H4 homogeneous R");
    e::Society::new_fixed(d,[tuple;4]).expect("four identical R actors")
}

fn record(period:usize,mu:usize,a:[u8;4])->e::PublicRecord {
    e::PublicRecord {
        period,matching:mu,regime:e::Regime::Random,
        fine_bits:[0,0],actions:a,
        payoffs:e::Society::payoff_for(mu,[0,0],a),
    }
}
fn verify(policy:e::Policy)->serde_json::Value {
    let mut s=selected_r(policy);
    let q=sigma(-1.0);
    let a1=record(1,0,[1,1,0,0]);
    assert_eq!(a1.payoffs,[1.0,1.0,0.0,0.0]);
    let mass1=s.joint_action_probability(0,[0,0],a1.actions);
    assert!((mass1-q*q*(1.0-q)*(1.0-q)).abs()<1e-14);
    for i in 0..4 {assert!((s.cooperation(i,0,0)-q).abs()<1e-14);}
    s.apply(AgentId(0),a1).expect("public t1 record");
    // t2 mu1 pairs actor0 with actor2, a2,1=0. Thus every legal C
    // rival's t2 p must be <= c_ceiling(), but selected R is 1/2.
    let p0=s.cooperation(0,1,0);
    assert!((p0-0.5).abs()<1e-14);
    assert!(p0>c_ceiling());
    for (i,want) in [(1,0.5),(2,q),(3,q)]{
        assert!((s.cooperation(i,1,0)-want).abs()<1e-14);
    }
    let first_alloc=if policy==e::Policy::A {1.0/24.0}else{1.0/3.0};
    let t2_alloc=if policy==e::Policy::A {1.0/12.0}else{1.0/3.0};
    let public_d_mass=first_alloc*mass1*t2_alloc;
    let lower=public_d_mass*(p0-c_ceiling());
    assert!(lower>0.0);
    let (expected_mass,expected_bound)=if policy==e::Policy::A {
        (0.00013422309823941982,1.6763919674633255e-6)
    }else{
        (0.004295139143661434,5.3644542958826415e-5)
    };
    assert!((public_d_mass-expected_mass).abs()<1e-15);
    assert!((lower-expected_bound).abs()<1e-15);

    // Also cross-check old, weaker T3 witness *separately* against the
    // implementation's transition ordering. Do not call T3 minimal.
    let a2=record(2,1,[1,0,1,0]);
    let mass2=s.joint_action_probability(1,[0,0],a2.actions);
    assert!((mass2-0.25*q*(1.0-q)).abs()<1e-14);
    assert_eq!(a2.payoffs,[1.0,0.0,1.0,0.0]);
    s.apply(AgentId(0),a2).expect("public t2 record");
    assert!((s.cooperation(0,0,0)-sigma(1.0)).abs()<1e-14);
    let historical_t3_bound=first_alloc.powi(2)*mass1*mass2*t2_alloc
        *(sigma(1.0)-c_ceiling());

    json!({"policy":policy.name(),
        "homogeneous_one_atom_R":true,
        "t1_action_mass":mass1,
        "t2_actor0_p":p0,
        "C_H4_universal_p_ceiling":c_ceiling(),
        "t2_public_condition_mass":public_d_mass,
        "T2_TV_lower_bound_vs_all_C_H4":lower,
        "legacy_T3_lower_bound":historical_t3_bound,
        "limits":"selected R witness, all-C H4 bound comes from separate analytic proof, not these tests"})
}
fn main(){
    let results:Vec<_>=e::Policy::ALL.into_iter().map(verify).collect();
    println!("{}",serde_json::to_string_pretty(&json!({
        "status":"AUTHOR_SIDE_RUST_CONFORMANCE_NOT_INDEPENDENT_P0",
        "results":results
    })).unwrap());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn A_positive_t2_witness_and_update_order(){verify(e::Policy::A);}
    #[test]
    fn A0_positive_t2_witness_and_update_order(){verify(e::Policy::A0);}
    #[test]
    fn C_ceiling_analytic_parameter_extremum(){
        assert!((c_ceiling()-0.48751040626053).abs()<1e-14);
        for beta in [0.1,0.2,1.0,5.0,10.0]{
            for eps in [0.0,0.25,0.5]{
                let p=eps/2.0+(1.0-eps)*sigma(-beta);
                assert!(p<=c_ceiling()+1e-14);
            }
        }
    }
}
