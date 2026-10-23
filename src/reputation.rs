//! Experiment 002: transparent reputation and endogenous opportunity-network world.
//! Exploration only: rules are a specified synthetic mechanism, not human behavior.
use hiyal::{
    core::{AgentId, World as HiyalWorld},
    experiment::EffectPolicy,
    rng::Streams,
    runtime::Receipt,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub const KIND: &str = "social-emergence-lab.reputation.v0";
/// Same model, but the world keeps no per-step `trace`: trajectories are
/// rebuilt from the verified journal (`reputation_reference::replay_journal_trace`).
/// State hashes then cover O(N²) state instead of a trace growing with every
/// event. A separate kind, so V0 journals and hashes are never reinterpreted.
pub const KIND_JOURNAL_TRACE: &str = "social-emergence-lab.reputation.journal-trace.v1";

/// The Hiyal world kind a configuration must be recorded under.
pub fn kind_for(config: &Config) -> &'static str {
    if config.journal_trace {
        KIND_JOURNAL_TRACE
    } else {
        KIND
    }
}
pub const MAX_GRAPH_ATTEMPTS: usize = 200_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Removal {
    LowestBelief,
    Uniform,
    None,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub actors: usize,
    pub degree: usize,
    pub beta: f64,
    pub epsilon: f64,
    pub delta: f64,
    pub learning: f64,
    pub public_learning: f64,
    pub kappa: f64,
    pub rewire_every: u64,
    pub removal: Removal,
    pub graph_attempts: usize,
    /// Development control: replace all selection links with a new balanced
    /// permuted-circulant graph before each opportunity; never retain ties.
    #[serde(default, skip_serializing_if = "is_false")]
    pub annealed: bool,
    /// Reproduce legacy journals exactly: old V0 manifests lacked this flag.
    #[serde(default, skip_serializing_if = "is_false")]
    pub survival_accounting: bool,
    /// Keep no `trace` in the world; derive it from the journal instead.
    /// Requires world kind `KIND_JOURNAL_TRACE`. Absent from older configs,
    /// whose serialization (and so every V0 hash) is unchanged.
    #[serde(default, skip_serializing_if = "is_false")]
    pub journal_trace: bool,
}
fn is_false(v: &bool) -> bool {
    !*v
}

impl Config {
    pub fn default_for(actors: usize, degree: usize) -> Self {
        Self {
            actors,
            degree,
            beta: 4.0,
            epsilon: 0.1,
            delta: 0.2,
            learning: 0.1,
            public_learning: 0.1,
            kappa: 6.0,
            rewire_every: 10,
            removal: Removal::LowestBelief,
            graph_attempts: MAX_GRAPH_ATTEMPTS,
            annealed: false,
            survival_accounting: true,
            journal_trace: false,
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        if !(6..=40).contains(&self.actors)
            || self.actors % 2 != 0
            || self.degree == 0
            || self.degree >= self.actors
            || (self.degree == self.actors - 1 && self.removal != Removal::None)
            || (self.annealed && self.removal != Removal::None)
            || self.rewire_every == 0
            || !(1..=MAX_GRAPH_ATTEMPTS).contains(&self.graph_attempts)
        {
            return Err("invalid population, degree, rewire period or bounded graph budget".into());
        }
        for (x, low, high) in [
            (self.beta, 0.0, 20.0),
            (self.epsilon, 0.0, 1.0),
            (self.delta, 0.0, 0.5),
            (self.learning, 0.0, 1.0),
            (self.public_learning, 0.0, 1.0),
            (self.kappa, 0.0, 20.0),
        ] {
            if !x.is_finite() || x < low || x > high {
                return Err("invalid model parameter".into());
            }
        }
        Ok(())
    }
}

/// Exact uniform sampling over simple directed d-in/d-out-regular graphs,
/// conditioned on configuration-model rejection. Stop rather than loop forever.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphSample {
    pub edges: Vec<Vec<usize>>,
    pub attempts: usize,
}

/// Count every attempted uniform stub pairing, including rejected graphs.
pub fn sample_regular_graph(
    config: &Config,
    streams: &mut Streams,
    name: &str,
) -> Result<GraphSample, String> {
    config.validate()?;
    let n = config.actors;
    let d = config.degree;
    for attempt in 1..=config.graph_attempts {
        let mut targets: Vec<usize> = (0..n).flat_map(|i| std::iter::repeat_n(i, d)).collect();
        for k in (1..targets.len()).rev() {
            let index = streams.uniform_index(name, k + 1)?;
            targets.swap(k, index);
        }
        let edges: Vec<Vec<usize>> = targets.chunks(d).map(<[usize]>::to_vec).collect();
        if validate_graph(&edges, n, d).is_ok() {
            return Ok(GraphSample {
                edges,
                attempts: attempt,
            });
        }
    }
    Err(format!(
        "simple {d}-regular directed graph not found after {} bounded draws on {name}",
        config.graph_attempts
    ))
}

pub fn regular_graph(
    config: &Config,
    streams: &mut Streams,
    name: &str,
) -> Result<Vec<Vec<usize>>, String> {
    Ok(sample_regular_graph(config, streams, name)?.edges)
}

/// Balanced but explicitly NONUNIFORM graph family for annealed-mixing null.
/// Node permutation is freshly drawn from the acting agent stream every step.
pub fn annealed_regular_graph(
    n: usize,
    d: usize,
    rng: &mut Streams,
    stream: &str,
) -> Result<Vec<Vec<usize>>, String> {
    if n < 3 || d == 0 || d >= n {
        return Err("bad annealed graph dimensions".into());
    }
    let mut order: Vec<usize> = (0..n).collect();
    for k in (1..n).rev() {
        let j = rng.uniform_index(stream, k + 1)?;
        order.swap(k, j);
    }
    let mut edges = vec![Vec::with_capacity(d); n];
    for k in 0..n {
        for h in 1..=d {
            edges[order[k]].push(order[(k + h) % n]);
        }
    }
    validate_graph(&edges, n, d)?;
    Ok(edges)
}

pub fn validate_graph(edges: &[Vec<usize>], n: usize, d: usize) -> Result<(), String> {
    if edges.len() != n {
        return Err("wrong number of graph rows".into());
    }
    let mut incoming = vec![0usize; n];
    for (j, row) in edges.iter().enumerate() {
        if row.len() != d {
            return Err("outdegree invariant failed".into());
        }
        let mut seen = vec![false; n];
        for &i in row {
            if i >= n || i == j || seen[i] {
                return Err("self, duplicate or invalid edge".into());
            }
            seen[i] = true;
            incoming[i] += 1;
        }
    }
    if incoming.iter().any(|&v| v != d) {
        return Err("indegree invariant failed".into());
    }
    Ok(())
}
fn check_rows(edges: &[Vec<usize>], n: usize, d: usize) -> Result<(), String> {
    if edges.len() != n {
        return Err("bad row count".into());
    }
    for (j, row) in edges.iter().enumerate() {
        if row.len() != d {
            return Err("outdegree changed".into());
        }
        let mut seen = vec![false; n];
        for &i in row {
            if i >= n || i == j || seen[i] {
                return Err("invalid current graph".into());
            }
            seen[i] = true;
        }
    }
    Ok(())
}

pub fn setup(config: &Config, rng: &mut Streams) -> Result<ReputationWorld, String> {
    config.validate()?;
    let n = config.actors;
    let mut names: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        let j = rng.uniform_index("setup/labels", i + 1)?;
        names.swap(i, j);
    }
    let mut boost = vec![false; n];
    for &i in &names[..n / 2] {
        boost[i] = true;
    }
    let (initial_graph, repair_graph) = if config.degree == n - 1 {
        let complete = (0..n)
            .map(|j| (0..n).filter(|&i| i != j).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        (complete.clone(), complete)
    } else {
        (
            regular_graph(config, rng, "setup/initial_graph")?,
            regular_graph(config, rng, "setup/repair_graph")?,
        )
    };
    let priors: Vec<f64> = boost
        .iter()
        .map(|&b| 0.5 + if b { config.delta } else { -config.delta })
        .collect();
    Ok(ReputationWorld {
        config: config.clone(),
        boost,
        graph: initial_graph,
        repair_graph,
        beliefs: vec![priors.clone(); n],
        public: priors,
        wealth: vec![10; n],
        step: 0,
        trace: vec![],
        preintervention_graph: None,
        surviving_edges: None,
        preintervention_wealth: None,
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Flow {
    pub degree: f64,
    pub inherited: f64,
    pub added: f64,
    pub selection: f64,
    pub total: f64,
}

/// Pre-event expectation, averaged over a uniformly scheduled allocator.
pub fn flow(world: &ReputationWorld) -> Result<Flow, String> {
    flow_from(
        &world.config,
        &world.boost,
        &world.graph,
        &world.beliefs,
        world.surviving_edges.as_ref(),
    )
}

/// Shared mathematical implementation for serializable worlds and compact
/// policy observations. This preserves the exact addition/summation order
/// used by V0 and V1's hash-verified event actions.
fn flow_from(
    config: &Config,
    boost: &[bool],
    graph: &[Vec<usize>],
    beliefs: &[Vec<f64>],
    surviving_edges: Option<&Vec<Vec<usize>>>,
) -> Result<Flow, String> {
    let n = config.actors;
    let d = config.degree;
    let mut deg = 0f64;
    let mut inherited = 0f64;
    let mut selected = 0f64;
    for j in 0..n {
        let row = &graph[j];
        let p = selection_probs_fields(config, graph, beliefs, j)?;
        for (k, &i) in row.iter().enumerate() {
            let sign = if boost[i] { 1.0 } else { -1.0 };
            deg += sign / (n * d) as f64;
            if surviving_edges.is_none_or(|base| base[j].contains(&i)) {
                inherited += sign / (n * d) as f64;
            }
            selected += sign * (p[k] - 1.0 / d as f64) / n as f64;
        }
    }
    let added = deg - inherited;
    Ok(Flow {
        degree: deg,
        inherited,
        added,
        selection: selected,
        total: deg + selected,
    })
}

fn softmax(values: &[f64], temperature: f64) -> Result<Vec<f64>, String> {
    if values.is_empty() {
        return Err("empty choice set".into());
    }
    let weighted: Vec<_> = values.iter().map(|x| x * temperature).collect();
    let max = weighted.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let weights: Vec<_> = weighted.iter().map(|x| (x - max).exp()).collect();
    let total: f64 = weights.iter().sum();
    if !total.is_finite() || total <= 0.0 {
        return Err("invalid selection weights".into());
    }
    Ok(weights.iter().map(|x| x / total).collect())
}
fn selection_probs_fields(
    config: &Config,
    graph: &[Vec<usize>],
    beliefs: &[Vec<f64>],
    j: usize,
) -> Result<Vec<f64>, String> {
    let values = graph
        .get(j)
        .ok_or("invalid allocator")?
        .iter()
        .map(|&i| beliefs[j][i])
        .collect::<Vec<_>>();
    let mut p = softmax(&values, config.beta)?;
    let d = p.len() as f64;
    for x in &mut p {
        *x = config.epsilon / d + (1.0 - config.epsilon) * (*x);
    }
    Ok(p)
}
fn uniform(rng: &mut Streams, name: &str) -> Result<f64, String> {
    Ok((rng.draw_u64(name)? >> 11) as f64 * (1.0 / ((1u64 << 53) as f64)))
}
fn choose(p: &[f64], rng: &mut Streams, name: &str) -> Result<usize, String> {
    let mut u = uniform(rng, name)?;
    for (i, &prob) in p.iter().enumerate() {
        if u < prob {
            return Ok(i);
        }
        u -= prob;
    }
    // Rounding can leave an ~ulp remainder after a probability sum.
    Ok(p.len() - 1)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rewire {
    pub allocator: usize,
    pub removed: usize,
    pub added: usize,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub recipient: usize,
    pub success: bool,
    pub rewire: Option<Rewire>,
    pub pre_flow: Flow,
    /// For the annealed null, record the freshly sampled ephemeral whole graph.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annealed_graph: Option<Vec<Vec<usize>>>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepTrace {
    pub step: u64,
    pub allocator: usize,
    pub recipient: usize,
    pub sign: i8,
    pub expectation: Flow,
    pub rewire: Option<Rewire>,
    /// Post-event checkpoint links still continuously present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surviving_count: Option<usize>,
}

impl StepTrace {
    /// The record of the opportunity that produced `post`, allocated by
    /// `allocator` with `action`. Shared by in-world traces and journal
    /// replay, so both trajectories are built by the same rule.
    pub fn after(post: &ReputationWorld, allocator: usize, action: &Action) -> Self {
        let r = action.recipient;
        Self {
            step: post.step,
            allocator,
            recipient: r,
            sign: if post.boost[r] { 1 } else { -1 },
            // Keep the recorded value, not a recomputed one: `f64::exp`
            // precision varies by platform, and replay hashes this state.
            expectation: action.pre_flow.clone(),
            rewire: action.rewire.clone(),
            surviving_count: if post.config.survival_accounting {
                post.surviving_edges
                    .as_ref()
                    .map(|rows| rows.iter().map(Vec::len).sum())
            } else {
                None
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReputationWorld {
    pub config: Config,
    pub boost: Vec<bool>,
    pub graph: Vec<Vec<usize>>,
    pub repair_graph: Vec<Vec<usize>>,
    pub beliefs: Vec<Vec<f64>>,
    pub public: Vec<f64>,
    pub wealth: Vec<u64>,
    pub step: u64,
    pub trace: Vec<StepTrace>,
    pub preintervention_graph: Option<Vec<Vec<usize>>>,
    /// Directed checkpoint edges that have survived **continuously** since τ.
    /// Once deleted, an edge is never returned to this set even if re-added.
    pub surviving_edges: Option<Vec<Vec<usize>>>,
    pub preintervention_wealth: Option<Vec<u64>>,
}
impl ReputationWorld {
    pub fn verify(&self) -> Result<(), String> {
        self.config.validate()?;
        let n = self.config.actors;
        if self.boost.len() != n
            || self.boost.iter().filter(|x| **x).count() != n / 2
            || self.beliefs.len() != n
            || self.public.len() != n
            || self.wealth.len() != n
            || self.beliefs.iter().any(|row| {
                row.len() != n
                    || row
                        .iter()
                        .any(|x| !x.is_finite() || !(0.0..=1.0).contains(x))
            })
            || self
                .public
                .iter()
                .any(|x| !x.is_finite() || !(0.0..=1.0).contains(x))
        {
            return Err("belief, label or state invariant failed".into());
        }
        check_rows(&self.graph, n, self.config.degree)?;
        validate_graph(&self.repair_graph, n, self.config.degree)?;
        if self.preintervention_graph.is_some() != self.preintervention_wealth.is_some()
            || self.preintervention_graph.is_some() != self.surviving_edges.is_some()
        {
            return Err("partial fork boundary".into());
        }
        if let Some(ref base) = self.preintervention_graph {
            check_rows(base, n, self.config.degree)?;
            let surviving = self
                .surviving_edges
                .as_ref()
                .ok_or("missing link survival audit")?;
            if surviving.len() != n {
                return Err("wrong link survival row count".into());
            }
            for (j, row) in surviving.iter().enumerate() {
                if row
                    .iter()
                    .any(|i| !base[j].contains(i) || !self.graph[j].contains(i))
                {
                    return Err("invalid uninterrupted-link survival attribution".into());
                }
            }
        }
        let sum: u128 = self.wealth.iter().map(|x| *x as u128).sum();
        if sum != 10u128 * n as u128 + self.step as u128 {
            return Err("wealth conservation failed".into());
        }
        if self.config.journal_trace {
            if !self.trace.is_empty() {
                return Err("journal-trace worlds keep no trace".into());
            }
        } else if self.trace.len() != self.step as usize {
            return Err("one trace required per opportunity".into());
        }
        Ok(())
    }
    /// The state an action is decided and checked against: this world or,
    /// under the annealed control, this world with the action's fresh graph.
    pub fn pre_event_state(&self, a: &Action) -> Result<Self, String> {
        let mut pre = self.clone();
        if let Some(ref new_graph) = a.annealed_graph {
            if !pre.config.annealed {
                return Err("unexpected annealed graph".into());
            }
            validate_graph(new_graph, pre.config.actors, pre.config.degree)?;
            pre.graph = new_graph.clone();
            if let Some(ref mut surviving) = pre.surviving_edges {
                for (row, links) in surviving.iter_mut().zip(&pre.graph) {
                    row.retain(|i| links.contains(i));
                }
            }
        } else if self.config.annealed {
            return Err("annealed policy must provide a fresh graph".into());
        }
        Ok(pre)
    }
    pub fn intervene(&mut self, arm: Arm) -> Result<(), String> {
        if self.preintervention_graph.is_some() {
            return Err("intervention already applied".into());
        }
        self.preintervention_graph = Some(self.graph.clone());
        self.surviving_edges = Some(self.graph.clone());
        self.preintervention_wealth = Some(self.wealth.clone());
        if matches!(arm, Arm::Information | Arm::Both) {
            for b in &mut self.beliefs {
                b.fill(0.5);
            }
            self.public.fill(0.5);
        }
        if matches!(arm, Arm::Structure | Arm::Both) {
            self.graph = self.repair_graph.clone();
            // Edges not retained across this intervention have already ceased
            // to be uninterrupted checkpoint edges.
            if let Some(ref mut surviving) = self.surviving_edges {
                for (j, row) in surviving.iter_mut().enumerate() {
                    row.retain(|i| self.graph[j].contains(i));
                }
            }
        }
        self.verify()
    }
}
/// Transient policy input; the expensive world trace and replay journal are
/// deliberately NOT observations. Each field is sufficient for exactly the
/// same action probabilities and rewiring decisions as the full world.
#[derive(Clone, Debug)]
pub struct ReputationObservation {
    pub config: Config,
    pub boost: Vec<bool>,
    pub graph: Vec<Vec<usize>>,
    pub beliefs: Vec<Vec<f64>>,
    pub public: Vec<f64>,
    pub step: u64,
    pub surviving_edges: Option<Vec<Vec<usize>>>,
}

impl HiyalWorld for ReputationWorld {
    type Action = Action;
    // EffectPolicy receives the world directly; the empty effect-request
    // path does not need a cloned, ever-growing step trace as an observation.
    type Observation = ReputationObservation;
    fn observe(&self, _: AgentId) -> Result<Self::Observation, String> {
        Ok(ReputationObservation {
            config: self.config.clone(),
            boost: self.boost.clone(),
            graph: self.graph.clone(),
            beliefs: self.beliefs.clone(),
            public: self.public.clone(),
            step: self.step,
            surviving_edges: self.surviving_edges.clone(),
        })
    }
    fn apply(&mut self, actor: AgentId, a: Action) -> Result<(), String> {
        let j = usize::try_from(actor.0).map_err(|e| e.to_string())?;
        let n = self.config.actors;
        if j >= n {
            return Err("allocator out of range".into());
        }
        let pre = self.pre_event_state(&a)?;
        if !pre.graph[j].contains(&a.recipient) {
            return Err("recipient not eligible".into());
        }
        let now = flow(&pre)?;
        for (v, x) in [
            (now.degree, a.pre_flow.degree),
            (now.inherited, a.pre_flow.inherited),
            (now.added, a.pre_flow.added),
            (now.selection, a.pre_flow.selection),
            (now.total, a.pre_flow.total),
        ] {
            if (v - x).abs() > 1e-9 {
                return Err("conditional exposure reference mismatch".into());
            }
        }
        let step = self.step.checked_add(1).ok_or("step overflow")?;
        let requires = self.config.removal != Removal::None && step % self.config.rewire_every == 0;
        if requires != a.rewire.is_some() {
            return Err("incorrect rewiring event timing".into());
        }
        // Stage all changes atomically, as Hiyal also stages its own world.
        let mut w = pre;
        w.wealth[a.recipient] = w.wealth[a.recipient]
            .checked_add(1)
            .ok_or("wealth overflow")?;
        let r = a.recipient;
        w.beliefs[j][r] = (1.0 - w.config.learning) * w.beliefs[j][r]
            + w.config.learning * f64::from(u8::from(a.success));
        w.public[r] = (1.0 - w.config.public_learning) * w.public[r]
            + w.config.public_learning * f64::from(u8::from(a.success));
        if let Some(ref change) = a.rewire {
            if change.allocator >= n
                || change.added >= n
                || change.removed >= n
                || change.added == change.removed
                || change.added == change.allocator
                || !w.graph[change.allocator].contains(&change.removed)
                || w.graph[change.allocator].contains(&change.added)
            {
                return Err("invalid replacement edge".into());
            }
            if w.config.removal == Removal::LowestBelief {
                let low = w.graph[change.allocator]
                    .iter()
                    .map(|&i| w.beliefs[change.allocator][i])
                    .fold(f64::INFINITY, f64::min);
                if (w.beliefs[change.allocator][change.removed] - low).abs() > 1e-12 {
                    return Err("rewiring did not remove lowest-belief neighbor".into());
                }
            }
            let idx = w.graph[change.allocator]
                .iter()
                .position(|&x| x == change.removed)
                .ok_or("missing edge")?;
            w.graph[change.allocator][idx] = change.added;
            if let Some(ref mut surviving) = w.surviving_edges {
                surviving[change.allocator].retain(|&i| i != change.removed);
            }
        }
        w.step = step;
        if !w.config.journal_trace {
            let entry = StepTrace::after(&w, j, &a);
            w.trace.push(entry);
        }
        w.verify()?;
        *self = w;
        Ok(())
    }
}

pub struct ReputationPolicy;
impl EffectPolicy<ReputationWorld> for ReputationPolicy {
    fn decide(
        &self,
        _: AgentId,
        _: &ReputationObservation,
        _: &[Receipt],
    ) -> Result<Action, String> {
        Err("randomized reputation decisions require trial-owned streams".into())
    }
    fn decide_with_streams(
        &self,
        actor: AgentId,
        world: &ReputationObservation,
        _: &[Receipt],
        rng: &mut Streams,
    ) -> Result<Action, String> {
        let j = usize::try_from(actor.0).map_err(|e| e.to_string())?;
        if j >= world.config.actors {
            return Err("unknown allocator".into());
        }
        // Borrow the original world in the common persistent-graph case;
        // only the annealed control needs an owned, ephemeral replacement.
        let (pre, annealed_graph): (Cow<'_, ReputationObservation>, Option<Vec<Vec<usize>>>) =
            if world.config.annealed {
                let stream = format!("agent/{j}/annealed");
                let edges =
                    annealed_regular_graph(world.config.actors, world.config.degree, rng, &stream)?;
                let mut changed = world.clone();
                changed.graph = edges.clone();
                if let Some(ref mut surviving) = changed.surviving_edges {
                    for (row, links) in surviving.iter_mut().zip(&changed.graph) {
                        row.retain(|i| links.contains(i));
                    }
                }
                (Cow::Owned(changed), Some(edges))
            } else {
                (Cow::Borrowed(world), None)
            };
        let pre_flow = flow_from(
            &pre.config,
            &pre.boost,
            &pre.graph,
            &pre.beliefs,
            pre.surviving_edges.as_ref(),
        )?;
        let sel = format!("agent/{j}/selection");
        let index = choose(
            &selection_probs_fields(&pre.config, &pre.graph, &pre.beliefs, j)?,
            rng,
            &sel,
        )?;
        let recipient = pre.graph[j][index];
        let sig = format!("agent/{j}/performance");
        let success = rng.draw_u64(&sig)? & 1 == 0;
        let step = world.step.checked_add(1).ok_or("step overflow")?;
        let rewire =
            if world.config.removal != Removal::None && step % world.config.rewire_every == 0 {
                let stream = format!("agent/{j}/rewiring");
                let k = rng.uniform_index(&stream, world.config.actors)?;
                let mut beliefs = world.beliefs[k].clone();
                let mut public = world.public.clone();
                if k == j {
                    beliefs[recipient] = (1.0 - world.config.learning) * beliefs[recipient]
                        + world.config.learning * f64::from(u8::from(success));
                }
                public[recipient] = (1.0 - world.config.public_learning) * public[recipient]
                    + world.config.public_learning * f64::from(u8::from(success));
                let row = &world.graph[k];
                let rem_index = match world.config.removal {
                    Removal::None => return Err("impossible rewiring state".into()),
                    Removal::Uniform => rng.uniform_index(&stream, row.len())?,
                    Removal::LowestBelief => {
                        let min = row
                            .iter()
                            .map(|&i| beliefs[i])
                            .fold(f64::INFINITY, f64::min);
                        let tied: Vec<_> = row
                            .iter()
                            .enumerate()
                            .filter(|(_, i)| (beliefs[**i] - min).abs() <= 1e-12)
                            .map(|(ix, _)| ix)
                            .collect();
                        tied[rng.uniform_index(&stream, tied.len())?]
                    }
                };
                let removed = row[rem_index];
                let candidates: Vec<_> = (0..world.config.actors)
                    .filter(|&x| x != k && x != removed && !row.contains(&x))
                    .collect();
                if candidates.is_empty() {
                    return Err("no alternative graph target".into());
                }
                let probs = softmax(
                    &candidates.iter().map(|&i| public[i]).collect::<Vec<_>>(),
                    world.config.kappa,
                )?;
                let added = candidates[choose(&probs, rng, &stream)?];
                Some(Rewire {
                    allocator: k,
                    removed,
                    added,
                })
            } else {
                None
            };
        Ok(Action {
            recipient,
            success,
            rewire,
            pre_flow,
            annealed_graph,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Arm {
    Sham,
    Information,
    Structure,
    Both,
}
impl Arm {
    pub const ALL: [Self; 4] = [Self::Sham, Self::Information, Self::Structure, Self::Both];
    pub fn name(self) -> &'static str {
        match self {
            Self::Sham => "sham",
            Self::Information => "information",
            Self::Structure => "structure",
            Self::Both => "both",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Window {
    pub from: u64,
    pub through: u64,
    pub opportunity_gap: f64,
    pub degree: f64,
    pub inherited: f64,
    pub added: f64,
    pub selection: f64,
    pub innovation: f64,
    pub rewire_events: usize,
    /// Mean fraction of checkpoint edges that have never been deleted,
    /// when all events in this window recorded uninterrupted-survival state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mean_surviving_edge_fraction: Option<f64>,
}
pub fn window(w: &ReputationWorld, from: u64, through: u64) -> Result<Window, String> {
    if through > w.step {
        return Err("invalid measurement window".into());
    }
    window_of(&w.config, &w.trace, from, through)
}

/// `window` over an explicit trajectory, e.g. one rebuilt from a journal.
pub fn window_of(
    config: &Config,
    trace: &[StepTrace],
    from: u64,
    through: u64,
) -> Result<Window, String> {
    if from == 0 || through < from {
        return Err("invalid measurement window".into());
    }
    let rows: Vec<_> = trace
        .iter()
        .filter(|x| (from..=through).contains(&x.step))
        .collect();
    if rows.len() != usize::try_from(through - from + 1).map_err(|e| e.to_string())? {
        return Err("missing window events".into());
    }
    let mean =
        |f: &dyn Fn(&StepTrace) -> f64| rows.iter().map(|t| f(t)).sum::<f64>() / rows.len() as f64;
    let gap = mean(&|x| x.sign as f64);
    let degree = mean(&|x| x.expectation.degree);
    let inherited = mean(&|x| x.expectation.inherited);
    let added = mean(&|x| x.expectation.added);
    let selection = mean(&|x| x.expectation.selection);
    Ok(Window {
        from,
        through,
        opportunity_gap: gap,
        degree,
        inherited,
        added,
        selection,
        innovation: gap - degree - selection,
        rewire_events: rows.iter().filter(|x| x.rewire.is_some()).count(),
        mean_surviving_edge_fraction: if rows.iter().all(|x| x.surviving_count.is_some()) {
            let denominator = (config.actors * config.degree) as f64;
            Some(
                rows.iter()
                    .map(|x| x.surviving_count.unwrap_or(0) as f64 / denominator)
                    .sum::<f64>()
                    / rows.len() as f64,
            )
        } else {
            None
        },
    })
}
