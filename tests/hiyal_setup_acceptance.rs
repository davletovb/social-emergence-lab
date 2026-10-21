//! Consumer acceptance test for Hiyal's `Trial::create_with_setup` in the
//! shape Experiment 002 needs: balanced false-boost labels and two uniformly
//! sampled d-regular digraphs (initial and repair) drawn from the trial's own
//! `setup/labels`, `setup/initial_graph` and `setup/repair_graph` streams, a
//! preintervention history, four forks (sham, correction, repair, both),
//! resume, and every audit Hiyal offers. The world is a minimal stand-in, not
//! the Experiment 002 model.
use hiyal::{
    core::{AgentId, World},
    experiment::{EffectPolicy, Trial, verify_setup},
    research::{fork_at_head, resume_fork, verify_fork},
    rng::Streams,
    runtime::{Receipt, ScriptedBackend},
    scheduler::Scheduling,
    storage::Store,
};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

const ACTORS: usize = 8;
const DEGREE: usize = 2;
const KIND: &str = "social-emergence-lab.setup-acceptance.v1";

fn dir(tag: &str) -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    std::env::temp_dir().join(format!(
        "lab-setup-{}-{}-{}",
        tag,
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Access {
    boosted: Vec<bool>,
    reputation: Vec<u64>,
    graph: Vec<Vec<usize>>,
    repair: Vec<Vec<usize>>,
    tokens: Vec<u64>,
}

impl World for Access {
    type Action = usize;
    type Observation = Vec<(usize, u64)>;
    fn observe(&self, actor: AgentId) -> Result<Self::Observation, String> {
        let neighbors = self.graph.get(actor.0 as usize).ok_or("bad allocator")?;
        Ok(neighbors.iter().map(|&i| (i, self.reputation[i])).collect())
    }
    fn apply(&mut self, actor: AgentId, recipient: usize) -> Result<(), String> {
        let neighbors = self.graph.get(actor.0 as usize).ok_or("bad allocator")?;
        if !neighbors.contains(&recipient) {
            return Err("recipient is not an eligible neighbor".into());
        }
        self.tokens[recipient] += 1;
        Ok(())
    }
}

/// Reputation-weighted choice among the allocator's neighbors, drawn from
/// the allocator's own `agent/<j>/selection` stream as Hiyal requires.
struct Select {
    stream: fn(AgentId) -> String,
}
impl EffectPolicy<Access> for Select {
    fn decide(&self, _: AgentId, _: &Vec<(usize, u64)>, _: &[Receipt]) -> Result<usize, String> {
        Err("selection requires named streams".into())
    }
    fn decide_with_streams(
        &self,
        actor: AgentId,
        obs: &Vec<(usize, u64)>,
        _: &[Receipt],
        rng: &mut Streams,
    ) -> Result<usize, String> {
        let total: u64 = obs.iter().map(|(_, r)| r + 1).sum();
        let mut ticket = rng.uniform_index(&(self.stream)(actor), total as usize)? as u64;
        for &(i, r) in obs {
            if ticket <= r {
                return Ok(i);
            }
            ticket -= r + 1;
        }
        Err("ticket out of range".into())
    }
}
fn scoped(actor: AgentId) -> String {
    format!("agent/{}/selection", actor.0)
}
fn flat(_: AgentId) -> String {
    "selection".into()
}

/// Uniform simple d-in/d-out digraph by configuration pairing with rejection.
fn regular_digraph(streams: &mut Streams, stream: &str) -> Result<Vec<Vec<usize>>, String> {
    loop {
        let mut targets: Vec<usize> = (0..ACTORS).flat_map(|i| [i; DEGREE]).collect();
        for k in (1..targets.len()).rev() {
            let j = streams.uniform_index(stream, k + 1)?;
            targets.swap(k, j);
        }
        let rows: Vec<Vec<usize>> = targets.chunks(DEGREE).map(<[usize]>::to_vec).collect();
        let simple = rows.iter().enumerate().all(|(j, row)| {
            !row.contains(&j) && (1..row.len()).all(|a| !row[..a].contains(&row[a]))
        });
        if simple {
            return Ok(rows);
        }
    }
}

fn setup(streams: &mut Streams) -> Result<Access, String> {
    let mut order: Vec<usize> = (0..ACTORS).collect();
    for i in (1..ACTORS).rev() {
        let j = streams.uniform_index("setup/labels", i + 1)?;
        order.swap(i, j);
    }
    let mut boosted = vec![false; ACTORS];
    for &i in &order[..ACTORS / 2] {
        boosted[i] = true;
    }
    Ok(Access {
        reputation: boosted.iter().map(|&b| if b { 7 } else { 3 }).collect(),
        boosted,
        graph: regular_digraph(streams, "setup/initial_graph")?,
        repair: regular_digraph(streams, "setup/repair_graph")?,
        tokens: vec![0; ACTORS],
    })
}

fn create_parent(path: &Path, stream: fn(AgentId) -> String) -> Trial<Access, Select> {
    Trial::create_with_setup(
        path,
        "setup-acceptance",
        2026,
        KIND,
        setup,
        Select { stream },
        Arc::new(ScriptedBackend),
        (0..ACTORS as u64).map(AgentId).collect(),
        Scheduling::Random,
    )
    .unwrap()
}

#[tokio::test]
async fn setup_streams_survive_history_forks_and_resume() {
    let root = dir("four-arm");
    let parent_dir = root.join("parent");
    let mut parent = create_parent(&parent_dir, scoped);
    let world = parent.store.genesis().initial_world.clone();
    assert_eq!(world.boosted.iter().filter(|b| **b).count(), ACTORS / 2);
    for graph in [&world.graph, &world.repair] {
        let mut in_degree = [0; ACTORS];
        for (j, row) in graph.iter().enumerate() {
            assert_eq!(row.len(), DEGREE);
            assert!(!row.contains(&j));
            row.iter().for_each(|&i| in_degree[i] += 1);
        }
        assert!(in_degree.iter().all(|&k| k == DEGREE));
    }
    let recorded = parent.store.genesis().initial_streams.positions.clone();
    let names: Vec<_> = recorded.keys().cloned().collect();
    assert_eq!(
        names,
        ["setup/initial_graph", "setup/labels", "setup/repair_graph"]
    );

    // Preintervention history, then four arms forked at the same head.
    assert_eq!(parent.run_attempts(30).await.unwrap(), 30);
    parent.checkpoint().unwrap();
    drop(parent);
    let arms: [(&str, fn(&mut Access)); 4] = [
        ("sham", |_| {}),
        ("correct", |w| w.reputation = vec![5; ACTORS]),
        ("repair", |w| w.graph = w.repair.clone()),
        ("both", |w| {
            w.reputation = vec![5; ACTORS];
            w.graph = w.repair.clone();
        }),
    ];
    for (arm, intervene) in arms {
        let mut trial = fork_at_head(
            &parent_dir,
            root.join(arm),
            "setup-acceptance",
            arm,
            serde_json::json!({}),
            |w: &mut Access| {
                intervene(w);
                Ok(())
            },
            Select { stream: scoped },
            Arc::new(ScriptedBackend),
        )
        .unwrap();
        // Forks inherit the setup positions untouched.
        for (name, position) in &recorded {
            assert_eq!(
                trial.store.genesis().initial_streams.positions.get(name),
                Some(position)
            );
        }
        assert_eq!(trial.run_attempts(20).await.unwrap(), 20);
        trial.checkpoint().unwrap();
    }

    // Resume one arm and continue, then audit everything.
    let mut resumed: Trial<Access, Select> = resume_fork(
        root.join("both"),
        Select { stream: scoped },
        Arc::new(ScriptedBackend),
        false,
    )
    .unwrap();
    assert_eq!(resumed.run_attempts(10).await.unwrap(), 10);
    resumed.checkpoint().unwrap();
    drop(resumed);
    for arm in ["sham", "correct", "repair", "both"] {
        verify_fork::<Access>(root.join(arm)).unwrap();
        let ro = Store::<Access, usize>::inspect_read_only(root.join(arm)).unwrap();
        assert_eq!(
            ro.world.tokens.iter().sum::<u64>(),
            30 + 20 + if arm == "both" { 10 } else { 0 }
        );
        // Setup is audited on the parent that ran it, never on a fork.
        assert!(verify_setup(root.join(arm), setup).is_err());
    }
    Store::<Access, usize>::inspect_read_only(&parent_dir).unwrap();
    verify_setup(&parent_dir, setup).unwrap();
    std::fs::remove_dir_all(root).unwrap();
}

#[tokio::test]
async fn policy_draws_outside_agent_streams_are_rejected() {
    // Experiment 002's design originally named flat streams such as
    // `selection`; Hiyal rejects any policy draw outside `agent/<id>/`.
    let root = dir("flat");
    let mut trial = create_parent(&root, flat);
    let error = trial.run_attempts(1).await.unwrap_err();
    assert!(
        error.contains("unauthorized RNG stream draw: selection"),
        "{error}"
    );
    drop(trial);
    std::fs::remove_dir_all(root).unwrap();
}
