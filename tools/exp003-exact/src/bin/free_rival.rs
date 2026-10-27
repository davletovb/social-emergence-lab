//! Experiment 003 A2 v0.2.2: intentionally RESTRICTED H4 rival feasibility.
//!
//! This is a deterministic engineering/numerical search, NOT a global H4
//! optimizer, certified nonlinear bound, inferential test, or family theorem.
//! One-atom H4 = the SAME tuple for all four actors; NEVER treat unrestricted
//! actor-specific fixed-F as an H4 one-atom population.
use hiyal::core::{AgentId, World};
use serde_json::{Value, json};
use social_emergence_lab::experiment003 as e;

const N_EVENTS: usize=384;
const RUST_TOL: f64=1e-11;
const MAX_CANDIDATE_EVALS: usize=2000;

fn record(index:usize,period:usize)->e::PublicRecord{
    assert!(index<N_EVENTS);
    let mask=index%16;
    let fine_index=(index/16)%4;
    let mu=(index/64)%3;
    let regime=if index/192==0{e::Regime::Random}else{e::Regime::Choice};
    let fine_bits=[(fine_index&1) as u8, ((fine_index>>1)&1) as u8];
    let actions=std::array::from_fn(|i|((mask>>i)&1) as u8);
    e::PublicRecord{
        period,regime,matching:mu,fine_bits,actions,
        payoffs:e::Society::payoff_for(mu,fine_bits,actions)
    }
}
fn probability(s:&e::Society,r:e::PublicRecord)->f64{
    let z=match (s.design.policy,r.regime){
        (e::Policy::A,_)=>0.5,
        (e::Policy::A0,e::Regime::Random)=>1.0,
        _=>return 0.0,
    };
    let fine=match s.design.policy{
        e::Policy::A=>0.25,
        e::Policy::A0 if r.fine_bits==[0,0]=>1.0,
        _=>return 0.0,
    };
    z*fine*s.broker(r.regime)[r.matching]
        *s.joint_action_probability(r.matching,r.fine_bits,r.actions)
}
fn full_period(s:&e::Society,period:usize)->Vec<f64>{
    (0..N_EVENTS).map(|i|probability(s,record(i,period))).collect()
}
fn tv(a:&[f64],b:&[f64])->f64{
    assert_eq!(a.len(),b.len());
    a.iter().zip(b).map(|(x,y)|(x-y).abs()).sum::<f64>()/2.0
}
fn normalize(v:&[f64])->bool{(v.iter().sum::<f64>()-1.0).abs()<RUST_TOL}
fn sig(x:f64)->f64{1.0/(1.0+(-x).exp())}
fn homogeneous(family:e::Family,policy:e::Policy,g:e::Globals,tuple:e::Tuple)->e::Society{
    let mut d=e::pilot_design(family,policy);
    d.globals=g;
    d.atoms=vec![e::Atom{weight:1.0,tuple}];
    e::Society::new_fixed(d,[tuple;4]).expect("bounded legal homogeneous H4")
}
fn true_n(policy:e::Policy)->e::Society{
    let g=e::Globals{beta:4.0,epsilon:0.0,rho:1.0,delta:0.0,omega:0.0};
    homogeneous(e::Family::N,policy,g,e::Tuple::N{eta:0.0,nu:0.75})
}
fn m_candidate(policy:e::Policy,beta:f64,epsilon:f64)->e::Society{
    let g=e::Globals{beta,epsilon,rho:1.0,delta:0.0,omega:0.0};
    homogeneous(e::Family::M,policy,g,e::Tuple::M{eta:0.0})
}
#[derive(Clone,Copy)]
struct BestM{beta:f64,epsilon:f64,tv:f64}
fn scan_m_grid(policy:e::Policy)->(BestM,usize){
    let true_mass=full_period(&true_n(policy),1);
    assert!(normalize(&true_mass));
    let mut best=BestM{beta:0.0,epsilon:0.0,tv:f64::INFINITY};
    let mut tested=0;
    // Fixed, fully enumerable 100x21 grid. This is NOT interval-enclosed
    // continuous parameter search. eta/rho cancel at T1 for homogeneous M.
    for b in 1..=100{
        for eps in 0..=20{
            let beta=b as f64/10.0;
            let epsilon=eps as f64/40.0;
            let cand=m_candidate(policy,beta,epsilon);
            let law=full_period(&cand,1);
            assert!(normalize(&law));
            let discrepancy=tv(&true_mass,&law);
            tested+=1;
            if discrepancy<best.tv{
                best=BestM{beta,epsilon,tv:discrepancy};
            }
        }
    }
    assert_eq!(tested,2100);
    (best,tested)
}
fn universal_m_a_lower_bound()->(f64,f64){
    // Under any legal M (including all allowed H4 atom mixtures), actor 0's
    // half-fine conditional cooperation is <= pMmax; its fine independently
    // equals 1/2 with mass 1/2 under policy A.
    // Therefore TV(full public A law, any M rival) >= the difference between
    // probabilities of event {actor0 fined, actor0 contributes}.
    let p_m_max=0.25+0.5*sig(-0.05); // beta=0.1, epsilon=0.5
    let p_n=sig(1.0);                // beta=4, nu=0.75, fine=0.5
    assert!(p_n>p_m_max && p_m_max<0.5);
    (p_m_max,0.5*(p_n-p_m_max))
}
fn m_vs_n(policy:e::Policy)->Value{
    let (best,evals)=scan_m_grid(policy);
    let (upper_m,bound)=universal_m_a_lower_bound();
    if policy==e::Policy::A{
        assert!(best.tv+RUST_TOL>=bound);
    }else{
        assert!(best.tv<RUST_TOL); // M(beta=1,epsilon=0) is an A0 alias
    }
    json!({
        "truth_family":"N",
        "rival_family":"M",
        "policy":policy.name(),
        "h4_restriction":"one identical homogeneous actor tuple, no free actor types",
        "target_nu":0.75,"target_beta":4.0,"target_epsilon":0.0,
        "candidate_grid":"beta = 0.1,...,10.0 (100 values); epsilon = 0,0.025,...,0.5 (21 values)",
        "candidate_evaluations":evals,
        "grid_min_T1_full_law_tv":best.tv,
        "best_grid_beta":best.beta,
        "best_grid_epsilon":best.epsilon,
        "all_H4_M_max_half_fine_cooperation":upper_m,
        "all_H4_M_A_full_law_tv_lower_bound":if policy==e::Policy::A{Some(bound)}else{None},
        "bound_logic":"analytic event inequality valid for any M H4 law under A, independent of eta/rho and matching; no statistical test",
        "grid_logic":"grid minimum is an attainable upper bound on continuous restricted-one-atom infimum, never a separation certificate"
    })
}
#[derive(Clone,Copy,Debug)]
struct LParams{beta:f64,epsilon:f64,rho:f64,eta:f64,kappa:f64,alpha:f64,zeta:f64}
impl LParams{
    fn values(self)->[f64;7]{
        [self.beta,self.epsilon,self.rho,self.eta,self.kappa,self.alpha,self.zeta]
    }
    fn from(v:[f64;7])->Self{
        Self{beta:v[0],epsilon:v[1],rho:v[2],eta:v[3],kappa:v[4],alpha:v[5],zeta:v[6]}
    }
}
const LO:[f64;7]=[0.1,0.0,0.1,-3.0-1.0/3.0,-4.0,0.0,0.0];
const HI:[f64;7]=[10.0,0.5,10.0,3.0-1.0/3.0,4.0,1.0,1.0];
const INITIAL_STEPS:[f64;7]=[1.0,0.1,1.0,0.25,1.0,0.25,0.25];
fn legal_params(p:LParams)->bool{
    p.values().iter().enumerate().all(|(i,v)|v.is_finite()&&*v>=LO[i]-RUST_TOL&&*v<=HI[i]+RUST_TOL)
}
fn l_candidate(policy:e::Policy,p:LParams)->e::Society{
    assert!(legal_params(p));
    let g=e::Globals{beta:p.beta,epsilon:p.epsilon,rho:p.rho,delta:0.0,omega:0.0};
    homogeneous(e::Family::L,policy,g,e::Tuple::L{
        eta:p.eta,kappa:p.kappa,alpha:p.alpha,zeta:p.zeta
    })
}
fn true_c(policy:e::Policy)->e::Society{
    let g=e::Globals{beta:1.0,epsilon:0.0,rho:1.0,delta:0.0,omega:0.0};
    homogeneous(e::Family::C,policy,g,e::Tuple::C{eta:0.0,kappa:1.0})
}
// Post-review amendment v0.2: three NON-isomorphic first-period histories.
// A: mixed (1,0)/(1,0); mixed (1,0)/(0,0); and fined mixed dyads.
// A0: the first two plus an all-contribution event. Under A0 there are no
// fines, so zeta is structurally inert at period 2 regardless of the prefix.
fn prefixes(policy:e::Policy)->[e::PublicRecord;3]{
    let actions=[[1,0,1,0],[1,0,0,0],[1,1,0,0]];
    std::array::from_fn(|i|{
        let fine_bits=if i==2 && policy==e::Policy::A {[1,0]}else{[0,0]};
        let a=if i==2 && policy==e::Policy::A0 {[1,1,1,1]}else{actions[i]};
        e::PublicRecord{
            period:1,regime:e::Regime::Random,matching:i,
            fine_bits,actions:a,
            payoffs:e::Society::payoff_for(i,fine_bits,a)
        }
    })
}
fn suffix_law(s:&e::Society,pr:e::PublicRecord)->Vec<f64>{
    let mut copy=s.clone();
    assert!(probability(&copy,pr)>0.0);
    copy.apply(AgentId(0),pr).unwrap();
    let law=full_period(&copy,2);
    assert!(normalize(&law));
    law
}
struct Reference{
    t1:Vec<f64>,
    suffixes:[Vec<f64>;3],
}
fn reference_c(policy:e::Policy)->Reference{
    let c=true_c(policy);
    let t1=full_period(&c,1);
    assert!(normalize(&t1));
    let suffixes=prefixes(policy).map(|r|suffix_law(&c,r));
    Reference{t1,suffixes}
}
fn conditional_objective(target:&Reference,rival:&e::Society)->(f64,f64,f64){
    // Deliberately declared PRE-analysis diagnostic discrepancy. Equal
    // weights for selected rare conditionals make this a useful sensitivity
    // search, but NOT a full T2 joint TV, likelihood, or inference statistic.
    let t1=tv(&target.t1,&full_period(rival,1));
    let mut mean_suffix=0.0;
    for (pr,src) in prefixes(rival.design.policy).into_iter().zip(target.suffixes.iter()){
        mean_suffix+=tv(src,&suffix_law(rival,pr))/3.0;
    }
    (0.2*t1+0.8*mean_suffix,t1,mean_suffix)
}
#[derive(Clone,Copy)]
struct BestL{params:LParams,objective:f64,t1_tv:f64,mean_conditional_tv:f64}
fn evaluate_l(target:&Reference,policy:e::Policy,p:LParams,evals:&mut usize)->BestL{
    *evals+=1;
    assert!(*evals<=MAX_CANDIDATE_EVALS,"bounded numerical search budget exceeded");
    let rival=l_candidate(policy,p);
    let (objective,t1_tv,mean_conditional_tv)=conditional_objective(target,&rival);
    assert!(objective.is_finite());
    BestL{params:p,objective,t1_tv,mean_conditional_tv}
}
fn fit_l(policy:e::Policy)->(BestL,BestL,usize){
    let target=reference_c(policy);
    // Baseline literal frozen-then-partner map is known to fail on rematching.
    let init_map=LParams{beta:1.0,epsilon:0.0,rho:1.0,eta:0.0,kappa:1.0,alpha:1.0,zeta:0.0};
    let starts=[
        init_map,
        LParams{beta:1.7,epsilon:0.08,rho:2.3,eta:0.1,kappa:1.4,alpha:0.5,zeta:0.5},
        LParams{beta:3.0,epsilon:0.25,rho:5.0,eta:0.0,kappa:-1.0,alpha:0.0,zeta:1.0},
    ];
    let mut evals=0;
    let baseline=evaluate_l(&target,policy,init_map,&mut evals);
    let mut winner=baseline;
    // Additional EXACT-T1-MATCH submanifold search. Keep beta=1,
    // epsilon=0, kappa=1: all t1 action laws exactly match C for
    // arbitrary alpha/zeta. Search along this manifold to avoid
    // TV1's nonsmooth cusp trapping independent-coordinate moves.
    // Under A0 zeta cannot affect the T2 selected law, so fix it to 0.
    for ai in 0..=10 {
        let z_count=if policy==e::Policy::A {11}else{1};
        for zi in 0..z_count{
            let alpha=ai as f64/10.0;
            let zeta=zi as f64/10.0;
            let candidate=evaluate_l(&target,policy,LParams{
                alpha,zeta,..init_map
            },&mut evals);
            assert!(candidate.t1_tv<RUST_TOL);
            if candidate.objective<winner.objective {
                winner=candidate;
            }
        }
    }
    for start in starts{
        let mut current=evaluate_l(&target,policy,start,&mut evals);
        // Coordinate pattern search with four decreasing step scales and two
        // sweeps per scale, fixed coordinate order and stable tie handling.
        // This is LOCAL search, not a global optimizer or interval certificate.
        for scale in [1.0,0.5,0.25,0.125]{
            for _ in 0..2{
                for k in 0..7{
                    let values=current.params.values();
                    for direction in [-1.0,1.0]{
                        let mut shifted=values;
                        shifted[k]=(shifted[k]+direction*scale*INITIAL_STEPS[k]).clamp(LO[k],HI[k]);
                        if shifted==values{continue;}
                        let candidate=evaluate_l(&target,policy,LParams::from(shifted),&mut evals);
                        if candidate.objective+1e-13<current.objective{
                            current=candidate;
                        }
                    }
                }
            }
        }
        if current.objective<winner.objective {winner=current;}
    }
    assert!(evals<=MAX_CANDIDATE_EVALS);
    assert!(winner.objective<=baseline.objective+RUST_TOL);
    (baseline,winner,evals)
}
fn l_detail(b:BestL)->Value{
    let p=b.params;
    json!({"objective":b.objective,"T1_full_record_tv":b.t1_tv,
        "mean_three_T2_suffix_conditional_tvs":b.mean_conditional_tv,
        "beta":p.beta,"epsilon":p.epsilon,"rho":p.rho,
        "eta":p.eta,"kappa":p.kappa,"alpha":p.alpha,"zeta":p.zeta})
}
fn c_vs_l(policy:e::Policy)->Value{
    let target=reference_c(policy);
    let self_match=conditional_objective(&target,&true_c(policy));
    assert!(self_match.0<RUST_TOL);
    let (baseline,best,evals)=fit_l(policy);
    json!({
        "truth_family":"C","rival_family":"L","policy":policy.name(),
        "h4_restriction":"one identical homogeneous actor tuple; NO multiple H4 atoms",
        "target":{"beta":1.0,"epsilon":0.0,"rho":1.0,"eta":0.0,"kappa":1.0},
        "target_self_match_objective":self_match.0,
        "prefixes":if policy==e::Policy::A {
            "three NON-ISOMORPHIC t1 records: random matching mu0/no fine/[1,0,1,0]; mu1/no fine/[1,0,0,0]; mu2/fine bits [1,0]/[1,1,0,0]"
        }else{
            "three NON-ISOMORPHIC t1 records: random mu0/no fine/[1,0,1,0]; mu1/no fine/[1,0,0,0]; mu2/no fine/[1,1,1,1]; zeta inert at T2 under A0"
        },
        "objective":"0.2 * TV(full 384-record T1 law) + 0.8/3 * sum TV(complete 384-record T2 suffix | one distinct t1 prefix)",
        "alpha_zeta_manifold_grid":if policy==e::Policy::A{"11x11 alpha/zeta, with beta=1 epsilon=0 kappa=1, exact T1 matching"}else{"11 alpha points with zeta=0, because zeta unidentifiable at T2 under A0"},
        "dimension":7,"parameters":["beta","epsilon","rho","eta","kappa","alpha","zeta"],
        "parameter_bounds":{"beta":[0.1,10.0],"epsilon":[0.0,0.5],"rho":[0.1,10.0],
            "eta":[-3.0-1.0/3.0,3.0-1.0/3.0],"kappa":[-4.0,4.0],
            "alpha":[0.0,1.0],"zeta":[0.0,1.0]},
        "starts":3,"coordinate_scales":[1.0,0.5,0.25,0.125],
        "sweeps_per_scale":2,"candidate_evaluations":evals,
        "fixed_frozen_partner_mapping":l_detail(baseline),
        "best_local_one_atom_fit":l_detail(best),
        "scope":"local deterministic pattern search over ONE-ATOM H4, not global one-atom infimum, not full 4-atom H4, not T2 full joint TV or exclusion certificate; positive residual proves nothing about attainable free-rival equality"
    })
}
fn evaluate()->Value{
    let mut policies=Vec::new();
    for p in e::Policy::ALL{
        policies.push(json!({"policy":p.name(),"N_to_M":m_vs_n(p),"C_to_L":c_vs_l(p)}));
    }
    json!({
        "model":"Experiment003-A2-v0.2.2",
        "version":"restricted-H4-one-atom-free-rival-feasibility-v0.2-post-review",
        "status":"EXPLORATORY_NUMERICAL_NO_GLOBAL_H4_IDENTIFICATION",
        "no_stochastic_draws":true,
        "target_level":"one-atom homogeneous actor-iid H4 only",
        "M_target_bound":"analytic public event lower bound for selected N truth valid versus ALL unrestricted M H4 under A",
        "C_target_limit":"three non-isomorphic T1 prefixes, fined history under A, exact T1 manifold alpha-zeta mesh plus local search; A0 zeta structurally inert through T2. No global free-rival lower bound",
        "policies":policies
    })
}
fn main(){println!("{}",serde_json::to_string_pretty(&evaluate()).unwrap());}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn indexing_and_full_support_normalization(){
        for policy in e::Policy::ALL{
            let c=true_c(policy);let n=true_n(policy);
            let q=full_period(&c,1);let m=full_period(&n,1);
            assert_eq!(q.len(),N_EVENTS);
            assert!(normalize(&q)&&normalize(&m));
            let count=q.iter().filter(|v|**v>0.0).count();
            assert_eq!(count,if policy==e::Policy::A{384}else{48});
            for pr in prefixes(policy){
                assert!(probability(&c,pr)>0.0);
                let law=suffix_law(&c,pr);assert!(normalize(&law));
            }
        }
    }
    #[test]
    fn analytic_all_m_h4_bound_is_a_valid_witness(){
        let (pm,lower)=universal_m_a_lower_bound();
        assert!((pm-0.49375130175789483).abs()<RUST_TOL);
        assert!((lower-0.11865363843605503).abs()<RUST_TOL);
        let (best,_) = scan_m_grid(e::Policy::A);
        assert!((best.tv-0.33386582682695076).abs()<RUST_TOL);
        assert!(best.tv+RUST_TOL>=lower);
        let (control,_) = scan_m_grid(e::Policy::A0);
        assert!(control.tv<RUST_TOL);
        // 0.3339 is an attainable grid UPPER bound on the true infimum,
        // whereas 0.11865 is an analytic all-H4 M LOWER bound.
        assert!(best.tv-lower>0.2);
    }
    #[test]
    fn prefixes_are_distinct_and_activate_fine_learning_only_under_a(){
        let a=prefixes(e::Policy::A);
        let null=prefixes(e::Policy::A0);
        assert_eq!(a[0].fine_bits,[0,0]);
        assert_eq!(a[1].fine_bits,[0,0]);
        assert_eq!(a[2].fine_bits,[1,0]);
        assert!(null.iter().all(|r|r.fine_bits==[0,0]));
        assert_eq!(null[2].actions,[1,1,1,1]);
        assert_eq!(a[1].actions,[1,0,0,0]);
        // The first two are nonisomorphic as the number of contributions
        // and the multiset of two dyad action compositions differ.
        assert_ne!(a[0].actions.iter().sum::<u8>(),a[1].actions.iter().sum::<u8>());
        // Relative to the same prior history, zeta is observable under A
        // in this selected fined prefix, while it cannot affect an A0 T2
        // law because the only prior fine is zero.
        let mut p=LParams{beta:1.0,epsilon:0.0,rho:1.0,eta:0.0,
            kappa:1.0,alpha:0.75,zeta:0.0};
        let l0=l_candidate(e::Policy::A,p);
        p.zeta=1.0;
        let l1=l_candidate(e::Policy::A,p);
        assert!(tv(&suffix_law(&l0,a[2]),&suffix_law(&l1,a[2]))>1e-4);
        let n0=l_candidate(e::Policy::A0,LParams{zeta:0.0,..p});
        let n1=l_candidate(e::Policy::A0,LParams{zeta:1.0,..p});
        for r in null {
            assert!(tv(&suffix_law(&n0,r),&suffix_law(&n1,r))<RUST_TOL);
        }
    }
    #[test]
    fn one_atom_free_l_search_never_worse_than_proposed_fixed_mapping(){
        for policy in e::Policy::ALL{
            let (baseline,best,n)=fit_l(policy);
            assert!(n<MAX_CANDIDATE_EVALS);
            assert!(best.objective<=baseline.objective+RUST_TOL);
            assert!(best.objective.is_finite());
            // Pin v0.2 amended, nonisomorphic-prefix results separately
            // from original (and superseded) v0.1 numerical values.
            let (expected_obj,expected_evals)=if policy==e::Policy::A{
                (0.11427257283116886,425)
            }else{(0.07188489112933487,311)};
            assert!((best.objective-expected_obj).abs()<RUST_TOL);
            assert_eq!(n,expected_evals);
        }
    }
}
