use std::cell::RefCell;

use crate::op;

use super::{tv::{Tv, Frequency, Binary}, bayesian::Bayesian};

pub struct BayesianBuilder<T: Clone> {
    vertices: RefCell<Vec<Tv<T>>>,
    par: Vec<i32>,
    index: i32,
}

impl<T: Clone + Default> BayesianBuilder<T> {
    pub fn new() -> Self {
        Self { vertices: RefCell::from(vec![]), index: 0, par: vec![] }
    }

    pub fn add_freq(
        &mut self,
        a: T,
        a_prime: T,
        param_a: f64,
        param_a_ap: f64
    ) -> &mut BayesianBuilder<T> {
        let v = Tv::Frequency(
            self.index,
            Frequency {
                a, a_prime, param_a, param_a_ap,
                count_a: 0, count_a_ap: 0,
            }
        );

        self.vertices.borrow_mut().push(v);
        self.par.push(-1);
        self.index += 1;

        self
    }

    pub fn add_bin_op(
        &mut self,
        left: i32,
        right: i32,
        op: op::BinOp,
    ) -> &mut BayesianBuilder<T> {
        let v: Tv<T>;
        match op {
            op::BinOp::Sum => {
                v = Tv::Sum(self.index, Binary { left, right });
            },
            op::BinOp::Subtract => {
                v = Tv::Subtract(self.index, Binary { left, right });
            },
            op::BinOp::Prod => {
                v = Tv::Prod(self.index, Binary { left, right });
            },
            op::BinOp::ProdDependent => {
                v = Tv::ProdDependent(self.index, Binary { left, right });
            }
            _ => { unimplemented!() }
        };

        self.vertices.borrow_mut().push(v);
        self.par.push(-1);
        self.par[left as usize] = self.index;
        self.par[right as usize] = self.index;
        self.index += 1;

        self
    }

    pub fn add_unary_op(
        &mut self,
        child: i32,
        constant: f64,
        op: op::UnaryOp
    ) -> &mut BayesianBuilder<T> {
        let v: Tv<T>;
        match op {
            op::UnaryOp::Prod => { 
                v = Tv::ProdUnary(self.index, child, constant)
            },
            op::UnaryOp::Square => {
                v = Tv::Square(self.index, child)
            }
            _ => { unimplemented!() }
        }

        self.vertices.borrow_mut().push(v);
        self.par.push(-1);
        self.par[child as usize] = self.index;
        self.index += 1;
        
        self
    }

    pub fn build(&self) -> Bayesian<T> {
        Bayesian {
            vertices: self.vertices.clone(),
            par: self.par.clone(),
            ..Default::default()
        }
    }
}