use std::{collections::HashMap, hash::Hash, fmt::Debug};

use crate::util;

// A term of an expression in addition normal form (ANF).
#[derive(Clone)]
pub struct ANFTerm<T: Clone> {
    pub d_sym: HashMap<T, i32>,
    pub d_trans: HashMap<(T, T), i32>,
    
    pub constant: f64,
}

#[derive(Clone)]
pub struct ANFExpr<T: Clone> {
    pub terms: Vec<ANFTerm<T>>
}

pub struct BayesianExp<T: Clone> {
    pub c_sym: HashMap<T, i32>,
    pub c_trans: HashMap<(T, T), i32>,
    pub phi: ANFExpr<T>,
    pub m: HashMap<(T, T), i32>,
    pub active: bool,
    pub q: T,
    pub h: Vec<f64>,
    pub exp: Option<f64>,
}

impl<T: Clone + Eq + Hash + Debug> BayesianExp<T> {
    pub fn init(&mut self, q: T) {
        for t in &self.phi.terms {
            for (k, v) in &t.d_trans {
                match self.m.get(k) {
                    None => { self.m.insert(k.clone(), *v); }
                    Some(d) => { self.m.insert(k.clone(), *v.min(d)); }
                }
            }
        }

        self.active = false;
        self.q = q;
    }

    pub fn next(&mut self, qp: T) -> Option<f64> {
        let trans = (self.q.clone(), qp.clone());

        // Update counts.
        if let Some(&x) = self.c_sym.get(&self.q) {
            self.c_sym.insert(self.q.clone(), x + 1);
        }
        
        if let Some(&x) = self.c_trans.get(&trans.clone()) {
            self.c_trans.insert(trans.clone(), x + 1);
        }
        
        if !self.active {
            if self.is_consistent() {
                self.active = true;
                for i in 0..self.phi.terms.len() {
                    self.h[i] = self.init_h(i);
                }
            }
        } else {
            // println!("-- update");
            for i in 0..self.phi.terms.len() {
                self.h[i] = self.update_h(i, self.q.clone(), qp.clone());
            }
        }

        if self.active {
            let mut exp_new = 0.0;
            for i in 0..self.phi.terms.len() {
                exp_new += self.phi.terms[i].constant * self.h[i];
            }
            self.exp = Some(exp_new);
        }

        self.q = qp;
        self.exp
    }

    pub fn is_consistent(&self) -> bool {
        for (k, v) in &self.c_trans {
            println!("{:?}, {:?}, {:?}, {:?}", k,v, self.m.get(k), self.m);
            if *v + self.m.get(k).unwrap() < 0 {
                return false;
            }
        }
        true
    }

    pub fn init_h(&self, index: usize) -> f64 {
        let t = &self.phi.terms[index];
        let mut result = 0.0;
        for (ij, &d_ij) in &t.d_trans {
            let c_ij = self.c_trans.get(ij).unwrap();
            if d_ij > 0 {
                result += util::ln_permute(c_ij - 1 + d_ij.abs(), d_ij.abs());
            } else {
                result -= util::ln_permute(c_ij - 1, d_ij.abs());
            }
        }
        for (i, &d_i) in &t.d_sym {
            let c_i = self.c_sym.get(i).unwrap();
            if d_i > 0 {
                result -= util::ln_permute(c_i - 1 + d_i.abs(), d_i.abs());
            } else {
                result += util::ln_permute(c_i - 1 , d_i.abs());
            }
        }

        result.exp()
    }

    pub fn update_h(&self, index: usize, q: T, qp: T) -> f64 {
        let trans = (q.clone(), qp.clone());

        let c_q   = *self.c_sym.get(&q.clone()).unwrap_or(&0) as f64;
        let c_qqp = *self.c_trans.get(&trans).unwrap_or(&0)   as f64;

        let t     = &self.phi.terms[index];
        let d_q   = *t.d_sym.get(&q.clone()).unwrap_or(&0)  as f64;
        let d_qqp = *t.d_trans.get(&trans).unwrap_or(&0)   as f64;

        let mut result = self.h[index];

        // let k: f64 = (c_qqp - 1.0 + d_qqp) / (c_qqp - 1.0) * (c_q - 1.0) / (c_q - 1.0 + d_q);
        // println!("-- update_h: k = {}", k);
        result *= (c_qqp - 1.0 + d_qqp) / (c_qqp - 1.0);
        result *= (c_q - 1.0) / (c_q - 1.0 + d_q);

        result
    }
}