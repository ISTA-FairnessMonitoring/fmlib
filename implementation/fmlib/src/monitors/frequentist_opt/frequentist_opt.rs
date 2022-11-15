use std::{collections::HashMap, cell::RefCell, rc::Rc, hash::Hash, fmt::Debug};

use crate::util;

use super::tv::Tv;

pub struct FrequentistOpt<T: Clone + Eq + Hash> {
    pub c_sym: HashMap<T, i32>,
    pub c_trans: HashMap<(T, T), i32>,
    pub x: HashMap<T, Rc<RefCell<Vec<Option<T>>>>>,
    pub phi: Rc<RefCell<Tv<T>>>,
    pub delta: f64,

    pub last: T,
    pub n: i32,
    pub mean: f64,
    pub verdict: (f64, f64),
    pub range: (f64, f64),
}

impl<T: Clone + Eq + Hash + Debug> FrequentistOpt<T> {
    pub fn update_verdict(&mut self, w: f64, n: i32) -> (f64, f64) {
        self.mean += (w - self.mean) / n as f64;
        let e = Self::hoeffding(self.range, self.delta, n);
        (self.mean - e, self.mean + e)
    }

    pub fn hoeffding(range: (f64, f64), delta: f64, n: i32) -> f64 {
        let diff = range.1 - range.0;
        (-diff * diff / (2.0 * n as f64) * (delta / 2.0).ln()).sqrt()
    }

    pub fn extract_outcome(&mut self, i: T, t: i32) {
        // Find all observed successors of `i`.
        let mut u: Vec<T> = vec![];
        for (k, _) in self.c_trans.iter() {
            if k.0 == i {
                u.push(k.1.clone());
            }
        }

        // `n` = # of samples.
        let n = t + 1 - self.x.get(&i).unwrap().borrow().len() as i32;
        
        for _ in 0..n {
            // All _observed_ successors (All j. c_{ij} > 0.)
            let mut v: Vec<(Option<T>, f64)> = vec![];
            let c_i = *self.c_sym.get(&i).unwrap();
            if c_i == 0 {
                break;
            }
            let mut prob_sum = 0.0;
            for j in &u {
                let c_ij = *self.c_trans.get(&(i.clone(), j.clone())).unwrap();
                let prob = c_ij as f64 / c_i as f64;
                if (prob - 0.0).abs() > 1e-10 {
                    v.push((Some(j.clone()), prob));
                    prob_sum += prob;
                }
            }

            if (1.0 - prob_sum).abs() > 1e-10 {
                v.push((None, 1.0 - prob_sum));
            }

            let j = util::weighted_choice(&v);
            self.x.get(&i).unwrap().borrow_mut().push(j.clone());
            
            if let Some(x) = j {
                let c_ij = *self.c_trans.get(&(i.clone(), x.clone())).unwrap();
                self.c_trans.insert((i.clone(), x.clone()), c_ij - 1);    
            }

            self.c_sym.insert(i.clone(), c_i - 1);
        }
    }

    pub fn init(&mut self) {
        self.reset_x();
        self.reset_t();
        self.reset_comp();
        self.set_range();
    }

    pub fn reset_x(&self) {
        for (_, v) in self.x.iter() {
            v.borrow_mut().clear();
        }
    }

    pub fn reset_t(&self) {
        self.phi.borrow_mut().reset_t();
    }

    pub fn reset_comp(&self) {
        self.phi.borrow_mut().reset_comp();
    }

    pub fn set_range(&mut self) {
        let r = self.phi.borrow().range();
        self.range = (r[0], r[1]);
    }

    pub fn next(&mut self, sigma: T) -> (f64, f64) {
        // Update counts.
        if let Some(x) = self.c_sym.get(&self.last.clone()) {
            self.c_sym.insert(self.last.clone(), *x + 1);
        }

        let trans = (self.last.clone(), sigma.clone());
        if let Some(x) = self.c_trans.get(&trans) {
            self.c_trans.insert(trans.clone(), *x + 1);
        }

        let phi = self.phi.clone();
        let w = phi.borrow_mut().eval(self);
        if let Some(x) = w {
            self.n += 1;
            self.verdict = self.update_verdict(x, self.n);
            
            self.reset_x();
            self.reset_t();
            self.reset_comp();
        }
        self.last = sigma;

        self.verdict
    }
}