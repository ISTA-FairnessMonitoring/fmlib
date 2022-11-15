// On-the-fly simulation of the admission MC

use crate::util;
use super::admission::Admission;

#[derive(Clone, Debug)]
pub struct Applicant {
    pub real_score: i32,
    pub label: bool,
}

pub type Cost = i32;

#[derive(Clone, Debug)]
pub enum Vertex {
    Start,
    Sample(Applicant),
    Cost(Applicant, Cost),
}

pub type Ammc = AdmissionMemorylessMarkovChain;

pub struct AdmissionMemorylessMarkovChain {
    pub admission: Admission,
    pub curr: Option<Vertex>,
}

impl AdmissionMemorylessMarkovChain {
    fn visit_start(&self) -> Vertex {
        let mut neighbors = Vec::<(Vertex, f64)>::new();
        for s in 0..self.admission.score {
            let u = Vertex::Sample(
                Applicant {
                    real_score: s,
                    label: if s >= self.admission.label_threshold {
                        true
                    } else {
                        false
                    }
                }
            );

            neighbors.push((u, 1.0 / self.admission.score as f64))
        }

        util::weighted_choice(&neighbors)
    }

    fn visit_sample(&self, a: Applicant) -> Vertex {
        let score_change = (self.admission.threshold - a.real_score).max(0);
        let change_prob = self.admission.change_prob[score_change as usize];
        let neighbors = vec![
            (
                Vertex::Cost(
                    a.clone(),
                    self.admission.cost[score_change as usize]
                ),
                change_prob
            ),
            (Vertex::Cost(a.clone(), 0), 1.0 - change_prob)
        ];

        util::weighted_choice(&neighbors)
    }

    fn visit_cost(&self) -> Vertex {
        Vertex::Start
    }
}

impl Iterator for AdmissionMemorylessMarkovChain {
    type Item = Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        let next: Option<Self::Item>;
        match &self.curr {
            None => {
                next = Some(Vertex::Start);
            },
            Some(v) => {
                match v {
                    Vertex::Start => {
                        next = Some(self.visit_start());
                    },
                    Vertex::Sample(a) => {
                        next = Some(self.visit_sample((*a).clone()));
                    },
                    Vertex::Cost(_, _) => { next = Some(self.visit_cost()); },
                }
            }
        }

        self.curr = next;
        self.curr.clone()
    }
}
