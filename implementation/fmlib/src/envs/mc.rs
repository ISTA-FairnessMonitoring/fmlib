// A basic MC for testing purposes

use crate::util::weighted_choice;

// Shorthand name for MarkovChain
pub type Mc = MarkovChain;

pub struct MarkovChain {
    pub m: Vec<Vec<(i32, f64)>>, // Transition probability matrix
    pub pi: Vec<(i32, f64)>,     // Initial distrubtion vector
    curr: Option<i32>,           // Current vertex of the Markov chain (for iteration)
    // `curr` is typed as Option<i32>,
    // as it initially points to no vertex in the chain.
}

impl MarkovChain {
    pub fn new(
        m: Vec<Vec<(i32, f64)>>,
        pi: Vec<(i32, f64)>
    ) -> MarkovChain {
        MarkovChain { m, pi, curr: None }
    }

    pub fn curr(&self) -> Option<i32> {
        self.curr.clone()
    }
}

impl Iterator for MarkovChain {
    // Iterator-required definition
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            // If the Markov chain has just begun,
            // we choose one vertex from the initial distribution.
            None => self.curr = Some(weighted_choice(self.pi.as_slice())),
            
            // If the markov chain is already started,
            // choose one neighbor uniformly at random
            // (with respect to transition weights).
            Some(x) => self.curr = Some(
                weighted_choice(self.m[x as usize].as_slice())
            )
        }
        self.curr.clone()
    }
}
