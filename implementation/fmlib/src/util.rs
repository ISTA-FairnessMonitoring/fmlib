use std::vec;

use crate::envs::admission::{
    admission::Admission,
    memoryless::Ammc
};

use crate::envs::mc::Mc;
use crate::envs::lending::{
    lending::Lending,
    markov_chain::*,
};
use std::collections::HashMap;
use rand::Rng;

// ================ Markov Chain Generators ================
// Each function in this division mainly generates a Markov chain,
// parameters of which are hard-coded in the function body.

pub fn markov_chain_memless() -> LendingMarkovChain {
    let payback_prob = (0..10).map(
        |x| (x, ((x + 5) % 10) as f64 / 10.0)
    ).collect::<HashMap<_, _>>();
    
    let mut policy = HashMap::<(i32, i32), f64>::new();
    (0..10).for_each(|i| {
        policy.insert((0, i), (i + 1) as f64 / 10.0);
    });
    (0..10).for_each(|i| {
        policy.insert((1, i), ((i + 8) % 10) as f64 / 10.0);
    });

    let init_credit = vec![
        vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        vec![5, 5, 1, 1, 1, 1, 1, 2, 2, 1]
    ];

    let lending = Lending {
        group_count: 2,
        credit_score: 10,
        group_population: vec![20, 20],
        payback_prob,
        init_credit,
        policy,
    };
    
    LendingMarkovChain::new(lending)
}

pub fn markov_chain_lending_medium() -> LendingMarkovChain {
    let payback_prob: HashMap<_, _> = vec![
        (0, 0.4), (1, 0.7), (2, 0.8),
    ].into_iter().collect();

    let policy: HashMap<_, _> = vec![
        ((0, 0), 0.5),
        ((0, 1), 0.6),
        ((0, 2), 0.7),
        ((1, 0), 0.7),
        ((1, 1), 0.7),
        ((1, 2), 0.7),
    ].into_iter().collect();

    let init_credit = vec![
        vec![2, 5, 3],
        vec![3, 3, 4],
    ];

    let lending = Lending {
        group_count: 2,
        credit_score: 3,
        group_population: vec![10, 10],
        payback_prob,
        init_credit,
        policy,
    };

    LendingMarkovChain::new(lending)
}



pub fn markov_chain_lending_medium2() -> LendingMarkovChain {
    let payback_prob: HashMap<_, _> = vec![
        (0, 0.1), (1, 0.3), (2, 0.5),(4, 0.),
    ].into_iter().collect();

    let policy: HashMap<_, _> = vec![
        ((0, 0), 0.5),
        ((0, 1), 0.6),
        ((0, 2), 0.7),
        ((1, 0), 0.7),
        ((1, 1), 0.7),
        ((1, 2), 0.7),
    ].into_iter().collect();

    let init_credit = vec![
        vec![2, 5, 3],
        vec![3, 3, 4],
    ];

    let lending = Lending {
        group_count: 2,
        credit_score: 3,
        group_population: vec![10, 10],
        payback_prob,
        init_credit,
        policy,
    };

    LendingMarkovChain::new(lending)
}

pub fn markov_chain_lending_medium_fair() -> LendingMarkovChain {
    let payback_prob: HashMap<_, _> = vec![
        (0, 0.4), (1, 0.7), (2, 0.8),
    ].into_iter().collect();

    let policy: HashMap<_, _> = vec![
        ((0, 0), 0.5),
        ((0, 1), 0.6),
        ((0, 2), 0.7),
        ((1, 0), 0.5),
        ((1, 1), 0.6),
        ((1, 2), 0.7),
    ].into_iter().collect();

    let init_credit = vec![
        vec![2, 5, 3],
        vec![2, 5, 3],
    ];

    let lending = Lending {
        group_count: 2,
        credit_score: 3,
        group_population: vec![10, 10],
        payback_prob,
        init_credit,
        policy,
    };

    LendingMarkovChain::new(lending)
}

pub fn markov_chain_lending_large() -> LendingMarkovChain {
    let payback_prob: HashMap<_, _> = vec![
        (0, 0.1), (1, 0.25), (2, 0.4), (3, 0.5), (4, 0.6),
        (5, 0.7), (6, 0.8), (7, 0.9), (8, 0.95), (9, 0.99)
    ].into_iter().collect();

    let policy: HashMap<_, _> = vec![
        ((0, 0), 0.05),
        ((0, 1), 0.1),
        ((0, 2), 0.2),
        ((0, 3), 0.3),
        ((0, 4), 0.4),
        ((0, 5), 0.5),
        ((0, 6), 0.6),
        ((0, 7), 0.7),
        ((0, 8), 0.8),
        ((0, 9), 0.9),
        ((1, 0), 0.2),
        ((1, 1), 0.3),
        ((1, 2), 0.4),
        ((1, 3), 0.5),
        ((1, 4), 0.6),
        ((1, 5), 0.7),
        ((1, 6), 0.8),
        ((1, 7), 0.85),
        ((1, 8), 0.9),
        ((1, 9), 0.95),
    ].into_iter().collect();

    let init_credit = vec![
        vec![  75, 100, 225, 200, 125, 100,  75,  50,  25,  25],
        vec![  250,  500, 1500, 2250, 1750, 1500, 1000,  500,  500,  250],
    ];

    let lending = Lending {
        group_count: 2,
        credit_score: 10,
        group_population: vec![1000, 10000],
        payback_prob,
        init_credit,
        policy,
    };

    LendingMarkovChain::new(lending)
}



pub fn markov_chain_lending_large_fair() -> LendingMarkovChain {
    let payback_prob: HashMap<_, _> = vec![
        (0, 0.1), (1, 0.25), (2, 0.4), (3, 0.5), (4, 0.6),
        (5, 0.7), (6, 0.8), (7, 0.9), (8, 0.95), (9, 0.99)
    ].into_iter().collect();

    let policy: HashMap<_, _> = vec![
        ((0, 0), 0.05),
        ((0, 1), 0.1),
        ((0, 2), 0.2),
        ((0, 3), 0.3),
        ((0, 4), 0.4),
        ((0, 5), 0.5),
        ((0, 6), 0.6),
        ((0, 7), 0.7),
        ((0, 8), 0.8),
        ((0, 9), 0.9),
        ((1, 0), 0.05),
        ((1, 1), 0.1),
        ((1, 2), 0.2),
        ((1, 3), 0.3),
        ((1, 4), 0.4),
        ((1, 5), 0.5),
        ((1, 6), 0.6),
        ((1, 7), 0.7),
        ((1, 8), 0.8),
        ((1, 9), 0.9),
    ].into_iter().collect();

    let init_credit = vec![
        vec![  250,  500, 1500, 2250, 1750, 1500, 1000,  500,  500,  250],
        vec![  250,  500, 1500, 2250, 1750, 1500, 1000,  500,  500,  250],
    ];

    let lending = Lending {
        group_count: 2,
        credit_score: 10,
        group_population: vec![10000, 10000],
        payback_prob,
        init_credit,
        policy,
    };

    LendingMarkovChain::new(lending)
}


pub fn markov_chain_7_state() -> Mc {
    let m = vec![
        vec![(1, 0.5), (4, 0.5)],
        vec![(2, 0.4), (3, 0.6)],
        vec![(0, 1.0)],
        vec![(0, 1.0)],
        vec![(5, 0.7), (6, 0.3)],
        vec![(0, 1.0)],
        vec![(0, 1.0)],
    ];
    
    let pi = vec![(0, 1.0)];
    
    Mc::new(m, pi)
}

pub fn markov_chain_admission_small() -> Ammc {
    let admission = Admission {
        score: 2,
        threshold: 2,
        label_threshold: 1,
        cost: vec![0,1,2],
        change_prob: vec![0.6, 0.3, 0.1],
    };

    Ammc { admission, curr: None }
}



pub fn markov_chain_admission_small_lv() -> Ammc {
    let admission = Admission {
        score: 2,
        threshold: 2,
        label_threshold: 1,
        cost: vec![0,1,2],
        change_prob: vec![0.9, 0.8, 0.2],
    };

    Ammc { admission, curr: None }
}


pub fn markov_chain_admission_medium() -> Ammc {
    let admission = Admission {
        score: 4,
        threshold: 3,
        label_threshold: 2,
        cost: vec![0,1,2,3,4],
        change_prob: vec![0.9, 0.8, 0.7,0.6,0.5],
    };

    Ammc { admission, curr: None }
}


pub fn markov_chain_admission_large() -> Ammc {
    let admission = Admission {
        score: 10,
        threshold: 7,
        label_threshold: 6,
        cost: vec![0,1,2,3,4,5,6,7,8,9,10],
        change_prob: vec![0.95,0.9,0.85,0.8,0.65,0.5,0.4,0.2,0.15,0.1,0.05],
    };

    Ammc { admission, curr: None }
}


pub fn markov_chain_admission_large_costs() -> Ammc {
    let admission = Admission {
        score: 10,
        threshold: 7,
        label_threshold: 6,
        cost: vec![0,1,2,3,4,5,6,7,8,9,10],
        change_prob: vec![0.95,0.9,0.85,0.8,0.65,0.5,0.4,0.2,0.15,0.1,0.05],
        // change_prob: vec![0.95,0.9,0.9,0.9,0.9,0.9,0.9,0.9,0.9,0.9,0.9],
    };

    Ammc { admission, curr: None }
}


// ================ Mathematics ================
// These functions are generic math functions,
// and refactored for more code cleanliness.


// Perform a weighted choice from a vector of (value, weight) pairs.
pub fn weighted_choice<T: Clone>(v: &[(T, f64)]) -> T {
    let mut weights = vec![];
    for (_, w) in v {
        weights.push(*w);
    }

    let cdf = to_cdf(&weights);

    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0.0..1.0);

    v[binary_search(&cdf, random).1 as usize].0.clone()
}

pub fn to_cdf(v: &Vec<f64>) -> Vec<f64> {
    v.iter().scan(
        0.0,
        |acc, x| {
            *acc += x;
            Some(*acc)
        }
    ).collect::<Vec<_>>()
}

pub fn binary_search(v: &Vec<f64>, k: f64) -> (i32, i32) {
    let (mut l, mut r): (i32, i32) = (-1, v.len() as i32);
    while r - l > 1 {
        let m = (r + l) >> 1;
        if v[m as usize] < k {
            l = m;
        } else {
            r = m;
        }
    }

    (l, r)
}

pub fn _update(v: &mut Vec<Vec<i32>>, (i, j): (i32, i32), k: i32) {
    v[i as usize][j as usize] += k;
}

pub fn _max_update(v: &mut Vec<Vec<i32>>, (i, j): (i32, i32), m: i32) {
    v[i as usize][j as usize] = v[i as usize][j as usize].max(m);
}

// These structures are used as arguments
// for the two Welford functions.
#[derive(Clone, Default)]
pub struct WelfordData {
    pub n: i32,
    pub mean: f64,
    pub m2: f64,
}

#[derive(Clone, Default)]
pub struct WelfordResult {
    pub mean: f64,
    pub variance: f64,
}

pub fn welford_update(
    d: WelfordData,
    new_x: f64
) -> WelfordData {
    let new_n = d.n + 1;
    let new_mean = d.mean + (new_x - d.mean) / new_n as f64;
    let new_m2 = d.m2 + (new_x - d.mean) * (new_x - new_mean);
    
    WelfordData {
        n: new_n,
        mean: new_mean,
        m2: new_m2
    }
}

// The return type is chosen to be `Option<WelfordResult>`,
// as the result may sometimes be invalid.
pub fn welford_finalize(
    d: WelfordData
) -> Option<WelfordResult> {
    if d.n < 2 {
        return None;
    }

    Some(
        WelfordResult {
            mean: d.mean,
            variance: d.m2 / (d.n as f64 - 1.0)
        }
    )
}

// Ln(P(n, k)) = Ln(k + 1) + ... + Ln(n).
pub fn ln_permute(n: i32, k: i32) -> f64 {
    assert!(n >= k);

    if n == 0 || k == 0 {
        return 1.0;
    }

    let mut result = 0.0;
    for i in (n-k+1)..(n+1) {
        result += (i as f64).ln();
    }

    result
}
