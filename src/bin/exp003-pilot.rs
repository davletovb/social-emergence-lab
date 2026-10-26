//! Manually invoked exploratory pilot. Never launched by CI.
//! cargo run --bin exp003-pilot -- /new/output/directory
use hiyal::{
    core::AgentId, experiment::{Trial,verify_setup},
    runtime::ScriptedBackend, scheduler::Scheduling, storage::Store,
};
use social_emergence_lab::experiment003::{self as e, Generator, PublicRecord, Society};
use std::{env,fs,path::Path,sync::Arc};

const WORLDS_PER_CELL:usize=2;
const FIRST_SEED:u64=303_000;

#[tokio::main]
async fn main(){
    let args:Vec<_>=env::args().collect();
    if args.len()!=2 {eprintln!("usage: exp003-pilot /new/output/directory");std::process::exit(2);}
    if let Err(err)=run(Path::new(&args[1])).await {eprintln!("exp003: {err}");std::process::exit(1);}
}
async fn run(root:&Path)->Result<(),String>{
    if root.exists(){return Err("output exists; refusing overwrite".into());}
    let max=6*2*WORLDS_PER_CELL*4*4;
    if max!=384 {return Err("pilot bound mismatch".into());}
    for family in e::Family::ALL {for policy in e::Policy::ALL {
        e::pilot_design(family,policy).validate()?;
    }}
    fs::create_dir_all(root.join("hiyal-internal")).map_err(|e|e.to_string())?;
    fs::create_dir_all(root.join("public-only")).map_err(|e|e.to_string())?;
    let mut manifest=String::from("id,seed,model,policy,periods,actors,kind\n");
    for (fi,family) in e::Family::ALL.iter().enumerate(){
        for (pi,policy) in e::Policy::ALL.iter().enumerate(){
            for k in 0..WORLDS_PER_CELL {
                let seed=FIRST_SEED+((fi*2+pi)*WORLDS_PER_CELL+k) as u64;
                let id=format!("exp003-{}-{}-{}",family.name(),policy.name(),k);
                let dir=root.join("hiyal-internal").join(&id);
                let design=e::pilot_design(*family,*policy);
                let periods=design.periods;
                let setup_design=design.clone();
                let mut trial=Trial::create_with_setup(
                    &dir,&id,seed,e::KIND,
                    move |rng| Society::setup(setup_design,rng),
                    Generator,Arc::new(ScriptedBackend),vec![AgentId(0)],
                    Scheduling::RoundRobin,
                )?;
                let accepted=trial.run_attempts(periods as u64).await?;
                if accepted!=periods as u64 {return Err("unexpected rejection".into());}
                trial.checkpoint()?;
                drop(trial);
                verify_setup(&dir, |rng|Society::setup(design.clone(),rng))?;
                let verified=Store::<Society,PublicRecord>::inspect_read_only(&dir)?;
                if verified.logical_step!=periods as u64 || verified.world.history.len()!=periods {
                    return Err("verified journal length mismatch".into());
                }
                // The classifier-facing files have no labels, latent variables,
                // private beliefs, scores, RNG draws or paired counterfactuals.
                let public=serde_json::to_vec_pretty(&verified.world.history).map_err(|e|e.to_string())?;
                fs::write(root.join("public-only").join(format!("{id}.json")),public).map_err(|e|e.to_string())?;
                manifest.push_str(&format!("{id},{seed},{},{},{periods},4,{}\n",family.name(),policy.name(),e::KIND));
            }
        }
    }
    // Private audit-only manifest must NOT be supplied to classifiers.
    fs::write(root.join("hiyal-internal").join("truth-manifest.csv"),manifest).map_err(|e|e.to_string())?;
    fs::write(root.join("README.txt"),format!(
        "EXPLORATORY ENGINEERING ONLY; NO CONFIRMATORY CLAIMS\nmodel={}\nHiyal pin={}\nHiyal journals include latent truth; do not use in classifier.\nPublic-only JSON omits truth and private state. Filenames are audit-only labels: rename/strip before fitting.\n24 societies, 4 periods, 384 actor-periods; 12 societies per policy, 2 per family-policy cell.\nNOT a held-out evaluation. No inferential accuracy or 95% coverage.\n",
        e::VERSION,social_emergence_lab::runner::hiyal_revision()?
    )).map_err(|e|e.to_string())?;
    Ok(())
}
