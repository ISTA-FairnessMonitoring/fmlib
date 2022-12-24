use std::{collections::{HashSet, HashMap}, hash::Hash, fmt::Debug};

use super::frequentist_opt::FrequentistOpt;

pub type Tv<T> = Vertex<T>;

#[derive(Clone)]
pub struct Variable<T: Clone + Eq + Hash> {
    pub s: T,
    pub sp: T,
    pub time: i32,
    pub vertex: BaseVertexData
}

#[derive(Clone)]
pub struct Binary<T: Clone + Eq + Hash> {
    pub left: Box<Tv<T>>,
    pub right: Box<Tv<T>>,
    pub vertex: BaseVertexData
}

#[derive(Clone)]
pub struct BaseVertexData {
    pub idx: i32,        // Index
    pub reg: Option<f64> // Register
}

impl BaseVertexData {
    pub fn new(idx: i32) -> Self {
        Self { idx, reg: None }
    }
}

#[derive(Clone)]
pub enum Vertex<T: Clone + Eq + Hash> {
    Variable(Variable<T>),
    Sum(Binary<T>),
    Subtract(Binary<T>),
    Prod(Binary<T>),
    ProdDep(Binary<T>),
    ProdUnary(BaseVertexData, Box<Tv<T>>, f64), // (Vertex, Child, Constant)
    Inverse(BaseVertexData, Box<Tv<T>>, i32),   // (Vertex, Child, Counter)
}

impl<T: Clone + Eq + Hash + PartialEq + Debug> Vertex<T> {
    pub fn reset_t(&mut self) {
        match self {
            Self::Variable(ref mut v) => {
                v.time = -1;
            },
            Self::Sum(b) |
            Self::Subtract(b) |
            Self::Prod(b) |
            Self::ProdDep(b) => {
                b.left.reset_t();
                b.right.reset_t();
            },
            Self::ProdUnary(_, tv, _) |
            Self::Inverse(_, tv, _) => {
                tv.reset_t();
            },
        }
    }

    pub fn reset_comp(&mut self) {
        match self {
            Self::Variable(v) => {
                v.vertex.reg = None;
            },
            Self::Sum(b) |
            Self::Subtract(b) |
            Self::Prod(b) |
            Self::ProdDep(b) => {
                b.vertex.reg = None;
                b.left.reset_comp();
                b.right.reset_comp();
            },
            Self::ProdUnary(v, tv, _) |
            Self::Inverse  (v, tv, _) => {
                v.reg = None;
                tv.reset_comp();
            },
        }
    }

    pub fn dep(&self) -> HashSet<T> {
        let mut result = HashSet::new();
        match self {
            Self::Variable(v) => {
                result.insert(v.s.clone());
            },
            Self::Sum(b) |
            Self::Subtract(b) |
            Self::Prod(b) |
            Self::ProdDep(b) => {
                result = b.left.dep();
                result.extend(b.right.dep());
            },
            Self::ProdUnary(_, tv, _) |
            Self::Inverse(_, tv, _) => {
                result = tv.dep();
            },
        }

        result
    }

    pub fn size(&self) -> i32 {
        let result: i32;
        match self {
            Self::Variable(_) => {
                result = 1;
            },
            Self::Sum(b) |
            Self::Subtract(b) |
            Self::Prod(b) |
            Self::ProdDep(b) => {
                result = b.left.size() + b.right.size();
            },
            Self::ProdUnary(_, tv, _) |
            Self::Inverse(_, tv, _) => {
                result = tv.size();
            },
        }

        result
    }

    pub fn vars(&self) -> HashSet<(T, T)> {
        let mut result = HashSet::new();
        match self {
            Self::Variable(v) => {
                result.insert((v.s.clone(), v.sp.clone()));
            },
            Self::Sum(b) |
            Self::Subtract(b) |
            Self::Prod(b) |
            Self::ProdDep(b) => {
                result = b.left.vars();
                result.extend(b.right.vars());
            },
            Self::ProdUnary(_, tv, _) |
            Self::Inverse(_, tv, _) => {
                result = tv.vars();
            },
        }

        result
    }

    pub fn max_t(&self, s: T) -> i32 {
        match self {
            Self::Variable(v) => {
                v.time
            },
            Self::Sum(b) |
            Self::Subtract(b) |
            Self::Prod(b) |
            Self::ProdDep(b) => {
                b.left.max_t(s.clone()).max(
                    b.right.max_t(s.clone())
                )
            },
            Self::ProdUnary(_, tv, _) |
            Self::Inverse(_, tv, _) => {
                tv.max_t(s)
            },
        }
    }

    pub fn phase_t(&mut self, s: T, t: i32) {
        match self {
            Self::Variable(v) => {
                if v.s == s {
                    // println!("------ var_[{:?}, {:?}] phase_t({})", v.s, v.sp, t);
                    v.time = t;
                }
            },
            Self::Sum(b) |
            Self::Subtract(b) |
            Self::Prod(b) |
            Self::ProdDep(b) => {
                b.left.phase_t(s.clone(), t);
                b.right.phase_t(s.clone(), t);
            },
            Self::ProdUnary(_, tv, _) |
            Self::Inverse(_, tv, _) => {
                tv.phase_t(s.clone(), t);
            },
        }
    }

    pub fn eval(&mut self, m: &mut FrequentistOpt<T>) -> Option<f64> {
        match self {
            Self::Variable(v) => {
                if v.vertex.reg.is_some() {
                    // println!("---- var_[{:?}, {:?}] is_some", v.s, v.sp);
                    v.vertex.reg
                } else {
                    let x_s = m.x.get(&v.s).unwrap();
                    if v.time + 1 >= x_s.len() as i32 {
                        // println!("---- var_[{:?}, {:?}] extract_outcome({})", v.s, v.sp, v.time + 1);
                        m.extract_outcome(v.s.clone(), v.time + 1);
                    }
                    else {
                        // println!("---- var_[{:?}, {:?}] extracted", v.s, v.sp);
                        v.vertex.reg = Some(0.0);
                        if let Some(x) = &x_s[(v.time + 1) as usize] {
                            if *x == v.sp {
                                v.vertex.reg = Some(1.0);
                            }
                        }
                    }
                    v.vertex.reg
                }
            },
            Self::Sum(b) => {
                if b.vertex.reg.is_some() {
                    b.vertex.reg
                } else {
                    let eval_l = b.left.eval(m);
                    let eval_r = b.right.eval(m);
                    if eval_l.is_some() && eval_r.is_some() {
                        b.vertex.reg = Some(eval_l.unwrap() + eval_r.unwrap());
                    }

                    b.vertex.reg
                }
            },
            Self::Subtract(b) => {
                if b.vertex.reg.is_some() {
                    b.vertex.reg
                } else {
                    let eval_l = b.left.eval(m);
                    let eval_r = b.right.eval(m);
                    if eval_l.is_some() && eval_r.is_some() {
                        b.vertex.reg = Some(eval_l.unwrap() - eval_r.unwrap());
                    }
                    b.vertex.reg
                }
            },
            Self::Prod(b) => {
                if b.vertex.reg.is_some() {
                    b.vertex.reg
                } else {
                    let eval_l = b.left.eval(m);
                    let eval_r = b.right.eval(m);
                    if eval_l.is_some() && eval_r.is_some() {
                        b.vertex.reg = Some(eval_l.unwrap() * eval_r.unwrap());
                    }
                    b.vertex.reg
                }
            }
            Self::ProdUnary(v, tv, c) => {
                if v.reg.is_some() {
                    v.reg
                } else {
                    let eval_tv = tv.eval(m);
                    if eval_tv.is_some() {
                        v.reg = Some(*c * eval_tv.unwrap());
                    }
                    v.reg
                }
                
            },
            Self::ProdDep(b) => {
                if b.vertex.reg.is_some() {
                    b.vertex.reg
                } else {
                    let dep_l = b.left.dep();
                    let mut dep_intrsct = HashSet::<T>::new();
                    for v in b.right.vars() {
                        if dep_l.contains(&v.0) {
                            dep_intrsct.insert(v.0);
                        }
                    }
                    
                    let mut t_intrsct = HashMap::<T, i32>::new();
                    for d in dep_intrsct {
                        let max_t = b.left.max_t(d.clone());
                        t_intrsct.insert(d.clone(), max_t);
                    }
                    
                    for (s, t) in t_intrsct {
                        b.right.phase_t(s, t + 1);
                    }

                    let eval_l = b.left.eval(m);
                    let eval_r = b.right.eval(m);
                    if eval_l.is_some() && eval_r.is_some() {
                        b.vertex.reg = Some(eval_l.unwrap() * eval_r.unwrap());
                    }
                    b.vertex.reg
                }
            }
            _ => { unimplemented!() }
        }
    }

    pub fn range(&self) -> [f64; 2] {
        match self {
            Self::Variable(_) => [0.0, 1.0],
            
            // [l + l', h + h']
            Self::Sum(b) => {
                let d_l = b.left.range();
                let d_r = b.right.range();
                [d_l[0] + d_r[0] , d_l[1] + d_r[1]]
            },

            // [l - h', l' - h]
            Self::Subtract(b) => {
                let d_l = b.left.range();
                let d_r = b.right.range();
                [d_l[0] - d_r[1] , d_l[1] - d_r[0]]
            },

            // [min(S), max(S)]
            // where S = {l*h, l*h', l'*h, l'*h'}.
            Self::Prod(b) | Self::ProdDep(b) => {
                let d_l = b.left.range();
                let d_r = b.right.range();

                let (mut min, mut max): (f64, f64) = (1e9, -1e9);
                for x in d_l.iter() {
                    for y in d_r.iter() {
                        min = min.min(x * y);
                        max = max.max(x * y);
                    }
                }

                [min, max]
            },

            Self::ProdUnary(_, tv, c) => {
                let d_tv = tv.range();
                if *c > 0.0 {
                    [d_tv[0] * c, d_tv[1] * c]
                } else {
                    [d_tv[1] * c, d_tv[0] * c]
                }
            },
            _ => { unimplemented!() }
        }
    }
}