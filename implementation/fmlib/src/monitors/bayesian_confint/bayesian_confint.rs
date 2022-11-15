use std::{hash::Hash, fmt::Debug, f64::NAN};

use crate::monitors::bayesian_exp::bayesian_exp::BayesianExp;

pub struct BayesianConfInt<T: Clone> {
    pub exp_monitor: BayesianExp<T>,
    pub exp2_monitor: BayesianExp<T>,
    pub delta: f64,
    pub verdict: (f64, f64)
}

impl<T: Clone + PartialEq + Eq + Hash + Debug> BayesianConfInt<T> {
    pub fn init(&mut self, sigma: T) {
        self.verdict = (NAN, NAN);
        self.exp_monitor.init(sigma.clone());
        self.exp2_monitor.init(sigma.clone());
    }

    pub fn next(&mut self, sigma: T) -> (f64, f64) {
        let e = self.exp_monitor.next(sigma.clone());
        let e2 = self.exp2_monitor.next(sigma.clone());
        
        if e.is_some() && e2.is_some() {
            let e = e.unwrap();
            let e2 = e2.unwrap();
            let s = e2 - e*e;
            let error = (s / self.delta).sqrt();
            
            // println!("-- e: {}, e2: {}, s: {}, error:  {}", e, e2,s,error);
            self.verdict = (e - error, e + error)
        }

        self.verdict
    }
}