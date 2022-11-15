use crate::monitors::{
    frequentist::frequentist::Frequentist,
    fafrequentist::frequentist::FaFrequentist,
    bayesian::bayesian::Bayesian,
};

use std::fmt::Debug;

pub enum Monitor<T: Clone> {
    Frequentist(Frequentist<T>),
    FairnessAwareFrequentist(FaFrequentist<T>),
    Bayesian(Bayesian<T>),
}

impl<T: Clone + PartialEq + Debug> Monitor<T> {
    pub fn init(&mut self, sigma: T) {
        match self {
            Self::Frequentist(ref mut f) => {
                f.init(sigma);
            },
            Self::FairnessAwareFrequentist(ref mut f) => {
                f.init(sigma);
            }
            Self::Bayesian(ref mut f) => {
                f.init(sigma);
            },
        }
    }

    // Currently, each monitor is returning a different value,
    // So, we unify them manually after a call
    // to the concrete monitor's `next`.
    pub fn next(&mut self, sigma: T) -> Option<(f64, f64)> {
        match self {
            Self::Frequentist(ref mut f) => {
                f.next(sigma)
            },
            Self::FairnessAwareFrequentist(ref mut f) => {
                let result = f.next(sigma);
                Some((result.value - result.epsilon, result.value + result.epsilon))
            }
            Self::Bayesian(ref mut f) => {
                let result = f.next(sigma);
                Some((result.to_number(), result.to_number()))
            },
        }
    }
}