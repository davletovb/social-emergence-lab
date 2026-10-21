//! Precision planning from *exploratory, paired-society* terminal outcomes.
//! This is an approximate normal-theory design heuristic, NOT a power proof.
use crate::stats::Pair;
use std::{collections::BTreeMap, path::Path, fs};

const HEADER: &str =
    "actors,initial,offset,horizon,seed,arm,step,gini,top_share,rank_persistence,mean_rank_movement";

/// Read one primary population/treatment/horizon from a complete Hiyal
/// trajectory export, pairing the shared uniform control with the treatment.
pub fn terminal_pairs(
    csv: &str, actors: usize, initial: u64, offset: u64, horizon: u64,
) -> Result<Vec<Pair>, String> {
    if actors < 2 || initial == 0 || offset == 0 || horizon == 0 {
        return Err("invalid design parameters".into());
    }
    let mut lines = csv.lines();
    if lines.next() != Some(HEADER) {
        return Err("unrecognized Hiyal trajectories schema".into());
    }
    let mut groups: BTreeMap<u64,(Option<f64>, Option<f64>)> = BTreeMap::new();
    for (i, line) in lines.enumerate() {
        let p: Vec<_> = line.split(',').collect();
        if p.len() != 11 { return Err(format!("bad row {}", i+2)); }
        let n = p[0].parse::<usize>().map_err(|_|"invalid actor count")?;
        let w0 = p[1].parse::<u64>().map_err(|_|"invalid starting wealth")?;
        let t = p[3].parse::<u64>().map_err(|_|"invalid horizon")?;
        let step = p[6].parse::<u64>().map_err(|_|"invalid recorded step")?;
        if n!=actors || w0!=initial || t!=horizon || step!=horizon {continue;}
        let seed = p[4].parse::<u64>().map_err(|_|"invalid seed")?;
        let gini = p[7].parse::<f64>().map_err(|_|"invalid gini")?;
        if !gini.is_finite() || !(0.0..=1.0).contains(&gini) {return Err("invalid terminal gini".into());}
        let record=groups.entry(seed).or_default();
        match p[5] {
            "control" if p[2].is_empty() => {
                if record.0.replace(gini).is_some(){return Err("duplicate uniform control".into());}
            },
            "treatment" if p[2] == offset.to_string() => {
                if record.1.replace(gini).is_some(){return Err("duplicate treatment".into());}
            },
            "control"|"treatment"=>{},
            _=>return Err("unrecognized arm".into()),
        }
    }
    if groups.len()<2{return Err("at least two pilot society pairs required".into());}
    groups.into_iter().map(|(seed,(control,treatment))|
        Ok(Pair{seed,horizon,control:control.ok_or("unpaired control")?,treatment:treatment.ok_or("unpaired treatment")?})
    ).collect()
}

pub fn terminal_pairs_file(
    path: &Path, actors: usize, initial: u64, offset: u64, horizon: u64,
) -> Result<Vec<Pair>, String> {
    terminal_pairs(&fs::read_to_string(path).map_err(|e|e.to_string())?,
        actors,initial,offset,horizon)
}

#[derive(Debug,Clone,serde::Serialize)]
pub struct PrecisionPlan {
    pub pilot_societies: usize,
    pub pilot_mean_paired_delta: f64,
    pub pilot_sd_paired_delta: f64,
    pub variance_inflation_factor_on_sd: f64,
    pub conservative_planning_sd: f64,
    pub target_ci_half_width: f64,
    pub approximate_societies_for_95_ci_half_width: usize,
    pub minimum_mean_difference_of_interest: f64,
    pub approximate_societies_for_90pct_power_two_sided_5pct: usize,
    pub illustrative_societies: usize,
    pub expected_95pct_ci_half_width_at_illustrative_n: f64,
    pub scientific_status: &'static str,
}

fn positive_finite(n:f64)->bool{n.is_finite()&&n>0.0}
fn safe_ceiling(n:f64)->Result<usize,String>{
    if !n.is_finite()||n<=0.0||n>100_000.0 {
        return Err("required sample size is invalid or over 100,000".into());
    }
    Ok(n.ceil().max(2.0) as usize)
}
/// Uses z=1.9599639845 and z_(90% power)=1.2815515655. The first is
/// approximate target CI precision; the second is an *unpaired-normal*
/// heuristic applied to SD of paired deltas (no trial randomness model).
/// These approximations must be independently reviewed before preregistration.
pub fn plan_precision(
    pairs:&[Pair],sd_inflation:f64,ci_half_width:f64,mde:f64,illustrative_n:usize,
)->Result<PrecisionPlan,String>{
    if pairs.len()<3||!positive_finite(sd_inflation)||sd_inflation<1.0
        ||!positive_finite(ci_half_width)||!positive_finite(mde)||illustrative_n<2 {
        return Err("invalid precision-design inputs".into());
    }
    let horizon=pairs[0].horizon;
    if pairs.iter().any(|p|p.horizon!=horizon){return Err("mixed pilot horizons".into());}
    let mean = pairs.iter().map(Pair::delta).sum::<f64>() / pairs.len() as f64;
    let sd = (pairs.iter().map(|p|(p.delta()-mean).powi(2)).sum::<f64>()/(pairs.len()-1) as f64).sqrt();
    if !positive_finite(sd){return Err("zero or invalid pilot variability".into());}
    let assumed=sd_inflation*sd;
    let z=1.959963984540054;
    let z_power=1.2815515655446004;
    Ok(PrecisionPlan{
        pilot_societies:pairs.len(),
        pilot_mean_paired_delta:mean,
        pilot_sd_paired_delta:sd,
        variance_inflation_factor_on_sd:sd_inflation,
        conservative_planning_sd:assumed,
        target_ci_half_width:ci_half_width,
        approximate_societies_for_95_ci_half_width:safe_ceiling((z*assumed/ci_half_width).powi(2))?,
        minimum_mean_difference_of_interest:mde,
        approximate_societies_for_90pct_power_two_sided_5pct:safe_ceiling(((z+z_power)*assumed/mde).powi(2))?,
        illustrative_societies:illustrative_n,
        expected_95pct_ci_half_width_at_illustrative_n:z*assumed/(illustrative_n as f64).sqrt(),
        scientific_status:"Exploratory pilot-based NORMAL-APPROXIMATION heuristic. Do not equate with empirical causal evidence, guarantee of confidence interval width, or registered power.",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pilot_design_uses_independent_societies_not_agents() {
        let p=(0..10).map(|i|Pair{
            seed:i, horizon:500,control:0.05,treatment:0.06+i as f64*0.002
        }).collect::<Vec<_>>();
        let x=plan_precision(&p,2.0,0.01,0.02,100).unwrap();
        assert_eq!(x.pilot_societies,10);
        assert!(x.conservative_planning_sd>x.pilot_sd_paired_delta);
        assert!(x.approximate_societies_for_95_ci_half_width>=2);
        assert!(x.expected_95pct_ci_half_width_at_illustrative_n>0.0);
    }
    #[test]
    fn trajectory_parser_respects_shared_control_and_seed_alignment() {
        let mut s=format!("{HEADER}\n");
        for seed in [9,10] {
            s.push_str(&format!("20,10,,500,{seed},control,500,0.1000,0.2,,\n"));
            s.push_str(&format!("20,10,1,500,{seed},treatment,500,0.1300,0.2,,\n"));
            s.push_str(&format!("20,10,10,500,{seed},treatment,500,0.1100,0.2,,\n"));
        }
        let pair=terminal_pairs(&s,20,10,1,500).unwrap();
        assert_eq!(pair.len(),2);
        assert!((pair[0].delta()-0.03).abs()<1e-12);
        s.push_str("20,10,,500,9,control,500,0.1,0.2,,\n");
        assert!(terminal_pairs(&s,20,10,1,500).is_err());
    }
}
