use std::{cell::RefCell, fmt::Debug};
use crate::util;

use super::tv::*;

#[derive(Default)]
pub struct Frequentist<T: Clone> {
    // The reason for using RefCell<...> was
    // making it possible to borrow `vertices` as mutable
    // in methods with immutable `self` references.
    pub vertices: RefCell<Vec<Tv<T>>>,
    pub par: Vec<i32>,
    pub delta: f64,
    pub last: T,
    
    // Result-related fields
    pub sum: f64,
    pub n: i32,
    pub mean: f64,
    pub variance: f64,
    // The `m2` field is related to the Welford calculations
    pub m2: f64,
    // Result caching is necessary to preserve last verdict of the monitor
    pub cached_result: Option<(f64, f64)>,
}

// Constant factor for limiting observations of the InverseAtomic operator
pub(crate) const INVERSE_LIMIT: i32 = 10;

impl<T: Clone + PartialEq + Debug> Frequentist<T> {
    pub fn init(&mut self, sigma: T) {
        self.last = sigma;
        
        self.sum = 0.0;
        self.n = 0;
    }

    // Find all Frequency vertices F(a, a') where a = last.
    // Return value is a vector of (v, x) pairs,
    // where v is a Frequency vertex of form F(last, ...),
    // and x is the sample value for this vertex
    // (x = 1 <=> v = F(last, sigma).)
    fn find_freq(&self, sigma: T) -> Vec<(Tv<T>, f64)> {
        let mut vertices = Vec::<Tv<T>>::new();
        for v in self.vertices.borrow().iter() {
            if let Tv::Frequency(_, f) = v {
                if self.last == f.a {
                    vertices.push(v.clone());
                }
            }
        }

        let mut result = Vec::<(Tv<T>, f64)>::new();
        for v in vertices {
            if let Tv::Frequency(_, ref f) = v {
                if sigma == f.a_prime {
                    result.push((v, 1.0));
                } else {
                    result.push((v, 0.0));
                }
            }
        }
        
        result
    }

    // Propagates a value up in the expression tree.
    // Return type is a (v, x) pair,
    // where v = parent(u) and u is the last vertex
    // with a valid return value,
    // and x is the last valid returned value.
    fn bubble_up(
        &self,
        f: &Tv<T>,
        call_value: f64
    ) -> (i32, f64) {
        let mut val = call_value;
        let (mut curr, mut prev) = (f.idx(), -1);
        while curr != -1 {
            let v = &mut self.vertices.borrow_mut()[curr as usize];
            let result = v.update(prev, val);
            
            if let Some(x) = result {
                val = x;
            } else {
                break;
            }

            prev = curr;
            curr = self.par[curr as usize];
        }
        (curr, val)
    }

    // Find root of the expression tree.
    fn root(&self) -> i32 {
        let mut result = -1;
        for x in self.vertices.borrow().iter() {
            if self.par[x.idx() as usize] == -1 {
                result = x.idx();
            }
        }

        result
    }

    // Shorthand function to evaluate final error of the monitor
    fn error(n: f64, d: f64, delta: f64) -> f64 {
        (-n * d * d / 2.0 * (delta / 2.0).ln()).sqrt() / n
    }

    fn root_domain(&self) -> f64 {
        let root = &self.vertices.borrow()[self.root() as usize];
        let root_domain = root.domain(&*self.vertices.borrow());
        root_domain[1] - root_domain[0]
    }

    pub fn next(&mut self, sigma: T) -> Option<(f64, f64)> {
        let freqs = self.find_freq(sigma.clone());
        
        for (f, val) in freqs {
            let (curr, value) = self.bubble_up(&f, val);
        
            // Self::bubble_up() is defined such that
            // if curr = -1, then root vertex has returned a valid value
            // (parent(root) = -1).
            if curr == -1 {
                let data = util::welford_update(
                        util::WelfordData { n: self.n, mean: self.mean, m2: self.m2 }, value
                    );
                
                self.n = data.n;
                self.mean =data.mean;
                self.m2 = data.m2;

                if self.n >= 2 {
                    self.variance = self.m2 / (self.n as f64 - 1.0);
                }

                let d = self.root_domain();
                let error = Self::error(self.n as f64, d, self.delta);
                
                self.cached_result = Some((self.mean - error, self.mean + error));
            }
        }
        
        self.last = sigma.clone();

        self.cached_result
    }
}
