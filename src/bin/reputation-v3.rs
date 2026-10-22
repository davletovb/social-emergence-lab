//! Manual-only V3 exploratory runner. Never launch the 9-cell grid in CI.
//! Usage: reputation-v3 run <NEW-ABS-DIR> <seed> <period> <mechanism> <n> <d> <tau> <T>
//!        reputation-v3 verify <DIR>
use social_emergence_lab::reputation_prior_pair::{run_pair, verify_pair};
use social_emergence_lab::reputation_v3::{Mechanism, cell_config};
use std::{env, path::Path, process};

const USAGE: &str = "usage: reputation-v3 run <NEW-ABS-DIR> <seed> <period:2|10|20> \
<mechanism:random-turnover|belief-deletion|full-feedback> <actors> <degree> <checkpoint> <total> | verify <DIR>";

async fn execute(args: &[String]) -> Result<(), String> {
    let report = match args {
        [command, dir] if command == "verify" => verify_pair(Path::new(dir))?,
        [
            command,
            dir,
            seed,
            period,
            mechanism,
            actors,
            degree,
            checkpoint,
            total,
        ] if command == "run" => {
            if !Path::new(dir).is_absolute() {
                return Err("V3 evidence output must be an absolute path".into());
            }
            let parse = |value: &String| {
                value
                    .parse::<u64>()
                    .map_err(|_| format!("invalid nonnegative integer: {value}"))
            };
            let size = |value: &String| usize::try_from(parse(value)?).map_err(|e| e.to_string());
            let mode = Mechanism::parse(mechanism)?;
            let config = cell_config(size(actors)?, size(degree)?, parse(period)?, mode)?;
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
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = execute(&arguments).await {
        eprintln!("reputation-v3: {error}");
        process::exit(2);
    }
}
