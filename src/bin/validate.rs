//! Independently re-execute accepted journal actions and compare to Hiyal's
//! fully verified state. This deliberately does not reuse the engine reducer.
use hiyal::storage::Store;
use social_emergence_lab::{
    Allocate,Society,gini,top_quintile_share,
    reference::{replay,reference_gini,reference_top_share},
};
use std::{env,path::Path};
fn main(){
    let args:Vec<_>=env::args().skip(1).collect();
    if args.len()!=1{
        eprintln!("usage: cargo run --locked --bin validate -- <arm-trial-directory>");
        std::process::exit(2);
    }
    let result=(||->Result<String,String>{
        let path=Path::new(&args[0]);
        // First: independent Hiyal ledger/hash/ancestry check via public read-only API.
        let ro=Store::<Society,Allocate>::inspect_read_only(path)?;
        // Then: deliberately independent, plain-vector reference implementation.
        let expected=replay(&path.join("events.jsonl"),&ro.genesis.initial_world.wealth)?;
        if expected.wealth!=ro.world.wealth || expected.opportunities!=ro.logical_step
            || expected.opportunities!=ro.world.opportunities {
            return Err("independent reference disagrees with verified Hiyal world".into());
        }
        let actual_gini=gini(&ro.world.wealth)?;
        let check_gini=reference_gini(&expected.wealth)?;
        let actual_top=top_quintile_share(&ro.world.wealth)?;
        let check_top=reference_top_share(&expected.wealth)?;
        if (actual_gini-check_gini).abs()>1e-12 ||(actual_top-check_top).abs()>1e-12 {
            return Err("independent reference metrics disagree".into());
        }
        Ok(format!("Reference validation passed: {} accepted opportunities, Gini {:.12}, chain head {}",expected.opportunities,check_gini,ro.chain_head))
    })();
    match result {
        Ok(message)=>println!("{message}"),
        Err(e)=>{eprintln!("reference validation failed: {e}");std::process::exit(2);}
    }
}
