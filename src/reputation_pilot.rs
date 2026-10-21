//! Bounded, replay-verified *exploratory* Experiment 002 runs. No confirmatory claims.
use crate::reputation::{Arm, Config, KIND, Removal, ReputationPolicy, ReputationWorld, Window, setup, window};
use hiyal::{
    core::AgentId,
    experiment::{Trial, verify_setup},
    research::{fork_at_head, verify_fork},
    runtime::ScriptedBackend,
    scheduler::Scheduling,
    storage::Store,
};
use serde::Serialize;
use std::{path::Path,sync::Arc,fs};

#[derive(Debug,Clone,Serialize)]
pub struct ArmReport {
    pub arm:String,
    pub full:Window,
    pub late:Window,
    pub quarters:Vec<Window>,
    pub final_tokens:Vec<u64>,
    pub all_events:u64,
}
#[derive(Debug,Clone,Serialize)]
pub struct PilotReport {
    pub scientific_status:&'static str,
    pub seed:u64,
    pub config:Config,
    pub checkpoint:u64,
    pub total_steps:u64,
    pub hiyal_revision:&'static str,
    pub per_arm:Vec<ArmReport>,
    pub primary_information_late_gap:f64,
    pub repair_after_truth_late:f64,
    pub repair_without_truth_full:f64,
    pub variance_and_significance_claims:&'static str,
}

pub async fn run(
    root:&Path,seed:u64,config:Config,checkpoint:u64,total:u64,
)->Result<PilotReport,String>{
    config.validate()?;
    if checkpoint==0||total<=checkpoint||total>1000||checkpoint>500||root.exists(){
        return Err("pilot needs a new path, 0<checkpoint<total<=1000 and checkpoint<=500".into());
    }
    let config_for_genesis=config.clone();
    let agent_ids=(0..config.actors as u64).map(AgentId).collect();
    fs::create_dir_all(root).map_err(|e|e.to_string())?;
    let parent_dir=root.join("parent");
    let backend=Arc::new(ScriptedBackend);
    let mut parent=Trial::create_with_setup(
        &parent_dir,"reputation-v0",seed,KIND,
        move |rng|setup(&config_for_genesis,rng),
        ReputationPolicy,backend.clone(),agent_ids,Scheduling::Random,
    )?;
    let attempts=usize::try_from(checkpoint).map_err(|e|e.to_string())?;
    if parent.run_attempts(attempts).await.map_err(|e|e.to_string())? !=attempts{
        return Err("parent did not commit all opportunities".into());
    }
    parent.checkpoint()?;
    parent.store.world().verify()?;
    drop(parent);
    verify_setup(&parent_dir,move |rng|setup(&config,rng))?;
    Store::<ReputationWorld,crate::reputation::Action>::inspect_read_only(&parent_dir)?;
    let mut reports=Vec::new();
    for arm in Arm::ALL {
        let path=root.join(arm.name());
        let mut trial=fork_at_head(
            &parent_dir,&path,"information-network-2x2",arm.name(),
            serde_json::json!({"checkpoint":checkpoint,"arm":arm.name()}),
            |w:&mut ReputationWorld|w.intervene(arm),
            ReputationPolicy,backend.clone(),
        )?;
        if trial.store.world().step!=checkpoint{
            return Err("fork does not start at checkpoint".into());
        }
        let post=usize::try_from(total-checkpoint).map_err(|e|e.to_string())?;
        if trial.run_attempts(post).await.map_err(|e|e.to_string())?!=post{
            return Err("fork did not commit all post-intervention opportunities".into());
        }
        trial.checkpoint()?;
        drop(trial);
        verify_fork::<ReputationWorld>(&path)?;
        let ro=Store::<ReputationWorld,crate::reputation::Action>::inspect_read_only(&path)?;
        ro.world.verify()?;
        if ro.world.step!=total||ro.logical_step!=total {
            return Err("final world/ledger step mismatch".into());
        }
        let last=total.saturating_sub(99).max(checkpoint+1);
        let quarters=if total-checkpoint>=400 && (total-checkpoint)%4==0 {
            let chunk=(total-checkpoint)/4;
            (0..4).map(|i|window(&ro.world,checkpoint+1+i*chunk,checkpoint+(i+1)*chunk)).collect::<Result<Vec<_>,_>>()?
        } else {vec![]};
        reports.push(ArmReport{
            arm:arm.name().into(),full:window(&ro.world,checkpoint+1,total)?,
            late:window(&ro.world,last,total)?,quarters,
            final_tokens:ro.world.wealth,all_events:ro.logical_step,
        });
    }
    let info=reports.iter().find(|x|x.arm=="information").ok_or("missing information arm")?;
    let both=reports.iter().find(|x|x.arm=="both").ok_or("missing combined arm")?;
    let sham=reports.iter().find(|x|x.arm=="sham").ok_or("missing sham arm")?;
    let structure=reports.iter().find(|x|x.arm=="structure").ok_or("missing structure arm")?;
    let answer=PilotReport{
        scientific_status:"EXPLORATORY_ONLY_NOT_PREREGISTERED",
        seed,
        config:config.clone(),checkpoint,total_steps:total,
        hiyal_revision:"4bb53aca5cffa6bc01b7558efbd690af7dbed79f",
        primary_information_late_gap:info.late.opportunity_gap,
        repair_after_truth_late:info.late.opportunity_gap-both.late.opportunity_gap,
        repair_without_truth_full:sham.full.opportunity_gap-structure.full.opportunity_gap,
        variance_and_significance_claims:"One parent seed; no uncertainty estimate, power or empirical causal inference. Run and report independent parent societies only after reviewing scientific design.",
        per_arm:reports,
    };
    fs::write(root.join("report.json"),serde_json::to_vec_pretty(&answer).map_err(|e|e.to_string())?)
        .map_err(|e|e.to_string())?;
    Ok(answer)
}
/// Declared negative controls for *exploratory-only* model tests.
pub fn control_config(n:usize,d:usize,case:&str)->Result<Config,String>{
    let mut c=Config::default_for(n,d);
    match case {
        "default"=>{},
        "no-false-prior"=>c.delta=0.0,
        "no-selection-bias"=>c.beta=0.0,
        "fixed-graph"=>c.removal=Removal::None,
        "random-churn"=>{c.removal=Removal::Uniform;c.kappa=0.0;},
        _=>return Err("unknown control: default|no-false-prior|no-selection-bias|fixed-graph|random-churn".into()),
    }
    c.validate()?;
    Ok(c)
}
