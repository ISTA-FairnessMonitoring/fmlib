// Functions used to build the syntax tree

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::Debug;

use crate::op::BinOp;

use super::tv::*;
use super::frequentist::FaFrequentist;

pub struct FaFrequentistBuilder<T: Clone> {
    pub(crate) vertices: RefCell<Vec<Tv<T>>>,
    pub(crate) index: i32,
    pub(crate) par: Vec<i32>,
}

impl<T: Clone + PartialEq + Default + Debug> FaFrequentistBuilder<T> {
    pub fn new() -> Self {
        Self { vertices: RefCell::from(vec![]), index: 0, par: vec![] }
    }

    pub fn set_delta(&mut self, delta: f64) -> &mut FaFrequentistBuilder<T> {
        let mut root: i32 = -1;
        for v in self.vertices.borrow().iter() {
            if self.par[v.idx() as usize] == -1 {
                root = v.idx();
            }
        }

        let mut q = VecDeque::<(i32, f64)>::new();
        q.push_back((root, delta));

        while !q.is_empty() {
            let (v, d) = q.pop_front().unwrap();
            let tv = &mut self.vertices.borrow_mut()[v as usize];
            tv.set_delta(d);
            for u in tv.children() {
                q.push_back((u, d/2.0));
            }
        }

        self
    }

    pub fn add_freq(&mut self, a: T, a_prime: T) -> &mut FaFrequentistBuilder<T> {
        let v: Tv<T> = Tv::Frequency(self.index, Frequency { a, a_prime, count: 0, estimate: 0.0, delta: 1.0 });

        self.vertices.borrow_mut().push(v);
        self.par.push(-1);
        self.index += 1;

        self
    }

    pub fn add_bin_op(
        &mut self,
        left: i32,
        right: i32,
        op: BinOp,
    ) -> &mut FaFrequentistBuilder<T> {
        let v: Tv<T>;
        match op {
            BinOp::Sum => {
                v = Tv::Sum(
                    self.index,
                    Binary { left, right }
                );
            },
            BinOp::Subtract => {
                v = Tv::Subtract(
                    self.index,
                    Binary { left, right }
                );
            },
            BinOp::Prod => {
                v = Tv::Prod(
                    self.index,
                    Binary { left, right }
                );
            },
            _ => { unimplemented!() }
        }

        self.vertices.borrow_mut().push(v);
        self.par.push(-1);
        self.par[left as usize] = self.index;
        self.par[right as usize] = self.index;
        self.index += 1;

        self
    }

    pub fn add_constant(
        &mut self, c: f64
    ) -> &mut FaFrequentistBuilder<T> {
        let v = Tv::Constant(self.index, c);
        self.vertices.borrow_mut().push(v);
        self.par.push(-1);
        self.index += 1;

        self
    }

    pub fn build(&self) -> FaFrequentist<T> {
        FaFrequentist {
            vertices: self.vertices.clone(),
            par: self.par.clone(),
            ..Default::default()
        }
    }
}
