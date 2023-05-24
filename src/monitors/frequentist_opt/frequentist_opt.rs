use std::{collections::{HashMap, HashSet}, hash::Hash, fmt::Debug};

use crate::util;

use super::tv::Tv;

pub struct FrequentistOpt<T: Clone + Eq + Hash> {
    pub c_sym: HashMap<T, i32>,
    pub c_trans: HashMap<(T, T), i32>,
    pub x: HashMap<T, Vec<Option<T>>>,
    pub phi: Box<Tv<T>>,
    pub delta: f64,

    pub last: T,
    pub n: i32,
    pub mean: f64,
    pub verdict: (f64, f64),
    pub range: (f64, f64),
}

impl<T: Clone + Eq + Hash + Debug> FrequentistOpt<T> {
    pub fn update_verdict(&mut self, w: f64, n: i32) -> (f64, f64) {
        self.mean += (w - self.mean) / n as f64;
        let e = Self::hoeffding(self.range, self.delta, n);
        (self.mean - e, self.mean + e)
    }

    pub fn hoeffding(range: (f64, f64), delta: f64, n: i32) -> f64 {
        let diff = range.1 - range.0;
        (-diff * diff / (2.0 * n as f64) * (delta / 2.0).ln()).sqrt()
    }

    pub fn extract_outcome(&mut self, i: T, t: i32) {
        // Find all observed successors of `i`.
        let mut u: Vec<T> = vec![];
        for (k, _) in self.c_trans.iter() {
            if k.0 == i {
                u.push(k.1.clone());
            }
        }

        // `n` = # of samples.
        let n = t + 1 - self.x.get(&i).unwrap().len() as i32;
        
        for _ in 0..n {
            // All _observed_ successors (All j. c_{ij} > 0.)
            let mut v: Vec<(Option<T>, f64)> = vec![];
            let c_i = *self.c_sym.get(&i).unwrap();
            if c_i == 0 {
                break;
            }
            let mut prob_sum = 0.0;
            for j in &u {
                let c_ij = *self.c_trans.get(&(i.clone(), j.clone())).unwrap();
                let prob = c_ij as f64 / c_i as f64;
                if (prob - 0.0).abs() > 1e-10 {
                    v.push((Some(j.clone()), prob));
                    prob_sum += prob;
                }
            }

            if (1.0 - prob_sum).abs() > 1e-10 {
                v.push((None, 1.0 - prob_sum));
            }

            let j = util::weighted_choice(&v);
            self.x.get_mut(&i).unwrap().push(j.clone());
            
            if let Some(x) = j {
                let c_ij = *self.c_trans.get(&(i.clone(), x.clone())).unwrap();
                self.c_trans.insert((i.clone(), x.clone()), c_ij - 1);    
            }

            self.c_sym.insert(i.clone(), c_i - 1);
        }
    }

    pub fn init(&mut self) {
        self.reset_x();
        self.reset_t();
        self.reset_comp();
        self.set_range();
    }

    pub fn reset_x(&mut self) {
        for (_, v) in self.x.iter_mut() {
            v.clear();
        }
    }

    pub fn reset_t(&mut self) {
        self.phi.reset_t();
    }

    pub fn reset_comp(&mut self) {
        self.phi.reset_comp();
    }

    pub fn set_range(&mut self) {
        let r = self.phi.range();
        self.range = (r[0], r[1]);
    }

    pub fn next(&mut self, sigma: T) -> (f64, f64) {
        // Update counts.
        if let Some(x) = self.c_sym.get(&self.last.clone()) {
            self.c_sym.insert(self.last.clone(), *x + 1);
        }

        let trans = (self.last.clone(), sigma.clone());
        if let Some(x) = self.c_trans.get(&trans) {
            self.c_trans.insert(trans.clone(), *x + 1);
        }

        let w = self.eval(self.phi.clone());
        if let Some(x) = w {
            self.n += 1;
            self.verdict = self.update_verdict(x, self.n);
            
            self.reset_x();
            self.reset_t();
            self.reset_comp();
        }
        self.last = sigma;

        self.verdict
    }

    pub fn eval(&mut self, phi: Box<Tv<T>>) -> Option<f64> {
        match *phi {
            Tv::Variable(mut v) => {
                if v.vertex.reg.is_some() {
                    // println!("---- var_[{:?}, {:?}] is_some", v.s, v.sp);
                    v.vertex.reg
                } else {
                    let x_s = self.x.get(&v.s).unwrap();
                    if v.time + 1 >= x_s.len() as i32 {
                        // println!("---- var_[{:?}, {:?}] extract_outcome({})", v.s, v.sp, v.time + 1);
                        self.extract_outcome(v.s.clone(), v.time + 1);
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
            Tv::Sum(mut b) => {
                if b.vertex.reg.is_some() {
                    b.vertex.reg
                } else {
                    let eval_l = self.eval(b.left);
                    let eval_r = self.eval(b.right);
                    if eval_l.is_some() && eval_r.is_some() {
                        b.vertex.reg = Some(eval_l.unwrap() + eval_r.unwrap());
                    }

                    b.vertex.reg
                }
            },
            Tv::Subtract(mut b) => {
                if b.vertex.reg.is_some() {
                    b.vertex.reg
                } else {
                    let eval_l = self.eval(b.left);
                    let eval_r = self.eval(b.right);
                    if eval_l.is_some() && eval_r.is_some() {
                        b.vertex.reg = Some(eval_l.unwrap() - eval_r.unwrap());
                    }

                    b.vertex.reg
                }
            },
            Tv::Prod(mut b) => {
                if b.vertex.reg.is_some() {
                    b.vertex.reg
                } else {
                    let eval_l = self.eval(b.left);
                    let eval_r = self.eval(b.right);
                    if eval_l.is_some() && eval_r.is_some() {
                        b.vertex.reg = Some(eval_l.unwrap() * eval_r.unwrap());
                    }

                    b.vertex.reg
                }
            },
            Tv::ProdUnary(mut v, tv, c) => {
                if v.reg.is_some() {
                    v.reg
                } else {
                    let eval_tv = self.eval(tv);
                    if eval_tv.is_some() {
                        v.reg = Some(c * eval_tv.unwrap());
                    }
                    v.reg
                }    
            },
            Tv::ProdDep(mut b) => {
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

                    let eval_l = self.eval(b.left);
                    let eval_r = self.eval(b.right);
                    if eval_l.is_some() && eval_r.is_some() {
                        b.vertex.reg = Some(eval_l.unwrap() * eval_r.unwrap());
                    }
                    b.vertex.reg
                }
            }
            _ => { unimplemented!() }
        }
    }
}