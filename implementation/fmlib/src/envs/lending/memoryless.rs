use crate::envs::lending::{
    lending::Lending,
    lv::*
};
use crate::util;

#[derive(Debug, Clone)]
pub enum Vertex {
    Env(Env),
    Sample(Env, Sample),
    Decision(Env, Sample, Decision),
    Payback(Env, Sample, Decision, Payback),
}

pub type Lmmc = LendingMemorylessMarkovChain;

pub struct LendingMemorylessMarkovChain {
    curr: Option<Vertex>,
    lending: Lending,
}

impl LendingMemorylessMarkovChain {
    pub fn new(lending: Lending) -> Self {
        Self {
            curr: None,
            lending
        }
    }

    fn visit_env(&self, e: &Env) -> Vertex {
        let (g, cs) = (self.lending.group_count, self.lending.credit_score);
        let gp = self.lending.group_population.clone();
        let total_population: f64 = gp.iter().sum::<i32>() as f64;

        let mut neighbors = Vec::<(Vertex, f64)>::new();

        for i in 0..g {
            for j in 0..cs {
                if e[i as usize][j as usize] == 0 {
                    continue;
                }
                
                let p_group = gp[i as usize] as f64 / total_population;
                let p_cs = (e[i as usize][j as usize] as f64) /
                    (gp[i as usize] as f64);

                neighbors.push((Vertex::Sample(e.clone(), (i, j)), p_group * p_cs));
            }
        }

        util::weighted_choice(&neighbors)
    }

    fn visit_sample(&self, e: &Env, s: Sample) -> Vertex {
        let u = Vec::from(
            [
                Vertex::Decision(e.clone(), s, Decision::Reject),
                Vertex::Decision(e.clone(), s, Decision::Accept),
            ]
        );

        let policy = &self.lending.policy;
        let mut neighbors = Vec::<(Vertex, f64)>::new();
        neighbors.push((u[0].clone(), 1.0 - policy[&s]));
        neighbors.push((u[1].clone(), policy[&s]));
        
        util::weighted_choice(&neighbors)
    }

    fn visit_decision(&self, e: &Env, s: Sample, d: Decision) -> Vertex {
        let payback_prob = self.lending.payback_prob.clone();
        match d {
            Decision::Reject => {
                Vertex::Env(e.clone())
            },
            Decision::Accept => {
                let u = vec![
                    Vertex::Payback(e.clone(), s, d, Payback::Fail),
                    Vertex::Payback(e.clone(), s, d, Payback::Success),
                ];

                let neighbors =  vec![
                    (u[0].clone(), 1. - payback_prob[&s.1]),
                    (u[1].clone(), payback_prob[&s.1]),
                ];

                util::weighted_choice(&neighbors)
            },
        }
    }

    fn visit_payback(&self, e: &Env, s: Sample, p: Payback) -> Vertex {
        let mut new_env = e.clone();
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

        Vertex::Env(new_env)
    }
}

impl Iterator for LendingMemorylessMarkovChain {
    type Item = Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        let next: Option<Self::Item>;
        match &self.curr {
            None => {
                next = Some(Vertex::Env(self.lending.init_credit.clone()));
            },
            Some(v) => {
                match v {
                    Vertex::Env(e) => {
                        next = Some(self.visit_env(e));
                    },
                    Vertex::Sample(e, s) => {
                        next = Some(self.visit_sample(e, *s));
                    },
                    Vertex::Decision(e, s, d) => {
                        next = Some(self.visit_decision(e, *s, *d));
                    },
                    Vertex::Payback(e, s, _d, p) => {
                        next = Some(self.visit_payback(e, *s, *p));
                    },
                }
            },
        }
        
        self.curr = next;
        self.curr.clone()
    }
}