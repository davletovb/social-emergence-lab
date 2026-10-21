//! Usage: cargo run --locked --bin reputation-pilot -- /tmp/new-trial 62000 6 2 20 60 default
//! Explicitly exploratory; output includes independent replayable journals and report.
use social_emergence_lab::reputation_pilot::{control_config,run};
use std::{env,path::Path};

#[tokio::main]
async fn main(){
    let args=env::args().skip(1).collect::<Vec<_>>();
    let result=async {
        if args.len()!=7 {
            return Err("usage: reputation-pilot <NEW-OUTPUT-DIR> <SEED> <ACTORS-even-6..40> <DEGREE> <CHECKPOINT> <TOTAL> <default|no-false-prior|no-selection-bias|fixed-graph|random-churn>".into());
        }
        let parse=|i:usize|args[i].parse::<u64>().map_err(|_|format!("invalid integer {}",args[i]));
        let seed=parse(1)?;
        let actors=usize::try_from(parse(2)?).map_err(|e|e.to_string())?;
        let degree=usize::try_from(parse(3)?).map_err(|e|e.to_string())?;
        let checkpoint=parse(4)?;let total=parse(5)?;
        let config=control_config(actors,degree,&args[6])?;
        let report=run(Path::new(&args[0]),seed,config,checkpoint,total).await?;
        println!("{}",serde_json::to_string_pretty(&report).map_err(|e|e.to_string())?);
        Ok::<(),String>(())
    }.await;
    if let Err(error)=result{eprintln!("reputation pilot failed: {error}");std::process::exit(2);}
}
