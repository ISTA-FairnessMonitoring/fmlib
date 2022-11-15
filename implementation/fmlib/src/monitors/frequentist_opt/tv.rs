use std::{rc::Rc, cell::RefCell, collections::{HashSet, HashMap}, hash::Hash, fmt::Debug};

use super::frequentist_opt::FrequentistOpt;

pub type Tv<T> = TreeVertex<T>;
pub type X<T> = RefCell<HashMap<T, RefCell<Vec<Option<T>>>>>;

pub struct Variable<T: Clone + Eq + Hash> {
    pub s: T,
    pub sp: T,
    pub time: i32,
}

pub struct Binary<T: Clone + Eq + Hash> {
    pub left: Rc<RefCell<Tv<T>>>,
    pub right: Rc<RefCell<Tv<T>>>,
}

pub enum TreeVertex<T: Clone + Eq + Hash> {
    Variable(i32, Option<f64>, Variable<T>),              // (ID, Register, Data)
    Sum(i32, Option<f64>, Binary<T>),                     // (ID, Register, Data)
    Subtract(i32, Option<f64>, Binary<T>),
    Prod(i32, Option<f64>, Binary<T>),
    ProdDep(i32, Option<f64>, Binary<T>),
    ProdUnary(i32, Option<f64>, Rc<RefCell<Tv<T>>>, f64), // (ID, Register, Child, Constant)
    Inverse(i32, Option<f64>, Rc<RefCell<Tv<T>>>, i32),   // (ID, Register, Child, Counter)
}

impl<T: Clone + Eq + Hash + PartialEq + Debug> TreeVertex<T> {
    pub fn reset_t(&mut self) {
        match self {
            Self::Variable(_, _, ref mut v) => {
                v.time = -1;
            },
            Self::Sum(_, _, b) |
            Self::Subtract(_, _, b) |
            Self::Prod(_, _, b) |
            Self::ProdDep(_, _, b) => {
                b.left.borrow_mut().reset_t();
                b.right.borrow_mut().reset_t();
            },
            Self::ProdUnary(_, _, tv, _) |
            Self::Inverse(_, _, tv, _) => {
                tv.borrow_mut().reset_t();
            },
        }
    }

    pub fn reset_comp(&mut self) {
        match self {
            Self::Variable(_, r, _) => {
                *r = None;
            },
            Self::Sum     (_, r, b) |
            Self::Subtract(_, r, b) |
            Self::Prod    (_, r, b) |
            Self::ProdDep (_, r, b) => {
                *r = None;
                b.left.borrow_mut().reset_comp();
                b.right.borrow_mut().reset_comp();
            },
            Self::ProdUnary(_, r, tv, _) |
            Self::Inverse  (_, r, tv, _) => {
                *r = None;
                tv.borrow_mut().reset_comp();
            },
        }
    }

    pub fn dep(&self) -> HashSet<T> {
        let mut result = HashSet::new();
        match self {
            Self::Variable(_, _, v) => {
                result.insert(v.s.clone());
            },
            Self::Sum(_, _, b) |
            Self::Subtract(_, _, b) |
            Self::Prod(_, _, b) |
            Self::ProdDep(_, _, b) => {
                result = b.left.borrow().dep();
                result.extend(b.right.borrow().dep());
            },
            Self::ProdUnary(_, _, tv, _) |
            Self::Inverse(_, _, tv, _) => {
                result = tv.borrow().dep();
            },
        }

        result
    }

    pub fn size(&self) -> i32 {
        let result: i32;
        match self {
            Self::Variable(_, _, _) => {
                result = 1;
            },
            Self::Sum(_, _, b) |
            Self::Subtract(_, _, b) |
            Self::Prod(_, _, b) |
            Self::ProdDep(_, _, b) => {
                result = b.left.borrow().size() + b.right.borrow().size();
            },
            Self::ProdUnary(_, _, tv, _) |
            Self::Inverse(_, _, tv, _) => {
                result = tv.borrow().size();
            },
        }

        result
    }

    pub fn vars(&self) -> HashSet<(T, T)> {
        let mut result = HashSet::new();
        match self {
            Self::Variable(_, _, v) => {
                result.insert((v.s.clone(), v.sp.clone()));
            },
            Self::Sum(_, _, b) |
            Self::Subtract(_, _, b) |
            Self::Prod(_, _, b) |
            Self::ProdDep(_, _, b) => {
                result = b.left.borrow().vars();
                result.extend(b.right.borrow().vars());
            },
            Self::ProdUnary(_, _, tv, _) |
            Self::Inverse(_, _, tv, _) => {
                result = tv.borrow().vars();
            },
        }

        result
    }

    pub fn max_t(&self, s: T) -> i32 {
        match self {
            Self::Variable(_, _, v) => {
                v.time
            },
            Self::Sum(_, _, b) |
            Self::Subtract(_, _, b) |
            Self::Prod(_, _, b) |
            Self::ProdDep(_, _, b) => {
                b.left.borrow().max_t(s.clone()).max(
                    b.right.borrow().max_t(s.clone())
                )
            },
            Self::ProdUnary(_, _, tv, _) |
            Self::Inverse(_, _, tv, _) => {
                tv.borrow().max_t(s)
            },
        }
    }

    pub fn phase_t(&mut self, s: T, t: i32) {
        match self {
            Self::Variable(_, _, v) => {
                if v.s == s {
                    // println!("------ var_[{:?}, {:?}] phase_t({})", v.s, v.sp, t);
                    v.time = t;
                }
            },
            Self::Sum(_, _, b) |
            Self::Subtract(_, _, b) |
            Self::Prod(_, _, b) |
            Self::ProdDep(_, _, b) => {
                b.left.borrow_mut().phase_t(s.clone(), t);
                b.right.borrow_mut().phase_t(s.clone(), t);
            },
            Self::ProdUnary(_, _, tv, _) |
            Self::Inverse(_, _, tv, _) => {
                tv.borrow_mut().phase_t(s.clone(), t);
            },
        }
    }

    pub fn eval(&mut self, m: &mut FrequentistOpt<T>) -> Option<f64> {
        match self {
            Self::Variable(_, r, v) => {
                if r.is_some() {
                    // println!("---- var_[{:?}, {:?}] is_some", v.s, v.sp);
                    *r
                } else {
                    let x_s = m.x.get(&v.s.clone()).unwrap().clone();
                    if v.time + 1 >= x_s.borrow().len() as i32 {
                        // println!("---- var_[{:?}, {:?}] extract_outcome({})", v.s, v.sp, v.time + 1);
                        m.extract_outcome(v.s.clone(), v.time + 1);
                    }
                    if v.time + 1 < x_s.borrow().len() as i32 {
                        // println!("---- var_[{:?}, {:?}] extracted", v.s, v.sp);
                        *r = Some(0.0);
                        if let Some(x) = &x_s.borrow()[(v.time + 1) as usize] {
                            if *x == v.sp {
                                *r = Some(1.0);
                            }
                        }
                    }
                    *r
                }
            },
            Self::Sum(_, r, b) => {
                if r.is_some() {
                    *r
                } else {
                    let eval_l = b.left.borrow_mut().eval(m);
                    let eval_r = b.right.borrow_mut().eval(m);
                    if eval_l.is_some() && eval_r.is_some() {
                        *r = Some(eval_l.unwrap() + eval_r.unwrap());
                    }

                    *r
                }
            },
            Self::Subtract(_, r, b) => {
                if r.is_some() {
                    *r
                } else {
                    let eval_l = b.left.borrow_mut().eval(m);
                    let eval_r = b.right.borrow_mut().eval(m);
                    if eval_l.is_some() && eval_r.is_some() {
                        *r = Some(eval_l.unwrap() - eval_r.unwrap());
                    }
                    *r
                }
            },
            Self::Prod(_, r, b) => {
                if r.is_some() {
                    *r
                } else {
                    let eval_l = b.left.borrow_mut().eval(m);
                    let eval_r = b.right.borrow_mut().eval(m);
                    if eval_l.is_some() && eval_r.is_some() {
                        *r = Some(eval_l.unwrap() * eval_r.unwrap());
                    }
                    *r
                }
            }
            Self::ProdUnary(_, r, tv, c) => {
                if r.is_some() {
                    *r
                } else {
                    let eval_tv = tv.borrow_mut().eval(m);
                    if eval_tv.is_some() {
                        *r = Some(*c * eval_tv.unwrap());
                    }
                    *r
                }
                
            },
            Self::ProdDep(_, r, b) => {
                if r.is_some() {
                    *r
                } else {
                    let dep_l = b.left.borrow().dep();
                    let mut dep_intrsct = HashSet::<T>::new();
                    for v in b.right.borrow().vars() {
                        if dep_l.contains(&v.0) {
                            dep_intrsct.insert(v.0);
                        }
                    }
                    
                    let mut t_intrsct = HashMap::<T, i32>::new();
                    for d in dep_intrsct {
                        let max_t = b.left.borrow().max_t(d.clone());
                        t_intrsct.insert(d.clone(), max_t);
                    }
                    
                    for (s, t) in t_intrsct {
                        b.right.borrow_mut().phase_t(s, t + 1);
                    }

                    let eval_l = b.left.borrow_mut().eval(m);
                    let eval_r = b.right.borrow_mut().eval(m);
                    if eval_l.is_some() && eval_r.is_some() {
                        *r = Some(eval_l.unwrap() * eval_r.unwrap());
                    }
                    *r
                }
            }
            _ => { unimplemented!() }
        }
    }

    pub fn range(&self) -> [f64; 2] {
        match self {
            Self::Variable(_, _, _) => [0.0, 1.0],
            
            // [l + l', h + h']
            Self::Sum(_, _, b) => {
                let d_l = b.left.borrow().range();
                let d_r = b.right.borrow().range();
                [d_l[0] + d_r[0] , d_l[1] + d_r[1]]
            },

            // [l - h', l' - h]
            Self::Subtract(_, _, b) => {
                let d_l = b.left.borrow().range();
                let d_r = b.right.borrow().range();
                [d_l[0] - d_r[1] , d_l[1] - d_r[0]]
            },

            // [min(S), max(S)]
            // where S = {l*h, l*h', l'*h, l'*h'}.
            Self::Prod(_, _, b) | Self::ProdDep(_, _, b) => {
                let d_l = b.left.borrow().range();
                let d_r = b.right.borrow().range();

                let (mut min, mut max): (f64, f64) = (1e9, -1e9);
                for x in d_l.iter() {
                    for y in d_r.iter() {
                        min = min.min(x * y);
                        max = max.max(x * y);
                    }
                }

                [min, max]
            },

            Self::ProdUnary(_, _, tv, c) => {
                let d_tv = tv.borrow().range();
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