use crate::envs::lending::{
    lending::Lending,
};
use crate::util;

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
    Env     (),
    Sample  (Sample),
    Decision(Sample, Decision),
    Payback (Sample, Decision, Payback),
}

// pub type Lmmc = LendingMarkovChain;

pub struct LendingMarkovChain {
    pub curr: Option<Vertex>,
    pub lending: Lending,
    pub env: Vec<Vec<i32>>,
}

impl LendingMarkovChain {
    pub fn new(lending: Lending) -> Self {
        Self {
            curr: None,
            env: lending.init_credit.clone(),
            lending,
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

impl Iterator for LendingMarkovChain {
    type Item = Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        let next: Option<Self::Item>;
        match &self.curr {
            None => {
                next = Some(Vertex::Env());
            },
            Some(v) => {
                match v {
                    Vertex::Env() => {
                        next = Some(self.visit_env());
                    },
                    Vertex::Sample(s) => {
                        next = Some(self.visit_sample(*s));
                    },
                    Vertex::Decision(s, d) => {
                        next = Some(self.visit_decision(*s, *d));
                    },
                    Vertex::Payback(s, _d, p) => {
                        next = Some(self.visit_payback(*s, *p));
                    },
                }
            },
        }
        
        self.curr = next;
        self.curr.clone()
    }
}