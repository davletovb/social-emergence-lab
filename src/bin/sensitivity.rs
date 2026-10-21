//! Standalone *reference-model* sensitivity grid, NOT Hiyal-derived trials.
//! Explores whether the direction of a mechanism survives different specifications.
//! Results may guide preregistration but must not be mistaken for confirmatory evidence.
use social_emergence_lab::{reference::{reference_gini,simulate_reference},stats::{Pair,summarize}};
use std::{env,fs,fmt::Write as _,path::Path};

fn main(){
    let args:Vec<_>=env::args().skip(1).collect();
    let run=(||->Result<(),String>{
        if args.len()!=3 {
            return Err("usage: cargo run --locked --bin sensitivity -- <output-dir> <pairs:10..=100> <first-seed>".into());
        }
        let root=Path::new(&args[0]);
        let pairs=args[1].parse::<usize>().map_err(|e|e.to_string())?;
        let first_seed=args[2].parse::<u64>().map_err(|e|e.to_string())?;
        if !(10..=100).contains(&pairs) {return Err("use 10..=100 paired seeds".into());}
        fs::create_dir_all(root).map_err(|e|e.to_string())?;
        let mut outcomes=String::from("actors,initial_wealth,offset,steps,seed,arm,gini\n");
        let mut summary=String::from("actors,initial_wealth,offset,steps,pairs,control_mean_gini,treatment_mean_gini,paired_delta,bootstrap_95_lo,bootstrap_95_hi\n");
        for actors in [10usize,20,40] {
            for offset in [1u64,10,100] {
                for steps in [100u64,500] {
                    let mut matched=Vec::new();
                    for i in 0..pairs {
                        let seed=first_seed.checked_add(i as u64).ok_or("seed overflow")?;
                        // Each arm restarts its own identical reference RNG domain.
                        let c=simulate_reference(seed,actors,10,offset,steps,false)?;
                        let t=simulate_reference(seed,actors,10,offset,steps,true)?;
                        let cg=reference_gini(&c.wealth)?;
                        let tg=reference_gini(&t.wealth)?;
                        writeln!(outcomes,"{actors},10,{offset},{steps},{seed},control,{cg:.12}").map_err(|e|e.to_string())?;
                        writeln!(outcomes,"{actors},10,{offset},{steps},{seed},treatment,{tg:.12}").map_err(|e|e.to_string())?;
                        matched.push(Pair{seed,horizon:steps,control:cg,treatment:tg});
                    }
                    let stat=summarize(&matched,2000,0x51_2026_001)?;
                    let interval=stat.bootstrap_percentile_95.ok_or("insufficient pairs")?;
                    writeln!(summary,"{actors},10,{offset},{steps},{pairs},{:.12},{:.12},{:.12},{:.12},{:.12}",
                        stat.mean_control,stat.mean_treatment,stat.mean_delta,interval[0],interval[1]
                    ).map_err(|e|e.to_string())?;
                }
            }
        }
        fs::write(root.join("reference-sensitivity-raw.csv"),outcomes).map_err(|e|e.to_string())?;
        fs::write(root.join("reference-sensitivity-summary.csv"),summary).map_err(|e|e.to_string())?;
        println!("Exploratory independent-reference sensitivity grid completed: {} paired seeds x 18 specifications; {}",pairs,root.display());
        Ok(())
    })();
    if let Err(e)=run{eprintln!("{e}");std::process::exit(2);}
}
