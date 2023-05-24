use std::{collections::HashMap, vec, hash::Hash, f64::NAN};

use crate::op::{BinOp, UnaryOp};

use super::{tv::{Tv, Variable, Binary, BaseVertexData}, frequentist_opt::FrequentistOpt};

pub struct FrequentistOptBuilder<T: Clone + Eq + Hash> {
    pub c_sym: HashMap<T, i32>,
    pub c_trans: HashMap<(T, T), i32>,
    pub x: HashMap<T, Vec<Option<T>>>,
    pub vertices: Vec<Tv<T>>,
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
        let v = Tv::Variable(
            Variable {
                s: s.clone(),
                sp: sp.clone(),
                time: 0,
                vertex: BaseVertexData::new(self.index),
            }
        );
        
        self.c_sym.insert(s.clone(), 0);
        self.c_trans.insert((s.clone(), sp.clone()), 0);
        self.x.insert(s.clone(), vec![]);

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
        let (left, right) = (left as usize, right as usize);
        match op {
            BinOp::Sum => {
                v = Tv::Sum( 
                    Binary {
                        left: Box::new(self.vertices[left].clone()),
                        right: Box::new(self.vertices[right].clone()),
                        vertex: BaseVertexData::new(self.index),
                    }
                );
            },
            BinOp::Subtract => {
                v = Tv::Subtract( 
                    Binary {
                        left: Box::new(self.vertices[left].clone()),
                        right: Box::new(self.vertices[right].clone()),
                        vertex: BaseVertexData::new(self.index),
                    }
                );
            },
            BinOp::Prod => {
                v = Tv::Prod(
                    Binary {
                        left: Box::new(self.vertices[left].clone()),
                        right: Box::new(self.vertices[right].clone()),
                        vertex: BaseVertexData::new(self.index),
                    }
                );
            },
            BinOp::ProdDependent => {
                v = Tv::ProdDep(
                    Binary {
                        left: Box::new(self.vertices[left].clone()),
                        right: Box::new(self.vertices[right].clone()),
                        vertex: BaseVertexData::new(self.index),
                    }
                );
            },
            _ => { unimplemented!() }
            
        }

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
        let child = child as usize;
        match op {
            UnaryOp::Prod => {
                v = Tv::ProdUnary(
                    BaseVertexData::new(self.index),
                    Box::new(self.vertices[child as usize].clone()),
                    constant,
                );
            },
            UnaryOp::InverseAtomic => {
                v = Tv::Inverse(
                    BaseVertexData::new(self.index),
                    Box::new(self.vertices[child as usize].clone()),
                    0,
                );
            },
            _ => { unimplemented!() }
        }

        self.vertices.push(v);
        self.index += 1;
        
        self
    }

    pub fn build(&mut self) -> FrequentistOpt<T> {
        let phi = self.vertices.last().cloned().unwrap();

        FrequentistOpt {
            c_sym: self.c_sym.clone(),
            c_trans: self.c_trans.clone(),
            phi: Box::new(phi),
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