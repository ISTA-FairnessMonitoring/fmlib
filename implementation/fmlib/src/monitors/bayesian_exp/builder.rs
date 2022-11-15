use std::{collections::HashMap, hash::Hash};

use super::bayesian_exp::{ANFTerm, ANFExpr, BayesianExp};

pub struct ANFTermBuilder<T: Clone> {
    pub d_sym: HashMap<T, i32>,
    pub d_trans: HashMap<(T, T), i32>,
    
    pub constant: f64,
}

impl<T: Clone + Eq + Hash> ANFTermBuilder<T> {
    pub fn new() -> Self {
        Self {
            d_sym: HashMap::new(),
            d_trans: HashMap::new(),
            constant: 0.0,
        }
    }

    pub fn set_constant(&mut self, c: f64) -> &mut Self {
        self.constant = c;
        self
    }

    pub fn add_var(&mut self, q: T, qp: T, d: i32) -> &mut Self {
        self.d_trans.insert((q.clone(), qp.clone()), d);
        match self.d_sym.get(&q) {
            None => { self.d_sym.insert(q.clone(), d); }
            Some(v) => { self.d_sym.insert(q.clone(), *v + d); }
        }

        self
    }

    pub fn build(&self) -> ANFTerm<T> {
        ANFTerm {
            d_sym: self.d_sym.clone(),
            d_trans: self.d_trans.clone(),
            constant: self.constant
        }
    }
}

pub struct ANFExprBuilder<T: Clone> {
    pub terms: Vec<ANFTerm<T>>
}

impl<T: Clone> ANFExprBuilder<T> {
    pub fn new() -> Self {
        Self {terms: vec![] }
    }

    pub fn add_term(&mut self, t: ANFTerm<T>) -> &mut Self {
        self.terms.push(t);
        self
    }

    pub fn build(&self) -> ANFExpr<T> {
        ANFExpr {
            terms: self.terms.clone()
        }
    }
}

pub struct BayesianExpBuilder<T: Clone> {
    pub c_sym: HashMap<T, i32>,
    pub c_trans: HashMap<(T, T), i32>,
    
    phi: ANFExpr<T>,
}

impl<T: Clone + Eq + Hash + Default> BayesianExpBuilder<T> {
    pub fn new() -> Self {
        Self {
            c_sym: HashMap::new(),
            c_trans: HashMap::new(),
            phi: ANFExpr { terms: vec![] }
        }
    }

    pub fn set_expr(&mut self, phi: ANFExpr<T>) -> &mut Self {
        self.phi = phi;
        self
    }

    pub fn set_sym_const(&mut self, q: T, c: i32) -> &mut Self {
        self.c_sym.insert(q, c);
        self
    }

    pub fn set_trans_const(&mut self, q: T, qp: T, c: i32) -> &mut Self {
        self.c_trans.insert((q, qp), c);
        self
    }

    pub fn build(&self) -> BayesianExp<T> {
        BayesianExp {
            c_sym: self.c_sym.clone(),
            c_trans: self.c_trans.clone(),
            phi: self.phi.clone(),
            m: HashMap::new(),
            active: false,
            q: T::default(),
            h: vec![0.0; self.phi.terms.len()],
            exp: None,
        }
    }
}
