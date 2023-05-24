// Definition of a tree vertex

use std::fmt::Debug;

pub type Tv<T> = TreeVertex<T>;

#[derive(Clone, Debug)]
pub struct Frequency<T: Clone> {
    pub a: T,
    pub a_prime: T,
    pub count: i32,
    pub estimate: f64,
    pub delta: f64,
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub left: i32,
    pub right: i32,
}

// Definition of tree vertex data types, the first member is always the vertex id (i32)
#[derive(Clone, Debug)]
pub enum TreeVertex<T: Clone> {
    Constant(i32, f64),
    Frequency(i32, Frequency<T>),
    Sum(i32, Binary),
    Subtract(i32, Binary),
    Prod(i32, Binary),
}

// Return type of eval function for each vertex
#[derive(Debug, Default)]
pub struct EvalResult {
    pub value: f64, // Center of the interval
    pub epsilon: f64,
    pub delta: f64,
}

impl<T: Clone + PartialEq + Debug> TreeVertex<T> {
    pub fn idx(&self) -> i32 {
        match self {
            Self::Constant(idx, _) => *idx,
            Self::Frequency(idx, _) => *idx,
            Self::Sum(idx, _) => *idx,
            Self::Subtract(idx, _) => *idx,
            Self::Prod(idx, _) => *idx,
        }
    }

    pub fn eval(&self, v: &Vec<Tv<T>>) -> EvalResult {
        match self {
            Self::Constant(_, c) => {
                EvalResult { value: *c, epsilon: 0.0, delta: 0.0 }
            },
            Self::Frequency(_, f) => {
                EvalResult {
                    value: f.estimate,
                    delta: f.delta,
                    epsilon: (-f.count as f64 / 2.0 * (f.delta / 2.0).ln()).sqrt() / f.count as f64,
                }
            },
            Self::Sum(_, b) => {
                let eval_left = v[b.left as usize].eval(v);
                let eval_right = v[b.right as usize].eval(v);

                EvalResult {
                    value: eval_left.value + eval_right.value,
                    epsilon: eval_left.epsilon + eval_right.epsilon,
                    delta: eval_left.delta + eval_right.delta
                }
            },
            Self::Subtract(_, b) => {
                let eval_left = v[b.left as usize].eval(v);
                let eval_right = v[b.right as usize].eval(v);

                EvalResult {
                    value: eval_left.value - eval_right.value,
                    epsilon: eval_left.epsilon + eval_right.epsilon,
                    delta: eval_left.delta + eval_right.delta
                }
            },
            Self::Prod(_, b) => { // This is exactly as per fairness aware programming paper.
                // Better bound can be worked out for the product. See the paper draft. 
                let eval_l = v[b.left as usize].eval(v);
                let eval_r = v[b.right as usize].eval(v);

                let value = eval_l.value * eval_r.value;
                let epsilon: f64 = (0.0 as f64).max(
                    (value - (eval_l.value + eval_l.epsilon) * (eval_r.value + eval_r.epsilon)).abs()
                ).max(
                    (value - (eval_l.value + eval_l.epsilon) * (eval_r.value - eval_r.epsilon)).abs()
                ).max(
                    (value - (eval_l.value - eval_l.epsilon) * (eval_r.value + eval_r.epsilon)).abs()
                ).max(
                    (value - (eval_l.value - eval_l.epsilon) * (eval_r.value - eval_r.epsilon)).abs()
                );

                EvalResult {
                    value,
                    delta: eval_l.delta + eval_r.delta,
                    epsilon,
                }
            }
        }
    }

    pub fn update(&mut self, sigma: T) {
        if let Self::Frequency(_, ref mut f) = self {
            f.count += 1;
            if f.a_prime == sigma {
                f.estimate += (1.0 - f.estimate) / f.count as f64;
            } else {
                f.estimate -= f.estimate / f.count as f64;
            }
        }
    }

    pub fn set_delta(&mut self, delta: f64) {
        match self {
            Self::Frequency(_, ref mut f) => {
                f.delta = f.delta.min(delta);
            },
            _ => {}
        }
    }

    pub fn children(&self) -> Vec<i32> {
        match self {
            Self::Sum(_, b) | Self::Subtract(_, b) | Self::Prod(_, b) => {
                vec![b.left, b.right]
            },
            _ => { vec![] }
        }
    }
}
