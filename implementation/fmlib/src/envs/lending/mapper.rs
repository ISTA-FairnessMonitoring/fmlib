use std::{collections::{VecDeque, HashMap}, rc::Rc};

use crate::envs::lending::{
    lending::Lending,
    lv::*,
    visitor::Visitor,
};
use crate::envs::mc::Mc;
use crate::util::*;

#[derive(Default)]
pub struct LendingMarkovChainMapper {
    lending: Lending,
    m: Vec<Vec<(i32, f64)>>,
    pi: Vec<(i32, f64)>,
    queue: VecDeque<Box<dyn LendingAbstractVertex>>,
    label_map: HashMap<Rc<Lv>, i32>,
    index: i32,
}

pub type LMCMapper = LendingMarkovChainMapper;

impl LendingMarkovChainMapper {
    pub fn new(lending: Lending) -> Self {
        Self {
            lending,
            ..Default::default()
        }
    }

    pub fn map(&mut self) -> (Mc, HashMap<Rc<Lv>, i32>) {
        let v = EnvVertex::new(self.lending.init_credit.clone());
        let v = Box::from(v);

        self.index(v.to_vertex());
        self.queue.push_back(v);

        while !self.queue.is_empty() {
            let v = self.queue.pop_front().unwrap();
            v.accept(self);
        }

        self.pi = vec![(0, 1.0)];

        (
            Mc::new(self.m.clone(), self.pi.clone()),
            self.label_map.clone()
        )
    }

    fn index(&mut self, v: Rc<Lv>) -> i32 {
        match self.label_map.get(&v) {
            Some(i) => *i,
            None => {
                let i = self.index;
                self.label_map.insert(v.clone(), i);
                self.index += 1;
                i
            }
        }
    }
}

impl Visitor for LendingMarkovChainMapper {
    fn visit_env(&mut self, v: &EnvVertex) {
        let (g, cs) = (self.lending.group_count, self.lending.credit_score);
        let gp = self.lending.group_population.clone();

        let mut neighbors = Vec::<(i32, f64)>::new();
        let p_group = 1.0 / (g as f64);

        for i in 0..g {
            for j in 0..cs {
                if v.env[i as usize][j as usize] == 0 {
                    continue;
                }

                let u = Box::from(SampleVertex::new(
                    v.env.clone(),
                    (i, j))
                );
                
                let p_cs = (v.env[i as usize][j as usize] as f64) /
                    (gp[i as usize] as f64);

                self.index(u.to_vertex());
                self.queue.push_back(u.to_owned());

                neighbors.push((self.index(u.to_vertex()), p_group * p_cs));
            }
        }

        self.m.push(neighbors);
    }

    fn visit_sample(&mut self, v: &SampleVertex) {
        let u = Vec::from(
            [
                Box::from(DecisionVertex::new(
                    v.env.clone(),
                    v.sample,
                    Decision::Reject)
                ),
                Box::from(DecisionVertex::new(
                    v.env.clone(),
                    v.sample,
                    Decision::Accept)
                ),
            ]
        );

        for u_ in &u {
            self.index(u_.to_vertex());
            self.queue.push_back(u_.to_owned());
        }

        let policy = self.lending.policy.clone();
        let mut neighbors = Vec::<(i32, f64)>::new();
        neighbors.push((self.index(u[0].to_vertex()), 1.0 - policy[&v.sample]));
        neighbors.push((self.index(u[1].to_vertex()), policy[&v.sample]));
        
        self.m.push(neighbors);
    }

    fn visit_decision(&mut self, v: &DecisionVertex) {
        let payback_prob = self.lending.payback_prob.clone();
        match v.decision {
            Decision::Reject => {
                let u = self.index(
                    Rc::from(
                        (Some(v.env.clone()), None, None, None)
                    )
                );
                self.m.push(vec![(u, 1.0)]);
            },
            Decision::Accept => {
                let u = vec![
                    Box::from(PaybackVertex::new(
                        v.env.clone(),
                        v.sample,
                        v.decision,
                        Payback::Fail)
                    ),
                    Box::from(PaybackVertex::new(
                        v.env.clone(),
                        v.sample,
                        v.decision,
                        Payback::Success)
                    ),
                ];

                for u_ in &u {
                    self.index(u_.to_vertex());
                    self.queue.push_back(u_.to_owned());
                }

                let neighbors =  vec![
                    (self.index(u[0].to_vertex()), payback_prob[&u[0].sample.1]),
                    (self.index(u[1].to_vertex()), payback_prob[&u[1].sample.1]),
                ];
                self.m.push(neighbors);
            },
        }
    }

    fn visit_payback(&mut self, v: &PaybackVertex) {
        let mut new_env = v.env.clone();
        let sample = v.sample;

        let cs = self.lending.credit_score;

        match v.payback {
            Payback::Fail => {
                _update(&mut new_env, sample, -1);
                _update(&mut new_env,
                    (sample.0, (sample.1 - 1).max(0)), 1);
                _max_update(&mut new_env, sample, 0);
                
            },
            Payback::Success => {
                _update(&mut new_env, sample, -1);
                _update(&mut new_env,
                    (sample.0, (sample.1 + 1).min(cs - 1)), 1);
                _max_update(&mut new_env, sample, 0);
            },
        }

        let u = Box::from(EnvVertex::new(new_env));

        if let None = self.label_map.get(&u.to_vertex()) {
            self.index(u.to_vertex());
            self.queue.push_back(u.to_owned());
        }

        let u = self.index(u.to_vertex());
        self.m.push(vec![(u, 1.0)]);
    }
}
