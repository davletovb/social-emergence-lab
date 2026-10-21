//! Hiyal-engine parameter grid with replay-derived trajectories and benchmarks.
use social_emergence_lab::hiyal_study::{StudyPlan, run};
use std::{env, path::Path};
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = async {
        const USAGE: &str = "usage: cargo run --locked --bin hiyal-grid -- <smoke|grid> <output-dir> <pairs> <first-seed>";
        if args.len() != 4 {
            return Err(USAGE.to_owned());
        }
        let pairs: usize = args[2]
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        let seed: u64 = args[3]
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        // run() validates the plan before touching the output directory.
        let plan = match args[0].as_str() {
            "smoke" => StudyPlan::small(seed, pairs)?,
            "grid" => StudyPlan::grid(seed, pairs)?,
            _ => return Err(USAGE.to_owned()),
        };
        run(Path::new(&args[1]), plan).await?;
        println!(
            "Hiyal-backed exploratory {} complete; outputs in {}",
            args[0], args[1]
        );
        Ok::<(), String>(())
    }
    .await;
    if let Err(error) = result {
        eprintln!("Hiyal study failed: {error}");
        std::process::exit(2);
    }
}
