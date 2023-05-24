pub type Tv<T> = TreeVertex<T>;

#[derive(Clone, Debug)]
pub struct Frequency<T: Clone> {
    pub a: T,
    pub a_prime: T,
    pub count_a: i32,
    pub count_a_ap: i32,
    pub param_a: f64,
    pub param_a_ap: f64,
}

#[derive(Clone)]
pub struct Binary {
    pub left: i32,
    pub right: i32,
}

#[derive(Clone)]
pub enum TreeVertex<T: Clone> {
    Frequency(i32, Frequency<T>),
    Sum(i32, Binary),
    Subtract(i32, Binary),
    Prod(i32, Binary),
    ProdDependent(i32, Binary),
    ProdUnary(i32, i32, f64),
    Square(i32, i32),
}

#[derive(Clone)]
pub struct Fraction {
    pub num: f64,
    pub denom: f64,
}

impl Fraction {
    pub fn to_number(&self) -> f64 {
        self.num / self.denom
    }

    pub fn normalize(&self) -> Fraction {
        if self.num + self.denom >= 1e9 {
            Fraction {
                num: self.num / 1e8,
                denom: self.denom / 1e8
            }
        } else {
            self.clone()
        }
    }
}

impl<T: Clone + PartialEq> TreeVertex<T> {
    pub fn idx(&self) -> i32 {
        match self {
            Self::Frequency(idx, _) => *idx,
            Self::Sum(idx, _) => *idx,
            Self::Subtract(idx, _) => *idx,
            Self::Prod(idx, _) => *idx,
            Self::ProdDependent(idx, _) => *idx,
            Self::ProdUnary(idx, _, _) => *idx,
            Self::Square(idx, _) => *idx,
        }
    }

    pub fn update(&mut self, sigma: T) {
        if let Self::Frequency(_, ref mut f) = self {
            f.count_a += 1;
            if f.a_prime == sigma {
                f.count_a_ap += 1;
            }
        }
    }

    pub fn eval(&self, v: &Vec<Tv<T>>) -> Fraction {
        match self {
            Self::Frequency(_, f) => {
                Fraction {
                    num: f.count_a_ap as f64 + f.param_a_ap,
                    denom: f.count_a as f64 + f.param_a,
                }
            },
            Self::Sum(_, b) => {
                let eval_l = v[b.left as usize].eval(v).normalize();
                let eval_r = v[b.right as usize].eval(v).normalize();
            
                
                Fraction {
                    num: eval_l.num * eval_r.denom + eval_r.num * eval_l.denom,
                    denom: eval_l.denom * eval_r.denom,    
                }
            },
            Self::Subtract(_, b) => {
                let eval_l = v[b.left as usize].eval(v).normalize();
                let eval_r = v[b.right as usize].eval(v).normalize();
            
                
                Fraction {
                    num: eval_l.num * eval_r.denom - eval_r.num * eval_l.denom,
                    denom: eval_l.denom * eval_r.denom,    
                }
            },
            Self::Prod(_, b) => {
                let eval_l = v[b.left as usize].eval(v).normalize();
                let eval_r = v[b.right as usize].eval(v).normalize();

                Fraction {
                    num: eval_l.num * eval_r.num,
                    denom: eval_l.denom * eval_r.denom,
                }
            },

            // Special case of `Prod`, where the operands of the form:
            // Freq {a, a_p}, Freq {a, a_pp} (a_p != a_pp).
            Self::ProdDependent(_, b) => {
                let eval_l = v[b.left as usize].eval(v).normalize();
                let eval_r = v[b.right as usize].eval(v).normalize();

                Fraction {
                    num: eval_l.num * eval_r.num,
                    denom: eval_l.denom * (eval_l.denom + 1.0)
                }
            },
            
            // Special case of `ProdDependent`, where a_p == a_pp.
            Self::Square(_, u) => {
                let eval_v = v[*u as usize].eval(v).normalize();
                Fraction {
                    num: eval_v.num * (eval_v.num + 1.0),
                    denom: eval_v.denom * (eval_v.denom + 1.0)
                }
            }

            // `u`, `c` indicate child index and constant factor, respectively
            Self::ProdUnary(_, u, c) => {
                let eval_v = v[*u as usize].eval(v).normalize();
                Fraction {
                    num: eval_v.num * c,
                    denom: eval_v.denom
                }
            }
        }
    }
}
