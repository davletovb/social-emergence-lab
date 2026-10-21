//! Usage: reputation-paired-prior run <NEW-DIR> <seed> <actors> <degree> <checkpoint> <total>
//!        reputation-paired-prior verify <EXISTING-DIR>
use social_emergence_lab::reputation::Config;
use social_emergence_lab::reputation_prior_pair::{run_pair, verify_pair};
use std::{env, path::Path, process};

const USAGE: &str = "usage: reputation-paired-prior run <NEW-ABSOLUTE-DIR> <seed> <actors> \
<degree> <checkpoint> <total> | verify <DIR>";

async fn main_result(args: &[String]) -> Result<(), String> {
    let report = match args {
        [command, dir] if command == "verify" => verify_pair(Path::new(dir))?,
        [command, dir, seed, actors, degree, checkpoint, total] if command == "run" => {
            let parse = |x: &String| x.parse::<u64>().map_err(|e| e.to_string());
            let size = |x: &String| usize::try_from(parse(x)?).map_err(|e| e.to_string());
            let config = Config::default_for(size(actors)?, size(degree)?);
            run_pair(
                Path::new(dir),
                parse(seed)?,
                config,
                parse(checkpoint)?,
                parse(total)?,
            )
            .await?
        }
        _ => return Err(USAGE.into()),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if let Err(error) = main_result(&args).await {
        eprintln!("reputation paired prior: {error}");
        process::exit(2);
    }
}
