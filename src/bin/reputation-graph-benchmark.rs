//! Bounded graph rejection benchmark. Each attempted seed is reported,
//! including failures; this is a *performance diagnostic*, not research data.
use hiyal::rng::Streams;
use serde::Serialize;
use social_emergence_lab::reputation::{Config,sample_regular_graph};
use std::{env,fs,path::Path,process, time::Instant};
#[derive(Serialize)]
struct Sample {seed:u64,graph:String,attempts:Option<usize>,elapsed_micros:u128,error:Option<String>}
#[derive(Serialize)]
struct Results<'a> {
    status:&'a str,actors:usize,degree:usize,seeds:Vec<u64>,attempt_cap:usize,
    attempted_graphs:usize,accepted_graphs:usize,failed_graphs:usize,
    samples:Vec<Sample>,
    caution:&'a str,
}
fn run()->Result<(),String>{
    let a=env::args().skip(1).collect::<Vec<_>>();
    if a.len()!=5{return Err("usage: reputation-graph-benchmark <new-output.json> <actors> <degree> <first-seed> <count-1..12>".into());}
    let out=Path::new(&a[0]);
    if out.exists(){return Err("refusing to overwrite existing benchmark".into());}
    let parse=|i:usize|a[i].parse::<u64>().map_err(|_|format!("invalid integer: {}",a[i]));
    let n=usize::try_from(parse(1)?).map_err(|e|e.to_string())?;
    let d=usize::try_from(parse(2)?).map_err(|e|e.to_string())?;
    let start=parse(3)?;
    let count=usize::try_from(parse(4)?).map_err(|e|e.to_string())?;
    if !(1..=12).contains(&count){return Err("benchmark count must be 1..12".into());}
    let cfg=Config::default_for(n,d);cfg.validate()?;
    let mut samples=Vec::new();
    let mut seeds=Vec::new();
    for i in 0..count {
        let seed=start.checked_add(i as u64).ok_or("seed overflow")?;
        seeds.push(seed);
        let mut rng=Streams::new(format!("v1-graph-benchmark-{n}-{d}"),seed);
        for stream in ["setup/initial_graph","setup/repair_graph"] {
            let t=Instant::now();
            let result=sample_regular_graph(&cfg,&mut rng,stream);
            let elapsed=t.elapsed().as_micros();
            match result {
                Ok(s)=>samples.push(Sample{seed,graph:stream.into(),attempts:Some(s.attempts),elapsed_micros:elapsed,error:None}),
                Err(e)=>samples.push(Sample{seed,graph:stream.into(),attempts:None,elapsed_micros:elapsed,error:Some(e)}),
            }
        }
    }
    let failed=samples.iter().filter(|x|x.error.is_some()).count();
    let report=Results{
        status:"EXPLORATORY_GRAPH_SAMPLER_BENCHMARK",
        actors:n,degree:d,seeds,attempt_cap:cfg.graph_attempts,
        attempted_graphs:samples.len(),accepted_graphs:samples.len()-failed,failed_graphs:failed,samples,
        caution:"Non-random hardware/runtime and fixed diagnostic seeds. No upper bound on true tail failure probability. Report failed seeds without substitution.",
    };
    fs::write(out,serde_json::to_vec_pretty(&report).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    println!("{}",serde_json::to_string_pretty(&report).map_err(|e|e.to_string())?);
    if failed>0{return Err(format!("{failed} graph draws hit the bounded cap; record them, do not replace seeds"));}
    Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("graph benchmark: {e}");process::exit(2);}}
