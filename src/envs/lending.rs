use std::collections::HashMap;

use crate::util;

#[derive(Default)]
pub struct LendingProblem {
  pub group_count: i32, // G
  pub max_credit_score: i32, // C; credit scores are in the range [0..C-1]
  pub group_population: Vec<i32>, // [N1, N2, ..., NG]
  pub payback_prob: HashMap<i32, f64>, // [0..C) -> [0.0, 1.0]
  pub init_credit: Vec<Vec<i32>>, // [0..G) * [0..C) -> [1..N]
  pub policy: HashMap<(i32, i32), f64>, // [0..G) * [0..C) -> [0.0, 1.0]
}

pub type Sample = (i32, i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Decision {
  Accept, Reject,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Payback {
  Success, Fail,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Vertex {
  Init,
  Sample  (Sample),
  Decision(Sample, Decision),
  Payback (Sample, Decision, Payback),
}

pub struct LendingMarkovChain {
  pub problem: LendingProblem,
  pub curr: Vertex,
  pub credit_distribution: Vec<Vec<i32>>,
}

impl LendingMarkovChain {
  pub fn new(problem: LendingProblem) -> Self {
    Self {
      curr: Vertex::Init,
      credit_distribution: problem.init_credit.clone(),
      // This field is valuated last to avoid move-related problems.
      problem,
    }
  }

  fn visit_env(&self) -> Vertex {
    let (gc, cs) = (
      self.problem.group_count, 
      self.problem.max_credit_score
    );
    let gp = self.problem.group_population.clone();
    let total_population: f64 = gp.iter().sum::<i32>() as f64;

    let mut neighbors = Vec::<(Vertex, f64)>::new();

    for i in 0..gc as usize {
      for j in 0..cs as usize {
        if self.credit_distribution[i][j] == 0 {
            continue;
        }
          
        let p_group = gp[i] as f64 / total_population;
        let p_cs = (
          self.credit_distribution[i][j] as f64
        ) / (gp[i] as f64);

        neighbors.push(
          (Vertex::Sample((i as i32, j as i32)), p_group * p_cs));
      }
    }

    util::weighted_choice(&neighbors)
  }

  fn visit_sample(&self, s: Sample) -> Vertex {
    let u = Vec::from(
      [
        Vertex::Decision(s, Decision::Reject),
        Vertex::Decision(s, Decision::Accept),
      ]
    );

    let policy = &self.problem.policy;
    let mut neighbors = Vec::<(Vertex, f64)>::new();
    
    neighbors.push(
      (u[0].clone(), 1.0 - policy[&s])
    );
    neighbors.push(
      (u[1].clone(), policy[&s])
    );
    
    util::weighted_choice(&neighbors)
  }

  fn visit_decision(&self, s: Sample, d: Decision) -> Vertex {
    let payback_prob = self.problem.payback_prob.clone();
    match d {
      Decision::Reject => {
        Vertex::Init
      },
      Decision::Accept => {
        let u = vec![
          Vertex::Payback(s, d, Payback::Fail),
          Vertex::Payback(s, d, Payback::Success),
        ];

        let neighbors =  vec![
          (u[0].clone(), 1. - payback_prob[&s.1]),
          (u[1].clone(), payback_prob[&s.1]),
        ];

        util::weighted_choice(&neighbors)
      },
    }
  }

  fn visit_payback(&mut self, s: Sample, p: Payback) -> Vertex {
    // new_distrib is the population distribution after moving the 
    // sampled individual in accordance with the payback status (p).
    let mut new_distrib = self.credit_distribution.clone();
    let sample = s;

    let cs = self.problem.max_credit_score;

    match p {
      Payback::Fail => {
        // Destination (group, credit score) combination to which the
        // sampled individual will be moved.
        let dest = (sample.0, (sample.1 - 1).max(0));
        util::_update(&mut new_distrib, sample, -1);
        util::_update(&mut new_distrib, dest, 1);
        util::_max_update(&mut new_distrib, sample, 0);
      },
      Payback::Success => {
        let dest = (sample.0, (sample.1 + 1).min(cs - 1));
        util::_update(&mut new_distrib, sample, -1);
        util::_update(&mut new_distrib, dest, 1);
        util::_max_update(&mut new_distrib, sample, 0);
      },
    }
      
    self.credit_distribution = new_distrib;
    Vertex::Init
  }
}

impl Iterator for LendingMarkovChain {
  type Item = Vertex;

  fn next(&mut self) -> Option<Self::Item> {
    let next: Vertex;
    match self.curr {
      Vertex::Init             => next = self.visit_env(),
      Vertex::Sample(s)        => next = self.visit_sample(s),
      Vertex::Decision(s, d)   => next = self.visit_decision(s, d),
      Vertex::Payback(s, _, p) => next = self.visit_payback(s, p),
    };
      
    let result = Some(self.curr.clone());
    self.curr = next;
    result
  }
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
pub enum ObservedVertex {
  Sample(Sample),
  Decision(Sample, Decision),
  Payback(Sample, Payback),
  #[default]
  Null,
}

pub struct LendingVertexMapper;

impl LendingVertexMapper {
  pub fn map(&self, s: &Vertex) -> Option<ObservedVertex> {
    match s {
      Vertex::Sample(s)        => Some(ObservedVertex::Sample(*s)),
      Vertex::Decision(s, d)   => Some(ObservedVertex::Decision(*s, *d)),
      Vertex::Payback(s, _, p) => Some(ObservedVertex::Payback(*s, *p)),
      _ => None,
    }
  }
}
