//! Experiment 003 A2 v0.2.2: EXPLORATORY, not an identified-family estimator.
//! Controlling contract: docs/research/r06/a2-v022-joint-law-hierarchy-20261025.md
//! Inherited kernels: docs/research/r06/a2-v021-six-kernel-design-20261025.md
//! This module uses Hiyal's versioned, named ChaCha20 streams and event journal.
//! Hiyal's internal world/observations include latent parameters for replay.
//! Never pass those internal artifacts to a scientific classifier: use PublicRecord only.
use hiyal::{
    core::{AgentId, World},
    experiment::EffectPolicy,
    rng::Streams,
    runtime::Receipt,
};
use serde::{Deserialize, Serialize};

pub const KIND: &str = "social-emergence-lab.experiment003-a2-v0.2.2.exploratory";
pub const MATCHES: [[[usize; 2]; 2]; 3] = [[[0, 1], [2, 3]], [[0, 2], [1, 3]], [[0, 3], [1, 2]]];
pub const VERSION: &str = "Experiment003-A2-v0.2.2";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Family {
    M,
    N,
    C,
    S,
    L,
    R,
}
impl Family {
    pub const ALL: [Self; 6] = [Self::M, Self::N, Self::C, Self::S, Self::L, Self::R];
    pub fn name(self) -> &'static str {
        match self {
            Self::M => "M",
            Self::N => "N",
            Self::C => "C",
            Self::S => "S",
            Self::L => "L",
            Self::R => "R",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Policy {
    A,
    A0,
}
impl Policy {
    pub const ALL: [Self; 2] = [Self::A, Self::A0];
    pub fn name(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::A0 => "A0",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Regime {
    Random,
    Choice,
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Tuple {
    M {
        eta: f64,
    },
    N {
        eta: f64,
        nu: f64,
    },
    C {
        eta: f64,
        kappa: f64,
    },
    S {
        eta: f64,
        tau: i8,
    },
    L {
        eta: f64,
        kappa: f64,
        alpha: f64,
        zeta: f64,
    },
    R {
        eta: f64,
        alpha: f64,
        lambda: f64,
    },
}
impl Tuple {
    pub fn family(self) -> Family {
        match self {
            Self::M { .. } => Family::M,
            Self::N { .. } => Family::N,
            Self::C { .. } => Family::C,
            Self::S { .. } => Family::S,
            Self::L { .. } => Family::L,
            Self::R { .. } => Family::R,
        }
    }
    fn eta(self) -> f64 {
        match self {
            Self::M { eta }
            | Self::N { eta, .. }
            | Self::C { eta, .. }
            | Self::S { eta, .. }
            | Self::L { eta, .. }
            | Self::R { eta, .. } => eta,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Globals {
    pub beta: f64,
    pub epsilon: f64,
    pub rho: f64,
    pub delta: f64,
    pub omega: f64,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Atom {
    pub weight: f64,
    pub tuple: Tuple,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Design {
    pub version: String,
    pub family: Family,
    pub policy: Policy,
    pub globals: Globals,
    /// ≤4 joint tuple atoms. Each of four actors is sampled iid once per society.
    pub atoms: Vec<Atom>,
    pub periods: usize,
}
fn bounded(x: f64, lo: f64, hi: f64) -> bool {
    x.is_finite() && x >= lo && x <= hi
}
impl Design {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != VERSION || !(1..=8).contains(&self.periods) {
            return Err("wrong version or exploratory horizon".into());
        }
        let g = self.globals;
        if !bounded(g.beta, 0.1, 10.0)
            || !bounded(g.epsilon, 0.0, 0.5)
            || !bounded(g.rho, 0.1, 10.0)
            || (self.family == Family::S
                && (!bounded(g.delta, 0.0, 2.0) || !bounded(g.omega, 0.0, 2.0)))
        {
            return Err("global parameters outside v0.2.2 domain".into());
        }
        if self.atoms.is_empty() || self.atoms.len() > 4 {
            return Err("H4 requires 1..=4 atoms".into());
        }
        let mut mass = 0.0;
        for a in &self.atoms {
            if a.tuple.family() != self.family || !bounded(a.weight, 0.0, 1.0) {
                return Err("invalid family atom or weight".into());
            }
            mass += a.weight;
            let tau = match a.tuple {
                Tuple::S { tau, .. } => tau as f64,
                _ => 0.0,
            };
            let c = 1.0 / 3.0
                + a.tuple.eta()
                + if self.family == Family::S {
                    g.omega * tau
                } else {
                    0.0
                };
            if !bounded(c, -3.0 - 1e-12, 3.0 + 1e-12) {
                return Err("effective appraisal outside [-3,3]".into());
            }
            let ok = match a.tuple {
                Tuple::M { .. } => true,
                Tuple::N { nu, .. } => bounded(nu, -2.0, 2.0),
                Tuple::C { kappa, .. } => bounded(kappa, -4.0, 4.0),
                Tuple::S { tau, .. } => tau == 1 || tau == -1,
                Tuple::L {
                    kappa, alpha, zeta, ..
                } => {
                    bounded(kappa, -4.0, 4.0) && bounded(alpha, 0.0, 1.0) && bounded(zeta, 0.0, 1.0)
                }
                Tuple::R { alpha, lambda, .. } => {
                    bounded(alpha, 0.0, 1.0) && bounded(lambda, 0.0, 1.0)
                }
            };
            if !ok {
                return Err("actor tuple outside v0.2.2 domain".into());
            }
        }
        if (mass - 1.0).abs() > 1e-12 {
            return Err("H4 atom weights must sum to one".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateState {
    pub previous: Option<[u8; 4]>,
    pub belief: [f64; 4],
    /// [actor][action][fine bit].
    pub q: [[[f64; 2]; 2]; 4],
    pub aspiration: [f64; 4],
}
impl Default for PrivateState {
    fn default() -> Self {
        Self {
            previous: None,
            belief: [0.5; 4],
            q: [[[0.0; 2]; 2]; 4],
            aspiration: [0.0; 4],
        }
    }
}
/// The ONLY classifier-facing schema. Policy, family, tuples, RNG positions and
/// counterfactual futures must not appear in a record.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicRecord {
    pub period: usize,
    pub regime: Regime,
    pub matching: usize,
    pub fine_bits: [u8; 2],
    pub actions: [u8; 4],
    pub payoffs: [f64; 4],
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Society {
    pub design: Design,
    pub actors: [Tuple; 4],
    pub private: PrivateState,
    pub history: Vec<PublicRecord>,
}
fn u01(rng: &mut Streams, label: &str) -> Result<f64, String> {
    Ok((rng.draw_u64(label)? >> 11) as f64 / ((1u64 << 53) as f64))
}
fn sigmoid(x: f64) -> f64 {
    if x >= 0.0 {
        1.0 / (1.0 + (-x).exp())
    } else {
        let e = x.exp();
        e / (1.0 + e)
    }
}
fn draw_cat(rng: &mut Streams, stream: &str, mass: &[f64]) -> Result<usize, String> {
    let mut draw = u01(rng, stream)?;
    for (index, &p) in mass.iter().enumerate() {
        if p < 0.0 || !p.is_finite() {
            return Err("invalid categorical weights".into());
        }
        if draw < p {
            return Ok(index);
        }
        draw -= p;
    }
    // Rounding can leave a tiny positive remainder after normalized weights.
    if draw <= 1e-12 {
        return Ok(mass.len() - 1);
    }
    Err("categorical weights not normalized".into())
}
pub fn partner(mu: usize, i: usize) -> usize {
    for d in MATCHES[mu] {
        if d[0] == i {
            return d[1];
        }
        if d[1] == i {
            return d[0];
        }
    }
    unreachable!("invalid matching")
}
fn dyad(mu: usize, i: usize) -> usize {
    if MATCHES[mu][0].contains(&i) { 0 } else { 1 }
}
fn f(bit: u8) -> f64 {
    0.5 * (bit as f64)
}
impl Society {
    pub fn setup(design: Design, rng: &mut Streams) -> Result<Self, String> {
        design.validate()?;
        let weights: Vec<f64> = design.atoms.iter().map(|a| a.weight).collect();
        let mut actors = [design.atoms[0].tuple; 4];
        for (i, t) in actors.iter_mut().enumerate() {
            *t = design.atoms[draw_cat(rng, &format!("setup/actor/{i}"), &weights)?].tuple;
        }
        Ok(Self {
            design,
            actors,
            private: PrivateState::default(),
            history: vec![],
        })
    }
    pub fn new_fixed(design: Design, actors: [Tuple; 4]) -> Result<Self, String> {
        design.validate()?;
        if actors.iter().any(|t| t.family() != design.family) {
            return Err("fixed F family mismatch".into());
        }
        // F tests may use distinct fixed actors, but never out-of-domain tuples.
        for actor in actors {
            let mut candidate = design.clone();
            candidate.atoms = vec![Atom {
                weight: 1.0,
                tuple: actor,
            }];
            candidate.validate()?;
        }
        Ok(Self {
            design,
            actors,
            private: PrivateState::default(),
            history: vec![],
        })
    }
    fn appraisal(&self, i: usize) -> f64 {
        1.0 / 3.0
            + self.actors[i].eta()
            + match self.actors[i] {
                Tuple::S { tau, .. } => self.design.globals.omega * tau as f64,
                _ => 0.0,
            }
    }
    pub fn broker(&self, regime: Regime) -> [f64; 3] {
        if regime == Regime::Random || self.private.previous.is_none() {
            return [1.0 / 3.0; 3];
        }
        let prev = self.private.previous.unwrap();
        let mut score = [0.0; 3];
        for (m, pairs) in MATCHES.iter().enumerate() {
            for &[i, j] in pairs {
                let vi = sigmoid(self.appraisal(i) * (2.0 * prev[j] as f64 - 1.0));
                let vj = sigmoid(self.appraisal(j) * (2.0 * prev[i] as f64 - 1.0));
                score[m] += vi.min(vj);
            }
            score[m] *= self.design.globals.rho;
        }
        let max = score.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let e = score.map(|s| (s - max).exp());
        let total: f64 = e.iter().sum();
        e.map(|v| v / total)
    }
    pub fn offset(&self, i: usize, mu: usize, fine_bit: u8) -> f64 {
        match self.actors[i] {
            Tuple::M { .. } => 0.0,
            Tuple::N { nu, .. } => nu,
            Tuple::C { kappa, .. } => {
                kappa
                    * self
                        .private
                        .previous
                        .map_or(0.5, |a| a[partner(mu, i)] as f64)
            }
            Tuple::S { tau, .. } => self.design.globals.delta * tau as f64,
            Tuple::L { kappa, .. } => kappa * self.private.belief[i],
            Tuple::R { .. } => {
                self.private.q[i][1][fine_bit as usize] - self.private.q[i][0][fine_bit as usize]
            }
        }
    }
    pub fn cooperation(&self, i: usize, mu: usize, fine_bit: u8) -> f64 {
        let g = self.design.globals;
        g.epsilon / 2.0
            + (1.0 - g.epsilon)
                * sigmoid(g.beta * (f(fine_bit) - 1.0 + self.offset(i, mu, fine_bit)))
    }
    pub fn joint_action_probability(&self, mu: usize, fines: [u8; 2], a: [u8; 4]) -> f64 {
        (0..4)
            .map(|i| {
                let p = self.cooperation(i, mu, fines[dyad(mu, i)]);
                if a[i] == 1 { p } else { 1.0 - p }
            })
            .product()
    }
    pub fn payoff_for(mu: usize, fines: [u8; 2], actions: [u8; 4]) -> [f64; 4] {
        std::array::from_fn(|i| {
            let j = partner(mu, i);
            let ai = actions[i] as f64;
            2.0 * actions[j] as f64 - ai - f(fines[dyad(mu, i)]) * (1.0 - ai)
        })
    }
    fn transition(&mut self, rec: PublicRecord) {
        // Post-period update ONLY. Current action law cannot see its own outcome.
        for i in 0..4 {
            let j = partner(rec.matching, i);
            let bit = rec.fine_bits[dyad(rec.matching, i)] as usize;
            match self.actors[i] {
                Tuple::L { alpha, zeta, .. } => {
                    let w = 1.0 - zeta * if bit == 1 { 1.0 } else { 0.0 };
                    self.private.belief[i] = (1.0 - alpha * w) * self.private.belief[i]
                        + alpha * w * rec.actions[j] as f64;
                }
                Tuple::R { alpha, lambda, .. } => {
                    let ai = rec.actions[i] as usize;
                    let u = rec.payoffs[i];
                    let old_asp = self.private.aspiration[i];
                    self.private.q[i][ai][bit] =
                        (self.private.q[i][ai][bit] + alpha * (u - old_asp)).clamp(-10.0, 10.0);
                    self.private.aspiration[i] = (1.0 - lambda) * old_asp + lambda * u;
                }
                _ => {}
            }
        }
        self.private.previous = Some(rec.actions);
        self.history.push(rec);
    }
}
impl World for Society {
    type Action = PublicRecord;
    type Observation = Society; // INTERNAL engine observation, NEVER exported for classifier.
    fn observe(&self, actor: AgentId) -> Result<Self::Observation, String> {
        if actor.0 != 0 {
            return Err("one Hiyal orchestrator actor only".into());
        }
        Ok(self.clone())
    }
    fn apply(&mut self, actor: AgentId, action: PublicRecord) -> Result<(), String> {
        if actor.0 != 0
            || self.history.len() >= self.design.periods
            || action.period != self.history.len() + 1
            || action.matching >= 3
            || action.fine_bits.iter().any(|&b| b > 1)
            || action.actions.iter().any(|&a| a > 1)
        {
            return Err("malformed period action".into());
        }
        if self.design.policy == Policy::A0
            && (action.regime != Regime::Random || action.fine_bits != [0, 0])
        {
            return Err("A0 cannot contain choice/fine intervention".into());
        }
        let expected = Self::payoff_for(action.matching, action.fine_bits, action.actions);
        if expected != action.payoffs || self.broker(action.regime)[action.matching] <= 0.0 {
            return Err("payoff mismatch or unsupported matching".into());
        }
        self.transition(action);
        Ok(())
    }
}
pub struct Generator;
impl EffectPolicy<Society> for Generator {
    fn decide(
        &self,
        _actor: AgentId,
        _obs: &Society,
        _effects: &[Receipt],
    ) -> Result<PublicRecord, String> {
        Err("Experiment003 requires Hiyal named streams".into())
    }
    fn decide_with_streams(
        &self,
        actor: AgentId,
        obs: &Society,
        _effects: &[Receipt],
        rng: &mut Streams,
    ) -> Result<PublicRecord, String> {
        if actor.0 != 0 || obs.history.len() >= obs.design.periods {
            return Err("outside exploratory horizon".into());
        }
        let t = obs.history.len() + 1;
        let prefix = format!("agent/0/period/{t}");
        let regime =
            if obs.design.policy == Policy::A && u01(rng, &format!("{prefix}/regime"))? < 0.5 {
                Regime::Choice
            } else {
                Regime::Random
            };
        let matching = draw_cat(rng, &format!("{prefix}/broker"), &obs.broker(regime))?;
        let mut fine_bits = [0; 2];
        if obs.design.policy == Policy::A {
            for (d, bit) in fine_bits.iter_mut().enumerate() {
                *bit = (u01(rng, &format!("{prefix}/fine/{d}"))? < 0.5) as u8;
            }
        }
        let mut actions = [0; 4];
        for (i, act) in actions.iter_mut().enumerate() {
            let p = obs.cooperation(i, matching, fine_bits[dyad(matching, i)]);
            *act = (u01(rng, &format!("{prefix}/action/{i}"))? < p) as u8;
        }
        Ok(PublicRecord {
            period: t,
            regime,
            matching,
            fine_bits,
            actions,
            payoffs: Society::payoff_for(matching, fine_bits, actions),
        })
    }
}

/// Non-confirmatory two-atom examples; chosen before running this engineering pilot.
/// They are not fitted priors and do not represent human parameter distributions.
pub fn pilot_design(family: Family, policy: Policy) -> Design {
    let g = Globals {
        beta: 1.7,
        epsilon: 0.08,
        rho: 2.3,
        delta: 0.7,
        omega: 0.4,
    };
    let eta = [-0.25, 0.1];
    let atoms = eta
        .iter()
        .enumerate()
        .map(|(i, &eta)| {
            let t = if i == 0 { 1 } else { -1 };
            let tuple = match family {
                Family::M => Tuple::M { eta },
                Family::N => Tuple::N {
                    eta,
                    nu: if i == 0 { 0.7 } else { -0.7 },
                },
                Family::C => Tuple::C {
                    eta,
                    kappa: if i == 0 { 1.25 } else { -1.4 },
                },
                Family::S => Tuple::S { eta, tau: t },
                Family::L => Tuple::L {
                    eta,
                    kappa: if i == 0 { 1.4 } else { -1.4 },
                    alpha: if i == 0 { 0.6 } else { 0.4 },
                    zeta: if i == 0 { 0.5 } else { 1.0 },
                },
                Family::R => Tuple::R {
                    eta,
                    alpha: if i == 0 { 0.7 } else { 0.2 },
                    lambda: if i == 0 { 0.5 } else { 0.25 },
                },
            };
            Atom {
                weight: if i == 0 { 0.4 } else { 0.6 },
                tuple,
            }
        })
        .collect();
    Design {
        version: VERSION.into(),
        family,
        policy,
        globals: g,
        atoms,
        periods: 4,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixed(fam: Family, tuples: [Tuple; 4]) -> Society {
        let mut d = pilot_design(fam, Policy::A);
        d.atoms = vec![Atom {
            weight: 1.0,
            tuple: tuples[0],
        }];
        Society::new_fixed(d, tuples).unwrap()
    }
    #[test]
    fn payoff_fines_and_t1_neutral() {
        assert_eq!(
            Society::payoff_for(0, [0, 1], [1, 0, 1, 1]),
            [-1.0, 2.0, 1.0, 1.0]
        );
        let s = fixed(
            Family::C,
            [Tuple::C {
                eta: 0.0,
                kappa: 2.0,
            }; 4],
        );
        assert_eq!(s.offset(0, 0, 0), 1.0);
        for z in [Regime::Choice, Regime::Random] {
            for p in s.broker(z) {
                assert!((p - 1.0 / 3.0).abs() < 1e-14);
            }
        }
        for mu in 0..3 {
            let mass: f64 = (0..16)
                .map(|x| {
                    s.joint_action_probability(
                        mu,
                        [0, 1],
                        std::array::from_fn(|i| ((x >> i) & 1) as u8),
                    )
                })
                .sum();
            assert!((mass - 1.0).abs() < 1e-12);
        }
    }
    #[test]
    fn rematching_c_is_not_frozen_l() {
        let mut c = fixed(
            Family::C,
            [Tuple::C {
                eta: 0.0,
                kappa: 1.0,
            }; 4],
        );
        let mut l = fixed(
            Family::L,
            [Tuple::L {
                eta: 0.0,
                kappa: 1.0,
                alpha: 1.0,
                zeta: 0.0,
            }; 4],
        );
        let actions = [1, 1, 0, 0];
        let rec = PublicRecord {
            period: 1,
            regime: Regime::Random,
            matching: 0,
            fine_bits: [0, 0],
            actions,
            payoffs: Society::payoff_for(0, [0, 0], actions),
        };
        c.apply(AgentId(0), rec).unwrap();
        l.apply(AgentId(0), rec).unwrap();
        assert_eq!(c.offset(0, 1, 0), 0.0); // new partner actor 2's last action
        assert_eq!(l.offset(0, 1, 0), 1.0); // old partner actor 1's observed action
    }
    #[test]
    fn h4_setup_and_rng_reproducible() {
        for fam in Family::ALL {
            let d = pilot_design(fam, Policy::A);
            d.validate().unwrap();
            let mut a = Streams::new("replay".into(), 303003);
            let mut b = Streams::new("replay".into(), 303003);
            let mut x = Society::setup(d.clone(), &mut a).unwrap();
            let mut y = Society::setup(d, &mut b).unwrap();
            assert_eq!(x, y);
            for _ in 0..4 {
                let p = Generator
                    .decide_with_streams(AgentId(0), &x, &[], &mut a)
                    .unwrap();
                let q = Generator
                    .decide_with_streams(AgentId(0), &y, &[], &mut b)
                    .unwrap();
                assert_eq!(p, q);
                x.apply(AgentId(0), p).unwrap();
                y.apply(AgentId(0), q).unwrap();
            }
            assert_eq!(x, y);
            assert_eq!(a, b);
        }
    }
    #[test]
    fn a0_rejects_fines_and_choice_without_state_mutation() {
        let d = pilot_design(Family::M, Policy::A0);
        let mut r = Streams::new("smoke".into(), 10);
        let mut w = Society::setup(d, &mut r).unwrap();
        let old = w.clone();
        let rec = PublicRecord {
            period: 1,
            regime: Regime::Choice,
            matching: 0,
            fine_bits: [1, 0],
            actions: [0; 4],
            payoffs: Society::payoff_for(0, [1, 0], [0; 4]),
        };
        assert!(w.apply(AgentId(0), rec).is_err());
        assert_eq!(w, old);
    }
    #[test]
    fn signed_boundary_and_invalid_h4_rejected() {
        let mut d = pilot_design(Family::N, Policy::A);
        d.atoms[0].tuple = Tuple::N {
            eta: -0.25,
            nu: 2.1,
        };
        assert!(d.validate().is_err());
        d.atoms[0].tuple = Tuple::N {
            eta: -0.25,
            nu: 2.0,
        };
        assert!(d.validate().is_ok());
        d.atoms[0].weight = f64::NAN;
        assert!(d.validate().is_err());
    }
    #[test]
    fn public_export_schema_contains_no_private_fields() {
        let r = PublicRecord {
            period: 1,
            regime: Regime::Random,
            matching: 0,
            fine_bits: [0, 1],
            actions: [1, 0, 1, 0],
            payoffs: Society::payoff_for(0, [0, 1], [1, 0, 1, 0]),
        };
        let json = serde_json::to_value(r).unwrap();
        let obj = json.as_object().unwrap();
        let keys: Vec<_> = obj.keys().map(|s| s.as_str()).collect();
        assert_eq!(
            keys,
            vec![
                "actions",
                "fine_bits",
                "matching",
                "payoffs",
                "period",
                "regime"
            ]
        );
        let encoded = serde_json::to_string(&[r]).unwrap();
        for secret in [
            "actors",
            "design",
            "family",
            "nu",
            "kappa",
            "belief",
            "aspiration",
            "seed",
        ] {
            assert!(!encoded.contains(secret), "leaked field {secret}");
        }
    }
    #[test]
    fn choice_broker_is_conditional_on_real_public_history() {
        let x = [
            Tuple::M { eta: 0.2 },
            Tuple::M { eta: -0.15 },
            Tuple::M { eta: 0.45 },
            Tuple::M { eta: -0.4 },
        ];
        let mut society = fixed(Family::M, x);
        let actions = [1, 0, 1, 0];
        let record = PublicRecord {
            period: 1,
            regime: Regime::Random,
            matching: 0,
            fine_bits: [0, 0],
            actions,
            payoffs: Society::payoff_for(0, [0, 0], actions),
        };
        society.apply(AgentId(0), record).unwrap();
        let p = society.broker(Regime::Choice);
        assert!((p.iter().sum::<f64>() - 1.0).abs() < 1e-12);
        assert!(p[1] > 0.55 && p[0] < 0.23 && p[2] < 0.23, "{p:?}");
        assert_eq!(society.broker(Regime::Random), [1.0 / 3.0; 3]);
    }
    #[test]
    fn r_updates_q_using_prior_aspiration_then_updates_aspiration() {
        let mut s = fixed(
            Family::R,
            [Tuple::R {
                eta: 0.0,
                alpha: 1.0,
                lambda: 0.5,
            }; 4],
        );
        let a = [1, 0, 1, 0];
        let rec = PublicRecord {
            period: 1,
            regime: Regime::Random,
            matching: 0,
            fine_bits: [1, 0],
            actions: a,
            payoffs: Society::payoff_for(0, [1, 0], a),
        };
        s.apply(AgentId(0), rec).unwrap();
        // actor 0 cooperated under a half-fine; payoff = -1, prior A=0
        assert_eq!(s.private.q[0][1][1], -1.0);
        assert_eq!(s.private.aspiration[0], -0.5);
        // actor 1 defected; payoff = 1.5, prior aspiration 0
        assert_eq!(s.private.q[1][0][1], 1.5);
        assert_eq!(s.private.aspiration[1], 0.75);
        assert_eq!(s.private.q[1][0][0], 0.0);
    }
    #[test]
    fn s_global_bounds_and_effective_appraisal_checked() {
        let mut d = pilot_design(Family::S, Policy::A);
        d.globals.omega = 2.1;
        assert!(d.validate().is_err());
        d.globals.omega = 0.4;
        d.globals.delta = -0.01;
        assert!(d.validate().is_err());
        d.globals.delta = 0.7;
        d.atoms[0].tuple = Tuple::S { eta: 3.0, tau: 1 };
        assert!(d.validate().is_err());
    }
    #[tokio::test]
    async fn hiyal_trial_setup_and_read_only_replay() {
        use hiyal::{
            experiment::{Trial, verify_setup},
            runtime::ScriptedBackend,
            scheduler::Scheduling,
            storage::Store,
        };
        use std::{
            fs,
            sync::Arc,
            time::{SystemTime, UNIX_EPOCH},
        };
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("exp003-hiyal-{}-{nonce}", std::process::id()));
        let d = pilot_design(Family::C, Policy::A);
        let d_setup = d.clone();
        let mut trial = Trial::create_with_setup(
            &path,
            "exp003-unit-replay",
            303_999,
            KIND,
            move |rng| Society::setup(d_setup, rng),
            Generator,
            Arc::new(ScriptedBackend),
            vec![AgentId(0)],
            Scheduling::RoundRobin,
        )
        .unwrap();
        assert_eq!(trial.run_attempts(4).await.unwrap(), 4);
        trial.checkpoint().unwrap();
        let expected = trial.store.world().clone();
        drop(trial);
        verify_setup(&path, |rng| Society::setup(d.clone(), rng)).unwrap();
        let view = Store::<Society, PublicRecord>::inspect_read_only(&path).unwrap();
        assert_eq!(view.logical_step, 4);
        assert_eq!(view.world, expected);
        fs::remove_dir_all(path).unwrap();
    }
}
