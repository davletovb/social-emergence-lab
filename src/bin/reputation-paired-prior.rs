//! Usage: reputation-paired-prior run <NEW-DIR> <seed> <actors> <degree> <checkpoint> <total>
//!        reputation-paired-prior verify <EXISTING-DIR>
use social_emergence_lab::reputation::{Config};
use social_emergence_lab::reputation_prior_pair::{run_pair,verify_pair};
use std::{env,path::Path,process};
#[tokio::main]
async fn main(){
    let args:Vec<_>=env::args().skip(1).collect();
    let result:Result<(),String>=async {
        if args.len()==2 && args[0]=="verify" {
            let r=verify_pair(Path::new(&args[1]))?;
            println!("{}",serde_json::to_string_pretty(&r).map_err(|e|e.to_string())?);
            return Ok(());
        }
        if args.len()!=7 || args[0]!="run" {
            return Err("usage: reputation-paired-prior run <NEW-ABSOLUTE-DIR> <seed> <actors> <degree> <checkpoint> <total> | verify <DIR>".into());
        }
        let u=|n:usize|args[n].parse::<u64>().map_err(|e|e.to_string());
        let seed=u(2)?;
        let n=usize::try_from(u(3)?).map_err(|e|e.to_string())?;
        let d=usize::try_from(u(4)?).map_err(|e|e.to_string())?;
        let result=run_pair(Path::new(&args[1]),seed,Config::default_for(n,d),u(5)?,u(6)?).await?;
        println!("{}",serde_json::to_string_pretty(&result).map_err(|e|e.to_string())?);
        Ok(())
    }.await;
    if let Err(error)=result {eprintln!("reputation paired prior: {error}");process::exit(2);}
}
