use std::collections::HashMap;

use crate::envs::lending::{
    markov_chain::{Vertex, Sample, Decision, Payback},
    lending::Lending
};
use crate::util;

type TransitionMatrix = HashMap<(Vertex, Vertex), f64>;

pub struct DynamicLendingMarkovChain {
    curr: Option<(Vertex, TransitionMatrix)>,
    env: Vec<Vec<i32>>,
    lending: Lending,
    m: TransitionMatrix,
}

impl DynamicLendingMarkovChain {
    pub fn new(lending: Lending) -> Self {
        Self {
            curr: None,
            env: lending.init_credit.clone(),
            lending,
            m: TransitionMatrix::new(),
        }
    }

    // This method can be called at any point to re-fill the transition matrix,
    // using the most recent value of its parameters.
    pub fn fill_m(&mut self) {
        let (gc, cs) = (self.lending.group_count, self.lending.credit_score);
        let gp = self.lending.group_population.clone();
        let total_population: f64 = gp.iter().sum::<i32>() as f64;
        let policy = &self.lending.policy;
        let payback_prob = &self.lending.payback_prob;

        for i in 0..gc as usize {
            for j in 0..cs as usize {
                let sample = (i as i32, j as i32);

                let p_group = gp[i as usize] as f64 / total_population;
                let p_cs = (self.env[i][j] as f64) / (gp[i as usize] as f64);
                self.m.insert(
                    ( Vertex::Env(), Vertex::Sample(sample) ),
                    p_group * p_cs,
                );
                
                let p_accept = policy[&sample];
                self.m.insert(
                    (
                        Vertex::Sample(sample),
                        Vertex::Decision(sample, Decision::Accept)
                    ),
                    p_accept,
                );
                self.m.insert(
                    (
                        Vertex::Sample(sample),
                        Vertex::Decision(sample, Decision::Reject)
                    ),
                    1.0 - p_accept,
                );

                let p_payback = payback_prob[&sample.1];
                self.m.insert(
                    (
                        Vertex::Decision(sample, Decision::Accept),
                        Vertex::Payback(sample, Decision::Accept, Payback::Success),
                    ),
                    p_payback,
                );
                self.m.insert(
                    (
                        Vertex::Decision(sample, Decision::Accept),
                        Vertex::Payback(sample, Decision::Accept, Payback::Fail),
                    ),
                    1.0 - p_payback,
                );

                // Edges from Vertex::Payback to Env never change,
                // so they can be skipped at this point.
            }
        }
    }

    fn visit_env(&self) -> Vertex {
        let (gc, cs) = (self.lending.group_count, self.lending.credit_score);
        let gp = self.lending.group_population.clone();
        let total_population: f64 = gp.iter().sum::<i32>() as f64;

        let mut neighbors = Vec::<(Vertex, f64)>::new();

        for i in 0..gc as usize {
            for j in 0..cs as usize {
                if self.env[i][j] == 0 {
                    continue;
                }
                
                let p_group = gp[i as usize] as f64 / total_population;
                let p_cs = (self.env[i][j] as f64) / (gp[i as usize] as f64);

                neighbors.push(
                    (Vertex::Sample((i as i32, j as i32)), p_group * p_cs)
                );
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

        let policy = &self.lending.policy;
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
        let payback_prob = self.lending.payback_prob.clone();
        match d {
            Decision::Reject => {
                Vertex::Env()
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
        let mut new_env = self.env.clone();
        let sample = s;

        let cs = self.lending.credit_score;

        match p {
            Payback::Fail => {
                util::_update(&mut new_env, sample, -1);
                util::_update(&mut new_env,
                    (sample.0, (sample.1 - 1).max(0)), 1);
                util::_max_update(&mut new_env, sample, 0);
                
            },
            Payback::Success => {
                util::_update(&mut new_env, sample, -1);
                util::_update(&mut new_env,
                    (sample.0, (sample.1 + 1).min(cs - 1)), 1);
                util::_max_update(&mut new_env, sample, 0);
            },
        }
        
        self.env = new_env;
        Vertex::Env()
    }
}

impl Iterator for DynamicLendingMarkovChain {
    type Item = (Vertex, TransitionMatrix);

    fn next(&mut self) -> Option<Self::Item> {
        let next: Option<Self::Item>;
        match &self.curr {
            None => {
                self.fill_m();
                next = Some( (Vertex::Env(), self.m.clone()) );
            },
            Some(v) => {
                match v.0 {
                    Vertex::Env() => {
                        next = Some( (self.visit_env(), v.1.clone()) );
                    },
                    Vertex::Sample(s) => {
                        next = Some( (self.visit_sample(s), v.1.clone()) );
                    },
                    Vertex::Decision(s, d) => {
                        next = Some( (self.visit_decision(s, d), v.1.clone()) );
                    },
                    Vertex::Payback(s, _d, p) => {
                        let u = self.visit_payback(s, p);
                        self.fill_m();
                        next = Some( (u, self.m.clone()) );
                    },
                }
            },
        }
        
        self.curr = next;
        self.curr.clone()
    }
}

