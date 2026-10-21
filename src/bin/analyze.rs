//! Read already produced Hiyal pilot CSV; bootstrap only across independent societies.
use social_emergence_lab::stats::{read_pairs, summarize};
use std::{env, path::Path};
fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("usage: cargo run --locked --bin analyze -- <results.csv>");
        std::process::exit(2);
    }
    let outcome = (|| -> Result<String, String> {
        let pairs = read_pairs(Path::new(&args[0]))?;
        let summary = summarize(&pairs, 10_000, 0x5eed_2026_1021)?;
        serde_json::to_string_pretty(&summary).map_err(|e| e.to_string())
    })();
    match outcome {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("analysis failed: {err}");
            std::process::exit(2);
        }
    }
}
