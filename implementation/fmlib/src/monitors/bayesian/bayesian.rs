use std::cell::RefCell;


use super::tv::{Tv, Fraction};

#[derive(Default)]
pub struct Bayesian<T: Clone> {
    pub vertices: RefCell<Vec<Tv<T>>>,
    pub par: Vec<i32>,
    pub last: T,
}

impl<T: Clone + PartialEq> Bayesian<T> {
    pub fn init(&mut self, sigma: T) {
        self.last = sigma;
    }

    pub fn find_freq(&self) -> Vec<i32> {
        let mut vertices = Vec::<i32>::new();
        for v in self.vertices.borrow().iter() {
            if let Tv::Frequency(_, f) = v {
                if self.last == f.a {
                    vertices.push(v.idx());
                }
            }
        }

        vertices
    }
    
    pub fn root(&self) -> i32 {
        let mut result = -1;
        for x in self.vertices.borrow().iter() {
            if self.par[x.idx() as usize] == -1 {
                result = x.idx();
            }
        }

        result
    }

    pub fn next(&mut self, sigma: T) -> Fraction {
        let freqs = self.find_freq();

        for f in freqs {
            self.vertices.borrow_mut()[f as usize].update(sigma.clone());
        }
        
        self.last = sigma;

        self.vertices.borrow()[self.root() as usize].eval(&*self.vertices.borrow())
    }
}