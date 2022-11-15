use std::collections::VecDeque;

// Shorthand name for TreeVertex
// Tv uses a generic type, so that we can define it
// over various Markov chain vertex types.
pub type Tv<T> = TreeVertex<T>;


// Concrete form of a _Frequency_ vertex
// in the Frequentist monitor
#[derive(Clone, Debug)]
pub struct Frequency<T: Clone> {
    pub a: T,
    pub a_prime: T,
    pub count: i32,
}

// Concrete form of a _Binary Operator_ vertex
// in the Frequentist monitor
#[derive(Clone, Debug)]
pub struct Binary {
    pub left: i32,              // Index of left operand vertex
    pub right: i32,             // Index of right operand vertex
    pub q_left: VecDeque<f64>,  // Queue for left operand observations
    pub q_right: VecDeque<f64>, // Queue for right operand observations
}

// Concrete form of an _InverseAtomic_ vertex
// in the Frequentist monitor
#[derive(Clone, Debug)]
pub struct InverseAtomic {
    pub count: i32,
    pub limit: i32, // Observation threshold
    // Limit enforces the condition that,
    // for any non-zero sample: count < limit 
}

#[derive(Clone, Debug)]
pub enum TreeVertex<T: Clone> {
    // The first element of each variate is the index field.
    Frequency(i32, Frequency<T>),
    Sum(i32, Binary),
    // For unary operator vertices, second and third elements
    // are child index and constant value, respectively.
    SumUnary(i32, i32, f64),
    Subtract(i32, Binary),
    SubtractUnary(i32, i32, f64),
    Prod(i32, Binary),
    ProdUnary(i32, i32, f64),
    ProdDependent(i32, Binary),
    InverseAtomic(i32, i32, InverseAtomic),
}

// Return type is chosen to be `Optional<f64>`,
// since for some vertices (e.g. Binary operators),
// a result cannot be readily returned.
impl<T: Clone> TreeVertex<T> {
    pub fn update(
        &mut self,
        caller: i32, // Index of calling child
        value: f64,  // Passed value from the child
    ) -> Option<f64> {
        // Result will be valuated in exactly one of the `match` arms.
        let result: Option<f64>;
        match self {
            Self::Frequency(_, ref mut f) => {
                f.count += 1;
                result = Some(value);
            },

            // For Binary operators, we initially place the passed value
            // in its corresponding queue. Then, with respect to operation itself,
            // we pop some values from the queues and construct a new value.
            // This new value will be stored in `result`.

            Self::Sum(_, ref mut b) => {
                if caller == b.left {
                    b.q_left.push_back(value);
                } else if caller == b.right {
                    b.q_right.push_back(value);
                }
                
                if !b.q_left.is_empty() && !b.q_right.is_empty() {
                    let left = b.q_left.pop_front().unwrap();
                    let right = b.q_right.pop_front().unwrap();
                    result = Some(left + right);
                } else {
                    result = None;
                }
            },

            // For unary operators, all that must be done
            // is to perform the specified operation,
            // with `value` and `c` as operands.
            
            Self::SumUnary(_, _, c) => {
                result = Some(value + *c);
            },
            Self::Subtract(_, b) => {
                if caller == b.left {
                    b.q_left.push_back(value);
                } else if caller == b.right {
                    b.q_right.push_back(value);
                }
                
                if !b.q_left.is_empty() && !b.q_right.is_empty() {
                    let left = b.q_left.pop_front().unwrap();
                    let right = b.q_right.pop_front().unwrap();
                    result = Some(left - right);
                } else {
                    result = None;
                }
            },
            Self::SubtractUnary(_, _, c) => {
                result = Some(value - *c);
            },
            Self::Prod(_, b) => {
                if caller == b.left {
                    b.q_left.push_back(value);
                } else if caller == b.right {
                    b.q_right.push_back(value);
                }

                if !b.q_left.is_empty() && !b.q_right.is_empty() {
                    let left = b.q_left.pop_front().unwrap();
                    let right = b.q_right.pop_front().unwrap();
                    result = Some(left * right);
                } else {
                    result = None;
                }
            },
            Self::ProdUnary(_, _, c) => {
                result = Some(value * *c)
            },

            // `ProdDependent` needs at least two observations from each child
            // to construct a new value.
            Self::ProdDependent(_, b) => {
                if caller == b.left {
                    b.q_left.push_back(value);
                } else if caller == b.right {
                    b.q_right.push_back(value);
                }
                
                if b.q_left.len() >= 2 && b.q_right.len() >= 2 {
                    let left = b.q_left.pop_front().unwrap();
                    b.q_left.pop_front();
                    
                    b.q_right.pop_front();
                    let right = b.q_right.pop_front().unwrap();
                    
                    result = Some(left * right);
                } else {
                    result = None;
                }
            }

            // `InverseAtomic` waits for seeing a non-zero observation
            // at most `limit` times; after this threshold, its counter resets to 0.
            Self::InverseAtomic(_, _, ref mut i) => {
                i.count += 1;
                if value == 0.0 {
                    if i.count == i.limit {
                        i.count = 0;
                    }
                    
                    result = None;
                } else {
                    result = Some(i.count as f64);
                    i.count = 0;
                }
            }
        }
        
        result
    }

    pub fn idx(&self) -> i32 {
        match self {
            Self::Frequency(idx, _) => *idx,
            Self::Sum(idx, _) => *idx,
            Self::SumUnary(idx, _, _) => *idx,
            Self::Subtract(idx, _) => *idx,
            Self::SubtractUnary(idx, _, _) => *idx,
            Self::Prod(idx, _) => *idx,
            Self::ProdUnary(idx, _, _) => *idx,
            Self::ProdDependent(idx, _) => *idx,
            Self::InverseAtomic(idx, _, _) => *idx,
        }
    }

    // Domain calculations are in accordance
    // to their interval arithmetic equivalents.
    pub fn domain(&self, v: &Vec<Tv<T>>) -> [f64; 2] {
        match self {
            // `Frequency` observations are always in {0, 1}.
            Self::Frequency(_, _) => [0.0, 1.0],
            
            // Sum of domains [l, h], [l', h'] is as follows:
            // [l + l', h + h']
            Self::Sum(_, b) => {
                let d_left = v[b.left as usize].domain(v);
                let d_right = v[b.right as usize].domain(v);
                [d_left[0] + d_right[0] , d_left[1] + d_right[1]]
            },

            // Subtraction of domains [l, h], [l', h'] is as follows:
            // [l - h', l' - h]
            Self::Subtract(_, b) => {
                let d_left = v[b.left as usize].domain(v);
                let d_right = v[b.right as usize].domain(v);
                [d_left[0] - d_right[1] , d_left[1] - d_right[0]]
            },
            // Product of domains [l, h], [l', h'] is as follows:
            // [min(S), max(S)]
            // where S = {l*h, l*h', l'*h, l'*h'}.
            Self::Prod(_, b) => {
                let d_left = v[b.left as usize].domain(v);
                let d_right = v[b.right as usize].domain(v);

                let (mut min, mut max): (f64, f64) = (1e9, -1e9);
                for x in d_left.iter() {
                    for y in d_right.iter() {
                        min = min.min(x * y);
                        max = max.max(x * y);
                    }
                }

                [min, max]
            },
            Self::SumUnary(_, child, c) => {
                let d_child = v[*child as usize].domain(v);
                [d_child[0] + c, d_child[1] + c]
            },
            Self::SubtractUnary(_, child, c) => {
                let d_child = v[*child as usize].domain(v);
                [d_child[0] - c, d_child[1] - c]
            }
            Self::ProdUnary(_, child, c) => {
                let d_child = v[*child as usize].domain(v);
                if *c > 0.0 {
                    [d_child[0] * c, d_child[1] * c]
                } else {
                    [d_child[1] * c, d_child[0] * c]
                }
            },
            Self::ProdDependent(_, b) => {
                let d_left = v[b.left as usize].domain(v);
                let d_right = v[b.right as usize].domain(v);

                let (mut min, mut max): (f64, f64) = (1e9, -1e9);
                for x in d_left.iter() {
                    for y in d_right.iter() {
                        min = min.min(x * y);
                        max = max.max(x * y);
                    }
                }

                [min, max]
            }
            Self::InverseAtomic(_, _, i) => {
                [1.0, i.limit as f64]
            }
        }
    }
}