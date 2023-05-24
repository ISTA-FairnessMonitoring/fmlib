// On-the-fly simulation of the admission MC

use crate::util;

// An instance of the admission problem. This struct is mainly used 
// as a parameter-set for the admission Markov chain.
#[derive(Clone)]
pub struct AdmissionProblem {
  // Scores are actually in the range [0, max_score-1]
  pub max_score: i32,
  // Used as the score threshold promoted by the college 
  pub college_threshold: i32,
  // Used for labelling Markov chain vertices 
  pub label_threshold: i32,
  // cost[i] is the cost incurred on an applicant who wants to change 
  // their score by i units.
  pub cost: Vec<i32>,
  // change_prob[i] is the probability that an applicant changes 
  // their score by i units. 
  pub change_prob: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct Applicant {
  pub real_score: i32,
  // label is defined as real_score >= label_threshold. There is also 
  // a helper function with the same name defined below.  
  pub label: bool,
}

impl Applicant {
  pub fn label(real_score: i32, label_threshold: i32) -> bool {
    real_score >= label_threshold
  }
}

pub type Cost = i32;

#[derive(Clone, Debug)]
pub enum Vertex {
  Start,
  Sample(Applicant),
  Cost(Applicant, Cost),
}

// The Markov chain needs an instance of the admission problem to 
// determine problem parameters. 
pub struct AdmissionMarkovChain {
  pub problem: AdmissionProblem,
  pub curr: Vertex, 
}

impl AdmissionMarkovChain {
  pub fn new(problem: AdmissionProblem) -> Self {
    AdmissionMarkovChain { problem, curr: Vertex::Start }
  }

  fn visit_start(&self) -> Vertex {
    let mut neighbors = Vec::<(Vertex, f64)>::new();
    let prob = 1.0 / self.problem.max_score as f64; 
    
    for s in 0..self.problem.max_score {
      let u = Vertex::Sample(
        Applicant {
          real_score: s,
          label: Applicant::label(s, self.problem.label_threshold)
        }
      );

      neighbors.push( (u, prob) );
    }

    util::weighted_choice(&neighbors)
  }

  fn visit_sample(&self, a: &Applicant) -> Vertex {
    // An applicant never makes a negative change to their score 
    let score_change = (
      self.problem.college_threshold - a.real_score).max(0);
    // Change type to use as index 
    let score_change = score_change as usize; 
    let change_prob = self.problem.change_prob[score_change];
    let neighbors = vec![
      (
        Vertex::Cost(a.clone(), self.problem.cost[score_change]),
        change_prob
      ),
      ( Vertex::Cost(a.clone(), 0), 1.0 - change_prob )
    ];

    util::weighted_choice(&neighbors)
  }

  fn visit_cost(&self) -> Vertex {
    Vertex::Start
  }
}

impl Iterator for AdmissionMarkovChain {
  type Item = Vertex;

  fn next(&mut self) -> Option<Self::Item> {
    let next: Vertex;
    match &self.curr {
      Vertex::Start      => next = self.visit_start(),
      Vertex::Sample(a)  => next = self.visit_sample(a),
      Vertex::Cost(_, _) => next = self.visit_cost(),
    }

    let result = Some(self.curr.clone());
    self.curr = next;
    result 
  }
}

// Interface for: admission Markov chain -> a monitor 

// Admission Markov chain vertex, as observed by a monitor. 
#[derive(Clone, PartialEq, Debug, Default, Eq, Hash)]
pub enum ObservedVertex {
  #[default] Null,
  Sample(bool),
  Cost(i32),
}

pub struct AdmissionVertexMapper;

impl AdmissionVertexMapper {
  pub fn map(&self, s: &Vertex) -> Option<ObservedVertex> {
    match s {
      Vertex::Sample(a)  => Some(ObservedVertex::Sample(a.label)), 
      Vertex::Cost(_, c) => Some(ObservedVertex::Cost(*c)),
      _ => None,
    }
  }
}
