//! Analysis-only instrument. Does not generate new society outcomes or unlock confirmation.
use social_emergence_lab::{
    analytic::predict_moments,
    design::{plan_precision,terminal_pairs_file},
};
use std::{env,path::Path};

fn main() {
    let args:Vec<String>=env::args().skip(1).collect();
    let result=(||->Result<(),String>{
        if args.len()!=1 {return Err("usage: cargo run --locked --bin design -- <replicated-hiyal-trajectories.csv>".into());}
        let input=Path::new(&args[0]);
        // Specification matches *observed* pilot data. Different horizons
        // require a different pilot; extrapolating to 1000 steps is unsound.
        let pairs=terminal_pairs_file(input,20,10,1,500)?;
        let proposed=plan_precision(&pairs,2.0,0.01,0.02,100)?;
        let theory=predict_moments(20,10,1,500)?;
        let output=serde_json::json!({
            "source":input.to_string_lossy(),
            "experiment":"001-emergence-of-inequality",
            "scientific_status":"exploratory_planning_only",
            "pilot":{"actors":20,"initial_wealth":10,"reinforcement_offset":1,"accepted_opportunities":500,
                "seeds":pairs.iter().map(|p|p.seed).collect::<Vec<_>>(),
                "source":"Hiyal verified 10-society exploratory batch; NOT held-out confirmation"},
            "independent_urn_variance_benchmark":{
                "control_increment_variance":theory.uniform_variance,
                "reinforcement_increment_variance":theory.reinforced_variance,
                "ratio":theory.variance_ratio,
                "note":"Exact mathematical variance ratio for the specified toy mechanism, not an empirical sociological discovery."
            },
            "planning":proposed,
            "pre_registration_status":"NOT LOCKED OR REGISTERED"
        });
        println!("{}",serde_json::to_string_pretty(&output).map_err(|e|e.to_string())?);
        Ok(())
    })();
    if let Err(e)=result{
        eprintln!("design failed: {e}");
        std::process::exit(2);
    }
}
