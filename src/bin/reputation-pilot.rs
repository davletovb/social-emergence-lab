//! Usage: cargo run --locked --bin reputation-pilot -- /tmp/new-trial 62000 6 2 20 60 default
//!        cargo run --locked --bin reputation-pilot -- verify /tmp/new-trial
//! Explicitly exploratory; output includes independent replayable journals and report.json,
//! which `verify` re-derives from those journals.
use social_emergence_lab::reputation_pilot::{control_config, run, verify};
use std::{env, path::Path};

const USAGE: &str = "usage: reputation-pilot <NEW-OUTPUT-DIR> <SEED> <ACTORS-even-6..40> <DEGREE> \
<CHECKPOINT> <TOTAL> <default|no-false-prior|no-selection-bias|fixed-graph|random-churn>\n       \
reputation-pilot verify <RUN-DIR>";

async fn main_result(args: &[String]) -> Result<(), String> {
    let report = match args {
        [command, dir] if command == "verify" => verify(Path::new(dir))?,
        [dir, seed, actors, degree, checkpoint, total, case] => {
            let parse = |x: &String| x.parse::<u64>().map_err(|_| format!("invalid integer {x}"));
            let size = |x: &String| usize::try_from(parse(x)?).map_err(|e| e.to_string());
            let config = control_config(size(actors)?, size(degree)?, case)?;
            run(
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
    let args = env::args().skip(1).collect::<Vec<_>>();
    if let Err(error) = main_result(&args).await {
        eprintln!("reputation pilot failed: {error}");
        std::process::exit(2);
    }
}
