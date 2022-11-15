use std::cell::RefCell;
use std::collections::VecDeque;

use crate::op::{BinOp, UnaryOp};

use super::tv::*;
use super::frequentist::{Frequentist, INVERSE_LIMIT};

pub struct FrequentistBuilder<T: Clone> {
    pub(crate) vertices: Vec<Tv<T>>, // Vector of all vertices
    pub(crate) index: i32,           // For indexing new vertices
    pub(crate) par: Vec<i32>,        // Parent relation; par[v] = index(parent(v)).
    pub(crate) delta: f64,
}

impl<T: Clone + Default> FrequentistBuilder<T> {
    pub fn new() -> Self {
        Self { vertices: vec![], index: 0, par: vec![], delta: 0.0 }
    }

    pub fn set_delta(&mut self, delta: f64) -> &mut FrequentistBuilder<T> {
        self.delta = delta;
        self
    }
    
    // For adding a Frequency vertex, we only need to know a, a'
    pub fn add_freq(&mut self, a: T, a_prime: T) -> &mut FrequentistBuilder<T> {
        let v: Tv<T> = Tv::Frequency(self.index, Frequency { a, a_prime, count: 0 });
    
        self.vertices.push(v);
        self.par.push(-1);
        self.index += 1;
    
        self
    }

    // For adding a Binary Operator vertex,
    // we need to know the left and right operands,
    // and the operator itself.
    pub fn add_bin_op(
        &mut self,
        left: i32,
        right: i32,
        op: BinOp,
    ) -> &mut FrequentistBuilder<T> {
        let (q_left, q_right) = (VecDeque::<f64>::new(), VecDeque::<f64>::new());
        let v: Tv<T>;
        match op {
            BinOp::Sum => {
                v = Tv::Sum(
                    self.index,
                    Binary { left, right, q_left, q_right }
                );
            },
            BinOp::Subtract => {
                v = Tv::Subtract(
                    self.index,
                    Binary { left, right, q_left, q_right }
                );
            },
            BinOp::Prod => {
                v = Tv::Prod(
                    self.index,
                    Binary { left, right, q_left, q_right }
                );
            },
            BinOp::ProdDependent => {
                v = Tv::ProdDependent(
                    self.index,
                    Binary { left, right, q_left, q_right }
                );
            },

            _ => { unimplemented!() }
        }

        self.vertices.push(v);
        self.par.push(-1);
        
        // Setting parent for the left and right children
        self.par[left as usize] = self.index;
        self.par[right as usize] = self.index;
        self.index += 1;
    
        self
    }

    pub fn add_unary_op(
        &mut self,
        child: i32,
        constant: f64,
        op: UnaryOp,
    ) -> &mut FrequentistBuilder<T> {
        let v: Tv<T>;
        match op {
            UnaryOp::Sum => {
                v = Tv::SumUnary(self.index, child, constant);
            },
            UnaryOp::Subtract => {
                v = Tv::SubtractUnary(self.index, child, constant);
            },
            UnaryOp::Prod => {
                v = Tv::ProdUnary(self.index, child, constant);
            },
            UnaryOp::InverseAtomic => {
                v = Tv::InverseAtomic(
                    self.index, child,
                    InverseAtomic { count: 0, limit: INVERSE_LIMIT }
                );
            },
            _ => { unimplemented!() }
        }

        self.vertices.push(v);
        self.par.push(-1);
        self.par[child as usize] = self.index;
        self.index += 1;
    
        self
    }

    // Create an instance of a Frequentist monitor.
    // Necessary data for the monitor is provided
    // by the data which already exists in the builder.
    pub fn build(&self) -> Frequentist<T> {
        Frequentist {
            vertices: RefCell::new(self.vertices.clone()),
            par: self.par.clone(),
            delta: self.delta,
            ..Default::default()
        }
    }
}
