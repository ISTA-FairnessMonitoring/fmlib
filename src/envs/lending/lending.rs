use std::collections::HashMap;

#[derive(Default)]
pub struct Lending {
    pub group_count: i32, // G
    pub credit_score: i32, // C
    pub group_population: Vec<i32>, // [N1, N2, ..., NG]
    pub payback_prob: HashMap<i32, f64>, // [0..C) -> [0.0, 1.0]
    pub init_credit: Vec<Vec<i32>>, // [0..G) * [0..C) -> [1..N]
    pub policy: HashMap<(i32, i32), f64>, // [0..G) * [0..C) -> [0.0, 1.0]
}
