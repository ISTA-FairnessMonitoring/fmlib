use std::{cell::RefCell, collections::HashMap, vec, rc::Rc, hash::Hash, f64::NAN};

use crate::op::{BinOp, UnaryOp};

use super::{tv::{Tv, Variable, Binary}, frequentist_opt::FrequentistOpt};

pub struct FrequentistOptBuilder<T: Clone + Eq + Hash> {
    pub c_sym: HashMap<T, i32>,
    pub c_trans: HashMap<(T, T), i32>,
    pub x: HashMap<T, Rc<RefCell<Vec<Option<T>>>>>,
    pub vertices: Vec<Rc<RefCell<Tv<T>>>>,
    pub delta: f64,
    pub index: i32,
}

impl<T: Clone + Eq + Hash + Default> FrequentistOptBuilder<T> {
    pub fn new() -> Self {
        FrequentistOptBuilder {
            c_sym: HashMap::new(),
            c_trans: HashMap::new(),
            x: HashMap::new(),
            vertices: vec![],
            delta: 0.0,
            index: 0,
        }
    }

    pub fn set_delta(&mut self, delta: f64) -> &mut Self {
        self.delta = delta;
        self
    }

    pub fn add_var(
        &mut self, s: T, sp: T
    ) -> &mut Self {
        let v = Tv::Variable(self.index, None, Variable {
            s: s.clone(), sp: sp.clone(), time: 0
        });
        
        self.c_sym.insert(s.clone(), 0);
        self.c_trans.insert((s.clone(), sp.clone()), 0);
        self.x.insert(s.clone(), Rc::new(RefCell::new(vec![])));

        let v = Rc::new(RefCell::new(v));
        self.vertices.push(v);
        self.index += 1;
        
        self
    }

    // For portability
    pub fn add_freq(&mut self, s: T, sp: T) -> &mut Self {
        self.add_var(s, sp);
        self
    }

    pub fn add_bin_op(
        &mut self,
        left: i32,
        right: i32,
        op: BinOp,
    ) -> &mut Self {
        let v: Tv<T>;
        match op {
            BinOp::Sum => {
                v = Tv::Sum(self.index, None, 
                    Binary {
                        left: self.vertices[left as usize].clone(),
                        right: self.vertices[right as usize].clone(),
                    }
                );
            },
            BinOp::Subtract => {
                v = Tv::Subtract(self.index, None, 
                    Binary {
                        left: self.vertices[left as usize].clone(),
                        right: self.vertices[right as usize].clone(),
                    }
                );
            },
            BinOp::Prod => {
                v = Tv::Prod(self.index, None,
                    Binary {
                        left: self.vertices[left as usize].clone(),
                        right: self.vertices[right as usize].clone(),
                    }
                );
            },
            BinOp::ProdDependent => {
                v = Tv::ProdDep(self.index, None,
                    Binary {
                        left: self.vertices[left as usize].clone(),
                        right: self.vertices[right as usize].clone(),
                    }
                );
            },
            _ => { unimplemented!() }
            
        }

        let v = Rc::new(RefCell::new(v));
        self.vertices.push(v);
        self.index += 1;
        
        self
    }

    pub fn add_unary_op(
        &mut self,
        child: i32,
        constant: f64,
        op: UnaryOp,
    ) -> &mut Self {
        let v: Tv<T>;
        match op {
            UnaryOp::Prod => {
                v = Tv::ProdUnary(self.index, None, self.vertices[child as usize].clone(), constant);
            },
            UnaryOp::InverseAtomic => {
                v = Tv::Inverse(self.index, None, self.vertices[child as usize].clone(), 0);
            },
            _ => { unimplemented!() }
        }

        let v = Rc::new(RefCell::new(v));
        self.vertices.push(v);
        self.index += 1;
        
        self
    }

    pub fn build(&mut self) -> FrequentistOpt<T> {
        let phi = self.vertices.last().unwrap();

        FrequentistOpt {
            c_sym: self.c_sym.clone(),
            c_trans: self.c_trans.clone(),
            phi: phi.clone(),
            n: 0,
            mean: 0.0,
            x: self.x.clone(),
            last: T::default(),
            verdict: (NAN, NAN),
            range: (NAN, NAN),
            delta: self.delta,
        }
    }
}