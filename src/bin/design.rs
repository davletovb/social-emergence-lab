//! Analysis-only instrument. Does not generate new society outcomes or unlock confirmation.
use social_emergence_lab::{
    ACTORS, INITIAL_WEALTH, OFFSET,
    analytic::predict_moments,
    design::{PilotSpec, plan_precision, terminal_pairs_file},
};
use std::{env, path::Path};

const USAGE: &str = "usage: cargo run --locked --bin design -- <replicated-hiyal-trajectories.csv> \
[--actors N] [--initial W] [--offset B] [--horizon T] [--sd-multiplier M] [--half-width H] [--mde D] [--illustrative-n K]";

fn parse<T: std::str::FromStr>(flag: &str, value: &str) -> Result<T, String> {
    value
        .parse()
        .map_err(|_| format!("invalid value for {flag}: {value}"))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = (|| -> Result<(), String> {
        let (input, flags) = args.split_first().ok_or(USAGE)?;
        if flags.len() % 2 != 0 {
            return Err(USAGE.into());
        }
        // Defaults are the observed pilot: the library's primary population at
        // T=500. Different horizons need their own pilot; extrapolating is unsound.
        let mut spec = PilotSpec {
            actors: ACTORS,
            initial: INITIAL_WEALTH,
            offset: OFFSET,
            horizon: 500,
        };
        let (mut sd_multiplier, mut half_width, mut mde, mut illustrative_n) =
            (2.0, 0.01, 0.02, 100);
        for pair in flags.chunks(2) {
            let (flag, value) = (pair[0].as_str(), pair[1].as_str());
            match flag {
                "--actors" => spec.actors = parse(flag, value)?,
                "--initial" => spec.initial = parse(flag, value)?,
                "--offset" => spec.offset = parse(flag, value)?,
                "--horizon" => spec.horizon = parse(flag, value)?,
                "--sd-multiplier" => sd_multiplier = parse(flag, value)?,
                "--half-width" => half_width = parse(flag, value)?,
                "--mde" => mde = parse(flag, value)?,
                "--illustrative-n" => illustrative_n = parse(flag, value)?,
                _ => return Err(USAGE.into()),
            }
        }
        let input = Path::new(input);
        let pairs = terminal_pairs_file(input, &spec)?;
        let proposed = plan_precision(&pairs, sd_multiplier, half_width, mde, illustrative_n)?;
        let theory = predict_moments(spec.actors, spec.initial, spec.offset, spec.horizon)?;
        let output = serde_json::json!({
            "source":input.to_string_lossy(),
            "experiment":"001-emergence-of-inequality",
            "scientific_status":"exploratory_planning_only",
            "pilot":{"actors":spec.actors,"initial_wealth":spec.initial,"reinforcement_offset":spec.offset,"accepted_opportunities":spec.horizon,
                "seeds":pairs.iter().map(|p|p.seed).collect::<Vec<_>>(),
                "source":format!("Hiyal verified {}-society exploratory batch; NOT held-out confirmation", pairs.len())},
            "independent_urn_variance_benchmark":{
                "control_increment_variance":theory.uniform_variance,
                "reinforcement_increment_variance":theory.reinforced_variance,
                "ratio":theory.variance_ratio,
                "note":"Exact mathematical variance ratio for the specified toy mechanism, not an empirical sociological discovery."
            },
            "planning":proposed,
            "pre_registration_status":"NOT LOCKED OR REGISTERED"
        });
        println!(
            "{}",
            serde_json::to_string_pretty(&output).map_err(|e| e.to_string())?
        );
        Ok(())
    })();
    if let Err(e) = result {
        eprintln!("design failed: {e}");
        std::process::exit(2);
    }
}
