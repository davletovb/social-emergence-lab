//! Experiment 001: small, integer-state opportunity allocation societies.
//! This is a model of a specified mechanism, not a claim about human societies.
#![forbid(unsafe_code)]

use hiyal::{
    core::{AgentId, World},
    experiment::EffectPolicy,
    rng::Streams,
    runtime::Receipt,
};
use serde::{Deserialize, Serialize};

pub const KIND: &str = "social-emergence-lab.inequality.v1";
pub const ACTORS: usize = 20;
pub const INITIAL_WEALTH: u64 = 10;
pub const OFFSET: u64 = 1;

/// Uniform is the control; linear wealth feedback is the preregistration candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Feedback {
    Uniform,
    Linear,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Society {
    pub wealth: Vec<u64>,
    pub feedback: Feedback,
    pub offset: u64,
    pub opportunities: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Observation {
    wealth: Vec<u64>,
    feedback: Feedback,
    offset: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allocate {
    pub recipient: usize,
}

impl Society {
    pub fn new(
        actors: usize,
        initial: u64,
        offset: u64,
        feedback: Feedback,
    ) -> Result<Self, String> {
        if !(2..=1_000).contains(&actors) || initial == 0 || offset == 0 {
            return Err("require 2..=1000 agents, positive initial wealth and offset".into());
        }
        initial.checked_add(offset).ok_or("weight overflow")?;
        let actors_u64 = u64::try_from(actors).map_err(|e| e.to_string())?;
        actors_u64
            .checked_mul(initial)
            .ok_or("initial total overflow")?;
        actors_u64
            .checked_mul(initial + offset)
            .ok_or("initial weight overflow")?;
        Ok(Self {
            wealth: vec![initial; actors],
            feedback,
            offset,
            opportunities: 0,
        })
    }

    pub fn total(&self) -> Result<u64, String> {
        self.wealth.iter().try_fold(0u64, |sum, w| {
            sum.checked_add(*w).ok_or_else(|| "total overflow".into())
        })
    }

    pub fn verify_accounting(&self, initial: u64) -> Result<(), String> {
        let expected = u64::try_from(self.wealth.len())
            .map_err(|e| e.to_string())?
            .checked_mul(initial)
            .and_then(|n| n.checked_add(self.opportunities))
            .ok_or("accounting overflow")?;
        if expected != self.total()? {
            return Err("wealth accounting invariant failed".into());
        }
        Ok(())
    }
}

impl World for Society {
    type Action = Allocate;
    type Observation = Observation;

    fn observe(&self, _actor: AgentId) -> Result<Observation, String> {
        Ok(Observation {
            wealth: self.wealth.clone(),
            feedback: self.feedback,
            offset: self.offset,
        })
    }

    fn apply(&mut self, _actor: AgentId, action: Allocate) -> Result<(), String> {
        let target = self
            .wealth
            .get_mut(action.recipient)
            .ok_or("recipient out of range")?;
        let next = target.checked_add(1).ok_or("recipient wealth overflow")?;
        let count = self
            .opportunities
            .checked_add(1)
            .ok_or("opportunity counter overflow")?;
        *target = next;
        self.opportunities = count;
        Ok(())
    }
}

/// Exact rejection sampling using multiplication-high (rather than modulo bias).
/// One sampled ticket per decision, with very rare additional draws if rejected.
pub fn unbiased_ticket(streams: &mut Streams, name: &str, bound: u64) -> Result<u64, String> {
    if bound == 0 {
        return Err("ticket bound is zero".into());
    }
    let threshold = bound.wrapping_neg() % bound;
    loop {
        let draw = streams.draw_u64(name)?;
        let product = u128::from(draw) * u128::from(bound);
        if (product as u64) >= threshold {
            return Ok((product >> 64) as u64);
        }
    }
}

fn weights(obs: &Observation) -> Result<Vec<u64>, String> {
    obs.wealth
        .iter()
        .map(|wealth| match obs.feedback {
            Feedback::Uniform => Ok(1),
            Feedback::Linear => wealth
                .checked_add(obs.offset)
                .ok_or_else(|| "feedback weight overflow".into()),
        })
        .collect()
}

pub fn select_ticket(weights: &[u64], ticket: u64) -> Result<usize, String> {
    let mut left = ticket;
    for (index, weight) in weights.iter().enumerate() {
        if *weight == 0 {
            return Err("nonpositive weight".into());
        }
        if left < *weight {
            return Ok(index);
        }
        left -= weight;
    }
    Err("sample ticket out of range".into())
}

pub struct OpportunityPolicy;
impl EffectPolicy<Society> for OpportunityPolicy {
    fn decide(
        &self,
        _actor: AgentId,
        _obs: &Observation,
        _effects: &[Receipt],
    ) -> Result<Allocate, String> {
        Err("stochastic decision requires Hiyal's named trial streams".into())
    }

    fn decide_with_streams(
        &self,
        actor: AgentId,
        obs: &Observation,
        _effects: &[Receipt],
        rng: &mut Streams,
    ) -> Result<Allocate, String> {
        let values = weights(obs)?;
        let total = values.iter().try_fold(0u64, |s, w| {
            s.checked_add(*w)
                .ok_or_else(|| "weight sum overflow".to_owned())
        })?;
        let ticket = unbiased_ticket(rng, &format!("agent/{}/opportunity", actor.0), total)?;
        Ok(Allocate {
            recipient: select_ticket(&values, ticket)?,
        })
    }
}

/// Gini = sum(i,j)|x_i-x_j| / (2*n*sum(x_i)); zero for equal positive balances.
pub fn gini(wealth: &[u64]) -> Result<f64, String> {
    if wealth.is_empty() {
        return Err("empty society".into());
    }
    let mut numerator = 0u128;
    let mut total = 0u128;
    for &w in wealth {
        total = total.checked_add(u128::from(w)).ok_or("sum overflow")?;
        for &v in wealth {
            numerator = numerator
                .checked_add(u128::from(w.abs_diff(v)))
                .ok_or("difference sum overflow")?;
        }
    }
    if total == 0 {
        return Err("undefined Gini for zero total".into());
    }
    Ok(numerator as f64 / (2.0 * wealth.len() as f64 * total as f64))
}

/// Share of resources of the highest ceil(n/5) balances.
pub fn top_quintile_share(wealth: &[u64]) -> Result<f64, String> {
    if wealth.is_empty() {
        return Err("empty society".into());
    }
    let mut values = wealth.to_vec();
    values.sort_unstable_by(|a, b| b.cmp(a));
    let count = values.len().div_ceil(5);
    let top = values[..count].iter().map(|w| u128::from(*w)).sum::<u128>();
    let total = values.iter().map(|w| u128::from(*w)).sum::<u128>();
    if total == 0 {
        return Err("undefined share for zero total".into());
    }
    Ok(top as f64 / total as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equal_wealth_has_zero_gini() {
        assert_eq!(gini(&[10, 10, 10]).unwrap(), 0.0);
        assert_eq!(gini(&[0, 10]).unwrap(), 0.5);
        assert_eq!(top_quintile_share(&[1, 1, 1, 1, 6]).unwrap(), 0.6);
        assert!(gini(&[0, 0]).is_err());
    }
    #[test]
    fn ticket_partition_is_exact_and_symmetric() {
        let w = vec![1, 1, 1, 1];
        assert_eq!(
            (0..4)
                .map(|x| select_ticket(&w, x).unwrap())
                .collect::<Vec<_>>(),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            (0..6)
                .map(|x| select_ticket(&[1, 2, 3], x).unwrap())
                .collect::<Vec<_>>(),
            vec![0, 1, 1, 2, 2, 2]
        );
        assert!(select_ticket(&w, 4).is_err());
    }
    #[test]
    fn transfers_are_accounted_and_errors_do_not_mutate() {
        let mut w = Society::new(20, 10, 1, Feedback::Uniform).unwrap();
        let old = w.clone();
        assert!(w.apply(AgentId(0), Allocate { recipient: 20 }).is_err());
        assert_eq!(w, old);
        for i in 0..100 {
            w.apply(AgentId(0), Allocate { recipient: i % 20 }).unwrap();
        }
        w.verify_accounting(10).unwrap();
        assert_eq!(w.total().unwrap(), 300);
        assert_eq!(gini(&w.wealth).unwrap(), 0.0);
        w.wealth[0] = u64::MAX;
        let old = w.clone();
        assert!(w.apply(AgentId(0), Allocate { recipient: 0 }).is_err());
        assert_eq!(w, old);
    }
    #[test]
    fn seeded_sampler_has_valid_support() {
        let mut s = Streams::new("test".into(), 42);
        for bound in [1, 2, 3, 20, 283, u64::MAX] {
            for _ in 0..100 {
                assert!(unbiased_ticket(&mut s, "agent/0/opportunity", bound).unwrap() < bound);
            }
        }
    }
}

